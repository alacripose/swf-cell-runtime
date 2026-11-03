# TICKET-0047 — A loaded package mounts into the single surface, consuming its validated bindings

- Owner: implementation agent (the mount), project owner (the rulings it rests on)
- Block: H9 client surface (the mount half only; the rendering witness stays the shell's)
- Branch: `ticket/TICKET-0047-surface-mount`
- Ledger: LEDGER-0053
- Depends on: TICKET-0046 (extent agreement at load), TICKET-0045 (binding), TICKET-0044 (frame grammar)

## Why this unit, and what the plan says

The plan's next rendering step is row 786, *one Rust `winit`/`wgpu` client mounts and renders one cell
surface*, and its H9 row is the first playable integration witness, which the plan assigns to the shell
and the user — not to an agent, and not to a parser suite. So this unit takes the half that can be
built and checked here without a witness claim: **the mount that consumes already-validated bindings**,
in `planet-package`, where nothing draws. The rendering half is named as still open below rather than
absorbed into this ticket.

## What was missing

`load` returned a `Loaded`: a receipt plus the frames each declaration resolved to, extent-checked.
Nothing mounted them. A client had no value it could adopt, and the only thing standing between a
validated package and a surface was a caller doing the pairing itself, unchecked.

## What this adds

- `cellframe::WindowMount { window, line, frame }` and `cellframe::SurfaceMount { receipt, windows }`,
  with `SurfaceMount::frame_of(window)`.
- `cellframe::mount_loaded(&Loaded, &Surface) -> Result<SurfaceMount, Refusals>`: it **consumes** the
  resolved, extent-checked bindings. Nothing is re-resolved, nothing is re-measured, and the frame
  objects are carried rather than rebuilt — a test mounts the same `Loaded` twice and proves the two
  mounts are equal and that the carried frame is the loaded one.
- The mount's own question is its pairing: every bound window must be one the surface declares, or the
  mount would carry a frame addressed at a window that does not exist. That is refused
  `cellframe-surface`, naming the window id, the frame id and the binding row's line.

## Why that one code is new

The `cellframe-*` family had no code for "the surface being mounted declares no such window". The
nearest neighbours do not fit and were not stretched: `surface-node` is about a node grouped under a
window, `surface-windows` is about a surface declaring no windows at all, and `cellframe-reference` is
about a frame id the package does not declare. So the family gained exactly one code,
`cellframe-surface`, and the ticket records that it was minted rather than reused.

## What this does not do

- No renderer output, no XML lowering, no colour resolution. `role` stays a declared word.
- `shploingus.cellframe/1` is unchanged in meaning: no key, row, bound or ordering inside the frame
  grammar was added, reordered or relaxed.
- `check_extent` and the load-time extent refusal behave exactly as they did: the smaller-frame case
  still refuses at load, and is re-proven at the end of the mount suite.
- The mount does not require every window to carry a frame; only a frame nothing binds is refused, as
  before.
- The client (`planet-client`) is untouched. Adopting a `SurfaceMount` there, and the H9 witness, are
  the next units.

## Acceptance test

`cargo test -p planet-package` exits 0, with the binding suite at 21 scenarios: a loaded package
mounts into the surface it was loaded for with each window's own frame and its binding line; mounting
into a surface that lacks the bound window is refused `cellframe-surface` with the window id, the
frame id and line 5, while the surface it was loaded for still mounts; mounting consumes rather than
rechecks, proven by two equal mounts and a carried frame; and a frame smaller than its window still
never reaches a mount. The 19 cellframe, 22 surface and 13 vocabulary scenarios pass unchanged.

`python tools/port_gate.py selftest` exits 0 (123 scenarios) and `python tools/port_gate.py check`
exits 0 (21 checks, 4 not enforceable).

## Closing

- Commit: this unit's commit on `ticket/TICKET-0047-surface-mount` (unmerged; no remote configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0053
