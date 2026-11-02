//! The client side of the package door, for the two responsibilities `docs/SCOPE-ACCOUNTING.md`
//! names against "client crate needed":
//!
//! - [`package_owner`] is a thin call into `planet-package`; it owns no bytes and decodes nothing itself.
//! - [`cell_mount`] adopts a package or refuses it whole, and deduplicates by frame digest, which is
//!   `Mount::watch`'s semantics as the account records them.
//!
//! Both consume the package crate's own receipt and parsed surface; neither re-implements a frame
//! reader, a surface parser or a second transport (AGENTS.md §2). No `winit` or `wgpu` dependency is
//! taken here: the render and lifecycle rows stay `not-implemented` until their own units.

pub mod cell_mount;
pub mod package_owner;

pub use cell_mount::{Adopted, CellMount};
pub use package_owner::{decode, digest, encode};
