//! Packed bitmap texels: the RGBA payload a package carries and reads back.
//!
//! An imported bitmap is a rectangle of pixels, not a colour. This module is the payload's own
//! encoding so the artifact table can carry it through the existing framing — a path, a digest and
//! bytes — and so a client can read the same pixels back without re-quantising anything.
//!
//! Layout, little-endian, fixed width so a reader never has to guess:
//!
//! ```text
//! id (u16) | width (u16) | height (u16) | format (u8) | layout (u8) | width*height RGBA quads
//! ```
//!
//! `format` is the lossless format the source stated (3, 4 or 5); `layout` is `0` for a counted
//! colour table, `1` for a full 256-entry table, `2` for packed channels. They travel so a read-back
//! can state how the source encoded the pixels, not only what they are.

use std::fmt;

/// Header bytes before the first texel: id (2), width (2), height (2), format (1), layout (1).
pub const HEADER_BYTES: usize = 8;

/// The largest texel block one artifact may carry (16 M pixels, 64 MiB of RGBA).
pub const MAX_PIXELS: usize = 16 * 1024 * 1024;

/// One bitmap's pixels, with the identity and the source's own encoding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Texels {
    pub id: u16,
    pub width: u16,
    pub height: u16,
    /// The lossless format the source stated: 3 (colormapped), 4 (15-bit) or 5 (32-bit).
    pub format: u8,
    /// `0` counted colour table, `1` full 256-entry table, `2` packed channels.
    pub layout: u8,
    /// Row-major, `width * height` entries, each `[red, green, blue, alpha]`.
    pub pixels: Vec<[u8; 4]>,
}

/// A refusal, named: the code is matchable and the message states what was found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TexelRefusal {
    pub code: &'static str,
    pub message: String,
}

impl TexelRefusal {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        TexelRefusal {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for TexelRefusal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl Texels {
    /// The artifact path this block travels under. The id is in the name, so a client that holds a
    /// character id can find its pixels without a second index.
    pub fn artifact_path(&self) -> String {
        format!("bitmap/{}.rgba", self.id)
    }

    /// Encode the block for an artifact row: fixed header, then the RGBA quads.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(HEADER_BYTES + self.pixels.len() * 4);
        out.extend_from_slice(&self.id.to_le_bytes());
        out.extend_from_slice(&self.width.to_le_bytes());
        out.extend_from_slice(&self.height.to_le_bytes());
        out.push(self.format);
        out.push(self.layout);
        for pixel in &self.pixels {
            out.extend_from_slice(pixel);
        }
        out
    }

    /// Read a block back. The header's stated size must match the pixels that follow exactly: a
    /// short payload and a long one are both refusals, so a truncation cannot pass as a bitmap.
    pub fn decode(bytes: &[u8]) -> Result<Texels, TexelRefusal> {
        if bytes.len() < HEADER_BYTES {
            return Err(TexelRefusal::new(
                "texel-header-short",
                format!("a texel block states {HEADER_BYTES} header bytes; found {}", bytes.len()),
            ));
        }
        let id = u16::from_le_bytes([bytes[0], bytes[1]]);
        let width = u16::from_le_bytes([bytes[2], bytes[3]]);
        let height = u16::from_le_bytes([bytes[4], bytes[5]]);
        let format = bytes[6];
        let layout = bytes[7];
        let wanted = width as usize * height as usize;
        if wanted > MAX_PIXELS {
            return Err(TexelRefusal::new(
                "texel-size-overflow",
                format!("bitmap {id} states {width}x{height}, over the {MAX_PIXELS}-pixel ceiling"),
            ));
        }
        let available = bytes.len() - HEADER_BYTES;
        if available != wanted * 4 {
            return Err(TexelRefusal::new(
                "texel-pixels-mismatch",
                format!(
                    "bitmap {id} states {width}x{height} ({wanted} texels, {} bytes); the payload carries {available}",
                    wanted * 4
                ),
            ));
        }
        let mut pixels = Vec::with_capacity(wanted);
        for texel in bytes[HEADER_BYTES..].chunks_exact(4) {
            pixels.push([texel[0], texel[1], texel[2], texel[3]]);
        }
        Ok(Texels {
            id,
            width,
            height,
            format,
            layout,
            pixels,
        })
    }

    /// The texel at `(x, y)`, if it is inside the block.
    pub fn pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.width as u32 || y >= self.height as u32 {
            return None;
        }
        self.pixels.get(y as usize * self.width as usize + x as usize).copied()
    }

    /// The mean of the opaque texels: the one colour a renderer that cannot draw texels may use, and
    /// a summary for evidence. It is never a replacement for the pixels.
    pub fn average(&self) -> (u8, u8, u8) {
        let mut red = 0u64;
        let mut green = 0u64;
        let mut blue = 0u64;
        let mut count = 0u64;
        for pixel in &self.pixels {
            if pixel[3] == 0 {
                continue;
            }
            red += pixel[0] as u64;
            green += pixel[1] as u64;
            blue += pixel[2] as u64;
            count += 1;
        }
        if count == 0 {
            return (0, 0, 0);
        }
        ((red / count) as u8, (green / count) as u8, (blue / count) as u8)
    }

    /// How many distinct opaque colours the block holds, capped so a probe cannot run away.
    pub fn distinct_colours(&self) -> usize {
        let mut seen: Vec<[u8; 4]> = Vec::new();
        for pixel in &self.pixels {
            if pixel[3] == 0 || seen.contains(pixel) {
                continue;
            }
            seen.push(*pixel);
            if seen.len() > 4096 {
                break;
            }
        }
        seen.len()
    }

    /// A stable 64-bit FNV-1a digest of the encoded block, as hex. Two blocks with the same digest
    /// carry the same bytes under this encoding, which is what a round-trip check compares.
    pub fn digest(&self) -> String {
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for byte in self.encode() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        format!("{hash:016x}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block() -> Texels {
        Texels {
            id: 146,
            width: 2,
            height: 2,
            format: 3,
            layout: 1,
            pixels: vec![
                [255, 0, 0, 255],
                [0, 255, 0, 255],
                [0, 0, 255, 255],
                [0, 0, 0, 0],
            ],
        }
    }

    #[test]
    fn a_block_round_trips_through_its_encoded_bytes() {
        let original = block();
        let bytes = original.encode();
        assert_eq!(bytes.len(), HEADER_BYTES + 16);
        let read = Texels::decode(&bytes).expect("the encoded block reads back");
        assert_eq!(read, original);
        assert_eq!(read.pixels, original.pixels, "texels are byte-for-byte identical");
        assert_eq!(read.digest(), original.digest());
    }

    #[test]
    fn the_artifact_path_names_the_character() {
        let block = block();
        assert_eq!(block.artifact_path(), format!("bitmap/{}.rgba", block.id));
    }

    #[test]
    fn a_short_header_is_refused_by_name() {
        let refusal = Texels::decode(&[1, 2, 3]).expect_err("three bytes is no header");
        assert_eq!(refusal.code, "texel-header-short");
    }

    #[test]
    fn a_short_pixel_payload_is_refused_by_name() {
        let mut bytes = block().encode();
        bytes.truncate(bytes.len() - 4);
        let refusal = Texels::decode(&bytes).expect_err("one texel is missing");
        assert_eq!(refusal.code, "texel-pixels-mismatch");
        assert!(refusal.message.contains("states 2x2"));
    }

    #[test]
    fn a_long_pixel_payload_is_refused_by_name() {
        let mut bytes = block().encode();
        bytes.extend_from_slice(&[9, 9, 9, 9]);
        let refusal = Texels::decode(&bytes).expect_err("one texel too many");
        assert_eq!(refusal.code, "texel-pixels-mismatch");
    }

    #[test]
    fn the_summary_ignores_transparent_texels() {
        let block = block();
        assert_eq!(block.average(), (85, 85, 85));
        assert_eq!(block.distinct_colours(), 3);
        assert_eq!(block.pixel(0, 0), Some([255, 0, 0, 255]));
        assert_eq!(block.pixel(2, 0), None);
    }
}
