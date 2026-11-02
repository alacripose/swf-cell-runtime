# TICKET-0013 — Propose the streamed transport's framing, ordering, receipt and exactly-once rule (D-4)

- Owner: implementation agent (proposal and gate), format owner (ruling), project owner (T-6)
- Block: H10 preparation
- Branch: `ticket/TICKET-0013-transport`
- Ledger: LEDGER-0015
- Depends on: TICKET-0007 (the D-4 ruling)

## Scope (source-safe)

D-4 ruled that a streamed transport is a new layer, not "mount-v2", and listed what it does not settle.
This writes `docs/CELL-TRANSPORT-DECISION.md` as a proposal covering all eight questions that leaves
open: framing, ordering and resume, receipt identity, exactly-once proof, the unit of delivery, whether
the base game travels this way, failure and tampering behaviour, and who admits plus what bounds the
transport.

Two pressure points are recorded as owner choices rather than resolved: a self-describing chunk against
D-4's no-second-family boundary, and a transport bound that must not become a content limit. The
document also agrees with the measured statement in `docs/PACKAGE-OWNERSHIP.md` that the receipt is the
door admission plus the ledger record.

## Acceptance test

- `docs/CELL-TRANSPORT-DECISION.md` has `T-1`…`T-8`, each with Recommendation, Rejected, Falsifier and
  Owner; T-6 offers no recommendation and prices both options.
- `config/cell-stream.json` holds the ids, labels and token list; `cell_stream` in the gate enforces
  them.
- `python tools/port_gate.py check` → PASS, 14 checks.
- `python tools/port_gate.py selftest` → PASS, 24/24, including `cell_stream_owner_removed_refused`,
  `cell_stream_wire_token_refused` and `cell_stream_question_missing_refused`, with every earlier
  refusal still firing unchanged.
- The governance and fixture lists are shown before and after the edit, with all four existing file
  entries still present.

## What this deliberately does not do

- It does not rule any question; each keeps its owner.
- It does not define chunk headers, revision-number shapes, or a receipt type.
- It does not claim a transport, a queue, or a store exists.
- It does not read, name, or describe the source movie or any private shell material.

## Incomplete state (declared, not hidden)

No transport exists. `SCOPE-ACCOUNTING.md` keeps the streamed transport at `not-implemented` and
`cell_stream` blocked on a client crate.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0015
