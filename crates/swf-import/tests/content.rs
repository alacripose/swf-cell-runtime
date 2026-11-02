//! Deeper measurement is proven on synthetic fixtures: shape bounds, sprite structure, media kinds.

use planet_package::mount;
use swf_import::content::{self, MediaKind};
use swf_import::header;

fn push_bits(out: &mut Vec<u8>, value: i32, nbits: usize) {
    for index in (0..nbits).rev() {
        out.push(((value >> index) & 1) as u8);
    }
}

fn rect_bytes(x_min: i32, x_max: i32, y_min: i32, y_max: i32, nbits: usize) -> Vec<u8> {
    let mut bits = Vec::new();
    push_bits(&mut bits, nbits as i32, 5);
    for value in [x_min, x_max, y_min, y_max] {
        push_bits(&mut bits, value, nbits);
    }
    let mut bytes = vec![0u8; (bits.len() + 7) / 8];
    for (index, bit) in bits.iter().enumerate() {
        if *bit == 1 {
            bytes[index / 8] |= 1 << (7 - (index % 8));
        }
    }
    bytes
}

fn tag(code: u16, body: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    if body.len() < 0x3f {
        out.extend_from_slice(&((code << 6) | body.len() as u16).to_le_bytes());
    } else {
        out.extend_from_slice(&((code << 6) | 0x3f).to_le_bytes());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    }
    out.extend_from_slice(body);
    out
}

fn body() -> Vec<u8> {
    let mut body = rect_bytes(0, 12_800, 0, 9_600, 16); // 640x480
    body.extend_from_slice(&10_240u16.to_le_bytes());
    body.extend_from_slice(&1u16.to_le_bytes());

    let mut shape = Vec::new();
    shape.extend_from_slice(&7u16.to_le_bytes());
    shape.extend(rect_bytes(0, 2_000, 0, 1_000, 15)); // 100x50
    body.extend(tag(2, &shape));

    let mut sprite = Vec::new();
    sprite.extend_from_slice(&9u16.to_le_bytes());
    sprite.extend_from_slice(&3u16.to_le_bytes());
    for _ in 0..3 {
        sprite.extend(tag(1, &[]));
    }
    sprite.extend_from_slice(&0u16.to_le_bytes());
    body.extend(tag(39, &sprite));

    let mut jpeg = Vec::new();
    jpeg.extend_from_slice(&5u16.to_le_bytes());
    jpeg.extend_from_slice(&[9, 9, 9, 9]);
    body.extend(tag(21, &jpeg));

    let mut sound = Vec::new();
    sound.extend_from_slice(&6u16.to_le_bytes());
    sound.extend_from_slice(&[1, 2, 3]);
    body.extend(tag(14, &sound));

    body.extend(tag(1, &[]));
    body.extend_from_slice(&0u16.to_le_bytes());
    body
}

fn fixture() -> Vec<u8> {
    let body = body();
    let mut file = Vec::new();
    file.extend_from_slice(b"FWS");
    file.push(8);
    file.extend_from_slice(&((8 + body.len()) as u32).to_le_bytes());
    file.extend_from_slice(&body);
    file
}

fn survey(file: &[u8]) -> (header::Header, Vec<swf_import::Tag>, content::Inventory) {
    let header = header::read(file).expect("a readable header");
    let tags = swf_import::tags::walk(&header).expect("a walkable stream");
    let inventory = content::survey(&header, &tags).expect("a measurable body");
    (header, tags, inventory)
}

#[test]
fn shape_bounds_are_measured() {
    let (_, _, inventory) = survey(&fixture());
    assert_eq!(inventory.shapes.len(), 1);
    let shape = &inventory.shapes[0];
    assert_eq!(shape.id, 7);
    assert_eq!(shape.code, 2);
    assert_eq!(shape.bounds.width_px(), 100);
    assert_eq!(shape.bounds.height_px(), 50);
    let union = inventory.bounds().expect("one shape has bounds");
    assert_eq!(union.width_px(), 100);
}

#[test]
fn sprite_structure_is_measured() {
    let (_, _, inventory) = survey(&fixture());
    assert_eq!(inventory.sprites.len(), 1);
    let sprite = &inventory.sprites[0];
    assert_eq!(sprite.id, 9);
    assert_eq!(sprite.frames, 3);
    assert_eq!(sprite.nested, 3, "three nested show-frame records");
    assert_eq!(inventory.show_frames, 1, "one top-level show frame");
}

#[test]
fn media_is_inventoried_by_kind_and_size() {
    let (_, _, inventory) = survey(&fixture());
    assert_eq!(inventory.media_count(MediaKind::Bitmap), 1);
    assert_eq!(inventory.media_bytes(MediaKind::Bitmap), 6);
    assert_eq!(inventory.media_count(MediaKind::Sound), 1);
    assert_eq!(inventory.media_bytes(MediaKind::Sound), 5);
    assert_eq!(inventory.media_count(MediaKind::Font), 0);
    assert_eq!(inventory.media_count(MediaKind::Text), 0);
    let bitmap = inventory
        .media
        .iter()
        .find(|media| media.kind == MediaKind::Bitmap)
        .expect("the jpeg tag");
    assert_eq!(bitmap.id, Some(5));
}

#[test]
fn a_sound_stream_head_has_no_definition_id() {
    let mut body = rect_bytes(0, 12_800, 0, 9_600, 16);
    body.extend_from_slice(&10_240u16.to_le_bytes());
    body.extend_from_slice(&1u16.to_le_bytes());
    body.extend(tag(18, &[0x00, 0x12, 0x00, 0x00]));
    body.extend(tag(1, &[]));
    body.extend_from_slice(&0u16.to_le_bytes());
    let mut file = Vec::new();
    file.extend_from_slice(b"FWS");
    file.push(8);
    file.extend_from_slice(&((8 + body.len()) as u32).to_le_bytes());
    file.extend_from_slice(&body);

    let (_, _, inventory) = survey(&file);
    let head = inventory
        .media
        .iter()
        .find(|media| media.code == 18)
        .expect("the stream head");
    assert_eq!(head.id, None, "a stream head carries a rate, not a definition id");
    assert_eq!(head.kind, MediaKind::Sound);
    assert_eq!(head.bytes, 4);
}

#[test]
fn the_emitted_package_carries_the_deeper_measurements() {
    let (header, tags, inventory) = survey(&fixture());
    let counts = swf_import::tags::counts(&tags);
    let frame = swf_import::emit::package_frame("swf1", &header, &counts, tags.len(), &inventory)
        .expect("an emitted frame");
    let receipt = mount(&frame, "swf1", 1, 0, 0).expect("the package mounts");
    assert_eq!(receipt.windows, 1);

    let doc = swf_import::emit::surface_document("swf1", &header, &counts, tags.len(), &inventory);
    assert!(doc.contains("shapes 1; union 100x50 px"), "{doc}");
    assert!(doc.contains("sprites 1; 3 nested tags"), "{doc}");
    assert!(doc.contains("bitmap 1 / 6 bytes"), "{doc}");
    assert!(doc.contains("sound 1 / 5 bytes"), "{doc}");
    assert!(doc.contains("1 show-frame records"), "{doc}");
}
