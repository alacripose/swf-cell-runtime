use planet_package::cellframe::HEADER as CELLFRAME_HEADER;
use planet_package::{load, resolve, Frame, Kind, Refusal, Surface, SURFACE_SCHEMA};

const WINDOW_COLS: usize = 17;

fn window_row(id: &str) -> String {
    format!(
        "window\t{id}\tTitle {id}\tframe\tright-dock\t0\t0\t42\t18\t35\t9\ttrue\tfalse\t\
         minimize\tdrag\tsurface\tcontent-{id}\n"
    )
}

/// A declaration whose windows bind the given `(window, frame)` pairs, in order.
///
/// Line 1 is the header, so a binding written first sits on line 5: header, `surface`, `requires`,
/// one `window` row, one `frame` row.
fn surface_document(bindings: &[(&str, &str)]) -> String {
    let mut document = format!("{SURFACE_SCHEMA}\nsurface\talpha-shell\nrequires\tobserve\n");
    for (window, frame) in bindings {
        document.push_str(&window_row(window));
        document.push_str(&format!("frame\t{window}\t{frame}\n"));
    }
    assert_eq!(
        document.lines().count(),
        3 + bindings.len() * 2,
        "the fixture keeps one window and one binding per pair"
    );
    document
}

fn frame_document(id: &str, window: &str, glyphs: &str) -> String {
    frame_document_sized(id, window, glyphs, 42, 18)
}

fn frame_document_sized(id: &str, window: &str, glyphs: &str, cols: u32, rows: u32) -> String {
    format!(
        "{CELLFRAME_HEADER}\n\
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

fn parse(document: &str) -> Surface {
    Surface::parse(document.as_bytes()).expect("the fixture surface is well formed")
}

fn frame_row_count(document: &str) -> usize {
    document.lines().filter(|line| line.starts_with("frame\t")).count()
}

#[test]
fn a_declaration_with_no_binding_still_loads_unchanged() {
    let document = surface_document(&[]);
    let surface = parse(&document);
    assert_eq!(surface.windows.len(), 0);
    assert_eq!(resolve(&surface, &[]), Ok(Vec::new()));
}

#[test]
fn each_declaration_resolves_to_the_frame_its_id_names() {
    let surface = parse(&surface_document(&[("main", "MAP-0001"), ("plan", "MAP-0002")]));
    let alpha = frame_document("MAP-0001", "main", "ALPHA");
    let beta = frame_document("MAP-0002", "plan", "BETA");

    let bound = resolve(&surface, &[&alpha, &beta]).expect("both references resolve");
    assert_eq!(bound.len(), 2);
    assert_eq!(bound[0].window, "main");
    assert_eq!(bound[0].frame.id, "MAP-0001");
    assert_eq!(bound[0].frame.runs[0].text, "ALPHA");
    assert_eq!(bound[0].line, 5);
    assert_eq!(bound[1].window, "plan");
    assert_eq!(bound[1].frame.id, "MAP-0002");
    assert_eq!(bound[1].frame.runs[0].text, "BETA");
    assert_eq!(bound[1].line, 7);

    // The order the package lists its frames in does not change which window gets which frame.
    let reversed = resolve(&surface, &[&beta, &alpha]).expect("order is not the binding");
    assert_eq!(reversed, bound);
}

#[test]
fn an_undeclared_frame_id_is_refused_with_the_line_of_its_reference() {
    let surface = parse(&surface_document(&[("main", "MAP-0009")]));
    let alpha = frame_document("MAP-0001", "main", "ALPHA");
    let refusals = resolve(&surface, &[&alpha]).expect_err("MAP-0009 is not declared");
    assert_eq!(refusals.code(), "cellframe-reference");
    let message = refusals.message();
    assert!(message.contains("cellframe line 5"), "the line is missing: {message}");
    assert!(message.contains("MAP-0009"), "the id is missing: {message}");
    assert!(message.contains("main"), "the window is missing: {message}");
}

#[test]
fn a_frame_no_declaration_binds_is_refused() {
    let surface = parse(&surface_document(&[("main", "MAP-0001")]));
    let alpha = frame_document("MAP-0001", "main", "ALPHA");
    let orphan = frame_document("MAP-0002", "plan", "ORPHAN");
    let refusals = resolve(&surface, &[&alpha, &orphan]).expect_err("MAP-0002 is unreachable");
    assert_eq!(refusals.code(), "cellframe-unreferenced");
    assert!(refusals.message().contains("MAP-0002"), "{}", refusals.message());
}

#[test]
fn one_frame_cannot_be_bound_twice() {
    let document = surface_document(&[("main", "MAP-0001"), ("plan", "MAP-0001")]);
    match Surface::parse(document.as_bytes()) {
        Err(refusal) => {
            assert_eq!(refusal.code(), "cellframe-duplicate");
            assert!(refusal.message().contains("line 7"), "the line is missing: {refusal:?}");
            assert!(refusal.message().contains("MAP-0001"), "{refusal:?}");
        }
        Ok(surface) => panic!("two windows bound the same frame: {surface:?}"),
    }
}

#[test]
fn one_window_cannot_name_two_frames() {
    let document = format!(
        "{}{}frame\tmain\tMAP-0002\n",
        surface_document(&[("main", "MAP-0001")]),
        String::new()
    );
    match Surface::parse(document.as_bytes()) {
        Err(refusal) => {
            assert_eq!(refusal.code(), "cellframe-duplicate");
            assert!(refusal.message().contains("line 6"), "the line is missing: {refusal:?}");
        }
        Ok(surface) => panic!("one window named two frames: {surface:?}"),
    }
}

#[test]
fn a_frame_row_before_its_window_is_refused() {
    let document = format!(
        "{SURFACE_SCHEMA}\nsurface\talpha-shell\nrequires\tobserve\nframe\tmain\tMAP-0001\n"
    );
    match Surface::parse(document.as_bytes()) {
        Err(refusal) => assert_eq!(refusal.code(), "surface-order"),
        Ok(surface) => panic!("a frame row with no window: {surface:?}"),
    }
}

#[test]
fn a_frame_row_under_another_window_is_refused() {
    let document = format!(
        "{}{}frame\tplan\tMAP-0001\n",
        format!("{SURFACE_SCHEMA}\nsurface\talpha-shell\nrequires\tobserve\n"),
        window_row("main")
    );
    match Surface::parse(document.as_bytes()) {
        Err(refusal) => assert_eq!(refusal.code(), "surface-order"),
        Ok(surface) => panic!("a frame row grouped under the wrong window: {surface:?}"),
    }
}

#[test]
fn loading_a_package_resolves_its_frames_rather_than_deferring_them() {
    let surface = surface_document(&[("main", "MAP-0001"), ("plan", "MAP-0002")]);
    let alpha = frame_document("MAP-0001", "main", "ALPHA");
    let beta = frame_document("MAP-0002", "plan", "BETA");
    let package = Frame::new(Kind::Decl, surface.as_bytes()).expect("a declaration frame");

    let loaded = load(&package, "alpha-shell", &[&alpha, &beta], 1, 0, 100)
        .expect("a package whose references resolve loads");
    assert_eq!(loaded.receipt.surface, "alpha-shell");
    assert_eq!(loaded.receipt.windows, 2);
    assert_eq!(loaded.frames.len(), 2);
    assert_eq!(loaded.frames[0].frame.runs[0].text, "ALPHA");
    assert_eq!(loaded.frames[1].frame.runs[0].text, "BETA");
}

#[test]
fn a_package_whose_reference_does_not_resolve_does_not_load() {
    let surface = surface_document(&[("main", "MAP-0009")]);
    let alpha = frame_document("MAP-0001", "main", "ALPHA");
    let package = Frame::new(Kind::Decl, surface.as_bytes()).expect("a declaration frame");

    let refusals = load(&package, "alpha-shell", &[&alpha], 1, 0, 100)
        .expect_err("the reference names a frame the package does not declare");
    assert_eq!(refusals.code(), "cellframe-reference");
    assert!(refusals.message().contains("line 5"), "{}", refusals.message());
}

#[test]
fn a_malformed_frame_is_refused_by_the_frame_grammar_first() {
    let surface = parse(&surface_document(&[("main", "MAP-0001")]));
    let broken = frame_document("MAP-0001", "main", "ALPHA").replace(
        "\tcols\t42\n",
        "",
    );
    let refusals = resolve(&surface, &[&broken]).expect_err("the frame is missing cols");
    assert_eq!(refusals.code(), "cellframe-missing");
    assert!(refusals.message().contains("cols"), "{}", refusals.message());
}

#[test]
fn the_binding_row_is_the_only_place_a_frame_is_named() {
    let document = surface_document(&[("main", "MAP-0001")]);
    assert_eq!(frame_row_count(&document), 1);
    assert!(!document.contains("cellframe"), "the declaration never carries the frame's grammar");
    let surface = parse(&document);
    assert_eq!(surface.windows[0].frame.as_ref().map(|bound| bound.id.as_str()), Some("MAP-0001"));
    assert_eq!(surface.windows.len() * WINDOW_COLS, WINDOW_COLS);
    // The refusal family this unit extends is the cellframe one; the surface codes are untouched.
    assert_eq!(Refusal::CellframeReference { line: 1, window: "w".into(), frame: "f".into() }.code(), "cellframe-reference");
}

#[test]
fn an_agreeing_frame_and_window_pair_loads() {
    let surface = surface_document(&[("main", "MAP-0001")]);
    let alpha = frame_document_sized("MAP-0001", "main", "ALPHA", 42, 18);
    let package = Frame::new(Kind::Decl, surface.as_bytes()).expect("a declaration frame");
    let loaded = load(&package, "alpha-shell", &[&alpha], 1, 0, 100)
        .expect("42 by 18 agrees with the window's 42 by 18");
    assert_eq!(loaded.frames.len(), 1);
    assert_eq!(loaded.frames[0].frame.cols, 42);
}

#[test]
fn a_frame_larger_than_its_window_is_refused_at_load() {
    let surface = surface_document(&[("main", "MAP-0001")]);
    let wide = frame_document_sized("MAP-0001", "main", "ALPHA", 56, 18);
    let package = Frame::new(Kind::Decl, surface.as_bytes()).expect("a declaration frame");
    let refusals = load(&package, "alpha-shell", &[&wide], 1, 0, 100)
        .expect_err("56 by 18 does not agree with 42 by 18");
    assert_eq!(refusals.code(), "cellframe-extent");
    let message = refusals.message();
    assert!(message.contains("cellframe line 5"), "the binding line is missing: {message}");
    assert!(message.contains("MAP-0001"), "the frame id is missing: {message}");
    assert!(message.contains("main"), "the window id is missing: {message}");
}

#[test]
fn a_frame_smaller_than_its_window_is_refused_at_load() {
    let surface = surface_document(&[("main", "MAP-0001")]);
    let narrow = frame_document_sized("MAP-0001", "main", "ALPHA", 40, 18);
    let package = Frame::new(Kind::Decl, surface.as_bytes()).expect("a declaration frame");
    let refusals = load(&package, "alpha-shell", &[&narrow], 1, 0, 100)
        .expect_err("40 by 18 leaves two declared columns undrawn");
    assert_eq!(refusals.code(), "cellframe-extent");
    let message = refusals.message();
    assert!(message.contains("cellframe line 5"), "the binding line is missing: {message}");
    assert!(message.contains("40 by 18"), "the frame's extent is missing: {message}");
    assert!(message.contains("42 by 18"), "the window's extent is missing: {message}");
}

#[test]
fn an_extent_disagreement_does_not_stop_the_other_window_resolving() {
    let surface = parse(&surface_document(&[("main", "MAP-0001"), ("plan", "MAP-0002")]));
    let wide = frame_document_sized("MAP-0001", "main", "ALPHA", 56, 18);
    let beta = frame_document("MAP-0002", "plan", "BETA");
    let refusals = resolve(&surface, &[&wide, &beta]).expect_err("main disagrees");
    assert_eq!(refusals.code(), "cellframe-extent");
    // The agreeing window is not reported as unreferenced: a bad extent is not a missing binding.
    assert!(
        !refusals.message().contains("bound by no declaration"),
        "an extent refusal was reported as an unreferenced frame: {}",
        refusals.message()
    );
}

#[test]
fn a_frame_addressed_to_another_window_is_refused() {
    let surface = surface_document(&[("main", "MAP-0001")]);
    let elsewhere = frame_document_sized("MAP-0001", "plan", "ALPHA", 42, 18);
    let package = Frame::new(Kind::Decl, surface.as_bytes()).expect("a declaration frame");
    let refusals = load(&package, "alpha-shell", &[&elsewhere], 1, 0, 100)
        .expect_err("the frame names a window the binding does not");
    assert_eq!(refusals.code(), "cellframe-extent");
    assert!(refusals.message().contains("line 5"), "{}", refusals.message());
}
