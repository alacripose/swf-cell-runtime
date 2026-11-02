//! `package_owner`: a thin call into `planet-package`.
//!
//! The account's row says this adapter "owns no bytes". It therefore keeps no buffer, no cache and no
//! second reader: it forwards to the package crate's `Frame` type, which owns the package bytes, and
//! answers with that crate's own refusal when the bytes are not a frame this family defines.

use planet_package::{Frame, Refusal};

/// Decode one package frame. Every malformation is the package crate's refusal, unchanged.
pub fn decode(bytes: &[u8]) -> Result<Frame, Refusal> {
    Frame::decode(bytes)
}

/// Encode the frame back to its canonical bytes. The adapter copies only what the caller asked for.
pub fn encode(frame: &Frame) -> Vec<u8> {
    frame.encode()
}

/// The frame's own digest, which is what a receipt and the ledger's `operation_id` name.
pub fn digest(frame: &Frame) -> String {
    frame.frame_digest_hex()
}
