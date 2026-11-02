//! Recording a mount in the shell ledger.
//!
//! A mount is not finished when `mount` returns a receipt: it is finished when the ledger carries a
//! `mount` entry for it and that entry is on disk (`secure-shell/docs/LEDGER.md`: "A change is landed
//! when its entry is appended and fsynced"). This module is that step, and it has three rules:
//!
//! 1. **One entry per mounted frame.** The ledger's `operation_id` is the frame's own digest, so the
//!    same bytes mounted twice are one entry: the second call answers `AlreadyRecorded` with the first
//!    entry's sequence and writes nothing. This is the ledger's existing idempotency rule
//!    (`docs/LEDGER.md` §3 step 3), read as the duplicate-mount rule rather than a new one.
//! 2. **A refusal leaves nothing.** The package refuses before the ledger is touched, and the ledger
//!    is cloned before the append, so a refused mount or a failed write leaves the caller's chain and
//!    the file exactly as they were. There is no side file and no partial entry.
//! 3. **The durable path is the ledger's.** The entry is written with `landing::land` — staging file
//!    beside the destination, `sync_all`, rename — the same path TICKET-0027 established, not a second
//!    writer.

use std::path::Path;

use planet_shell_ledger::{
    landing, Appended, Evidence, Kind, Ledger, Refusal as LedgerRefusal, Request,
};

use crate::frame::{Frame, Refusal};
use crate::mount::{mount, Receipt};

/// What a recorded mount answered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Recorded {
    /// A new entry landed: the receipt and the ledger sequence it occupies.
    Landed { receipt: Receipt, seq: u64 },
    /// The same frame was already recorded; the first entry's sequence is answered, nothing written.
    AlreadyRecorded { receipt: Receipt, seq: u64 },
}

#[derive(Debug)]
pub enum MountRefusal {
    Package(Refusal),
    Ledger(LedgerRefusal),
    LedgerUnavailable,
}

impl MountRefusal {
    /// The refusal's name, in the crate's style: package codes unchanged, ledger codes in the
    /// family's vocabulary (`ledger-stale`, `ledger-corrupt`, `ledger-invalid`, `ledger-unavailable`).
    pub fn code(&self) -> String {
        match self {
            MountRefusal::Package(refusal) => refusal.code().to_string(),
            MountRefusal::Ledger(refusal) => ledger_code(refusal),
            MountRefusal::LedgerUnavailable => "ledger-unavailable".to_string(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            MountRefusal::Package(refusal) => refusal.message(),
            MountRefusal::Ledger(refusal) => refusal.message(),
            MountRefusal::LedgerUnavailable => {
                "the ledger could not be written; no entry was landed".to_string()
            }
        }
    }
}

/// The family's ledger codes. `secure-shell/docs/LEDGER.md` and `docs/ARCHITECTURE.md` §7 name
/// `ledger-stale`, `ledger-corrupt` and `ledger-unavailable`; the shape refusals this crate's ledger
/// can answer with are grouped under `ledger-invalid`.
pub fn ledger_code(refusal: &LedgerRefusal) -> String {
    match refusal {
        LedgerRefusal::StaleSeq { .. } => "ledger-stale".to_string(),
        LedgerRefusal::BrokenChain { .. }
        | LedgerRefusal::NoGenesis
        | LedgerRefusal::MalformedLine { .. }
        | LedgerRefusal::WrongSchema { .. } => "ledger-corrupt".to_string(),
        LedgerRefusal::MissingField(_)
        | LedgerRefusal::EmptyOperationId
        | LedgerRefusal::Unrepresentable { .. }
        | LedgerRefusal::DigestNotSha256 { .. } => "ledger-invalid".to_string(),
    }
}

impl From<Refusal> for MountRefusal {
    fn from(refusal: Refusal) -> Self {
        MountRefusal::Package(refusal)
    }
}

impl From<LedgerRefusal> for MountRefusal {
    fn from(refusal: LedgerRefusal) -> Self {
        MountRefusal::Ledger(refusal)
    }
}

/// The idempotency key for a mount: the frame's own digest. Same bytes, same operation, one entry.
pub fn operation_id(frame: &Frame) -> String {
    format!("mount-{}", frame.frame_digest_hex())
}

/// Mount a declaration frame and land its receipt in the ledger.
///
/// `expected_seq` is the head the caller saw; a stale writer is refused `ledger-stale` rather than
/// overwriting, and `path` is the ledger's own durable destination.
#[allow(clippy::too_many_arguments)]
pub fn mount_and_record(
    frame: &Frame,
    surface: &str,
    sequence: u64,
    previous_sequence: u64,
    at: u64,
    ledger: &mut Ledger,
    path: &Path,
    expected_seq: u64,
) -> Result<Recorded, MountRefusal> {
    let receipt = mount(frame, surface, sequence, previous_sequence, at)?;
    let digest = frame.frame_digest_hex();
    let request = Request::new(
        &operation_id(frame),
        at,
        "planet-package",
        Kind::Mount,
        "TICKET-0031",
        &format!("mount/{surface}"),
        &digest,
        Evidence::new(
            "planet-package mount",
            0,
            receipt.windows,
            "planet-package",
            &digest,
        ),
    );

    // Clone first: a failed land must leave the caller's chain exactly as it was.
    let mut next = ledger.clone();
    match next.append(expected_seq, request)? {
        Appended::New => {
            landing::land(&next, path).map_err(|_| MountRefusal::LedgerUnavailable)?;
            let seq = next.head_seq();
            *ledger = next;
            Ok(Recorded::Landed { receipt, seq })
        }
        Appended::Existing(seq) => Ok(Recorded::AlreadyRecorded { receipt, seq }),
    }
}
