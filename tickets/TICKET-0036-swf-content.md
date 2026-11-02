# TICKET-0036 — The importer measures tag bodies, not just tag codes

- Owner: implementation agent, under `AGENTS.md` §1.1 and `D-7`
- Block: H2 import boundary (streaming proof)
- Branch: `ticket/TICKET-0036-swf-content`
- Ledger: LEDGER-0042
- Depends on: TICKET-0035 (header and tag walk)

## What was missing

TICKET-0035 counted tag codes. A count says a movie has 129 sprites; it does not say how many frames
they hold, where the shapes sit, or how much of the file is bitmap, sound, font or text. The package
carried two numbers.

## What this builds

- `content::survey` reads tag bodies: `DefineShape/2/3/4` identifier and bit-packed bounds;
  `DefineSprite` identifier, declared frame count and nested tag count (via `tags::walk_range`);
  `ShowFrame` occurrences; and every media-carrying tag classified as bitmap, sound, font or text with
  its definition identifier (where the tag has one) and body size.
- `header::rect_at` and `tags::walk_range` became reusable readers, so shape bounds and sprite bodies
  are read by the same code as the header and top-level stream.
- The emitted document adds three nodes: shape count with the union of shape bounds in pixels, sprite
  count with total nested tags, and a media table with count and bytes per kind.

No tag body becomes geometry, a timeline or behaviour. `MediaKind` is an inventory classification in
the importer, not a package kind; the package still carries only the declared surface document.

## Real measurements from the file (one run)

- stage 640×480 px, 12 frames at 40 fps, 12 show-frame records, 600 tags
- 138 shapes; union bounds 2036×1614 px at (-688,-502)
- 129 sprites holding 10,866 nested tags
- media: bitmap 18 / 258,207 bytes; sound 9 / 20,668; font 6 / 37,385; text 51 / 2,156

Package digest `6023bd12b71f4ebd578cb43b7737f46b59003fd71bf46e6e47dbe36e133e810f`, mounted and
recorded; a second identical run answered the same digest and the same sequence with no new entry.

## Acceptance test

`cargo test -p swf-import` — 7 existing tests plus 5 in `tests/content.rs`: a shape's identifier and
bounds are measured, a sprite's frames and nested tags are measured, media is inventoried by kind and
size with a bitmap's identifier, a `SoundStreamHead` has no definition identifier, and the emitted
document carries the shape bounds, sprite structure and media table while still mounting.

## What remains blocked

Lowering a shape to geometry, a sprite to a timeline, or an action tag to behaviour needs
`docs/CELL-SCHEMA.md` Q-1…Q-6, media M-1…M-5 and D-3. Recorded in `D-7a`.

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0042
