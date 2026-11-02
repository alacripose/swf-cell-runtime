//! The mount receipt, in the shape `secure-shell` vendors as `surface.mount/2`: a receipt about a
//! push, carrying `surface`, `sequence`, `digest`, `state`, `windows`, `at`. No field is added to it.
//!
//! What this module checks is the boundary, not the surface grammar: the frame's kind, the sequence,
//! the document's own schema row, its surface id, and that it declares at least one window. Unknown
//! rows and keys inside a `dl-enclave.surface/1` document are a later unit; this module does not claim
//! to refuse them.

use planet_shell_ledger::sha256;

use crate::frame::{frame_digest, Frame, Kind, Refusal};

pub const SURFACE_SCHEMA: &str = "dl-enclave.surface/1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Receipt {
    pub surface: String,
    pub sequence: u64,
    pub digest: String,
    pub state: String,
    pub windows: u32,
    pub at: u64,
}

/// Mount a declaration frame and answer with its receipt. A `code` frame packs and is refused here.
pub fn mount(
    frame: &Frame,
    surface: &str,
    sequence: u64,
    previous_sequence: u64,
    at: u64,
) -> Result<Receipt, Refusal> {
    if frame.kind == Kind::Code {
        return Err(Refusal::FrameKindReserved { kind: "code" });
    }
    if sequence != previous_sequence + 1 {
        return Err(Refusal::MountSequence {
            previous: previous_sequence,
            found: sequence,
        });
    }
    if !frame.payload.ends_with(b"\n") {
        return Err(Refusal::SurfaceSchema {
            found: "no final newline".to_string(),
        });
    }
    let text = std::str::from_utf8(&frame.payload).map_err(|_| Refusal::SurfaceSchema {
        found: "not UTF-8".to_string(),
    })?;
    let header = text.lines().next().unwrap_or("");
    if header != SURFACE_SCHEMA {
        return Err(Refusal::SurfaceSchema {
            found: header.to_string(),
        });
    }
    let declared = text
        .lines()
        .find_map(|line| line.strip_prefix("surface\t"))
        .unwrap_or("");
    if declared != surface {
        return Err(Refusal::SurfaceSchema {
            found: format!("surface {declared:?}"),
        });
    }
    let windows = text
        .lines()
        .filter(|line| line.starts_with("window\t"))
        .count();
    if windows == 0 {
        return Err(Refusal::SurfaceWindows { found: 0 });
    }
    Ok(Receipt {
        surface: surface.to_string(),
        sequence,
        digest: sha256::hex(&frame_digest(&frame.encode())),
        state: "loaded".to_string(),
        windows: windows as u32,
        at,
    })
}
