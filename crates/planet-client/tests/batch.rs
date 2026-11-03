//! The draw batch, proven by behaviour: its geometry reproduces the picture byte for byte, paper is
//! coalesced rather than repeated per cell, a glyph's ink is its own rectangles in its own ink, and a
//! character the font cannot draw is refused with the window, the frame and the binding line.

use planet_client::batch::{self, Batch};
use planet_client::draw::{self, WindowRaster};
use planet_client::palette::Palette;
use planet_client::{font, picture, CellMount};
use planet_package::{Frame, Kind};

fn document() -> String {
    let mut text = String::from("dl-enclave.surface/1\nsurface\talpha-shell\nrequires\tobserve\n");
    text.push_str("window\tmain\tMain\tframe\tright-dock\t0\t0\t42\t18\t35\t9\ttrue\tfalse\tminimize\tdrag\tsurface\tcontent-main\n");
    text.push_str("node\tmain\troot\t-\tpane\tsurface\t-\n");
    text.push_str("frame\tmain\tMAP-0001\n");
    text
}

fn frame_document(cols: u32, glyphs: &str) -> String {
    format!(
        "shploingus.cellframe/1\n\
         frame\tMAP-0001\n\
         \twindow\tmain\n\
         \tcols\t{cols}\n\
         \trows\t18\n\
         \torder\t10\n\
         \trequires\tnone\n\
         \tfill\t0\t0\t{cols}\tsurface\n\
         \tfill\t0\t1\t3\tchrome\n\
         \trun\t2\t4\ttext\t{glyphs}\n"
    )
}

fn drawn(glyphs: &str) -> Vec<WindowRaster> {
    let bytes = Frame::new(Kind::Decl, document().as_bytes())
        .expect("a declaration frame")
        .encode();
    let alpha = frame_document(42, glyphs);
    let mut mount = CellMount::new();
    let adopted = mount
        .adopt_frames(&bytes, &[&alpha], 7)
        .expect("it mounts");
    draw::rasters(adopted, &Palette::declared()).expect("every role is carried")
}

fn planned(glyphs: &str) -> Batch {
    let windows = drawn(glyphs);
    batch::plan(&windows[0]).expect("every glyph is in the font")
}

/// The replay a consumer would do: fill each rectangle, in order, into a buffer of the batch's size.
fn replay(batch: &Batch) -> Vec<u8> {
    let stride = batch.width * 4;
    let mut pixels = vec![0u8; (stride * batch.height) as usize];
    for quad in &batch.quads {
        for row in 0..quad.h {
            for col in 0..quad.w {
                let offset = ((quad.y + row) * stride + (quad.x + col) * 4) as usize;
                pixels[offset..offset + 4].copy_from_slice(&[
                    quad.colour.r,
                    quad.colour.g,
                    quad.colour.b,
                    quad.colour.a,
                ]);
            }
        }
    }
    pixels
}

#[test]
fn the_batch_has_the_pictures_own_size_and_no_quad_leaves_it() {
    let batch = planned("AB");
    let windows = drawn("AB");
    let picture = picture::paint(&windows[0]).expect("painted");
    assert_eq!(batch.width, picture.width);
    assert_eq!(batch.height, picture.height);
    assert!(!batch.quads.is_empty(), "nothing to draw at all");
    for quad in &batch.quads {
        assert!(quad.w > 0 && quad.h > 0, "an empty quad: {quad:?}");
        assert!(
            quad.x + quad.w <= batch.width,
            "{} exceeds the width {}",
            quad.x + quad.w,
            batch.width
        );
        assert!(
            quad.y + quad.h <= batch.height,
            "{} exceeds the height {}",
            quad.y + quad.h,
            batch.height
        );
    }
}

#[test]
fn replaying_the_batch_reproduces_the_picture_byte_for_byte() {
    for glyphs in ["A", "AB", "HELLO", "0123"] {
        let windows = drawn(glyphs);
        let painted = picture::paint(&windows[0]).expect("painted");
        let batch = batch::plan(&windows[0]).expect("planned");
        assert_eq!(
            replay(&batch),
            painted.pixels,
            "the batch is not the picture for {glyphs}"
        );
        assert_eq!(replay(&batch).len() as u32, painted.stride * painted.height);
    }
}

#[test]
fn a_row_of_one_paper_is_one_quad_rather_than_one_per_cell() {
    let palette = Palette::declared();
    let surface = palette.resolve("surface").unwrap();
    let chrome = palette.resolve("chrome").unwrap();
    let batch = planned("AB");

    let surface_quads: Vec<_> = batch
        .quads
        .iter()
        .filter(|quad| quad.colour == surface && quad.h == font::CELL_H)
        .collect();
    assert_eq!(surface_quads.len(), 1, "row 0 is 42 cells of one paper");
    assert_eq!(surface_quads[0].x, 0);
    assert_eq!(surface_quads[0].y, 0);
    assert_eq!(surface_quads[0].w, 42 * font::CELL_W);

    // The three chrome cells of row 1 are one rectangle, at row 1's own pixel band.
    let chrome_quads: Vec<_> = batch
        .quads
        .iter()
        .filter(|quad| quad.colour == chrome)
        .collect();
    assert_eq!(chrome_quads.len(), 1);
    assert_eq!(chrome_quads[0].x, 0);
    assert_eq!(chrome_quads[0].y, font::CELL_H);
    assert_eq!(chrome_quads[0].w, 3 * font::CELL_W);

    // A row the frame never painted contributes nothing: the canvas is already the background.
    assert!(batch
        .quads
        .iter()
        .filter(|quad| quad.h == font::CELL_H)
        .all(|quad| quad.y < 2 * font::CELL_H));
}

#[test]
fn a_glyphs_ink_is_its_own_rectangles_in_its_own_ink() {
    let palette = Palette::declared();
    let ink = palette.resolve("text").unwrap();
    let batch = planned("A");
    let origin_x = 2 * font::CELL_W;
    let origin_y = 4 * font::CELL_H;

    let ink_quads: Vec<_> = batch
        .quads
        .iter()
        .filter(|quad| quad.colour == ink)
        .collect();
    // 'A' is 0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001: twelve horizontal runs.
    assert_eq!(ink_quads.len(), 12, "one rectangle per run of set bits");
    let covered: u32 = ink_quads.iter().map(|quad| quad.w * quad.h).sum();
    assert_eq!(covered, 18, "the font's 18 set pixels, nothing more");

    // The crossbar is one five-pixel run on the glyph's fourth row, inside the whole-cell paper band.
    assert!(
        ink_quads.iter().any(|quad| quad.x == origin_x
            && quad.y == origin_y + 3
            && quad.w == 5
            && quad.h == 1),
        "the crossbar is missing: {ink_quads:?}"
    );
    // A single pixel of the left stem, at the glyph's top row.
    assert!(ink_quads
        .iter()
        .any(|quad| quad.x == origin_x + 1 && quad.y == origin_y && quad.w == 3));
    // Nothing outside the 5 by 7 bitmap is ink.
    for quad in &ink_quads {
        assert!(quad.x >= origin_x && quad.x + quad.w <= origin_x + 5);
        assert!(quad.y >= origin_y && quad.y + quad.h <= origin_y + 7);
    }
}

#[test]
fn the_batch_carries_only_palette_colours_and_no_text() {
    let palette = Palette::declared();
    let allowed: Vec<[u8; 4]> = palette
        .roles()
        .filter_map(|role| palette.resolve(role))
        .map(|colour| [colour.r, colour.g, colour.b, colour.a])
        .collect();
    let batch = planned("AB");
    for quad in &batch.quads {
        let colour = quad.colour;
        assert!(
            allowed.contains(&[colour.r, colour.g, colour.b, colour.a]),
            "a colour no role answers for: {quad:?}"
        );
    }
    // A batch is numbers and colours: replaying it cannot produce a word, and the picture it makes
    // carries none either.
    let windows = drawn("AB");
    let picture = picture::paint(&windows[0]).expect("painted");
    assert_eq!(replay(&batch), picture.pixels);
    for word in [&b"surface"[..], &b"chrome"[..], &b"cellframe"[..]] {
        assert!(!picture
            .pixels
            .windows(word.len())
            .any(|run| run == word));
    }
}

#[test]
fn a_character_the_font_cannot_draw_is_refused_before_a_batch_exists() {
    let windows = drawn("A~");
    let refusals = batch::plan(&windows[0]).expect_err("the font has no tilde");
    assert_eq!(refusals.code(), "cellframe-glyph");
    let message = refusals.message();
    assert!(message.contains("line 6"), "the binding line is missing: {message}");
    assert!(message.contains("MAP-0001"), "the frame id is missing: {message}");
    assert!(message.contains("main"), "the window id is missing: {message}");
    assert!(message.contains('~'), "the character is missing: {message}");
}

#[test]
fn a_frame_smaller_than_its_window_never_reaches_a_batch() {
    let bytes = Frame::new(Kind::Decl, document().as_bytes())
        .expect("a declaration frame")
        .encode();
    let narrow = frame_document(40, "AB");
    let mut mount = CellMount::new();
    let refusals = mount
        .adopt_frames(&bytes, &[&narrow], 7)
        .expect_err("40 by 18 leaves declared cells undrawn");
    assert_eq!(refusals.code(), "cellframe-extent");
    assert!(refusals.message().contains("line 6"), "{}", refusals.message());
}
