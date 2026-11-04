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

    // **The timeline, read frame by frame.** A movie is not one display list: each `ShowFrame`
    // closes a frame, and a `RemoveObject`/`RemoveObject2` takes a character off it again. The
    // package ships one frame, so it ships the state the movie **ends** in — the display list every
    // placement and removal up to the last `ShowFrame` leaves in force — and the report names which
    // frame that is. Reading the tags as one list instead let a curtain the movie removes stand in
    // for the scene: a frame composited out of records the file had already taken away.
    let timeline = match swf_import::placement::timeline(&tags, &header.body) {
        Ok(states) => states,
        Err(refusal) => return abort_stream(stream, refusal.code()),
    };
    let (grid_cols, grid_rows) = swf_import::emit::grid(&header);
    // **Which frame the package ships.** A movie is a timeline and a package frame is one picture, so
    // one frame is chosen and the choice is stated. The rule is the picture itself: the frame whose
    // *visible* cells carry the most distinct colours, the later frame winning a tie. It selects a
    // frame the file draws; it never alters one, and the report names the frame that was chosen.
    // **Every timeline frame is lowered and shipped.** The package binds the first one and carries
    // the rest as sibling frame artifacts in order, so the client has a cadence to advance: a
    // one-frame package is a movie that shows its first frame forever.
    let mut lowerings: Vec<swf_import::emit::Lowering> = timeline
        .iter()
        .enumerate()
        .map(|(index, state)| {
            // **The movie's frame index is handed to the walk**, so a sprite draws its own frame of
            // the movie rather than repeating its first one twelve times.
            swf_import::emit::lower(&state.placements, &inventory, grid_cols, grid_rows, index)
        })
        .collect();
    if lowerings.is_empty() {
        return abort_stream(stream, "timeline-empty");
    }
    let mut lowering = lowerings.remove(0);
    let shown_frame = 1usize;
    lowering.refused = timeline[0].unreadable.len();
    lowering.tolerated = timeline[0].tolerated;
    lowering.placements += lowering.refused;

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
    // **Every timeline frame is lowered and carried.** The bound frame names the rest in its
    // `[timeline]` section, each as a carried artifact with the order a client advances it in, so the
    // package ships a cadence and not one picture. That is affordable because the declaration
    // ceiling is the interim floor (8 MiB) that admits the ruled surface class: twelve frames are
    // about 1.6 MB, which the old 512 KiB constant refused and the ruling says was never the answer.
    let later: Vec<(u32, String)> = lowerings
        .iter()
        .enumerate()
        .map(|(index, later)| {
            let order = index as u32 + 2;
            (
                order,
                swf_import::emit::timeline_document(
                    &header,
                    later,
                    &format!("STAGE-{order:04}"),
                    order,
                ),
            )
        })
        .collect();
    // **The sounds the movie starts, and on which frame.** The event names a character the audio
    // section already carries; a package with payloads and no events holds audio nothing can place.
    let mut events: Vec<(u32, String)> = Vec::new();
    for (index, state) in timeline.iter().enumerate() {
        let order = index as u32 + 1;
        for sound in &state.sounds {
            // A stream block has no character id; the audio table names its clip `-`, and an event
            // names the same. A definition character is named by its own id.
            let id = if *sound == 0 {
                "-".to_string()
            } else {
                sound.to_string()
            };
            // Only a sound the audio table carries can be an event: an event naming a payload the
            // package does not carry is a moment the client could never play, and the grammar
            // refuses it by name.
            if !inventory
                .sounds
                .iter()
                .any(|clip| clip.id.map(|id| id.to_string()).unwrap_or_else(|| "-".to_string()) == id)
            {
                continue;
            }
            events.push((order, id));
        }
    }
    let frame_bytes = match swf_import::emit::frame_bytes(
        &header,
        &lowering,
        &inventory.sounds,
        &inventory.sprites,
        &later,
        &events,
           &inventory.actions,
    ) {
        Ok(bytes) => bytes,
        Err(refusal) => {
            return abort(
                stream,
                refusal.code(),
                format!("refused: {} {}", refusal.code(), refusal.message()),
            )
        }
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
            let geometry = inventory
                .shapes
                .iter()
                .filter(|shape| shape.geometry.is_some())
                .count();
            let regions: usize = inventory
                .shapes
                .iter()
                .filter_map(|shape| shape.geometry.as_ref())
                .map(|geometry| geometry.regions.len())
                .sum();
            println!("shape_geometry {geometry} regions {regions}");
            // The definitions that did not reach an exact walk, named: a count of four says nothing
            // about which four, and each one is a place the client would draw a box.
            let incomplete: Vec<u16> = inventory
                .shapes
                .iter()
                .filter(|shape| {
                    matches!(shape.code, 2 | 22 | 32 | 83) && shape.geometry.is_none()
                })
                .map(|shape| shape.id)
                .collect();
            println!("shape_incomplete {} {:?}", incomplete.len(), incomplete);
            let with_fill = inventory
                .shapes
                .iter()
                .filter(|shape| shape.fill.is_some())
                .count();
            let mut distinct: Vec<(u8, u8, u8)> = inventory
                .shapes
                .iter()
                .filter_map(|shape| shape.fill)
                .collect();
            distinct.sort_unstable();
            distinct.dedup();
            println!("shape_fills {with_fill} distinct {}", distinct.len());
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
                    "sprite id {} frames {} nested {} codes {:?} first {} bytes {}",
                    sprite.id,
                    sprite.frames,
                    sprite.nested,
                    sprite.codes,
                    sprite.first.len(),
                    sprite.bytes
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
            println!("timeline_documents {}", later.len() + 1);
            println!("audio_events {}", events.len());
            let script_bytes: usize = inventory.actions.iter().map(|(_, bytes)| bytes.len()).sum();
            println!("script_records {} bytes {}", inventory.actions.len(), script_bytes);
            let mut op_counts: std::collections::BTreeMap<u8, usize> = Default::default();
            let mut refused_records = 0usize;
            let mut records = 0usize;
            for (index, (tag_code, bytes)) in inventory.actions.iter().enumerate() {
                records += 1;
                match swf_import::avm1::lower(*tag_code, bytes) {
                    Ok(instructions) => {
                        for instruction in instructions {
                            *op_counts.entry(instruction.code).or_default() += 1;
                        }
                    }
                    Err(refusal) => {
                        refused_records += 1;
                        println!(
                            "script_refused record {} tag {} {} {}",
                            index + 1,
                            tag_code,
                            refusal.code,
                            refusal.message
                        );
                    }
                }
            }
            for (code, count) in &op_counts {
                println!(
                    "script_opcode 0x{:02x} {} {}",
                    code,
                    swf_import::avm1::name_of(*code).unwrap_or("unknown"),
                    count
                );
            }
            println!(
                "script_lower records {} decoded {} refused {} opcodes {}",
                records,
                records - refused_records,
                refused_records,
                op_counts.len()
            );
            println!("placements {}", lowering.summary());
            // Which frame the package ships, and what the rest of the timeline holds that it does
            // not: a named gap, so "the client draws frame 1" is a statement about the package.
            println!(
                "timeline_frame {} of {} at {} fps",
                shown_frame,
                timeline.len(),
                header.frame_rate
            );
            // What each timeline frame paints, as a colour count: a picture that comes out one colour
            // is a fact about the chosen frame, and this says which frames carry more than that.
            let (grid_cols, grid_rows) = swf_import::emit::grid(&header);
            for (index, state) in timeline.iter().enumerate() {
                let probe = swf_import::emit::lower(
                    &swf_import::placement::by_depth(&state.placements),
                    &inventory,
                    grid_cols,
                    grid_rows,
                    index,
                );
                let mut papers: Vec<String> =
                    probe.cells.iter().map(|cell| cell.paper.clone()).collect();
                papers.sort();
                papers.dedup();
                println!(
                    "frame_paint {} papers {} painted {} visible {}",
                    index + 1,
                    papers.len(),
                    probe.painted,
                    swf_import::emit::visible_papers(&probe.cells, grid_cols, grid_rows)
                );
                // **What is in front, named.** A picture that comes out one colour is traced here
                // to the last cells the painter draws: the character, its depth path and its paper.
                let mut chars: std::collections::BTreeMap<u16, (usize, usize)> =
                    std::collections::BTreeMap::new();
                for cell in &probe.cells {
                    let entry = chars.entry(cell.character).or_insert((0, 0));
                    entry.0 += 1;
                    entry.1 += (cell.cols * cell.rows) as usize;
                }
                let names: Vec<String> = chars
                    .iter()
                    .map(|(id, (runs, cells))| format!("{id}:{runs}rows/{cells}cells"))
                    .collect();
                let white: usize = probe
                    .cells
                    .iter()
                    .filter(|cell| {
                        probe
                            .colours
                            .iter()
                            .any(|(role, (r, g, b))| *role == cell.paper && *r == 255 && *g == 255 && *b == 255)
                    })
                    .map(|cell| (cell.cols * cell.rows) as usize)
                    .sum();
                println!("frame_chars {} {}", index + 1, names.join(" "));
                println!("frame_white {} {}", index + 1, white);
                for cell in probe.cells.iter().rev().take(2) {
                    println!(
                        "frame_top {} character {} depth {} order {:?} paper {} at ({},{}) {}x{}",
                        index + 1,
                        cell.character,
                        cell.depth,
                        cell.order,
                        cell.paper,
                        cell.col,
                        cell.row,
                        cell.cols,
                        cell.rows
                    );
                }
            }
            // The display list the package ships, front to back: a picture that comes out one colour
            // is traced here to the depths and characters the file puts in front.
            let mut views = timeline[0].placements.clone();
            views.sort_by_key(|placement| std::cmp::Reverse(placement.depth));
            for placement in views.iter().take(10) {
                println!(
                    "view depth {} character {} transparent {}",
                    placement.depth, placement.character, placement.transparent
                );
            }
            println!(
                "timeline_states {:?}",
                timeline
                    .iter()
                    .map(|state| state.placements.len())
                    .collect::<Vec<_>>()
            );
            // The headless proof: how each placed character drew. `placeholder` is the box drawn in
            // place of geometry, so this line is what makes "no grid boxes" checkable.
            println!("placeholder {}", lowering.placeholder);
            println!("unmeasured {}", lowering.unmeasured);
            for (character, kind, cells) in &lowering.proof {
                println!("proof_placement {character} {kind} cells {cells}");
            }
            // The audio proof, read back out of the frame that was just written: the source payload's
            // digest and the carried payload's digest must agree, byte for byte.
            let carried_audio = std::fs::read_to_string(&frame_path)
                .ok()
                .and_then(|doc| planet_package::cellframe::CellFrame::parse(&doc).ok())
                .map(|frame| frame.audio)
                .unwrap_or_default();
            let mut audio_bytes = 0usize;
            for sound in &inventory.sounds {
                let id = sound
                    .id
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "-".to_string());
                let source = planet_package::cellframe::payload_digest(&sound.bytes);
                let carried = carried_audio
                    .iter()
                    .find(|clip| clip.id == id && clip.kind == sound.kind)
                    .map(|clip| planet_package::cellframe::payload_digest(&clip.bytes));
                let same = carried.as_deref() == Some(source.as_str());
                audio_bytes += sound.bytes.len();
                println!(
                    "sound_source {id} {} {source} code {:#04x} rate {} samples {}",
                    sound.bytes.len(),
                    sound.format_byte,
                    sound.rate,
                    sound.samples
                );
                println!(
                    "sound_carried {id} {} {} match {same}",
                    sound.bytes.len(),
                    carried.unwrap_or_else(|| "-".to_string())
                );
            }
            // What this import carried. Whether a client consumes it is the client's own report:
            // `planet-client`'s `inspect` binary prints one `sound_sink` row per payload it took.
            println!("sound_total {} {audio_bytes}", inventory.sounds.len());
            println!(
                "unmeasured sprites {} unknown {}",
                lowering.unmeasured_sprite, lowering.unmeasured_unknown
            );
            // A record this reader could not read is named where it is, with the reason: a count of
            // sixteen says nothing about which bytes defeated the reader.
            for (offset, code, reason) in &timeline[0].unreadable {
                println!("unreadable at {offset} tag {code} {reason}");
            }
            if !lowering.unmeasured_ids.is_empty() {
                println!("unmeasured characters {:?}", lowering.unmeasured_ids);
            }
            // Sprites whose decoded body carries no display record: they draw nothing by what the
            // file states, so they are named apart from the ones nothing could measure. The two
            // reasons are stated separately, because "declares no frame" and "has frames, none of
            // them a display record" are different facts about the file.
            let empty: Vec<u16> = inventory
                .sprites
                .iter()
                .filter(|sprite| !sprite.draws)
                .map(|sprite| sprite.id)
                .collect();
            if !empty.is_empty() {
                let frameless: Vec<u16> = inventory
                    .sprites
                    .iter()
                    .filter(|sprite| !sprite.draws && sprite.frames == 0)
                    .map(|sprite| sprite.id)
                    .collect();
                let recordless: Vec<u16> = inventory
                    .sprites
                    .iter()
                    .filter(|sprite| !sprite.draws && sprite.frames > 0)
                    .map(|sprite| sprite.id)
                    .collect();
                println!("empty sprites {} {empty:?}", empty.len());
                println!("frameless sprites {} {frameless:?}", frameless.len());
                println!("recordless sprites {} {recordless:?}", recordless.len());
            }
            // What the frame states it does not carry, read back out of the frame that was just
            // written: the client's report is these rows, so they are printed here beside the
            // survey's own count of empty sprites. A sprite with no art is not a cell the client
            // drew a box for; it is a row naming the character, the reason and the owning rule.
            let carried_absences = std::fs::read_to_string(&frame_path)
                .ok()
                .and_then(|doc| planet_package::cellframe::CellFrame::parse(&doc).ok())
                .map(|frame| frame.absences)
                .unwrap_or_default();
            for state in &carried_absences {
                println!(
                    "sprite_absent {kind} {id} {reason} rule {rule}",
                    kind = state.kind,
                    id = state.id,
                    reason = state.reason,
                    rule = state.rule
                );
            }
            println!("sprite_absent_total {}", carried_absences.len());
            println!("absent cells {}", lowering.absent);
            // The cells the frame carries, with the colour each one was given: a count alone does
            // not show a picture, and this is the line that does.
            for cell in lowering.cells.iter().take(8) {
                let rgb = lowering
                    .colours
                    .iter()
                    .find(|(role, _)| role == &cell.paper)
                    .map(|(_, colour)| *colour)
                    .unwrap_or((0, 0, 0));
                println!(
                    "cell {} {} {}x{} paper {} rgb {} {} {}",
                    cell.col, cell.row, cell.cols, cell.rows, cell.paper, rgb.0, rgb.1, rgb.2
                );
            }
            // What paints last is what a viewer sees. A picture that comes out one colour is traced
            // here to the record behind it: the topmost cells by the painter's path, then the
            // characters that cover the most cells at all.
            for cell in lowering.cells.iter().rev().take(6) {
                println!(
                    "top_cell character {} depth {} order {:?} paper {} at ({},{}) {}x{}",
                    cell.character,
                    cell.depth,
                    cell.order,
                    cell.paper,
                    cell.col,
                    cell.row,
                    cell.cols,
                    cell.rows
                );
            }
            let mut coverage: Vec<(u16, usize)> = Vec::new();
            for cell in &lowering.cells {
                match coverage
                    .iter_mut()
                    .find(|(character, _)| *character == cell.character)
                {
                    Some((_, count)) => *count += cell.cols as usize,
                    None => coverage.push((cell.character, cell.cols as usize)),
                }
            }
            coverage.sort_by(|left, right| right.1.cmp(&left.1));
            for (character, cells) in coverage.iter().take(6) {
                println!("draw_coverage character {character} cells {cells}");
            }
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
