# TICKET-0019 — Prove every enforced check can refuse, and only for its own reason

- Owner: implementation agent
- Block: H1 (the gate's own honesty), touching PATCH-0001..0006
- Branch: `ticket/TICKET-0019-mutation-sweep`
- Ledger: LEDGER-0021
- Depends on: TICKET-0015 (the record), TICKET-0017, TICKET-0018

## Problem

`selftest` asserted `expected_failure in failed`, so a scenario passed when *any* check failed, and
nothing proved the named check was the one that refused. Three enforced checks —
`governance_files`, `format_magics`, `scope_accounting` — had no violating fixture at all, so they
passed without ever being shown to refuse. A check that cannot fail is an assertion, not a gate.

## What this adds

A `SWEEP` table with one violating mutation per enforced check, each paired with the exact wording
that check alone is expected to emit. Every entry is run twice on its own throwaway fixture:

1. **Enabled** — the expected wording must appear among *that check's* problems, and must not appear
   in any other check's problems. A message from a neighbouring check does not count.
2. **Disabled** — the same checks run with the check under test skipped. The expected wording must
   then be absent, so the refusal is attributable to the check and not to the mutation alone.

A `sweep_covers_every_check` entry fails the suite when a check exists with no sweep entry, so a
future check cannot be added without a violation that proves it.

## Acceptance test

- `selftest` reports 58 of 58, including 19 `sweep_<check>` entries and `sweep_covers_every_check`.
- Disabling any single check in `SWEEP` makes that sweep entry fail (proved for one check by hand).
- `check` remains PASS at 19 enforced and `NOT-ENFORCED (4)` with the four blockers intact.

## What this deliberately does not do

- It does not demote `format_magics`, which inspects zero code files on this tree but reports that in
  its own guard note and refuses a real second magic once code exists.
- It does not claim a fixture proves the client mounts anything; the sweep is about the gate.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0021
