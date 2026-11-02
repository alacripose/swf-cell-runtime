# TICKET-0025 — Record E-5: the plan's remaining mount-v2 references

- Owner: implementation agent
- Block: H2/H3 (the plan is the governing document; its errors propagate to every block)
- Branch: `ticket/TICKET-0025-plan-mount-v2-errata`
- Ledger: LEDGER-0029
- Depends on: TICKET-0008 (E-2), TICKET-0010 (D-1..D-4)

## Defect

The plan's architecture diagram says `existing mount-v2 transport path`, and four later sections
instruct implementers to use it: §2.5 "Use the existing mount-v2 stream/receipt path as the transport
and mount boundary", §5.1 "Consumes the existing mount-v2 delivery path", §6.2 "Receive the exact
package bytes through the existing mount-v2 stream path", and §12 item 5 "the mount-v2
byte/digest/receipt contract". Erratum E-2 measured that no `surface.mount`, `mount/2`, mount-v2 or
receipt type exists, and D-4 ruled the streamed transport a new layer, explicitly not mount-v2. E-2
superseded only §3.1's first sentence and the table row, so the rest of the plan still tells an
implementation to adopt a path that does not exist.

## What this unit does

Records E-5 in the plan, in the same shape as E-1..E-4: the superseded sentences are quoted, the
original wording stays visible in the body, the measurement cites the files that settle it, and the
erratum is listed in the head index. `config/plan-errata.json` gains the entry, so `plan_errata`
enforces it like the other four.

Nothing is rewritten to match the correction, and no decision is made: the transport is still a new,
undecided layer, recorded as `not-implemented` in `docs/SCOPE-ACCOUNTING.md`.

## Acceptance test

- `plan_errata` passes with five errata, each keeping its block, its quotes, its citations and the
  original wording in the body.
- `selftest` 119 of 119 (no new check, so the four proofs are unchanged); `check` PASS at 20 checks
  with `NOT-ENFORCED (4)`.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0029
