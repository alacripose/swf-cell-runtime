//! A node role is a theme role or a type role; the parser now records which, and the two tables are
//! proven disjoint so the classification is total and unambiguous.

use planet_package::vocabulary::{classify_role, RoleClass, Vocabulary};
use planet_package::{Refusal, Surface, SURFACE_SCHEMA};

fn window_row(theme: &str) -> String {
    format!(
        "window\tmain\tTitle\tframe\tright-dock\t0\t0\t42\t18\t35\t9\ttrue\tfalse\tminimize\tdrag\t{theme}\tcontent\n"
    )
}

fn document(theme: &str) -> String {
    format!(
        "{SURFACE_SCHEMA}\nsurface\talpha-shell\nrequires\tobserve\n{}",
        window_row(theme)
    )
}

fn with_role(kind: &str, role: &str) -> String {
    format!("{}node\tmain\troot\t-\t{kind}\t{role}\t-\n", document("surface"))
}

#[test]
fn every_theme_role_classifies_as_theme() {
    for role in Vocabulary::ThemeRole.entries() {
        assert_eq!(classify_role(role), Some(RoleClass::Theme), "{role}");
    }
}

#[test]
fn every_type_role_classifies_as_type() {
    for role in Vocabulary::TypeRole.entries() {
        assert_eq!(classify_role(role), Some(RoleClass::Type), "{role}");
    }
}

#[test]
fn an_undeclared_role_classifies_as_nothing() {
    assert_eq!(classify_role("big"), None);
    assert_eq!(classify_role(""), None);
}

#[test]
fn the_two_role_tables_are_disjoint() {
    for theme in Vocabulary::ThemeRole.entries() {
        assert!(
            !Vocabulary::TypeRole.accepts(theme),
            "{theme} is declared as both a theme and a type role, so its class is ambiguous"
        );
    }
}

#[test]
fn a_parsed_node_records_which_table_its_role_came_from() {
    for role in Vocabulary::ThemeRole.entries() {
        let surface = Surface::parse(with_role("pane", role).as_bytes()).expect("declared role");
        let node = &surface.windows[0].nodes[0];
        assert_eq!(node.role, *role);
        assert_eq!(node.role_class, RoleClass::Theme);
    }
    for role in Vocabulary::TypeRole.entries() {
        let surface = Surface::parse(with_role("label", role).as_bytes()).expect("declared role");
        let node = &surface.windows[0].nodes[0];
        assert_eq!(node.role, *role);
        assert_eq!(node.role_class, RoleClass::Type);
    }
}

#[test]
fn an_undeclared_role_is_refused_with_unknown_node_role() {
    match Surface::parse(with_role("pane", "big").as_bytes()) {
        Err(Refusal::UnknownValue {
            vocabulary,
            code,
            found,
        }) => {
            assert_eq!(vocabulary, "node role");
            assert_eq!(code, "unknown-node-role");
            assert_eq!(found, "big");
        }
        other => panic!("expected a node-role refusal, got {other:?}"),
    }
}
