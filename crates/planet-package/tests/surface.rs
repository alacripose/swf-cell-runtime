use planet_package::{mount, Frame, Kind, Refusal, Requirement, Surface, SURFACE_SCHEMA};

fn header(requires: &str) -> String {
    format!("{SURFACE_SCHEMA}\nsurface\talpha-shell\nrequires\t{requires}\n")
}

fn window_row(id: &str) -> String {
    format!(
        "window\t{id}\tTitle {id}\tframe\tright-dock\t0\t0\t42\t18\t35\t9\ttrue\tfalse\tminimize\tdrag\tsurface\tcontent-{id}\n"
    )
}

fn base() -> String {
    format!("{}{}", header("observe"), window_row("main"))
}

fn parse(document: &str) -> Result<Surface, Refusal> {
    Surface::parse(document.as_bytes())
}

#[test]
fn a_valid_surface_parses_into_types() {
    let surface = parse(&base()).unwrap();
    assert_eq!(surface.id, "alpha-shell");
    assert_eq!(surface.requires, Some(Requirement::Observe));
    assert_eq!(surface.windows.len(), 1);
    let window = &surface.windows[0];
    assert_eq!(window.id, "main");
    assert_eq!(window.title, "Title main");
    assert_eq!((window.x, window.y, window.w, window.h), (0, 0, 42, 18));
    assert_eq!((window.min_w, window.min_h), (35, 9));
    assert!(window.pinnable);
    assert!(!window.pinned);
    assert_eq!(window.content, "content-main");
    assert!(window.nodes.is_empty());
    assert!(window.actions.is_empty());
}

#[test]
fn nodes_and_actions_are_typed_with_their_parents() {
    let document = format!(
        "{}{}{}{}",
        base(),
        "node\tmain\troot\t-\tpane\tsurface\t-\n",
        "node\tmain\trefs\troot\ttable\tbody\t@refs\n",
        "action\tmain\tverify\tmain.verify\t-\n"
    );
    let window = &parse(&document).unwrap().windows[0];
    assert_eq!(window.nodes.len(), 2);
    assert_eq!(window.nodes[0].parent, None);
    assert_eq!(window.nodes[1].parent.as_deref(), Some("root"));
    assert_eq!(window.nodes[1].kind, "table");
    assert_eq!(window.actions.len(), 1);
    assert_eq!(window.actions[0].verb, "verify");
    assert_eq!(window.actions[0].target, "-");
}

#[test]
fn an_unknown_row_is_refused_by_name() {
    let document = format!("{}pane\tmain\tnode\n", base());
    assert_eq!(
        parse(&document),
        Err(Refusal::SurfaceRow {
            row: "pane".to_string()
        })
    );
}

#[test]
fn an_unknown_window_column_is_refused_by_name() {
    let document = format!(
        "{}{}",
        header("observe"),
        "window\tmain\tTitle\tframe\tright-dock\t0\t0\t42\t18\t35\t9\ttrue\tfalse\tminimize\tdrag\tsurface\tcontent\textra\n"
    );
    match parse(&document) {
        Err(refusal) => {
            assert_eq!(refusal.code(), "surface-field");
            assert!(refusal.message().contains("window"), "{}", refusal.message());
        }
        ok => panic!("expected a refusal, got {ok:?}"),
    }
}

#[test]
fn a_blank_row_is_refused() {
    let document = format!("{}\n", base());
    assert_eq!(
        parse(&document),
        Err(Refusal::SurfaceRow {
            row: "<blank>".to_string()
        })
    );
}

#[test]
fn a_duplicate_window_is_refused_by_name() {
    let document = format!("{}{}", base(), window_row("main"));
    assert_eq!(
        parse(&document),
        Err(Refusal::SurfaceDuplicate {
            key: "main".to_string()
        })
    );
}

#[test]
fn a_duplicate_surface_row_is_refused() {
    let document = format!("{}surface\talpha-shell\n{}", header("observe"), window_row("main"));
    assert_eq!(
        parse(&document),
        Err(Refusal::SurfaceDuplicate {
            key: "surface".to_string()
        })
    );
}

#[test]
fn a_duplicate_node_is_refused_by_name() {
    let document = format!(
        "{}{}{}",
        base(),
        "node\tmain\troot\t-\tpane\tsurface\t-\n",
        "node\tmain\troot\troot\tpane\tsurface\t-\n"
    );
    assert_eq!(
        parse(&document),
        Err(Refusal::SurfaceDuplicate {
            key: "root".to_string()
        })
    );
}

#[test]
fn an_older_surface_schema_is_refused_by_name() {
    let document = base().replace(SURFACE_SCHEMA, "dl-enclave.surface/0");
    assert_eq!(
        parse(&document),
        Err(Refusal::SurfaceSchema {
            found: "dl-enclave.surface/0".to_string()
        })
    );
}

#[test]
fn a_newer_surface_schema_is_refused_by_name() {
    let document = base().replace(SURFACE_SCHEMA, "dl-enclave.surface/2");
    match parse(&document) {
        Err(Refusal::SurfaceSchema { found }) => assert_eq!(found, "dl-enclave.surface/2"),
        other => panic!("expected a schema refusal, got {other:?}"),
    }
}

#[test]
fn a_parent_that_has_not_been_declared_is_refused() {
    let document = format!("{}node\tmain\ta\tghost\tpane\tsurface\t-\n", base());
    assert_eq!(
        parse(&document),
        Err(Refusal::SurfaceNode {
            window: "main".to_string(),
            node: "a".to_string()
        })
    );
}

#[test]
fn a_node_under_another_window_is_refused() {
    let document = format!("{}{}node\tmain\ta\t-\tpane\tsurface\t-\n", base(), window_row("other"));
    match parse(&document) {
        Err(Refusal::SurfaceNode { window, node }) => {
            assert_eq!(window, "main");
            assert_eq!(node, "a");
        }
        other => panic!("expected a node refusal, got {other:?}"),
    }
}

#[test]
fn an_action_under_another_window_is_refused() {
    let document = format!("{}{}action\tmain\tverify\ttool\t-\n", base(), window_row("other"));
    match parse(&document) {
        Err(Refusal::SurfaceAction { window, verb }) => {
            assert_eq!(window, "main");
            assert_eq!(verb, "verify");
        }
        other => panic!("expected an action refusal, got {other:?}"),
    }
}

#[test]
fn requires_after_a_window_is_refused() {
    let document = format!(
        "{}
surface	alpha-shell
{}requires	observe
",
        SURFACE_SCHEMA,
        window_row("main")
    );
    assert_eq!(
        parse(&document),
        Err(Refusal::SurfaceOrder {
            row: "requires".to_string()
        })
    );
}

#[test]
fn a_window_before_the_surface_row_is_refused() {
    let document = format!("{SURFACE_SCHEMA}\n{}", window_row("main"));
    assert_eq!(
        parse(&document),
        Err(Refusal::SurfaceOrder {
            row: "window".to_string()
        })
    );
}

#[test]
fn an_unknown_requirement_is_refused_by_name() {
    let document = format!("{}{}", header("write"), window_row("main"));
    assert_eq!(
        parse(&document),
        Err(Refusal::SurfaceRequires {
            found: "write".to_string()
        })
    );
}

#[test]
fn a_malformed_boolean_is_refused_by_name() {
    let document = format!(
        "{}{}",
        header("observe"),
        "window\tmain\tTitle\tframe\tright-dock\t0\t0\t42\t18\t35\t9\tmaybe\tfalse\tminimize\tdrag\tsurface\tcontent\n"
    );
    match parse(&document) {
        Err(refusal) => {
            assert_eq!(refusal.code(), "surface-field");
            assert!(refusal.message().contains("pinnable"), "{}", refusal.message());
        }
        ok => panic!("expected a refusal, got {ok:?}"),
    }
}

#[test]
fn an_impossible_extent_is_refused() {
    let document = format!(
        "{}{}",
        header("observe"),
        "window\tmain\tTitle\tframe\tright-dock\t0\t0\t0\t18\t35\t9\ttrue\tfalse\tminimize\tdrag\tsurface\tcontent\n"
    );
    match parse(&document) {
        Err(refusal) => {
            assert_eq!(refusal.code(), "surface-field");
            assert!(refusal.message().contains("extent"), "{}", refusal.message());
        }
        ok => panic!("expected a refusal, got {ok:?}"),
    }
}

#[test]
fn a_surface_that_is_not_this_schema_is_refused() {
    let document = "dl-enclave.surface\nsurface\talpha-shell\n";
    assert_eq!(
        parse(document),
        Err(Refusal::SurfaceSchema {
            found: "dl-enclave.surface".to_string()
        })
    );
}

#[test]
fn the_mount_path_cannot_bypass_the_grammar() {
    let payload = format!("{}pane\tmain\tnode\n", base());
    let frame = Frame::new(Kind::Decl, payload.as_bytes()).unwrap();
    match mount(&frame, "alpha-shell", 1, 0, 1_700_000_000) {
        Err(refusal) => {
            assert_eq!(refusal.code(), "surface-row");
            assert!(refusal.message().contains("pane"), "{}", refusal.message());
        }
        ok => panic!("a surface with an unknown row produced a receipt: {ok:?}"),
    }
}

#[test]
fn a_valid_surface_still_mounts() {
    let frame = Frame::new(Kind::Decl, base().as_bytes()).unwrap();
    let receipt = mount(&frame, "alpha-shell", 1, 0, 1_700_000_000).unwrap();
    assert_eq!(receipt.surface, "alpha-shell");
    assert_eq!(receipt.windows, 1);
    assert_eq!(receipt.state, "loaded");
}
