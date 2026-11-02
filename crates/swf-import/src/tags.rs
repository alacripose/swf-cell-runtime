//! The tag stream: length-prefixed records after the header, ending at code 0.
//!
//! This module measures the stream. It interprets no tag body: converting a shape, a timeline or a
//! script is a lowering decision the format still owns.

use std::collections::BTreeMap;

use crate::header::{Header, Refusal};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tag {
    pub code: u16,
    pub length: usize,
    /// Offset of the tag body inside the decompressed stream.
    pub at: usize,
}

pub type TagCounts = BTreeMap<u16, usize>;

pub fn walk(header: &Header) -> Result<Vec<Tag>, Refusal> {
    let body = &header.body;
    let mut at = header.tag_start;
    let mut tags = Vec::new();
    loop {
        if at + 2 > body.len() {
            return Err(Refusal::Truncated {
                needed: at + 2,
                found: body.len(),
            });
        }
        let word = u16::from_le_bytes([body[at], body[at + 1]]);
        let code = word >> 6;
        let mut length = (word & 0x3f) as usize;
        let mut start = at + 2;
        if length == 0x3f {
            if start + 4 > body.len() {
                return Err(Refusal::Truncated {
                    needed: start + 4,
                    found: body.len(),
                });
            }
            length = u32::from_le_bytes([
                body[start],
                body[start + 1],
                body[start + 2],
                body[start + 3],
            ]) as usize;
            start += 4;
        }
        if code == 0 {
            break;
        }
        if start + length > body.len() {
            return Err(Refusal::Truncated {
                needed: start + length,
                found: body.len(),
            });
        }
        tags.push(Tag {
            code,
            length,
            at: start,
        });
        at = start + length;
    }
    Ok(tags)
}

pub fn counts(tags: &[Tag]) -> TagCounts {
    let mut counts = TagCounts::new();
    for tag in tags {
        *counts.entry(tag.code).or_insert(0) += 1;
    }
    counts
}
