//! Placement: where a display list puts a character, at what depth, with what transform.
//!
//! This is the first step D-7b orders, and it lowers nothing on its own: it reads the placement records
//! (`PlaceObject`, tag 4, and `PlaceObject2`, tag 26) into an approximation — a character id, a depth and
//! an optional transform — so a later step can turn them into cells. Values are numbers the format already
//! speaks: twips for translation, 16.16 fixed point for scale and rotation, both of which convert to the
//! pixels the surface declares by dividing by 20 and 65536.
//!
//! A record this reader cannot keep up with is refused by name rather than guessed at: a truncated bit
//! field is `placement-truncated`, and a flag combination it does not parse is `placement-unsupported`,
//! because skipping a field it did not read would put every later field at the wrong offset.

use crate::tags::Tag;

/// The matrix a placement carries, as the format writes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Matrix {
    pub translate_x: i32,
    pub translate_y: i32,
    pub scale_x: i32,
    pub scale_y: i32,
    pub rotate_0: i32,
    pub rotate_1: i32,
}

impl Matrix {
    pub const IDENTITY: Matrix = Matrix {
        translate_x: 0,
        translate_y: 0,
        scale_x: 0x1_0000,
        scale_y: 0x1_0000,
        rotate_0: 0,
        rotate_1: 0,
    };

    /// The translation in pixels, as the surface measures them.
    pub fn translation_px(&self) -> (i32, i32) {
        (self.translate_x / 20, self.translate_y / 20)
    }

    pub fn scale(&self) -> (f64, f64) {
        (
            self.scale_x as f64 / 65_536.0,
            self.scale_y as f64 / 65_536.0,
        )
    }
}

/// One placement: which character, at which depth, and how it is transformed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placement {
    pub character: u16,
    pub depth: u16,
    pub matrix: Option<Matrix>,
    /// True for a `PlaceObject2` that modifies an existing depth instead of adding one.
    pub moved: bool,
    /// The tag code the placement came from, so a reader can tell the two forms apart.
    pub code: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// The body ends inside a bit field the record declares.
    Truncated { at: usize },
    /// A flag this reader does not parse, named with its bits.
    Unsupported { flags: u8 },
}

impl Refusal {
    pub fn code(&self) -> &'static str {
        match self {
            Refusal::Truncated { .. } => "placement-truncated",
            Refusal::Unsupported { .. } => "placement-unsupported",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Refusal::Truncated { at } => {
                format!("the placement record ends at byte {at} inside a declared field")
            }
            Refusal::Unsupported { flags } => format!(
                "placement flags {flags:#010b} name a field this reader does not parse; \
                 skipping it would misread every field after it"
            ),
        }
    }
}

/// A bit reader over a tag body. SWF writes bits most significant first and realigns to a byte boundary
/// between the matrix's scale/rotate group and its translation group.
struct Bits<'a> {
    body: &'a [u8],
    bit: usize,
}

impl<'a> Bits<'a> {
    fn new(body: &'a [u8]) -> Bits<'a> {
        Bits { body, bit: 0 }
    }

    fn at(&self) -> usize {
        self.bit / 8
    }

    fn align(&mut self) {
        if self.bit % 8 != 0 {
            self.bit += 8 - self.bit % 8;
        }
    }

    fn bit(&mut self) -> Result<u32, Refusal> {
        if self.bit >= self.body.len() * 8 {
            return Err(Refusal::Truncated { at: self.at() });
        }
        let byte = self.body[self.bit / 8];
        let bit = (byte >> (7 - (self.bit % 8))) & 1;
        self.bit += 1;
        Ok(bit as u32)
    }

    fn bits(&mut self, count: usize) -> Result<u32, Refusal> {
        let mut value = 0u32;
        for _ in 0..count {
            value = (value << 1) | self.bit()?;
        }
        Ok(value)
    }

    /// A signed field of `count` bits, two's complement.
    fn signed(&mut self, count: usize) -> Result<i32, Refusal> {
        if count == 0 {
            return Ok(0);
        }
        let raw = self.bits(count)?;
        let shift = 32 - count;
        Ok(((raw << shift) as i32) >> shift)
    }
}

/// The matrix every placement may carry. `None` when the record declares none.
fn matrix(body: &[u8], at: usize) -> Result<(Option<Matrix>, usize), Refusal> {
    if body.len() <= at {
        return Ok((None, at));
    }
    // The field order is the format's, not a convenient one: each of scale and rotation carries its
    // own presence bit and its own width immediately before its own values, so a matrix with a scale
    // and no rotation does not have to declare a rotation width it never uses.
    let mut bits = Bits::new(&body[at..]);
    let mut matrix = Matrix::IDENTITY;
    if bits.bit()? == 1 {
        let nbits = bits.bits(5)? as usize;
        matrix.scale_x = bits.signed(nbits)?;
        matrix.scale_y = bits.signed(nbits)?;
    }
    if bits.bit()? == 1 {
        let nbits = bits.bits(5)? as usize;
        matrix.rotate_0 = bits.signed(nbits)?;
        matrix.rotate_1 = bits.signed(nbits)?;
    }
    bits.align();
    let translate_bits = bits.bits(5)? as usize;
    if translate_bits > 0 {
        matrix.translate_x = bits.signed(translate_bits)?;
        matrix.translate_y = bits.signed(translate_bits)?;
    }
    bits.align();
    Ok((Some(matrix), at + bits.at()))
}

/// `PlaceObject`: character, depth, matrix, and an optional colour transform this reader skips only when
/// the body ends where the matrix did.
pub fn place_object(body: &[u8]) -> Result<Placement, Refusal> {
    if body.len() < 4 {
        return Err(Refusal::Truncated { at: body.len() });
    }
    let character = u16::from_le_bytes([body[0], body[1]]);
    let depth = u16::from_le_bytes([body[2], body[3]]);
    let (matrix, at) = matrix(body, 4)?;
    if at < body.len() {
        // A colour transform follows; it is not read here, and pretending it is absent would be wrong.
        return Err(Refusal::Unsupported { flags: 0xff });
    }
    Ok(Placement {
        character,
        depth,
        matrix,
        moved: false,
        code: 4,
    })
}

const HAS_CLIP_ACTIONS: u8 = 0x80;
const HAS_CLIP_DEPTH: u8 = 0x40;
const HAS_NAME: u8 = 0x20;
const HAS_RATIO: u8 = 0x10;
const HAS_CXFORM: u8 = 0x08;
const HAS_MATRIX: u8 = 0x04;
const HAS_CHARACTER: u8 = 0x02;
const MOVE: u8 = 0x01;

/// `PlaceObject2`: a flags byte, a depth, then the fields the flags name in the format's own order.
pub fn place_object_2(body: &[u8]) -> Result<Placement, Refusal> {
    if body.len() < 3 {
        return Err(Refusal::Truncated { at: body.len() });
    }
    let flags = body[0];
    let unread = flags & (HAS_CLIP_ACTIONS | HAS_CLIP_DEPTH | HAS_NAME | HAS_RATIO | HAS_CXFORM);
    if unread != 0 {
        return Err(Refusal::Unsupported { flags });
    }
    let depth = u16::from_le_bytes([body[1], body[2]]);
    let mut at = 3;
    let character = if flags & HAS_CHARACTER != 0 {
        if body.len() < at + 2 {
            return Err(Refusal::Truncated { at });
        }
        let id = u16::from_le_bytes([body[at], body[at + 1]]);
        at += 2;
        id
    } else {
        0
    };
    let (matrix, at) = if flags & HAS_MATRIX != 0 {
        matrix(body, at)?
    } else {
        (None, at)
    };
    if at < body.len() {
        return Err(Refusal::Unsupported { flags });
    }
    Ok(Placement {
        character,
        depth,
        matrix,
        moved: flags & MOVE != 0,
        code: 26,
    })
}

/// Every placement the tag stream carries, in stream order. Bodies are slices of the decompressed
/// stream the walk recorded offsets into; nothing is copied or re-read.
pub fn survey(tags: &[Tag], body: &[u8]) -> Result<Vec<Placement>, Refusal> {
    let mut placements = Vec::new();
    for tag in tags {
        let end = tag.at + tag.length;
        if end > body.len() {
            return Err(Refusal::Truncated { at: body.len() });
        }
        let slice = &body[tag.at..end];
        match tag.code {
            4 => placements.push(place_object(slice)?),
            26 => placements.push(place_object_2(slice)?),
            _ => {}
        }
    }
    Ok(placements)
}

/// The placement in force at each depth: the last one in stream order wins, and a move that names no
/// character keeps the character already there.
pub fn by_depth(placements: &[Placement]) -> Vec<Placement> {
    let mut depths: Vec<Placement> = Vec::new();
    for placement in placements {
        match depths.iter_mut().find(|existing| existing.depth == placement.depth) {
            Some(existing) => {
                let character = if placement.character == 0 && placement.moved {
                    existing.character
                } else {
                    placement.character
                };
                *existing = Placement {
                    character,
                    ..*placement
                };
            }
            None => depths.push(*placement),
        }
    }
    depths.sort_by_key(|placement| placement.depth);
    depths
}
