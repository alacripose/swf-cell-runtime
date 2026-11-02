//! The mount receipt, in the shape `secure-shell` vendors as `surface.mount/2`: a receipt about a
//! push, carrying `surface`, `sequence`, `digest`, `state`, `windows`, `at`. No field is added to it.
//!
//! There is one path through this module and it parses the payload with [`Surface::parse`], so a frame
//! carrying a row or key the grammar does not define is refused before any receipt exists. No lenient
//! scan is kept beside it.

use planet_shell_ledger::sha256;

use crate::frame::{frame_digest, Frame, Kind, Refusal};
use crate::surface::Surface;

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
    let parsed = Surface::parse(&frame.payload)?;
    if parsed.id != surface {
        return Err(Refusal::SurfaceSchema {
            found: format!("surface {:?}", parsed.id),
        });
    }
    if parsed.windows.is_empty() {
        return Err(Refusal::SurfaceWindows { found: 0 });
    }
    Ok(Receipt {
        surface: parsed.id,
        sequence,
        digest: sha256::hex(&frame_digest(&frame.encode())),
        state: "loaded".to_string(),
        windows: parsed.windows.len() as u32,
        at,
    })
}
