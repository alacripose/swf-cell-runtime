//! The headless inspection: a mounted frame drawn, its sounds decoded and played, without a window.
//!
//! It reads the package's frame document through the package grammar, draws it into a raster, and
//! takes every carried sound payload through the client's audio channel: decode into samples, queue,
//! play. The lines it prints are the proof — the cells that were drawn, and per sound the encoding,
//! rate, channels, frames and samples that actually came out of the decoder, plus anything that could
//! not be decoded and why.
//!
//! `--render <package> <frame> <out.ppm>` is the picture half of the same proof: the package is
//! adopted and mounted the way the window does it, the frame is drawn through the client's own
//! `draw::rasters`, and the result is painted into the client's own RGBA8 `Picture` and written as a
//! binary PPM. The pixels in that file are the pixels the window presents, so "the client renders
//! imported content" is a file that can be looked at rather than a claim about code.

use planet_client::audio::Sink;
use planet_client::palette::Palette;
use planet_client::{draw, picture, CellMount};
use planet_package::cellframe::CellFrame;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--render") {
        return render(&args[1..]);
    }
    if args.first().map(String::as_str) == Some("--evidence") {
        return evidence(&args[1..]);
    }
    if args.is_empty() {
        eprintln!("usage: inspect <frame.cellframe>");
        eprintln!("       inspect --render <package.pkg> <frame.cellframe> <out.ppm>");
        return std::process::ExitCode::from(2);
    }
    let path = &args[0];
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("inspect: read {path}: {error}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let frame = match CellFrame::parse(&text) {
        Ok(frame) => frame,
        Err(refusals) => {
            eprintln!(
                "inspect: frame refused {} {}",
                refusals.code(),
                refusals.message()
            );
            return std::process::ExitCode::FAILURE;
        }
    };

    // Each draw row's paper, resolved against the package's declared colours and the format's own
    // roles: the same question a window asks, with the answer reported instead of presented.
    let mut unresolved = 0usize;
    for fill in &frame.fills {
        let declared = frame.colours.iter().any(|(name, _)| *name == fill.paper)
            || planet_package::cellframe::PAPERS.contains(&fill.paper.as_str());
        if !declared {
            unresolved += 1;
        }
    }
    let painted: usize = frame.fills.iter().map(|fill| fill.cols as usize).sum();
    println!(
        "frame {} window {} cols {} rows {} fills {} runs {} painted {} unresolved {}",
        frame.id,
        frame.window,
        frame.cols,
        frame.rows,
        frame.fills.len(),
        frame.runs.len(),
        painted,
        unresolved
    );

    // **What the package states it does not carry.** These are rows, not pixels: a sprite with no
    // art in the file is reported by name, with the reason the package gives and the owning rule
    // it cites. The client draws nothing for it, which is the only honest answer available.
    for state in &frame.absences {
        println!(
            "sprite_absent {} {} {} rule {}",
            state.kind, state.id, state.reason, state.rule
        );
    }
    println!("sprite_absent_total {}", frame.absences.len());

    let mut sink = Sink::new();
    sink.push_all(frame.audio.iter());

    // What the channel holds, per sound: the encoding the package declares and what the decoder
    // actually produced. `declared` and `rate` are printed apart so a mismatch is visible.
    for clip in sink.decoded() {
        println!(
            "sound_decoded {} {} {} declared {} rate {} channels {} frames {} samples {} bytes {} peak {:.4} digest {}",
            clip.kind,
            clip.id,
            clip.format,
            clip.declared_rate,
            clip.rate,
            clip.channels,
            clip.frames,
            clip.samples.len(),
            clip.bytes,
            clip.peak,
            clip.digest
        );
    }
    for clip in sink.unreadable() {
        println!(
            "sound_unreadable {} {} bytes {} digest {} code {} reason {}",
            clip.kind, clip.id, clip.bytes, clip.digest, clip.code, clip.reason
        );
    }
    // The carried-bytes line keeps its old shape: it is what the importer's ledger evidence quotes.
    for clip in sink.decoded() {
        println!(
            "sound_sink {} {} {} {} {} {}",
            clip.kind, clip.id, clip.format, clip.rate, clip.bytes, clip.digest
        );
    }
    println!(
        "sound_sink_total {} {}",
        sink.decoded().len() + sink.unreadable().len(),
        sink.total_bytes()
    );

    // Playing is the step that separates "the client knows a payload" from "the client has sound":
    // the decoded samples are rendered through the channel and counted.
    let played = sink.play();
    println!(
        "sound_played frames {} samples {} peak {:.4} frames_per_sound {}",
        played.frames,
        played.samples,
        played.peak,
        if sink.decoded().is_empty() {
            0
        } else {
            played.frames / sink.decoded().len()
        }
    );
    println!(
        "sound_decoded_total {} frames {} samples {} bytes {}",
        sink.decoded().len(),
        sink.total_frames(),
        sink.total_samples(),
        sink.decoded().iter().map(|clip| clip.bytes).sum::<usize>()
    );
    println!("sound_unreadable_total {}", sink.unreadable().len());
    std::process::ExitCode::SUCCESS
}

/// **The picture, through the client's own path.** Adopt and mount the package the way the window
/// does, draw the bound frame with `draw::rasters`, paint it with `picture::paint`, and write the
/// RGBA8 buffer as a binary PPM. The counts printed are the evidence: how many cells the mount
/// answered, how many of them carry a paper, and how many distinct colours came out.
fn render(args: &[String]) -> std::process::ExitCode {
    let [package, frame, out] = args else {
        eprintln!("usage: inspect --render <package.pkg> <frame.cellframe> <out.ppm>");
        return std::process::ExitCode::from(2);
    };
    let bytes = match std::fs::read(package) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("inspect: read {package}: {error}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let document = match std::fs::read_to_string(frame) {
        Ok(document) => document,
        Err(error) => {
            eprintln!("inspect: read {frame}: {error}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let mut mount = CellMount::new();
    let adopted = match mount.adopt_frames(&bytes, &[document.as_str()], 0) {
        Ok(adopted) => adopted,
        Err(refusals) => {
            eprintln!("inspect: refused {} {}", refusals.code(), refusals.message());
            return std::process::ExitCode::FAILURE;
        }
    };
    // **This is headless, and it says so.** No window was created and no surface was presented:
    // the pixels below are the client's draw path over the mounted package, which is evidence that
    // the package is drawable, never a witness that the game drew a window. H9 stays unclaimed.
    println!("headless true window never opened — evidence the package draws, not a client witness");
    println!(
        "mounted surface {} windows {}",
        adopted.surface.id, adopted.surface.windows.len()
    );
    let rasters = match draw::rasters(&adopted, &Palette::declared()) {
        Ok(rasters) => rasters,
        Err(refusals) => {
            eprintln!("inspect: not drawn {} {}", refusals.code(), refusals.message());
            return std::process::ExitCode::FAILURE;
        }
    };
    let mut wrote = 0usize;
    for raster in &rasters {
        let picture = match picture::paint(raster) {
            Ok(picture) => picture,
            Err(refusals) => {
                eprintln!("inspect: not painted {} {}", refusals.code(), refusals.message());
                return std::process::ExitCode::FAILURE;
            }
        };
        let mut papers: Vec<(u8, u8, u8, u8)> = raster
            .raster
            .cells
            .iter()
            .map(|cell| {
                (
                    cell.paper.r,
                    cell.paper.g,
                    cell.paper.b,
                    cell.paper.a,
                )
            })
            .collect();
        papers.sort_unstable();
        papers.dedup();
        let painted = raster
            .raster
            .cells
            .iter()
            .filter(|cell| cell.paper != planet_client::palette::Rgba::TRANSPARENT)
            .count();
        println!(
            "raster {} window {} {}x{} cells painted {} papers {} picture {}x{} {}",
            raster.frame,
            raster.window,
            raster.raster.cols,
            raster.raster.rows,
            painted,
            papers.len(),
            picture.width,
            picture.height,
            picture::Picture::FORMAT
        );
        if let Err(error) = write_ppm(&picture, out) {
            eprintln!("inspect: write {out}: {error}");
            return std::process::ExitCode::FAILURE;
        }
        wrote += 1;
    }
    println!("picture_written {wrote} {out}");
    if wrote == 0 {
        eprintln!("inspect: the mount bound no window, so there is no picture");
        return std::process::ExitCode::FAILURE;
    }
    std::process::ExitCode::SUCCESS
}

/// **The headless evidence run.** Over every frame the package carries: draw it through the client's
/// own path, paint it, diff the picture against a blank surface, attribute every changed cell to the
/// imported character whose placement covers it, decode the frame's own sounds and align them to the
/// movie tick, and write all of it — plus the script lane's refusal — as one JSON artifact and one
/// PPM per frame.
///
/// It states what it is: headless. No window was created, so this is evidence that the client's draw
/// path turns the package's cells into different pixels, and it is **not** a client witness.
fn evidence(args: &[String]) -> std::process::ExitCode {
    let [package, outdir] = args else {
        eprintln!("usage: inspect --evidence <package.pkg> <outdir>");
        return std::process::ExitCode::from(2);
    };
    let bytes = match std::fs::read(package) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("inspect: read {package}: {error}");
            return std::process::ExitCode::FAILURE;
        }
    };
    if let Err(error) = std::fs::create_dir_all(outdir) {
        eprintln!("inspect: create {outdir}: {error}");
        return std::process::ExitCode::FAILURE;
    }
    // **The frames the package declares, as the client already carries them.** A package names its
    // timeline documents by path; the client reads them the same way the window does, so the evidence
    // covers every frame the mount binds rather than the first one.
    let documents = frame_documents();
    let views: Vec<&str> = documents.iter().map(String::as_str).collect();
    let mut mount = CellMount::new();
    let adopted = match mount.adopt_frames(&bytes, &views, 0) {
        Ok(adopted) => adopted,
        Err(refusals) => {
            eprintln!("inspect: refused {} {}", refusals.code(), refusals.message());
            return std::process::ExitCode::FAILURE;
        }
    };
    let script = planet_client::axis::script_state(&bytes);
    let placements = planet_client::axis::placements(&adopted.surface);
    let fps = declared_fps(&adopted.surface);
    let timeline = planet_client::Timeline::of(adopted, "stage").at_rate(fps);
    let palette = Palette::declared();
    println!(
        "headless true window never opened — evidence the package draws, not a client witness"
    );
    println!(
        "mounted surface {} frames {} fps {} script {} {}",
        adopted.surface.id,
        timeline.len(),
        fps,
        script.state,
        script.code
    );
    let frame_nanos = timeline.frame_nanos().unwrap_or(0);
    // **The score, over the frames the package carries.** Every frame's clips are decoded once; the
    // per-tick advance below starts a frame's sounds on the tick that shows it.
    let mut score = planet_client::playback::Score::prepare(timeline.frames().iter().copied());
    let mut sound_starts = 0usize;
    let mut ticks: Vec<String> = Vec::new();
    let mut changed_total = 0usize;
    let mut sound_total = 0usize;
    for (index, frame) in timeline.frames().iter().enumerate() {
        let tick = index as u64;
        let raster = match draw::raster_of("stage", frame, 1, &palette) {
            Ok(raster) => raster,
            Err(refusals) => {
                eprintln!("inspect: tick {tick} not drawn {} {}", refusals.code(), refusals.message());
                return std::process::ExitCode::FAILURE;
            }
        };
        let picture = match picture::paint(&raster) {
            Ok(picture) => picture,
            Err(refusals) => {
                eprintln!("inspect: tick {tick} not painted {} {}", refusals.code(), refusals.message());
                return std::process::ExitCode::FAILURE;
            }
        };
        let mut sink = Sink::new();
        sink.push_all(frame.audio.iter());
        let mut sounds: Vec<planet_client::axis::SoundAxis> = Vec::new();
        for clip in sink.decoded() {
            // **Tick alignment is arithmetic, not a guess.** A clip carried by frame `index` starts
            // at that frame's own offset in the movie clock, and the sample offset is that offset in
            // the clip's own sample rate.
            let start_ns = (u128::from(tick)) * frame_nanos;
            let sample_offset = (start_ns * u128::from(clip.rate) / 1_000_000_000u128) as u64;
            sounds.push(planet_client::axis::SoundAxis {
                id: clip.id.clone(),
                format: clip.format.clone(),
                rate: clip.rate,
                channels: clip.channels,
                frames: clip.frames,
                samples: clip.samples.len(),
                peak: clip.peak,
                digest: clip.digest.clone(),
                tick,
                sample_offset,
            });
        }
        for unreadable in sink.unreadable() {
            println!(
                "sound_refused tick {tick} {} code {} reason {}",
                unreadable.id, unreadable.code, unreadable.reason
            );
        }
        sound_total += sounds.len();
        let snapshot = planet_client::axis::Snapshot::read(
            frame,
            tick,
            &raster,
            &picture,
            sounds,
            script.clone(),
            &placements,
            8,
            16,
        );
        changed_total += snapshot.changed_pixels;
        println!("{}", snapshot.changed_line());
        for sound in &snapshot.sounds {
            println!(
                "sound_tick {} tick {} rate {} channels {} frames {} samples {} peak {:.4} sample_offset {}",
                sound.id, sound.tick, sound.rate, sound.channels, sound.frames, sound.samples, sound.peak, sound.sample_offset
            );
        }
        // **A frame's own sounds start on the tick that shows it**, and the samples are rendered
        // before the tick is left, so "the start produced audio" is a count and a peak rather than
        // an assertion. No device is opened: `Score::DEVICE` states that in the line itself.
        if let Some(started) = score.advance(tick, index).started {
            if !started.sounds.is_empty() {
                let played = score.render(started.samples);
                sound_starts += 1;
                println!(
                    "sound_start tick {} frame {} sounds {:?} samples {} rendered {} peak {:.4} device {}",
                    started.tick,
                    started.frame,
                    started.sounds,
                    started.samples,
                    played.samples,
                    played.peak,
                    planet_client::playback::Score::DEVICE
                );
            }
        }
        let ppm = format!("{outdir}/frame-{index:02}.ppm");
        if let Err(error) = write_ppm(&picture, &ppm) {
            eprintln!("inspect: write {ppm}: {error}");
            return std::process::ExitCode::FAILURE;
        }
        ticks.push(snapshot.json());
    }
    let report = format!(
        "{{\"kind\":\"headless-evidence/1\",\"headless\":true,\"client_witness\":false,\"note\":\"the client draw path over a mounted package; no window was opened, so this is not a client witness (AGENTS.md section 7)\",\"package\":\"{}\",\"surface\":\"{}\",\"frames\":{},\"fps\":{},\"frame_nanos\":{},\"script\":{{\"state\":\"{}\",\"code\":\"{}\",\"message\":\"{}\",\"probe\":\"{}\"}},\"bitmaps\":{{\"state\":\"not-implemented\",\"reason\":\"bitmap fills are carried as one neutral colour by the importer; no bitmap payload is decoded, so this evidence attributes sprite and shape cells, never a bitmap's pixels\"}},\"axis\":[{}]}}",
        package,
        adopted.surface.id,
        timeline.len(),
        fps,
        frame_nanos,
        script.state,
        script.code,
        script.message.replace('\\', "\\\\").replace('\"', "\\\""),
        script.probe,
        ticks.join(",")
    );
    let path = format!("{outdir}/evidence.json");
    if let Err(error) = std::fs::write(&path, &report) {
        eprintln!("inspect: write {path}: {error}");
        return std::process::ExitCode::FAILURE;
    }
    println!(
        "sound_score clips {} unreadable {} starts {sound_starts} device {}",
        score.clip_total(),
        score.unreadable_total(),
        planet_client::playback::Score::DEVICE
    );
    println!(
        "evidence {path} frames {} changed_pixels_total {changed_total} sounds {sound_total} script {}",
        ticks.len(),
        script.code
    );
    std::process::ExitCode::SUCCESS
}

/// Every frame document the client carries from `generated`, sorted by path so the order is stable.
fn frame_documents() -> Vec<String> {
    let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir("generated")
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

/// The movie's declared rate, read from the surface's own `frames at N fps` row. A surface that does
/// not state one answers zero, which `Timeline` treats as an undeclared rate rather than a default.
fn declared_fps(surface: &planet_package::surface::Surface) -> f32 {
    for window in &surface.windows {
        for node in &window.nodes {
            if let Some(position) = node.value.find(" frames at ") {
                let rest = &node.value[position + " frames at ".len()..];
                if let Some(number) = rest.split_whitespace().next() {
                    if let Ok(fps) = number.parse::<f32>() {
                        return fps;
                    }
                }
            }
        }
    }
    0.0
}

/// Write an RGBA8 picture as a binary PPM (`P6`), alpha dropped: the buffer is the client's own,
/// and the format is the plainest one that carries every pixel without a second encoder.
fn write_ppm(picture: &picture::Picture, path: &str) -> std::io::Result<()> {
    use std::io::Write;
    let mut file = std::fs::File::create(path)?;
    write!(file, "P6\n{} {}\n255\n", picture.width, picture.height)?;
    let mut rgb = Vec::with_capacity((picture.width * picture.height * 3) as usize);
    for y in 0..picture.height {
        for x in 0..picture.width {
            if let Some(pixel) = picture.pixel(x, y) {
                rgb.extend_from_slice(&pixel[..3]);
            }
        }
    }
    file.write_all(&rgb)
}
