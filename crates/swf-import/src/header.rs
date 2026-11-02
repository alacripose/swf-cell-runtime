//! The SWF header: signature, compression, stage rectangle, frame rate and frame count.
//!
//! The signature is read from the bytes, never from the file extension. `FWS` is read as it stands,
//! `CWS` is inflated, and `ZWS` is refused by name because this importer carries no LZMA decoder yet.

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compression {
    None,
    Zlib,
    Lzma,
}

impl Compression {
    pub fn name(self) -> &'static str {
        match self {
            Compression::None => "none",
            Compression::Zlib => "zlib",
            Compression::Lzma => "lzma",
        }
    }
}

/// The stage rectangle in twips, as the header declares it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
}

impl Rect {
    pub fn width_px(self) -> i32 {
        (self.x_max - self.x_min) / 20
    }

    pub fn height_px(self) -> i32 {
        (self.y_max - self.y_min) / 20
    }
}

#[derive(Clone, Debug)]
pub struct Header {
    pub compression: Compression,
    pub version: u8,
    pub declared_len: u32,
    pub stage: Rect,
    pub frame_rate: f64,
    pub frame_count: u16,
    /// The decompressed body, header fields included.
    pub body: Vec<u8>,
    /// Where the tag stream starts in `body`.
    pub tag_start: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Refusal {
    Signature { found: String },
    Compression { found: String },
    Truncated { needed: usize, found: usize },
    Frame(String),
}

impl Refusal {
    pub fn code(&self) -> &'static str {
        match self {
            Refusal::Signature { .. } => "swf-signature",
            Refusal::Compression { .. } => "swf-compression",
            Refusal::Truncated { .. } => "swf-truncated",
            Refusal::Frame(_) => "swf-frame",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Refusal::Signature { found } => format!("signature {found:?} is not FWS, CWS or ZWS"),
            Refusal::Compression { found } => {
                format!("compression {found} is not supported by this importer")
            }
            Refusal::Truncated { needed, found } => format!("needs {needed} bytes and has {found}"),
            Refusal::Frame(why) => format!("the emitted package was refused: {why}"),
        }
    }
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code(), self.message())
    }
}

pub fn read(bytes: &[u8]) -> Result<Header, Refusal> {
    if bytes.len() < 8 {
        return Err(Refusal::Truncated {
            needed: 8,
            found: bytes.len(),
        });
    }
    let compression = match &bytes[0..3] {
        b"FWS" => Compression::None,
        b"CWS" => Compression::Zlib,
        b"ZWS" => Compression::Lzma,
        other => {
            return Err(Refusal::Signature {
                found: String::from_utf8_lossy(other).to_string(),
            })
        }
    };
    if compression == Compression::Lzma {
        return Err(Refusal::Compression {
            found: "lzma".to_string(),
        });
    }
    let version = bytes[3];
    let declared_len = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let body = match compression {
        Compression::Zlib => {
            use flate2::read::ZlibDecoder;
            use std::io::Read;
            let mut out = Vec::new();
            ZlibDecoder::new(&bytes[8..])
                .read_to_end(&mut out)
                .map_err(|_| Refusal::Compression {
                    found: "zlib stream unreadable".to_string(),
                })?;
            out
        }
        _ => bytes[8..].to_vec(),
    };

    let (stage, at) = parse_rect(&body)?;
    if body.len() < at + 4 {
        return Err(Refusal::Truncated {
            needed: at + 4,
            found: body.len(),
        });
    }
    let frame_rate = u16::from_le_bytes([body[at], body[at + 1]]) as f64 / 256.0;
    let frame_count = u16::from_le_bytes([body[at + 2], body[at + 3]]);

    Ok(Header {
        compression,
        version,
        declared_len,
        stage,
        frame_rate,
        frame_count,
        body,
        tag_start: at + 4,
    })
}

fn parse_rect(body: &[u8]) -> Result<(Rect, usize), Refusal> {
    if body.is_empty() {
        return Err(Refusal::Truncated {
            needed: 1,
            found: 0,
        });
    }
    let nbits = (body[0] >> 3) as usize;
    let needed = (5 + 4 * nbits + 7) / 8;
    if body.len() < needed {
        return Err(Refusal::Truncated {
            needed,
            found: body.len(),
        });
    }
    let x_min = signed_bits(body, 5, nbits);
    let x_max = signed_bits(body, 5 + nbits, nbits);
    let y_min = signed_bits(body, 5 + 2 * nbits, nbits);
    let y_max = signed_bits(body, 5 + 3 * nbits, nbits);
    Ok((
        Rect {
            x_min,
            x_max,
            y_min,
            y_max,
        },
        needed,
    ))
}

/// One signed bit field, most significant bit first, as the SWF bit stream writes it.
fn signed_bits(bytes: &[u8], start: usize, nbits: usize) -> i32 {
    if nbits == 0 {
        return 0;
    }
    let mut value: i64 = 0;
    for index in 0..nbits {
        let position = start + index;
        let bit = (bytes[position / 8] >> (7 - (position % 8))) & 1;
        value = (value << 1) | bit as i64;
    }
    if value & (1i64 << (nbits - 1)) != 0 {
        value -= 1i64 << nbits;
    }
    value as i32
}
