# TICKET-0071 — The display list is lowered to cells, and the playtest says what is still missing

Goal, quoted from `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §1:

> "The base game and its mods must become peers in one cell-native package system."

Until this unit the importer read placements and nothing consumed them: the emitted frame was one
stage-sized paper. This unit adds the lowering, wires it into the frame and the declaration, and runs
the playtest — which does **not** yet draw the display list. The measured outcome is below, and it is
the artifact.

## What changed

- `emit::lower` turns placements into cells from facts the package already carries: a character's
  measured bounds (shape, sprite, or text rectangle), the placement's own translation (twips → pixels)
  and 16.16 scale, mapped into the frame's 8 × 16 cell grid. Nothing is invented: a character with no
  measured rectangle yields no cell and is counted.
- `emit::Lowering` carries `cells`, `outside`, `unmeasured`, `refused`, `placements`, and one
  `summary()` line. The declaration prints that line, so the counts a reader sees are the counts the
  frame carries; up to 24 placements are also named with depth and extent.
- `emit::frame_document` now emits one `fill … chrome` per lowered cell, back to front, over the
  stage paper, and still parses the whole document with `CellFrame::parse` before any bytes are
  written.
- `placement::survey_partial` counts a record it cannot read instead of aborting the import. The
  distinction is framing: a tag's length says where the next tag starts, so a record skipped whole
  costs nothing after it, while a field skipped inside one would move every later field.
- `placement::place_object_2` now reads the optional fields it used to refuse — name, ratio, colour
  transform, clip depth — each self-delimiting and in the format's order. Clip actions stay refused
  by name.
- `content::survey` measures text rectangles, buttons, edit-text and morph shapes, and characters
  **defined inside sprite bodies** (recursively, depth-bounded), because this format lets a sprite
  body define a character that the top-level display list places.
- New tests: eleven in `tests/frame_doc.rs` (lowering by shape, by scale, by translation, outside,
  unmeasured, sprite bounds, fill count, summary agreement, unreadable-record split) and two in
  `tests/placement.rs` (a named placement read to its end; clip actions refused).

## The playtest, measured

    placements 64 placements; 0 lowered into cells; 6 outside the stage;
               32 without measured bounds; 26 unreadable records

The client, mounted on that package, still reports `paper surface at (0, 0) for 80 cells` and
`batch 45 quads` — the stage, not the display list. So the lowering is implemented and tested, and
the end-to-end drawing the unit was opened for has **not** been achieved.

What the numbers say, and what they do not:

- **26 unreadable records** are refused by name, and the count is declared. They are the records the
  reader still does not parse (clip actions, and records carrying bytes beyond the fields this reader
  knows).
- **32 placements name characters with no measured rectangle.** The survey now measures shapes,
  morphs, buttons, edit-text, text and sprite bodies; these characters are defined by tag kinds it
  still does not read, and none of them may be given an invented extent.
- **6 placements are resolved and fall outside the stage's content space** — recorded, not a reason
  to widen the window, which stays exactly the frame's extent.

## Exit test

    cargo test -p swf-import -p planet-package -p planet-client   # exits 0, 13 new tests
    swf-import inputs/swf1.swf generated/swf1.pkg generated/mount-ledger.tsv swf1
    # prints the placements summary quoted above

## Blocked on, named

The 32 characters are the media and definition kinds whose *representation* the format still leaves
open: bitmap, font and sound payloads (D-2 names the kinds; M-1…M-5 do not yet declare their bytes)
and the definition tags this reader has not classified. Lowering them means declaring those payloads
in the package — not guessing a rectangle. Until that ruling lands, a placement of such a character
is counted and declared, never drawn as a box of invented size.

## Not claimed

The client does not yet draw the display list. `client_witness` and `client_run_h9` remain
`NOT-ENFORCED`; the capture is mechanical evidence of what the client printed, not a witness.
