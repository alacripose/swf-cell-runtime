# TICKET-0045 — A declaration binds its frame by id, and loading resolves it

- Owner: implementation agent (the binding and the load path), project owner (the rulings it rests on)
- Block: H1 declarations
- Branch: `ticket/TICKET-0045-frame-binding`
- Ledger: LEDGER-0051
- Depends on: TICKET-0044 (the frame grammar), TICKET-0043 (the read that settled it), `docs/CELL-SCHEMA.md`

## What was missing

`planet-package` could read a frame through `cellframe` and could mount a declaration surface, and the
two never met. A package had no way to say *which* frame a window loads with, so a frame could be
declared and never reached, or referenced and never resolved, and neither state was visible. A
renderer was the only place a reference could have been noticed, which is exactly the deferral that
turns a missing frame into an empty window.

## The reference form

The declaration grammar already groups rows under the window they belong to — `node` and `action` both
open with the window id and are refused when they do not follow it. The binding is the same shape and
adds no syntax of its own:

```text
window<TAB>main<TAB>...<TAB>content-main
frame<TAB>main<TAB>MAP-0001
```

Three columns, grouped under the current window, refused `surface-order` when it follows no window or
the wrong one. The declaration never carries the frame's grammar, only the id the package already
uses. No second window declaration format and no new payload format was introduced.

## What this adds

- `surface::DeclaredFrame { id, line }`, one per window, and `Window::frame`.
- `surface::Surface::parse` refuses a second binding for one window and a frame id bound by two
  windows, both `cellframe-duplicate` with the line of the offending row.
- `cellframe::resolve(&Surface, &[&str]) -> Vec<Bound>`: every frame document is read through
  `CellFrame::parse` first, so a malformed frame is refused by the frame grammar rather than by its
  reference; then a reference the package does not declare is refused `cellframe-reference` with the
  line of the `frame` row, and a declared frame bound by no declaration is refused
  `cellframe-unreferenced`.
- `cellframe::load(frame, surface, documents, sequence, previous, at)`: the ordinary mount, then the
  resolution, so a package whose reference does not resolve does not load at all.

Bound windows are answered in declaration order, and the order the package lists its frame documents
in does not change which window gets which frame.

## What this does not do

- `shploingus.cellframe/1` is unchanged in meaning: no key, row or bound was added, reordered or
  relaxed by this unit.
- No renderer draws a frame, no XML is lowered, and no role is resolved to a colour.
- A window with no binding is still a valid window; only a *frame* no declaration binds is refused.

## Acceptance test

`cargo test -p planet-package` exits 0, including 12 scenarios in `tests/binding.rs`: a declaration
with no binding still loads; two frames with distinguishable draw rows each resolve to the declaration
that names them, in either document order; an undeclared id is refused with its line; an unreferenced
frame is refused; a frame bound twice and a window naming two frames are both refused with lines; a
frame row before or under the wrong window is refused; a package loads and a package with a dangling
reference does not; a malformed frame is refused by the frame grammar first.

`python tools/port_gate.py selftest` exits 0 (123 scenarios) and `python tools/port_gate.py check`
exits 0 (21 checks, 4 not enforceable), with the 22 surface and 13 vocabulary scenarios unchanged.

## Closing

- Commit: this unit's commit on `ticket/TICKET-0045-frame-binding` (unmerged; no remote configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0051
