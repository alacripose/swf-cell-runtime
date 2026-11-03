//! Synthetic fixtures only: no byte of the supplied movie appears here.

use planet_package::mount;
use swf_import::header::{self, Compression};
use swf_import::{emit, tags};

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

fn body() -> Vec<u8> {
    let mut body = rect_bytes(0, 11_000, 0, 8_000, 16);
    body.extend_from_slice(&10_240u16.to_le_bytes()); // 40 fps
    body.extend_from_slice(&12u16.to_le_bytes());
    body.extend_from_slice(&((9u16 << 6) | 3).to_le_bytes());
    body.extend_from_slice(&[1, 2, 3]);
    body.extend_from_slice(&0u16.to_le_bytes());
    body
}

fn fws() -> Vec<u8> {
    let body = body();
    let mut file = Vec::new();
    file.extend_from_slice(b"FWS");
    file.push(8);
    file.extend_from_slice(&((8 + body.len()) as u32).to_le_bytes());
    file.extend_from_slice(&body);
    file
}

#[test]
fn the_header_and_tag_stream_are_measured() {
    let header = header::read(&fws()).expect("a readable header");
    assert_eq!(header.compression, Compression::None);
    assert_eq!(header.version, 8);
    assert_eq!(header.stage.width_px(), 550);
    assert_eq!(header.stage.height_px(), 400);
    assert_eq!(header.frame_rate, 40.0);
    assert_eq!(header.frame_count, 12);

    let tags = tags::walk(&header).expect("a walkable stream");
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].code, 9);
    assert_eq!(tags[0].length, 3);
    assert_eq!(tags::counts(&tags).get(&9), Some(&1));
}

#[test]
fn a_zlib_body_is_read() {
    use flate2::write::ZlibEncoder;
    use std::io::Write;

    let body = body();
    let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(&body).unwrap();
    let compressed = encoder.finish().unwrap();

    let mut file = Vec::new();
    file.extend_from_slice(b"CWS");
    file.push(8);
    file.extend_from_slice(&((8 + body.len()) as u32).to_le_bytes());
    file.extend_from_slice(&compressed);

    let header = header::read(&file).expect("a readable compressed header");
    assert_eq!(header.compression, Compression::Zlib);
    assert_eq!(header.stage.width_px(), 550);
    assert_eq!(tags::walk(&header).expect("walkable").len(), 1);
}

#[test]
fn the_emitted_package_mounts_through_the_canonical_path() {
    let header = header::read(&fws()).unwrap();
    let tags = tags::walk(&header).unwrap();
    let counts = tags::counts(&tags);
    let inventory = swf_import::content::survey(&header, &tags).expect("surveyed");
    let frame = emit::package_frame("swf1", &header, &counts, tags.len(), &inventory)
        .expect("an emitted frame");

    let receipt = mount(&frame, "swf1", 1, 0, 0).expect("the package mounts");
    assert_eq!(receipt.surface, "swf1");
    assert_eq!(receipt.windows, 1);
    assert_eq!(receipt.digest, frame.frame_digest_hex());

    let doc = emit::surface_document("swf1", &header, &counts, tags.len(), &inventory);
    // The window's content space is the frame's grid, because a frame's extent must equal it exactly
    // and a frame counts in its own cells: 550 by 400 pixels is 69 by 25 cells, rounded up.
    assert!(doc.contains("\t69\t25\t"), "the stage is the measured cells, not the pixel count");
    assert!(doc.contains("12 frames at 40 fps"), "the frame count is measured");
    assert!(
        doc.contains(&format!("frame\tstage\t{}\n", emit::FRAME_ID)),
        "the stage window binds the frame the package ships"
    );
}

#[test]
fn an_unknown_signature_is_refused_by_name() {
    let mut file = fws();
    file[0] = b'X';
    let refusal = header::read(&file).expect_err("not a SWF");
    assert_eq!(refusal.code(), "swf-signature");
    assert!(refusal.message().contains("XWS"));
}

#[test]
fn lzma_is_refused_by_name() {
    let mut file = fws();
    file[0] = b'Z';
    let refusal = header::read(&file).expect_err("no LZMA decoder");
    assert_eq!(refusal.code(), "swf-compression");
}

#[test]
fn a_truncated_tag_is_refused() {
    let mut file = fws();
    let last = file.len() - 1;
    file[last] = 0xff;
    file[last - 1] = 0xff;
    let header = header::read(&file).expect("header still reads");
    let refusal = tags::walk(&header).expect_err("the tag overruns the body");
    assert_eq!(refusal.code(), "swf-truncated");
}

#[test]
fn a_short_file_is_refused() {
    let refusal = header::read(&[b'F', b'W', b'S']).expect_err("too short");
    assert_eq!(refusal.code(), "swf-truncated");
}
