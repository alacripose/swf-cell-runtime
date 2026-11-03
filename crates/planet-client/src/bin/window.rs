//! The wgpu client format: a window over a mounted package.
//!
//! Reads the newest `.pkg` in `generated/`, adopts it through `planet_client::CellMount` (the same
//! adopt-or-refuse-whole path the mount uses), and draws the first window's title and label lines
//! through `planet_client::render`. Nothing here parses a frame, a surface or a package.

use std::path::PathBuf;
use std::time::SystemTime;

use planet_client::render::{self, Scene};
use planet_client::CellMount;

fn newest_package() -> Option<PathBuf> {
    let dir = PathBuf::from("generated");
    let mut best: Option<(SystemTime, PathBuf)> = None;
    for entry in std::fs::read_dir(dir).ok()? {
        let path = entry.ok()?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("pkg") {
            continue;
        }
        let modified = std::fs::metadata(&path).ok()?.modified().ok()?;
        let better = match &best {
            None => true,
            Some((when, name)) => (modified, &path) > (*when, name),
        };
        if better {
            best = Some((modified, path));
        }
    }
    best.map(|(_, path)| path)
}

fn adopted() -> Scene {
    let Some(path) = newest_package() else {
        return Scene::new(
            "no package in generated/".to_string(),
            vec!["run the importer first: swf-import <input.swf> generated/swf1.pkg generated/mount-ledger.tsv swf1".to_string()],
        );
    };
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) => return Scene::new(format!("{}: {error}", path.display()), Vec::new()),
    };
    let mut mount = CellMount::new();
    match mount.adopt(&bytes, 0) {
        Ok(entry) => {
            let short = entry.digest.chars().take(12).collect::<String>();
            let mut lines = vec![format!("{} — digest {short}", path.display())];
            if let Some(window) = entry.surface.windows.first() {
                lines.push(format!("window {} — {}", window.id, window.title));
                for node in &window.nodes {
                    if node.kind == "label" || node.kind == "table" {
                        lines.push(format!("{}: {}", node.id, node.value));
                    }
                }
            }
            Scene::new(entry.surface.id.clone(), lines)
        }
        Err(refusal) => Scene::new(
            format!("refused {}", refusal.code()),
            vec![refusal.message()],
        ),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    render::run(adopted())
}
