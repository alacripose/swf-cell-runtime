//! Texel painting: the package's carried RGBA blocks drawn into the picture.
//!
//! A bitmap fill's colour is not the bitmap. This module reads the `bitmap/<id>.rgba` artifacts a
//! frame carries, and draws those pixels into a [`Picture`] — one texel per pixel at unit scale, and
//! nearest-neighbour sampling when the placement's rect is a different size. Nothing here invents a
//! colour: a region whose character has no carried block is left as it was.

use planet_package::cellframe::CellFrame;
use planet_package::texels::Texels;

use crate::picture::Picture;

/// Every texel block a frame's artifact table carries, in artifact order.
pub fn texels_in(frame: &CellFrame) -> Vec<Texels> {
    frame
        .artifacts
        .iter()
        .filter(|artifact| artifact.path.starts_with("bitmap/") && artifact.path.ends_with(".rgba"))
        .filter_map(|artifact| Texels::decode(&artifact.bytes).ok())
        .collect()
}

/// Draw one texel block into `picture` at `(x, y)`, scaled to `w` by `h` pixels.
///
/// Returns how many pixels were written. A transparent texel writes nothing, so the paper under it
/// stays: a sprite's holes are holes, not black squares.
pub fn paint_texels(picture: &mut Picture, texels: &Texels, x: u32, y: u32, w: u32, h: u32) -> usize {
    if w == 0 || h == 0 || texels.width == 0 || texels.height == 0 {
        return 0;
    }
    let mut painted = 0usize;
    for row in 0..h {
        for col in 0..w {
            let source_x = (col as u64 * texels.width as u64 / w as u64) as u32;
            let source_y = (row as u64 * texels.height as u64 / h as u64) as u32;
            let Some(pixel) = texels.pixel(source_x, source_y) else {
                continue;
            };
            if pixel[3] == 0 || x + col >= picture.width || y + row >= picture.height {
                continue;
            }
            picture.set(x + col, y + row, pixel);
            painted += 1;
        }
    }
    painted
}

/// What one frame's bitmap painting did: how many sprite regions were drawn, and how many pixels.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Painted {
    pub regions: usize,
    pub pixels: usize,
}

/// Draw a frame's bitmap characters, each in the rectangle its placement covers.
///
/// The placement's rect is in cells, so a cell is `cell_w` by `cell_h` pixels; the block is sampled
/// across that rect. A placement whose character carries no texels is skipped rather than filled
/// with a stand-in colour.
pub fn paint_frame_bitmaps(
    picture: &mut Picture,
    frame: &CellFrame,
    placements: &[crate::axis::Placement],
    cell_w: u32,
    cell_h: u32,
) -> Painted {
    let blocks = texels_in(frame);
    let mut out = Painted::default();
    for place in placements {
        let Some(block) = blocks.iter().find(|block| block.id == place.character) else {
            continue;
        };
        let x = place.col * cell_w;
        let y = place.row * cell_h;
        let w = place.cols * cell_w;
        let h = place.rows * cell_h;
        let painted = paint_texels(picture, block, x, y, w, h);
        if painted > 0 {
            out.regions += 1;
            out.pixels += painted;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block() -> Texels {
        Texels {
            id: 7,
            width: 2,
            height: 1,
            format: 3,
            layout: 0,
            pixels: vec![[255, 0, 0, 255], [0, 128, 0, 255]],
        }
    }

    fn picture(width: u32, height: u32) -> Picture {
        Picture {
            width,
            height,
            stride: width * 4,
            pixels: vec![0u8; (width * height * 4) as usize],
        }
    }

    #[test]
    fn unit_scale_painting_writes_the_payload_pixel_for_pixel() {
        let mut canvas = picture(2, 1);
        let painted = paint_texels(&mut canvas, &block(), 0, 0, 2, 1);
        assert_eq!(painted, 2);
        assert_eq!(canvas.pixel(0, 0), Some([255, 0, 0, 255]));
        assert_eq!(canvas.pixel(1, 0), Some([0, 128, 0, 255]));
    }

    #[test]
    fn scaling_samples_the_payload_and_keeps_its_colours() {
        let mut canvas = picture(8, 4);
        let painted = paint_texels(&mut canvas, &block(), 0, 0, 8, 4);
        assert_eq!(painted, 32);
        assert_eq!(canvas.pixel(0, 0), Some([255, 0, 0, 255]));
        assert_eq!(canvas.pixel(7, 0), Some([0, 128, 0, 255]));
    }

    #[test]
    fn a_transparent_texel_leaves_the_canvas_alone() {
        let mut canvas = picture(1, 1);
        let mut clear = block();
        clear.pixels[0][3] = 0;
        clear.width = 1;
        clear.pixels.truncate(1);
        assert_eq!(paint_texels(&mut canvas, &clear, 0, 0, 1, 1), 0);
        assert_eq!(canvas.pixel(0, 0), Some([0, 0, 0, 0]));
    }

    #[test]
    fn an_out_of_range_target_writes_nothing_beyond_the_canvas() {
        let mut canvas = picture(1, 1);
        let mut block = block();
        block.width = 1;
        block.pixels.truncate(1);
        assert_eq!(paint_texels(&mut canvas, &block, 4, 4, 2, 2), 0);
    }
}
