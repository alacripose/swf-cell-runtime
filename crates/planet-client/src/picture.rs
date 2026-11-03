//! The picture: a drawn raster turned into an offscreen RGBA8 image.
//!
//! The input is the [`WindowRaster`] the client already produced from the mounted, extent-checked
//! package, so nothing here resolves a role, re-checks or re-measures a frame, or parses a document:
//! every colour in the buffer is one the palette already answered. Each cell occupies its own pixel
//! rect — the format's 8 by 16 — and a glyph is composited into that rect from the client's bitmap
//! font. A cell with no glyph keeps its paper; an unpainted cell keeps the transparent paper the raster
//! gave it, so a hole in the picture is never a guessed colour.
//!
//! The output is inspectable: a width, a height, a stride and straight RGBA8 bytes, top row first.

use planet_package::{Refusal, Refusals};

use crate::draw::WindowRaster;
use crate::font;
use crate::palette::Rgba;

/// One offscreen picture.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Picture {
    pub width: u32,
    pub height: u32,
    /// Bytes per row: `width * 4`.
    pub stride: u32,
    /// RGBA8, straight alpha, rows top-down.
    pub pixels: Vec<u8>,
}

impl Picture {
    /// The pixel format, named so a consumer does not have to infer it.
    pub const FORMAT: &'static str = "rgba8";

    pub fn pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let offset = (y * self.stride + x * 4) as usize;
        self.pixels
            .get(offset..offset + 4)
            .map(|bytes| [bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

fn fill_rect(
    pixels: &mut [u8],
    stride: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    colour: Rgba,
) {
    for row in 0..h {
        for col in 0..w {
            let offset = ((y + row) * stride + (x + col) * 4) as usize;
            if let Some(slot) = pixels.get_mut(offset..offset + 4) {
                slot.copy_from_slice(&[colour.r, colour.g, colour.b, colour.a]);
            }
        }
    }
}

/// **Paint one drawn frame.** Paper first, then the glyph in its ink, cell by cell.
pub fn paint(drawn: &WindowRaster) -> Result<Picture, Refusals> {
    let width = drawn.raster.cols * font::CELL_W;
    let height = drawn.raster.rows * font::CELL_H;
    let stride = width * 4;
    // Transparent black is the canvas: a cell that carries no paper keeps it, and a hole is a hole.
    let mut pixels = vec![0u8; (stride * height) as usize];
    let mut problems = Refusals::new();

    for cell in &drawn.raster.cells {
        let x = cell.x * font::CELL_W;
        let y = cell.y * font::CELL_H;
        fill_rect(&mut pixels, stride, x, y, font::CELL_W, font::CELL_H, cell.paper);
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
            for col in 0..5u32 {
                if bits & (1 << (4 - col)) != 0 {
                    let offset = ((y + row as u32) * stride + (x + col) * 4) as usize;
                    if let Some(slot) = pixels.get_mut(offset..offset + 4) {
                        slot.copy_from_slice(&[ink.r, ink.g, ink.b, ink.a]);
                    }
                }
            }
        }
    }

    if problems.is_empty() {
        Ok(Picture {
            width,
            height,
            stride,
            pixels,
        })
    } else {
        Err(problems)
    }
}
