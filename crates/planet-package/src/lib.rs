//! The canonical package layer: one magic, one frame, one surface grammar, one receipt, one entry.
//!
//! `V4PKG001` is the only package magic this repository knows (AGENTS.md §2). The frame carries a
//! schema, a kind, a length and a payload digest framed `domain || schema (LE u16) || len (LE u64) ||
//! payload`, so a truncated payload cannot collide with a longer one and a frame from another schema
//! is refused by name rather than converted (AGENTS.md §12).
//!
//! A declaration payload is parsed by [`surface`] into typed windows, nodes and actions, and every row
//! or key the grammar does not define is refused by name. [`mount`] has no other path, and
//! [`record::mount_and_record`] writes the receipt as one `mount` entry through the ledger's own
//! durable path, idempotent on the frame's digest.

pub mod frame;
pub mod mount;
pub mod record;
pub mod surface;

pub use frame::{Frame, Kind, Refusal, DOMAIN, HEADER, MAGIC, PAYLOAD_LIMIT, SCHEMA};
pub use mount::{mount, Receipt};
pub use record::{ledger_code, mount_and_record, operation_id, MountRefusal, Recorded};
pub use surface::{Action, Node, Requirement, Surface, Window, SURFACE_SCHEMA};
