//! The surface vocabularies, declared once.
//!
//! The plan does not name these tables; the shell family does, and AGENTS.md makes the family the
//! authority (D-6). They are taken verbatim from `secure-shell/tools/port/surface.mjs`, which is the
//! shell's projection of `dl-enclave.surface/1` and holds the declared sets beside the refusals
//! `unknown-lane` and `unknown-rule` that enforce them. Nothing here is invented: the names, their
//! order and their counts are the family's, and a test pins the counts so a silent edit fails.

/// One declared vocabulary: its name, the refusal code that enforces it, and its entries in declared
/// order. A table is data, so the parser and the tests read the same list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vocabulary {
    Lane,
    Rule,
    NodeKind,
    ThemeRole,
    TypeRole,
    Chrome,
    Moves,
    Capability,
}

/// The lanes a window may declare — `surface.mjs` `LANES`.
pub const LANES: [&str; 6] = ["frame", "control", "tab", "world", "beacon", "motion"];

/// The placement rules a window may declare — `surface.mjs` `RULES`.
pub const RULES: [&str; 10] = [
    "fill",
    "full-bleed",
    "centre",
    "centre-wide",
    "right-dock",
    "right-lower",
    "right-stack",
    "bottom-strip",
    "bottom-left",
    "small-right",
];

/// The node kinds — `surface.mjs` `NODE_KINDS`.
pub const NODE_KINDS: [&str; 7] = ["pane", "row", "field", "label", "list", "table", "chart"];

/// The theme roles — `surface.mjs` `THEME_ROLES`.
pub const THEME_ROLES: [&str; 2] = ["surface", "hud"];

/// The type roles — `surface.mjs` `TYPE_ROLES`.
pub const TYPE_ROLES: [&str; 7] = [
    "micro", "small", "body", "lead", "title", "display", "mono",
];

/// The chrome controls — `surface.mjs` `CHROME`.
pub const CHROME: [&str; 3] = ["minimize", "maximize", "close"];

/// The move kinds — `surface.mjs` `MOVES`.
pub const MOVES: [&str; 2] = ["drag", "resize"];

/// The capability ladder — `surface.mjs` `CAPABILITIES`, ranked by `CAPABILITY_RANK`.
pub const CAPABILITIES: [&str; 3] = ["observe", "act", "admin"];

impl Vocabulary {
    pub const ALL: [Vocabulary; 8] = [
        Vocabulary::Lane,
        Vocabulary::Rule,
        Vocabulary::NodeKind,
        Vocabulary::ThemeRole,
        Vocabulary::TypeRole,
        Vocabulary::Chrome,
        Vocabulary::Moves,
        Vocabulary::Capability,
    ];

    /// The table's name, as a refusal names it.
    pub fn name(self) -> &'static str {
        match self {
            Vocabulary::Lane => "lane",
            Vocabulary::Rule => "rule",
            Vocabulary::NodeKind => "node kind",
            Vocabulary::ThemeRole => "theme role",
            Vocabulary::TypeRole => "type role",
            Vocabulary::Chrome => "chrome",
            Vocabulary::Moves => "moves",
            Vocabulary::Capability => "capability",
        }
    }

    /// The refusal code. The family declares `unknown-lane` and `unknown-rule`; the rest follow the
    /// same shape so a reader can tell a vocabulary refusal from a field-shape refusal.
    pub fn code(self) -> &'static str {
        match self {
            Vocabulary::Lane => "unknown-lane",
            Vocabulary::Rule => "unknown-rule",
            Vocabulary::NodeKind => "unknown-node-kind",
            Vocabulary::ThemeRole => "unknown-theme-role",
            Vocabulary::TypeRole => "unknown-type-role",
            Vocabulary::Chrome => "unknown-chrome",
            Vocabulary::Moves => "unknown-moves",
            Vocabulary::Capability => "unknown-capability",
        }
    }

    pub fn entries(self) -> &'static [&'static str] {
        match self {
            Vocabulary::Lane => &LANES,
            Vocabulary::Rule => &RULES,
            Vocabulary::NodeKind => &NODE_KINDS,
            Vocabulary::ThemeRole => &THEME_ROLES,
            Vocabulary::TypeRole => &TYPE_ROLES,
            Vocabulary::Chrome => &CHROME,
            Vocabulary::Moves => &MOVES,
            Vocabulary::Capability => &CAPABILITIES,
        }
    }

    pub fn accepts(self, value: &str) -> bool {
        self.entries().iter().any(|entry| *entry == value)
    }
}

/// A node's role is drawn from the theme or type roles; the family declares both lists beside the
/// node grammar, so the union is the narrowest reading that accepts the declared names and nothing
/// else.
pub fn role_accepts(role: &str) -> bool {
    Vocabulary::ThemeRole.accepts(role) || Vocabulary::TypeRole.accepts(role)
}

/// A chrome or moves column is a comma-separated list of declared tokens, or `-` for none.
pub fn list_accepts(vocabulary: Vocabulary, field: &str) -> bool {
    field
        .split(',')
        .all(|token| token == "-" || vocabulary.accepts(token))
}

/// The capability rank, from `surface.mjs` `CAPABILITY_RANK`.
pub fn capability_rank(value: &str) -> Option<u8> {
    CAPABILITIES
        .iter()
        .position(|entry| *entry == value)
        .map(|index| index as u8)
}
