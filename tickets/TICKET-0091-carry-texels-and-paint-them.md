# TICKET-0091 — carry the bitmaps' texels in the package and paint them on the client

## Problem

`LEDGER-0094` decoded sixteen lossless payloads to real pixels, and then threw the pixels away: a
fill carried one mean colour, and the package carried no pixels at all. A client therefore had
nothing to draw but flat rectangles.

## Requirement

- `planet_package::texels::Texels` — the payload's own encoding: id, width, height, the source's
  format and colour-table layout, then `width * height` RGBA quads. A short or long payload is
  refused by name (`texel-header-short`, `texel-pixels-mismatch`, `texel-size-overflow`).
- The importer writes one `bitmap/<id>.rgba` artifact per decoded payload through the existing
  artifact framing (path, digest, hex bytes) and prints `bitmap_texel <id> <w>x<h> format <f>
  layout <l> digest <d>` for each, so a reader can compare the source decode with the read-back.
- `planet_client::texel` reads those artifacts and paints them: one texel per pixel at unit scale,
  nearest-neighbour when a placement's rect differs, transparent texels leaving the paper alone.
- A JPEG-tag bitmap stays counted and refused (`bitmap-tag-unimplemented`), never faked.

## Acceptance evidence

- `swf-import` prints `bitmaps 16 decoded 2 refused`, `bitmap_texels 16 bytes 1382400`, sixteen
  `bitmap_texel` rows, and two `bitmap_refused` rows naming `bitmap-tag-unimplemented`.
- `crates/swf-import/tests/texels.rs` reads the frame document back through the grammar, decodes
  every carried block, compares each block's digest with the digest the importer printed for the
  texels it decoded from the payload (byte-for-byte equality), re-encodes and decodes each block,
  paints the first block pixel-for-pixel into a picture and asserts zero mismatching pixels, then
  paints it into a stage region and asserts the region holds more than eight colours and no colour
  the payload does not hold.
- `cargo test -p swf-import` and `cargo test -p planet-client --lib` pass; `port_gate.py check`,
  `port_gate.py selftest` and `health.py` pass.
