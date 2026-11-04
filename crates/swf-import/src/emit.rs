//! Emission: a measured file becomes the one canonical declaration frame the mount accepts.
//!
//! The document is validated by `planet_package::Surface::parse` before it is framed, so the importer
//! cannot write a package the mount would refuse. The stage window is sized from the declared stage
//! rectangle; the label and table nodes carry the measured frame count, tag count, shape bounds,
//! sprite structure and media inventory. No shape, timeline or script is lowered here.

use planet_package::cellframe::{
    payload_digest, Absence, CellFrame, ABSENCE_SECTION, CELL_H_DP, CELL_W_DP, EVENT_SECTION,
    MAX_CELLS, MAX_COLS, MAX_ROWS, TIMELINE_SECTION,
};
use planet_package::{Frame, Kind, Refusal as PackageRefusal, Surface};

use crate::placement::Placement;

use crate::content::{Inventory, MediaKind};
use crate::header::{Header, Rect, Refusal};
use crate::placement::Affine;
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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub depth: u16,
    /// The painter's order: the placement path from the stage to this cell, outermost first. A
    /// nested sprite's children carry their **own** depths inside the parent's slot, so a sprite's
    /// background cannot paint over its own art. Cells are emitted sorted by this path.
    pub order: Vec<u16>,
    pub character: u16,
    pub col: u32,
    pub row: u32,
    pub cols: u32,
    pub rows: u32,
    /// The paper role this cell is drawn in. For a definition with a measured solid fill this is a
    /// role the package declares for that colour (`fill-N`), so the frame draws in the file's own
    /// colour; a definition with no readable solid colour is drawn `chrome`.
    pub paper: String,
}

/// What the display list lowered to, and every fact about what it could not lower.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Lowering {
    /// The draws that fit the stage, in depth order. One row per run: a run of one paper is one
    /// `fill`, so a stage-wide shape costs a row rather than a cell per pixel of it.
    pub cells: Vec<Cell>,
    /// The cells those runs cover, counting a run by its width. This is the number the summary must
    /// state as "lowered into cells": a fill row is one row and many cells.
    pub painted: usize,
    /// Placements whose rectangle starts outside the stage's content space. Recorded, never a reason
    /// to widen the window: the window's extent is the frame's extent, and that is a binding.
    pub outside: usize,
    /// Placements whose character has no measured shape — a sprite, or a definition the survey did
    /// not read as a shape. Their geometry is unknown, so no cell is invented for them.
    pub unmeasured: usize,
    /// Records the display list held that this reader could not read. Counted, not dropped in
    /// silence: each one is a record whose fields did not fit, so its geometry is unknown.
    pub refused: usize,
    /// Records read with a tail ignored: the geometry fields were read, and the rest of the record
    /// was not. Counted separately from `refused`, because "read the part that matters" and "not
    /// read" are different claims.
    pub tolerated: usize,
    /// Placements of a sprite character whose own extent could not be measured. Named separately
    /// from `unmeasured_unknown` so "the survey saw this definition and could not size it" is not
    /// confused with "no definition of this id was measured at all".
    pub unmeasured_sprite: usize,
    /// Placements whose character id matches no shape, sprite or text this survey measured.
    pub unmeasured_unknown: usize,
    /// The characters that reached the walk with no extent at all, in the order they were met.
    /// Named rather than counted, because "five characters" is not an answer to "which five, and
    /// why did no measurement in the file cover them".
    pub unmeasured_ids: Vec<u16>,
    /// Sprite characters the walk met that state no art at all. No cell is drawn for them — a
    /// measured bounding box is not the sprite's picture — and the frame's `[absence]` section
    /// carries the reason, so the client reports the fact instead of receiving a rectangle.
    pub absent: usize,
    /// Placements that declare a clip depth: **masks**, not drawn characters. Painting one as a
    /// rectangle is how a full-stage mask turns a picture into one colour, so none is drawn and the
    /// count is stated. Clipping itself is not modelled, and that is a named gap.
    pub masked: usize,
    /// Placements whose colour transform sets alpha to zero: invisible, so they paint nothing. Alpha
    /// blending is not modelled; only the unambiguous invisible state is acted on.
    pub transparent: usize,
    /// The package's own colour roles: role name and the source colour it stands for. Empty when no
    /// lowered cell carried a colour, in which case the frame declares no vocabulary section and
    /// draws in the format's declared roles.
    pub colours: Vec<(String, (u8, u8, u8))>,
    /// Every placement the display list held, lowered or not, so the counts can be checked.
    pub placements: usize,
    /// Shapes the walk had to draw as a rectangle because their geometry did not decode. Imported
    /// content must reach zero here: a non-zero count means the client would draw a grid box.
    pub placeholder: usize,
    /// One row per placed character, in display-list order: the character, how its cells came about
    /// (`decoded` from the file's own fills and contours, `decoded-offscreen` when the geometry
    /// painted no cell, `placeholder` when a box stood in for geometry), and the cells it added.
    pub proof: Vec<(u16, &'static str, usize)>,
}

impl Lowering {
    /// The one line that has to agree with the number of cells drawn.
    pub fn summary(&self) -> String {
        let mut text = format!(
            "{total} placements; {lowered} lowered into cells; {outside} outside the stage; \
             {unmeasured} without measured bounds; {refused} unreadable records; {tolerated} read \
             with a tail ignored; {absent} stated absent; {masked} masks not drawn; \
             {transparent} invisible not drawn",
            total = self.placements,
            lowered = self.painted,
            outside = self.outside,
            unmeasured = self.unmeasured,
            refused = self.refused,
            tolerated = self.tolerated,
            absent = self.absent,
            masked = self.masked,
            transparent = self.transparent
        );
        text
    }
}

/// Lower each placement to cells from facts the package already carries.
///
/// A shape or text is drawn as the rectangle it states. A **sprite is walked**, not boxed: its first
/// frame's own placements are composed through its transform and lowered in turn, so a sprite that
/// draws a scene draws that scene. The walk stops at the survey's own nesting ceiling, and a sprite
/// whose first frame places nothing falls back to its measured union.
pub fn lower(
    placements: &[Placement],
    inventory: &crate::content::Inventory,
    cols: u32,
    rows: u32,
    frame: usize,
) -> Lowering {
    let mut lowering = Lowering {
        placements: placements.len(),
        ..Lowering::default()
    };
    // One role per distinct source colour, named by the package rather than by the client: the
    // importer read the colour, so the importer states it, and two packages never collide.
    let mut roles: Vec<((u8, u8, u8), String)> = Vec::new();
    if cols == 0 || rows == 0 {
        lowering.outside = placements.len();
        return lowering;
    }
    for placement in placements {
        let affine = placement
            .matrix
            .map(|matrix| matrix.affine())
            .unwrap_or(Affine::IDENTITY);
        let cells_before = lowering.cells.len();
        let placeholder_before = lowering.placeholder;
        let unmeasured_before = lowering.unmeasured;
        let absent_before = lowering.absent;
        let masked_before = lowering.masked;
        let transparent_before = lowering.transparent;
        walk(
            placement.character,
            placement.depth,
            &affine,
            0,
            &[placement.depth],
            inventory,
            cols,
            rows,
            &mut lowering,
            &mut roles,
            placement.clip_depth.is_some(),
            placement.transparent,
            None,
            frame,
        );
        let added = lowering.cells.len() - cells_before;
        let kind = if lowering.placeholder > placeholder_before {
            "placeholder"
        } else if lowering.unmeasured > unmeasured_before {
            "unmeasured"
        } else if lowering.absent > absent_before {
            "absent"
        } else if lowering.masked > masked_before {
            "mask"
        } else if lowering.transparent > transparent_before {
            "invisible"
        } else if added > 0 {
            "decoded"
        } else {
            "decoded-offscreen"
        };
        lowering.proof.push((placement.character, kind, added));
    }
    // **Back to front, by the placement path.** The display list states each character's depth, and
    // a higher depth is in front — inside a sprite as well as on the stage. Rows are painted in the
    // order the frame carries them, so the cells are ordered here by their whole path (outer depth
    // first, then each nested depth), stably, so two draws on one depth keep the stream's order.
    // Ordering by the outer depth alone let a sprite's own background, placed last in its first
    // frame, paint over the scene: one flat colour where the file's art should be.
    lowering.cells.sort_by(|left, right| left.order.cmp(&right.order));
    lowering
}

/// The walk's ceiling: the same nesting depth the survey reads sprites to.
const MAX_WALK: usize = 8;

/// **Which of a sprite's frames the movie's frame draws.** Frame *n* of the movie draws frame *n* of
/// the sprite; a sprite that states fewer frames than the movie shows its last one, and a sprite the
/// reader could not walk frame by frame falls back to the first frame it did read. `None` means the
/// sprite places nothing at all, which the caller states as an absence.
fn sprite_frame<'a>(
    sprite: &'a crate::content::SpriteInfo,
    frame: usize,
) -> Option<&'a Vec<Placement>> {
    if !sprite.frames_list.is_empty() {
        Some(&sprite.frames_list[frame.min(sprite.frames_list.len() - 1)])
    } else if !sprite.first.is_empty() {
        Some(&sprite.first)
    } else {
        None
    }
}

/// Walk one character into cells: draw what states an extent, and **step inside** what places
/// others. A sprite is not a box on the stage; it is the display list its first frame declares.
#[allow(clippy::too_many_arguments)]
fn walk(
    character: u16,
    depth: u16,
    affine: &Affine,
    steps: usize,
    order: &[u16],
    inventory: &crate::content::Inventory,
    cols: u32,
    rows: u32,
    lowering: &mut Lowering,
    roles: &mut Vec<((u8, u8, u8), String)>,
    mask: bool,
    transparent: bool,
    inherited: Option<(u8, u8, u8)>,
    frame: usize,
) {
    // **A mask is not a colour.** The record states the depths it clips, not a picture; drawing its
    // shape would paint a rectangle the movie uses as a window. It is counted and not drawn.
    if mask {
        lowering.masked += 1;
        return;
    }
    // **An invisible record paints nothing.** A placement whose alpha multiplier is zero is the
    // shape a movie fades in or out of; drawing it opaque is a white screen where the scene is.
    if transparent {
        lowering.transparent += 1;
        return;
    }
    if steps > MAX_WALK {
        lowering.unmeasured += 1;
        lowering.unmeasured_unknown += 1;
        note_unmeasured(lowering, character);
        return;
    }
    let shape = inventory.shapes.iter().find(|shape| shape.id == character);
    let text = inventory.texts.iter().find(|text| text.id == character);
    if let Some(bounds) = shape
        .map(|shape| shape.bounds)
        .or_else(|| text.map(|text| text.bounds))
    {
        // **The file's own geometry first.** A shape states filled contours, and those are what the
        // cells should carry; the rectangle is the fallback for a definition whose edges this
        // reader could not follow.
        if let Some(geometry) = shape.and_then(|shape| shape.geometry.as_ref()) {
            // Geometry that decoded is never replaced by a box, not even when it paints no cell:
            // a rectangle there would be content the file did not state. `decoded-offscreen` says
            // the cells came out empty, not that the shape failed to decode.
            if !geometry.is_empty() {
                rasterise(geometry, character, depth, order, affine, cols, rows, lowering, roles);
            }
            return;
        }
        // A *shape definition* whose geometry did not decode is the one case that draws a box in
        // place of content: it is counted here, by name, so "no imported content is a placeholder"
        // is a measured claim. A button or edit-text field states a rectangle rather than edges, and
        // drawing that rectangle is the file's own answer, not a substitute for geometry.
        if matches!(shape.map(|shape| shape.code), Some(2 | 22 | 32 | 83)) {
            lowering.placeholder += 1;
        }
        draw(
            bounds,
            shape.and_then(|shape| shape.fill).or(inherited),
            character,
            depth,
            order,
            affine,
            cols,
            rows,
            lowering,
            roles,
        );
        return;
    }
    if let Some(sprite) = inventory.sprites.iter().find(|sprite| sprite.id == character) {
        if let Some(shown) = sprite_frame(sprite, frame) {
            for inner in shown {
                let matrix = inner
                    .matrix
                    .map(|matrix| matrix.affine())
                    .unwrap_or(Affine::IDENTITY);
                // The child's own depth extends the parent's path: the sprite keeps its slot on the
                // stage, and inside that slot its children stack the way the sprite's first frame
                // says, not in the order the tags happened to be written.
                let mut inner_order = order.to_vec();
                inner_order.push(inner.depth);
                walk(
                    inner.character,
                    depth,
                    &affine.then(&matrix),
                    steps + 1,
                    &inner_order,
                    inventory,
                    cols,
                    rows,
                    lowering,
                    roles,
                    inner.clip_depth.is_some(),
                    inner.transparent,
                    sprite.fill.or(inherited),
                    frame,
                );
            }
            return;
        }
        // **A sprite is never its bounding box.** Its extent is the union of the shapes nested in
        // it — a rectangle around a scene is a grid standing where the file's picture should be,
        // which is exactly what this port must not draw. The first frame placed nothing, so there is
        // nothing to lower: the character is stated absent, with the reason, and no cell is invented
        // for it. A sprite whose body carries no display record lands here too, and that is not a
        // failed measurement — counting it unmeasured would report a hole where the file states an
        // empty list.
        if sprite.bounds.is_some() || sprite.draws {
            lowering.absent += 1;
        }
        return;
    }
    lowering.unmeasured += 1;
    lowering.unmeasured_unknown += 1;
    note_unmeasured(lowering, character);
}

/// Rasterise a definition's filled contours into the cell grid under `affine`.
///
/// Each cell is sampled at four points inside itself (Q113's supersampling), and a cell belongs to a
/// region when at least half its samples fall inside that region's contours by the non-zero winding
/// rule. The answer is the shape the file states, at the grid's resolution — not a rectangle around
/// it. Answers `false` when nothing was inside the frame, so the caller may fall back.
#[allow(clippy::too_many_arguments)]
fn rasterise(
    geometry: &crate::shape::Geometry,
    character: u16,
    depth: u16,
    order: &[u16],
    affine: &Affine,
    cols: u32,
    rows: u32,
    lowering: &mut Lowering,
    roles: &mut Vec<((u8, u8, u8), String)>,
) -> bool {
    let cell_w = CELL_W_DP as f64;
    let cell_h = CELL_H_DP as f64;
    // A fill is sampled in the definition's own coordinates, so the cell's pixel centre is mapped
    // back through the placement's transform before the fill answers its colour.
    let inverse = affine.inverse();
    let mut grid: Vec<Option<(u8, u8, u8)>> = vec![None; (cols * rows) as usize];
    let mut painted = false;
    for region in &geometry.regions {
        let contours: Vec<Vec<(f64, f64)>> = region
            .contours
            .iter()
            .map(|contour| {
                contour
                    .iter()
                    .map(|(x, y)| affine.apply(x / 20.0, y / 20.0))
                    .collect()
            })
            .collect();
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for contour in &contours {
            for (x, y) in contour {
                min_x = min_x.min(*x);
                min_y = min_y.min(*y);
                max_x = max_x.max(*x);
                max_y = max_y.max(*y);
            }
        }
        if !min_x.is_finite() {
            continue;
        }
        let col0 = ((min_x / cell_w).floor()).max(0.0) as u32;
        let row0 = ((min_y / cell_h).floor()).max(0.0) as u32;
        let col1 = (((max_x / cell_w).ceil()) as i64).clamp(0, cols as i64 - 1) as u32;
        let row1 = (((max_y / cell_h).ceil()) as i64).clamp(0, rows as i64 - 1) as u32;
        for row in row0..=row1 {
            for col in col0..=col1 {
                let mut hits = 0;
                let mut colour: Option<(u8, u8, u8)> = None;
                for (sx, sy) in [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)] {
                    let point = ((col as f64 + sx) * cell_w, (row as f64 + sy) * cell_h);
                    if crate::shape::inside(&contours, point) {
                        hits += 1;
                        if colour.is_none() {
                            if let Some(inverse) = inverse {
                                let local = inverse.apply(point.0, point.1);
                                colour = region.fill.colour_at((local.0 * 20.0, local.1 * 20.0));
                            }
                        }
                    }
                }
                if hits >= 2 {
                    if let Some(colour) = colour {
                        let index = (row * cols + col) as usize;
                        if grid[index].is_none() {
                            grid[index] = Some(colour);
                            painted = true;
                        }
                    }
                }
            }
        }
    }
    if !painted {
        return false;
    }        for row in 0..rows {
            let mut col = 0;
            while col < cols {
                let Some(colour) = grid[(row * cols + col) as usize] else {
                    col += 1;
                    continue;
                };
                // A stretch of one colour in a row is one `fill`: the format's run row exists for
                // exactly this, the picture is unchanged, and a stage-sized shape costs a row
                // instead of a cell for every pixel of it.
                let mut end = col + 1;
                while end < cols && grid[(row * cols + end) as usize] == Some(colour) {
                    end += 1;
                }
                if lowering.cells.len() >= MAX_CELLS as usize {
                    return true;
                }
                let paper = role_for(colour, roles, lowering);
                let wide = end - col;
                lowering.painted += wide as usize;
                lowering.cells.push(Cell {
                    depth,
                    order: order.to_vec(),
                    character,
                    col,
                    row,
                    cols: wide,
                    rows: 1,
                    paper,
                });
                col = end;
            }
        }
    true
}

/// How many distinct papers are **visible** when the cells are painted in their order.
///
/// A frame's painted-cell count says how much it covers; the papers it names say how many colours it
/// mentions; neither says what a viewer sees, because a later cell hides an earlier one. This is the
/// visible count, and it is the measure used to choose which timeline frame a one-frame package
/// ships — a choice about *which* frame, never about what the frame contains.
pub fn visible_papers(cells: &[Cell], cols: u32, rows: u32) -> usize {
    let mut grid: Vec<Option<&str>> = vec![None; (cols * rows) as usize];
    for cell in cells {
        for row in 0..cell.rows {
            for col in 0..cell.cols {
                let x = cell.col + col;
                let y = cell.row + row;
                if x < cols && y < rows {
                    grid[(y * cols + x) as usize] = Some(cell.paper.as_str());
                }
            }
        }
    }
    let mut seen: Vec<&str> = grid.into_iter().flatten().collect();
    seen.sort_unstable();
    seen.dedup();
    seen.len()
}

/// The role a colour is drawn in, declared once per distinct colour.
fn role_for(
    colour: (u8, u8, u8),
    roles: &mut Vec<((u8, u8, u8), String)>,
    lowering: &mut Lowering,
) -> String {
    match roles.iter().find(|(known, _)| *known == colour) {
        Some((_, role)) => role.clone(),
        None => {
            let role = format!("fill-{}", roles.len());
            roles.push((colour, role.clone()));
            lowering.colours.push((role.clone(), colour));
            role
        }
    }
}

fn note_unmeasured(lowering: &mut Lowering, character: u16) {
    if !lowering.unmeasured_ids.contains(&character) {
        lowering.unmeasured_ids.push(character);
    }
}

/// Draw one measured rectangle: its four corners through the transform, the union of those corners
/// clipped to the frame, and one cell per cell the clipped rectangle covers.
#[allow(clippy::too_many_arguments)]
fn draw(
    bounds: Rect,
    colour: Option<(u8, u8, u8)>,
    character: u16,
    depth: u16,
    order: &[u16],
    affine: &Affine,
    cols: u32,
    rows: u32,
    lowering: &mut Lowering,
    roles: &mut Vec<((u8, u8, u8), String)>,
) {
    let x0 = bounds.x_min as f64 / 20.0;
    let x1 = bounds.x_max as f64 / 20.0;
    let y0 = bounds.y_min as f64 / 20.0;
    let y1 = bounds.y_max as f64 / 20.0;
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for (x, y) in [(x0, y0), (x1, y0), (x0, y1), (x1, y1)] {
        let (px, py) = affine.apply(x, y);
        min_x = min_x.min(px);
        min_y = min_y.min(py);
        max_x = max_x.max(px);
        max_y = max_y.max(py);
    }
    let width = (max_x - min_x).max(1.0);
    let height = (max_y - min_y).max(1.0);
    // Q187: a rectangle is **clipped to the frame**, not dropped for starting outside it. Only the
    // part inside the grid becomes cells; a rectangle with no inside part is counted, and the
    // window's extent is untouched — the frame is never widened to reach a placement.
    let frame_w = cols as f64 * CELL_W_DP as f64;
    let frame_h = rows as f64 * CELL_H_DP as f64;
    let inside_left = min_x.max(0.0);
    let inside_top = min_y.max(0.0);
    let inside_right = (min_x + width).min(frame_w);
    let inside_bottom = (min_y + height).min(frame_h);
    let col = (inside_left / CELL_W_DP as f64).floor();
    let row = (inside_top / CELL_H_DP as f64).floor();
    if inside_right <= inside_left
        || inside_bottom <= inside_top
        || col >= cols as f64
        || row >= rows as f64
    {
        lowering.outside += 1;
        return;
    }
    let wide = (((inside_right - inside_left) / CELL_W_DP as f64).ceil() as u32)
        .max(1)
        .min(cols - col as u32);
    let tall = (((inside_bottom - inside_top) / CELL_H_DP as f64).ceil() as u32)
        .max(1)
        .min(rows - row as u32);
    // The colour role is declared only for a cell that is actually drawn: a placement clipped away
    // states no colour the frame has to carry.
    let paper = match colour {
        Some(colour) => role_for(colour, roles, lowering),
        None => "chrome".to_string(),
    };
    if lowering.cells.len() >= MAX_CELLS as usize {
        return;
    }
    lowering.painted += (wide * tall) as usize;
    lowering.cells.push(Cell {
        depth,
        order: order.to_vec(),
        character,
        col: col as u32,
        row: row as u32,
        cols: wide,
        rows: tall,
        paper,
    });
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
pub fn frame_document(
    header: &Header,
    lowering: &Lowering,
    sounds: &[crate::content::SoundInfo],
    sprites: &[crate::content::SpriteInfo],
    later_frames: &[(u32, String)],
    events: &[(u32, String)],
    actions: &[(u16, Vec<u8>)],
) -> String {
    let (cols, rows) = grid(header);
    // **The package's files first, then what the frame names.** The payload bytes live in the
    // artifact table keyed by package-relative path, and the audio rows reference a path — the shape
    // family B's `recipe.rs` artifact table and `cellframe.rs`'s sibling artifacts give, rather than
    // bytes spliced into a drawing row. A frame with no sound carries neither section.
    let mut artifact_rows = String::new();
    let mut audio_section = String::new();
    if !sounds.is_empty() {
        audio_section.push_str("[audio]\n");
        for sound in sounds {
            let path = artifact_path(sound);
            artifact_rows.push_str(&artifact_row(&path, &sound.bytes));
            audio_section.push_str(&format!(
                "{kind}\t{id}\t{path}\t{format}\t{rate}\t{samples}\n",
                kind = sound.kind,
                id = sound.id.map(|id| id.to_string()).unwrap_or_else(|| "-".to_string()),
                format = sound.format,
                rate = sound.rate,
                samples = sound.samples,
            ));
        }
        audio_section.push('\n');
    }
    // **The timeline's frames ride in the same table.** A one-frame package is a movie that never
    // advances: the client shows the frame it happens to carry and the tick has nowhere to go. Each
    // later frame is a sibling artifact named by order, which is the shape `voxel-package`'s frame
    // grammar and `voxel-client`'s `frame_artifacts` give — a frame is a file the package names by
    // id, not bytes spliced into a drawing row.
    let mut timeline_section = String::new();
    if !later_frames.is_empty() {
        timeline_section.push_str(TIMELINE_SECTION);
        timeline_section.push('\n');
        for (order, document) in later_frames {
            let path = format!("frames/STAGE-{order:04}.frame");
            artifact_rows.push_str(&artifact_row(&path, document.as_bytes()));
            timeline_section.push_str(&format!("frame\t{order}\t{path}\n"));
        }
        timeline_section.push('\n');
    }
    // **Which sound each frame starts.** The character's payload is carried once in `[audio]`; the
    // event names the frame and the character, so a client can schedule the movie's own sound
    // instead of holding payloads it has no moment to play.
    let mut event_section = String::new();
    if !events.is_empty() {
        event_section.push_str(EVENT_SECTION);
        event_section.push('\n');
        for (frame, sound) in events {
            event_section.push_str(&format!("sound\t{frame}\t{sound}\n"));
        }
        event_section.push('\n');
    }
    // **The root's own script, carried the same way its sounds are.** One artifact row per action
    // record, named by its position and the tag it came on, with its digest and its bytes: a client
    // can then say the package holds this script, and executing it is the code lane's decision.
    for (index, (code, bytes)) in actions.iter().enumerate() {
        let path = format!("script/action-{}-tag{code}.bin", index + 1);
        artifact_rows.push_str(&artifact_row(&path, bytes));
    }
    let artifact_section = if artifact_rows.is_empty() {
        String::new()
    } else {
        format!("[artifact]\n{artifact_rows}\n")
    };
    // **What the package does not carry is stated, not drawn.** Every sprite the survey measured
    // that has no art gets one row here, named by its character id with the reason and the owning
    // rule. A client that reads the package therefore reports `no-display-list` for a sprite whose
    // body declares no frame, instead of receiving a rectangle standing in for a picture.
    let states = sprite_absences(sprites);
    let mut absence_section = String::new();
    if !states.is_empty() {
        absence_section.push_str(ABSENCE_SECTION);
        absence_section.push('\n');
        for state in &states {
            absence_section.push_str(&format!(
                "{kind}\t{id}\t{reason}\t{rule}\n",
                kind = state.kind,
                id = state.id,
                reason = state.reason,
                rule = state.rule
            ));
        }
        absence_section.push('\n');
    }
    let mut doc = format!(
        "{artifact_section}{audio_section}{absence_section}{timeline_section}{event_section}"
    );
    doc.push_str(&drawing_document(lowering, cols, rows, FRAME_ID, 1));
    doc
}

/// **One timeline frame, as its own document.** A later frame is the same picture shape as the bound
/// one — the vocabulary it draws in, its own `frame` row with its own id and order, then the cells —
/// with no artifact table and no audio: the package carries the payloads once, in the bound frame,
/// and the timeline names frames rather than restating them.
pub fn timeline_document(header: &Header, lowering: &Lowering, id: &str, order: u32) -> String {
    let (cols, rows) = grid(header);
    drawing_document(lowering, cols, rows, id, order)
}

/// The streamed vocabulary section: the format's declared papers, the package's own colour roles,
/// and one `colour` row per role so the frame states the file's colour rather than borrowing the
/// client's nearest grey.
fn vocabulary_section(lowering: &Lowering) -> String {
    if lowering.colours.is_empty() {
        return String::new();
    }
    let mut papers: Vec<String> = planet_package::cellframe::PAPERS
        .iter()
        .map(|role| role.to_string())
        .collect();
    for (role, _) in &lowering.colours {
        papers.push(role.clone());
    }
    let mut section = String::new();
    section.push_str("[vocabulary]\n");
    section.push_str(&format!("paper\t{}\n", papers.join(",")));
    for (role, (r, g, b)) in &lowering.colours {
        section.push_str(&format!("colour\t{role}\t{r}\t{g}\t{b}\n"));
    }
    section.push('\n');
    section
}

/// **One picture, at the grammar's own shape.** The window's content space is addressed in full,
/// one row at a time — the shape family B's frames use
/// (`openlock/shell/alpha-shell/frames/MAP-0001.frame` fills every line it addresses), and the reason
/// a hole in the stage is a hole in the *file* rather than in the window. The cells follow back to
/// front: they arrive sorted by depth, so a higher depth is painted later, which is what draws it in
/// front. No run is added: the frame draws the cells the file states and nothing else.
fn drawing_document(
    lowering: &Lowering,
    cols: u32,
    rows: u32,
    id: &str,
    order: u32,
) -> String {
    let mut doc = format!(
        "{section}{header_line}\n\
         frame\t{id}\n\
         \twindow\tstage\n\
         \tcols\t{cols}\n\
         \trows\t{rows}\n\
         \torder\t{order}\n\
         \trequires\tnone\n",
        section = vocabulary_section(lowering),
        header_line = planet_package::cellframe::HEADER,
    );
    for row in 0..rows {
        doc.push_str(&format!("fill\t0\t{row}\t{cols}\tsurface\n"));
    }
    for cell in lowering.cells.iter().take(MAX_CELLS as usize) {
        doc.push_str(&format!(
            "fill\t{col}\t{row}\t{cols}\t{paper}\n",
            col = cell.col,
            row = cell.row,
            cols = cell.cols,
            paper = cell.paper
        ));
    }
    doc
}

/// Every sprite the survey measured that carries no art, stated as an absence row.
///
/// Three reasons are kept apart because they are three different facts about the file:
///
/// * `no-display-list` — the sprite body declares no frame at all, so there is no list to lower.
/// * `no-drawn-record` — it declares frames, and not one of them carries a display record (a body
///   that is only a sound stream is the case this port met).
/// * `undecoded-display-record` — a record exists and this reader could not read it, so the art is
///   unread rather than missing.
///
/// All three cite `cellframe::malformed-record`: family B's frame reader
/// (`shploingus-clean/crates/voxel-package/src/cellframe.rs`) refuses a frame with no `fill` and no
/// `run` under `RefusalKind::MalformedRecord`, because "an absence has to be stated as one". The
/// client reports the rule it violates and draws nothing, which is what that rule asks for.
pub fn sprite_absences(sprites: &[crate::content::SpriteInfo]) -> Vec<Absence> {
    let mut rows = Vec::new();
    for sprite in sprites {
        let reason = if !sprite.draws {
            if sprite.frames == 0 {
                "no-display-list"
            } else {
                "no-drawn-record"
            }
        } else if sprite.first.is_empty() {
            "undecoded-display-record"
        } else {
            continue;
        };
        rows.push(Absence {
            kind: "sprite".to_string(),
            id: sprite.id.to_string(),
            reason: reason.to_string(),
            rule: "cellframe::malformed-record".to_string(),
        });
    }
    rows
}

/// One artifact row: its package-relative path, the digest of its bytes, and the bytes as hex.
fn artifact_row(path: &str, bytes: &[u8]) -> String {
    let digest = payload_digest(bytes);
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        hex.push_str(&format!("{byte:02x}"));
    }
    format!("{path}\t{digest}\t{hex}\n")
}

/// The package-relative path a sound's payload is carried under. One row per character, named by the
/// id the file states; a stream block states none, so its name is fixed and its kind is in the row.
fn artifact_path(sound: &crate::content::SoundInfo) -> String {
    match sound.id {
        Some(id) => format!("audio/sound-{id}.{}", sound.format),
        None => format!("audio/stream.{}", sound.format),
    }
}

/// The frame, validated by the grammar's own reader before it is written: the importer cannot emit a
/// frame the mount would refuse.
pub fn frame_bytes(
    header: &Header,
    lowering: &Lowering,
    sounds: &[crate::content::SoundInfo],
    sprites: &[crate::content::SpriteInfo],
    later_frames: &[(u32, String)],
    events: &[(u32, String)],
    actions: &[(u16, Vec<u8>)],
) -> Result<Vec<u8>, Refusal> {
    let doc = frame_document(header, lowering, sounds, sprites, later_frames, events, actions);
    let parsed = CellFrame::parse(&doc)
        .map_err(|refusals| Refusal::Frame(format!("{} {}", refusals.code(), refusals.message())))?;
    // **The payload the frame carries is the payload the file states.** The frame is re-read through
    // the grammar and every clip compared byte for byte, so a truncation or an encoding mistake is
    // refused here rather than discovered by a listener.
    if parsed.audio.len() != sounds.len() {
        return Err(Refusal::Frame(format!(
            "audio-count: frame carries {} sounds, source states {}",
            parsed.audio.len(),
            sounds.len()
        )));
    }
    for (carried, source) in parsed.audio.iter().zip(sounds) {
        if carried.bytes != source.bytes {
            return Err(Refusal::Frame(format!(
                "audio-payload: sound {} carries {} bytes, source states {}",
                carried.id,
                carried.bytes.len(),
                source.bytes.len()
            )));
        }
    }
    // **Every absence the emitter states is an absence the grammar read back**, character for
    // character: a row that silently vanished would leave a client with nothing to report.
    let stated = sprite_absences(sprites);
    for (carried, source) in parsed.absences.iter().zip(&stated) {
        if carried.id != source.id || carried.reason != source.reason || carried.rule != source.rule {
            return Err(Refusal::Frame(format!(
                "absence-row: sprite {} states {} by {}, the frame read {} by {}",
                source.id, source.reason, source.rule, carried.reason, carried.rule
            )));
        }
    }
    if parsed.absences.len() != stated.len() {
        return Err(Refusal::Frame(format!(
            "absence-count: frame states {} absences, the survey measured {}",
            parsed.absences.len(),
            stated.len()
        )));
    }
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
