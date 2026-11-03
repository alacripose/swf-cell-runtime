//! The client side of the package door, for the two responsibilities `docs/SCOPE-ACCOUNTING.md`
//! names against "client crate needed":
//!
//! - [`package_owner`] is a thin call into `planet-package`; it owns no bytes and decodes nothing itself.
//! - [`cell_mount`] adopts a package or refuses it whole, and deduplicates by frame digest, which is
//!   `Mount::watch`'s semantics as the account records them.
//! - [`render`] is the one window loop the client formats share: a title and lines, drawn with glyphon
//!   over a wgpu surface.
//! - [`xml`] is the second format: a window definition lowered to those same lines.
//!
//! Both client binaries consume the package crate's own receipt and parsed surface; neither
//! re-implements a frame reader, a surface parser or a second transport (AGENTS.md §2).

pub mod cell_mount;
pub mod package_owner;
pub mod render;
pub mod scene;
pub mod xml;

pub use cell_mount::{Adopted, CellMount};
pub use package_owner::{decode, digest, encode};
pub use render::Scene;
pub use scene::{from_adopted, opened};
pub use xml::Window;
