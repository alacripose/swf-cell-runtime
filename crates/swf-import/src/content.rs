//! Per-tag content, measured but not lowered.
//!
//! A shape record's identifier and bounds, a sprite's frame structure, and every media tag's kind and
//! byte size are read out of the file. Nothing here turns a tag body into geometry, a timeline or
//! behaviour: those representations are still undeclared (`docs/CELL-SCHEMA.md` Q-1…Q-6, media M-1…M-5,
//! D-3), so this module measures and stops.

use crate::header::{rect_at, Header, Rect, Refusal};
use crate::tags::{self, Tag};

/// The four media classes the tag stream actually carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MediaKind {
    Bitmap,
    Sound,
    Font,
    Text,
}

impl MediaKind {
    pub fn name(self) -> &'static str {
        match self {
            MediaKind::Bitmap => "bitmap",
            MediaKind::Sound => "sound",
            MediaKind::Font => "font",
            MediaKind::Text => "text",
        }
    }
}

/// A definition record that carries a shape: `DefineShape`, `DefineShape2/3/4`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShapeInfo {
    pub id: u16,
    pub bounds: Rect,
    pub code: u16,
    pub bytes: usize,
}

/// A `DefineSprite`: identifier, declared frame count, and the tags nested inside it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpriteInfo {
    pub id: u16,
    pub frames: u16,
    pub nested: usize,
    pub bytes: usize,
}

/// One media-carrying tag.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Media {
    pub kind: MediaKind,
    pub code: u16,
    /// The definition identifier where the tag carries one; `SoundStreamHead` carries none.
    pub id: Option<u16>,
    pub bytes: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Inventory {
    pub shapes: Vec<ShapeInfo>,
    pub sprites: Vec<SpriteInfo>,
    pub media: Vec<Media>,
    pub show_frames: usize,
}

impl Inventory {
    /// The union of every shape's bounds, in twips, once at least one shape exists.
    pub fn bounds(&self) -> Option<Rect> {
        let mut bounds: Option<Rect> = None;
        for shape in &self.shapes {
            bounds = Some(match bounds {
                None => shape.bounds,
                Some(current) => Rect {
                    x_min: current.x_min.min(shape.bounds.x_min),
                    x_max: current.x_max.max(shape.bounds.x_max),
                    y_min: current.y_min.min(shape.bounds.y_min),
                    y_max: current.y_max.max(shape.bounds.y_max),
                },
            });
        }
        bounds
    }

    pub fn media_bytes(&self, kind: MediaKind) -> usize {
        self.media
            .iter()
            .filter(|media| media.kind == kind)
            .map(|media| media.bytes)
            .sum()
    }

    pub fn media_count(&self, kind: MediaKind) -> usize {
        self.media.iter().filter(|media| media.kind == kind).count()
    }
}

fn classify(code: u16) -> Option<MediaKind> {
    match code {
        6 | 20 | 21 | 35 | 36 => Some(MediaKind::Bitmap),
        14 | 18 | 45 => Some(MediaKind::Sound),
        10 | 48 | 75 => Some(MediaKind::Font),
        11 | 33 | 37 => Some(MediaKind::Text),
        _ => None,
    }
}

fn carries_id(code: u16) -> bool {
    matches!(
        code,
        6 | 10 | 11 | 14 | 20 | 21 | 33 | 34 | 35 | 36 | 37 | 48 | 75
    )
}

pub fn survey(header: &Header, tags: &[Tag]) -> Result<Inventory, Refusal> {
    let mut inventory = Inventory::default();
    for tag in tags {
        let body = &header.body;
        match tag.code {
            1 => inventory.show_frames += 1,
            2 | 22 | 32 | 83 => {
                if tag.length < 2 {
                    return Err(Refusal::Truncated {
                        needed: tag.at + 2,
                        found: body.len(),
                    });
                }
                let id = u16::from_le_bytes([body[tag.at], body[tag.at + 1]]);
                let (bounds, _) = rect_at(body, tag.at + 2)?;
                inventory.shapes.push(ShapeInfo {
                    id,
                    bounds,
                    code: tag.code,
                    bytes: tag.length,
                });
            }
            39 => {
                if tag.length < 4 {
                    return Err(Refusal::Truncated {
                        needed: tag.at + 4,
                        found: body.len(),
                    });
                }
                let id = u16::from_le_bytes([body[tag.at], body[tag.at + 1]]);
                let frames = u16::from_le_bytes([body[tag.at + 2], body[tag.at + 3]]);
                let nested = tags::walk_range(body, tag.at + 4, tag.at + tag.length)?.len();
                inventory.sprites.push(SpriteInfo {
                    id,
                    frames,
                    nested,
                    bytes: tag.length,
                });
            }
            code => {
                if let Some(kind) = classify(code) {
                    let id = if carries_id(code) && tag.length >= 2 {
                        Some(u16::from_le_bytes([body[tag.at], body[tag.at + 1]]))
                    } else {
                        None
                    };
                    inventory.media.push(Media {
                        kind,
                        code,
                        id,
                        bytes: tag.length,
                    });
                }
            }
        }
    }
    Ok(inventory)
}
