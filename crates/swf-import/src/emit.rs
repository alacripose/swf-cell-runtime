//! Emission: a measured file becomes the one canonical declaration frame the mount accepts.
//!
//! The document is validated by `planet_package::Surface::parse` before it is framed, so the importer
//! cannot write a package the mount would refuse. The stage window is sized from the declared stage
//! rectangle; the two label nodes carry the measured frame count and tag count. No shape, timeline or
//! script is lowered here — those representations are still the format owner's to declare.

use planet_package::{Frame, Kind, Refusal as PackageRefusal, Surface, SURFACE_SCHEMA};

use crate::header::{Header, Refusal};
use crate::tags::TagCounts;

pub fn surface_document(id: &str, header: &Header, counts: &TagCounts, tag_count: usize) -> String {
    let width = header.stage.width_px().max(1);
    let height = header.stage.height_px().max(1);
    let mut doc = String::new();
    doc.push_str("dl-enclave.surface/1\n");
    doc.push_str(&format!("surface\t{id}\n"));
    doc.push_str("requires\tobserve\n");
    doc.push_str(&format!(
        "window\tstage\tStage\tworld\tfill\t0\t0\t{width}\t{height}\t{width}\t{height}\ttrue\tfalse\t-\t-\tsurface\tstage\n"
    ));
    doc.push_str("node\tstage\tstage-root\t-\tpane\tsurface\t-\n");
    doc.push_str(&format!(
        "node\tstage\tframe-count\tstage-root\tlabel\tbody\t{count} frames at {rate} fps\n",
        count = header.frame_count,
        rate = header.frame_rate
    ));
    doc.push_str(&format!(
        "node\tstage\ttag-count\tstage-root\tlabel\tbody\t{tag_count} tags in {classes} classes\n",
        classes = counts.len()
    ));
    doc
}

pub fn package_frame(
    id: &str,
    header: &Header,
    counts: &TagCounts,
    tag_count: usize,
) -> Result<Frame, Refusal> {
    let doc = surface_document(id, header, counts, tag_count);
    Surface::parse(doc.as_bytes()).map_err(|refusal| {
        Refusal::Frame(format!("{} {}", refusal.code(), refusal.message()))
    })?;
    Frame::new(Kind::Decl, doc.as_bytes()).map_err(|refusal: PackageRefusal| {
        Refusal::Frame(format!("{} {}", refusal.code(), refusal.message()))
    })
}
