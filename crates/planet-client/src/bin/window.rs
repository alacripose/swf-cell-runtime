//! The wgpu client format: a window over a mounted package.
//!
//! Reads the newest `.pkg` in `generated/` and every `.frame` beside it, adopts and mounts them, then
//! shows the scene the mount answers, the cells the palette resolved, the picture those cells paint
//! and the batch that picture projects into. The batch is handed to the window: the one device path in
//! the client draws it as coloured instances before the text. Nothing here parses a frame, a surface
//! or a package, and nothing here pairs a window with a frame: the mount did that.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use planet_client::batch::Quad;
use planet_client::palette::Palette;
use planet_client::render::{self, Scene};
use planet_client::{batch, draw, picture, scene, CellMount};

fn newest(dir: &Path, extension: &str) -> Option<PathBuf> {
    let mut best: Option<(SystemTime, PathBuf)> = None;
    for entry in std::fs::read_dir(dir).ok()? {
        let path = entry.ok()?.path();
        if path.extension().and_then(|e| e.to_str()) != Some(extension) {
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

/// Every frame document beside the package, in file-name order.
fn frame_documents() -> Vec<String> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir("generated")
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("frame"))
        .collect();
    paths.sort();
    paths
        .iter()
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .collect()
}

/// The scene to show, and the batch the window should present over it.
fn opened() -> (Scene, Vec<Quad>) {
    let Some(path) = newest(Path::new("generated"), "pkg") else {
        return (
            Scene::new(
                "no package in generated/".to_string(),
                vec!["run the importer first: swf-import <input.swf> generated/swf1.pkg generated/mount-ledger.tsv swf1".to_string()],
            ),
            Vec::new(),
        );
    };
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return (
                Scene::new(format!("{}: {error}", path.display()), Vec::new()),
                Vec::new(),
            )
        }
    };
    let frames = frame_documents();
    let documents: Vec<&str> = frames.iter().map(String::as_str).collect();
    let mut mount = CellMount::new();
    let adopted = match mount.adopt_frames(&bytes, &documents, 0) {
        Ok(adopted) => adopted,
        Err(refusals) => {
            return (
                Scene::new(
                    format!("refused {}", refusals.code()),
                    vec![refusals.message()],
                ),
                Vec::new(),
            )
        }
    };
    let mut shown = scene::from_adopted(adopted);
    let mut quads: Vec<Quad> = Vec::new();
    match draw::rasters(adopted, &Palette::declared()) {
        Ok(rasters) => {
            for raster in &rasters {
                shown.lines.push(format!(
                    "raster {} by {} cells from {}",
                    raster.raster.cols, raster.raster.rows, raster.frame
                ));
                match (picture::paint(raster), batch::plan(raster)) {
                    (Ok(picture), Ok(planned)) => {
                        shown.lines.push(format!(
                            "picture {} by {} {} (stride {})",
                            picture.width,
                            picture.height,
                            picture::Picture::FORMAT,
                            picture.stride
                        ));
                        shown.lines.push(format!(
                            "batch {} quads for {}",
                            planned.quads.len(),
                            raster.frame
                        ));
                        quads.extend(planned.quads.iter().copied());
                    }
                    (Err(refusals), _) | (_, Err(refusals)) => shown.lines.push(format!(
                        "not painted: {} — {}",
                        refusals.code(),
                        refusals.message()
                    )),
                }
            }
        }
        Err(refusals) => shown.lines.push(format!(
            "not drawn: {} — {}",
            refusals.code(),
            refusals.message()
        )),
    }
    (shown, quads)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (shown, quads) = opened();
    render::run_with_batch(shown, quads)
}
