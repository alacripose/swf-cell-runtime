//! Emission: a measured file becomes the one canonical declaration frame the mount accepts.
//!
//! The document is validated by `planet_package::Surface::parse` before it is framed, so the importer
//! cannot write a package the mount would refuse. The stage window is sized from the declared stage
//! rectangle; the label and table nodes carry the measured frame count, tag count, shape bounds,
//! sprite structure and media inventory. No shape, timeline or script is lowered here.

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
    doc.push_str(&format!(
        "window\tstage\tStage\tworld\tfill\t0\t0\t{width}\t{height}\t{width}\t{height}\ttrue\tfalse\t-\t-\tsurface\tstage\n"
    ));
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
