# TICKET-0043 — The v2 addendum is bound, and its precondition read

- Owner: project owner (the added document), implementation agent (the reads and the record)
- Block: H1 declarations
- Branch: `ticket/TICKET-0043-v2-addendum`
- Ledger: LEDGER-0049
- Depends on: `docs/cellular_generativity_axis_civilian_expectations_prototype_rechecked.md`
  section 13, TICKET-0042, and the owning cell readers named below

## What was wrong

The added document states a precondition this port had not met. CGA-21 accepts a cellframe and a cell
set only when they use their *owning* versioned readers, and its refusal condition names a "guessed
grammar". TICKET-0042 recorded the frame grammar from a sample frame in a neighbouring tree —
`MAP-0001.frame` — which is a document, not the reader, so the grammar was inferred from one instance
and the closure rules were unknown. Section 13.10 lists the actual owning schemas, readers and tests
as reads that precede implementation.

## What this reads

- `shploingus-clean/crates/voxel-package/src/cellframe.rs` (1709 lines) — the reader for
  `shploingus.cellframe/1`: header, `frame<TAB>id`, the required keys, the optional `feature` key, the
  draw rows, and the optional-together sampling rows; the format's own bounds; and the refusal
  messages for an unknown key, a duplicate key, a missing key, a row past the grid, and a frame that
  draws nothing.
- `shploingus-clean/crates/voxel-package/src/cells.rs` (371 lines) — the reader for
  `shploingus.cells/1`: a header and one `cells` line selecting a drawing; an older declaration header
  refused at its file and line rather than read under the newer rules; a declaration that names no
  drawing refused; and a package with no such file at all as an absence the caller states.

## What this records

Two answers stop being the proposal's and become the format's, in `docs/CELL-SCHEMA.md`:

- **Q-1** — several row kinds inside one frame: typed keys, field bindings, typed draw rows. The
  reader's key table and row match are the evidence, not a sample file.
- **Q-5** — a binding is a declared field name plus a qualified law token. The reader refuses a field
  name it does not sample, refuses a second source for one field, and refuses prose where a law token
  belongs. No path, no expression, no arithmetic, and therefore no second interpreter to budget.

## What this changes, and what it does not

Changes: the schema document now cites the reader instead of a sample frame, and carries the reader's
bounds and refusal set. Not changed: no frame grammar is implemented, no XML is lowered, and the
sampling rows are recorded as the reader's grammar rather than as this port's.

Names two gaps instead of closing them:

1. The numeric containment bounds the expectations doc asks of a declarative surface (bytes, node
   count, depth, text length) are not wired into `crates/planet-client/src/xml.rs`. It has named
   refusals and no budgets.
2. The format id reader upgrades a legacy numeric id to the computed one, and CGA-21 refuses a
   *silent* version conversion. The upgrade must stay visible in the record it produces, not only in
   the value it returns.

Also recorded: eleven identifier families from sections 13.3 and 13.10 are integration anchors only.
They are not contracts this repository has read, and no unit may cite them as implemented.

## Acceptance test

`python tools/port_gate.py selftest` exits 0 (123 scenarios), and `python tools/port_gate.py check`
exits 0 with `cell_schema` passing: every question keeps Recommendation, Rejected, Falsifier and
Owner, the document still calls itself a proposal, no port wire token appears, and the family-B
raster is named only where it is rejected.

## Closing

- Commit: this unit's commit on `ticket/TICKET-0043-v2-addendum` (unmerged; no remote configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0049
