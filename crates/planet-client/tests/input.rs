//! The input router, exercised only by synthetic events against a declared surface: hit-testing by the
//! declared window rectangles, focus from a press, capture across a drag, key repeat, and the shapes
//! that route to nothing. No window is opened and no live input is produced.

use planet_client::input::{Button, Cell, Event, Key, Routed, Router};
use planet_package::Surface;

fn surface() -> Surface {
    let mut text = String::from("dl-enclave.surface/1\nsurface\talpha-shell\nrequires\tobserve\n");
    // 8 by 16 pixel cells: `main` covers px 0..336, `dock` covers px 336..416.
    text.push_str("window\tmain\tMain\tframe\tright-dock\t0\t0\t42\t18\t35\t9\ttrue\tfalse\tminimize\tdrag\tsurface\tcontent-main\n");
    text.push_str("node\tmain\troot\t-\tpane\tsurface\t-\n");
    text.push_str("window\tdock\tDock\tframe\tright-dock\t42\t0\t10\t18\t8\t9\ttrue\tfalse\tminimize\tdrag\tsurface\tcontent-dock\n");
    Surface::parse(text.as_bytes()).expect("a declaration that parses")
}

fn router() -> Router {
    Router::new(&surface())
}

fn cell(x: u32, y: u32) -> Option<Cell> {
    Some(Cell { x, y })
}

#[test]
fn a_pointer_inside_a_window_reports_that_window_and_its_cell() {
    let mut router = router();
    let routed = router.handle(Event::PointerMoved { x: 17, y: 33 });
    assert_eq!(
        routed,
        Routed::Hover {
            window: "main".to_string(),
            cell: cell(2, 2),
        }
    );
    assert_eq!(router.pointer(), Some((17, 33)));
}

#[test]
fn a_pointer_outside_every_window_routes_nothing_and_leaving_is_left() {
    let mut router = router();
    assert_eq!(router.handle(Event::PointerMoved { x: 500, y: 10 }), Routed::Nothing);
    assert_eq!(router.handle(Event::PointerLeft), Routed::Left);
    assert_eq!(router.pointer(), None);
}

#[test]
fn two_windows_route_by_the_point_and_not_by_declaration_order() {
    let mut router = router();
    assert_eq!(
        router.handle(Event::PointerMoved { x: 340, y: 8 }),
        Routed::Hover {
            window: "dock".to_string(),
            cell: cell(0, 0),
        }
    );
    // One pixel left of the boundary is the other window's last column.
    assert_eq!(
        router.handle(Event::PointerMoved { x: 335, y: 8 }),
        Routed::Hover {
            window: "main".to_string(),
            cell: cell(41, 0),
        }
    );
}

#[test]
fn a_press_focuses_and_captures_so_a_release_outside_still_reaches_it() {
    let mut router = router();
    router.handle(Event::PointerMoved { x: 17, y: 17 });
    assert_eq!(
        router.handle(Event::PointerPressed {
            button: Button::Primary
        }),
        Routed::Press {
            window: "main".to_string(),
            cell: Cell { x: 2, y: 1 },
        }
    );
    assert_eq!(router.focus(), Some("main"));
    assert_eq!(router.captured(), Some("main"));

    // Dragged outside the window: the capture still names it, with no cell to report.
    assert_eq!(
        router.handle(Event::PointerMoved { x: 500, y: 500 }),
        Routed::Hover {
            window: "main".to_string(),
            cell: None,
        }
    );
    assert_eq!(
        router.handle(Event::PointerReleased {
            button: Button::Primary
        }),
        Routed::Release {
            window: "main".to_string(),
            cell: None,
            captured: true,
        }
    );
    assert_eq!(router.captured(), None, "the release ends the capture");
}

#[test]
fn a_wheel_goes_to_the_window_under_the_pointer_with_its_delta() {
    let mut router = router();
    assert_eq!(router.handle(Event::Wheel { delta: 3 }), Routed::Nothing);
    router.handle(Event::PointerMoved { x: 340, y: 24 });
    assert_eq!(
        router.handle(Event::Wheel { delta: -2 }),
        Routed::Wheel {
            window: "dock".to_string(),
            cell: cell(0, 1),
            delta: -2,
        }
    );
}

#[test]
fn the_second_press_of_a_held_key_is_a_repeat() {
    let mut router = router();
    router.handle(Event::PointerMoved { x: 4, y: 4 });
    router.handle(Event::PointerPressed {
        button: Button::Primary,
    });
    let key = Key::Character('w');
    assert_eq!(
        router.handle(Event::KeyPressed(key)),
        Routed::Key {
            window: Some("main".to_string()),
            key,
            repeat: false,
        }
    );
    assert_eq!(
        router.handle(Event::KeyPressed(key)),
        Routed::Key {
            window: Some("main".to_string()),
            key,
            repeat: true,
        },
        "a key already held is the platform's repeat, and the client says so"
    );
    router.handle(Event::KeyReleased(key));
    assert_eq!(
        router.handle(Event::KeyPressed(key)),
        Routed::Key {
            window: Some("main".to_string()),
            key,
            repeat: false,
        }
    );
}

#[test]
fn text_and_keys_name_the_focused_window_and_no_window_before_a_press() {
    let mut router = router();
    assert_eq!(
        router.handle(Event::Text('x')),
        Routed::Text {
            window: None,
            glyph: 'x',
        }
    );
    assert_eq!(
        router.handle(Event::KeyPressed(Key::Enter)),
        Routed::Key {
            window: None,
            key: Key::Enter,
            repeat: false,
        }
    );
    router.handle(Event::PointerMoved { x: 340, y: 4 });
    router.handle(Event::PointerPressed {
        button: Button::Primary,
    });
    assert_eq!(
        router.handle(Event::Text('x')),
        Routed::Text {
            window: Some("dock".to_string()),
            glyph: 'x',
        }
    );
    assert_eq!(router.focus(), Some("dock"), "the last press owns the focus");
}

#[test]
fn a_press_outside_every_window_routes_nothing_and_keeps_the_focus() {
    let mut router = router();
    router.handle(Event::PointerMoved { x: 4, y: 4 });
    router.handle(Event::PointerPressed {
        button: Button::Primary,
    });
    router.handle(Event::PointerReleased {
        button: Button::Primary,
    });
    router.handle(Event::PointerMoved { x: 500, y: 500 });
    assert_eq!(
        router.handle(Event::PointerPressed {
            button: Button::Primary
        }),
        Routed::Nothing
    );
    assert_eq!(router.focus(), Some("main"), "a press on nothing moves no focus");
    assert_eq!(router.captured(), None);
}
