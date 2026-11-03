//! The importer: one measured movie becomes the canonical declaration package, and its measurement
//! stages become a sealed stream when a key and a path are supplied.
//!
//! The stream is the proof Q7 asks for — a row per stage and per 1,000 measured records, flushed as
//! written, and a row that names an abort so a killed import leaves evidence instead of nothing. It
//! records the measurement, not the emission: the package and its ledger entry are the emission record.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use planet_package::{mount_and_record, Recorded};
use planet_shell_ledger::{landing, Ledger};
use swf_import::stream::{Kind, Refusal as StreamRefusal, Stream};

/// Records per sealed row inside a stage.
const CADENCE: usize = 1_000;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!("usage: swf-import <input.swf> <out.pkg> <ledger.tsv> [surface-id] [stream.jsonl]");
        eprintln!("       PLANET_STREAM_KEY=<64 hex> seals the stream when a path is given");
        return ExitCode::from(2);
    }
    let input = PathBuf::from(&args[0]);
    let out = PathBuf::from(&args[1]);
    let ledger_path = PathBuf::from(&args[2]);
    let id = args.get(3).cloned().unwrap_or_else(|| "swf1".to_string());
    let stream_path = args.get(4).map(PathBuf::from);

    let mut stream = match open_stream(stream_path.as_deref()) {
        Ok(stream) => stream,
        Err(refusal) => {
            eprintln!("stream refused: {} {}", refusal.code(), refusal.message());
            return ExitCode::FAILURE;
        }
    };

    let bytes = match std::fs::read(&input) {
        Ok(bytes) => bytes,
        Err(error) => return abort(stream, "stream-io", format!("read {}: {error}", input.display())),
    };
    let header = match swf_import::header::read(&bytes) {
        Ok(header) => header,
        Err(refusal) => {
            return abort_stream(stream, refusal.code());
        }
    };
    let tags = match swf_import::tags::walk(&header) {
        Ok(tags) => tags,
        Err(refusal) => {
            return abort_stream(stream, refusal.code());
        }
    };
    let inventory = match swf_import::content::survey(&header, &tags) {
        Ok(inventory) => inventory,
        Err(refusal) => {
            return abort_stream(stream, refusal.code());
        }
    };

    if let Some(stream) = stream.as_mut() {
        if let Err(refusal) = record_stage(stream, &tags) {
            eprintln!("stream refused: {} {}", refusal.code(), refusal.message());
            return ExitCode::FAILURE;
        }
    }

    // The display list, lowered to the frame's cells from the shape bounds the survey measured. A
    // record the reader cannot parse is counted and declared; only a truncated stream aborts.
    let (placements, unreadable) = match swf_import::placement::survey_partial(&tags, &header.body) {
        Ok(found) => found,
        Err(refusal) => return abort_stream(stream, refusal.code()),
    };
    let placements = swf_import::placement::by_depth(&placements);
    let (grid_cols, grid_rows) = swf_import::emit::grid(&header);
    let mut lowering = swf_import::emit::lower(&placements, &inventory, grid_cols, grid_rows);
    lowering.refused = unreadable;
    lowering.placements += unreadable;

    let counts = swf_import::tags::counts(&tags);
    let frame = match swf_import::emit::package_frame(
        &id,
        &header,
        &counts,
        tags.len(),
        &inventory,
        &lowering,
    ) {
            Ok(frame) => frame,
            Err(refusal) => {
                return abort_stream(stream, refusal.code());
            }
        };
    if let Some(parent) = out.parent() {
        if let Err(error) = std::fs::create_dir_all(parent) {
            return abort(stream, "stream-io", format!("mkdir {}: {error}", parent.display()));
        }
    }
    if let Err(error) = std::fs::write(&out, frame.encode()) {
        return abort(stream, "stream-io", format!("write {}: {error}", out.display()));
    }
    // The frame the stage window binds, beside the package: a package that declares a window and
    // ships nothing for it draws an empty stage, and that is a different claim from drawing it.
    let frame_path = out.with_extension("frame");
    let frame_bytes = match swf_import::emit::frame_bytes(&header, &lowering) {
        Ok(bytes) => bytes,
        Err(refusal) => return abort_stream(stream, refusal.code()),
    };
    if let Err(error) = std::fs::write(&frame_path, frame_bytes) {
        return abort(stream, "stream-io", format!("write {}: {error}", frame_path.display()));
    }

    let mut ledger = match load(&ledger_path) {
        Ok(ledger) => ledger,
        Err(why) => return abort(stream, "import-aborted", format!("ledger {}: {why}", ledger_path.display())),
    };
    let sequence = ledger.head_seq() + 1;
    match mount_and_record(
        &frame,
        &id,
        sequence,
        sequence - 1,
        1_760_000_000,
        &mut ledger,
        &ledger_path,
        sequence - 1,
    ) {
        Ok(Recorded::Landed { receipt, seq }) | Ok(Recorded::AlreadyRecorded { receipt, seq }) => {
            println!("compression {}", header.compression.name());
            println!("version {}", header.version);
            println!("declared_len {}", header.declared_len);
            println!(
                "stage {}x{} px",
                header.stage.width_px(),
                header.stage.height_px()
            );
            println!("frames {} at {} fps", header.frame_count, header.frame_rate);
            println!("show_frames {}", inventory.show_frames);
            println!("tags {}", tags.len());
            for (code, count) in &counts {
                println!("tag {code} {count}");
            }
            println!("shapes {}", inventory.shapes.len());
            if let Some(bounds) = inventory.bounds() {
                println!(
                    "shape_bounds {}x{} px at ({},{})",
                    bounds.width_px(),
                    bounds.height_px(),
                    bounds.x_min / 20,
                    bounds.y_min / 20
                );
            }
            let nested: usize = inventory.sprites.iter().map(|sprite| sprite.nested).sum();
            println!("sprites {} nested {}", inventory.sprites.len(), nested);
            for sprite in &inventory.sprites {
                println!(
                    "sprite id {} frames {} nested {} bytes {}",
                    sprite.id, sprite.frames, sprite.nested, sprite.bytes
                );
            }
            for kind in [
                swf_import::content::MediaKind::Bitmap,
                swf_import::content::MediaKind::Sound,
                swf_import::content::MediaKind::Font,
                swf_import::content::MediaKind::Text,
            ] {
                println!(
                    "media_{} {} {} bytes",
                    kind.name(),
                    inventory.media_count(kind),
                    inventory.media_bytes(kind)
                );
            }
            println!("surface {}", receipt.surface);
            println!("windows {}", receipt.windows);
            println!("digest {}", receipt.digest);
            println!("seq {seq}");
            println!("package {}", out.display());
            println!("frame {}", frame_path.display());
            println!("placements {}", lowering.summary());
            if let Some(stream) = stream.as_ref() {
                println!("stream {}", stream.path().display());
            }
            ExitCode::SUCCESS
        }
        Err(refusal) => abort_stream(stream, &refusal.code()),
    }
}

/// A row per 1,000 measured records inside the stage, then the row that closes it.
fn record_stage(stream: &mut Stream, tags: &[swf_import::Tag]) -> Result<(), StreamRefusal> {
    for (index, chunk) in tags.chunks(CADENCE).enumerate() {
        let start = index * CADENCE;
        let end = start + chunk.len() - 1;
        stream.row(Kind::Record, &format!("tags {start}-{end}"))?;
    }
    stream.row(Kind::Stage, &format!("tags {} measured", tags.len()))?;
    stream.finish()?;
    Ok(())
}

/// The stream needs a path and a key together; one without the other is refused by name rather than
/// silently dropping the proof.
fn open_stream(path: Option<&Path>) -> Result<Option<Stream>, StreamRefusal> {
    let Some(path) = path else {
        return Ok(None);
    };
    let Some(hex) = std::env::var("PLANET_STREAM_KEY").ok().filter(|value| !value.is_empty()) else {
        return Err(StreamRefusal::Order(
            "PLANET_STREAM_KEY is not set, so the stream cannot be sealed".to_string(),
        ));
    };
    let key = planet_aead::Key::from_hex(&hex).map_err(StreamRefusal::Seal)?;
    Stream::create(path, key).map(Some)
}

/// Abort the stream with a named code, say why, and fail.
fn abort_stream(stream: Option<Stream>, code: &str) -> ExitCode {
    abort(stream, code, format!("refused: {code}"))
}

fn abort(mut stream: Option<Stream>, code: &str, why: String) -> ExitCode {
    eprintln!("{why}");
    if let Some(stream) = stream.as_mut() {
        match stream.abort(code) {
            Ok(row) => eprintln!("stream marked truncated at row {}", row.seq),
            Err(refusal) => eprintln!("stream refused: {} {}", refusal.code(), refusal.message()),
        }
    }
    ExitCode::FAILURE
}

fn load(path: &Path) -> Result<Ledger, String> {
    if !path.exists() {
        return Ok(Ledger::new());
    }
    match landing::load(path) {
        Ok(Ok(ledger)) => Ok(ledger),
        Ok(Err(refusal)) => Err(format!("{} {}", refusal.code(), refusal.message())),
        Err(error) => Err(error.to_string()),
    }
}
