# TICKET-0015 — Make the selftest record deterministic without weakening what it proves

- Owner: implementation agent
- Block: none; protects the gate's own evidence
- Branch: `ticket/TICKET-0015-selftest-record`
- Ledger: LEDGER-0017
- Depends on: TICKET-0003 (the gate and its record), TICKET-0014

## The defect

Every `selftest` run rewrote `artifacts/redacted-test-results/port-gate-selftest.json`, so verifying the
repository dirtied it. Two causes: a `timestamp` field written from the wall clock, and problem text
that quoted the fixture's throwaway commit hash, which differs on every run because the fixture is
committed with the machine's current time.

That is worse than untidy: a genuinely dirty tree and the gate's own churn look identical, so the
artifact stops being trustworthy evidence.

## The fix

- The record carries `fixture.identity`, a digest over the same inputs a run exercises, computed from
  content only. It is stable across runs on an unchanged tree and changes when any fixture input
  changes, so it still proves which state was exercised.
- Problem text replaces a fixture commit hash with `<fixture-commit>`. A throwaway commit's hash is not
  evidence about the repository; the message, the scenario and the check are.
- The record is a pure function of its own scenario results plus that identity, so a second run writes
  identical bytes.

## Acceptance test

- `python tools/port_gate.py selftest` twice in a row leaves `git status` clean and the artifact
  byte-identical.
- `selftest_record` (new) refuses a record whose identity does not describe the current tree, and one
  that cannot be rebuilt from its own results — the second is what proves no clock or random value
  survives in it.
- Self-test scenarios `selftest_record_stale_refused` (a fixture input changes) and
  `selftest_record_volatile_field_refused` (an extra field appears) both fire.
- `check` → PASS, 16 checks; `selftest` → PASS, 31/31; every earlier refusal unchanged.

## What this deliberately does not do

- It does not delete or weaken any recorded evidence: every scenario, expected failure, failing check
  and problem message is still recorded.
- It does not hide a real fixture change behind the digest; the digest is what detects one.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0017
