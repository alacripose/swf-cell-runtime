# TICKET-0034 — The client crate adopts a package or refuses it whole

- Owner: implementation agent
- Block: H9 runtime rows `package_owner` and `cell_mount`
- Branch: `ticket/TICKET-0034-client-cell-mount`
- Ledger: LEDGER-0040
- Depends on: TICKET-0029 (mount), TICKET-0031 (the mount entry), TICKET-0032/0033 (surface vocabulary)

## What was wrong

`docs/SCOPE-ACCOUNTING.md` records two runtime rows as "client crate needed": `package_owner` and
`cell_mount`. No client crate existed, so the mount receipt and the parsed surface could only be
reached from `planet-package`'s own tests. Nothing in the tree adopted a package.

## What this builds

`crates/planet-client`:

- `package_owner` — `decode`, `encode`, `digest` forward to `planet_package::Frame`. It keeps no
  buffer, no cache and no second reader; the frame type owns the bytes.
- `cell_mount::{CellMount, Adopted}` — `adopt(bytes, at)` decodes, compares the frame digest with what
  is already adopted, refuses the code kind, parses the surface, takes the receipt from
  `planet_package::mount`, and stores the result only after every step has passed. The receipt's
  sequence is the package's position in the adopted list. The same bytes answer the first adoption
  (same receipt, same `at`) and add nothing.

## The readings taken

- **"owns no bytes"** — the adapter stores no buffer and does not parse; it forwards and answers the
  package crate's refusal unchanged.
- **"`Mount::watch` semantics: digest dedup, adopt-or-refuse-whole"** — this unit takes exactly the
  two semantics the account spells out. A watched directory that picks the newest file by `(mtime,
  name)` needs a directory root and the stream row (`cell_stream`), which the account marks against
  the transport proposal; it is not smuggled in here.
- **Refusal codes are the package crate's own** — `frame-kind-reserved`, `unknown-lane`,
  `frame-digest`, `frame-magic`, `frame-truncated`. No code was added or renamed.
- **No ledger write in this unit.** Recording a mount is `planet_package::mount_and_record`'s job and
  belongs to the stream/admission row; this crate consumes the receipt, it does not invent a second
  writer.
- **No `winit`, no `wgpu`, no window.** The `client` and `render_wgpu` rows stay `not-implemented`;
  a renderer that draws nothing yet would be scaffolding, not a unit.

## What this does not do

- `cell_stream`, `render_wgpu`, the `winit` lifecycle, the executor, any renderer.

## Acceptance test

`crates/planet-client/tests/adopt.rs`, 8 tests driven through the public API: the receipt and parsed
surface come back (surface id, windows, sequence, state, `at`, frame digest, window lane, content,
node kind and role class); the same bytes adopt once and answer the first receipt; two distinct
packages take sequences 1 and 2; a code frame is refused `frame-kind-reserved` with nothing adopted;
an undeclared lane is refused `unknown-lane` with nothing adopted; a tampered payload is refused
`frame-digest`; a wrong magic and a truncated frame are refused `frame-magic` and `frame-truncated`;
`package_owner` round-trips the frame it does not own.

`python tools/port_gate.py selftest` then `check` green, four `absent` rows still named.

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0040
