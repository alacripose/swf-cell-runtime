//! The declared vocabularies are load-bearing: every entry parses, and an undeclared value is
//! refused with its named code rather than defaulted.

use planet_package::{Refusal, Surface, Vocabulary, SURFACE_SCHEMA};

fn window_row(lane: &str, rule: &str, chrome: &str, moves: &str, theme: &str) -> String {
    format!(
        "window\tmain\tTitle\t{lane}\t{rule}\t0\t0\t42\t18\t35\t9\ttrue\tfalse\t{chrome}\t{moves}\t{theme}\tcontent\n"
    )
}

fn document(lane: &str, rule: &str, chrome: &str, moves: &str, theme: &str) -> String {
    format!(
        "{SURFACE_SCHEMA}\nsurface\talpha-shell\nrequires\tobserve\n{}",
        window_row(lane, rule, chrome, moves, theme)
    )
}

fn defaults() -> String {
    document("frame", "right-dock", "minimize", "drag", "surface")
}

fn with_node(kind: &str, role: &str) -> String {
    format!("{}node\tmain\troot\t-\t{kind}\t{role}\t-\n", defaults())
}

fn refusal_code(doc: &str) -> String {
    match Surface::parse(doc.as_bytes()) {
        Ok(_) => "accepted".to_string(),
        Err(refusal) => refusal.code().to_string(),
    }
}

#[test]
fn table_counts_are_pinned() {
    assert_eq!(Vocabulary::ALL.len(), 8);
    assert_eq!(Vocabulary::Lane.entries(), &["frame", "control", "tab", "world", "beacon", "motion"]);
    assert_eq!(Vocabulary::Lane.entries().len(), 6);
    assert_eq!(
        Vocabulary::Rule.entries(),
        &[
            "fill", "full-bleed", "centre", "centre-wide", "right-dock", "right-lower", "right-stack",
            "bottom-strip", "bottom-left", "small-right",
        ]
    );
    assert_eq!(Vocabulary::Rule.entries().len(), 10);
    assert_eq!(
        Vocabulary::NodeKind.entries(),
        &["pane", "row", "field", "label", "list", "table", "chart"]
    );
    assert_eq!(Vocabulary::NodeKind.entries().len(), 7);
    assert_eq!(Vocabulary::ThemeRole.entries(), &["surface", "hud"]);
    assert_eq!(Vocabulary::ThemeRole.entries().len(), 2);
    assert_eq!(
        Vocabulary::TypeRole.entries(),
        &["micro", "small", "body", "lead", "title", "display", "mono"]
    );
    assert_eq!(Vocabulary::TypeRole.entries().len(), 7);
    assert_eq!(Vocabulary::Chrome.entries(), &["minimize", "maximize", "close"]);
    assert_eq!(Vocabulary::Chrome.entries().len(), 3);
    assert_eq!(Vocabulary::Moves.entries(), &["drag", "resize"]);
    assert_eq!(Vocabulary::Moves.entries().len(), 2);
    assert_eq!(Vocabulary::Capability.entries(), &["observe", "act", "admin"]);
    assert_eq!(Vocabulary::Capability.entries().len(), 3);
}

#[test]
fn every_lane_is_accepted() {
    for lane in Vocabulary::Lane.entries() {
        let doc = document(lane, "right-dock", "minimize", "drag", "surface");
        let surface = Surface::parse(doc.as_bytes()).expect("declared lane must parse");
        assert_eq!(surface.windows[0].lane, *lane);
    }
}

#[test]
fn every_rule_is_accepted() {
    for rule in Vocabulary::Rule.entries() {
        let doc = document("frame", rule, "minimize", "drag", "surface");
        let surface = Surface::parse(doc.as_bytes()).expect("declared rule must parse");
        assert_eq!(surface.windows[0].rule, *rule);
    }
}

#[test]
fn every_theme_role_is_accepted() {
    for theme in Vocabulary::ThemeRole.entries() {
        let doc = document("frame", "right-dock", "minimize", "drag", theme);
        let surface = Surface::parse(doc.as_bytes()).expect("declared theme role must parse");
        assert_eq!(surface.windows[0].theme_role, *theme);
    }
}

#[test]
fn every_chrome_token_is_accepted() {
    for chrome in Vocabulary::Chrome.entries() {
        let doc = document("frame", "right-dock", chrome, "drag", "surface");
        let surface = Surface::parse(doc.as_bytes()).expect("declared chrome must parse");
        assert_eq!(surface.windows[0].chrome, *chrome);
    }
    let all = document("frame", "right-dock", "minimize,maximize,close", "drag", "surface");
    assert!(Surface::parse(all.as_bytes()).is_ok());
    let none = document("frame", "right-dock", "-", "drag", "surface");
    assert!(Surface::parse(none.as_bytes()).is_ok());
}

#[test]
fn every_move_is_accepted() {
    for moves in Vocabulary::Moves.entries() {
        let doc = document("frame", "right-dock", "minimize", moves, "surface");
        let surface = Surface::parse(doc.as_bytes()).expect("declared moves must parse");
        assert_eq!(surface.windows[0].moves, *moves);
    }
    let both = document("frame", "right-dock", "minimize", "drag,resize", "surface");
    assert!(Surface::parse(both.as_bytes()).is_ok());
}

#[test]
fn every_node_kind_is_accepted() {
    for kind in Vocabulary::NodeKind.entries() {
        let surface = Surface::parse(with_node(kind, "body").as_bytes())
            .expect("declared node kind must parse");
        assert_eq!(surface.windows[0].nodes[0].kind, *kind);
    }
}

#[test]
fn every_declared_role_is_accepted_as_a_node_role() {
    for role in Vocabulary::ThemeRole.entries() {
        let surface = Surface::parse(with_node("pane", role).as_bytes())
            .expect("declared theme role must parse as a node role");
        assert_eq!(surface.windows[0].nodes[0].role, *role);
    }
    for role in Vocabulary::TypeRole.entries() {
        let surface = Surface::parse(with_node("pane", role).as_bytes())
            .expect("declared type role must parse as a node role");
        assert_eq!(surface.windows[0].nodes[0].role, *role);
    }
}

#[test]
fn out_of_vocabulary_values_are_refused_by_name() {
    let cases: [(&str, String, &str); 7] = [
        (
            "lane",
            document("dock", "right-dock", "minimize", "drag", "surface"),
            "unknown-lane",
        ),
        (
            "rule",
            document("frame", "centre-left", "minimize", "drag", "surface"),
            "unknown-rule",
        ),
        (
            "chrome",
            document("frame", "right-dock", "hover", "drag", "surface"),
            "unknown-chrome",
        ),
        (
            "moves",
            document("frame", "right-dock", "minimize", "swipe", "surface"),
            "unknown-moves",
        ),
        (
            "theme",
            document("frame", "right-dock", "minimize", "drag", "chrome"),
            "unknown-theme-role",
        ),
        ("kind", with_node("pane2", "body"), "unknown-node-kind"),
        ("role", with_node("pane", "big"), "unknown-node-role"),
    ];
    for (subject, doc, expected) in cases {
        assert_eq!(refusal_code(&doc), expected, "{subject} must be refused");
    }
}

#[test]
fn the_refusal_names_the_value_it_refused() {
    let doc = document("dock", "right-dock", "minimize", "drag", "surface");
    match Surface::parse(doc.as_bytes()) {
        Err(Refusal::UnknownValue {
            vocabulary,
            code,
            found,
        }) => {
            assert_eq!(vocabulary, "lane");
            assert_eq!(code, "unknown-lane");
            assert_eq!(found, "dock");
        }
        other => panic!("expected a vocabulary refusal, got {other:?}"),
    }
}

#[test]
fn capability_ranks_are_the_declared_ladder() {
    assert_eq!(planet_package::capability_rank("observe"), Some(0));
    assert_eq!(planet_package::capability_rank("act"), Some(1));
    assert_eq!(planet_package::capability_rank("admin"), Some(2));
    assert_eq!(planet_package::capability_rank("root"), None);
}

#[test]
fn every_capability_is_accepted_by_requires() {
    for capability in Vocabulary::Capability.entries() {
        let doc = format!(
            "{SURFACE_SCHEMA}
surface	alpha-shell
requires	{capability}
{}",
            window_row("frame", "right-dock", "minimize", "drag", "surface")
        );
        let surface = Surface::parse(doc.as_bytes()).expect("declared capability must parse");
        assert_eq!(surface.requires.map(|r| r.name()), Some(*capability));
    }
}

#[test]
fn an_undeclared_capability_keeps_the_surface_requires_code() {
    let doc = format!(
        "{SURFACE_SCHEMA}
surface	alpha-shell
requires	root
{}",
        window_row("frame", "right-dock", "minimize", "drag", "surface")
    );
    assert_eq!(refusal_code(&doc), "surface-requires");
}
