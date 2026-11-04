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

pub mod audio;
pub mod axis;
pub mod batch;
pub mod cell_mount;
pub mod console;
pub mod draw;
pub mod font;
pub mod input;
pub mod package_owner;
pub mod palette;
pub mod present;
pub mod picture;
pub mod render;
pub mod scene;
pub mod script;
pub mod timeline;
pub mod xml;

pub use batch::{plan as batch_plan, Batch, Quad};
pub use cell_mount::{Adopted, CellMount};
pub use draw::{rasters, Cell, Raster, WindowRaster};
pub use input::Router;
pub use palette::{Palette, Rgba};
pub use present::Instance;
pub use picture::Picture;
pub use package_owner::{decode, digest, encode};
pub use render::Scene;
pub use scene::{from_adopted, opened};
pub use script::{Driver, Step};
pub use timeline::Timeline;
pub use xml::Window;
