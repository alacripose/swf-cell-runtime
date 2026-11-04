# TICKET-0090 — decode the lossless bitmap payloads to pixels and colour bitmap fills from them

## Problem

Every bitmap fill was drawn in one neutral colour because nothing decoded the 18 bitmap payloads the
file carries. "18 undecoded payloads" was a named gap in the evidence, not a picture.

## Requirement

- `crates/swf-import/src/bitmap.rs`: decode `DefineBitsLossless` (20) and `DefineBitsLossless2` (36)
  lossless formats 3, 4 and 5 to RGBA, refusing by name what it cannot read:
  `bitmap-tag-unimplemented`, `bitmap-body-truncated`, `bitmap-format-unimplemented`,
  `bitmap-size-zero`, `bitmap-size-overflow`, `bitmap-decompress-failed`, `bitmap-pixels-short`.
- The payload's framing is measured, not assumed: the 7-byte documented header and the 8-byte
  framing this file uses, zlib and raw deflate, are both attempted and a candidate is accepted only
  when what it inflates to is large enough for the bitmap's own stated size and format. The colour
  table's layout is measured the same way (`counted` or `full-256`).
- A bitmap fill carries the mean of its referenced bitmap's opaque texels, so a cell is drawn in the
  file's own colour instead of a stand-in; a fill whose bitmap was not decoded keeps `None`.

## Acceptance evidence

- `swf-import` prints `bitmaps 16 decoded 0 refused` and one line per bitmap with its size, format,
  texel count, distinct colours, first opaque texel and mean colour.
- `shape_fills 83 distinct 51` — the fill colours come from the payloads, not from one placeholder.
- `cargo test -p swf-import` passes, including nine decoder tests covering each format, both
  framings and every refusal.
- `inspect --evidence` still exits 0 over the imported package.

## Not carried by this unit

The pixels are decoded in the importer and used for fill colours; the package still carries colours
rather than texels, JPEG tags (6/21/35) are not decoded, and nothing executes the recorded programs
or routes input yet.
