//! The hashed format identity: `family.noun/<32 hex>` (owner rulings Q1-Q3, 2026-11-02).
//!
//! A numeric id (`dl-enclave.surface/1`) is a legacy id: it names a format but not a version of it, so
//! two different grammars can both be `/1`. The identity this module computes covers the canonical
//! format text, the schema numbers the reader accepts, the reader version and the full refusal set, so
//! a change to any of those changes the id and an old reader cannot silently accept a new document.
//!
//! Three answers, and the owner's ruling decides which are refusals:
//!
//! - **reading**: the computed id is accepted, and a legacy numeric id is accepted and **upgraded** —
//!   the parsed document is the current format, and a writer re-emits it under the hashed id. Legacy
//!   ids are the upstream store's to replace (Q20), so refusing them on the way in would strand
//!   packages this reader can still understand.
//! - **reading a hash this reader did not compute**: refused `format-hash-mismatch`, naming what the
//!   document declared and what this reader computes. That is a different grammar, not an older name.
//! - **emission**: a legacy numeric id is refused `format-id-legacy-emitted`. A new package may only
//!   present a hashed id as current.
//!
//! The width is 32 hex characters (Q3). The preimage is line-oriented and sorted where order carries
//! no meaning, so the id does not depend on the order the source file lists its refusals.
//!
//! This unit does not move the id into the binary frame header: `V4PKG001` keeps its `schema` field
//! and its layout, and the hashed id travels as the declaration's first line, replacing the numeric
//! schema text. Changing the header layout is a separate unit with its own reader.

use std::sync::OnceLock;

use planet_shell_ledger::sha256;

use crate::frame::Refusal;

/// The format's family and noun. The id is this, a slash, and 32 hex characters.
pub const FAMILY: &str = "dl-enclave.surface";

/// The reader version: part of the preimage, so a reader that stops accepting a construct can change
/// the id by saying so rather than by widening the grammar in place.
pub const READER_VERSION: &str = "planet-package.reader.v1";

/// The schema numbers this reader accepts, in order.
pub const SCHEMAS: &[u16] = &[crate::frame::SCHEMA];

/// The refusal codes this reader can answer with. Kept as text, not as an enum walk, because the
/// preimage is the family's published set and must not drift when a variant is renamed internally.
pub const REFUSALS: &[&str] = &[
    "frame-magic",
    "frame-schema",
    "frame-kind",
    "frame-truncated",
    "frame-length",
    "frame-digest",
    "package-payload-limit",
    "frame-kind-reserved",
    "mount-sequence",
    "surface-row",
    "surface-field",
    "surface-duplicate",
    "surface-order",
    "surface-requires",
    "surface-node",
    "surface-action",
    "surface-schema",
    "surface-windows",
    "unknown-lane",
    "unknown-rule",
    "unknown-node",
    "unknown-role",
    "unknown-type-role",
    "unknown-chrome",
    "unknown-moves",
    "unknown-capability",
    "format-hash-mismatch",
    "format-id-legacy-emitted",
];

/// What a declaration's first line was, from this reader's point of view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The line is the id this reader computes.
    Current,
    /// The line is a legacy numeric id; the document is read as the current format.
    UpgradedLegacy,
}

impl Outcome {
    pub fn upgraded(self) -> bool {
        matches!(self, Outcome::UpgradedLegacy)
    }
}

/// The canonical format text the hash covers. One fact per line; refusals sorted, because the set is
/// a set and a reordering is not a format change.
pub fn preimage() -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("family\t{FAMILY}"));
    lines.push(format!("reader\t{READER_VERSION}"));
    for schema in SCHEMAS {
        lines.push(format!("schema\t{schema}"));
    }
    let mut codes: Vec<&str> = REFUSALS.to_vec();
    codes.sort_unstable();
    for code in codes {
        lines.push(format!("refusal\t{code}"));
    }
    let mut text = lines.join("\n");
    text.push('\n');
    text
}

/// The hashed id: `dl-enclave.surface/<32 hex>`.
pub fn id() -> &'static str {
    static ID: OnceLock<String> = OnceLock::new();
    ID.get_or_init(|| {
        let digest = sha256::sha256(preimage().as_bytes());
        let hex = digest.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        format!("{FAMILY}/{}", &hex[..32])
    })
}

/// Whether `text` is a hashed id: the family, a slash, and 32 lowercase hex characters.
pub fn is_hashed(text: &str) -> bool {
    let Some((family, rest)) = text.rsplit_once('/') else {
        return false;
    };
    family == FAMILY
        && rest.len() == 32
        && rest
            .chars()
            .all(|ch| ch.is_ascii_digit() || ('a'..='f').contains(&ch))
}

/// Whether `text` is a numeric id for this family: `dl-enclave.surface/1`, `/0`, `/2`.
pub fn is_legacy(text: &str) -> bool {
    let Some((family, rest)) = text.rsplit_once('/') else {
        return false;
    };
    family == FAMILY && !rest.is_empty() && rest.chars().all(|ch| ch.is_ascii_digit())
}

/// Reading: accept the computed id, accept a legacy id as an upgrade, refuse any other hash by name.
///
/// Text that is neither is left to the caller, which refuses it `surface-schema` (the header is not
/// this family's id at all).
pub fn read(declared: &str) -> Result<Outcome, Refusal> {
    if declared == id() {
        return Ok(Outcome::Current);
    }
    if is_legacy(declared) {
        return Ok(Outcome::UpgradedLegacy);
    }
    if is_hashed(declared) {
        return Err(Refusal::FormatHashMismatch {
            declared: declared.to_string(),
            found: id().to_string(),
        });
    }
    Err(Refusal::SurfaceSchema {
        found: declared.to_string(),
    })
}

/// Emission: a new document carries the computed id, and a legacy id is refused by name.
pub fn emit(declared: &str) -> Result<(), Refusal> {
    if declared == id() {
        return Ok(());
    }
    if is_legacy(declared) {
        return Err(Refusal::FormatIdLegacyEmitted {
            found: declared.to_string(),
            current: id().to_string(),
        });
    }
    Err(Refusal::FormatHashMismatch {
        declared: declared.to_string(),
        found: id().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_is_family_and_32_hex() {
        let id = id();
        assert!(is_hashed(id), "{id}");
        assert!(id.starts_with("dl-enclave.surface/"));
        assert_eq!(id.len(), FAMILY.len() + 1 + 32);
    }

    #[test]
    fn id_is_stable_across_calls() {
        assert_eq!(id(), id());
    }

    #[test]
    fn preimage_names_the_reader_and_every_refusal() {
        let text = preimage();
        assert!(text.contains(READER_VERSION));
        for code in REFUSALS {
            assert!(text.contains(code), "preimage omits {code}");
        }
    }

    #[test]
    fn current_id_reads_as_current() {
        assert_eq!(read(id()), Ok(Outcome::Current));
    }

    #[test]
    fn legacy_id_reads_as_an_upgrade() {
        for legacy in ["dl-enclave.surface/0", "dl-enclave.surface/1", "dl-enclave.surface/2"] {
            assert_eq!(read(legacy), Ok(Outcome::UpgradedLegacy), "{legacy}");
        }
    }

    #[test]
    fn foreign_hash_is_refused_by_name() {
        let foreign = format!("{FAMILY}/{}", "a".repeat(32));
        match read(&foreign) {
            Err(Refusal::FormatHashMismatch { declared, found }) => {
                assert_eq!(declared, foreign);
                assert_eq!(found, id());
            }
            other => panic!("expected format-hash-mismatch, found {other:?}"),
        }
    }

    #[test]
    fn other_text_is_refused_as_a_schema() {
        match read("dl-enclave.surface") {
            Err(Refusal::SurfaceSchema { found }) => assert_eq!(found, "dl-enclave.surface"),
            other => panic!("expected surface-schema, found {other:?}"),
        }
    }

    #[test]
    fn emission_refuses_a_legacy_id() {
        match emit("dl-enclave.surface/1") {
            Err(Refusal::FormatIdLegacyEmitted { found, current }) => {
                assert_eq!(found, "dl-enclave.surface/1");
                assert_eq!(current, id());
            }
            other => panic!("expected format-id-legacy-emitted, found {other:?}"),
        }
        assert!(emit(id()).is_ok());
    }

    #[test]
    fn short_hash_is_not_a_hashed_id() {
        assert!(!is_hashed("dl-enclave.surface/0123456789abcdef"));
        assert!(is_legacy("dl-enclave.surface/0"));
        assert!(!is_legacy("dl-enclave.surface/one"));
    }
}
