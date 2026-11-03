//! The picture, proven by behaviour: dimensions and stride, each cell's paper at its pixel rect, a
//! glyph's ink composited where the font says, holes left transparent, and an undrawable character
//! refused rather than skipped.

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
         fill\t0\t0\t{cols}\tsurface\n\
         fill\t0\t1\t3\tchrome\n\
         run\t2\t4\ttext\t{glyphs}\n"
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

fn painted(glyphs: &str) -> picture::Picture {
    let windows = drawn(glyphs);
    picture::paint(&windows[0]).expect("every glyph is in the font")
}

#[test]
fn the_picture_has_the_formats_dimensions_and_stride() {
    let picture = painted("AB");
    assert_eq!(picture.width, 42 * font::CELL_W);
    assert_eq!(picture.height, 18 * font::CELL_H);
    assert_eq!(picture.stride, picture.width * 4);
    assert_eq!(picture.pixels.len() as u32, picture.stride * picture.height);
    assert_eq!(picture::Picture::FORMAT, "rgba8");
    assert_eq!(picture.pixel(picture.width - 1, picture.height - 1).is_some(), true);
    assert_eq!(picture.pixel(picture.width, 0), None);
}

#[test]
fn every_painted_cell_puts_its_paper_at_its_own_pixel_rect() {
    let palette = Palette::declared();
    let surface = palette.resolve("surface").unwrap();
    let chrome = palette.resolve("chrome").unwrap();
    let picture = painted("AB");
    let cell_w = font::CELL_W;
    let cell_h = font::CELL_H;

    for (x, y) in [(0, 0), (41, 0), (17, 0)] {
        for py in 0..cell_h {
            for px in 0..cell_w {
                assert_eq!(
                    picture.pixel(x * cell_w + px, y * cell_h + py),
                    Some([surface.r, surface.g, surface.b, surface.a]),
                    "paper at cell ({x}, {y}) pixel ({px}, {py})"
                );
            }
        }
    }
    for px in 0..2 * cell_w {
        assert_eq!(
            picture.pixel(px, cell_h),
            Some([chrome.r, chrome.g, chrome.b, chrome.a]),
            "the chrome run's second cell"
        );
    }
    // A cell the frame never painted is a hole, not a guess.
    for py in 0..cell_h {
        for px in 0..cell_w {
            assert_eq!(picture.pixel(px, 8 * cell_h + py), Some([0, 0, 0, 0]));
        }
    }
}

#[test]
fn a_glyphs_ink_is_composited_where_the_font_says_and_nowhere_else() {
    let palette = Palette::declared();
    let text = palette.resolve("text").unwrap();
    let ink = [text.r, text.g, text.b, text.a];
    let picture = painted("A");
    let origin_x = 2 * font::CELL_W;
    let origin_y = 4 * font::CELL_H;

    let bits = font::glyph('A').expect("the font carries A");
    let mut expected: Vec<(u32, u32)> = Vec::new();
    for (row, bits) in bits.iter().enumerate() {
        for col in 0..5u32 {
            if bits & (1 << (4 - col)) != 0 {
                expected.push((col, row as u32));
            }
        }
    }
    assert_eq!(expected.len(), 18, "the font's own bitmap has 18 pixels set");
    for (col, row) in &expected {
        assert_eq!(
            picture.pixel(origin_x + col, origin_y + row),
            Some(ink),
            "ink at glyph pixel ({col}, {row})"
        );
    }
    // Outside the 5 by 7 bitmap the cell keeps its paper.
    assert_eq!(picture.pixel(origin_x + 5, origin_y), Some([0, 0, 0, 0]));
    assert_eq!(picture.pixel(origin_x, origin_y + 7), Some([0, 0, 0, 0]));
    // The cell after the glyph is untouched.
    assert_eq!(picture.pixel(origin_x + font::CELL_W, origin_y), Some([0, 0, 0, 0]));
}

#[test]
fn no_role_word_or_document_text_reaches_the_buffer() {
    let picture = painted("AB");
    for word in [&b"surface"[..], &b"cellframe"[..], &b"chrome"[..], &b"text"[..], &b"fill"[..]] {
        assert!(
            !picture.pixels.windows(word.len()).any(|run| run == word),
            "{} reached the pixel buffer",
            String::from_utf8_lossy(word)
        );
    }
    // Every painted pixel is one of the palette's colours or the transparent canvas.
    let palette = Palette::declared();
    let allowed: Vec<[u8; 4]> = palette
        .roles()
        .filter_map(|role| palette.resolve(role))
        .map(|colour| [colour.r, colour.g, colour.b, colour.a])
        .chain([[0, 0, 0, 0]])
        .collect();
    for y in 0..picture.height {
        for x in 0..picture.width {
            let pixel = picture.pixel(x, y).unwrap();
            assert!(allowed.contains(&pixel), "unresolved colour {pixel:?} at ({x}, {y})");
        }
    }
}

#[test]
fn a_character_the_font_cannot_draw_is_refused_rather_than_skipped() {
    let windows = drawn("A~");
    let refusals = picture::paint(&windows[0]).expect_err("the font has no tilde");
    assert_eq!(refusals.code(), "cellframe-glyph");
    let message = refusals.message();
    assert!(message.contains("line 6"), "the binding line is missing: {message}");
    assert!(message.contains("MAP-0001"), "the frame id is missing: {message}");
    assert!(message.contains("main"), "the window id is missing: {message}");
    assert!(message.contains('~'), "the character is missing: {message}");
}

#[test]
fn a_frame_smaller_than_its_window_never_reaches_a_picture() {
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
