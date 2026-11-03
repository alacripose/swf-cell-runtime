# TICKET-0051 — The client paints a drawn frame into an offscreen RGBA8 picture

- Owner: implementation agent (the font and the picture), project owner (the rulings it rests on)
- Block: H9 client surface (offscreen drawing; the on-screen witness stays the shell's)
- Branch: `ticket/TICKET-0051-picture`
- Ledger: LEDGER-0057
- Depends on: TICKET-0050 (the cell raster), TICKET-0049 (the scene), TICKET-0048 (client adoption)

## Baseline, proven before writing anything

Run, not read from a ticket:

- `git log --oneline -1` → `b33f651`, and `git status --short` clean apart from the untracked helper.
- `grep -rn 'Rgba|Palette|pixel' crates/planet-package/src` → only the two cell-size constants' doc
  comments; the package carries no colour, palette, raster or pixel concept.
- `draw::rasters(adopted: &Adopted, palette: &Palette) -> Result<Vec<WindowRaster>, Refusals>` present,
  and `WindowRaster { window, frame, line, raster }` carries the ids and the binding line.
- `bin/window.rs` emits its `raster <cols> by <rows> cells from <frame-id>` line per drawn frame.
- `cellframe-role` present with `line`, `window`, `frame`, `role`.
- Suites at their counts and green when run: draw 5, scene 6, client adopt 12, binding 21, cellframe
  19, surface 22, vocabulary 13.

Nothing needed restoring.

## The plan, and how this unit follows it

The plan's completion item 5 is *one Rust `winit`/`wgpu` client mounts and renders one cell surface*,
and H9 is the first playable witness, assigned to the shell and the user. The previous ticket named an
offscreen texture with glyphs composited; this unit is that, minus the GPU: a concrete RGBA8 image a
test can assert on pixel by pixel, with no window opened, no device requested and no witness claimed.
The divergence is stated for the fourth time and is deliberate — the on-screen half stays open.

## What this adds

**`crates/planet-client/src/font.rs`** — a 5 by 7 bitmap for upper case, digits and the punctuation
frames use, with lower case folded to upper case (stated, not hidden), and `glyph(c) -> Option<[u8; 7]>`
where the leftmost pixel is bit 4. Dependency choice: the client already depends on glyphon, but
rasterising through glyphon needs a device, and this unit is headless, so a bitmap table is chosen — it
is deterministic, inspectable and assertable, and adds no dependency.

**`crates/planet-client/src/picture.rs`** — `Picture { width, height, stride, pixels }` with
`FORMAT = "rgba8"`, straight alpha, rows top-down, and `paint(&WindowRaster) -> Result<Picture,
Refusals>`. Its input is the raster the client already produced from the mounted, extent-checked
package, so nothing on this path resolves a role, re-checks or re-measures a frame, or parses a
document. Each cell's paper fills its 8 by 16 pixel rect; a glyph composites its ink inside that rect;
a cell with no glyph keeps its paper; an unpainted cell keeps the transparent paper the raster gave it,
so a hole is never a guessed colour.

**`bin/window.rs`** paints each drawn frame and appends `picture <w> by <h> rgba8 (stride <s>)`.

## One code, and why

`cellframe-glyph` is new. The family had nothing for *a character the drawing font cannot render* —
`cellframe-role` is about a role the palette lacks, and this is a glyph the font lacks. It names the
window id, the frame id, the binding row's line and the character, and no picture is produced. It lives
with the family because the rendered vocabulary is the family's; `planet-package` still gains no colour,
palette, raster or rendering concept.

## What this does not do

- Nothing is presented, no window is opened, no GPU is requested or depended on.
- No XML lowering; `shploingus.cellframe/1` keeps its exact meaning; `check_extent` and the load-time
  extent refusal are unchanged, including the repair that refuses a frame *smaller* than its window.
- H9 and everything the plan assigns to the shell or the user stay open.

## Repairs made while building it

Two, both in my own fixtures: the buffer-scan list mixed `&[u8]` with `&[u8; N]` and needed slicing,
and the narrow-frame fixture kept a 42-wide fill while declaring 40 columns — the package rightly
refused that as `cellframe-grid` before extents were ever consulted, which would have tested the wrong
predicate. The fixture now carries its columns into both the `cols` row and the fill.

## Acceptance test

`cargo test -p planet-package -p planet-client` exits 0. The new picture suite is 6 scenarios: the
picture has the format's dimensions and stride (`42*8` by `18*16`, stride `width*4`, buffer length
`stride*height`); every painted cell puts its paper at its own pixel rect and an unpainted cell stays
`[0,0,0,0]`; a glyph's ink is composited exactly where the font's 18 set bits say and nowhere else;
no role word or document text appears in the buffer and every pixel is a palette colour or the
transparent canvas; a character the font cannot draw is refused `cellframe-glyph` with window, frame,
line 6 and the character; and a frame smaller than its window is refused `cellframe-extent` before any
picture exists.

The 5 draw, 6 scene, 12 client adopt, 21 binding, 19 cellframe, 22 surface and 13 vocabulary scenarios
are unchanged and green. `python tools/port_gate.py selftest` exits 0 (123 scenarios) and
`python tools/port_gate.py check` exits 0 (21 checks, 4 not enforceable).

## Closing

- Commit: this unit's commit on `ticket/TICKET-0051-picture` (unmerged; no remote configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0057
