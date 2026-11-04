//! The emitted frame and the placement lowering, proved against the grammar's own reader. Synthetic
//! bytes only: no byte of any supplied movie appears here.

use planet_package::cellframe::{check_extent, CellFrame, MAX_CELLS};
use planet_package::Surface;
use swf_import::content::{survey, Inventory, ShapeInfo};
use swf_import::emit::{self, lower, Lowering};
use swf_import::header::{self, Header, Rect};
use swf_import::placement::{Matrix, Placement};
use swf_import::tags;

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

/// A stage of 11,000 by 8,000 twips: 550 by 400 pixels, 69 by 25 cells.
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

fn shape(id: u16, x: i32, y: i32, w: i32, h: i32) -> ShapeInfo {
    ShapeInfo {
        id,
        bounds: Rect {
            x_min: x,
            x_max: x + w,
            y_min: y,
            y_max: y + h,
        },
        code: 2,
        bytes: 0,
        fill: None,
        geometry: None,
    }
}

fn inventory(shapes: Vec<ShapeInfo>) -> Inventory {
    Inventory {
        shapes,
        ..Inventory::default()
    }
}

fn sprite(id: u16, bounds: Rect) -> swf_import::content::SpriteInfo {
    swf_import::content::SpriteInfo {
        id,
        frames: 1,
        nested: 1,
        codes: vec![26, 1],
        draws: true,
        bounds: Some(bounds),
        bytes: 0,
        fill: None,
        first: Vec::new(),
        frames_list: Vec::new(),
    }
}

#[test]
fn a_placement_that_declares_a_clip_depth_is_a_mask_and_is_not_drawn() {
    // A mask states the depths it clips, not a picture. Painting its shape is how a full-stage mask
    // turns the whole frame into one colour, so no cell is drawn for it and the fact is counted.
    let shapes = inventory(vec![shape(7, 0, 0, 160, 320)]);
    let mut placement = placed(7, 4, (0, 0), 0x1_0000);
    placement.clip_depth = Some(10);
    let lowering = lower(&[placement], &shapes, 69, 25, 0);
    assert!(
        lowering.cells.is_empty(),
        "a mask is a region, not a colour"
    );
    assert_eq!(lowering.masked, 1);
    assert_eq!(lowering.proof[0].1, "mask");
}

#[test]
fn a_sprite_with_no_first_frame_is_stated_absent_not_boxed() {
    // The sprite states an extent (the union of its nested shapes) and its first frame places
    // nothing. A rectangle around that extent is a grid standing where the picture should be, so no
    // cell is drawn for it and the frame states the absence instead.
    let mut world = inventory(vec![]);
    world.sprites.push(sprite(
        21,
        Rect {
            x_min: 800,
            x_max: 2_400,
            y_min: 800,
            y_max: 4_000,
        },
    ));
    let placements = vec![placed(21, 6, (0, 0), 0x1_0000)];
    let lowering = lower(&placements, &world, 69, 25, 0);
    assert!(
        lowering.cells.is_empty(),
        "a sprite is never drawn as its bounding box"
    );
    assert_eq!(lowering.absent, 1, "the fact is stated, not silently dropped");
    assert_eq!(lowering.unmeasured, 0, "the extent was measured; the art is absent");
    assert_eq!(lowering.proof[0].1, "absent");
}

#[test]
fn a_sprite_draws_its_own_frame_of_the_movie() {
    // **The movie's frame n draws the sprite's frame n.** A loading bar advances because the sprite
    // inside the movie advances; re-reading the first frame for every tick is what made all twelve
    // frames identical and left the bar motionless.
    let mut world = inventory(vec![shape(7, 0, 0, 160, 320)]);
    let mut bar = sprite(
        9,
        Rect {
            x_min: 0,
            x_max: 160,
            y_min: 0,
            y_max: 3_200,
        },
    );
    bar.frames_list = vec![
        vec![placed(7, 1, (0, 0), 0x1_0000)],
        vec![placed(7, 1, (0, 2_400), 0x1_0000)],
    ];
    world.sprites.push(bar);
    let stage = vec![placed(9, 5, (0, 0), 0x1_0000)];
    let first = lower(&stage, &world, 69, 25, 0);
    let second = lower(&stage, &world, 69, 25, 1);
    assert!(!first.cells.is_empty(), "the sprite's first frame draws");
    assert!(!second.cells.is_empty(), "the sprite's second frame draws");
    assert_ne!(
        first.cells[0].row, second.cells[0].row,
        "frame 2 draws the sprite's second frame, not its first again"
    );
}

#[test]
fn a_sprite_without_art_is_stated_in_the_frame_document() {
    // A frameless sprite body: the file declares no display list, so the frame states the absence
    // with the reason and the owning rule rather than leaving the client to guess.
    let header = header();
    let mut world = inventory(vec![]);
    let mut empty = sprite(
        204,
        Rect {
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
        },
    );
    empty.frames = 0;
    empty.draws = false;
    empty.bounds = None;
    world.sprites.push(empty);
    let frame = CellFrame::parse(&emit::frame_document(
        &header,&Lowering::default(), &[], &world.sprites, &[], &[], &[], &[],
    ))
    .expect("the grammar accepts the absence section");
    assert_eq!(frame.absences.len(), 1);
    assert_eq!(frame.absences[0].kind, "sprite");
    assert_eq!(frame.absences[0].id, "204");
    assert_eq!(frame.absences[0].reason, "no-display-list");
    assert_eq!(frame.absences[0].rule, "cellframe::malformed-record");
    assert!(frame.fills.len() >= 1, "the stage paper still stands under it");
}

#[test]
fn a_sprite_whose_record_could_not_be_read_says_so() {
    let header = header();
    let mut world = inventory(vec![]);
    let mut unread = sprite(
        209,
        Rect {
            x_min: 0,
            x_max: 1_600,
            y_min: 0,
            y_max: 3_200,
        },
    );
    unread.first.clear();
    unread.draws = true;
    world.sprites.push(unread);
    let states = emit::sprite_absences(&world.sprites);
    assert_eq!(states.len(), 1);
    assert_eq!(states[0].reason, "undecoded-display-record");
    assert_eq!(states[0].rule, "cellframe::malformed-record");
}

fn placed(character: u16, depth: u16, translate: (i32, i32), scale: i32) -> Placement {
    Placement {
        character,
        depth,
        clip_depth: None,
        transparent: false,
        matrix: Some(Matrix {
            translate_x: translate.0,
            translate_y: translate.1,
            scale_x: scale,
            scale_y: scale,
            rotate_0: 0,
            rotate_1: 0,
        }),
        moved: false,
        code: 26,
    }
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
fn a_sprites_children_stack_by_their_own_depth_inside_the_sprites_slot() {
    // The sprite's first frame places the depth-5 child before the depth-1 child, which is a stream
    // order the painter must not follow: inside the sprite, the depth-1 child is behind.
    let mut world = inventory(vec![shape(7, 0, 0, 160, 320), shape(8, 0, 0, 160, 320)]);
    let mut group = sprite(
        30,
        Rect {
            x_min: 0,
            x_max: 1_600,
            y_min: 0,
            y_max: 3_200,
        },
    );
    group.first = vec![
        placed(7, 5, (0, 0), 0x1_0000),
        placed(8, 1, (0, 0), 0x1_0000),
    ];
    world.sprites.push(group);
    let placements = vec![placed(30, 3, (0, 0), 0x1_0000)];
    let lowering = lower(&placements, &world, 69, 25, 0);
    let drawn: Vec<u16> = lowering.cells.iter().map(|cell| cell.character).collect();
    assert_eq!(
        drawn,
        vec![8, 7],
        "the depth-1 child is painted before the depth-5 child"
    );
    assert_eq!(
        lowering.cells[0].depth, 3,
        "both children still sit at the sprite's own stage depth"
    );
    assert_eq!(
        lowering.cells[0].order,
        vec![3, 1],
        "the painter's path is the sprite's depth, then the child's"
    );
}

#[test]
fn the_frame_parses_and_binds_the_stage_window() {
    let header = header();
    let frame = CellFrame::parse(&emit::frame_document(&header, &Lowering::default(), &[], &[], &[], &[], &[], &[]))
        .expect("the grammar accepts it");
    assert_eq!(frame.window, "stage");
    assert_eq!(frame.id, emit::FRAME_ID);
    assert!(frame.cols * frame.rows <= MAX_CELLS, "the grid fits the ceiling");
    assert_eq!(
        frame.fills.len(),
        frame.rows as usize,
        "an empty display list draws the stage alone, one row per content line"
    );
    assert!(frame.runs.is_empty(), "the frame carries no annotation of its own");
}

#[test]
fn the_frame_extent_is_the_extent_the_window_declares() {
    let header = header();
    let frame = CellFrame::parse(&emit::frame_document(&header, &Lowering::default(), &[], &[], &[], &[], &[], &[]))
        .expect("the grammar accepts it");
    let tags = tags::walk(&header).expect("tags");
    let inventory = survey(&header, &tags).expect("survey");
    let counts = tags::counts(&tags);
    let package = emit::package_frame(
        "synthetic",
        &header,
        &counts,
        tags.len(),
        &inventory,
        &Lowering::default(),
    )
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
    let package = emit::package_frame(
        "synthetic",
        &header,
        &counts,
        tags.len(),
        &inventory,
        &Lowering::default(),
    )
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
    let frame = CellFrame::parse(&emit::frame_document(&header, &Lowering::default(), &[], &[], &[], &[], &[], &[]))
        .expect("the grammar accepts it");
    assert_eq!(
        (frame.cols, frame.rows),
        ((550 + 7) / 8, (400 + 15) / 16),
        "550 by 400 pixels is 69 by 25 whole cells, rounded up"
    );
}

#[test]
fn a_frame_the_grammar_refuses_is_never_returned() {
    let header = header();
    let bytes = emit::frame_bytes(&header, &Lowering::default(), &[], &[], &[], &[], &[], &[]).expect("a valid frame");
    let text = String::from_utf8(bytes).expect("utf-8");
    assert!(CellFrame::parse(&text).is_ok());
}

#[test]
fn a_placement_is_lowered_to_the_cells_its_shape_covers() {
    // A shape eight by sixteen pixels is exactly one cell, and it sits at the origin.
    let mut shapes = inventory(vec![shape(7, 0, 0, 160, 320)]);
    shapes.sprites.clear();
    let placements = vec![placed(7, 1, (0, 0), 0x1_0000)];
    let lowering = lower(&placements, &shapes, 69, 25, 0);
    assert_eq!(lowering.cells.len(), 1);
    let cell = &lowering.cells[0];
    assert_eq!((cell.col, cell.row, cell.cols, cell.rows), (0, 0, 1, 1));
    assert_eq!((cell.character, cell.depth), (7, 1));
    assert_eq!(lowering.placements, 1);
}

#[test]
fn a_scaled_placement_covers_the_cells_its_scale_makes() {
    // Twice the size is twice the cells in each direction, from the same measured bounds.
    let shapes = inventory(vec![shape(7, 0, 0, 160, 320)]);
    let placements = vec![placed(7, 2, (0, 0), 0x2_0000)];
    let lowering = lower(&placements, &shapes, 69, 25, 0);
    let cell = &lowering.cells[0];
    assert_eq!((cell.cols, cell.rows), (2, 2));
    assert_eq!(lowering.painted, 4, "the run covers four cells");
}

#[test]
fn a_translated_placement_lands_in_the_cells_its_translation_names() {
    // 1,600 twips is 80 pixels: ten cells right, five down.
    let shapes = inventory(vec![shape(7, 0, 0, 160, 320)]);
    let placements = vec![placed(7, 3, (1_600, 1_600), 0x1_0000)];
    let lowering = lower(&placements, &shapes, 69, 25, 0);
    let cell = &lowering.cells[0];
    assert_eq!((cell.col, cell.row), (10, 5));
}

#[test]
fn a_placement_outside_the_stage_is_counted_not_drawn() {
    // A negative translation puts the rectangle's start outside; the window's extent is a binding,
    // so the fact is recorded and the frame is not widened.
    let shapes = inventory(vec![shape(7, 0, 0, 160, 320)]);
    let placements = vec![placed(7, 4, (-2_000, 0), 0x1_0000)];
    let lowering = lower(&placements, &shapes, 69, 25, 0);
    assert!(lowering.cells.is_empty());
    assert_eq!(lowering.outside, 1);
    assert_eq!(lowering.placements, 1);
}

#[test]
fn a_character_with_no_measured_shape_is_not_invented() {
    let shapes = inventory(vec![shape(7, 0, 0, 160, 320)]);
    let placements = vec![placed(999, 5, (0, 0), 0x1_0000)];
    let lowering = lower(&placements, &shapes, 69, 25, 0);
    assert!(lowering.cells.is_empty());
    assert_eq!(lowering.unmeasured, 1);
}

#[test]
fn the_summary_agrees_with_the_cells_the_frame_draws() {
    let shapes = inventory(vec![shape(7, 0, 0, 160, 320)]);
    let placements = vec![
        placed(7, 1, (0, 0), 0x1_0000),
        placed(7, 2, (1_600, 1_600), 0x1_0000),
        placed(999, 3, (0, 0), 0x1_0000),
    ];
    let lowering = lower(&placements, &shapes, 69, 25, 0);
    assert_eq!(lowering.cells.len(), 2, "one cell per measured placement");
    assert_eq!(lowering.unmeasured, 1);
    assert_eq!(lowering.placements, 3);
    let summary = lowering.summary();
    assert!(summary.contains("3 placements"), "{summary}");
    assert!(summary.contains("2 lowered"), "{summary}");
    assert!(summary.contains("1 without measured bounds"), "{summary}");
}

#[test]
fn the_frame_draws_one_fill_per_lowered_placement_over_the_stage() {
    let header = header();
    let shapes = inventory(vec![shape(7, 0, 0, 160, 320), shape(8, 1_600, 1_600, 320, 640)]);
    let placements = vec![placed(7, 1, (0, 0), 0x1_0000), placed(8, 2, (0, 0), 0x1_0000)];
    let lowering = lower(&placements, &shapes, 69, 25, 0);
    let frame = CellFrame::parse(&emit::frame_document(&header, &lowering, &[], &[], &[], &[], &[], &[]))
        .expect("the grammar accepts the lowered frame");
    assert_eq!(
        frame.fills.len(),
        frame.rows as usize + lowering.cells.len(),
        "the stage paper plus one fill per placement"
    );
    assert_eq!(frame.fills[0].cols, frame.cols, "the stage stands under them");
}

#[test]
fn a_higher_depth_is_painted_after_a_lower_one() {
    // The display list is not in depth order: the last placed character is the lowest depth here.
    // The frame must carry the cells back to front, or a late low-depth rectangle paints over the
    // scene and the picture collapses to one colour.
    let shapes = inventory(vec![shape(7, 0, 0, 160, 320), shape(8, 0, 0, 160, 320)]);
    let placements = vec![
        placed(8, 9, (0, 0), 0x1_0000),
        placed(7, 1, (0, 0), 0x1_0000),
    ];
    let lowering = lower(&placements, &shapes, 69, 25, 0);
    let depths: Vec<u16> = lowering.cells.iter().map(|cell| cell.depth).collect();
    assert_eq!(
        depths,
        vec![1, 9],
        "the surface (1) is carried before the figure drawn in front of it (9)"
    );
}

#[test]
fn an_unreadable_record_is_counted_while_the_rest_is_lowered() {
    // A PlaceObject body shorter than its own fields: the strict survey refuses the stream, the
    // partial one counts it, because the tag's length still says where the next tag begins.
    let body = vec![1u8, 2];
    let tags = vec![swf_import::Tag {
        code: 4,
        length: 2,
        at: 0,
    }];
    assert!(
        swf_import::placement::survey(&tags, &body).is_err(),
        "a record skipped inside itself is not allowed"
    );
    let (placements, unreadable) =
        swf_import::placement::survey_partial(&tags, &body).expect("the framing is intact");
    assert!(placements.is_empty());
    assert_eq!(unreadable, 1);
}

#[test]
fn the_summary_declares_the_records_it_could_not_read() {
    let shapes = inventory(vec![shape(7, 0, 0, 160, 320)]);
    let placements = vec![placed(7, 1, (0, 0), 0x1_0000)];
    let mut lowering = lower(&placements, &shapes, 69, 25, 0);
    lowering.refused = 2;
    lowering.placements += 2;
    let summary = lowering.summary();
    assert!(summary.contains("3 placements"), "{summary}");
    assert!(summary.contains("1 lowered"), "{summary}");
    assert!(summary.contains("2 unreadable records"), "{summary}");
}
