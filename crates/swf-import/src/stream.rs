//! The importer's stream: one sealed row per stage and per 1,000 measured records, flushed as written.
//!
//! Q7 fixed the cadence — *"Per stage, plus every 1,000 records inside a stage"* — and Q6 fixed the
//! sealing: a row without its AEAD tag is refused. The domain is the store-record one, because these
//! rows are what the store receives; a row sealed for the ledger or a frame does not open here.
//!
//! The property that matters is the one this file is written around: **a stream that stops without its
//! `done` row is `import-truncated`, never a silent partial.** Rows already written stay readable — a
//! killed import leaves evidence, not nothing — and the reader says how many it found.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use planet_aead::{open, seal, Domain, Key};

/// The header a reader can recognise before decoding a single row.
pub const HEADER: &str = "planet.import-stream";

/// The stream schema, sealed inside every row rather than worn as a format id. The gate refuses an
/// unregistered `name/N` id in code, and a version inside the seal is the stronger place for it.
pub const SCHEMA: u8 = 1;

/// What a row records.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A stage boundary: the measurement a stage produced.
    Stage,
    /// Every 1,000 records inside a stage.
    Record,
    /// The stream ended because the import finished.
    Done,
    /// The stream ended because the import aborted, and names the code it aborted with.
    Truncated,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::Stage => "stage",
            Kind::Record => "record",
            Kind::Done => "done",
            Kind::Truncated => "truncated",
        }
    }

    fn parse(text: &str) -> Option<Kind> {
        Some(match text {
            "stage" => Kind::Stage,
            "record" => Kind::Record,
            "done" => Kind::Done,
            "truncated" => Kind::Truncated,
            _ => return None,
        })
    }
}

/// One row as the reader hands it back. The detail is the plaintext the seal covers, so a reader that
/// can open the row has already proved it was not edited.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub kind: Kind,
    pub seq: u64,
    pub detail: String,
    /// The sealing key's short public id, never the key.
    pub key: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    Seal(planet_aead::Refusal),
    Open(planet_aead::Refusal),
    /// The line is not one of this stream's rows.
    Row(String),
    /// The row is well formed but does not belong where it is: a row after the stream closed, a key
    /// that is not this stream's.
    Order(String),
    /// The stream stopped without its `done` row.
    Truncated { rows: usize },
    Io(String),
}

impl Refusal {
    pub fn code(&self) -> &'static str {
        match self {
            Refusal::Seal(_) => "stream-seal",
            Refusal::Open(refusal) => refusal.code(),
            Refusal::Row(_) => "stream-row",
            Refusal::Order(_) => "stream-order",
            Refusal::Truncated { .. } => "import-truncated",
            Refusal::Io(_) => "stream-io",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Refusal::Seal(refusal) | Refusal::Open(refusal) => refusal.message(),
            Refusal::Row(line) => format!("not one of this stream's rows: {line}"),
            Refusal::Order(detail) => format!("row out of place: {detail}"),
            Refusal::Truncated { rows } => format!(
                "the stream ends without its done row after {rows} sealed rows; the import did not finish"
            ),
            Refusal::Io(error) => format!("stream could not be read or written: {error}"),
        }
    }
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn from_hex(text: &str) -> Option<Vec<u8>> {
    if text.len() % 2 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(text.len() / 2);
    for pair in text.as_bytes().chunks(2) {
        let pair = std::str::from_utf8(pair).ok()?;
        out.push(u8::from_str_radix(pair, 16).ok()?);
    }
    Some(out)
}

/// A stream being written. Every row is sealed, appended and flushed before the next one exists.
#[derive(Debug)]
pub struct Stream {
    path: PathBuf,
    key: Key,
    seq: u64,
    out: File,
    closed: bool,
}

impl Stream {
    /// Start a stream at `path`. A stream never inherits another's rows: the file is created afresh.
    pub fn create(path: &Path, key: Key) -> Result<Stream, Refusal> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| Refusal::Io(error.to_string()))?;
        }
        let out = File::create(path).map_err(|error| Refusal::Io(error.to_string()))?;
        Ok(Stream {
            path: path.to_path_buf(),
            key,
            seq: 0,
            out,
            closed: false,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Append one sealed row and flush it. Sealing happens before anything is written, so a row that
    /// cannot be sealed is not a row at all.
    pub fn row(&mut self, kind: Kind, detail: &str) -> Result<Row, Refusal> {
        if self.closed {
            return Err(Refusal::Order(format!("row after {}", Kind::Done.name())));
        }
        self.seq += 1;
        let plaintext = format!("{SCHEMA}\t{}\t{}\t{}", kind.name(), self.seq, detail);
        let sealed = seal(&self.key, Domain::StoreRecord, plaintext.as_bytes());
        let row = Row {
            kind,
            seq: self.seq,
            detail: detail.to_string(),
            key: self.key.id(),
        };
        let line = format!(
            "{{\"row\":\"{}\",\"seq\":{},\"key\":\"{}\",\"sealed\":\"{}\"}}\n",
            kind.name(),
            self.seq,
            row.key,
            to_hex(&sealed)
        );
        self.out
            .write_all(line.as_bytes())
            .map_err(|error| Refusal::Io(error.to_string()))?;
        self.out
            .flush()
            .map_err(|error| Refusal::Io(error.to_string()))?;
        if matches!(kind, Kind::Done | Kind::Truncated) {
            self.closed = true;
        }
        Ok(row)
    }

    /// The stream finished: the row that makes it complete.
    pub fn finish(&mut self) -> Result<Row, Refusal> {
        self.row(Kind::Done, "")
    }

    /// The import aborted: the row that names why, so a partial stream is never silent.
    pub fn abort(&mut self, code: &str) -> Result<Row, Refusal> {
        self.row(Kind::Truncated, code)
    }
}

/// What a reader found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Read {
    pub rows: Vec<Row>,
    /// True only when the last row is `done`.
    pub complete: bool,
}

fn parse_line(line: &str, key: &Key) -> Result<Row, Refusal> {
    let body = line
        .strip_prefix('{')
        .and_then(|rest| rest.strip_suffix('}'))
        .ok_or_else(|| Refusal::Row(line.to_string()))?;
    let rest = body
        .strip_prefix("\"row\":\"")
        .ok_or_else(|| Refusal::Row(line.to_string()))?;
    let (kind_text, rest) = rest
        .split_once("\",\"seq\":")
        .ok_or_else(|| Refusal::Row(line.to_string()))?;
    let (seq_text, rest) = rest
        .split_once(",\"key\":\"")
        .ok_or_else(|| Refusal::Row(line.to_string()))?;
    let (key_text, rest) = rest
        .split_once("\",\"sealed\":\"")
        .ok_or_else(|| Refusal::Row(line.to_string()))?;
    let (sealed_text, tail) = rest
        .split_once('"')
        .ok_or_else(|| Refusal::Row(line.to_string()))?;
    if !tail.is_empty() {
        return Err(Refusal::Row(line.to_string()));
    }
    let kind = Kind::parse(kind_text).ok_or_else(|| Refusal::Row(line.to_string()))?;
    let seq: u64 = seq_text
        .parse()
        .map_err(|_| Refusal::Row(line.to_string()))?;
    if key_text != key.id() {
        return Err(Refusal::Order(format!(
            "row {seq} was sealed by another key"
        )));
    }
    let sealed = from_hex(sealed_text).ok_or_else(|| Refusal::Row(line.to_string()))?;
    let plaintext = open(key, Domain::StoreRecord, &sealed).map_err(Refusal::Open)?;
    let plaintext = String::from_utf8(plaintext).map_err(|_| Refusal::Row(line.to_string()))?;
    let mut parts = plaintext.splitn(4, '\t');
    let version = parts.next().and_then(|text| text.parse::<u8>().ok());
    let sealed_kind = parts.next().and_then(Kind::parse);
    let sealed_seq = parts.next().and_then(|text| text.parse::<u64>().ok());
    let detail = parts.next().unwrap_or("").to_string();
    if version != Some(SCHEMA) {
        return Err(Refusal::Order(format!(
            "row {seq} carries stream schema {version:?}; this reader reads {SCHEMA}"
        )));
    }
    if sealed_kind != Some(kind) || sealed_seq != Some(seq) {
        return Err(Refusal::Order(format!(
            "row {seq} opens as a different row than its envelope claims"
        )));
    }
    Ok(Row {
        kind,
        seq,
        detail,
        key: key_text.to_string(),
    })
}

/// Read a stream without judging whether it finished. A truncated stream still yields every row it
/// managed to seal, which is the evidence a killed import should leave.
pub fn read(path: &Path, key: &Key) -> Result<Read, Refusal> {
    let text = fs::read_to_string(path).map_err(|error| Refusal::Io(error.to_string()))?;
    let mut rows = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        rows.push(parse_line(line, key)?);
    }
    let complete = rows.last().map(|row| row.kind == Kind::Done).unwrap_or(false);
    Ok(Read { rows, complete })
}

/// Read a stream and refuse anything that did not finish.
pub fn read_complete(path: &Path, key: &Key) -> Result<Read, Refusal> {
    let read = read(path, key)?;
    if !read.complete {
        return Err(Refusal::Truncated { rows: read.rows.len() });
    }
    Ok(read)
}
