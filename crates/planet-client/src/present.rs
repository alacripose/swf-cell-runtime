//! The presentation boundary: a draw batch turned into the instance data a GPU consumes.
//!
//! This module is pure data — pixel rectangles in, clip-space instances out — and owns no device, no
//! window and no wgpu type. That division is the point: everything the device path is handed can be
//! proved by a headless test, and the only unproved part is the upload and the draw itself, which
//! needs a real adapter and is therefore run by hand, never by a test or the gate.
//!
//! Coordinates follow the GPU's own convention: clip space is `-1..1`, `y` grows upwards, and a pixel
//! rectangle's origin is top left, so a rectangle's height is carried as a negative clip-space height.
//! Colours are the palette's bytes divided by 255, stated as exactly that — no gamma transform, no
//! premultiplication and no colour-space conversion is claimed here.

use crate::batch::Quad;
use crate::palette::Rgba;

/// One instance: a clip-space rectangle and its colour, both four floats.
///
/// `rect` is `[x, y, width, height]` in clip space, with `height` negative because the picture's
/// origin is top left and clip space grows upwards.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Instance {
    pub rect: [f32; 4],
    pub colour: [f32; 4],
}

impl Instance {
    /// Bytes per instance: eight floats.
    pub const STRIDE: u32 = 32;

    /// The instance's bytes, little-endian, in the order `rect` then `colour` — the layout the vertex
    /// buffer declares.
    pub fn bytes(&self) -> [u8; 32] {
        let mut out = [0u8; 32];
        for (index, value) in self.rect.iter().chain(self.colour.iter()).enumerate() {
            out[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
        }
        out
    }
}

/// The palette's colour as the four floats the shader receives: each byte over 255.
pub fn colour(value: Rgba) -> [f32; 4] {
    [
        value.r as f32 / 255.0,
        value.g as f32 / 255.0,
        value.b as f32 / 255.0,
        value.a as f32 / 255.0,
    ]
}

/// **Project a draw batch into the instances a viewport of `viewport` pixels consumes.**
///
/// The order is the batch's order, so the instance at index `i` is the batch's rectangle at index `i`.
/// An empty batch projects to nothing: there is no instance for a hole, because a hole is what the
/// canvas already is.
pub fn instances(quads: &[Quad], viewport: (u32, u32)) -> Vec<Instance> {
    let width = viewport.0.max(1) as f32;
    let height = viewport.1.max(1) as f32;
    quads
        .iter()
        .map(|quad| Instance {
            rect: [
                2.0 * quad.x as f32 / width - 1.0,
                1.0 - 2.0 * quad.y as f32 / height,
                2.0 * quad.w as f32 / width,
                -2.0 * quad.h as f32 / height,
            ],
            colour: colour(quad.colour),
        })
        .collect()
}

/// The whole batch's instance bytes, one instance after another — what `queue.write_buffer` receives.
pub fn bytes(instances: &[Instance]) -> Vec<u8> {
    let mut out = Vec::with_capacity(instances.len() * Instance::STRIDE as usize);
    for instance in instances {
        out.extend_from_slice(&instance.bytes());
    }
    out
}
