# TICKET-0070 — The importer ships the frame its stage window binds

Goal, quoted from `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §1:

> "The base game and its mods must become peers in one cell-native package system."

A package that declares a window and ships nothing for it draws an empty stage. Until this unit the
importer emitted a declaration only, so the client mounted it, resolved its palette and painted an
empty batch: the mount was proved and nothing was drawn.

## What changed

- `emit::frame_document` writes the `shploingus.cellframe/1` document for the stage: one frame bound
  to the `stage` window, the whole stage as one `surface` paper, and one `text` run naming the stage.
  `emit::frame_bytes` parses it with the grammar's own reader before returning bytes, so the importer
  cannot write a frame the mount would refuse.
- `emit::surface_document` declares the window's content space in the frame's unit — cells of 8 by 16
  device-independent pixels — and binds the frame with a `frame<TAB>stage<TAB>STAGE-0001` row.
  A window declared in pixels can never be bound: `check_extent` requires the frame's extent to
  *equal* the window's content, and a content declared as 550 by 400 is 220,000 cells, far past the
  16,384-cell ceiling. The first attempt wrote the grid into the window's second size pair; the
  client refused `cellframe-extent`, naming both numbers, which is how the field was found.
- `bin/swf-import` writes `<out>.frame` beside the package and prints its path.
- New `tests/frame_doc.rs`: the frame parses, binds `stage`, its extent equals the window's declared
  content, the declaration binds the frame it ships, and the grid is the measured size in cells.

## Exit test

    cargo test -p swf-import -p planet-package -p planet-client   # exits 0
    swf-import inputs/swf1.swf generated/swf1.pkg generated/mount-ledger.tsv swf1
    # prints "frame generated/swf1.frame"; the client then reports
    # raster 80 by 30 cells, picture 640 by 480 rgba8, batch 45 quads

## What the client showed

Mounted, not refused: `window stage — Stage`, `frame STAGE-0001 — 80 by 30, order 10`, `paper
surface at (0, 0) for 80 cells`, `text: STAGE`, then the measured lines the declaration carries, and
`batch 45 quads for STAGE-0001`. The batch was empty before this unit.

## Not done here, and named

- The frame is one paper and one label. No placement, shape or sprite is lowered: the display list is
  read (TICKET-0064, TICKET-0067) and nothing consumes it, so what is drawn is the stage rectangle,
  not the game's artwork.
- One frame, not the timeline: the measured frame count is a label, not a sequence.
- Media counts are declared as text; no bitmap, sound or font is carried in the package yet.
- `client_witness` stays NOT-ENFORCED. A capture of a drawing client is not the owner's witness that
  the game plays.
