# TICKET-0064 — Placement parsing: character, depth and transform, read not guessed

- Owner: implementation agent, under D-7b
- Block: H2 import boundary
- Branch: `ticket/TICKET-0064-placement`
- Ledger: LEDGER-0070
- Step 1 of the order D-7b fixes

## What this is

The first thing any drawing needs is where a display list puts a character. `PlaceObject` (tag 4) and
`PlaceObject2` (tag 26) carry that: a character id, a depth, and a transform written as a bit-packed
matrix. `crates/swf-import/src/placement.rs` reads both into an approximation — `Placement { character,
depth, matrix, moved, code }` — with the matrix kept in the format's own units: twips for translation,
16.16 fixed point for scale and rotation, with helpers that convert to the pixels the surface declares.

It lowers nothing yet. No cell is produced, nothing is drawn, and the emitted declaration is unchanged;
this unit makes the placement data available and proven, which is what step 2 needs.

## Refusals, by name

- A body that ends inside a bit field it declared is `placement-truncated`, not a zero.
- A `PlaceObject2` whose flags name a field this reader does not parse (clip actions, clip depth, name,
  ratio, colour transform) is `placement-unsupported`. Skipping a field it did not read would put every
  later field at the wrong offset, so the record is refused rather than misread.
- `PlaceObject` with bytes after its matrix is likewise `placement-unsupported`: the trailing bytes are a
  colour transform, and calling them absent would be a lie.

## Depth semantics

`by_depth` folds the stream into the display list in force: the last placement at a depth wins, a
`place_object_2` with the move flag and no character edits the character already there, and the result is
ordered by depth. That is the display list a later step paints, not a second opinion about it.

## What this does not do

- No geometry: shape records are still only measured, not turned into cells.
- No timeline: show-frames are not accumulated into per-frame documents.
- No media, no script, and no change to `emit.rs` or the package.

## Acceptance test

`cargo test -p swf-import` — 7 placement scenarios: a `PlaceObject` names character, depth and pixel
position; a `PlaceObject2` reads scale and rotation; a move without a character keeps the character at
that depth; depths are ordered and the last placement wins; a truncated matrix is `placement-truncated`;
an unparsed flag is `placement-unsupported` and says why; the stream survey reads only the two placement
codes. Plus the unchanged suites, the gate's selftest and check, and a fresh-checkout identity probe.

## Closing

- Commit: this unit's commit, fast-forwarded onto `master`; nothing pushed
- Shell gate state: unassigned
- Ledger entry: LEDGER-0070
