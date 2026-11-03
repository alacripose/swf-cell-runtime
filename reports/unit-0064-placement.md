# UNIT-0064 - placement parsing

Ticket `tickets/TICKET-0064-placement.md`, ledger `LEDGER-0070`, block H2, step 1 of D-7b's order.

## What changed

- `crates/swf-import/src/placement.rs`: `Placement`, `Matrix`, a bit reader that realigns between the
  matrix's scale/rotate group and its translation group, `place_object`, `place_object_2`, `survey` (over
  slices of the decompressed stream the walk recorded offsets into) and `by_depth`.
- `crates/swf-import/tests/placement.rs`: seven synthetic scenarios, including the two refusals that keep
  the reader honest — a truncated matrix is named, and an unparsed flag is refused rather than skipped.

## What it refuses

`placement-truncated` for a body ending inside a declared field; `placement-unsupported` for a flag
combination this reader does not parse and for `PlaceObject` bytes trailing its matrix. Both refusals
carry a message that says what would go wrong if the record were guessed at.

## What it does not do

No cells, no frames, no drawing, and no change to the emitted declaration. The display list is available
as data; painting it is the next step.

## Checks

`cargo test -p swf-import` (placement 7, stream 8, import 7, content 5), `cargo test --workspace`,
`port_gate.py selftest` and `check`, and a fresh-checkout identity probe — statuses recorded in
LEDGER-0070.
