use planet_package::{mount, Frame, Kind, Refusal, DOMAIN, HEADER, PAYLOAD_LIMIT, SCHEMA};
use planet_shell_ledger::sha256;

fn surface_text(windows: usize) -> String {
    let mut text = String::from("dl-enclave.surface/1\nsurface\talpha-shell\nrequires\tobserve\n");
    for index in 0..windows {
        text.push_str(&format!(
            "window\tw{index}\tWindow {index}\tframe\tright-dock\t0\t0\t42\t18\t35\t9\ttrue\tfalse\tminimize\tdrag\tsurface\tcontent{index}\n"
        ));
    }
    text
}

fn declaration(windows: usize) -> Frame {
    Frame::new(Kind::Decl, surface_text(windows).as_bytes()).expect("a declaration frame")
}

fn patched(bytes: &[u8], at: usize, value: &[u8]) -> Vec<u8> {
    let mut copy = bytes.to_vec();
    copy[at..at + value.len()].copy_from_slice(value);
    copy
}

#[test]
fn a_frame_round_trips() {
    let frame = declaration(1);
    let bytes = frame.encode();
    assert_eq!(&bytes[0..8], b"V4PKG001");
    assert_eq!(Frame::decode(&bytes).unwrap(), frame);
}

#[test]
fn the_payload_digest_frames_domain_schema_and_length() {
    let frame = declaration(1);
    let mut message = Vec::new();
    message.extend_from_slice(DOMAIN);
    message.extend_from_slice(&SCHEMA.to_le_bytes());
    message.extend_from_slice(&(frame.payload.len() as u64).to_le_bytes());
    message.extend_from_slice(&frame.payload);
    assert_eq!(
        frame.payload_digest_hex(),
        sha256::hex(&sha256::sha256(&message))
    );
    assert_eq!(frame.payload_digest_hex().len(), 64);
}

#[test]
fn an_older_schema_is_refused_by_name() {
    let bytes = patched(&declaration(1).encode(), 8, &2u16.to_le_bytes());
    assert_eq!(
        Frame::decode(&bytes),
        Err(Refusal::FrameSchema {
            found: 2,
            supported: 1
        })
    );
    // The schema is read before the payload, so the refusal does not depend on the digest.
    assert_eq!(
        Frame::read_schema(&bytes),
        Err(Refusal::FrameSchema {
            found: 2,
            supported: 1
        })
    );
}

#[test]
fn another_magic_is_refused() {
    let mut bytes = declaration(1).encode();
    // A second magic is never spelled in this tree: the gate refuses one by name, so the fixture is
    // made by altering a byte rather than by naming a format this repository does not have.
    bytes[7] = b'9';
    match Frame::decode(&bytes) {
        Err(Refusal::FrameMagic { found }) => {
            assert_eq!(found.len(), 8);
            assert!(found.ends_with('9'), "found {found:?}");
        }
        other => panic!("expected a magic refusal, got {other:?}"),
    }
}

#[test]
fn the_kind_set_is_closed() {
    let bytes = patched(&declaration(1).encode(), 10, &3u16.to_le_bytes());
    assert_eq!(Frame::decode(&bytes), Err(Refusal::FrameKind { found: 3 }));
}

#[test]
fn a_truncated_frame_is_refused() {
    let bytes = declaration(1).encode();
    let short = &bytes[..HEADER - 1];
    assert_eq!(
        Frame::decode(short),
        Err(Refusal::FrameTruncated {
            needed: HEADER,
            found: HEADER - 1
        })
    );
}

#[test]
fn a_declared_length_must_match_the_payload() {
    let frame = declaration(1);
    let mut bytes = frame.encode();
    let declared = (frame.payload.len() as u64 + 1).to_le_bytes();
    bytes[12..20].copy_from_slice(&declared);
    assert_eq!(
        Frame::decode(&bytes),
        Err(Refusal::FrameLength {
            declared: frame.payload.len() as u64 + 1,
            found: frame.payload.len()
        })
    );
}

#[test]
fn an_altered_payload_breaks_the_digest() {
    let frame = declaration(1);
    let mut bytes = frame.encode();
    let last = bytes.len() - 1;
    bytes[last] = b'X';
    match Frame::decode(&bytes) {
        Err(Refusal::FrameDigest { declared, found }) => assert_ne!(declared, found),
        other => panic!("expected a digest refusal, got {other:?}"),
    }
}

#[test]
fn a_payload_over_the_cap_is_refused() {
    assert_eq!(
        Frame::new(Kind::Decl, &vec![b'x'; PAYLOAD_LIMIT + 1]),
        Err(Refusal::PackagePayloadLimit {
            limit: PAYLOAD_LIMIT,
            found: PAYLOAD_LIMIT + 1
        })
    );
}

#[test]
fn a_code_frame_packs_and_is_refused_at_mount() {
    let frame = Frame::new(Kind::Code, surface_text(1).as_bytes()).unwrap();
    assert_eq!(Frame::decode(&frame.encode()).unwrap().kind, Kind::Code);
    assert_eq!(
        mount(&frame, "alpha-shell", 1, 0, 1_700_000_000),
        Err(Refusal::FrameKindReserved { kind: "code" })
    );
}

#[test]
fn a_mount_sequence_must_follow_the_previous_one() {
    assert_eq!(
        mount(&declaration(1), "alpha-shell", 3, 1, 1_700_000_000),
        Err(Refusal::MountSequence {
            previous: 1,
            found: 3
        })
    );
}

#[test]
fn a_mount_receipt_names_the_frame_and_its_windows() {
    let frame = declaration(2);
    let receipt = mount(&frame, "alpha-shell", 1, 0, 1_700_000_000).unwrap();
    assert_eq!(receipt.surface, "alpha-shell");
    assert_eq!(receipt.sequence, 1);
    assert_eq!(receipt.state, "loaded");
    assert_eq!(receipt.windows, 2);
    assert_eq!(receipt.at, 1_700_000_000);
    assert_eq!(receipt.digest, frame.frame_digest_hex());
}

#[test]
fn a_payload_that_is_not_a_surface_is_refused() {
    let frame = Frame::new(Kind::Decl, b"shploingus.recipe/3\nwindow\tw\n").unwrap();
    match mount(&frame, "alpha-shell", 1, 0, 0) {
        Err(Refusal::SurfaceSchema { found }) => assert!(found.starts_with("shploingus.recipe/3")),
        other => panic!("expected a surface-schema refusal, got {other:?}"),
    }
}

#[test]
fn a_surface_without_its_final_newline_is_refused() {
    let frame = Frame::new(Kind::Decl, surface_text(1).trim_end().as_bytes()).unwrap();
    assert_eq!(
        mount(&frame, "alpha-shell", 1, 0, 0),
        Err(Refusal::SurfaceSchema {
            found: "no final newline".to_string()
        })
    );
}

#[test]
fn a_receipt_must_name_the_surface_the_document_declares() {
    match mount(&declaration(1), "beta-shell", 1, 0, 0) {
        Err(Refusal::SurfaceSchema { found }) => assert!(found.contains("alpha-shell")),
        other => panic!("expected a surface-schema refusal, got {other:?}"),
    }
}

#[test]
fn a_surface_with_no_windows_declares_nothing() {
    let frame = declaration(0);
    assert_eq!(
        mount(&frame, "alpha-shell", 1, 0, 0),
        Err(Refusal::SurfaceWindows { found: 0 })
    );
}
