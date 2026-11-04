//! Placement parsing, proved on synthetic bodies: no byte of any supplied movie appears here.

use swf_import::placement::{self, Matrix, Placement, Refusal};
use swf_import::Tag;

/// A bit writer for fixture bodies, most significant bit first.
struct Writer {
    bytes: Vec<u8>,
    bit: usize,
}

impl Writer {
    fn new() -> Writer {
        Writer {
            bytes: Vec::new(),
            bit: 0,
        }
    }

    fn bit(&mut self, value: u32) {
        if self.bit % 8 == 0 {
            self.bytes.push(0);
        }
        if value & 1 == 1 {
            let index = self.bit / 8;
            self.bytes[index] |= 1 << (7 - (self.bit % 8));
        }
        self.bit += 1;
    }

    fn bits(&mut self, value: i32, count: usize) {
        for index in (0..count).rev() {
            self.bit(((value >> index) & 1) as u32);
        }
    }

    fn align(&mut self) {
        while self.bit % 8 != 0 {
            self.bit(0);
        }
    }

    fn take(self) -> Vec<u8> {
        self.bytes
    }
}

/// A matrix with translation only, as `PlaceObject` writes it.
fn translate_only(x: i32, y: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.bit(0); // no scale
    w.bit(0); // no rotation
    // No alignment and no padding field: the format packs the matrix continuously, so the translate
    // width follows the rotation group bit-immediately. Both were defects this fixture encoded.
    w.bits(16, 5); // translation uses 16 bits
    w.bits(x, 16);
    w.bits(y, 16);
    // The matrix ends at the last translate bit; the trailing bits of the final byte are padding,
    // not an aligned field, so no padding is written here.
    w.take()
}

/// A matrix with scale, rotation and translation.
fn scaled(scale: f64, rotate: (f64, f64), x: i32, y: i32) -> Vec<u8> {
    // The field width is declared, not implied: a scale of 2.0 is 131,072 in 16.16, which a signed
    // 16-bit field cannot hold (its ceiling is 32,767, or 0.49998 of a pixel). A fixture that wrote it
    // in 16 bits would be asking the reader to truncate and then blaming it for returning zero.
    let mut w = Writer::new();
    w.bit(1);
    w.bits(19, 5);
    w.bits((scale * 65_536.0) as i32, 19);
    w.bits((scale * 65_536.0) as i32, 19);
    w.bit(1);
    w.bits(19, 5);
    w.bits((rotate.0 * 65_536.0) as i32, 19);
    w.bits((rotate.1 * 65_536.0) as i32, 19);
    w.bits(17, 5);
    w.bits(x, 17);
    w.bits(y, 17);
    w.take()
}

#[test]
fn a_place_object_names_its_character_depth_and_position() {
    let mut body = Vec::new();
    body.extend_from_slice(&7u16.to_le_bytes());
    body.extend_from_slice(&1u16.to_le_bytes());
    body.extend(translate_only(400, 200));

    let placement = placement::place_object(&body).expect("a placement");
    assert_eq!(placement.character, 7);
    assert_eq!(placement.depth, 1);
    assert_eq!(placement.code, 4);
    let matrix = placement.matrix.expect("a matrix");
    assert_eq!(matrix.translation_px(), (20, 10), "400 twips is 20 px");
    assert_eq!(matrix.scale(), (1.0, 1.0));
}

#[test]
fn a_place_object_2_reads_scale_and_rotation() {
    let mut body = vec![0x06]; // has matrix and has character, not a move
    body.extend_from_slice(&3u16.to_le_bytes());
    body.extend_from_slice(&9u16.to_le_bytes());
    body.extend(scaled(2.0, (0.5, 0.0), 100, -40));

    let placement = placement::place_object_2(&body).expect("a placement");
    assert_eq!(placement.character, 9);
    assert_eq!(placement.depth, 3);
    assert!(!placement.moved);
    let matrix = placement.matrix.expect("a matrix");
    assert_eq!(matrix.scale(), (2.0, 2.0));
    assert_eq!(matrix.rotate_0, 32_768);
    assert_eq!(matrix.translation_px(), (5, -2));
}

#[test]
fn a_matrix_may_carry_a_scale_with_no_rotation() {
    // Under a reader that reads the rotation flag beside the scale flag, this body's rotation flag is
    // read out of the scale width and the scale comes back truncated to zero: the failure mode the
    // old order had, proved rather than described.
    let mut w = Writer::new();
    w.bit(1);
    w.bits(19, 5);
    w.bits((2.0 * 65_536.0) as i32, 19);
    w.bits((2.0 * 65_536.0) as i32, 19);
    w.bit(0);
    w.bits(17, 5);
    w.bits(300, 17);
    w.bits(-200, 17);
    w.align();

    let mut body = vec![0x06];
    body.extend_from_slice(&3u16.to_le_bytes());
    body.extend_from_slice(&9u16.to_le_bytes());
    body.extend(w.take());

    let matrix = placement::place_object_2(&body)
        .expect("a placement")
        .matrix
        .expect("a matrix");
    assert_eq!(matrix.scale(), (2.0, 2.0));
    assert_eq!((matrix.rotate_0, matrix.rotate_1), (0, 0), "no rotation is declared");
    assert_eq!(matrix.translation_px(), (15, -10));
}

#[test]
fn a_matrix_may_carry_a_rotation_with_no_scale() {
    let mut w = Writer::new();
    w.bit(0);
    w.bit(1);
    w.bits(19, 5);
    w.bits(16_384, 19);
    w.bits(0, 19);
    w.bits(17, 5);
    w.bits(0, 17);
    w.bits(0, 17);
    w.align();

    let mut body = vec![0x06];
    body.extend_from_slice(&1u16.to_le_bytes());
    body.extend_from_slice(&2u16.to_le_bytes());
    body.extend(w.take());

    let matrix = placement::place_object_2(&body)
        .expect("a placement")
        .matrix
        .expect("a matrix");
    assert_eq!(matrix.scale(), (1.0, 1.0), "an undeclared scale is identity, not zero");
    assert_eq!(matrix.rotate_0, 16_384);
    assert_eq!(matrix.rotate_1, 0);
}

#[test]
fn a_move_without_a_character_keeps_the_character_at_that_depth() {
    let first = Placement {
        character: 12,
        depth: 4,
        clip_depth: None,
        transparent: false,
        matrix: Some(Matrix::IDENTITY),
        moved: false,
        code: 4,
    };
    let mut body = vec![0x05]; // has matrix + move, no character
    body.extend_from_slice(&4u16.to_le_bytes());
    body.extend(translate_only(60, 0));
    let moved = placement::place_object_2(&body).expect("a move");

    assert_eq!(moved.character, 0, "the record names no character");
    let depths = placement::by_depth(&[first, moved]);
    assert_eq!(depths.len(), 1);
    assert_eq!(depths[0].character, 12, "the move edits the existing depth");
    assert_eq!(depths[0].matrix.expect("a matrix").translation_px(), (3, 0));
}

#[test]
fn depths_are_ordered_and_the_last_placement_wins() {
    let low = Placement {
        character: 1,
        depth: 2,
        clip_depth: None,
        transparent: false,
        matrix: None,
        moved: false,
        code: 26,
    };
    let high = Placement {
        character: 2,
        depth: 9,
        clip_depth: None,
        transparent: false,
        matrix: None,
        moved: false,
        code: 26,
    };
    let replaced = Placement {
        character: 5,
        depth: 2,
        clip_depth: None,
        transparent: false,
        matrix: Some(Matrix::IDENTITY),
        moved: false,
        code: 26,
    };
    let depths = placement::by_depth(&[high, low, replaced]);
    assert_eq!(depths.iter().map(|p| p.depth).collect::<Vec<_>>(), vec![2, 9]);
    assert_eq!(depths[0].character, 5, "the later record replaced the character");
}

#[test]
fn a_clip_depth_is_read_rather_than_skipped() {
    // flags: has clip depth (0x40) and has character (0x02), no matrix. The clip depth is the last
    // field of the record, and it states the depths this placement masks.
    let mut body = vec![0x42];
    body.extend_from_slice(&7u16.to_le_bytes());
    body.extend_from_slice(&9u16.to_le_bytes());
    body.extend_from_slice(&12u16.to_le_bytes());
    let placement = placement::place_object_2(&body).expect("a masked placement");
    assert_eq!(placement.clip_depth, Some(12));
    assert_eq!(placement.character, 9);
    assert_eq!(placement.depth, 7);
}

#[test]
fn a_truncated_matrix_is_refused_by_name() {
    let mut body = Vec::new();
    body.extend_from_slice(&7u16.to_le_bytes());
    body.extend_from_slice(&1u16.to_le_bytes());
    body.extend(translate_only(400, 200)[..1].to_vec());

    let refusal = placement::place_object(&body).expect_err("a cut matrix");
    assert_eq!(refusal.code(), "placement-truncated");
    assert!(matches!(refusal, Refusal::Truncated { .. }));
}

#[test]
fn a_named_placement_is_read_to_its_end() {
    // Name, ratio and clip depth are ordered, self-delimiting fields: reading them exactly keeps the
    // fields after them at their own offsets, which is the rule a skip would break.
    let mut body = vec![0x22 | 0x40]; // has character + has name + has clip depth
    body.extend_from_slice(&1u16.to_le_bytes());
    body.extend_from_slice(&8u16.to_le_bytes());
    body.extend_from_slice(b"n\0");
    body.extend_from_slice(&4u16.to_le_bytes());
    let placement = placement::place_object_2(&body).expect("a named placement");
    assert_eq!((placement.character, placement.depth), (8, 1));
}

#[test]
fn clip_actions_are_refused_rather_than_skipped() {
    // Clip actions are a nested variable-length record; refusing them by name is counted by the
    // caller, and a partial read would put every later field at a wrong offset.
    let mut body = vec![0x02 | 0x80]; // has character + has clip actions
    body.extend_from_slice(&1u16.to_le_bytes());
    body.extend_from_slice(&8u16.to_le_bytes());
    body.extend_from_slice(&[0, 0, 0, 0]);
    let refusal = placement::place_object_2(&body).expect_err("clip actions");
    assert_eq!(refusal.code(), "placement-unsupported");
    assert!(refusal.message().contains("misread"));
}

#[test]
fn the_stream_survey_reads_only_the_two_placement_codes() {
    let mut place = Vec::new();
    place.extend_from_slice(&4u16.to_le_bytes());
    place.extend_from_slice(&2u16.to_le_bytes());
    place.extend(translate_only(0, 0));

    let mut stream = place.clone();
    stream.extend_from_slice(&0u16.to_le_bytes());
    let tags = vec![
        Tag {
            code: 4,
            length: place.len(),
            at: 0,
        },
        Tag {
            code: 1,
            length: 0,
            at: place.len(),
        },
    ];
    let placements = placement::survey(&tags, &stream).expect("surveyed");
    assert_eq!(placements.len(), 1, "a show-frame tag places nothing");
    assert_eq!(placements[0].character, 4);
}
