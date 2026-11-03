//! The presentation boundary, proven headlessly: a pixel rectangle becomes a clip-space instance by an
//! exact rule, that rule is reversible, the bytes are the layout the vertex buffer declares, and a
//! mounted package presents exactly the batch it drew — in order, with nothing invented for a hole.

use planet_client::batch::{self, Batch, Quad};
use planet_client::draw;
use planet_client::palette::Palette;
use planet_client::present::{self, Instance};
use planet_client::CellMount;
use planet_package::{Frame, Kind};

fn document() -> String {
    let mut text = String::from("dl-enclave.surface/1\nsurface\talpha-shell\nrequires\tobserve\n");
    text.push_str("window\tmain\tMain\tframe\tright-dock\t0\t0\t42\t18\t35\t9\ttrue\tfalse\tminimize\tdrag\tsurface\tcontent-main\n");
    text.push_str("node\tmain\troot\t-\tpane\tsurface\t-\n");
    text.push_str("frame\tmain\tMAP-0001\n");
    text
}

fn frame_document(cols: u32, glyphs: &str) -> String {
    format!(
        "shploingus.cellframe/1\n\
         frame\tMAP-0001\n\
         \twindow\tmain\n\
         \tcols\t{cols}\n\
         \trows\t18\n\
         \torder\t10\n\
         \trequires\tnone\n\
         \tfill\t0\t0\t{cols}\tsurface\n\
         \tfill\t0\t1\t3\tchrome\n\
         \trun\t2\t4\ttext\t{glyphs}\n"
    )
}

fn planned(glyphs: &str) -> Batch {
    let bytes = Frame::new(Kind::Decl, document().as_bytes())
        .expect("a declaration frame")
        .encode();
    let alpha = frame_document(42, glyphs);
    let mut mount = CellMount::new();
    let adopted = mount
        .adopt_frames(&bytes, &[&alpha], 7)
        .expect("it mounts");
    let drawn = draw::rasters(adopted, &Palette::declared()).expect("every role is carried");
    batch::plan(&drawn[0]).expect("every glyph is in the font")
}

/// The inverse of the projection, written out here rather than borrowed from the module under test.
fn back_to_pixels(instance: &Instance, viewport: (u32, u32)) -> (f32, f32, f32, f32) {
    let (w, h) = (viewport.0 as f32, viewport.1 as f32);
    let x = (instance.rect[0] + 1.0) / 2.0 * w;
    let y = (1.0 - instance.rect[1]) / 2.0 * h;
    let width = instance.rect[2] / 2.0 * w;
    let height = -instance.rect[3] / 2.0 * h;
    (x, y, width, height)
}

#[test]
fn every_instance_maps_back_to_the_batch_rectangle_it_came_from() {
    let batch = planned("AB");
    let viewport = (336, 288);
    let instances = present::instances(&batch.quads, viewport);
    assert_eq!(instances.len(), batch.quads.len());
    for (quad, instance) in batch.quads.iter().zip(&instances) {
        let (x, y, w, h) = back_to_pixels(instance, viewport);
        let expected = (quad.x as f32, quad.y as f32, quad.w as f32, quad.h as f32);
        for (found, want) in [(x, expected.0), (y, expected.1), (w, expected.2), (h, expected.3)] {
            assert!(
                (found - want).abs() < 0.0001,
                "quad {quad:?} came back as {} not {want}",
                found
            );
        }
    }
}

#[test]
fn the_origin_is_the_top_left_corner_and_y_grows_upwards_in_clip_space() {
    let viewport = (336u32, 288u32);
    let top = Quad {
        x: 0,
        y: 0,
        w: 8,
        h: 16,
        colour: Palette::declared().resolve("surface").unwrap(),
    };
    let second_row = Quad { y: 16, ..top };
    let projected = present::instances(&[top, second_row], viewport);

    assert_eq!(projected[0].rect[0], -1.0, "the left edge is the clip minimum");
    assert_eq!(projected[0].rect[1], 1.0, "picture row 0 is the clip maximum");
    assert_eq!(projected[0].rect[3], -2.0 * 16.0 / 288.0, "height is carried negative");
    assert_eq!(
        projected[1].rect[1],
        1.0 - 2.0 * 16.0 / 288.0,
        "one cell down is two cells lower in clip space"
    );
    // The same rectangle in a viewport twice as wide covers half the clip width.
    let wide = present::instances(&[top], (672, 288));
    assert_eq!(wide[0].rect[2], projected[0].rect[2] / 2.0);
}

#[test]
fn the_colour_is_the_palette_byte_over_255() {
    let palette = Palette::declared();
    let surface = palette.resolve("surface").unwrap();
    let quad = Quad {
        x: 0,
        y: 0,
        w: 8,
        h: 16,
        colour: surface,
    };
    let instance = present::instances(&[quad], (336, 288))[0];
    assert_eq!(
        instance.colour,
        [20.0 / 255.0, 22.0 / 255.0, 28.0 / 255.0, 1.0]
    );
    assert_eq!(instance.colour, present::colour(surface));
}

#[test]
fn the_instance_bytes_are_the_layout_the_vertex_buffer_declares() {
    let batch = planned("AB");
    let instances = present::instances(&batch.quads, (336, 288));
    let bytes = present::bytes(&instances);
    assert_eq!(Instance::STRIDE, 32);
    assert_eq!(bytes.len(), instances.len() * 32);

    for (index, instance) in instances.iter().enumerate() {
        let slice = &bytes[index * 32..index * 32 + 32];
        assert_eq!(slice, &instance.bytes()[..]);
        let floats: Vec<f32> = slice
            .chunks_exact(4)
            .map(|four| f32::from_le_bytes([four[0], four[1], four[2], four[3]]))
            .collect();
        assert_eq!(floats[0..4], instance.rect);
        assert_eq!(floats[4..8], instance.colour);
    }
}

#[test]
fn a_mounted_package_presents_exactly_the_batch_it_drew_in_order() {
    let batch = planned("A");
    let viewport = (336, 288);
    let instances = present::instances(&batch.quads, viewport);
    for (index, quad) in batch.quads.iter().enumerate() {
        let alone = present::instances(&[quad.clone()], viewport);
        assert_eq!(instances[index], alone[0], "instance {index} is not its quad");
    }
    // The glyph's ink is presented as its own rectangles, inside the glyph's cell band.
    let ink = Palette::declared().resolve("text").unwrap();
    let ink_instances: Vec<&Instance> = instances
        .iter()
        .zip(&batch.quads)
        .filter(|(_, quad)| quad.colour == ink)
        .map(|(instance, _)| instance)
        .collect();
    assert_eq!(ink_instances.len(), 12, "one instance per run of set bits");
    for instance in ink_instances {
        assert!(instance.rect[2] > 0.0, "a run has width");
        assert!(instance.rect[3] < 0.0, "a run's height is negative, one pixel tall");
        assert!(
            (instance.rect[3] + 2.0 * 1.0 / 288.0).abs() < 0.0001,
            "an ink run is exactly one picture pixel tall"
        );
    }
}

#[test]
fn a_hole_in_the_picture_presents_nothing() {
    assert!(present::instances(&[], (336, 288)).is_empty());
    assert!(present::bytes(&[]).is_empty());
    // The batch's transparent rows produced no rectangle at all, so nothing is invented for them.
    let batch = planned("AB");
    let paper: Vec<&Quad> = batch
        .quads
        .iter()
        .filter(|quad| quad.h == 16)
        .collect();
    assert!(
        paper.iter().all(|quad| quad.y < 32),
        "only the two painted rows carry paper"
    );
}
