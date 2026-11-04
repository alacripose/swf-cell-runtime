//! The wgpu client format: a window over a mounted package, driven by the package's cadence.
//!
//! Reads the newest `.pkg` in `generated/` and every `.frame` beside it, adopts and mounts them, then
//! shows the scene the mount answers and the frames the package carries. **The loader serves any
//! carried frame over the tick**: the loop offers tick opportunities as fast as the process is
//! scheduled, `planet_package::cadence` converts elapsed time into ticks, the tick selects a carried
//! frame (wrapping over however many the package carries), and that frame is planned into a batch the
//! first time a tick lands on it. Drawing is not on that ladder — it happens every loop turn, between
//! ticks, and never advances one.
//!
//! Nothing here parses a frame, a surface or a package, and nothing here pairs a window with a frame:
//! the mount did that.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use planet_client::input::Router;
use planet_client::palette::{Palette, Rgba};
use planet_client::render::{self, FrameBatch, FramePlan, Scene};
use planet_client::{batch, draw, picture, scene, CellMount, Timeline};

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

/// The frame rate the scene already states, e.g. `frames 12 at 40 fps`. The rate is the movie's own
/// number, carried from the package by the scene lines; this only reads it back out.
fn declared_fps(lines: &[String]) -> f32 {
    for line in lines {
        if let Some(rest) = line.split(" at ").nth(1) {
            if let Some(number) = rest.split_whitespace().next() {
                if let Ok(fps) = number.parse::<f32>() {
                    return fps;
                }
            }
        }
    }
    0.0
}

struct Opened {
    scene: Scene,
    /// **Every frame the package carries**, in the order the package advances them. The loader plans
    /// each one when a tick first lands on it, so a full game's timeline is served over the tick
    /// rather than only the frames a run can afford to plan at startup.
    carried: Vec<planet_package::CellFrame>,
    plan: FramePlan,
    fps: f32,
    router: Option<Router>,
}

fn opened() -> Opened {
    let none = |title: String, lines: Vec<String>| Opened {
        scene: Scene::new(title, lines),
        carried: Vec::new(),
        plan: Box::new(|frame: &planet_package::CellFrame| {
            Err(format!("frame {} has no package", frame.id))
        }),
        fps: 0.0,
        router: None,
    };
    let Some(path) = newest(Path::new("generated"), "pkg") else {
        return none(
            "no package in generated/".to_string(),
            vec![
                "run the importer first: swf-import <input.swf> generated/swf1.pkg generated/mount-ledger.tsv swf1"
                    .to_string(),
            ],
        );
    };
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) => return none(format!("{}: {error}", path.display()), Vec::new()),
    };
    let documents = frame_documents();
    let views: Vec<&str> = documents.iter().map(String::as_str).collect();
    let mut mount = CellMount::new();
    let adopted = match mount.adopt_frames(&bytes, &views, 0) {
        Ok(adopted) => adopted,
        Err(refusals) => return none(format!("refused {}", refusals.code()), vec![refusals.message()]),
    };
    let router = Router::new(&adopted.surface);
    let mut shown = scene::from_adopted(adopted);
    let fps = declared_fps(&shown.lines);
    let timeline = Timeline::of(adopted, "stage").at_rate(fps);
    eprintln!(
        "timeline {} frames at {fps} fps ({} rate units, fixed point), one tick per movie frame, {} ns a frame; the window draws at the display's refresh",
        timeline.len(),
        timeline.frame_rate().units_value(),
        timeline.frame_nanos().unwrap_or(0)
    );

    // **Every carried frame, cloned out of the mount.** The clock wraps over this list, so the tick
    // can ask for any frame the package carries: the frames are the package's, and the loader has no
    // horizon of its own.
    let carried: Vec<planet_package::CellFrame> = timeline
        .frames()
        .iter()
        .map(|frame| (*frame).clone())
        .collect();

    // The first frame is drawn once for the report, so a run states the cells, the picture and the
    // batch it will hand the device; the rest are planned lazily, when the tick first lands on them.
    let displayed = Palette::declared();
    if let Some(frame) = timeline.frames().first() {
        match draw::raster_of("stage", frame, 1, &displayed) {
            Ok(raster) => {
                shown.lines.push(format!(
                    "raster {} by {} cells from {}",
                    raster.raster.cols, raster.raster.rows, raster.frame
                ));
                // The drawn cells themselves, on stdout: the window shows the picture, and this is
                // the same picture stated as cells and colours so a run can be checked without
                // looking at it.
                let painted: Vec<&planet_client::Cell> = raster
                    .raster
                    .cells
                    .iter()
                    .filter(|cell| cell.paper != Rgba::TRANSPARENT)
                    .collect();
                eprintln!("drawn {} cells in {}", painted.len(), raster.frame);
                for cell in painted.iter().take(8) {
                    eprintln!(
                        "cell {} {} rgba {} {} {} {}",
                        cell.x, cell.y, cell.paper.r, cell.paper.g, cell.paper.b, cell.paper.a
                    );
                }
                match (picture::paint(&raster), batch::plan(&raster)) {
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
                    }
                    (Err(refusals), _) | (_, Err(refusals)) => shown.lines.push(format!(
                        "not painted: {} — {}",
                        refusals.code(),
                        refusals.message()
                    )),
                }
            }
            Err(refusals) => shown.lines.push(format!(
                "not drawn: {} — {}",
                refusals.code(),
                refusals.message()
            )),
        }
    }
    eprintln!(
        "frames {} carried; each is planned the first tick it is drawn",
        carried.len()
    );
    // **No program is admitted, and the run says so.** A `code` frame is still refused at mount
    // (`frame-kind-reserved`, AGENTS.md §12), so the code lane's driver runs with nothing and the
    // timeline is the clock's alone. Nothing here invents a goto, a stop or a sound to fill the gap.
    eprintln!(
        "script none: no admitted code frame in the package, so every tick is the clock's (AGENTS.md §12)"
    );

    // **The plan is the same path for every frame**: the mount's cells, the palette's colours, the
    // picture and the batch. A frame the palette refuses answers the refusal, never a blank picture.
    let plan: FramePlan = Box::new(move |frame: &planet_package::CellFrame| {
        let palette = Palette::declared();
        let raster = draw::raster_of("stage", frame, 1, &palette)
            .map_err(|refusals| format!("{} {}", refusals.code(), refusals.message()))?;
        let planned = batch::plan(&raster)
            .map_err(|refusals| format!("{} {}", refusals.code(), refusals.message()))?;
        Ok(FrameBatch {
            id: frame.id.clone(),
            quads: planned.quads.iter().copied().collect(),
        })
    });
    Opened {
        scene: shown,
        carried,
        plan,
        fps,
        router: Some(router),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // **The debug console comes first, and its absence is a refusal.** A game window whose debug
    // output has nowhere to appear is refused, not opened: the console is part of the client, and
    // dropping it is the one change the gate below refuses.
    let _console = planet_client::console::DebugConsole::require()?;
    let Opened {
        scene,
        carried,
        plan,
        fps,
        router,
    } = opened();
    render::run_with_timeline(scene, carried, plan, fps, router, None)
}
