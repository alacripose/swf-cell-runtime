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

fn bound_document() -> String {
    format!("{}frame\tmain\tMAP-0001\n", document("content-main"))
}

fn frame_document(id: &str, window: &str, cols: u32, rows: u32, glyphs: &str) -> String {
    format!(
        "shploingus.cellframe/1\n\
         frame\t{id}\n\
         \twindow\t{window}\n\
         \tcols\t{cols}\n\
         \trows\t{rows}\n\
         \torder\t10\n\
         \trequires\tnone\n\
         fill\t0\t0\t{cols}\tsurface\n\
         run\t0\t0\ttext\t{glyphs}\n"
    )
}

#[test]
fn a_client_adopts_a_mounted_extent_checked_surface() {
    let bytes = Frame::new(Kind::Decl, bound_document().as_bytes())
        .expect("a declaration frame")
        .encode();
    let alpha = frame_document("MAP-0001", "main", 42, 18, "ALPHA");
    let mut mount = CellMount::new();
    let adopted = mount
        .adopt_frames(&bytes, &[&alpha], 42)
        .expect("the reference resolves and the extents agree");

    assert_eq!(adopted.receipt.windows, 1);
    assert_eq!(adopted.mount.windows.len(), 1, "the mount carries the bound window");
    assert_eq!(adopted.mount.windows[0].line, 6, "the binding row's line travels with the mount");
    let frame = adopted.frame_of("main").expect("the client asks the mount");
    assert_eq!(frame.id, "MAP-0001");
    assert_eq!(frame.runs[0].text, "ALPHA");
    assert_eq!(adopted.frame_of("absent"), None, "an unbound window has no frame");
    assert_eq!(adopted.digest, planet_client::package_owner::digest(&package_owner::decode(&bytes).unwrap()));
}

#[test]
fn a_client_refuses_a_package_whose_frame_disagrees_with_its_window() {
    let bytes = Frame::new(Kind::Decl, bound_document().as_bytes())
        .expect("a declaration frame")
        .encode();
    let wide = frame_document("MAP-0001", "main", 56, 18, "ALPHA");
    let mut mount = CellMount::new();
    let refusals = mount
        .adopt_frames(&bytes, &[&wide], 42)
        .expect_err("56 by 18 disagrees with the window's 42 by 18");
    assert_eq!(refusals.code(), "cellframe-extent");
    let message = refusals.message();
    assert!(message.contains("line 6"), "the binding line is missing: {message}");
    assert!(message.contains("MAP-0001"), "the frame id is missing: {message}");
    assert!(message.contains("main"), "the window id is missing: {message}");
    assert!(mount.adopted().is_empty(), "a refused package stores nothing");
}

#[test]
fn a_client_refuses_a_frame_the_declaration_never_binds() {
    let bytes = Frame::new(Kind::Decl, document("content-main").as_bytes())
        .expect("a declaration frame")
        .encode();
    let alpha = frame_document("MAP-0001", "main", 42, 18, "ALPHA");
    let mut mount = CellMount::new();
    let refusals = mount
        .adopt_frames(&bytes, &[&alpha], 42)
        .expect_err("no frame row binds MAP-0001");
    assert_eq!(refusals.code(), "cellframe-unreferenced");
    assert!(mount.adopted().is_empty());
}

#[test]
fn adopting_the_same_mounted_package_twice_answers_the_first() {
    let bytes = Frame::new(Kind::Decl, bound_document().as_bytes())
        .expect("a declaration frame")
        .encode();
    let alpha = frame_document("MAP-0001", "main", 42, 18, "ALPHA");
    let mut mount = CellMount::new();
    {
        let first = mount.adopt_frames(&bytes, &[&alpha], 7).expect("first");
        assert_eq!(first.receipt.sequence, 1);
    }
    let second = mount.adopt_frames(&bytes, &[&alpha], 99).expect("retry");
    assert_eq!(second.receipt.sequence, 1);
    assert_eq!(second.receipt.at, 7);
    assert!(second.frame_of("main").is_some());
    let adopted = mount.adopted().len();
    assert_eq!(adopted, 1);
}
