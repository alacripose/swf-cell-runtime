# TICKET-0031 — A mount lands one durable ledger entry

- Owner: implementation agent
- Block: H2/H6 boundary (the record a mount produces)
- Branch: `ticket/TICKET-0031-ledger-mount-entry`
- Ledger: LEDGER-0036
- Depends on: TICKET-0027 (the ledger's stage/sync/rename path), TICKET-0030 (the strict grammar)

## What was wrong

`mount` returned a receipt and nothing else. `secure-shell/docs/LEDGER.md` is explicit that a change is
landed when its entry is appended **and fsynced**, and its register rule is that a defect or a mount
that is not an entry did not happen. A receipt held in a return value is not a receipt.

## What this builds

`crates/planet-package/src/record.rs`:

- `mount_and_record(frame, surface, sequence, previous, at, ledger, path, expected_seq)` — mounts
  through the strict grammar, then appends a ledger `mount` entry and lands it through the ledger's own
  `landing::land` path (staging file beside the destination, `sync_all`, rename). No side file, no
  second writer.
- Entry shape: `kind` = `mount`; `subject` = `mount/<surface>`; `digest` = the frame's whole-frame
  digest; `operation_id` = `mount-<frame digest>`; `authority_ref` = `TICKET-0031`; `evidence` =
  `{ command: "planet-package mount", exit_status: 0, checks: windows, surface: "planet-package",
  digest: frame digest }`.
- `Recorded::{Landed{receipt, seq}, AlreadyRecorded{receipt, seq}}`.
- `MountRefusal::{Package, Ledger, LedgerUnavailable}` with `code()` in the crate's existing style:
  package codes unchanged (`surface-row`, `frame-kind-reserved`, …), ledger codes in the family's
  vocabulary (`ledger-stale`, `ledger-corrupt`, `ledger-invalid`, `ledger-unavailable`).

## The duplicate rule, and the reading it comes from

**Idempotent, keyed on the frame digest.** `LEDGER.md` §3 step 3 says an operation id already in the
chain "returns the original entry (`already-committed`); a different command is
`ledger-duplicate-operation`", and `AGENTS.md` §9 requires retries to be idempotent. Reading a re-mount
as a retry of the same operation is the narrowest rule the text supports: the same bytes mounting twice
answer `AlreadyRecorded` with the first sequence and write nothing, while a different frame is a
different operation and lands its own entry. The alternative — refusing a duplicate outright — would
make a retry after a timeout an error, which contradicts the same section.

## What this does not do

- No vocabulary tables, no client crate, no executor (the scope guard).
- The entry records that a surface mounted and what its bytes were. It does not prove the surface was
  rendered, and it does not sign anything: the ledger's checkpoint is `dl-enclave.sig/1` and is not in
  this unit.

## Acceptance test

`cargo test -p planet-package` — the 37 existing tests plus: a mount lands one entry a reload reads
back and verifies; the same frame twice records once; a refused surface (unknown row) writes nothing and
leaves the chain at zero; a `code` frame writes nothing; two distinct frames land two entries; a stale
`expected_seq` is refused `ledger-stale` with the file byte-identical; an unwritable destination lands
nothing and does not advance the chain; the ledger code mapping.

`python tools/port_gate.py selftest` then `check` green, with the four `absent` rows still named.

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0036
