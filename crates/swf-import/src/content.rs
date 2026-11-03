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

/// A `DefineSprite`: identifier, declared frame count, the tags nested inside it, and the extent of
/// the shapes it defines.
///
/// The bounds are the union of the nested shape definitions' own rectangles, in twips. They are the
/// sprite's content extent as measured, not a transformed display extent: the placements inside the
/// sprite are not composed, which is named rather than guessed at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpriteInfo {
    pub id: u16,
    pub frames: u16,
    pub nested: usize,
    pub bounds: Option<Rect>,
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
    /// Text characters (`DefineText`, `DefineText2`) with their measured text-bounds rectangle.
    ///
    /// They are characters a display list can place, so their extent is measured here even though no
    /// glyph run is lowered yet: a placement of a text character has a rectangle, and inventing none
    /// would report it as unmeasurable when the file states it.
    pub texts: Vec<ShapeInfo>,
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

/// Read one shape definition into the inventory.
fn shape_into(body: &[u8], tag: &Tag, inventory: &mut Inventory) -> Result<(), Refusal> {
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
    Ok(())
}

/// Read one text definition's measured rectangle into the inventory.
fn text_into(body: &[u8], tag: &Tag, inventory: &mut Inventory) -> Result<(), Refusal> {
    if tag.length < 2 {
        return Err(Refusal::Truncated {
            needed: tag.at + 2,
            found: body.len(),
        });
    }
    let id = u16::from_le_bytes([body[tag.at], body[tag.at + 1]]);
    let (bounds, _) = rect_at(body, tag.at + 2)?;
    inventory.texts.push(ShapeInfo {
        id,
        bounds,
        code: tag.code,
        bytes: tag.length,
    });
    Ok(())
}

/// The definitions a sprite body carries. A file may define a character inside a sprite and place it
/// from the top-level display list, so those definitions are collected too; the depth ceiling keeps a
/// malformed nesting from being walked forever, and the ceiling is a bound rather than a licence to
/// guess.
fn collect(
    body: &[u8],
    start: usize,
    end: usize,
    inventory: &mut Inventory,
    sites: &mut Vec<(usize, usize, usize)>,
    depth: usize,
) -> Result<(), Refusal> {
    if depth > 8 {
        return Ok(());
    }
    for tag in tags::walk_range(body, start, end)? {
        match tag.code {
            2 | 22 | 32 | 83 => shape_into(body, &tag, inventory)?,
            11 | 33 => text_into(body, &tag, inventory)?,
            39 => {
                if tag.length < 4 {
                    continue;
                }
                let id = u16::from_le_bytes([body[tag.at], body[tag.at + 1]]);
                let frames = u16::from_le_bytes([body[tag.at + 2], body[tag.at + 3]]);
                let nested = tags::walk_range(body, tag.at + 4, tag.at + tag.length)?.len();
                inventory.sprites.push(SpriteInfo {
                    id,
                    frames,
                    nested,
                    bounds: None,
                    bytes: tag.length,
                });
                sites.push((inventory.sprites.len() - 1, tag.at, tag.length));
                collect(body, tag.at + 4, tag.at + tag.length, inventory, sites, depth + 1)?;
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn survey(header: &Header, tags: &[Tag]) -> Result<Inventory, Refusal> {
    let mut inventory = Inventory::default();
    // Sprites are measured after the shapes are known: a sprite usually *places* top-level shapes
    // rather than defining its own, so its extent can only be computed once every shape is read.
    let mut sprite_sites: Vec<(usize, usize, usize)> = Vec::new();
    for tag in tags {
        let body = &header.body;
        match tag.code {
            1 => inventory.show_frames += 1,
            // Every definition that states an identifier and then a bounds rectangle is a character
            // the display list can place: shapes, morph shapes, buttons and edit-text fields.
            2 | 22 | 32 | 83 | 34 | 37 | 46 | 84 => shape_into(body, tag, &mut inventory)?,
            11 | 33 => text_into(body, tag, &mut inventory)?,
            39 => {
                if tag.length < 4 {
                    return Err(Refusal::Truncated {
                        needed: tag.at + 4,
                        found: body.len(),
                    });
                }
                let id = u16::from_le_bytes([body[tag.at], body[tag.at + 1]]);
                let frames = u16::from_le_bytes([body[tag.at + 2], body[tag.at + 3]]);
                let nested_tags = tags::walk_range(body, tag.at + 4, tag.at + tag.length)?;
                inventory.sprites.push(SpriteInfo {
                    id,
                    frames,
                    nested: nested_tags.len(),
                    bounds: None,
                    bytes: tag.length,
                });
                sprite_sites.push((inventory.sprites.len() - 1, tag.at, tag.length));
                // Characters the sprite body defines are placed from the top level in this format,
                // so they have to be measured here or their placements have no extent.
                collect(
                    body,
                    tag.at + 4,
                    tag.at + tag.length,
                    &mut inventory,
                    &mut sprite_sites,
                    1,
                )?;
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
    for (index, at, length) in sprite_sites {
        inventory.sprites[index].bounds =
            sprite_bounds(&header.body, at, length, &inventory.shapes)?;
    }
    Ok(inventory)
}

/// The extent a sprite shows, in twips: every shape its own placements put inside it, transformed by
/// that placement's matrix, unioned. Nested sprites are not composed — a sprite inside a sprite is
/// one level deeper than this measurement goes, and it is named rather than guessed at.
fn sprite_bounds(
    body: &[u8],
    at: usize,
    length: usize,
    shapes: &[ShapeInfo],
) -> Result<Option<Rect>, Refusal> {
    let mut bounds: Option<Rect> = None;
    for nested in tags::walk_range(body, at + 4, at + length)? {
        let end = nested.at + nested.length;
        if end > body.len() {
            continue;
        }
        let slice = &body[nested.at..end];
        let placement = match nested.code {
            4 => crate::placement::place_object(slice),
            26 => crate::placement::place_object_2(slice),
            _ => continue,
        };
        let Ok(placement) = placement else { continue };
        let Some(shape) = shapes.iter().find(|shape| shape.id == placement.character) else {
            continue;
        };
        let (scale_x, scale_y) = match placement.matrix {
            Some(matrix) => matrix.scale(),
            None => (1.0, 1.0),
        };
        let (translate_x, translate_y) = match placement.matrix {
            Some(matrix) => matrix.translation_px(),
            None => (0, 0),
        };
        let found = Rect {
            x_min: (shape.bounds.x_min as f64 * scale_x) as i32 + translate_x * 20,
            x_max: (shape.bounds.x_max as f64 * scale_x) as i32 + translate_x * 20,
            y_min: (shape.bounds.y_min as f64 * scale_y) as i32 + translate_y * 20,
            y_max: (shape.bounds.y_max as f64 * scale_y) as i32 + translate_y * 20,
        };
        bounds = Some(match bounds {
            None => found,
            Some(current) => Rect {
                x_min: current.x_min.min(found.x_min),
                x_max: current.x_max.max(found.x_max),
                y_min: current.y_min.min(found.y_min),
                y_max: current.y_max.max(found.y_max),
            },
        });
    }
    Ok(bounds)
}
