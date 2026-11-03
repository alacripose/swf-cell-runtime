# TICKET-0046 — A bound frame must agree with its window's content space, checked at load

- Owner: implementation agent (the check and its wiring), project owner (the rulings it rests on)
- Block: H1 declarations
- Branch: `ticket/TICKET-0046-frame-extent`
- Ledger: LEDGER-0052
- Depends on: TICKET-0045 (binding and resolution), TICKET-0044 (the frame grammar), `docs/CELL-SCHEMA.md`

## What was missing

`cellframe::check_extent` existed and nothing called it. A package could bind a 56 by 18 frame to a
42 by 18 window and load cleanly, and the disagreement would have surfaced only where a renderer
clipped or overran. That is the same deferral the binding unit removed for references: the load path
already held both the frame and the window, and did not compare them.

## No new code was needed

The `cellframe-*` family already carried `cellframe-extent` for exactly this disagreement, so this unit
reuses it and mints nothing. Two things changed inside it:

1. the refusal now carries `line`, the one-based line of the `frame` row that made the claim, so the
   message reads `cellframe line 5: frame MAP-0001 is 56 by 18 and window main has 42 by 18 ...`;
2. the predicate is **agreement** rather than fit. `check_extent` previously refused only a frame
   *larger* than its window. The end state for this unit requires a smaller frame to be refused too,
   because a frame narrower than its content space leaves declared cells undrawn — an erased region
   the format does not leave implicit. `check_extent` keeps its name, its code and its two-part
   question (does the frame name this window, and do the extents agree), and gained the line argument
   at the boundary. The ticket records the tightening so the difference is not silent.

## What this adds

- `cellframe::resolve` calls `check_extent` in the same pass that resolves the reference: a bound
  frame is resolved, then compared with the window's own `w` and `h` from the declaration, and a
  disagreement is refused `cellframe-extent` with the binding line. A mismatched binding never
  reaches a caller as a loaded frame.
- The unreferenced-frame rule still uses *referenced*, not *bound*, so an extent disagreement is
  reported as an extent disagreement and never doubled up as a frame no declaration binds.
- `cellframe::load` therefore fails on a mismatched binding with no opt-in call: the check is part of
  loading, not a second step a caller could forget.

## What this does not do

- `shploingus.cellframe/1` is byte-for-byte unchanged in meaning: no key, row, bound or refusal inside
  the frame grammar was added, reordered or relaxed. The frame still states its own extent; the
  declaration still states the window's.
- No renderer draws anything, no XML is lowered, and no role is resolved to a colour.
- A window with no binding is still a valid window, and a frame bound to a window that does not exist
  is refused by the surface grammar's own ordering rules, not by this check.

## Acceptance test

`cargo test -p planet-package` exits 0, including the binding suite's 17 scenarios: an agreeing pair
loads; a frame larger than its window and a frame smaller than its window are each refused
`cellframe-extent` naming the window id, the frame id and the binding line; a frame addressed to a
different window is refused the same way; one window's disagreement does not silence the other's
resolution or report the agreeing frame as unreferenced; and the resolving-reference and
duplicate-binding scenarios behave as they did. The 19 cellframe scenarios still pass, with the
extent predicate's smaller-frame case added at that layer.

`python tools/port_gate.py selftest` exits 0 (123 scenarios) and `python tools/port_gate.py check`
exits 0 (21 checks, 4 not enforceable), with the 22 surface and 13 vocabulary scenarios unchanged.

## Closing

- Commit: this unit's commit on `ticket/TICKET-0046-frame-extent` (unmerged; no remote configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0052
