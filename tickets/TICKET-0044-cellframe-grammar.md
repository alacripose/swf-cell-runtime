# TICKET-0044 — The frame grammar is implemented from the owning reader

- Owner: implementation agent (the grammar), project owner (the rulings it rests on)
- Block: H1 declarations
- Branch: `ticket/TICKET-0044-cellframe-grammar`
- Ledger: LEDGER-0050
- Depends on: TICKET-0043 (the read that settled the grammar), `docs/CELL-SCHEMA.md` Q-1, Q-3, Q-5

## What was missing

`planet-package` could parse a surface declaration and nothing else. A `shploingus.cellframe/1`
document — the artifact that carries a window's cells — had no reader in this repository, so P6
(lowering a window definition into declared cells) had nowhere to lower into. Q-3 had also been ruled
without an implementation: the open vocabulary was a sentence in a document.

## What this adds

`crates/planet-package/src/cellframe.rs`, built to `shploingus-clean/crates/voxel-package/src/cellframe.rs`
rather than to a sample:

- the header, the `frame<TAB>id` row, the five required keys and the optional `feature` key;
- the two draw rows, `fill` and `run`, whose paper and ink are declared role words and never colours;
- the seven sampling rows `field`, `shaper`, `projection`, `centre`, `cell`, `extent`, `pitch`, as one
  declaration: any of them present means all of them are required, or the frame would be drawn as
  glyphs while claiming to be a map;
- the format's bounds — 512 KiB, 512 by 512, 16 384 cells, an 8 by 16 cell — as refusals, never as
  truncations;
- one refusal per problem, in one pass, with the line number: `cellframe-schema`, `cellframe-row`,
  `cellframe-key`, `cellframe-duplicate`, `cellframe-missing`, `cellframe-grid`, `cellframe-empty`,
  `cellframe-binding`, `cellframe-bound`, `cellframe-vocabulary`, `cellframe-extent`;
- `check_extent`, so a frame is refused when it does not fit the window's content space rather than
  drawn through the chrome. The window's extent is the declaration's own (`surface::Window::w`/`h`),
  so the caller passes it and the answer stays the format's.

Q-3 implemented: a declaration may open with a `[vocabulary]` section, terminated by a blank line,
declaring the `paper` and `ink` words that package may use. Without the section the format's own set
applies, so the section is additive and never required; with it, the set is replaced and a draw row
using an undeclared word is refused rather than drawn as the nearest known role. The section is
stripped before the frame grammar reads the document, so the owning grammar is not extended, reordered
or made optional by it.

## What this does not do

- No XML is lowered and no frame is attached to a declaration payload yet: a `V4PKG001` declaration
  still carries a surface, and the sibling-artifact path a package uses to name a frame by id is the
  next unit.
- No renderer draws a frame. The client surface is unchanged, and this unit claims no witness.
- The role words are strings validated against the streamed set. No palette is introduced: the raster
  that resolves a role to a colour is not this crate's.

## Acceptance test

`cargo test -p planet-package` exits 0, including 19 new scenarios in `tests/cellframe.rs`: a valid
frame and a full map declaration parse into typed records; a foreign header, an unknown key, a
duplicate key, missing keys in one pass, a row past the grid, an empty frame, an undeclared role, a
prose field source, an unsampled field, a second source for one field, a partial map, each bound, an
over-ceiling declaration, an unterminated vocabulary section and a frame wider than its window are
each refused by name with the line.

`python tools/port_gate.py selftest` exits 0 (123 scenarios) and `python tools/port_gate.py check`
exits 0 (21 checks, 4 not enforceable).

## Closing

- Commit: this unit's commit on `ticket/TICKET-0044-cellframe-grammar` (unmerged; no remote configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0050
