//! SWF → canonical declaration package, for the streaming proof.
//!
//! Authorized by `AGENTS.md` §1.1: this crate reads `inputs/swf1.swf` and nothing else, and emits the
//! one canonical package frame the mount already accepts. It is not a Flash player, a renderer, or a
//! second package reader: it measures a file's structure and writes a declaration frame. Tag bodies
//! are measured — shape bounds, sprite structure, media kinds — and never lowered into geometry,
//! timeline or behaviour, because those representations are not declared.

pub mod avm1;
pub mod bitmap;
pub mod content;
pub mod emit;
pub mod header;
pub mod placement;
pub mod shape;
pub mod stream;
pub mod tags;

pub use content::{survey, Inventory, Media, MediaKind, ShapeInfo, SpriteInfo};
pub use emit::{package_frame, surface_document};
pub use header::{read, Compression, Header, Rect, Refusal};
pub use placement::{Matrix, Placement};
pub use stream::{Read, Stream};
pub use tags::{counts, walk, Tag, TagCounts};
