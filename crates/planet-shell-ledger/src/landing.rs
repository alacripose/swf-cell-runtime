//! Landing a chain on disk, and reading it back.
//!
//! The write is staged beside the destination, synced, and renamed over it, so a reader never opens a
//! half-written log and a refused entry leaves the destination byte-identical to what it was.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::{Ledger, Refusal};

/// The staging sibling for a destination, inside the destination's own directory.
pub fn staging_path(path: &Path) -> PathBuf {
    let mut name = path.file_name().map(|n| n.to_os_string()).unwrap_or_default();
    name.push(".staging");
    path.with_file_name(name)
}

/// Land the chain: stage, sync, rename. A refused render creates nothing.
pub fn land(ledger: &Ledger, path: &Path) -> io::Result<()> {
    let rendered = ledger
        .render()
        .map_err(|refusal| io::Error::new(io::ErrorKind::InvalidInput, refusal.message()))?;
    let staging = staging_path(path);
    {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&staging)?;
        file.write_all(rendered.as_bytes())?;
        file.sync_all()?;
    }
    fs::rename(&staging, path)?;
    if let Some(parent) = path.parent() {
        if let Ok(directory) = File::open(parent) {
            let _ = directory.sync_all();
        }
    }
    Ok(())
}

/// Read a chain from its durable form, verifying it before it is returned. A missing file is an
/// empty chain, not an error; an unreadable or broken one is.
pub fn load(path: &Path) -> io::Result<Result<Ledger, Refusal>> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Ledger::read(&text)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Ok(Ledger::new())),
        Err(error) => Err(error),
    }
}
