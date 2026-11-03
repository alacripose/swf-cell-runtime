//! `cell_mount`: adoption with `Mount::watch`'s semantics — digest deduplication and
//! adopt-or-refuse-whole.
//!
//! There is one path through this module. A frame is decoded by the package crate, its digest is
//! compared with what is already adopted, and only then is the surface parsed and the receipt taken
//! from `planet_package::mount`. Nothing is stored until every step has passed, so a refusal leaves
//! the mount exactly as it was; and adopting the same bytes twice answers the first adoption instead
//! of adding a second, because the digest is the identity (`AGENTS.md` §9: a retry is idempotent).

use planet_package::{Kind, Receipt, Refusal, Refusals, Surface, SurfaceMount};

use crate::package_owner;

/// One adopted package: the receipt the package crate issued, the surface it parsed, and the digest
/// that identifies the bytes.
#[derive(Clone, Debug, PartialEq)]
pub struct Adopted {
    pub receipt: Receipt,
    pub surface: Surface,
    pub digest: String,
    /// **The mount**: which frame each bound window carries, as the package crate answered it.
    pub mount: SurfaceMount,
}

impl Adopted {
    /// The frame this package's mount answers for `window`, or `None` when it bound none.
    ///
    /// The client asks the mount; it never re-resolves a reference, re-checks an extent or guesses a
    /// pairing.
    pub fn frame_of(&self, window: &str) -> Option<&planet_package::CellFrame> {
        self.mount.frame_of(window)
    }
}

/// What the client has adopted, in adoption order.
#[derive(Clone, Debug, Default)]
pub struct CellMount {
    adopted: Vec<Adopted>,
}

impl CellMount {
    pub fn new() -> Self {
        CellMount::default()
    }

    /// The adopted packages, oldest first. The sequence in each receipt is its position here.
    pub fn adopted(&self) -> &[Adopted] {
        &self.adopted
    }

    /// Adopt one package, or refuse it whole.
    ///
    /// The same bytes answer the first adoption and add nothing; a code frame is refused before its
    /// payload is read; a payload the surface grammar refuses stores nothing.
    pub fn adopt(&mut self, bytes: &[u8], at: u64) -> Result<&Adopted, Refusal> {
        let frame = package_owner::decode(bytes)?;
        let digest = package_owner::digest(&frame);
        if let Some(index) = self.adopted.iter().position(|entry| entry.digest == digest) {
            return Ok(&self.adopted[index]);
        }
        if frame.kind == Kind::Code {
            return Err(Refusal::FrameKindReserved { kind: "code" });
        }
        let surface = Surface::parse(&frame.payload)?;
        let sequence = self.adopted.len() as u64 + 1;
        let receipt = planet_package::mount(&frame, &surface.id, sequence, sequence - 1, at)?;
        self.adopted.push(Adopted {
            mount: SurfaceMount {
                receipt: receipt.clone(),
                windows: Vec::new(),
            },
            receipt,
            surface,
            digest,
        });
        let index = self.adopted.len() - 1;
        Ok(&self.adopted[index])
    }

    /// Adopt one package together with the frames it declares, or refuse it whole.
    ///
    /// The package is loaded through `planet_package::load_declared`, which resolves every `frame` row
    /// against the frames the package declares and refuses an extent that disagrees with its window.
    /// The mount is then produced from those validated bindings, so this client asks the mount which
    /// frame belongs to which window and never re-resolves, re-checks or re-measures one.
    pub fn adopt_frames(
        &mut self,
        bytes: &[u8],
        frames: &[&str],
        at: u64,
    ) -> Result<&Adopted, Refusals> {
        let package = match package_owner::decode(bytes) {
            Ok(package) => package,
            Err(refusal) => {
                let mut problems = Refusals::new();
                problems.refuse(refusal);
                return Err(problems);
            }
        };
        let digest = package_owner::digest(&package);
        if let Some(index) = self.adopted.iter().position(|entry| entry.digest == digest) {
            return Ok(&self.adopted[index]);
        }
        if package.kind == Kind::Code {
            let mut problems = Refusals::new();
            problems.refuse(Refusal::FrameKindReserved { kind: "code" });
            return Err(problems);
        }
        let sequence = self.adopted.len() as u64 + 1;
        let loaded =
            planet_package::load_declared(&package, frames, sequence, sequence - 1, at)?;
        let mount = loaded.mount()?;
        self.adopted.push(Adopted {
            receipt: mount.receipt.clone(),
            surface: loaded.surface.clone(),
            digest,
            mount,
        });
        let index = self.adopted.len() - 1;
        Ok(&self.adopted[index])
    }
}
