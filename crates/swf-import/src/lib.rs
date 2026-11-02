//! SWF → canonical declaration package, for the streaming proof.
//!
//! Authorized by `AGENTS.md` §1.1: this crate reads `inputs/swf1.swf` and nothing else, and emits the
//! one canonical package frame the mount already accepts. It is not a Flash player, a renderer, or a
//! second package reader: it measures a file's structure and writes a declaration frame.

pub mod emit;
pub mod header;
pub mod tags;

pub use emit::{package_frame, surface_document};
pub use header::{read, Compression, Header, Rect, Refusal};
pub use tags::{counts, walk, Tag, TagCounts};
