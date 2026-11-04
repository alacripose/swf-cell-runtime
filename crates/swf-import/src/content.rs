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
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeInfo {
    pub id: u16,
    pub bounds: Rect,
    pub code: u16,
    pub bytes: usize,
    /// The definition's geometry: its solid fills and the contours each covers. `None` when the
    /// definition is not a shape, or when its style array carries a fill this reader cannot skip
    /// exactly — in which case the lowering falls back to the measured rectangle rather than
    /// inventing a picture.
    pub geometry: Option<crate::shape::Geometry>,
    /// The RGB of the definition's **first solid fill style**, when the first style is solid.
    ///
    /// A gradient or bitmap first style answers `None`: this reader does not model either, and a
    /// colour invented for one would be a claim the file did not make. The value is the file's own,
    /// read from the style array the definition carries, not from any rendering of it.
    pub fill: Option<(u8, u8, u8)>,
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
    /// The tag codes the sprite body holds, in stream order: a sprite measured with no extent is
    /// explained by what its body actually carries, not by the count of records.
    pub codes: Vec<u16>,
    /// Whether the decoded body carries any display record at all. A sprite made of nothing but a
    /// sound stream is empty, which is a fact about the file, not a measurement that failed.
    pub draws: bool,
    pub bounds: Option<Rect>,
    pub bytes: usize,
    /// The first solid fill colour the sprite's **first frame** places, so a cell drawn for a sprite
    /// carries the file's own colour instead of the unresolved default. `None` when the first frame
    /// places nothing whose colour this reader can read.
    pub fill: Option<(u8, u8, u8)>,
    /// The placements of the sprite's **first frame**, in stream order and in the sprite's own
    /// coordinates. A sprite on the stage draws these, transformed by the sprite's placement — so
    /// lowering a sprite by its bounding box alone draws one rectangle where the file draws a scene.
    pub first: Vec<crate::placement::Placement>,
    /// **Every frame's display list, in order.** Each `ShowFrame` inside the sprite closes a frame,
    /// and this is the list in force at that moment. A sprite is a timeline inside the movie's
    /// timeline, so lowering movie frame *n* draws the sprite's frame *n*: reading only the first
    /// frame made every movie frame identical and a loading bar stand still for the whole movie.
    /// A sprite with fewer frames than the movie shows its last frame, which is what it states.
    pub frames_list: Vec<Vec<crate::placement::Placement>>,
}

/// One sound the package carries: the character's identity, its declared encoding, and its payload
/// bytes exactly as the file states them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SoundInfo {
    /// `sound` for a `DefineSound` character, `stream` for a `SoundStreamBlock`.
    pub kind: &'static str,
    /// The character id; `None` for a stream block, which carries none.
    pub id: Option<u16>,
    pub format: &'static str,
    /// The source's own `SoundFormat` byte, carried so a reader can check the encoding the labels
    /// were derived from rather than trusting them.
    pub format_byte: u8,
    pub rate: u32,
    pub samples: u32,
    pub bytes: Vec<u8>,
}

/// The samples-per-second a `DefineSound` rate code names (bits 2-3 of the format byte).
fn sound_rate(format: u8) -> u32 {
    match (format >> 2) & 0x3 {
        0 => 5_512,
        1 => 11_025,
        2 => 22_050,
        _ => 44_100,
    }
}

/// The encoding a compression nibble names (bits 4-7 of the format byte). An unknown nibble is
/// named rather than guessed.
fn sound_format(format: u8) -> &'static str {
    match format >> 4 {
        0 => "pcm",
        1 => "adpcm",
        2 => "mp3",
        3 => "pcm-le",
        4 | 5 | 6 => "nellymoser",
        11 => "speex",
        _ => "unknown",
    }
}

/// Read one sound tag's payload: `DefineSound` states id, format and sample count before its data; a
/// `SoundStreamBlock` is payload alone.
fn sound_clip(body: &[u8], tag: &Tag) -> Option<SoundInfo> {
    let end = tag.at.checked_add(tag.length)?;
    if end > body.len() {
        return None;
    }
    match tag.code {
        14 if tag.length >= 8 => {
            let id = u16::from_le_bytes([body[tag.at], body[tag.at + 1]]);
            let format = body[tag.at + 2];
            let samples = u32::from_le_bytes([
                body[tag.at + 3],
                body[tag.at + 4],
                body[tag.at + 5],
                body[tag.at + 6],
            ]);
            let bytes = body[tag.at + 7..end].to_vec();
            if bytes.is_empty() {
                return None;
            }
            Some(SoundInfo {
                kind: "sound",
                id: Some(id),
                format: sound_format(format),
                format_byte: format,
                rate: sound_rate(format),
                samples,
                bytes,
            })
        }
        45 => {
            let bytes = body[tag.at..end].to_vec();
            if bytes.is_empty() {
                return None;
            }
            Some(SoundInfo {
                kind: "stream",
                id: None,
                format: "mp3",
                format_byte: 0,
                rate: 0,
                samples: 0,
                bytes,
            })
        }
        _ => None,
    }
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

#[derive(Clone, Debug, Default, PartialEq)]
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
    /// The sounds this file states, payload included. A sound measured only by its tag length is a
    /// count of audio; these are the bytes a player can consume.
    pub sounds: Vec<SoundInfo>,
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
    let (bounds, used) = rect_at(body, tag.at + 2)?;
    let fill = first_solid_fill(body, tag.at + 2 + used, tag.code);
    // Geometry is read for the four shape definitions only: a button or an edit-text field states a
    // rectangle, not edges.
    let geometry = match tag.code {
        2 | 22 | 32 | 83 => crate::shape::read(body, tag.at, tag.length, tag.code),
        _ => None,
    };
    inventory.shapes.push(ShapeInfo {
        id,
        bounds,
        code: tag.code,
        bytes: tag.length,
        fill,
        geometry,
    });
    Ok(())
}

/// The first solid fill style's RGB, when the definition's first style is solid.
///
/// The style array follows the bounds rectangle: a count byte, extended to a word when it is 0xFF,
/// then the styles themselves. Only the first style is read, because only the first is needed to
/// answer "what colour is this definition", and a style this reader does not model answers `None`
/// rather than a guess.
fn first_solid_fill(body: &[u8], mut at: usize, code: u16) -> Option<(u8, u8, u8)> {
    // `DefineShape4` carries an edge-bounds rectangle and a flags byte before its style array.
    if code == 83 {
        let (_, used) = rect_at(body, at).ok()?;
        at = at.checked_add(used)?.checked_add(1)?;
    }
    let mut count = *body.get(at)? as usize;
    at += 1;
    if count == 0xFF {
        if body.len() < at + 2 {
            return None;
        }
        count = u16::from_le_bytes([body[at], body[at + 1]]) as usize;
        at += 2;
    }
    if count == 0 {
        return None;
    }
    match *body.get(at)? {
        0x00 if body.len() >= at + 4 => Some((body[at + 1], body[at + 2], body[at + 3])),
        _ => None,
    }
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
        fill: None,
        geometry: None,
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
                let nested_tags = tags::walk_range(body, tag.at + 4, tag.at + tag.length)?;
                inventory.sprites.push(SpriteInfo {
                    id,
                    frames,
                    nested: nested_tags.len(),
                    codes: nested_tags.iter().map(|tag| tag.code).collect(),
                    draws: nested_tags.iter().any(|tag| is_placement(tag.code)),
                    bounds: None,
                    bytes: tag.length,
                    fill: None,
                    first: first_frame(body, tag.at, tag.length),
                    frames_list: sprite_frames(body, tag.at, tag.length),
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
                    codes: nested_tags.iter().map(|tag| tag.code).collect(),
                    draws: nested_tags.iter().any(|tag| is_placement(tag.code)),
                    bounds: None,
                    bytes: tag.length,
                    fill: None,
                    first: first_frame(body, tag.at, tag.length),
                    frames_list: sprite_frames(body, tag.at, tag.length),
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
                if let Some(sound) = sound_clip(body, tag) {
                    inventory.sounds.push(sound);
                }
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
    // A sprite's extent may be stated by sprites it places, so the measurement is resolved in
    // passes: each pass can size every sprite whose own placements are already sized, and the
    // passes stop when one changes nothing. The ceiling is the nesting depth `collect` walks, so a
    // malformed cycle cannot spin here.
    // Paired with the sprite's own id: a sprite's position in `sprites` is the order it was read in,
    // never its identifier, so a bounds lookup keyed by position would size the wrong definition.
    let mut resolved: Vec<(u16, Option<Rect>, Option<(u8, u8, u8)>)> = inventory
        .sprites
        .iter()
        .map(|sprite| (sprite.id, sprite.bounds, sprite.fill))
        .collect();
    for _ in 0..8 {
        let mut changed = false;
        for (index, at, length) in &sprite_sites {
            // **Resolved once, never revised.** Re-measuring a sprite across passes composes its
            // nested transforms again on every pass, so a sprite that places a sprite which places
            // it back grows without bound — which is how an extent came out millions of cells wide.
            if resolved[*index].1.is_some() {
                continue;
            }
            let found = sprite_bounds(
                &header.body,
                *at,
                *length,
                &inventory.shapes,
                &resolved,
                &inventory.texts,
            )?;
            if found.0 != resolved[*index].1 || found.1 != resolved[*index].2 {
                resolved[*index].1 = found.0;
                resolved[*index].2 = found.1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for (index, _, _) in &sprite_sites {
        inventory.sprites[*index].bounds = resolved[*index].1;
        inventory.sprites[*index].fill = resolved[*index].2;
    }
    Ok(inventory)
}

/// Whether a tag code is a display record: the three placement forms this reader reads. Named once so
/// "this sprite draws nothing" is one answer about the file rather than a fourth tag-code list.
fn is_placement(code: u16) -> bool {
    matches!(code, 4 | 26 | 70)
}

/// The placements a sprite shows in its **first frame**: the records before the first `ShowFrame`.
///
/// Each is read tolerantly, because a record's geometry precedes whatever tail it carries; a record
/// whose geometry itself is unreadable is dropped here and counted by the placement survey, not
/// invented.
fn first_frame(body: &[u8], at: usize, length: usize) -> Vec<crate::placement::Placement> {
    let mut placements = Vec::new();
    let Ok(nested_tags) = tags::walk_range(body, at + 4, at + length) else {
        return placements;
    };
    for nested in nested_tags {
        if nested.code == 1 {
            // A frame is the placements before its `ShowFrame`. The **first frame that places
            // anything** is what the sprite shows when it is mounted: a sprite whose frame 1 is
            // empty would otherwise be an unmeasurable hole even though its next frame draws.
            if !placements.is_empty() {
                return placements;
            }
            continue;
        }
        let end = nested.at + nested.length;
        if end > body.len() {
            continue;
        }
        let slice = &body[nested.at..end];
        let read = match nested.code {
            4 => crate::placement::place_object_tolerant(slice),
            26 => crate::placement::place_object_2_tolerant(slice),
            70 => crate::placement::place_object_3_tolerant(slice),
            _ => continue,
        };
        if let Ok((placement, _)) = read {
            placements.push(placement);
        }
    }
    placements
}

/// The extent a sprite shows, in twips: every shape its own placements put inside it, transformed by
/// that placement's matrix, unioned. Nested sprites are not composed — a sprite inside a sprite is
/// one level deeper than this measurement goes, and it is named rather than guessed at.
/// The bounds a character states, whatever kind of definition it is: a shape, a sprite already
/// sized, or a text rectangle. `None` means no definition this survey measured states one.
fn character_bounds(
    character: u16,
    shapes: &[ShapeInfo],
    sprites: &[(u16, Option<Rect>, Option<(u8, u8, u8)>)],
    texts: &[ShapeInfo],
) -> Option<Rect> {
    shapes
        .iter()
        .find(|shape| shape.id == character)
        .map(|shape| shape.bounds)
        .or_else(|| {
            sprites
                .iter()
                .find(|(id, _, _)| *id == character)
                .and_then(|(_, bounds, _)| *bounds)
        })
        .or_else(|| {
            texts
                .iter()
                .find(|text| text.id == character)
                .map(|text| text.bounds)
        })
}

/// **Every frame of a sprite, read by the same timeline walk the stage uses.** A sprite's body is a
/// tag stream like the movie's: placements accumulate, `ShowFrame` closes a frame, and a removal
/// takes a depth away. Reading it frame by frame is what lets the client advance the movie's
/// contents instead of repeating the sprite's first frame forever.
fn sprite_frames(body: &[u8], at: usize, length: usize) -> Vec<Vec<crate::placement::Placement>> {
    let Ok(nested) = tags::walk_range(body, at + 4, at + length) else {
        return Vec::new();
    };
    match crate::placement::timeline(&nested, body) {
        Ok(states) => states
            .into_iter()
            .map(|state| state.placements)
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn sprite_bounds(
    body: &[u8],
    at: usize,
    length: usize,
    shapes: &[ShapeInfo],
    sprites: &[(u16, Option<Rect>, Option<(u8, u8, u8)>)],
    texts: &[ShapeInfo],
) -> Result<(Option<Rect>, Option<(u8, u8, u8)>), Refusal> {
    let mut bounds: Option<Rect> = None;
    let mut colour: Option<(u8, u8, u8)> = None;
    for nested in tags::walk_range(body, at + 4, at + length)? {
        // The extent this sprite shows **at mount** is its first frame's display list: the records
        // before the first `ShowFrame`. Unioning every frame of a moving sprite measures the whole
        // journey instead of the picture, which is how a 930-frame sprite came out hundreds of
        // thousands of cells wide. The frame count stays declared; only the measured extent is the
        // first frame's, and that is the state the package's first frame draws.
        if nested.code == 1 {
            break;
        }
        let end = nested.at + nested.length;
        if end > body.len() {
            continue;
        }
        let slice = &body[nested.at..end];
        // A sprite body's records are as variable as the top level's: a clip action or a field this
        // reader does not model comes after the geometry, so the tolerant read sizes the sprite
        // from fields that are still the file's own. A record whose geometry itself is unreadable is
        // skipped, and the sprite's extent is then the union of the records that did read.
        let placement = match nested.code {
            4 => crate::placement::place_object_tolerant(slice),
            26 => crate::placement::place_object_2_tolerant(slice),
            70 => crate::placement::place_object_3_tolerant(slice),
            _ => continue,
        };
        let Ok((placement, _)) = placement else { continue };
        let Some(base) = character_bounds(placement.character, shapes, sprites, texts) else {
            continue;
        };
        if colour.is_none() {
            colour = shapes
                .iter()
                .find(|shape| shape.id == placement.character)
                .and_then(|shape| shape.fill);
        }
        let (scale_x, scale_y) = match placement.matrix {
            Some(matrix) => matrix.scale(),
            None => (1.0, 1.0),
        };
        let (translate_x, translate_y) = match placement.matrix {
            Some(matrix) => matrix.translation_px(),
            None => (0, 0),
        };
        let found = Rect {
            x_min: (base.x_min as f64 * scale_x) as i32 + translate_x * 20,
            x_max: (base.x_max as f64 * scale_x) as i32 + translate_x * 20,
            y_min: (base.y_min as f64 * scale_y) as i32 + translate_y * 20,
            y_max: (base.y_max as f64 * scale_y) as i32 + translate_y * 20,
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
    // A sprite whose first frame places nothing still has an extent: the shapes it *defines* have
    // decoded contours, and their extent is the sprite's own. Measuring it from the geometry the
    // file states is the answer a missing placement rectangle cannot give.
    if bounds.is_none() {
        for nested in tags::walk_range(body, at + 4, at + length)? {
            if !matches!(nested.code, 2 | 22 | 32 | 83) || nested.length < 2 {
                continue;
            }
            let id = u16::from_le_bytes([body[nested.at], body[nested.at + 1]]);
            let Some(shape) = shapes.iter().find(|shape| shape.id == id) else {
                continue;
            };
            let Some((x_min, y_min, x_max, y_max)) =
                shape.geometry.as_ref().and_then(|geometry| geometry.bounds())
            else {
                continue;
            };
            let found = Rect {
                x_min: x_min as i32,
                x_max: x_max as i32,
                y_min: y_min as i32,
                y_max: y_max as i32,
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
    }
    Ok((bounds, colour))
}
