//! The canonical package layer: one magic, one frame, one receipt.
//!
//! `V4PKG001` is the only package magic this repository knows (AGENTS.md §2). The frame carries a
//! schema, a kind, a length and a payload digest, and the digest is framed the way the family's
//! `planet-core/src/hash.rs` frames it — `domain || schema (LE u16) || len (LE u64) || payload` — so a
//! truncated payload cannot collide with a longer one and a frame from another schema is refused by
//! name rather than converted (AGENTS.md §12).

pub mod frame;
pub mod mount;

pub use frame::{Frame, Kind, Refusal, DOMAIN, HEADER, MAGIC, PAYLOAD_LIMIT, SCHEMA};
pub use mount::{mount, Receipt, SURFACE_SCHEMA};
