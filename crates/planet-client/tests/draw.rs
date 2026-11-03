//! The drawn picture, proven by behaviour: every row lands in its cell in the colour the palette
//! answers, no role survives as text, and a role the palette lacks is refused rather than substituted.

use planet_client::draw::{self, Raster};
use planet_client::palette::{Palette, Rgba};
use planet_client::{scene, CellMount};
use planet_package::{Frame, Kind};

fn document(binding: Option<&str>) -> String {
    let mut text = String::from("dl-enclave.surface/1\nsurface\talpha-shell\nrequires\tobserve\n");
    text.push_str("window\tmain\tMain\tframe\tright-dock\t0\t0\t42\t18\t35\t9\ttrue\tfalse\tminimize\tdrag\tsurface\tcontent-main\n");
    text.push_str("node\tmain\troot\t-\tpane\tsurface\t-\n");
    if let Some(id) = binding {
        text.push_str(&format!("frame\tmain\t{id}\n"));
    }
    text
}

fn bytes(document: &str) -> Vec<u8> {
    Frame::new(Kind::Decl, document.as_bytes())
        .expect("a declaration frame")
        .encode()
}

fn frame_document(id: &str, cols: u32, rows: u32) -> String {
    format!(
        "shploingus.cellframe/1\n\
         frame\t{id}\n\
         \twindow\tmain\n\
         \tcols\t{cols}\n\
         \trows\t{rows}\n\
         \torder\t10\n\
         \trequires\tnone\n\
         fill\t0\t0\t{cols}\tsurface\n\
         fill\t0\t1\t3\tchrome\n\
         run\t2\t4\ttext\tAB\n\
         run\t5\t4\taccent\tCD\n"
    )
}

fn drawn(palette: &Palette) -> Vec<draw::WindowRaster> {
    let alpha = frame_document("MAP-0001", 42, 18);
    let mut mount = CellMount::new();
    let adopted = mount
        .adopt_frames(&bytes(&document(Some("MAP-0001"))), &[&alpha], 7)
        .expect("it mounts");
    draw::rasters(adopted, palette).expect("every role is carried")
}

#[test]
fn every_row_is_drawn_in_its_cell_in_the_colour_the_palette_answers() {
    let palette = Palette::declared();
    let windows = drawn(&palette);
    assert_eq!(windows.len(), 1);
    let drawn = &windows[0];
    assert_eq!(drawn.window, "main");
    assert_eq!(drawn.frame, "MAP-0001");
    assert_eq!(drawn.line, 6);
    let raster: &Raster = &drawn.raster;
    assert_eq!((raster.cols, raster.rows), (42, 18));
    assert_eq!(raster.cells.len(), 42 * 18);

    let surface = palette.resolve("surface").unwrap();
    let chrome = palette.resolve("chrome").unwrap();
    let text = palette.resolve("text").unwrap();

    // The paper run covers row 0, and the accent run covers three cells of row 1.
    assert!(raster.cell(0, 0).unwrap().paper == surface);
    assert!(raster.cell(41, 0).unwrap().paper == surface);
    assert!(raster.cell(0, 1).unwrap().paper == chrome);
    assert!(raster.cell(2, 1).unwrap().paper == chrome);
    assert!(raster.cell(3, 1).unwrap().paper != chrome);
    // An unpainted cell is the window showing through, not a guessed colour.
    assert_eq!(raster.cell(0, 2).unwrap().paper, palette.resolve("transparent").unwrap());

    // The glyph runs land on their row, in their ink.
    assert_eq!(raster.row_text(4), "ABCD");
    assert_eq!(raster.cell(2, 4).unwrap().ink, Some(text));
    assert_eq!(raster.cell(3, 4).unwrap().glyph, Some('B'));
    assert_eq!(raster.cell(5, 4).unwrap().ink, Some(palette.resolve("accent").unwrap()));
    for y in [0, 1, 2, 3] {
        assert_eq!(raster.row_text(y), "", "a row with no run draws no glyph");
    }
}

#[test]
fn no_role_or_document_text_survives_into_the_drawn_picture() {
    let palette = Palette::declared();
    let windows = drawn(&palette);
    let raster = &windows[0].raster;
    let glyphs: String = raster
        .cells
        .iter()
        .filter_map(|cell| cell.glyph)
        .collect();
    assert_eq!(glyphs, "ABCD", "only the run's own glyphs are drawn");
    for word in ["surface", "accent", "text", "dim", "cellframe", "fill", "run"] {
        assert!(!glyphs.contains(word), "{word} leaked into the glyphs: {glyphs}");
    }
    // Every drawn cell carries colours, never a role name: the cells are colours and glyphs only.
    for cell in &raster.cells {
        let _: Rgba = cell.paper;
    }
}

#[test]
fn a_role_the_palette_lacks_is_refused_rather_than_substituted() {
    let palette = Palette::declared().without("accent");
    let alpha = frame_document("MAP-0001", 42, 18);
    let mut mount = CellMount::new();
    let adopted = mount
        .adopt_frames(&bytes(&document(Some("MAP-0001"))), &[&alpha], 7)
        .expect("the package itself is valid");
    let refusals = draw::rasters(adopted, &palette).expect_err("accent has no colour here");
    assert_eq!(refusals.code(), "cellframe-role");
    let message = refusals.message();
    assert!(message.contains("line 6"), "the binding line is missing: {message}");
    assert!(message.contains("MAP-0001"), "the frame id is missing: {message}");
    assert!(message.contains("main"), "the window id is missing: {message}");
    assert!(message.contains("accent"), "the role is missing: {message}");
}

#[test]
fn the_picture_comes_from_the_mount_and_not_from_a_second_parse() {
    let palette = Palette::declared();
    let windows = drawn(&palette);
    // The scene and the drawing describe the same mounted frame.
    let alpha = frame_document("MAP-0001", 42, 18);
    let mut mount = CellMount::new();
    let adopted = mount
        .adopt_frames(&bytes(&document(Some("MAP-0001"))), &[&alpha], 7)
        .expect("it mounts");
    let shown = scene::from_adopted(adopted);
    assert!(shown.body().contains("frame MAP-0001 — 42 by 18, order 10"));
    assert_eq!(windows[0].frame, "MAP-0001");
    assert_eq!(windows[0].raster.cols, 42);
}

#[test]
fn a_frame_smaller_than_its_window_is_refused_before_anything_is_drawn() {
    let narrow = frame_document("MAP-0001", 40, 18);
    let mut mount = CellMount::new();
    let refusals = mount
        .adopt_frames(&bytes(&document(Some("MAP-0001"))), &[&narrow], 7)
        .expect_err("40 by 18 leaves declared cells undrawn");
    assert_eq!(refusals.code(), "cellframe-extent");
    assert!(refusals.message().contains("line 6"), "{}", refusals.message());
}
