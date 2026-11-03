# TICKET-0052 — The client projects a drawn frame into a draw batch

- Owner: implementation agent
- Block: H9 client surface (the headless half; the witness stays the shell's)
- Branch: `ticket/TICKET-0052-draw-batch`
- Ledger: LEDGER-0058
- Depends on: TICKET-0051 (the picture), TICKET-0050 (the cell raster), TICKET-0049 (the scene)

## Baseline, proven before writing anything

Run, not read from a ticket:

- `git branch -vv` shows **no `main` branch**: the trunk of this repository is `master`, and it was at
  `96ff265` (TICKET-0036). HEAD was `05e4586` on `ticket/TICKET-0051-picture`. The previous ticket's
  "main is at b33f651" was true of the campaign's previous *branch tip*, not of the trunk: every unit
  from TICKET-0037 on had been committed on stacked ticket branches and never landed. `master` was a
  direct ancestor of HEAD, so the trunk was fast-forwarded to `05e4586` before this unit started, and
  the picture unit is now on `master` with a clean tree. Nothing was rewritten or discarded.
- `grep -rnE 'Rgba|Palette|rgba|pixel|colour|color' crates/planet-package/src` returns nothing: the
  package crate still carries no colour, palette, raster or rendering concept.
- `crates/planet-client/src/picture.rs` reads only `WindowRaster`: no role resolution, no extent
  re-measurement, no document or binding parse on that path, and no string in the buffer.
- `crates/planet-client/src/palette.rs` is still the single place a role becomes a colour.
- Suites green when run at `05e4586`: picture 6, draw 5, scene 6, client adopt 12, binding 21,
  cellframe 19, surface 22, vocabulary 13.

Nothing needed repair; the only defect found was in the repository's history, not its code.

## Where the plan puts this unit

The plan's completion item 5 is *one Rust `winit`/`wgpu` client mounts and renders one cell surface*,
and §5.2 names what the renderer produces from mounted cell state: *static and dynamic mesh/quad
batches*, *font/text runs and glyph buffers*, layer/depth/mask/filter passes, and audio commands, with
device presentation in §5.1. Of those, the draw batch is the last step that is headless and assertable;
the rest need a device (presented by the shell) and gameplay is item 6 (the shell and the user). So this
unit implements the batch and stops at the device boundary — the divergence the previous tickets
carried forward, stated once more rather than quietly resolved differently.

## What this adds

**`crates/planet-client/src/batch.rs`** — `Quad { x, y, w, h, colour }`,
`Batch { width, height, quads }` and `plan(&WindowRaster) -> Result<Batch, Refusals>`. The input is the
raster the client already drew; nothing here resolves a role, re-measures a frame, or parses a
document. Paper is coalesced left to right into one rectangle per run of one colour (a run of the
transparent canvas is not a rectangle — the canvas is already the background), then each glyph
contributes one rectangle per horizontal run of set bits from the client's own font, in that glyph's
ink. Replaying the rectangles in order into a buffer of the batch's own size **is** the picture, byte
for byte; that equivalence is the unit's exit test, not a claim in prose.

**`bin/window.rs`** now appends `batch <n> quads for <frame>` beside the picture line, computed from
the same raster.

## Codes

**No new `cellframe-*` code was minted.** The batch reuses `cellframe-glyph` for a character the font
cannot draw (naming window, frame, the binding row's line and the character) and inherits
`cellframe-extent` from the load, which refuses a frame smaller than its window before any batch
exists. The batch is geometry, so it has no failure of its own that those two do not already name.

## What this does not do

- Nothing is presented, no window is opened, no GPU is requested or depended on, and no witness is
  claimed. H9 stays with the shell and the user.
- No XML lowering; `shploingus.cellframe/1` is untouched; `planet-package` gains no colour, palette,
  raster, geometry or rendering concept; `check_extent` and the load-time extent refusal are unchanged.
- The input router of §4.7 is not built here: its semantics are compiled by the shell from source
  behaviour, and guessing hit areas from declared nodes would be inventing the semantics the plan
  assigns to the shell's compilation.

## Acceptance test

`cargo test -p planet-package -p planet-client` exits 0. New `tests/batch.rs`, 7 scenarios: the batch
has the picture's own size and no rectangle leaves it; replaying the batch reproduces `picture::paint`
byte for byte for four different frame glyph-runs; a 42-cell paper row is one rectangle and a 3-cell
chrome run is one rectangle at row 1's pixel band, with unpainted rows contributing none; a glyph's ink
is 12 runs covering exactly the font's 18 set pixels, inside the glyph's 5 by 7 box; every rectangle's
colour is a palette colour and the picture it replays carries no role word; a character the font cannot
draw is refused `cellframe-glyph` with window, frame, line 6 and the character; and a frame smaller
than its window is refused `cellframe-extent` before any batch exists.

The 6 picture, 5 draw, 6 scene, 12 client adopt, 21 binding, 19 cellframe, 22 surface and 13
vocabulary scenarios are unchanged and green. `python tools/port_gate.py selftest` exits 0 (123
scenarios) and `python tools/port_gate.py check` exits 0 (21 checks, 4 not enforceable).

## Closing

- Commit: this unit's commit, fast-forwarded onto `master`
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0058
