//! Adoption is proven by behaviour: a receipt and parsed surface come back, the same bytes adopt
//! once, distinct packages take sequence positions, and every refusal leaves the mount empty.

use planet_client::{package_owner, CellMount};
use planet_package::{Frame, Kind, RoleClass};

fn document(content: &str) -> String {
    let mut text = String::from("dl-enclave.surface/1\nsurface\talpha-shell\nrequires\tobserve\n");
    text.push_str("window\tmain\tMain\tframe\tright-dock\t0\t0\t42\t18\t35\t9\ttrue\tfalse\tminimize\tdrag\tsurface\t");
    text.push_str(content);
    text.push_str("\nnode\tmain\troot\t-\tpane\tsurface\t-\n");
    text
}

fn declaration(content: &str) -> Frame {
    Frame::new(Kind::Decl, document(content).as_bytes()).expect("a declaration frame")
}

fn encoded(content: &str) -> Vec<u8> {
    declaration(content).encode()
}

#[test]
fn an_adopted_package_exposes_its_receipt_and_parsed_surface() {
    let mut mount = CellMount::new();
    let adopted = mount.adopt(&encoded("content-main"), 1_700_000_000).expect("adopted");
    let frame = declaration("content-main");

    assert_eq!(adopted.receipt.surface, "alpha-shell");
    assert_eq!(adopted.receipt.windows, 1);
    assert_eq!(adopted.receipt.sequence, 1);
    assert_eq!(adopted.receipt.state, "loaded");
    assert_eq!(adopted.receipt.at, 1_700_000_000);
    assert_eq!(adopted.digest, frame.frame_digest_hex());
    assert_eq!(adopted.receipt.digest, frame.frame_digest_hex());

    let window = &adopted.surface.windows[0];
    assert_eq!(window.lane, "frame");
    assert_eq!(window.content, "content-main");
    assert_eq!(window.nodes[0].kind, "pane");
    assert_eq!(window.nodes[0].role_class, RoleClass::Theme);
}

#[test]
fn the_same_bytes_adopt_once_and_answer_the_first_receipt() {
    let mut mount = CellMount::new();
    let bytes = encoded("content-main");
    {
        let first = mount.adopt(&bytes, 10).expect("first adoption");
        assert_eq!(first.receipt.sequence, 1);
        assert_eq!(first.receipt.at, 10);
    }
    let second = mount.adopt(&bytes, 99).expect("retry");
    assert_eq!(second.receipt.sequence, 1, "a retry keeps the first position");
    assert_eq!(second.receipt.at, 10, "a retry answers the first adoption");
    assert_eq!(mount.adopted().len(), 1, "a retry adds nothing");
}

#[test]
fn two_distinct_packages_take_two_sequence_positions() {
    let mut mount = CellMount::new();
    {
        let first = mount.adopt(&encoded("content-main"), 1).expect("first");
        assert_eq!(first.receipt.sequence, 1);
    }
    let second = mount.adopt(&encoded("content-side"), 2).expect("second");
    assert_eq!(second.receipt.sequence, 2);
    assert_eq!(second.surface.windows[0].content, "content-side");
    assert_eq!(mount.adopted().len(), 2);
}

#[test]
fn a_code_frame_is_refused_and_nothing_is_adopted() {
    let frame = Frame::new(Kind::Code, document("content-main").as_bytes()).expect("a code frame");
    let mut mount = CellMount::new();
    let refusal = mount.adopt(&frame.encode(), 1).expect_err("code is reserved");
    assert_eq!(refusal.code(), "frame-kind-reserved");
    assert!(mount.adopted().is_empty());
}

#[test]
fn an_undeclared_lane_refuses_the_whole_package() {
    let text = document("content-main").replace("\tframe\tright-dock", "\tdock\tright-dock");
    let frame = Frame::new(Kind::Decl, text.as_bytes()).expect("a declaration frame");
    let mut mount = CellMount::new();
    let refusal = mount.adopt(&frame.encode(), 1).expect_err("the lane is undeclared");
    assert_eq!(refusal.code(), "unknown-lane");
    assert!(mount.adopted().is_empty(), "a refused package stores nothing");
}

#[test]
fn a_tampered_frame_is_refused_before_adoption() {
    let mut bytes = encoded("content-main");
    let last = bytes.len() - 1;
    bytes[last] ^= 0xff;
    let mut mount = CellMount::new();
    let refusal = mount.adopt(&bytes, 1).expect_err("the digest no longer matches");
    assert_eq!(refusal.code(), "frame-digest");
    assert!(mount.adopted().is_empty());
}

#[test]
fn bytes_that_are_not_a_frame_are_refused() {
    let mut bytes = encoded("content-main");
    bytes[0] ^= 0xff;
    let mut mount = CellMount::new();
    assert_eq!(mount.adopt(&bytes, 1).unwrap_err().code(), "frame-magic");
    let short = &encoded("content-main")[..8];
    assert_eq!(mount.adopt(short, 1).unwrap_err().code(), "frame-truncated");
    assert!(mount.adopted().is_empty());
}

#[test]
fn the_package_owner_round_trips_the_frame_it_does_not_own() {
    let frame = declaration("content-main");
    let bytes = package_owner::encode(&frame);
    let decoded = package_owner::decode(&bytes).expect("a frame the owner decodes");
    assert_eq!(package_owner::digest(&decoded), frame.frame_digest_hex());
    assert_eq!(decoded.payload, frame.payload);
    assert_eq!(decoded.kind, Kind::Decl);
}
