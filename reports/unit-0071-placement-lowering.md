# Unit 0071 — placement lowering, measured

Source-safe: no supplied movie, its bytes, identifiers, offsets or media appear here. The only
numbers are counts the importer itself prints.

| Command | Exit |
|---|---|
| `cargo test -p swf-import -p planet-package -p planet-client` | 0 — 13 new tests |
| `swf-import <input> generated/swf1.pkg generated/mount-ledger.tsv swf1` | 0 |
| importer summary | `64 placements; 0 lowered into cells; 6 outside the stage; 32 without measured bounds; 26 unreadable records` |
| client | `paper surface at (0, 0) for 80 cells`, `batch 45 quads` |

## What the lowering does

For each placement in depth order: take the character's measured rectangle (shape, sprite union, or
text bounds), scale it by the placement's 16.16 factor, translate it by its twips/20 pixels, and map
the result into 8 × 16 cells. A rectangle starting outside the stage is counted; a character with no
measured rectangle is counted; a record the reader cannot parse is counted. Every count is printed in
the declaration, so the frame and the text agree by construction, and a test asserts it.

## What it does not do, in numbers

- 32 of 64 characters have no measured rectangle: their definitions are tag kinds this reader has not
  classified. The survey reads shapes, morph shapes, buttons, edit-text, text rectangles, and
  characters defined inside sprite bodies.
- 26 of 64 records are refused by name: clip actions and records with fields beyond this reader's
  knowledge.
- 0 cells were drawn, so the client still paints the stage paper. The batch did not grow.

## Why no box was invented for them

A placement is a position, not a size. Drawing 32 characters as arbitrary boxes would put invented
geometry into a package whose job is to preserve what the file says. The measured alternative is the
count, printed beside the frame: 32 declared, not 32 silently missing.
