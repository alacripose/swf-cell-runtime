use std::path::{Path, PathBuf};
use std::process::ExitCode;

use planet_package::{mount_and_record, Recorded};
use planet_shell_ledger::{landing, Ledger};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!("usage: swf-import <input.swf> <out.pkg> <ledger.tsv> [surface-id]");
        return ExitCode::from(2);
    }
    let input = PathBuf::from(&args[0]);
    let out = PathBuf::from(&args[1]);
    let ledger_path = PathBuf::from(&args[2]);
    let id = args.get(3).cloned().unwrap_or_else(|| "swf1".to_string());

    let bytes = match std::fs::read(&input) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("read {}: {error}", input.display());
            return ExitCode::FAILURE;
        }
    };
    let header = match swf_import::header::read(&bytes) {
        Ok(header) => header,
        Err(refusal) => {
            eprintln!("refused {refusal}");
            return ExitCode::FAILURE;
        }
    };
    let tags = match swf_import::tags::walk(&header) {
        Ok(tags) => tags,
        Err(refusal) => {
            eprintln!("refused {refusal}");
            return ExitCode::FAILURE;
        }
    };
    let inventory = match swf_import::content::survey(&header, &tags) {
        Ok(inventory) => inventory,
        Err(refusal) => {
            eprintln!("refused {refusal}");
            return ExitCode::FAILURE;
        }
    };
    let counts = swf_import::tags::counts(&tags);
    let frame =
        match swf_import::emit::package_frame(&id, &header, &counts, tags.len(), &inventory) {
            Ok(frame) => frame,
            Err(refusal) => {
                eprintln!("refused {refusal}");
                return ExitCode::FAILURE;
            }
        };
    if let Some(parent) = out.parent() {
        if let Err(error) = std::fs::create_dir_all(parent) {
            eprintln!("mkdir {}: {error}", parent.display());
            return ExitCode::FAILURE;
        }
    }
    if let Err(error) = std::fs::write(&out, frame.encode()) {
        eprintln!("write {}: {error}", out.display());
        return ExitCode::FAILURE;
    }

    let mut ledger = match load(&ledger_path) {
        Ok(ledger) => ledger,
        Err(why) => {
            eprintln!("ledger {}: {why}", ledger_path.display());
            return ExitCode::FAILURE;
        }
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
            ExitCode::SUCCESS
        }
        Err(refusal) => {
            eprintln!("mount refused: {} {}", refusal.code(), refusal.message());
            ExitCode::FAILURE
        }
    }
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
