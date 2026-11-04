//! A shape definition's **geometry**: the fills it states and the contours each one covers.
//!
//! A cell is meant to carry the game's own content, not a rectangle around it. Two facts make that
//! work: the style arrays must be walked **exactly** — every fill and line style, gradient and bitmap
//! styles included — so the record stream starts where the file says; and the fills themselves must be
//! carried, including the gradients a solid-only reader would drop.
//!
//! The oracle for the first fact is the tag's own length: a correct walk consumes the styles, the
//! record stream and nothing else, so `records end == tag end` for every definition. `SWF_SHAPE_DEBUG`
//! prints that equality per shape.
//!
//! The oracle for the second is the fill's own stops: a gradient region is drawn by sampling the ramp
//! the file states, through the matrix the file states, so its cells are the file's colours.

/// Segments one curved edge is flattened into.
pub const CURVE_STEPS: usize = 6;

/// An affine map: `x' = a·x + b·y + tx`, `y' = c·x + d·y + ty`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Map {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}

impl Map {
    pub const IDENTITY: Map = Map {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        tx: 0.0,
        ty: 0.0,
    };

    pub fn apply(&self, point: (f64, f64)) -> (f64, f64) {
        (
            self.a * point.0 + self.b * point.1 + self.tx,
            self.c * point.0 + self.d * point.1 + self.ty,
        )
    }

    /// The inverse map, or `None` for a degenerate one.
    pub fn inverse(&self) -> Option<Map> {
        let determinant = self.a * self.d - self.b * self.c;
        if determinant.abs() < 1e-12 {
            return None;
        }
        let a = self.d / determinant;
        let b = -self.b / determinant;
        let c = -self.c / determinant;
        let d = self.a / determinant;
        Some(Map {
            a,
            b,
            c,
            d,
            tx: -(a * self.tx + b * self.ty),
            ty: -(c * self.tx + d * self.ty),
        })
    }
}

/// One gradient: its stops and the matrix that places them in the definition's coordinates.
#[derive(Clone, Debug, PartialEq)]
pub struct Gradient {
    /// `(ratio in 0..=1, colour)`, sorted by ratio as the file lists them.
    pub stops: Vec<(f64, (u8, u8, u8))>,
    pub matrix: Map,
}

impl Gradient {
    /// The ramp's colour at `t`, interpolated between the two stops that bracket it.
    pub fn colour_at(&self, t: f64) -> Option<(u8, u8, u8)> {
        let t = t.clamp(0.0, 1.0);
        let first = self.stops.first()?;
        let last = self.stops.last()?;
        if self.stops.len() == 1 {
            return Some(first.1);
        }
        if t <= first.0 {
            return Some(first.1);
        }
        if t >= last.0 {
            return Some(last.1);
        }
        for pair in self.stops.windows(2) {
            let (left, right) = (pair[0], pair[1]);
            if t >= left.0 && t <= right.0 {
                let span = (right.0 - left.0).max(1e-9);
                let k = (t - left.0) / span;
                let mix = |a: u8, b: u8| {
                    (f64::from(a) + (f64::from(b) - f64::from(a)) * k).round().clamp(0.0, 255.0) as u8
                };
                return Some((mix(left.1 .0, right.1 .0), mix(left.1 .1, right.1 .1), mix(left.1 .2, right.1 .2)));
            }
        }
        Some(last.1)
    }
}

/// What fills a region.
#[derive(Clone, Debug, PartialEq)]
pub enum Fill {
    Solid((u8, u8, u8)),
    Linear(Gradient),
    Radial(Gradient),
    Focal { gradient: Gradient, focal: f64 },
    /// A bitmap fill keeps the character it references and its placement; the rasteriser draws the
    /// region in the bitmap's own average is not available, so the region is carried and drawn in a
    /// neutral colour rather than dropped.
    Bitmap { character: u16, matrix: Map },
}

impl Fill {
    /// The colour this fill gives a point, in the definition's coordinates.
    pub fn colour_at(&self, point: (f64, f64)) -> Option<(u8, u8, u8)> {
        match self {
            Fill::Solid(rgb) => Some(*rgb),
            Fill::Linear(gradient) => {
                let inverse = gradient.matrix.inverse()?;
                let (x, _) = inverse.apply(point);
                // The format's gradient square spans -16384..16384 in gradient space.
                Some(gradient.colour_at((x + 16_384.0) / 32_768.0)?)
            }
            Fill::Radial(gradient) | Fill::Focal { gradient, .. } => {
                let inverse = gradient.matrix.inverse()?;
                let (x, y) = inverse.apply(point);
                let radius = (x * x + y * y).sqrt();
                Some(gradient.colour_at((radius / 16_384.0).clamp(0.0, 1.0))?)
            }
            Fill::Bitmap { .. } => Some((128, 128, 128)),
        }
    }

    /// A short name for the kind, so a diagnostic can say what was carried.
    pub fn kind(&self) -> &'static str {
        match self {
            Fill::Solid(_) => "solid",
            Fill::Linear(_) => "linear",
            Fill::Radial(_) => "radial",
            Fill::Focal { .. } => "focal",
            Fill::Bitmap { .. } => "bitmap",
        }
    }
}

/// One filled region: the fill, and the contours (twips) that bound it.
#[derive(Clone, Debug, PartialEq)]
pub struct Region {
    pub fill: Fill,
    pub contours: Vec<Vec<(f64, f64)>>,
}

/// A definition's geometry, as read.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Geometry {
    pub regions: Vec<Region>,
    /// Whether the walk consumed the definition's tag exactly. A shape whose walk does not consume
    /// its tag was misread somewhere, and saying so is part of the evidence.
    pub exact: bool,
}

impl Geometry {
    /// The extent of the decoded contours, in twips: the file's own geometry, measured rather than
    /// taken from a header rectangle. `None` when no contour carries a point.
    pub fn bounds(&self) -> Option<(f64, f64, f64, f64)> {
        let mut found: Option<(f64, f64, f64, f64)> = None;
        for region in &self.regions {
            for contour in &region.contours {
                for (x, y) in contour {
                    found = Some(match found {
                        None => (*x, *y, *x, *y),
                        Some((x_min, y_min, x_max, y_max)) => (
                            x_min.min(*x),
                            y_min.min(*y),
                            x_max.max(*x),
                            y_max.max(*y),
                        ),
                    });
                }
            }
        }
        found
    }

    pub fn is_empty(&self) -> bool {
        self.regions.iter().all(|region| region.contours.is_empty())
    }

    pub fn region_kinds(&self) -> Vec<&'static str> {
        self.regions.iter().map(|region| region.fill.kind()).collect()
    }
}

struct Bits<'a> {
    body: &'a [u8],
    bit: usize,
}

impl<'a> Bits<'a> {
    fn new(body: &'a [u8], at: usize) -> Bits<'a> {
        Bits {
            body,
            bit: at * 8,
        }
    }

    fn bit(&mut self) -> Option<u32> {
        if self.bit >= self.body.len() * 8 {
            return None;
        }
        let byte = self.body[self.bit / 8];
        let value = (byte >> (7 - (self.bit % 8))) & 1;
        self.bit += 1;
        Some(value as u32)
    }

    fn bits(&mut self, count: usize) -> Option<u32> {
        let mut value = 0u32;
        for _ in 0..count {
            value = (value << 1) | self.bit()?;
        }
        Some(value)
    }

    fn signed(&mut self, count: usize) -> Option<i32> {
        if count == 0 {
            return Some(0);
        }
        let raw = self.bits(count)?;
        let shift = 32 - count;
        Some(((raw << shift) as i32) >> shift)
    }

    fn align(&mut self) {
        if self.bit % 8 != 0 {
            self.bit += 8 - self.bit % 8;
        }
    }

    fn byte(&mut self) -> Option<u8> {
        self.align();
        if self.bit / 8 >= self.body.len() {
            return None;
        }
        let value = self.body[self.bit / 8];
        self.bit += 8;
        Some(value)
    }

    fn u16(&mut self) -> Option<u16> {
        let low = self.byte()? as u16;
        let high = self.byte()? as u16;
        Some(low | (high << 8))
    }

    fn u32(&mut self) -> Option<u32> {
        let low = self.u16()? as u32;
        let high = self.u16()? as u32;
        Some(low | (high << 16))
    }
}

fn count(bits: &mut Bits) -> Option<usize> {
    let first = octet(bits)?;
    if first == 0xFF {
        Some(word(bits)? as usize)
    } else {
        Some(first as usize)
    }
}

/// The raw bits from `at`, as a string, so a record's fields can be read against the reference
/// implementation's decoded values rather than from this reader's own interpretation.
fn bits_text(body: &[u8], at: usize, count: usize) -> String {
    let mut text = String::new();
    for index in 0..count {
        let bit = at + index;
        if bit / 8 >= body.len() {
            break;
        }
        let value = (body[bit / 8] >> (7 - (bit % 8))) & 1;
        text.push(if value == 1 { '1' } else { '0' });
        if index % 8 == 7 {
            text.push(' ');
        }
    }
    text
}

/// One byte of a style array. Both entry paths in this package leave the walk byte-aligned before
/// the array, and aligning the byte measures better than reading eight bits mid-stream, so the
/// aligned reading is the one kept.
fn octet(bits: &mut Bits) -> Option<u8> {
    bits.byte()
}

/// One little-endian word of a style array.
fn word(bits: &mut Bits) -> Option<u16> {
    bits.u16()
}

/// Read one matrix as a map. Scale and rotation are 16.16; translation is in twips, like every other
/// translation in this format.
fn read_matrix(bits: &mut Bits, align: bool) -> Option<Map> {
    let mut map = Map::IDENTITY;
    if bits.bit()? == 1 {
        let nbits = bits.bits(5)? as usize;
        map.a = bits.signed(nbits)? as f64 / 65_536.0;
        map.d = bits.signed(nbits)? as f64 / 65_536.0;
    }
    if bits.bit()? == 1 {
        let nbits = bits.bits(5)? as usize;
        map.b = bits.signed(nbits)? as f64 / 65_536.0;
        map.c = bits.signed(nbits)? as f64 / 65_536.0;
    }
    let translate_bits = bits.bits(5)? as usize;
    if translate_bits > 0 {
        map.tx = bits.signed(translate_bits)? as f64;
        map.ty = bits.signed(translate_bits)? as f64;
    }
    if align {
        bits.align();
    }
    Some(map)
}

/// One gradient record table: a count, then ratio plus colour for each entry.
fn read_gradient(bits: &mut Bits, with_alpha: bool) -> Option<Vec<(f64, (u8, u8, u8))>> {
    // The count is one packed byte: SpreadMode (2 bits), InterpolationMode (2 bits) and
    // NumGradients (4 bits). Reading it as a whole byte overcounts every gradient whose spread or
    // interpolation mode is not zero, which desyncs the style array from there on.
    let packed = octet(bits)?;
    let entries = (packed & 0x0F) as usize;
    let mut stops = Vec::with_capacity(entries);
    for _ in 0..entries {
        let ratio = octet(bits)? as f64 / 255.0;
        let colour = (octet(bits)?, octet(bits)?, octet(bits)?);
        if with_alpha {
            octet(bits)?;
        }
        stops.push((ratio, colour));
    }
    Some(stops)
}

/// Read a definition's geometry. `None` only when a style cannot be walked at all.
pub fn read(body: &[u8], at: usize, length: usize, code: u16) -> Option<Geometry> {
    // One deterministic walk: the style arrays, the NumFillBits/NumLineBits byte, then the record
    // stream. The definition's own tag length is the check on that walk, never a search over starts.
    read_once(body, at, length, code, true, true)
}

fn read_once(
    body: &[u8],
    at: usize,
    length: usize,
    code: u16,
    grad_alpha: bool,
    matrix_align: bool,
) -> Option<Geometry> {
    let body = body.get(..at.checked_add(length)?)?;
    let debug = std::env::var("SWF_SHAPE_DEBUG").is_ok();
    let with_alpha = matches!(code, 32 | 83);
    let character_id = u16::from_le_bytes([body[at], body[at + 1]]);
    let mut bits = Bits::new(body, at);
    bits.u16()?;
    let (_, used) = crate::header::rect_at(body, bits.bit / 8).ok()?;
    bits.bit = (bits.bit / 8 + used) * 8;
    if code == 83 {
        let (_, edge_used) = crate::header::rect_at(body, bits.bit / 8).ok()?;
        bits.bit = (bits.bit / 8 + edge_used) * 8;
        bits.byte()?;
    }

    // --- fill and line styles ----------------------------------------------
    // A gradient record's colour is RGB in DefineShape/DefineShape2 and RGBA in the two later
    // definitions: the stop width follows the shape version, not the reader's preference.
    let mut fills = read_fills(&mut bits, code, with_alpha, grad_alpha && with_alpha, matrix_align)?;
    let mut line_count = skip_lines(&mut bits, code, with_alpha)?;

    // The style arrays are followed by one byte carrying the bit widths the record stream uses:
    // NumFillBits in its high nibble, NumLineBits in its low nibble. The widths are declared by the
    // definition, not derived from the counts, and the arrays end *before* this byte.
    // NumFillBits in the high nibble, NumLineBits in the low nibble of one byte.
    bits.align();
    let widths = bits.byte()?;
    let mut fill_bits = (widths >> 4) as usize;
    let mut line_bits = (widths & 0x0F) as usize;
    let styles_end = bits.bit / 8;
    if debug {
        eprintln!(
            "style_bits {} mod8 {} styles_end {styles_end} fill_bits {fill_bits} line_bits {line_bits}",
            bits.bit,
            bits.bit % 8
        );
    }

    let records_start = styles_end;
    bits.bit = records_start * 8;
    let mut regions: Vec<Region> = Vec::new();
    let mut current: Option<usize> = None;
    let mut contour: Vec<(f64, f64)> = Vec::new();
    let mut position = (0.0f64, 0.0f64);
    let mut start = (0.0f64, 0.0f64);
    let mut changes = 0usize;
    let mut edges = 0usize;
    let mut closed = 0usize;


    fn finish(
        current: &mut Option<usize>,
        contour: &mut Vec<(f64, f64)>,
        regions: &mut Vec<Region>,
        fills: &[Option<Fill>],
        closed: &mut usize,
    ) {
        let Some(index) = *current else {
            contour.clear();
            return;
        };
        if contour.len() >= 3 {
            if let Some(fill) = fills.get(index).cloned().flatten() {
                *closed += 1;
                match regions.iter_mut().find(|region| region.fill == fill) {
                    Some(region) => region.contours.push(std::mem::take(contour)),
                    None => regions.push(Region {
                        fill,
                        contours: vec![std::mem::take(contour)],
                    }),
                }
                return;
            }
        }
        contour.clear();
    }

    loop {
        // A shape record starts with one type flag: 0 is a style-change record, 1 is an edge whose
        // next bit says straight (1) or curved (0). Reading six bits as a type is what made every
        // edge fall through as an undefined kind and left the stream unable to reach its own end.
        // Running out of bits before the end-of-shape marker is a failed parse, not a finished one.
        let kind_at = bits.bit;
        let Some(edge) = bits.bit() else { return None };
        let kind = if edge == 0 {
            0
        } else if bits.bit()? == 1 {
            1
        } else {
            2
        };
        if debug {
            eprintln!("trace_record at {kind_at} kind {kind} chars {changes} edges {edges}");
            if character_id == 2 {
                eprintln!("trace_bits at {kind_at} {}", bits_text(body, kind_at, 48));
            }
        }
        if kind == 0 {
            let flags = bits.bits(5)?;
            if debug {
                eprintln!(
                    "trace_flags at {kind_at} flags {flags:#07b} newstyles {} linestyle {} fill1 {} fill0 {} moved {}",
                    flags & 0x10 != 0,
                    flags & 0x08 != 0,
                    flags & 0x04 != 0,
                    flags & 0x02 != 0,
                    flags & 0x01 != 0
                );
            }
            // A non-edge record with every flag clear is the end-of-shape marker; without this the
            // reader would walk a run of zero padding to the end of the tag and call it a parse.
            if flags == 0 {
                finish(&mut current, &mut contour, &mut regions, &fills, &mut closed);
                break;
            }
            let moved = flags & 0x01 != 0;
            if moved {
                let nbits = bits.bits(5)? as usize;
                let x = bits.signed(nbits)? as f64;
                let y = bits.signed(nbits)? as f64;
                if debug && character_id == 2 {
                    eprintln!("trace_move at {kind_at} movebits {nbits} x {x} y {y}");
                }
                finish(&mut current, &mut contour, &mut regions, &fills, &mut closed);
                position = (x, y);
                start = (x, y);
            }
            let fill0 = if flags & 0x02 != 0 { bits.bits(fill_bits)? as usize } else { 0 };
            let fill1 = if flags & 0x04 != 0 { bits.bits(fill_bits)? as usize } else { 0 };
            if flags & 0x08 != 0 {
                bits.bits(line_bits)?;
            }
            if flags & 0x10 != 0 {
                // A style-change record may redeclare the style arrays: the record stream continues
                // under the new arrays and new widths, it does not end here. On this path the arrays
                // begin at the current bit position; they are not byte-aligned here.
                if debug {
                    eprintln!(
                        "trace_newstyles id {character_id} at {} kind 0 flags_at {kind_at}",
                        bits.bit
                    );
                }
                fills = match read_fills(&mut bits, code, with_alpha, grad_alpha && with_alpha, matrix_align) {
                    Some(fills) => fills,
                    None => {
                        if debug {
                            eprintln!("shape_fill_none id {character_id} at {}", bits.bit);
                        }
                        return None;
                    }
                };
                line_count = skip_lines(&mut bits, code, with_alpha)?;
                bits.align();
                let widths = bits.byte()?;
                fill_bits = (widths >> 4) as usize;
                line_bits = (widths & 0x0F) as usize;
            }
            let declared = if fill1 != 0 {
                Some(fill1 - 1)
            } else if fill0 != 0 {
                Some(fill0 - 1)
            } else {
                None
            };
            if let Some(next) = declared {
                changes += 1;
                if current != Some(next) {
                    finish(&mut current, &mut contour, &mut regions, &fills, &mut closed);
                    current = Some(next);
                }
                if contour.is_empty() {
                    contour.push(position);
                }
            } else if flags & (0x02 | 0x04) != 0 {
                finish(&mut current, &mut contour, &mut regions, &fills, &mut closed);
                current = None;
            } else if current.is_some() && contour.is_empty() {
                contour.push(position);
            }
        } else if kind == 1 {
            let nbits = bits.bits(4)? as usize + 2;
            // A general line states both deltas; a non-general one states a single delta, vertical
            // or horizontal as the flag says. Reading presence bits the general form does not have
            // is what desynced every straight edge.
            let (dx, dy) = if bits.bit()? == 1 {
                (bits.signed(nbits)? as f64, bits.signed(nbits)? as f64)
            } else if bits.bit()? == 1 {
                (0.0, bits.signed(nbits)? as f64)
            } else {
                (bits.signed(nbits)? as f64, 0.0)
            };
            edges += 1;
            position = (position.0 + dx, position.1 + dy);
            if current.is_some() {
                if contour.is_empty() {
                    contour.push(start);
                }
                contour.push(position);
            }
        } else if kind == 2 {
            let nbits_at = bits.bit;
            let nbits = bits.bits(4)? as usize + 2;
            // A curved edge states its four deltas unconditionally: control, then anchor, each
            // relative to the point before it. Presence bits belong to the straight edge alone;
            // reading them here consumed a curve in ten bits and left the walk on padding.
            let control = (
                position.0 + bits.signed(nbits)? as f64,
                position.1 + bits.signed(nbits)? as f64,
            );
            let anchor = (
                control.0 + bits.signed(nbits)? as f64,
                control.1 + bits.signed(nbits)? as f64,
            );
            edges += 1;
            if current.is_some() {
                if contour.is_empty() {
                    contour.push(position);
                }
                for step in 1..=CURVE_STEPS {
                    let t = step as f64 / CURVE_STEPS as f64;
                    let one = 1.0 - t;
                    contour.push((
                        one * one * position.0 + 2.0 * one * t * control.0 + t * t * anchor.0,
                        one * one * position.1 + 2.0 * one * t * control.1 + t * t * anchor.1,
                    ));
                }
            }
            if debug && character_id == 2 {
                eprintln!(
                    "trace_curve at {kind_at} nbits_at {nbits_at} nbits {nbits} control ({},{}) anchor ({},{})",
                    control.0, control.1, anchor.0, anchor.1
                );
            }
            position = anchor;
        } else {
            break;
        }
    }
    finish(&mut current, &mut contour, &mut regions, &fills, &mut closed);
    bits.align();
    let records_end = bits.bit / 8;
    let exact = records_end == body.len();

    if debug && grad_alpha && matrix_align {
        let from = at.min(body.len());
        let to = if length <= 320 {
            (at + length).min(body.len())
        } else {
            (from + 64).min(body.len())
        };
        let head: Vec<String> = body[from..to].iter().map(|b| format!("{b:02x}")).collect();
        eprintln!("shape_at_styles {styles_end} from {from} bytes {}", head.join(" "));
    }

    if debug {
        eprintln!(
            "shape id {} code {code} tag {at}..{} styles_end {styles_end} records_end {records_end} exact {exact} grad_alpha {grad_alpha} matrix_align {matrix_align} fills {} lines {} changes {changes} edges {edges} closed {closed} regions {}",
            u16::from_le_bytes([body[at], body[at + 1]]),
            body.len(),
            fills.len(),
            line_count,
            regions.len()
        );
        let mut contours = 0usize;
        let mut degenerate = 0usize;
        let mut holes = 0usize;
        for region in &regions {
            for (index, contour) in region.contours.iter().enumerate() {
                contours += 1;
                if contour_area(contour).abs() < 1e-6 {
                    degenerate += 1;
                }
                let nested = region.contours.iter().enumerate().any(|(other, outer)| {
                    other != index
                        && contour
                            .first()
                            .is_some_and(|point| inside(&[outer.clone()], *point))
                });
                if nested {
                    holes += 1;
                }
            }
        }
        eprintln!(
            "geometry_areas regions {} contours {contours} degenerate {degenerate} holes {holes}",
            regions.len()
        );
    }
    Some(Geometry { regions, exact })
}

/// One fill style array: its count, then each fill's own fields. A gradient carries its matrix and
/// its stops, a bitmap the character it references and its matrix.
fn read_fills(
    bits: &mut Bits,
    _code: u16,
    with_alpha: bool,
    grad_alpha: bool,
    matrix_align: bool,
) -> Option<Vec<Option<Fill>>> {
    let fill_count = count(bits)?;
    let mut fills: Vec<Option<Fill>> = Vec::with_capacity(fill_count);
    for _ in 0..fill_count {
        match read_fill(bits, with_alpha, grad_alpha, matrix_align) {
            Some(fill) => fills.push(Some(fill)),
            None => {
                if std::env::var("SWF_SHAPE_DEBUG").is_ok() {
                    eprintln!(
                        "fill_array_failed at_bit {} at_byte {} fill {}/{}",
                        bits.bit,
                        bits.bit / 8,
                        fills.len(),
                        fill_count
                    );
                }
                return None;
            }
        }
    }
    Some(fills)
}

/// One FILLSTYLE record: its type byte, then the fields that type carries.
fn read_fill(
    bits: &mut Bits,
    with_alpha: bool,
    grad_alpha: bool,
    matrix_align: bool,
) -> Option<Fill> {
    let kind = octet(bits)?;
    match kind {
        0x00 => {
            let rgb = (octet(bits)?, octet(bits)?, octet(bits)?);
            if with_alpha {
                octet(bits)?;
            }
            Some(Fill::Solid(rgb))
        }
        0x10 | 0x12 => {
            let matrix = read_matrix(bits, matrix_align)?;
            let stops = read_gradient(bits, grad_alpha)?;
            let gradient = Gradient { stops, matrix };
            Some(if kind == 0x10 {
                Fill::Linear(gradient)
            } else {
                Fill::Radial(gradient)
            })
        }
        0x13 => {
            let matrix = read_matrix(bits, matrix_align)?;
            let stops = read_gradient(bits, grad_alpha)?;
            // A focal gradient appends exactly one FIXED8 focal point after the stop table.
            let focal = word(bits)? as i16 as f64 / 256.0;
            Some(Fill::Focal {
                gradient: Gradient { stops, matrix },
                focal,
            })
        }
        0x40..=0x43 => {
            let character = word(bits)?;
            let matrix = read_matrix(bits, matrix_align)?;
            Some(Fill::Bitmap { character, matrix })
        }
        other => {
            if std::env::var("SWF_SHAPE_DEBUG").is_ok() {
                eprintln!(
                    "fill_type_unknown {other:#04x} at_bit {} at_byte {}",
                    bits.bit,
                    bits.bit / 8
                );
            }
            None
        }
    }
}

/// Walk the line style array so the walk continues at the widths byte. Line styles are counted, not
/// carried: nothing in this package draws a stroke yet.
fn skip_lines(bits: &mut Bits, code: u16, with_alpha: bool) -> Option<usize> {
    let line_count = count(bits)?;
    for _ in 0..line_count {
        word(bits)?;
        if code == 83 {
            // LineStyle2: a two-byte flags word, a miter limit only for a miter join, then either
            // an inline fill style or the RGBA colour.
            // LineStyle2 flag bits, per the format's own masks: HasFill is bit 3 and JoinStyle is
            // bits 4-5 (bits 6-7 are the start cap). Reading them two bits to the left made every
            // miter limit conditional on the wrong flag and desynced the line array.
            let flags = word(bits)?;
            let has_fill = (flags >> 3) & 1;
            let join = (flags >> 4) & 0x3;
            if join == 2 {
                word(bits)?;
            }
            if has_fill == 1 {
                read_fill(bits, with_alpha, with_alpha, true)?;
            } else {
                octet(bits)?;
                octet(bits)?;
                octet(bits)?;
                octet(bits)?;
            }
        } else {
            octet(bits)?;
            octet(bits)?;
            octet(bits)?;
            if with_alpha {
                octet(bits)?;
            }
        }
    }
    Some(line_count)
}

/// The signed area of a closed contour (shoelace), in twips².
fn contour_area(contour: &[(f64, f64)]) -> f64 {
    let mut sum = 0.0;
    for index in 0..contour.len() {
        let a = contour[index];
        let b = contour[(index + 1) % contour.len()];
        sum += a.0 * b.1 - b.0 * a.1;
    }
    sum / 2.0
}

/// Whether `point` is inside the contours, by the non-zero winding rule.
pub fn inside(contours: &[Vec<(f64, f64)>], point: (f64, f64)) -> bool {
    let mut winding = 0i32;
    for contour in contours {
        if contour.len() < 3 {
            continue;
        }
        for index in 0..contour.len() {
            let a = contour[index];
            let b = contour[(index + 1) % contour.len()];
            if a.1 <= point.1 {
                if b.1 > point.1 && cross(a, b, point) > 0.0 {
                    winding += 1;
                }
            } else if b.1 <= point.1 && cross(a, b, point) < 0.0 {
                winding -= 1;
            }
        }
    }
    winding != 0
}

fn cross(a: (f64, f64), b: (f64, f64), point: (f64, f64)) -> f64 {
    (b.0 - a.0) * (point.1 - a.1) - (b.1 - a.1) * (point.0 - a.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_square_is_inside_and_outside() {
        let square = vec![vec![(0.0, 0.0), (100.0, 0.0), (100.0, 100.0), (0.0, 100.0)]];
        assert!(inside(&square, (50.0, 50.0)));
        assert!(!inside(&square, (150.0, 50.0)));
    }

    #[test]
    fn a_gradient_mixes_its_stops() {
        let gradient = Gradient {
            stops: vec![(0.0, (0, 0, 0)), (1.0, (200, 100, 50))],
            matrix: Map::IDENTITY,
        };
        assert_eq!(gradient.colour_at(0.5), Some((100, 50, 25)));
        assert_eq!(gradient.colour_at(-1.0), Some((0, 0, 0)));
    }

    #[test]
    fn a_map_inverts() {
        let map = Map {
            a: 2.0,
            b: 0.0,
            c: 0.0,
            d: 4.0,
            tx: 10.0,
            ty: -5.0,
        };
        let inverse = map.inverse().expect("invertible");
        let point = map.apply((3.0, 7.0));
        let back = inverse.apply(point);
        assert!((back.0 - 3.0).abs() < 1e-9);
        assert!((back.1 - 7.0).abs() < 1e-9);
    }

    #[test]
    fn an_empty_geometry_reports_itself() {
        assert!(Geometry::default().is_empty());
    }
}
