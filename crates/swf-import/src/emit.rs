//! Emission: a measured file becomes the one canonical declaration frame the mount accepts.
//!
//! The document is validated by `planet_package::Surface::parse` before it is framed, so the importer
//! cannot write a package the mount would refuse. The stage window is sized from the declared stage
//! rectangle; the label and table nodes carry the measured frame count, tag count, shape bounds,
//! sprite structure and media inventory. No shape, timeline or script is lowered here.

use planet_package::cellframe::{CellFrame, MAX_CELLS, MAX_COLS, MAX_ROWS};
use planet_package::{Frame, Kind, Refusal as PackageRefusal, Surface};

use crate::content::{Inventory, MediaKind};
use crate::header::{Header, Refusal};
use crate::tags::TagCounts;

pub fn surface_document(
    id: &str,
    header: &Header,
    counts: &TagCounts,
    tag_count: usize,
    inventory: &Inventory,
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
    doc
}

/// The frame document's id: one frame per package while nothing is lowered from the timeline.
pub const FRAME_ID: &str = "STAGE-0001";

/// The stage as the frame's own grid: cells of 8 by 16 device-independent pixels, bounded by the
/// format's declared ceiling rather than truncated silently.
fn grid(header: &Header) -> (u32, u32) {
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
pub fn frame_document(header: &Header) -> String {
    let (cols, rows) = grid(header);
    format!(
        "{header_line}\n\
         frame\t{FRAME_ID}\n\
         \twindow\tstage\n\
         \tcols\t{cols}\n\
         \trows\t{rows}\n\
         \torder\t10\n\
         \trequires\tnone\n\
         fill\t0\t0\t{cols}\tsurface\n\
         run\t0\t0\ttext\tSTAGE\n",
        header_line = planet_package::cellframe::HEADER,
    )
}

/// The frame, validated by the grammar's own reader before it is written: the importer cannot emit a
/// frame the mount would refuse.
pub fn frame_bytes(header: &Header) -> Result<Vec<u8>, Refusal> {
    let doc = frame_document(header);
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
) -> Result<Frame, Refusal> {
    let doc = surface_document(id, header, counts, tag_count, inventory);
    Surface::parse(doc.as_bytes())
        .map_err(|refusal| Refusal::Frame(format!("{} {}", refusal.code(), refusal.message())))?;
    Frame::new(Kind::Decl, doc.as_bytes()).map_err(|refusal: PackageRefusal| {
        Refusal::Frame(format!("{} {}", refusal.code(), refusal.message()))
    })
}
