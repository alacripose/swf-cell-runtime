//! The emitted frame document, proved against the grammar's own reader. Synthetic bytes only.

use planet_package::cellframe::{check_extent, CellFrame, MAX_CELLS};
use planet_package::Surface;
use swf_import::content::survey;
use swf_import::header::{self, Header};
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

/// A stage of 11,000 by 8,000 twips: 550 by 400 pixels.
fn header() -> Header {
    let mut body = rect_bytes(0, 11_000, 0, 8_000, 16);
    body.extend_from_slice(&10_240u16.to_le_bytes());
    body.extend_from_slice(&12u16.to_le_bytes());
    body.extend_from_slice(&((9u16 << 6) | 3).to_le_bytes());
    body.extend_from_slice(&[1, 2, 3]);
    body.extend_from_slice(&0u16.to_le_bytes());
    let mut file = Vec::new();
    file.extend_from_slice(b"FWS");
    file.push(8);
    file.extend_from_slice(&((8 + body.len()) as u32).to_le_bytes());
    file.extend_from_slice(&body);
    header::read(&file).expect("a synthetic header")
}

/// The `(cols, rows)` a window row declares, read back from the emitted declaration.
fn window_extent(document: &str) -> (u32, u32) {
    let row = document
        .lines()
        .find(|line| line.starts_with("window\t"))
        .expect("a window row");
    let fields: Vec<&str> = row.split('\t').collect();
    (
        fields[7].parse().expect("cols"),
        fields[8].parse().expect("rows"),
    )
}

#[test]
fn the_frame_parses_and_binds_the_stage_window() {
    let header = header();
    let frame = CellFrame::parse(&emit::frame_document(&header)).expect("the grammar accepts it");
    assert_eq!(frame.window, "stage");
    assert_eq!(frame.id, emit::FRAME_ID);
    assert!(frame.cols * frame.rows <= MAX_CELLS, "the grid fits the ceiling");
    assert_eq!(frame.fills.len(), 1, "the stage is one paper");
    assert_eq!(frame.runs.len(), 1, "the stage names itself once");
}

#[test]
fn the_frame_extent_is_the_extent_the_window_declares() {
    let header = header();
    let frame = CellFrame::parse(&emit::frame_document(&header)).expect("the grammar accepts it");
    let tags = tags::walk(&header).expect("tags");
    let inventory = survey(&header, &tags).expect("survey");
    let counts = tags::counts(&tags);
    let package = emit::package_frame("synthetic", &header, &counts, tags.len(), &inventory)
        .expect("a declaration");
    let document = String::from_utf8(package.payload.clone()).expect("utf-8");
    let (cols, rows) = window_extent(&document);
    assert_eq!(
        (frame.cols, frame.rows),
        (cols, rows),
        "a frame's extent must equal its window's content exactly, or the binding is refused"
    );
    check_extent(&frame, "stage", [cols, rows], 1).expect("the binding is acceptable");
}

#[test]
fn the_declaration_binds_the_frame_it_ships() {
    let header = header();
    let tags = tags::walk(&header).expect("tags");
    let inventory = survey(&header, &tags).expect("survey");
    let counts = tags::counts(&tags);
    let package = emit::package_frame("synthetic", &header, &counts, tags.len(), &inventory)
        .expect("a declaration");
    let surface = Surface::parse(&package.payload).expect("the declaration parses");
    assert_eq!(surface.windows.len(), 1);
    let document = String::from_utf8(package.payload.clone()).expect("utf-8");
    assert!(
        document.contains(&format!("frame\tstage\t{}\n", emit::FRAME_ID)),
        "the window names the frame this package ships"
    );
}

#[test]
fn the_stage_is_measured_in_the_frames_own_cells() {
    let header = header();
    let frame = CellFrame::parse(&emit::frame_document(&header)).expect("the grammar accepts it");
    assert_eq!(
        (frame.cols, frame.rows),
        ((550 + 7) / 8, (400 + 15) / 16),
        "550 by 400 pixels is 69 by 25 whole cells, rounded up"
    );
}

#[test]
fn a_frame_the_grammar_refuses_is_never_returned() {
    // The writer validates before it hands bytes back, so a document that would not parse cannot
    // reach a file: this asserts the contract, and the parse above proves it holds for this input.
    let header = header();
    let bytes = emit::frame_bytes(&header).expect("a valid frame");
    let text = String::from_utf8(bytes).expect("utf-8");
    assert!(CellFrame::parse(&text).is_ok());
}
