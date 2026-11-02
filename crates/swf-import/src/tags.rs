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
    walk_range(&header.body, header.tag_start, header.body.len())
}

/// Walk a tag stream between `start` and `end`, stopping at the End tag or at `end`.
pub fn walk_range(body: &[u8], start: usize, end: usize) -> Result<Vec<Tag>, Refusal> {
    let mut at = start;
    let mut tags = Vec::new();
    loop {
        if at + 2 > end {
            return Err(Refusal::Truncated {
                needed: at + 2,
                found: end,
            });
        }
        let word = u16::from_le_bytes([body[at], body[at + 1]]);
        let code = word >> 6;
        let mut length = (word & 0x3f) as usize;
        let mut body_at = at + 2;
        if length == 0x3f {
            if body_at + 4 > end {
                return Err(Refusal::Truncated {
                    needed: body_at + 4,
                    found: end,
                });
            }
            length = u32::from_le_bytes([
                body[body_at],
                body[body_at + 1],
                body[body_at + 2],
                body[body_at + 3],
            ]) as usize;
            body_at += 4;
        }
        if code == 0 {
            break;
        }
        if body_at + length > end {
            return Err(Refusal::Truncated {
                needed: body_at + length,
                found: end,
            });
        }
        tags.push(Tag {
            code,
            length,
            at: body_at,
        });
        at = body_at + length;
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
