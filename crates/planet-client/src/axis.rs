//! **The read-only client axis.** A position in the mounted movie a reviewer or a bot can quote:
//! the movie tick, the frame it showed, the imported cells that painted it and the character each
//! cell is attributable to, the sounds the frame carries with the tick they start on, and the state
//! of the script lane.
//!
//! It is **read-only and it schedules nothing**. It queries a frame that the mount already resolved
//! and the client already drew, and it answers in JSON so the answer is a file rather than a claim.
//! It does not advance a tick, choose a frame, decode audio or store content — anything that would
//! make it a second runtime is refused by `AGENTS.md` §2, and `docs/AGENTS-PROPOSALS-2026-11-04.md`
//! P-6 states the boundary.
//!
//! **Two witnesses stay separate.** An axis read is evidence that the client's own draw path put
//! those cells and sounds at those positions. It is *not* a client witness: no window was opened, and
//! neither a bot's read nor this snapshot may be reported as one (`AGENTS.md` §7).

use planet_package::cellframe::CellFrame;
use planet_package::surface::{Node, Surface};

use crate::draw::WindowRaster;
use crate::palette::Rgba;
use crate::picture::Picture;

/// One imported character placement the surface declares: the character, its depth, and the cell
/// rectangle it occupies on the stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placement {
    pub character: u16,
    pub depth: u16,
    pub col: u32,
    pub row: u32,
    pub cols: u32,
    pub rows: u32,
}

/// Parse the surface's `placement-N` rows: `depth D character C at (col,row) WxH cells`.
///
/// A row that does not state those fields is skipped rather than guessed at, so attribution never
/// invents a character.
pub fn placements(surface: &Surface) -> Vec<Placement> {
    let mut found = Vec::new();
    for window in &surface.windows {
        for node in &window.nodes {
            if let Some(placement) = placement_of(node) {
                found.push(placement);
            }
        }
    }
    found
}

fn placement_of(node: &Node) -> Option<Placement> {
    if !node.id.starts_with("placement-") {
        return None;
    }
    let value = node.value.as_str();
    let after_depth = value.strip_prefix("depth ")?.trim_start();
    let mut fields = after_depth.split_whitespace();
    let depth: u16 = fields.next()?.parse().ok()?;
    if fields.next()? != "character" {
        return None;
    }
    let character: u16 = fields.next()?.parse().ok()?;
    if fields.next()? != "at" {
        return None;
    }
    let rect = fields.next()?.trim_start_matches('(');
    let (col, rest) = rect.split_once(',')?;
    let (row, rest) = rest.split_once(')')?;
    let size = rest.trim_start_matches(')').trim_start_matches(' ');
    let _ = size;
    let extent = fields.next()?;
    let (cols, rows) = extent.split_once('x')?;
    Some(Placement {
        character,
        depth,
        col: col.parse().ok()?,
        row: row.parse().ok()?,
        cols: cols.parse().ok()?,
        rows: rows.parse().ok()?,
    })
}

/// One painted cell, attributed to the imported character whose placement covers it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellAxis {
    pub col: u32,
    pub row: u32,
    pub paper: Rgba,
    /// The pixel rectangle this cell occupies in the picture: `(x, y, w, h)`.
    pub pixels: (u32, u32, u32, u32),
    pub character: Option<u16>,
    pub depth: Option<u16>,
    /// `true` when the cell changed at least one pixel from the blank surface.
    pub changed: bool,
}

/// One sound the frame carries, with the tick it is aligned to.
#[derive(Clone, Debug, PartialEq)]
pub struct SoundAxis {
    pub id: String,
    pub format: String,
    pub rate: u32,
    pub channels: u16,
    pub frames: usize,
    pub samples: usize,
    pub peak: f32,
    pub digest: String,
    pub tick: u64,
    pub sample_offset: u64,
}

/// The script lane's state, as the mount reported it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptAxis {
    /// `refused` when the mount refused a code frame, `absent` when no code frame was offered.
    pub state: &'static str,
    pub code: String,
    pub message: String,
    /// How the state was obtained, so the refusal is never a hard-coded string.
    pub probe: String,
}

/// One read of the axis: everything a reader can quote about one tick.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub tick: u64,
    pub frame: String,
    pub cols: u32,
    pub rows: u32,
    pub cells: Vec<CellAxis>,
    pub sounds: Vec<SoundAxis>,
    pub script: ScriptAxis,
    pub changed_pixels: usize,
    pub blank_pixels: usize,
}

impl Snapshot {
    /// **Read one tick.** `raster` is the client's own draw of `frame`; `picture` is that raster
    /// painted. The blank surface is the same buffer with every byte zero, which is what "nothing
    /// was drawn" means for a picture: a transparent canvas.
    pub fn read(
        frame: &CellFrame,
        tick: u64,
        raster: &WindowRaster,
        picture: &Picture,
        sounds: Vec<SoundAxis>,
        script: ScriptAxis,
        placements: &[Placement],
        cell_w: u32,
        cell_h: u32,
    ) -> Snapshot {
        let mut cells = Vec::new();
        let mut changed_pixels = 0usize;
        for cell in &raster.raster.cells {
            let x0 = cell.x * cell_w;
            let y0 = cell.y * cell_h;
            let mut changed = 0usize;
            for row in 0..cell_h {
                for col in 0..cell_w {
                    if let Some(pixel) = picture.pixel(x0 + col, y0 + row) {
                        if pixel != [0, 0, 0, 0] {
                            changed += 1;
                            changed_pixels += 1;
                        }
                    }
                }
            }
            if cell.paper == Rgba::TRANSPARENT {
                continue;
            }
            let placement = placements
                .iter()
                .find(|place| {
                    cell.x >= place.col
                        && cell.x < place.col + place.cols
                        && cell.y >= place.row
                        && cell.y < place.row + place.rows
                })
                .copied();
            cells.push(CellAxis {
                col: cell.x,
                row: cell.y,
                paper: cell.paper,
                pixels: (x0, y0, cell_w, cell_h),
                character: placement.map(|place| place.character),
                depth: placement.map(|place| place.depth),
                changed: changed > 0,
            });
        }
        Snapshot {
            tick,
            frame: frame.id.clone(),
            cols: frame.cols,
            rows: frame.rows,
            cells,
            sounds,
            script,
            changed_pixels,
            blank_pixels: (picture.width * picture.height) as usize,
        }
    }

    /// The axis as JSON. Field names are stable; a reader must not have to infer a unit.
    pub fn json(&self) -> String {
        let mut out = String::new();
        out.push('{');
        out.push_str(&format!(
            "\"tick\":{},\"frame\":\"{}\",\"cols\":{},\"rows\":{},\"changed_pixels\":{},\"picture_pixels\":{},",
            self.tick,
            escape(&self.frame),
            self.cols,
            self.rows,
            self.changed_pixels,
            self.blank_pixels
        ));
        out.push_str(&format!(
            "\"script\":{{\"state\":\"{}\",\"code\":\"{}\",\"message\":\"{}\",\"probe\":\"{}\"}},",
            self.script.state,
            escape(&self.script.code),
            escape(&self.script.message),
            escape(&self.script.probe)
        ));
        out.push_str("\"cells\":[");
        for (index, cell) in self.cells.iter().enumerate() {
            if index > 0 {
                out.push(',');
            }
            out.push_str(&format!(
                "{{\"col\":{},\"row\":{},\"paper\":[{},{},{},{}],\"pixels\":[{},{},{},{}],\"character\":{},\"depth\":{},\"changed\":{}}}",
                cell.col,
                cell.row,
                cell.paper.r,
                cell.paper.g,
                cell.paper.b,
                cell.paper.a,
                cell.pixels.0,
                cell.pixels.1,
                cell.pixels.2,
                cell.pixels.3,
                cell.character
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "null".to_string()),
                cell.depth
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "null".to_string()),
                cell.changed
            ));
        }
        out.push_str("],\"sounds\":[");
        for (index, sound) in self.sounds.iter().enumerate() {
            if index > 0 {
                out.push(',');
            }
            out.push_str(&format!(
                "{{\"id\":\"{}\",\"format\":\"{}\",\"rate\":{},\"channels\":{},\"frames\":{},\"samples\":{},\"peak\":{:.6},\"digest\":\"{}\",\"tick\":{},\"sample_offset\":{}}}",
                escape(&sound.id),
                escape(&sound.format),
                sound.rate,
                sound.channels,
                sound.frames,
                sound.samples,
                sound.peak,
                escape(&sound.digest),
                sound.tick,
                sound.sample_offset
            ));
        }
        out.push(']');
        out.push('}');
        out
    }

    /// The painted cells that changed a pixel, as one line a run can print.
    pub fn changed_line(&self) -> String {
        let changed: Vec<&CellAxis> = self.cells.iter().filter(|cell| cell.changed).collect();
        let named: Vec<String> = changed
            .iter()
            .take(6)
            .map(|cell| {
                format!(
                    "{}/{}:{} character {} pixels [{},{},{},{}]",
                    self.frame,
                    cell.col,
                    cell.row,
                    cell.character
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "-".to_string()),
                    cell.pixels.0,
                    cell.pixels.1,
                    cell.pixels.2,
                    cell.pixels.3
                )
            })
            .collect();
        format!(
            "axis tick {} frame {} changed_pixels {} changed_cells {} of {} painted [{}]",
            self.tick,
            self.frame,
            self.changed_pixels,
            changed.len(),
            self.cells.len(),
            named.join("; ")
        )
    }
}

/// The script lane's state, read by **offering the real mount a code frame**. The package is decoded
/// by the named reader (`package_owner::decode`), the decoded frame's own `kind` is set to `code`, and
/// that frame is offered to `planet_package::mount`. The refusal's own code and message are what the
/// axis reports, so a missing lane shows up as a recorded refusal, never as a silent blank.
///
/// Nothing here parses or rewrites package bytes: the only judge is the mount, and the only reader is
/// the one the ownership record names (`AGENTS.md` §2).
pub fn script_state(package: &[u8]) -> ScriptAxis {
    let mut frame = match crate::package_owner::decode(package) {
        Ok(frame) => frame,
        Err(refusal) => {
            return ScriptAxis {
                state: "absent",
                code: refusal.code().to_string(),
                message: refusal.message(),
                probe: "the package did not decode, so no code frame could be offered".to_string(),
            };
        }
    };
    let surface = planet_package::Surface::parse(&frame.payload)
        .map(|surface| surface.id)
        .unwrap_or_else(|_| "probe".to_string());
    frame.kind = planet_package::Kind::Code;
    match planet_package::mount(&frame, &surface, 1, 0, 0) {
        Ok(_) => ScriptAxis {
            state: "admitted",
            code: String::new(),
            message: "the mount accepted a code frame; this is not the expected state".to_string(),
            probe: "the decoded frame, its kind set to code, offered to planet_package::mount".to_string(),
        },
        Err(refusal) => ScriptAxis {
            state: "refused",
            code: refusal.code().to_string(),
            message: refusal.message(),
            probe: "the decoded frame, its kind set to code, offered to planet_package::mount".to_string(),
        },
    }
}

fn escape(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str, value: &str) -> Node {
        Node {
            id: id.to_string(),
            parent: None,
            kind: "label".to_string(),
            role: "body".to_string(),
            role_class: planet_package::RoleClass::Theme,
            value: value.to_string(),
        }
    }

    #[test]
    fn a_placement_row_is_read_into_a_rectangle() {
        let placement = placement_of(&node(
            "placement-3",
            "depth 548 character 137 at (2,3) 4x5 cells",
        ))
        .expect("a placement");
        assert_eq!(
            placement,
            Placement {
                character: 137,
                depth: 548,
                col: 2,
                row: 3,
                cols: 4,
                rows: 5
            }
        );
    }

    #[test]
    fn a_row_that_does_not_state_a_placement_is_not_guessed_at() {
        assert!(placement_of(&node("placement-0", "depth 1 character 2")).is_none());
        assert!(placement_of(&node("frame-count", "12 frames at 40 fps")).is_none());
    }

    #[test]
    fn bytes_that_are_not_a_package_leave_the_script_lane_absent_rather_than_blank() {
        let script = script_state(b"not a package at all");
        assert_eq!(script.state, "absent");
        assert!(!script.code.is_empty());
        assert!(script.message.contains("package") || script.message.contains("magic"));
    }
}
