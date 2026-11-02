# TICKET-0009 — H9 prep: scope the family-A cell model (D-1), schema left open

- Owner: implementation agent (scope), format owner (schema), project owner (surface choice)
- Block: H9 (preparation); satisfies the model half of D-1
- Branch: `ticket/TICKET-0009-cell-model`
- Ledger: LEDGER-0011
- Depends on: TICKET-0007 (the D-1 ruling)

## Scope (source-safe)

Turn ruling D-1 into a scoped model: what a cell is, what it must be able to do, what it must refuse,
which invariants hold, and which questions stay open with their owner named. No wire schema, no `Kind`,
no parser, no client. No source movie is read, named, or described.

## Acceptance test

- `docs/CELL-MODEL.md` states the model, the required capabilities (from the plan's own `Cell frames`
  row), six named refusals, six invariants, and an explicit open-question table with owners.
- The document contains no field layout, encoding, kind number, ceiling value, or op code: searching it
  for a schema decision should find only the statement that the decision is open.
- `docs/SCOPE-ACCOUNTING.md` still reads `not-implemented` for the cell model, pointing at this doc.
- `python tools/port_gate.py check` passes (10 checks).

## What this deliberately does not do

- It does not choose between one `Kind` and several.
- It does not name cells, bindings, shapers or projections in wire terms.
- It does not claim any cell is drawn, mounted, or parsed anywhere.
- It does not adopt family B's raster, `shploingus.cells/1`, or `shploingus.cellframe/1`.

## Incomplete state (declared, not hidden)

Every row of §5 in `docs/CELL-MODEL.md` is open. The format owner's ticket does not exist yet. H9's
first surface (whether it uses cells at all) is the project owner's call.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0011
