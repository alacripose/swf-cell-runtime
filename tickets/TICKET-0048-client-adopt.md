# TICKET-0048 — The client adopts an already-mounted, extent-checked surface

- Owner: implementation agent (the adoption path), project owner (the rulings it rests on)
- Block: H9 client surface (adoption; the rendering witness stays the shell's)
- Branch: `ticket/TICKET-0048-client-adopt`
- Ledger: LEDGER-0054
- Depends on: TICKET-0047 (the surface mount), TICKET-0046 (extent agreement), TICKET-0045 (binding)

## Confirmed before starting

This unit assumes nothing from the previous turn's report. Checked first, in the tree: commit
`7645065` is HEAD with a clean tree; `cellframe::mount_loaded`, `SurfaceMount` and
`SurfaceMount::frame_of` are present in `crates/planet-package/src/cellframe.rs`; and
`tests/binding.rs` holds 21 scenarios, all green. Nothing needed restoring.

## What the plan defines for this step, and how this unit follows it

The plan's completion list, item 5, is *one Rust `winit`/`wgpu` client mounts and renders one cell
surface*, and its H9 row is the first playable integration witness — assigned to the shell and the
user, never to an agent. The previous ticket named this step the client mount and cited the rendering
half; the plan keeps that half out of agent scope. This unit therefore implements the half the plan
leaves buildable and checkable here: **the client consuming an already-mounted surface**, with no
renderer output. The divergence is stated rather than papered over: rendering, and the H9 witness,
remain open below.

## What was missing

`CellMount::adopt` decoded a package, parsed the surface and took a receipt — and stopped there. It
had no frames, so a client that wanted the frame bound to a window had to pair them itself, and the
validated `SurfaceMount` from the previous unit had no consumer. Adoption also re-derived the surface
id by parsing before mounting, because `load` demanded the id up front.

## What this adds

In `planet-package`:

- `Loaded` now carries the `surface` it parsed into, so the mount is for the surface the declaration
  actually names and nothing has to be parsed twice to learn it.
- `load_declared(frame, documents, sequence, previous, at)` — the same load as `load`, for a caller
  that holds the package and not the surface's id. `load` is now the same path plus its id check, so
  there is one load path, not two.
- `Loaded::mount()` — the loaded package mounts into its own surface, consuming its bindings.

In `planet-client`:

- `Adopted` carries `mount: SurfaceMount`, so adoption answers which frame belongs to which window.
- `Adopted::frame_of(window)` asks the mount; the client never re-resolves a reference, re-checks an
  extent or guesses a pairing.
- `CellMount::adopt_frames(bytes, frames, at)` loads through `load_declared`, mounts through
  `Loaded::mount`, and stores only if every step passed — digest deduplication, the code-kind
  refusal and the store-nothing-on-refusal rule are the ones `adopt` already had. `adopt` stays as it
  was for a package with no frame documents, and its `mount` is empty, which is what "no frames" means.

## Refusal vocabulary

No code was minted. The client surfaces the package crate's own refusals in the `cellframe-*` family
that this work already established: `cellframe-extent` for a frame whose extent disagrees with its
window (naming window id, frame id and binding line), `cellframe-unreferenced` for a frame no
declaration binds, and the pre-existing `frame-kind-reserved`, `frame-digest`, `frame-magic` and
`frame-truncated` for the package bytes themselves.

## What this does not do

- No renderer output, no wgpu/winit surface configuration that emits, no XML lowering, no colour
  resolution: a role stays a declared word.
- `shploingus.cellframe/1` keeps its exact meaning — no key, row, bound or ordering added, reordered,
  relaxed or reinterpreted.
- `check_extent` and the load-time extent refusal behave exactly as before, including the repair that
  refuses a frame *smaller* than its window.
- `bin/window.rs` still adopts with no frame documents; lowering a mounted frame into that window's
  scene is the next unit.
- The H9 playable witness, and everything the plan assigns to the shell or the user, stay open.

## Acceptance test

`cargo test -p planet-package -p planet-client` exits 0. The client's adopt suite is now 12 scenarios:
a client adopts a package whose frame resolves and agrees, and `frame_of("main")` answers the frame
the mount carries — with the binding's line and `None` for an unbound window; a package whose frame
disagrees with its window is refused `cellframe-extent` naming window, frame and line while the mount
stays empty; a frame the declaration never binds is refused `cellframe-unreferenced`; and adopting the
same mounted package twice answers the first adoption. The 21 binding, 19 cellframe, 22 surface and 13
vocabulary scenarios are unchanged and green, and the smaller-frame extent refusal is still proven.

`python tools/port_gate.py selftest` exits 0 (123 scenarios) and `python tools/port_gate.py check`
exits 0 (21 checks, 4 not enforceable).

## Closing

- Commit: this unit's commit on `ticket/TICKET-0048-client-adopt` (unmerged; no remote configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0054
