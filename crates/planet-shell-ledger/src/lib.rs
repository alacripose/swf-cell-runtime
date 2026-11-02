//! The shell's commit boundary, taken from the shell family's own ledger shape: one typed entry per
//! change, where the durable record is the change and its evidence together; a SHA-256 chain with a
//! genesis head; a caller-supplied head (`expected_seq`) so a stale writer is refused rather than
//! silently rebased; idempotent operation ids; and durability through `landing`, which stages beside the destination, syncs, then renames.

pub mod landing;
pub mod sha256;

/// The chain's domain separator, so a hash from another structure cannot be replayed here.
pub const DOMAIN: &[u8] = b"planet-shell.ledger/1";
/// The head of an empty chain. No entry hash may equal it.
pub const GENESIS: &str = "genesis";
pub const SCHEMA: &str = "planet-shell.ledger/1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Baseline,
    Milestone,
    Recipe,
    Record,
    Patch,
    Test,
    Build,
    Mount,
    Prompt,
    Convention,
    CodeChange,
    Retention,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Baseline => "baseline",
            Kind::Milestone => "milestone",
            Kind::Recipe => "recipe",
            Kind::Record => "record",
            Kind::Patch => "patch",
            Kind::Test => "test",
            Kind::Build => "build",
            Kind::Mount => "mount",
            Kind::Prompt => "prompt",
            Kind::Convention => "convention",
            Kind::CodeChange => "code-change",
            Kind::Retention => "retention",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Some(match text {
            "baseline" => Kind::Baseline,
            "milestone" => Kind::Milestone,
            "recipe" => Kind::Recipe,
            "record" => Kind::Record,
            "patch" => Kind::Patch,
            "test" => Kind::Test,
            "build" => Kind::Build,
            "mount" => Kind::Mount,
            "prompt" => Kind::Prompt,
            "convention" => Kind::Convention,
            "code-change" => Kind::CodeChange,
            "retention" => Kind::Retention,
            _ => return None,
        })
    }
}

/// What a test or build was, in a form a reader can replay: command, exit status, check count,
/// surface and digest. An entry with no evidence is refused, not defaulted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    pub command: String,
    pub exit_status: i32,
    pub checks: u32,
    pub surface: String,
    pub digest: String,
}

impl Evidence {
    pub fn new(command: &str, exit_status: i32, checks: u32, surface: &str, digest: &str) -> Self {
        Evidence {
            command: command.to_string(),
            exit_status,
            checks,
            surface: surface.to_string(),
            digest: digest.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub seq: u64,
    pub prev: String,
    pub hash: String,
    pub operation_id: String,
    pub at: u64,
    pub actor: String,
    pub kind: Kind,
    pub authority_ref: String,
    pub subject: String,
    pub digest: String,
    pub evidence: Evidence,
    pub outcome: String,
}

/// What a caller proposes. The chain fields are the ledger's to fill, never the caller's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub operation_id: String,
    pub at: u64,
    pub actor: String,
    pub kind: Kind,
    pub authority_ref: String,
    pub subject: String,
    pub digest: String,
    pub evidence: Evidence,
    pub outcome: String,
}

impl Request {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_id: &str,
        at: u64,
        actor: &str,
        kind: Kind,
        authority_ref: &str,
        subject: &str,
        digest: &str,
        evidence: Evidence,
    ) -> Self {
        Request {
            operation_id: operation_id.to_string(),
            at,
            actor: actor.to_string(),
            kind,
            authority_ref: authority_ref.to_string(),
            subject: subject.to_string(),
            digest: digest.to_string(),
            evidence,
            outcome: "landed".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Refusal {
    MissingField(&'static str),
    EmptyOperationId,
    StaleSeq { expected: u64, head: u64 },
    BrokenChain { seq: u64 },
    NoGenesis,
    DigestNotSha256 { field: &'static str },
    Unrepresentable { field: &'static str },
    WrongSchema { found: String },
    MalformedLine { line: usize },
}

impl Refusal {
    pub fn code(&self) -> &'static str {
        match self {
            Refusal::MissingField(_) => "missing-field",
            Refusal::EmptyOperationId => "empty-operation-id",
            Refusal::StaleSeq { .. } => "stale-seq",
            Refusal::BrokenChain { .. } => "broken-chain",
            Refusal::NoGenesis => "no-genesis",
            Refusal::DigestNotSha256 { .. } => "digest-not-sha256",
            Refusal::Unrepresentable { .. } => "unrepresentable",
            Refusal::WrongSchema { .. } => "wrong-schema",
            Refusal::MalformedLine { .. } => "malformed-line",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Refusal::MissingField(field) => {
                format!("{field} is empty; an entry names its author and subject")
            }
            Refusal::EmptyOperationId => {
                "operation_id is empty; a retry cannot be recognised".to_string()
            }
            Refusal::StaleSeq { expected, head } => format!(
                "expected_seq {expected} is not the current head {head}; the writer saw a different chain"
            ),
            Refusal::BrokenChain { seq } => {
                format!("entry {seq} does not hash to its recorded head")
            }
            Refusal::NoGenesis => {
                "the chain has no genesis head; a truncated log is not a ledger".to_string()
            }
            Refusal::DigestNotSha256 { field } => {
                format!("{field} is not 64 lowercase hex characters")
            }
            Refusal::Unrepresentable { field } => {
                format!("{field} contains a tab or newline and cannot be stored")
            }
            Refusal::WrongSchema { found } => format!("schema {found:?} is not {SCHEMA}"),
            Refusal::MalformedLine { line } => {
                format!("line {line} does not have the entry's fields")
            }
        }
    }
}

/// The outcome of an append: a new entry, or the one an idempotent retry already landed.
#[derive(Debug, PartialEq, Eq)]
pub enum Appended {
    New,
    Existing(u64),
}

fn put(out: &mut Vec<u8>, value: &str) {
    out.extend_from_slice(&(value.len() as u32).to_be_bytes());
    out.extend_from_slice(value.as_bytes());
}

impl Entry {
    /// The entry's own hash: the domain, the chain position, and every field that carries meaning.
    pub fn compute_hash(seq: u64, prev: &str, request: &Request) -> String {
        let mut message = Vec::new();
        message.extend_from_slice(DOMAIN);
        message.extend_from_slice(&seq.to_be_bytes());
        put(&mut message, prev);
        put(&mut message, &request.operation_id);
        message.extend_from_slice(&request.at.to_be_bytes());
        put(&mut message, &request.actor);
        put(&mut message, request.kind.as_str());
        put(&mut message, &request.authority_ref);
        put(&mut message, &request.subject);
        put(&mut message, &request.digest);
        put(&mut message, &request.evidence.command);
        message.extend_from_slice(&request.evidence.exit_status.to_be_bytes());
        message.extend_from_slice(&request.evidence.checks.to_be_bytes());
        put(&mut message, &request.evidence.surface);
        put(&mut message, &request.evidence.digest);
        put(&mut message, &request.outcome);
        sha256::hex(&sha256::sha256(&message))
    }

    fn to_line(&self) -> Result<String, Refusal> {
        let fields = [
            ("schema", SCHEMA.to_string()),
            ("seq", self.seq.to_string()),
            ("prev", self.prev.clone()),
            ("hash", self.hash.clone()),
            ("operation_id", self.operation_id.clone()),
            ("at", self.at.to_string()),
            ("actor", self.actor.clone()),
            ("kind", self.kind.as_str().to_string()),
            ("authority_ref", self.authority_ref.clone()),
            ("subject", self.subject.clone()),
            ("digest", self.digest.clone()),
            ("command", self.evidence.command.clone()),
            ("exit_status", self.evidence.exit_status.to_string()),
            ("checks", self.evidence.checks.to_string()),
            ("surface", self.evidence.surface.clone()),
            ("evidence_digest", self.evidence.digest.clone()),
            ("outcome", self.outcome.clone()),
        ];
        let mut line = String::new();
        for (index, (name, value)) in fields.iter().enumerate() {
            if value.contains('\t') || value.contains('\n') {
                return Err(Refusal::Unrepresentable { field: name });
            }
            if index > 0 {
                line.push('\t');
            }
            line.push_str(value);
        }
        Ok(line)
    }

    fn from_line(line: &str, number: usize) -> Result<Self, Refusal> {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 17 {
            return Err(Refusal::MalformedLine { line: number });
        }
        if fields[0] != SCHEMA {
            return Err(Refusal::WrongSchema { found: fields[0].to_string() });
        }
        let seq = fields[1].parse().map_err(|_| Refusal::MalformedLine { line: number })?;
        let at = fields[5].parse().map_err(|_| Refusal::MalformedLine { line: number })?;
        let exit_status = fields[12].parse().map_err(|_| Refusal::MalformedLine { line: number })?;
        let checks = fields[13].parse().map_err(|_| Refusal::MalformedLine { line: number })?;
        let kind = Kind::parse(fields[7]).ok_or(Refusal::MalformedLine { line: number })?;
        Ok(Entry {
            seq,
            prev: fields[2].to_string(),
            hash: fields[3].to_string(),
            operation_id: fields[4].to_string(),
            at,
            actor: fields[6].to_string(),
            kind,
            authority_ref: fields[8].to_string(),
            subject: fields[9].to_string(),
            digest: fields[10].to_string(),
            evidence: Evidence {
                command: fields[11].to_string(),
                exit_status,
                checks,
                surface: fields[14].to_string(),
                digest: fields[15].to_string(),
            },
            outcome: fields[16].to_string(),
        })
    }
}

/// A chain held in memory, read from and rendered to the durable line form.
pub struct Ledger {
    entries: Vec<Entry>,
}

impl Default for Ledger {
    fn default() -> Self {
        Ledger::new()
    }
}

impl Ledger {
    pub fn new() -> Self {
        Ledger { entries: Vec::new() }
    }

    /// Adopt a chain read from disk, verifying it before anything may append to it.
    pub fn from_entries(entries: Vec<Entry>) -> Result<Self, Refusal> {
        let ledger = Ledger { entries };
        ledger.verify()?;
        Ok(ledger)
    }

    pub fn read(text: &str) -> Result<Self, Refusal> {
        let mut entries = Vec::new();
        for (index, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            entries.push(Entry::from_line(line, index + 1)?);
        }
        Ledger::from_entries(entries)
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn head_seq(&self) -> u64 {
        self.entries.last().map(|entry| entry.seq).unwrap_or(0)
    }

    pub fn head_hash(&self) -> &str {
        self.entries
            .last()
            .map(|entry| entry.hash.as_str())
            .unwrap_or(GENESIS)
    }

    pub fn find(&self, operation_id: &str) -> Option<&Entry> {
        self.entries
            .iter()
            .find(|entry| entry.operation_id == operation_id)
    }

    /// Verify every link: seq order, prev/hash wiring, and each entry's own hash.
    pub fn verify(&self) -> Result<(), Refusal> {
        if self.entries.is_empty() {
            return Ok(());
        }
        if self.entries[0].prev != GENESIS {
            return Err(Refusal::NoGenesis);
        }
        let mut expected = 1u64;
        let mut prev = GENESIS.to_string();
        for entry in &self.entries {
            if entry.seq != expected || entry.prev != prev {
                return Err(Refusal::BrokenChain { seq: entry.seq });
            }
            let request = Request {
                operation_id: entry.operation_id.clone(),
                at: entry.at,
                actor: entry.actor.clone(),
                kind: entry.kind,
                authority_ref: entry.authority_ref.clone(),
                subject: entry.subject.clone(),
                digest: entry.digest.clone(),
                evidence: entry.evidence.clone(),
                outcome: entry.outcome.clone(),
            };
            if Entry::compute_hash(entry.seq, &entry.prev, &request) != entry.hash {
                return Err(Refusal::BrokenChain { seq: entry.seq });
            }
            prev = entry.hash.clone();
            expected += 1;
        }
        Ok(())
    }

    /// Append a request whose writer saw `expected_seq`. A retry of a landed operation id returns
    /// `Existing` rather than appending a second time.
    pub fn append(&mut self, expected_seq: u64, request: Request) -> Result<Appended, Refusal> {
        if let Some(existing) = self.find(&request.operation_id) {
            return Ok(Appended::Existing(existing.seq));
        }
        if request.operation_id.trim().is_empty() {
            return Err(Refusal::EmptyOperationId);
        }
        let required = [
            ("actor", request.actor.as_str()),
            ("authority_ref", request.authority_ref.as_str()),
            ("subject", request.subject.as_str()),
            ("evidence.command", request.evidence.command.as_str()),
            ("evidence.surface", request.evidence.surface.as_str()),
        ];
        for (name, value) in required {
            if value.trim().is_empty() {
                return Err(Refusal::MissingField(name));
            }
        }
        let digests = [
            ("digest", request.digest.as_str()),
            ("evidence.digest", request.evidence.digest.as_str()),
        ];
        for (name, value) in digests {
            if !sha256::is_sha256_hex(value) {
                return Err(Refusal::DigestNotSha256 { field: name });
            }
        }
        if expected_seq != self.head_seq() {
            return Err(Refusal::StaleSeq {
                expected: expected_seq,
                head: self.head_seq(),
            });
        }
        let seq = self.head_seq() + 1;
        let prev = self.head_hash().to_string();
        let hash = Entry::compute_hash(seq, &prev, &request);
        self.entries.push(Entry {
            seq,
            prev,
            hash,
            operation_id: request.operation_id,
            at: request.at,
            actor: request.actor,
            kind: request.kind,
            authority_ref: request.authority_ref,
            subject: request.subject,
            digest: request.digest,
            evidence: request.evidence,
            outcome: request.outcome,
        });
        Ok(Appended::New)
    }

    /// The whole chain in its durable line form.
    pub fn render(&self) -> Result<String, Refusal> {
        let mut out = String::new();
        for entry in &self.entries {
            out.push_str(&entry.to_line()?);
            out.push('\n');
        }
        Ok(out)
    }
}
