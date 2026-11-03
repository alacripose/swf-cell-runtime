//! A window's scene, derived from a mounted, extent-checked package.
//!
//! The mount answers which frame belongs to which window; this module turns that answer into the
//! lines a window shows. It resolves nothing of its own: a frame's paper and ink reach the scene as
//! the role words the package declared, never as colours, and no draw row becomes a pixel here.
//!
//! [`opened`] is the path a binary uses — adopt, mount, lower — so the window never pairs a window
//! with a frame itself and never re-resolves, re-checks or re-measures one. A package whose frame
//! disagrees with its window is refused before a scene exists, in the package crate's own
//! `cellframe-*` family, with the binding row's line.

use planet_package::Refusals;

use crate::cell_mount::{Adopted, CellMount};
use crate::render::Scene;

/// Adopt a package with the frames it declares, mount it, and answer the scene it shows.
///
/// The refusal is the package crate's, unchanged: `cellframe-extent` for a frame that disagrees with
/// its window, `cellframe-unreferenced` for a frame no declaration binds, and the frame codes for bad
/// bytes. Nothing is stored and no scene exists when a refusal comes back.
pub fn opened(bytes: &[u8], frames: &[&str], at: u64) -> Result<Scene, Refusals> {
    let mut mount = CellMount::new();
    let adopted = mount.adopt_frames(bytes, frames, at)?;
    Ok(from_adopted(adopted))
}

/// The scene a mounted package shows: per window, its title, the frame the mount answered for it, and
/// the words that frame carries.
pub fn from_adopted(adopted: &Adopted) -> Scene {
    let short = adopted.digest.chars().take(12).collect::<String>();
    let mut lines = vec![format!("digest {short}")];
    for window in &adopted.surface.windows {
        lines.push(format!("window {} — {}", window.id, window.title));
        match adopted.frame_of(&window.id) {
            Some(frame) => {
                lines.push(format!(
                    "frame {} — {} by {}, order {}",
                    frame.id, frame.cols, frame.rows, frame.order
                ));
                for fill in &frame.fills {
                    lines.push(format!(
                        "paper {} at ({}, {}) for {} cells",
                        fill.paper, fill.col, fill.row, fill.cols
                    ));
                }
                for run in &frame.runs {
                    lines.push(format!("{}: {}", run.ink, run.text));
                }
            }
            None => lines.push(format!("window {} carries no frame", window.id)),
        }
        for node in &window.nodes {
            if node.kind == "label" || node.kind == "table" {
                lines.push(format!("{}: {}", node.id, node.value));
            }
        }
    }
    Scene::new(adopted.surface.id.clone(), lines)
}
