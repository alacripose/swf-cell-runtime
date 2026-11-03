//! Emission: a measured file becomes the one canonical declaration frame the mount accepts.
//!
//! The document is validated by `planet_package::Surface::parse` before it is framed, so the importer
//! cannot write a package the mount would refuse. The stage window is sized from the declared stage
//! rectangle; the label and table nodes carry the measured frame count, tag count, shape bounds,
//! sprite structure and media inventory. No shape, timeline or script is lowered here.

use planet_package::cellframe::{CellFrame, MAX_CELLS, MAX_COLS, MAX_ROWS};
use planet_package::{Frame, Kind, Refusal as PackageRefusal, Surface};

use crate::placement::Placement;

use crate::content::{Inventory, MediaKind};
use crate::header::{Header, Refusal};
use crate::tags::TagCounts;

pub fn surface_document(
    id: &str,
    header: &Header,
    counts: &TagCounts,
    tag_count: usize,
    inventory: &Inventory,
    lowering: &Lowering,
) -> String {
    let width = header.stage.width_px().max(1);
    let height = header.stage.height_px().max(1);
    let mut doc = String::new();
    doc.push_str(planet_package::schema_id());
    doc.push('\n');
    doc.push_str(&format!("surface\t{id}\n"));
    doc.push_str("requires\tobserve\n");
    // A window's content space is what a frame's extent must equal exactly, and the frame's unit is
    // its own cell (8 by 16 device-independent pixels). Declaring the stage in pixels would make the
    // window's content 60,000 cells wide for this movie's stage, which no frame may hold; declaring
    // it in cells is the same rectangle measured in the unit the binding speaks.
    let (cols, rows) = grid(header);
    doc.push_str(&format!(
        "window\tstage\tStage\tworld\tfill\t0\t0\t{cols}\t{rows}\t{cols}\t{rows}\ttrue\tfalse\t-\t-\tsurface\tstage\n"
    ));
    doc.push_str(&format!("frame\tstage\t{FRAME_ID}\n"));
    doc.push_str("node\tstage\tstage-root\t-\tpane\tsurface\t-\n");
    doc.push_str(&format!(
        "node\tstage\tframe-count\tstage-root\tlabel\tbody\t{count} frames at {rate} fps; {show} show-frame records\n",
        count = header.frame_count,
        rate = header.frame_rate,
        show = inventory.show_frames
    ));
    doc.push_str(&format!(
        "node\tstage\ttag-count\tstage-root\tlabel\tbody\t{tag_count} tags in {classes} classes\n",
        classes = counts.len()
    ));

    let shapes = match inventory.bounds() {
        Some(bounds) => format!(
            "shapes {count}; union {width}x{height} px at ({x},{y})",
            count = inventory.shapes.len(),
            width = bounds.width_px(),
            height = bounds.height_px(),
            x = bounds.x_min / 20,
            y = bounds.y_min / 20
        ),
        None => "shapes 0".to_string(),
    };
    doc.push_str(&format!(
        "node\tstage\tshapes\tstage-root\tlabel\tbody\t{shapes}\n"
    ));

    let nested: usize = inventory.sprites.iter().map(|sprite| sprite.nested).sum();
    doc.push_str(&format!(
        "node\tstage\tsprites\tstage-root\tlabel\tbody\tsprites {count}; {nested} nested tags\n",
        count = inventory.sprites.len()
    ));

    let mut media = String::new();
    for kind in [
        MediaKind::Bitmap,
        MediaKind::Sound,
        MediaKind::Font,
        MediaKind::Text,
    ] {
        if !media.is_empty() {
            media.push_str("; ");
        }
        media.push_str(&format!(
            "{name} {count} / {bytes} bytes",
            name = kind.name(),
            count = inventory.media_count(kind),
            bytes = inventory.media_bytes(kind)
        ));
    }
    doc.push_str(&format!(
        "node\tstage\tmedia\tstage-root\ttable\tbody\t{media}\n"
    ));
    // The display list, as declared facts beside the cells that were drawn from it: the line's
    // counts are the counts the frame carries, and each named placement says the depth and the
    // extent the lowering gave it.
    doc.push_str(&format!(
        "node\tstage\tplacements\tstage-root\tlabel\tbody\t{}\n",
        lowering.summary()
    ));
    for (index, cell) in lowering.cells.iter().take(24).enumerate() {
        doc.push_str(&format!(
            "node\tstage\tplacement-{index}\tstage-root\tlabel\tbody\tdepth {depth} character \
             {character} at ({col},{row}) {cols}x{rows} cells\n",
            depth = cell.depth,
            character = cell.character,
            col = cell.col,
            row = cell.row,
            cols = cell.cols,
            rows = cell.rows
        ));
    }
    doc
}

/// The frame document's id: one frame per package while nothing is lowered from the timeline.
pub const FRAME_ID: &str = "STAGE-0001";

/// One placement's rectangle in the frame's own cells: where the display list put a character.
///
/// `col`/`row` is the cell the placement's top-left corner falls in, and `cols`/`rows` the extent it
/// covers, rounded up so a character narrower than a cell still draws one. The depth rides with it,
/// because a cell that cannot say which depth produced it cannot be checked against the display
/// list by anything.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub depth: u16,
    pub character: u16,
    pub col: u32,
    pub row: u32,
    pub cols: u32,
    pub rows: u32,
}

/// What the display list lowered to, and every fact about what it could not lower.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Lowering {
    /// The cells that fit the stage, one per placement, in depth order.
    pub cells: Vec<Cell>,
    /// Placements whose rectangle starts outside the stage's content space. Recorded, never a reason
    /// to widen the window: the window's extent is the frame's extent, and that is a binding.
    pub outside: usize,
    /// Placements whose character has no measured shape — a sprite, or a definition the survey did
    /// not read as a shape. Their geometry is unknown, so no cell is invented for them.
    pub unmeasured: usize,
    /// Records the display list held that this reader could not read. Counted, not dropped in
    /// silence: each one is a record whose fields did not fit, so its geometry is unknown.
    pub refused: usize,
    /// Every placement the display list held, lowered or not, so the counts can be checked.
    pub placements: usize,
}

impl Lowering {
    /// The one line that has to agree with the number of cells drawn.
    pub fn summary(&self) -> String {
        format!(
            "{total} placements; {lowered} lowered into cells; {outside} outside the stage; \
             {unmeasured} without measured bounds; {refused} unreadable records",
            total = self.placements,
            lowered = self.cells.len(),
            outside = self.outside,
            unmeasured = self.unmeasured,
            refused = self.refused
        )
    }
}

/// Lower each placement to cells from facts the package already carries: the character's measured
/// shape bounds, and the placement's own translation and scale. Nothing is guessed — a character
/// with no measured shape yields no cell, and a rectangle that starts outside the stage is counted.
pub fn lower(placements: &[Placement], inventory: &crate::content::Inventory, cols: u32, rows: u32) -> Lowering {
    let mut lowering = Lowering {
        placements: placements.len(),
        ..Lowering::default()
    };
    if cols == 0 || rows == 0 {
        lowering.outside = placements.len();
        return lowering;
    }
    for placement in placements {
        // A character is either a shape with measured bounds, or a sprite whose bounds are the union
        // of the shapes it defines. Anything else has no measured extent, and none is invented.
        let bounds = inventory
            .shapes
            .iter()
            .find(|shape| shape.id == placement.character)
            .map(|shape| shape.bounds)
            .or_else(|| {
                inventory
                    .sprites
                    .iter()
                    .find(|sprite| sprite.id == placement.character)
                    .and_then(|sprite| sprite.bounds)
            })
            .or_else(|| {
                inventory
                    .texts
                    .iter()
                    .find(|text| text.id == placement.character)
                    .map(|text| text.bounds)
            });
        let Some(bounds) = bounds else {
            lowering.unmeasured += 1;
            continue;
        };
        let (scale_x, scale_y) = match placement.matrix {
            Some(matrix) => matrix.scale(),
            None => (1.0, 1.0),
        };
        let (translate_x, translate_y) = match placement.matrix {
            Some(matrix) => matrix.translation_px(),
            None => (0, 0),
        };
        // Twips are twentieths of a pixel; the shape's rectangle is scaled by the placement's own
        // 16.16 factor and moved by its own translation, in that order.
        let left = (bounds.x_min as f64 / 20.0) * scale_x + translate_x as f64;
        let top = (bounds.y_min as f64 / 20.0) * scale_y + translate_y as f64;
        let width = (((bounds.x_max - bounds.x_min) as f64 / 20.0) * scale_x).max(1.0);
        let height = (((bounds.y_max - bounds.y_min) as f64 / 20.0) * scale_y).max(1.0);
        let col = (left / 8.0).floor();
        let row = (top / 16.0).floor();
        if col < 0.0 || row < 0.0 || col >= cols as f64 || row >= rows as f64 {
            lowering.outside += 1;
            continue;
        }
        let wide = ((width / 8.0).ceil() as u32)
            .max(1)
            .min(cols - col as u32);
        let tall = ((height / 16.0).ceil() as u32)
            .max(1)
            .min(rows - row as u32);
        lowering.cells.push(Cell {
            depth: placement.depth,
            character: placement.character,
            col: col as u32,
            row: row as u32,
            cols: wide,
            rows: tall,
        });
    }
    lowering
}

/// The stage as the frame's own grid: cells of 8 by 16 device-independent pixels, bounded by the
/// format's declared ceiling rather than truncated silently.
pub fn grid(header: &Header) -> (u32, u32) {
    let width = header.stage.width_px().max(1) as u32;
    let height = header.stage.height_px().max(1) as u32;
    let cols = ((width + 7) / 8).clamp(1, MAX_COLS);
    let rows = ((height + 15) / 16).clamp(1, MAX_ROWS);
    if cols * rows > MAX_CELLS {
        (cols, (MAX_CELLS / cols).max(1))
    } else {
        (cols, rows)
    }
}

/// The frame the package's stage window binds: the whole stage as one surface paper, with the
/// measured facts as one run. Nothing is lowered from the placement list yet, and the run says so.
pub fn frame_document(header: &Header, lowering: &Lowering) -> String {
    let (cols, rows) = grid(header);
    let mut doc = format!(
        "{header_line}\n\
         frame\t{FRAME_ID}\n\
         \twindow\tstage\n\
         \tcols\t{cols}\n\
         \trows\t{rows}\n\
         \torder\t10\n\
         \trequires\tnone\n\
         fill\t0\t0\t{cols}\tsurface\n",
        header_line = planet_package::cellframe::HEADER,
    );
    // Back to front: a higher depth is painted later, which is what draws it in front.
    for cell in lowering.cells.iter().take(MAX_CELLS as usize) {
        doc.push_str(&format!(
            "fill\t{col}\t{row}\t{cols}\tchrome\n",
            col = cell.col,
            row = cell.row,
            cols = cell.cols
        ));
    }
    doc.push_str("run\t0\t0\ttext\tSTAGE\n");
    doc
}

/// The frame, validated by the grammar's own reader before it is written: the importer cannot emit a
/// frame the mount would refuse.
pub fn frame_bytes(header: &Header, lowering: &Lowering) -> Result<Vec<u8>, Refusal> {
    let doc = frame_document(header, lowering);
    CellFrame::parse(&doc)
        .map_err(|refusals| Refusal::Frame(format!("{} {}", refusals.code(), refusals.message())))?;
    Ok(doc.into_bytes())
}

pub fn package_frame(
    id: &str,
    header: &Header,
    counts: &TagCounts,
    tag_count: usize,
    inventory: &Inventory,
    lowering: &Lowering,
) -> Result<Frame, Refusal> {
    let doc = surface_document(id, header, counts, tag_count, inventory, lowering);
    Surface::parse(doc.as_bytes())
        .map_err(|refusal| Refusal::Frame(format!("{} {}", refusal.code(), refusal.message())))?;
    Frame::new(Kind::Decl, doc.as_bytes()).map_err(|refusal: PackageRefusal| {
        Refusal::Frame(format!("{} {}", refusal.code(), refusal.message()))
    })
}
