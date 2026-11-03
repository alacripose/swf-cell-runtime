# TICKET-0049 — A window's scene is derived from the mounted, extent-checked surface

- Owner: implementation agent (the scene), project owner (the rulings it rests on)
- Block: H9 client surface (scene data; rendering and the witness stay the shell's)
- Branch: `ticket/TICKET-0049-window-scene`
- Ledger: LEDGER-0055
- Depends on: TICKET-0048 (client adoption), TICKET-0047 (surface mount), TICKET-0046 (extent agreement)

## Confirmed before starting

Checked in the tree rather than assumed: commit `224d07a` is HEAD with a clean working tree (only the
untracked helper this unit writes); `Adopted.mount: SurfaceMount`, `Adopted::frame_of`,
`CellMount::adopt_frames` and `Loaded::mount` are all present; and the suites hold the reported counts
— client adopt 12, binding 21, cellframe 19, surface 22, vocabulary 13 — all green before any edit.
Nothing needed restoring.

## The plan, and how this unit follows it

The plan's completion item 5 is *one Rust `winit`/`wgpu` client mounts and renders one cell surface*,
and its H9 row is the first playable witness, assigned to the shell and the user. The previous ticket
named this step "lowering a mounted frame into the client scene", which is what this unit does — with
no renderer output. The divergence from the plan's sentence is the same one the last two tickets
recorded and is stated again: **the render half is not claimed here**; the scene this unit produces is
data the existing loop already consumes.

## What was missing

`bin/window.rs` adopted a bare package with no frame documents, so the frames a package declares never
reached the window. It also assembled its own lines from `surface.windows` and their nodes, which meant
the client re-decided what a window shows instead of consuming what the mount answered.

## What this adds

`crates/planet-client/src/scene.rs`:

- `scene::opened(bytes, frames, at)` — adopt with the frames the package declares, mount, then lower.
  This is the only path the window binary uses, and the refusal it returns is the package crate's own:
  `cellframe-extent` (with window id, frame id and the binding row's line), `cellframe-unreferenced`,
  and the frame codes for bad bytes. A refused package produces no scene at all.
- `scene::from_adopted(&Adopted)` — per window: its title, the frame the mount answered for it, that
  frame's paper and ink rows, then the window's label and table nodes. A window the mount bound nothing
  to says `window <id> carries no frame` instead of inventing one.
- A role reaches the scene as the **word** the package declared (`paper surface`, `text: ALPHA`). No
  colour, no palette, no draw call: a test asserts no `#`, `rgb`, `rgba`, `0x`, `color(` or `colour(`
  marker can appear in a scene body.

`bin/window.rs` now reads every `.frame` beside the newest package, hands them to `scene::opened`, and
shows the refusal's code and message if the package cannot be mounted. It parses nothing, pairs
nothing, and re-checks nothing.

## Refusal vocabulary

No code was minted. The unit surfaces `cellframe-extent`, `cellframe-unreferenced` and the existing
frame codes from `planet-package`, with their window id, frame id and binding line intact, as `Result`s.

## What this does not do

- No renderer output, no wgpu/winit surface configuration that emits, no XML lowering, no colour
  resolution.
- `shploingus.cellframe/1` keeps its exact meaning: no key, row, bound or ordering added, reordered,
  relaxed or reinterpreted.
- `check_extent` and the load-time extent refusal are unchanged, including the repair that refuses a
  frame *smaller* than its window — re-proven in the scene suite as well.
- The scene is not a cell raster: a frame's rows become text lines, which is what the one existing loop
  draws. Drawing cells, with their roles resolved, is a later unit and a different claim.
- The H9 playable witness, and everything the plan assigns to the shell or the user, stay open.

## Acceptance test

`cargo test -p planet-package -p planet-client` exits 0. The new client scene suite is 6 scenarios: a
window shows the frame the mount answered for it (id, extent, order, paper row and run text); a role
stays a word and no colour marker reaches the scene; a window with no binding says so rather than
inventing a frame; a package whose frame disagrees with its window has no scene and is refused
`cellframe-extent` with line 6, the window id and the frame id; a frame the declaration never binds has
no scene and is refused `cellframe-unreferenced`; and a frame smaller than its window has no scene.

The 12 client adopt, 21 binding, 19 cellframe, 22 surface and 13 vocabulary scenarios are unchanged and
green. `python tools/port_gate.py selftest` exits 0 (123 scenarios) and `python tools/port_gate.py
check` exits 0 (21 checks, 4 not enforceable).

## Closing

- Commit: this unit's commit on `ticket/TICKET-0049-window-scene` (unmerged; no remote configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0055
