//! What a window shows, proven by behaviour: the scene comes from the mount, a frame's words reach it
//! as role words, and a package that cannot be mounted has no scene at all.

use planet_client::render::Scene;
use planet_client::scene;
use planet_package::{Frame, Kind};

fn document(binding: Option<&str>) -> String {
    let mut text = String::from("dl-enclave.surface/1\nsurface\talpha-shell\nrequires\tobserve\n");
    text.push_str("window\tmain\tMain\tframe\tright-dock\t0\t0\t42\t18\t35\t9\ttrue\tfalse\tminimize\tdrag\tsurface\tcontent-main\n");
    text.push_str("node\tmain\troot\t-\tpane\tsurface\t-\n");
    if let Some(id) = binding {
        text.push_str(&format!("frame\tmain\t{id}\n"));
    }
    text
}

fn bytes(binding: Option<&str>) -> Vec<u8> {
    Frame::new(Kind::Decl, document(binding).as_bytes())
        .expect("a declaration frame")
        .encode()
}

fn frame_document(id: &str, cols: u32, rows: u32, glyphs: &str) -> String {
    format!(
        "shploingus.cellframe/1\n\
         frame\t{id}\n\
         \twindow\tmain\n\
         \tcols\t{cols}\n\
         \trows\t{rows}\n\
         \torder\t10\n\
         \trequires\tnone\n\
         fill\t0\t0\t{cols}\tsurface\n\
         run\t0\t0\ttext\t{glyphs}\n"
    )
}

fn body(scene: &Scene) -> String {
    scene.body()
}

#[test]
fn a_window_shows_the_frame_the_mount_answered_for_it() {
    let alpha = frame_document("MAP-0001", 42, 18, "ALPHA");
    let opened = scene::opened(&bytes(Some("MAP-0001")), &[&alpha], 7).expect("it mounts");
    assert_eq!(opened.title, "alpha-shell");
    let text = body(&opened);
    assert!(text.contains("window main — Main"), "{text}");
    assert!(text.contains("frame MAP-0001 — 42 by 18, order 10"), "{text}");
    assert!(text.contains("paper surface at (0, 0) for 42 cells"), "{text}");
    assert!(text.contains("text: ALPHA"), "{text}");
    assert!(!text.contains("carries no frame"), "{text}");
}

#[test]
fn a_role_stays_a_word_and_never_becomes_a_colour() {
    let alpha = frame_document("MAP-0001", 42, 18, "ALPHA");
    let text = body(&scene::opened(&bytes(Some("MAP-0001")), &[&alpha], 7).expect("it mounts"));
    assert!(text.contains("paper surface"), "{text}");
    assert!(text.contains("text: ALPHA"), "{text}");
    for marker in ["#", "rgb", "rgba", "0x", "colour(", "color("] {
        assert!(!text.contains(marker), "a colour reached the scene: {marker} in {text}");
    }
}

#[test]
fn a_window_with_no_binding_says_so_rather_than_inventing_a_frame() {
    let opened = scene::opened(&bytes(None), &[], 7).expect("a package with no frames still opens");
    let text = body(&opened);
    assert!(text.contains("window main — Main"), "{text}");
    assert!(text.contains("window main carries no frame"), "{text}");
}

#[test]
fn a_package_whose_frame_disagrees_with_its_window_has_no_scene() {
    let wide = frame_document("MAP-0001", 56, 18, "ALPHA");
    let refusals = scene::opened(&bytes(Some("MAP-0001")), &[&wide], 7)
        .expect_err("56 by 18 disagrees with the window's 42 by 18");
    assert_eq!(refusals.code(), "cellframe-extent");
    let message = refusals.message();
    assert!(message.contains("line 6"), "the binding line is missing: {message}");
    assert!(message.contains("MAP-0001"), "the frame id is missing: {message}");
    assert!(message.contains("main"), "the window id is missing: {message}");
}

#[test]
fn a_frame_the_declaration_never_binds_has_no_scene() {
    let alpha = frame_document("MAP-0001", 42, 18, "ALPHA");
    let refusals = scene::opened(&bytes(None), &[&alpha], 7)
        .expect_err("no frame row binds MAP-0001");
    assert_eq!(refusals.code(), "cellframe-unreferenced");
}

#[test]
fn a_frame_smaller_than_its_window_has_no_scene() {
    let narrow = frame_document("MAP-0001", 40, 18, "ALPHA");
    let refusals = scene::opened(&bytes(Some("MAP-0001")), &[&narrow], 7)
        .expect_err("40 by 18 leaves declared cells undrawn");
    assert_eq!(refusals.code(), "cellframe-extent");
    assert!(refusals.message().contains("line 6"), "{}", refusals.message());
}
