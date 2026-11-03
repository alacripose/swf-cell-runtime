//! The draw batch: the drawn raster projected into the rectangles a GPU would consume.
//!
//! The input is the [`WindowRaster`] the client already drew — no role is resolved here, no frame is
//! re-measured, no document or binding is parsed, and no glyph is looked up in a second place. The
//! output is a list of axis-aligned rectangles in picture pixels, each carrying a colour the palette
//! already answered: coalesced paper runs across a row first, then one rectangle per run of ink pixels
//! inside each glyph's own bitmap.
//!
//! Nothing is drawn and no device is touched. The contract is stated exactly: replaying the
//! rectangles, in order, into a buffer of the batch's own size reproduces the picture byte for byte —
//! which is what `tests/batch.rs` asserts, pixel by pixel, against `picture::paint`.

use planet_package::{Refusal, Refusals};

use crate::draw::WindowRaster;
use crate::font;
use crate::palette::Rgba;

/// One coloured rectangle in picture pixels, origin top left.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quad {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    /// The colour, as the palette already answered it.
    pub colour: Rgba,
}

/// One frame's draw batch: the picture's size, and the rectangles that fill it.
///
/// It carries no string at all — not a role, not a row, not a glyph — so there is nothing in it that
/// could be mistaken for document text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Batch {
    pub width: u32,
    pub height: u32,
    /// Paper rectangles first, then ink; replaying in this order is the picture.
    pub quads: Vec<Quad>,
}

/// **Project one drawn frame.** Paper runs coalesced, then ink runs, in the frame's own order.
///
/// A character the font cannot draw is refused (`cellframe-glyph`) with the window, the frame and the
/// binding row's line, and no batch is produced: a missing glyph is never a silently shorter batch.
pub fn plan(drawn: &WindowRaster) -> Result<Batch, Refusals> {
    let cols = drawn.raster.cols;
    let rows = drawn.raster.rows;
    let width = cols * font::CELL_W;
    let height = rows * font::CELL_H;
    let mut quads: Vec<Quad> = Vec::new();
    let mut problems = Refusals::new();

    // Paper: one rectangle per run of one colour, left to right, row by row. The transparent canvas is
    // already the background, so a run of it is not a rectangle at all.
    for y in 0..rows {
        let mut x = 0;
        while x < cols {
            let Some(cell) = drawn.raster.cell(x, y) else { break };
            let paper = cell.paper;
            let start = x;
            x += 1;
            while x < cols {
                match drawn.raster.cell(x, y) {
                    Some(next) if next.paper == paper => x += 1,
                    _ => break,
                }
            }
            if paper == Rgba::TRANSPARENT {
                continue;
            }
            quads.push(Quad {
                x: start * font::CELL_W,
                y: y * font::CELL_H,
                w: (x - start) * font::CELL_W,
                h: font::CELL_H,
                colour: paper,
            });
        }
    }

    // Ink: one rectangle per horizontal run of set bits, placed by the cell the glyph sits in.
    for cell in &drawn.raster.cells {
        let (Some(glyph), Some(ink)) = (cell.glyph, cell.ink) else {
            continue;
        };
        let Some(bits) = font::glyph(glyph) else {
            problems.refuse(Refusal::CellframeGlyph {
                line: drawn.line,
                window: drawn.window.clone(),
                frame: drawn.frame.clone(),
                glyph,
            });
            continue;
        };
        for (row, bits) in bits.iter().enumerate() {
            let mut col = 0u32;
            while col < 5 {
                if bits & (1 << (4 - col)) == 0 {
                    col += 1;
                    continue;
                }
                let start = col;
                while col < 5 && bits & (1 << (4 - col)) != 0 {
                    col += 1;
                }
                quads.push(Quad {
                    x: cell.x * font::CELL_W + start,
                    y: cell.y * font::CELL_H + row as u32,
                    w: col - start,
                    h: 1,
                    colour: ink,
                });
            }
        }
    }

    if problems.is_empty() {
        Ok(Batch {
            width,
            height,
            quads,
        })
    } else {
        Err(problems)
    }
}
