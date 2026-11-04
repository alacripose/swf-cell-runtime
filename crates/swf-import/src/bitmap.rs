//! Lossless bitmap payloads, decoded to pixels.
//!
//! A `DefineBitsLossless` (tag 20) or `DefineBitsLossless2` (tag 36) body states a character id, a
//! format, a size, and a zlib stream. This module inflates the stream and reads the format's own
//! pixel encoding, so the package can carry pixels instead of a stand-in colour. A body that cannot
//! be decoded is refused by name; nothing here guesses a colour for a bitmap it did not read.

use std::io::Read;

/// A decoded bitmap: its character id, the format it stated, its size, and one RGBA pixel per texel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bitmap {
    pub id: u16,
    pub format: u8,
    pub width: u16,
    pub height: u16,
    /// Row-major, `width * height` entries, each `[red, green, blue, alpha]`.
    pub pixels: Vec<[u8; 4]>,
    /// Which colormap layout the inflated stream used: `counted` for the documented one-byte colour
    /// count followed by that many entries, `full-256` for a producer that wrote a complete 256-entry
    /// table with no count, or `packed` for formats 4 and 5.
    pub colormap: &'static str,
}

impl Bitmap {
    /// The mean of the opaque pixels. Fully transparent texels do not drag the colour to black.
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

    /// The first opaque texel in stream order, for a diagnostic that names one real pixel.
    pub fn first_opaque(&self) -> Option<(u8, u8, u8)> {
        self.pixels
            .iter()
            .find(|pixel| pixel[3] != 0)
            .map(|pixel| (pixel[0], pixel[1], pixel[2]))
    }

    /// How many distinct opaque colours the payload states. A decoded bitmap with one colour is a
    /// flat image; a decoded bitmap with many is a picture, and the count says which without a render.
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
}

/// A refusal, named: the code is matchable and the message names what was found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitmapRefusal {
    pub code: &'static str,
    pub message: String,
}

impl BitmapRefusal {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        BitmapRefusal {
            code,
            message: message.into(),
        }
    }
}

/// The largest inflated payload this reader will accept for one bitmap (64 MiB).
const MAX_INFLATED: usize = 64 * 1024 * 1024;

/// Decode `DefineBitsLossless` (20) or `DefineBitsLossless2` (36).
pub fn decode(tag_code: u16, body: &[u8]) -> Result<Bitmap, BitmapRefusal> {
    if tag_code != 20 && tag_code != 36 {
        return Err(BitmapRefusal::new(
            "bitmap-tag-unimplemented",
            format!(
                "tag {tag_code} is not a lossless bitmap; only 20 and 36 carry an uncompressed \
                 pixel payload"
            ),
        ));
    }
    if body.len() < 7 {
        return Err(BitmapRefusal::new(
            "bitmap-body-truncated",
            format!(
                "a lossless bitmap states 7 header bytes; the body has {}",
                body.len()
            ),
        ));
    }
    let id = u16::from_le_bytes([body[0], body[1]]);
    let format = body[2];
    let width = u16::from_le_bytes([body[3], body[4]]);
    let height = u16::from_le_bytes([body[5], body[6]]);
    let with_alpha = tag_code == 36;
    if !matches!(format, 3 | 4 | 5) {
        return Err(BitmapRefusal::new(
            "bitmap-format-unimplemented",
            format!("bitmap {id} states lossless format {format}; 3, 4 and 5 are implemented"),
        ));
    }
    if width == 0 || height == 0 {
        return Err(BitmapRefusal::new(
            "bitmap-size-zero",
            format!("bitmap {id} states {width}x{height}"),
        ));
    }
    let expected = width as usize * height as usize * 4;
    if expected > MAX_INFLATED {
        return Err(BitmapRefusal::new(
            "bitmap-size-overflow",
            format!("bitmap {id} states {width}x{height}, over the {MAX_INFLATED}-byte ceiling"),
        ));
    }
    // **The payload's framing is measured, not assumed.** A lossless body is documented as a
    // 7-byte header followed by a zlib stream, and a writer that emits raw deflate, or that starts
    // the stream one byte later, states the same pixels with different framing. Sixteen of this
    // movie's bitmaps inflate only from byte 8 and none from byte 7, so both offsets and both
    // framings are tried, and a candidate is accepted only when what it inflates to is large enough
    // for the bitmap's own stated size and format. A refusal names every candidate it tried.
    let mut raw = Vec::new();
    let mut accepted = false;
    let mut attempts: Vec<String> = Vec::new();
    'candidate: for offset in [7usize, 8] {
        if body.len() <= offset {
            attempts.push(format!("offset {offset}: no payload"));
            continue;
        }
        for framing in ["zlib", "deflate"] {
            let inflated = if framing == "zlib" {
                inflate(flate2::read::ZlibDecoder::new(&body[offset..]))
            } else {
                inflate(flate2::read::DeflateDecoder::new(&body[offset..]))
            };
            match inflated {
                None => attempts.push(format!("offset {offset} {framing}: does not inflate")),
                Some(bytes) => {
                    if pixels_fit(format, width, height, bytes.len()) {
                        raw = bytes;
                        accepted = true;
                        break 'candidate;
                    }
                    attempts.push(format!(
                        "offset {offset} {framing}: inflates to {} bytes, short of the stated {}x{}",
                        bytes.len(),
                        width,
                        height
                    ));
                }
            }
        }
    }
    if !accepted {
        return Err(BitmapRefusal::new(
            "bitmap-decompress-failed",
            format!("bitmap {id} states format {format} {width}x{height}: {}", attempts.join("; ")),
        ));
    }
    let (pixels, colormap) = match format {
        3 => decode_colormapped(id, width, height, with_alpha, &raw)?,
        4 => (decode_rgb15(id, width, height, &raw)?, "packed"),
        5 => (decode_rgb32(id, width, height, &raw)?, "packed"),
        other => {
            return Err(BitmapRefusal::new(
                "bitmap-format-unimplemented",
                format!("bitmap {id} states lossless format {other}; 3, 4 and 5 are implemented"),
            ))
        }
    };
    Ok(Bitmap {
        id,
        format,
        width,
        height,
        pixels,
        colormap,
    })
}

fn short(id: u16, format: u8, needed: usize, found: usize) -> BitmapRefusal {
    BitmapRefusal::new(
        "bitmap-pixels-short",
        format!("bitmap {id} format {format} needs {needed} inflated bytes; found {found}"),
    )
}

/// Format 3: a colour table, then one index byte per texel on a four-byte-aligned row.
///
/// The colour table's framing is **measured**: the documented layout is a one-byte count followed by
/// that many entries, and a producer that writes a complete 256-entry table with no count states the
/// same pixels with a different prefix. Both are sized against the stream's own length, so the
/// layout is chosen by arithmetic rather than assumed, and a stream that fits neither is refused.
fn decode_colormapped(
    id: u16,
    width: u16,
    height: u16,
    with_alpha: bool,
    raw: &[u8],
) -> Result<(Vec<[u8; 4]>, &'static str), BitmapRefusal> {
    let entry = if with_alpha { 4 } else { 3 };
    let stride = (width as usize + 3) & !3;
    let body = stride * height as usize;
    let count = raw.first().copied().unwrap_or(0) as usize;
    let counted_end = 1 + count * entry;
    let full_end = 256 * entry;
    let (entries, table_end, layout) = if raw.len() >= counted_end + body && count > 0 {
        (count, counted_end, "counted")
    } else if raw.len() >= full_end + body {
        (256usize, full_end, "full-256")
    } else {
        return Err(short(id, 3, counted_end.min(full_end) + body, raw.len()));
    };
    let mut table: Vec<[u8; 4]> = Vec::with_capacity(entries);
    for index in 0..entries {
        let at = table_end - entries * entry + index * entry;
        table.push(if with_alpha {
            [raw[at], raw[at + 1], raw[at + 2], raw[at + 3]]
        } else {
            [raw[at], raw[at + 1], raw[at + 2], 255]
        });
    }
    let needed = table_end + body;
    if raw.len() < needed {
        return Err(short(id, 3, needed, raw.len()));
    }
    let mut pixels = Vec::with_capacity(width as usize * height as usize);
    for row in 0..height as usize {
        for col in 0..width as usize {
            let index = raw[table_end + row * stride + col] as usize;
            pixels.push(table.get(index).copied().unwrap_or([0, 0, 0, 0]));
        }
    }
    Ok((pixels, layout))
}

/// Format 4: one 15-bit RGB word per texel, five bits per channel, on a four-byte-aligned row.
fn decode_rgb15(id: u16, width: u16, height: u16, raw: &[u8]) -> Result<Vec<[u8; 4]>, BitmapRefusal> {
    let stride = (width as usize * 2 + 3) & !3;
    let needed = stride * height as usize;
    if raw.len() < needed {
        return Err(short(id, 4, needed, raw.len()));
    }
    let mut pixels = Vec::with_capacity(width as usize * height as usize);
    for row in 0..height as usize {
        for col in 0..width as usize {
            let at = row * stride + col * 2;
            let word = u16::from_le_bytes([raw[at], raw[at + 1]]);
            let expand = |value: u16| ((value as u32 * 255) / 31) as u8;
            pixels.push([
                expand((word >> 10) & 0x1F),
                expand((word >> 5) & 0x1F),
                expand(word & 0x1F),
                255,
            ]);
        }
    }
    Ok(pixels)
}

/// Format 5: four bytes per texel, stored alpha, red, green, blue.
fn decode_rgb32(id: u16, width: u16, height: u16, raw: &[u8]) -> Result<Vec<[u8; 4]>, BitmapRefusal> {
    let needed = width as usize * height as usize * 4;
    if raw.len() < needed {
        return Err(short(id, 5, needed, raw.len()));
    }
    let mut pixels = Vec::with_capacity(width as usize * height as usize);
    for texel in raw.chunks_exact(4).take(width as usize * height as usize) {
        pixels.push([texel[1], texel[2], texel[3], texel[0]]);
    }
    Ok(pixels)
}

/// Inflate a stream to completion, or answer `None`: a decoder that fails anywhere yields nothing.
fn inflate(mut reader: impl Read) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    match reader.read_to_end(&mut out) {
        Ok(_) => Some(out),
        Err(_) => None,
    }
}

/// Whether `length` inflated bytes are enough for the stated format, width and height.
fn pixels_fit(format: u8, width: u16, height: u16, length: usize) -> bool {
    let wide = width as usize;
    let tall = height as usize;
    let stride = match format {
        3 => (wide + 3) & !3,
        4 => (wide * 2 + 3) & !3,
        5 => wide * 4,
        _ => return false,
    };
    let table = if format == 3 {
        // The colour table's own size is inside the stream; its first byte is the entry count.
        0
    } else {
        0
    };
    let _ = table;
    length >= stride * tall + usize::from(format == 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zlib(bytes: &[u8]) -> Vec<u8> {
        use flate2::write::ZlibEncoder;
        use flate2::Compression;
        use std::io::Write;
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(bytes).expect("encoding a test payload");
        encoder.finish().expect("finishing a test payload")
    }

    fn lossless(tag: u16, format: u8, width: u16, height: u16, raw: &[u8]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(&7u16.to_le_bytes());
        body.push(format);
        body.extend_from_slice(&width.to_le_bytes());
        body.extend_from_slice(&height.to_le_bytes());
        body.extend_from_slice(&zlib(raw));
        let _ = tag;
        body
    }

    #[test]
    fn a_colormapped_bitmap_decodes_to_its_own_pixels() {
        // Two colours, a 2x1 image of indices 0 and 1, padded to four bytes.
        let raw = [2u8, 255, 0, 0, 0, 255, 0, 0, 1, 0, 0];
        let bitmap = decode(20, &lossless(20, 3, 2, 1, &raw)).expect("a decodable bitmap");
        assert_eq!(bitmap.width, 2);
        assert_eq!(bitmap.pixels[0], [255, 0, 0, 255]);
        assert_eq!(bitmap.pixels[1], [0, 255, 0, 255]);
        assert_eq!(bitmap.distinct_colours(), 2);
        assert_eq!(bitmap.average(), (127, 127, 0));
    }

    #[test]
    fn a_fifteen_bit_bitmap_decodes_its_channels() {
        // Pure red in 15-bit, then a padding word.
        let raw = [0x00u8, 0x7C, 0x00, 0x00];
        let bitmap = decode(20, &lossless(20, 4, 1, 1, &raw)).expect("a decodable bitmap");
        assert_eq!(bitmap.pixels[0], [255, 0, 0, 255]);
    }

    #[test]
    fn a_thirty_two_bit_bitmap_carries_its_alpha() {
        // alpha, red, green, blue: a half-transparent blue texel.
        let raw = [128u8, 0, 0, 255];
        let bitmap = decode(36, &lossless(36, 5, 1, 1, &raw)).expect("a decodable bitmap");
        assert_eq!(bitmap.pixels[0], [0, 0, 255, 128]);
        assert_eq!(bitmap.average(), (0, 0, 255));
    }

    #[test]
    fn a_transparent_bitmap_has_no_colour_to_average() {
        let raw = [0u8, 10, 20, 30];
        let bitmap = decode(36, &lossless(36, 5, 1, 1, &raw)).expect("a decodable bitmap");
        assert_eq!(bitmap.average(), (0, 0, 0));
        assert_eq!(bitmap.first_opaque(), None);
    }

    #[test]
    fn an_unimplemented_format_is_refused_by_name() {
        let raw = [0u8, 1, 2, 3];
        let refusal = decode(20, &lossless(20, 7, 1, 1, &raw)).expect_err("format 7 is not read");
        assert_eq!(refusal.code, "bitmap-format-unimplemented");
        assert!(refusal.message.contains("format 7"));
    }

    #[test]
    fn a_payload_that_is_not_zlib_is_refused_by_name() {
        let mut body = Vec::new();
        body.extend_from_slice(&9u16.to_le_bytes());
        body.push(5);
        body.extend_from_slice(&1u16.to_le_bytes());
        body.extend_from_slice(&1u16.to_le_bytes());
        body.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);
        let refusal = decode(20, &body).expect_err("garbage does not inflate");
        assert_eq!(refusal.code, "bitmap-decompress-failed");
    }

    #[test]
    fn a_short_pixel_stream_is_refused_by_name() {
        // One colour, one index byte, but the row needs four bytes of index space.
        let raw = [1u8, 0, 0, 0, 0];
        let refusal = decode(20, &lossless(20, 3, 4, 1, &raw)).expect_err("the row is short");
        assert_eq!(refusal.code, "bitmap-pixels-short");
    }

    #[test]
    fn a_tag_that_is_not_lossless_is_refused_by_name() {
        let refusal = decode(21, &[0u8; 8]).expect_err("tag 21 is a JPEG");
        assert_eq!(refusal.code, "bitmap-tag-unimplemented");
    }

    #[test]
    fn a_zero_sized_bitmap_is_refused_by_name() {
        let refusal = decode(20, &lossless(20, 5, 0, 4, &[])).expect_err("zero width");
        assert_eq!(refusal.code, "bitmap-size-zero");
    }
}
