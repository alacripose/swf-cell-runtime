//! The XML client format: a window over a window definition.
//!
//! Takes the path in the first argument, or the newest `.xml` in `windows/`, lowers it through
//! `planet_client::xml::parse`, and draws the same scene shape the wgpu client draws — so the two
//! formats are compared on one renderer and one package. An undeclared element or attribute is shown
//! by name instead of being passed through.

use std::path::PathBuf;
use std::time::SystemTime;

use planet_client::render::{self, Scene};
use planet_client::xml;

fn newest_definition() -> Option<PathBuf> {
    let dir = PathBuf::from("windows");
    let mut best: Option<(SystemTime, PathBuf)> = None;
    for entry in std::fs::read_dir(dir).ok()? {
        let path = entry.ok()?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("xml") {
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

fn lowered() -> Scene {
    let path = match std::env::args().nth(1).map(PathBuf::from) {
        Some(path) => path,
        None => match newest_definition() {
            Some(path) => path,
            None => {
                return Scene::new(
                    "no window definition".to_string(),
                    vec!["pass a path, or place a .xml in windows/".to_string()],
                )
            }
        },
    };
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => return Scene::new(format!("{}: {error}", path.display()), Vec::new()),
    };
    match xml::parse(&text) {
        Ok(window) => window.scene(),
        Err(refusal) => Scene::new(
            format!("refused {} — {}", refusal.code(), path.display()),
            vec![refusal.message()],
        ),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    render::run(lowered())
}
