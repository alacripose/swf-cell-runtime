# TICKET-0050 — A mounted frame is drawn as cells, each role resolved by a palette outside the package

- Owner: implementation agent (the palette and the raster), project owner (the rulings it rests on)
- Block: H9 client surface (drawing data; the on-screen witness stays the shell's)
- Branch: `ticket/TICKET-0050-draw-raster`
- Ledger: LEDGER-0056
- Depends on: TICKET-0049 (the window scene), TICKET-0048 (client adoption), TICKET-0047 (surface mount)

## Confirmed before starting

Checked in the tree rather than assumed: `7085d5b` is HEAD with a clean working tree apart from the
untracked helper this unit writes; `scene::opened(bytes, frames, at)` and `scene::from_adopted(&Adopted)`
are present; `bin/window.rs` reads the `.frame` files beside the newest package; and the suites hold
their reported counts — scene 6, client adopt 12, binding 21, cellframe 19, surface 22, vocabulary 13 —
all green on a baseline run before any edit. Nothing needed restoring.

## The plan, and how this unit follows it

The plan's completion item 5 is *one Rust `winit`/`wgpu` client mounts and renders one cell surface*,
and H9 is the first playable witness, assigned to the shell and the user. This unit is the drawing
half of that sentence taken as far as it can be taken headlessly: a raster that can be asserted on,
with no window opened and no pixel presented. The divergence is the same one the last three tickets
recorded — **the on-screen witness is not claimed here**.

## What was missing

The scene from the previous unit emitted `paper surface` and `text: ALPHA` as text. A role had no
colour anywhere in the repository, so nothing could be drawn: `planet-package` declares roles and
knows no colour, and the client had no palette to resolve one with.

## What this adds

**`crates/planet-client/src/palette.rs`** — the only place a role becomes a colour. `Rgba`, and
`Palette::declared()` carrying every paper and ink role the format names, in its own order;
`resolve(role)`, `without(role)` and `roles()`. Twelve roles, from `surface` to `orange`.

**`crates/planet-client/src/draw.rs`** — `rasters(&Adopted, &Palette) -> Result<Vec<WindowRaster>,
Refusals>`. Its input is the mounted package, so nothing on this path parses a frame document, a
binding or a surface, and nothing re-measures an extent. Its output is typed cells — position, resolved
paper colour, and optionally a glyph with its resolved ink colour — so no role word and no document
text survives into the picture. `Raster::cell(x, y)` and `Raster::row_text(y)` make the result
assertable; rows are emitted in the frame's own order: paper runs, then glyph runs.

**`bin/window.rs`** now adopts and mounts, shows the scene, and adds one `raster <cols> by <rows> cells
from <frame-id>` line per drawn frame — or the refusal it got instead.

## One code, and why

`cellframe-role` is new. The family had no code for *a paper or ink role the drawing cannot resolve*:
`cellframe-vocabulary` is about a word the package's own vocabulary section does not declare, which is
the package's fault, while this is the renderer's — the package is valid and the palette is short. The
refusal names the window id, the frame id, the binding row's line and the role, and no raster is
produced. It lives in `planet-package` because roles are that crate's vocabulary; the crate still
gains no colour, no palette and no rendering concept.

## What this does not do

- Nothing is presented and no window is opened. There is no wgpu or winit configuration work here, and
  no claim that anything is on screen.
- `planet-package` gains no colour knowledge, no palette, no raster.
- `shploingus.cellframe/1` keeps its exact meaning: no key, row, bound or ordering added, reordered,
  relaxed or reinterpreted.
- `check_extent` and the load-time extent refusal are unchanged, including the repair that refuses a
  frame *smaller* than its window — re-proven in the drawing suite.
- No XML lowering, and the H9 playable witness, with everything else the plan assigns to the shell or
  the user, stays open.

## What had to be repaired while building it

The drawing suite's first fixture used `accent` as a **paper** role. The package grammar correctly
refused it — papers and inks are different vocabularies — so the test was proving the grammar rather
than the drawing. The fixture now fills `chrome` and inks `accent`, and the palette-miss scenario tests
an ink role, which is the shape a real palette gap takes.

## Acceptance test

`cargo test -p planet-package -p planet-client` exits 0. The new drawing suite is 5 scenarios: every
row lands in its cell in the colour the palette answers (`surface` across row 0, `chrome` on three cells
of row 1, an unpainted cell transparent, `text` and `accent` glyphs on row 4 with `row_text(4) ==
"ABCD"`); no role or document text survives into the picture (only the run's glyphs, and no role word
among them); a role the palette lacks is refused `cellframe-role` naming window, frame, line 6 and the
role, with no substitution; the picture comes from the mount and describes the same frame the scene
does; and a frame smaller than its window is refused `cellframe-extent` before anything is drawn.

The 6 scene, 12 client adopt, 21 binding, 19 cellframe, 22 surface and 13 vocabulary scenarios are
unchanged and green. `python tools/port_gate.py selftest` exits 0 (123 scenarios) and
`python tools/port_gate.py check` exits 0 (21 checks, 4 not enforceable).

## Closing

- Commit: this unit's commit on `ticket/TICKET-0050-draw-raster` (unmerged; no remote configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0056
