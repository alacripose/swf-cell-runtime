# TICKET-0007 — H2: record the four rulings, and the corrected cell model

- Owner: project owner (rulings), implementation agent (record), format owner (schemas)
- Block: H2 (records); H5 and later consume D-3/D-4
- Branch: `ticket/TICKET-0007-rulings`
- Ledger: LEDGER-0009
- Depends on: TICKET-0006 (the errata that left these open)

## Scope (source-safe)

Record the four decisions the errata left open, with their limits, and correct this repository's own
framing error that treated family B's 8 × 16 paper/ink grid as the port's cell model. No source movie
is read, named, or described. Nothing was built, mounted, run, or executed.

## Rulings recorded

| Id | Ruling | Settles | Leaves open |
|---|---|---|---|
| D-1 | A cell is a declared display unit over data, not a character grid — the family-B raster is that family's renderer, standing where a map or XML renderer stands elsewhere | the model | `Kind` name, field layout, shaper/projection vocabulary |
| D-2 | Media travel as per-type kinds (bitmap, font, audio), one field each | carriage shape | each kind's field contents |
| D-3 | The code kind executes a bounded deterministic cell program under a closed instruction set with budgets, limits, cancellation, capability checks and deterministic failure | executor class | op codes, encoding, capability vocabulary, budget numbers |
| D-4 | A streamed transport is added as a new layer, explicitly not "mount-v2" | that it is in scope, and that its identity is new | framing, ordering/resume, receipt form, exactly-once proof |

## The correction this ticket carries

The erratum E-1 and `docs/PACKAGE-OWNERSHIP.md` §6 described the open question as "a cell raster with
family B's semantics, or no cell raster at all". That framing imported the grid as though it were the
concept. It is superseded by D-1 and named as superseded in place; the bullet is retained so the
correction is readable, not deleted.

## Acceptance test

- `docs/DECISIONS-PORT.md` carries all four rulings, each with what it settles, what it does not, and
  the consequence that must not be crossed.
- The plan carries a **Rulings** block that supersedes only the *Status* lines calling these open; the
  errata's corrections of fact stand unchanged.
- `docs/PACKAGE-OWNERSHIP.md` §8 states which bullet D-1 supersedes.
- Every affected `docs/SCOPE-ACCOUNTING.md` row reads `not-implemented`, never `present`.
- `python tools/port_gate.py check` passes; `python tools/port_gate.py selftest` passes as before.

## Explicitly not done

No parser, no `Kind`, no op-code table, no transport framing, no client. The rulings are recorded, not
implemented. H4 remains shell-only; H5's schema document does not exist yet.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0009
