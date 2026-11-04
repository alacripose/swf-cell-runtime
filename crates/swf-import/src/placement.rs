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

/// A placement's transform as an affine map, in pixels.
///
/// The format writes a matrix as `[scaleX rotateSkew0; rotateSkew1 scaleY]` in 16.16 and a
/// translation in twips; composing two of them is what turns a sprite's own placements into the
/// stage's coordinates, and doing it as a map (rather than by scaling a rectangle's two numbers)
/// is what keeps a rotated or sheared placement from being measured as if it were axis-aligned.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}

impl Affine {
    pub const IDENTITY: Affine = Affine {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        tx: 0.0,
        ty: 0.0,
    };

    /// `self` applied **after** `inner`: the map that takes a point through `inner` and then `self`.
    pub fn then(&self, inner: &Affine) -> Affine {
        Affine {
            a: self.a * inner.a + self.b * inner.c,
            b: self.a * inner.b + self.b * inner.d,
            c: self.c * inner.a + self.d * inner.c,
            d: self.c * inner.b + self.d * inner.d,
            tx: self.a * inner.tx + self.b * inner.ty + self.tx,
            ty: self.c * inner.tx + self.d * inner.ty + self.ty,
        }
    }

    /// The inverse map, or `None` for a degenerate placement: a fill is sampled by mapping a cell's
    /// centre back into the definition's own coordinates.
    pub fn inverse(&self) -> Option<Affine> {
        let determinant = self.a * self.d - self.b * self.c;
        if determinant.abs() < 1e-12 {
            return None;
        }
        let a = self.d / determinant;
        let b = -self.b / determinant;
        let c = -self.c / determinant;
        let d = self.a / determinant;
        Some(Affine {
            a,
            b,
            c,
            d,
            tx: -(a * self.tx + b * self.ty),
            ty: -(c * self.tx + d * self.ty),
        })
    }

    pub fn apply(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.a * x + self.b * y + self.tx,
            self.c * x + self.d * y + self.ty,
        )
    }
}

impl Matrix {
    /// The same transform as a map over pixels.
    pub fn affine(&self) -> Affine {
        Affine {
            a: self.scale_x as f64 / 65_536.0,
            b: self.rotate_0 as f64 / 65_536.0,
            c: self.rotate_1 as f64 / 65_536.0,
            d: self.scale_y as f64 / 65_536.0,
            tx: self.translate_x as f64 / 20.0,
            ty: self.translate_y as f64 / 20.0,
        }
    }
}

/// One placement: which character, at which depth, and how it is transformed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placement {
    pub character: u16,
    pub depth: u16,
    /// The last depth this placement clips, when the record declares one.
    ///
    /// A placement carrying a clip depth is a **mask**, not a drawn character: it defines the region
    /// where the depths below are visible. Painting it as a rectangle is how a full-stage mask turns
    /// the whole picture into one colour, so the fact travels here and the lowering declines to draw
    /// it, naming the gap rather than filling it with a shape the file did not state.
    pub clip_depth: Option<u16>,
    pub matrix: Option<Matrix>,
    /// True for a `PlaceObject2` that modifies an existing depth instead of adding one.
    pub moved: bool,
    /// The tag code the placement came from, so a reader can tell the two forms apart.
    pub code: u16,
    /// True when the placement's colour transform sets the alpha multiplier to zero.
    ///
    /// A fully transparent placement **paints nothing**: it is the shape a movie fades in or out of,
    /// and drawing it opaque is how a transparent white curtain comes out as a white screen. Alpha
    /// blending itself is not modelled, so only the one state that is unambiguous — invisible — is
    /// acted on, and a partially transparent placement is still drawn opaque and should be named.
    pub transparent: bool,
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
    // **No alignment here.** The format packs scale, rotate and the translate group continuously:
    // the width field of the translate follows the rotate values bit-immediately, and only the
    // matrix as a whole ends on a byte boundary. Aligning in between shifted every translate read by
    // up to seven bits, which is how a small offset decoded as millions of twips.
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
    let (matrix, mut at) = matrix(body, 4)?;
    // A `PlaceObject` may carry a colour transform without alpha after its matrix. It is consumed
    // exactly so the record ends where it says it does, and nothing is kept from it.
    if at < body.len() {
        let mut bits = Bits::new(&body[at..]);
        // A `PlaceObject` colour transform has no alpha channel, so nothing here can be invisible.
        let _ = cxform(&mut bits, false)?;
        at += bits.at();
    }
    if at != body.len() {
        return Err(Refusal::Unsupported { flags: 0xff });
    }
    Ok(Placement {
        character,
        depth,
        clip_depth: None,
        matrix,
        moved: false,
        code: 4,
        transparent: false,
    })
}

/// `PlaceObject`, tolerantly: the character, depth and transform are read and the tail ignored.
///
/// The same pass as [`place_object_2_tolerant`], for the older record: a colour transform or a field
/// this reader does not model comes after the geometry, so the geometry is still the file's own.
pub fn place_object_tolerant(body: &[u8]) -> Result<(Placement, usize), Refusal> {
    if body.len() < 4 {
        return Err(Refusal::Truncated { at: body.len() });
    }
    let character = u16::from_le_bytes([body[0], body[1]]);
    let depth = u16::from_le_bytes([body[2], body[3]]);
    let (matrix, at) = matrix(body, 4)?;
    Ok((
        Placement {
            character,
            depth,
            clip_depth: None,
            matrix,
            moved: false,
            transparent: false,
            code: 4,
        },
        body.len().saturating_sub(at),
    ))
}

const HAS_CLIP_ACTIONS: u8 = 0x80;
const HAS_CLIP_DEPTH: u8 = 0x40;
const HAS_NAME: u8 = 0x20;
const HAS_RATIO: u8 = 0x10;
const HAS_CXFORM: u8 = 0x08;
const HAS_MATRIX: u8 = 0x04;
const HAS_CHARACTER: u8 = 0x02;
const MOVE: u8 = 0x01;

/// A colour transform, read far enough to know where it ends and no further.
///
/// Its extent has to be consumed exactly: the fields after it sit at offsets this reader would
/// otherwise misread. It carries no geometry, so nothing is kept from it — that is a named gap, not
/// a silent one.
/// Opaque, in the format's 8.8 fixed point: 256 is alpha 1.0.
const ALPHA_OPAQUE: i32 = 256;

fn cxform(bits: &mut Bits, with_alpha: bool) -> Result<i32, Refusal> {
    let has_add = bits.bit()? == 1;
    let has_mult = bits.bit()? == 1;
    let nbits = bits.bits(4)? as usize;
    let channels = if with_alpha { 4 } else { 3 };
    let mut alpha = ALPHA_OPAQUE;
    if has_mult {
        for index in 0..channels {
            let value = bits.signed(nbits)?;
            // The alpha multiplier is the last channel when the transform carries one. Its value is
            // the only fact here that changes what is drawn: zero is invisible, not white.
            if with_alpha && index + 1 == channels {
                alpha = value;
            }
        }
    }
    if has_add {
        for _ in 0..channels {
            let _ = bits.signed(nbits)?;
        }
    }
    bits.align();
    Ok(alpha)
}

/// The fields of a `PlaceObject2` this reader keeps, and the offset just past them.
///
/// Clip actions and any bytes after the last field this reader parses are **not read**, but they do
/// not move an offset this reader uses: they sit after every field kept here, and a tag's own length
/// says where the next tag begins. That is why a tolerant pass can read the placement and ignore the
/// tail without misreading the geometry — the distinction `survey` draws between a field skipped
/// inside a record and a record skipped whole.
fn place_object_2_fields(body: &[u8]) -> Result<(Placement, usize), Refusal> {
    if body.len() < 3 {
        return Err(Refusal::Truncated { at: body.len() });
    }
    let flags = body[0];
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
    let (matrix, mut at) = if flags & HAS_MATRIX != 0 {
        matrix(body, at)?
    } else {
        (None, at)
    };
    // The optional fields follow the matrix in the format's own order, each self-delimiting.
    let mut transparent = false;
    if flags & HAS_CXFORM != 0 {
        let mut bits = Bits::new(&body[at..]);
        let alpha = cxform(&mut bits, true)?;
        transparent = alpha <= 0;
        at += bits.at();
    }
    if flags & HAS_RATIO != 0 {
        if body.len() < at + 2 {
            return Err(Refusal::Truncated { at });
        }
        at += 2;
    }
    if flags & HAS_NAME != 0 {
        match body[at..].iter().position(|byte| *byte == 0) {
            Some(end) => at += end + 1,
            None => return Err(Refusal::Truncated { at }),
        }
    }
    let mut clip_depth: Option<u16> = None;
    if flags & HAS_CLIP_DEPTH != 0 {
        if body.len() < at + 2 {
            return Err(Refusal::Truncated { at });
        }
        clip_depth = Some(u16::from_le_bytes([body[at], body[at + 1]]));
        at += 2;
    }
    Ok((
        Placement {
            character,
            depth,
            clip_depth,
            matrix,
            moved: flags & MOVE != 0,
            code: 26,
            transparent,
        },
        at.min(body.len()),
    ))
}

/// `PlaceObject2`, strictly: every byte the record carries has to be accounted for.
///
/// Clip actions are a nested, variable-length record with its own versioning, so a record carrying
/// them is refused by name rather than guessed at.
pub fn place_object_2(body: &[u8]) -> Result<Placement, Refusal> {
    if body.first().map_or(false, |flags| flags & HAS_CLIP_ACTIONS != 0) {
        return Err(Refusal::Unsupported { flags: body[0] });
    }
    let (placement, at) = place_object_2_fields(body)?;
    if at != body.len() {
        return Err(Refusal::Unsupported { flags: body[0] });
    }
    Ok(placement)
}

/// `PlaceObject2`, tolerantly: the geometry fields are read and the tail is ignored.
///
/// Answers the placement and how many trailing bytes were not read. This is the pass Q204 authorized:
/// a record carrying clip actions, or fields this reader does not model, still states its character,
/// its depth and its transform, and those are the facts a cell is lowered from.
pub fn place_object_2_tolerant(body: &[u8]) -> Result<(Placement, usize), Refusal> {
    let (placement, at) = place_object_2_fields(body)?;
    Ok((placement, body.len().saturating_sub(at)))
}

const HAS_CLASS_NAME_IN_FLAGS_2: u8 = 0x10;
const HAS_IMAGE_IN_FLAGS_2: u8 = 0x08;

/// `PlaceObject3` (tag 70), tolerantly: the character, depth and transform are read and the tail is
/// ignored.
///
/// `PlaceObject3` is `PlaceObject2` with a second flags byte and three fields after it — a filter
/// list, a blend mode and a bitmap-cache flag — none of which carry geometry. The geometry fields sit
/// at the same offsets they do in `PlaceObject2` once the second flags byte is consumed, so the same
/// pass reads them; only the class-name string, which precedes the character id when the record
/// declares one, has to be stepped over first.
pub fn place_object_3_tolerant(body: &[u8]) -> Result<(Placement, usize), Refusal> {
    if body.len() < 4 {
        return Err(Refusal::Truncated { at: body.len() });
    }
    let flags = body[0];
    let flags_2 = body[1];
    let depth = u16::from_le_bytes([body[2], body[3]]);
    let mut at = 4;
    let has_character = flags & HAS_CHARACTER != 0;
    // The format says a class name stands in place of the character id, and the owning reader reads
    // it when the record declares one or declares an image with no character.
    let has_class_name = flags_2 & HAS_CLASS_NAME_IN_FLAGS_2 != 0
        || (flags_2 & HAS_IMAGE_IN_FLAGS_2 != 0 && !has_character);
    if has_class_name {
        match body[at..].iter().position(|byte| *byte == 0) {
            Some(end) => at += end + 1,
            None => return Err(Refusal::Truncated { at }),
        }
    }
    let character = if has_character {
        if body.len() < at + 2 {
            return Err(Refusal::Truncated { at });
        }
        let id = u16::from_le_bytes([body[at], body[at + 1]]);
        at += 2;
        id
    } else {
        0
    };
    let (matrix, end) = if flags & HAS_MATRIX != 0 {
        matrix(body, at)?
    } else {
        (None, at)
    };
    // The clip depth sits after the transform and the three optional fields that may precede it, so
    // each is stepped over exactly before the depth is read. A mask read from the wrong offset would
    // clip the wrong depths, which is worse than not clipping at all.
    let mut tail = end;
    let mut transparent = false;
    if flags & HAS_CXFORM != 0 && tail < body.len() {
        let mut bits = Bits::new(&body[tail..]);
        let alpha = cxform(&mut bits, true)?;
        transparent = alpha <= 0;
        tail += bits.at();
    }
    if flags & HAS_RATIO != 0 {
        tail += 2;
    }
    if flags & HAS_NAME != 0 {
        match body.get(tail..).and_then(|rest| rest.iter().position(|byte| *byte == 0)) {
            Some(end) => tail += end + 1,
            None => return Err(Refusal::Truncated { at: tail }),
        }
    }
    let clip_depth = if flags & HAS_CLIP_DEPTH != 0 && tail + 2 <= body.len() {
        Some(u16::from_le_bytes([body[tail], body[tail + 1]]))
    } else {
        None
    };
    Ok((
        Placement {
            character,
            depth,
            clip_depth,
            matrix,
            moved: flags & MOVE != 0,
            code: 70,
            transparent,
        },
        body.len().saturating_sub(end),
    ))
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

/// Every placement the tag stream carries, reading each record with the tolerant pass.
///
/// Answers the placements, how many records were read with a tail ignored, and how many records
/// could not be read at all — three numbers, because "read exactly", "read the part that matters"
/// and "not read" are three different claims about the same stream.
pub fn survey_tolerant(
    tags: &[Tag],
    body: &[u8],
) -> Result<(Vec<Placement>, usize, Vec<(usize, u16, String)>), Refusal> {
    let mut placements = Vec::new();
    let mut tolerated = 0usize;
    let mut unreadable: Vec<(usize, u16, String)> = Vec::new();
    for tag in tags {
        let end = tag.at + tag.length;
        if end > body.len() {
            return Err(Refusal::Truncated { at: body.len() });
        }
        let slice = &body[tag.at..end];
        let attempt = match tag.code {
            4 => place_object_tolerant(slice),
            26 => place_object_2_tolerant(slice),
            70 => place_object_3_tolerant(slice),
            _ => continue,
        };
        match attempt {
            Ok((placement, ignored)) => {
                if ignored > 0 {
                    tolerated += 1;
                }
                placements.push(placement);
            }
            Err(refusal) => unreadable.push((
                tag.at,
                tag.code,
                format!("{} {}", refusal.code(), refusal.message()),
            )),
        }
    }
    Ok((placements, tolerated, unreadable))
}

/// One timeline frame's display list: what is in force when a `ShowFrame` closes it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DisplayState {
    /// The placements in force at each depth, in stream order.
    pub placements: Vec<Placement>,
    /// Records read with a tail ignored, cumulative to this frame.
    pub tolerated: usize,
    /// Records that could not be read, cumulative to this frame.
    pub unreadable: Vec<(usize, u16, String)>,
    /// The sound characters this frame starts (`StartSound`, code 15), in stream order.
    ///
    /// An event, not a payload: the character is defined elsewhere and carried once in the frame's
    /// audio table, and this is the frame the movie starts it on. A package that carried the sounds
    /// and not their frames would hold audio the client can never schedule.
    pub sounds: Vec<u16>,
}

/// **The timeline, frame by frame.** Each `ShowFrame` closes a frame, and the display list it closes
/// is the state the movie shows at that moment: a placement adds or moves one depth, and
/// `RemoveObject` (5) or `RemoveObject2` (28) clears one.
///
/// Reading the placements alone left every removed character in the list, so a curtain the movie
/// takes away still covered the picture: a frame composited out of records the file had already
/// removed. A removal is a record about the display list, and it is read here.
pub fn timeline(tags: &[Tag], body: &[u8]) -> Result<Vec<DisplayState>, Refusal> {
    let mut live: Vec<Placement> = Vec::new();
    let mut states: Vec<DisplayState> = Vec::new();
    let mut tolerated = 0usize;
    let mut unreadable: Vec<(usize, u16, String)> = Vec::new();
    let mut sounds: Vec<u16> = Vec::new();
    for tag in tags {
        let end = tag.at + tag.length;
        if end > body.len() {
            return Err(Refusal::Truncated { at: body.len() });
        }
        let slice = &body[tag.at..end];
        match tag.code {
            1 => states.push(DisplayState {
                placements: by_depth(&live),
                tolerated,
                unreadable: unreadable.clone(),
                sounds: std::mem::take(&mut sounds),
            }),
            // `RemoveObject2` states a depth; `RemoveObject` states a character and a depth. The
            // depth is what the display list is keyed by, so both clear by depth.
            28 if slice.len() >= 2 => {
                let depth = u16::from_le_bytes([slice[0], slice[1]]);
                live.retain(|placement| placement.depth != depth);
            }
            5 if slice.len() >= 4 => {
                let depth = u16::from_le_bytes([slice[2], slice[3]]);
                live.retain(|placement| placement.depth != depth);
            }
            // `StartSound` states the character it starts; the frame it lands in is the one the
            // next `ShowFrame` closes. The payload is carried once, in the audio table.
            15 if slice.len() >= 2 => {
                sounds.push(u16::from_le_bytes([slice[0], slice[1]]));
            }
            // **A `SoundStreamBlock` is audio on the frame it appears in.** The stream has no
            // character id of its own, so it is recorded as `0` and the package names the stream
            // clip by the audio table's own id for it (`-`): an event is where a sound starts, and
            // a stream starts on the frame that carries its block.
            45 if !slice.is_empty() => sounds.push(0),
            4 | 26 | 70 => {
                let attempt = if tag.code == 4 {
                    place_object_tolerant(slice)
                } else if tag.code == 26 {
                    place_object_2_tolerant(slice)
                } else {
                    place_object_3_tolerant(slice)
                };
                match attempt {
                    Ok((placement, ignored)) => {
                        if ignored > 0 {
                            tolerated += 1;
                        }
                        live.retain(|known| known.depth != placement.depth);
                        live.push(placement);
                    }
                    Err(refusal) => unreadable.push((
                        tag.at,
                        tag.code,
                        format!("{} {}", refusal.code(), refusal.message()),
                    )),
                }
            }
            _ => {}
        }
    }
    // A movie whose last frame is not closed by `ShowFrame` still has a display list: the one it
    // ends in. It is the last state, not a frame that was never shown.
    if states.is_empty()
        || states
            .last()
            .map(|state| state.placements.len() != live.len())
            .unwrap_or(true)
    {
        states.push(DisplayState {
            placements: by_depth(&live),
            tolerated,
            unreadable: unreadable.clone(),
            sounds: sounds.clone(),
        });
    }
    Ok(states)
}

/// Like [`survey`], but a record this reader cannot read is **counted** rather than aborting the
/// import.
///
/// The distinction is the framing: a tag's length says where the next tag starts, so a record skipped
/// whole costs nothing after it, while a field skipped *inside* a record would move every later field
/// in that record — which is why `survey` keeps refusing. The count travels into the declaration, so
/// a skipped record is declared rather than silently missing.
pub fn survey_partial(tags: &[Tag], body: &[u8]) -> Result<(Vec<Placement>, usize), Refusal> {
    let mut placements = Vec::new();
    let mut unreadable = 0usize;
    for tag in tags {
        let end = tag.at + tag.length;
        if end > body.len() {
            return Err(Refusal::Truncated { at: body.len() });
        }
        let slice = &body[tag.at..end];
        match tag.code {
            4 => match place_object(slice) {
                Ok(placement) => placements.push(placement),
                Err(_) => unreadable += 1,
            },
            26 => match place_object_2(slice) {
                Ok(placement) => placements.push(placement),
                Err(_) => unreadable += 1,
            },
            _ => {}
        }
    }
    Ok((placements, unreadable))
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
