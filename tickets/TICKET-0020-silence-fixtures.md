# TICKET-0020 — Prove each check stays quiet when a different rule is broken

- Owner: implementation agent
- Block: H1 (the gate's own honesty), touching PATCH-0001..0006
- Branch: `ticket/TICKET-0020-silence-fixtures`
- Ledger: LEDGER-0022
- Depends on: TICKET-0019

## Problem

`SWEEP` proves every enforced check can refuse a violation of its own rule. It cannot prove a check
refuses *only* that rule: a check that also fires on a neighbouring rule's damage passes its sweep
entry, and `valid_fixture` only proves it is quiet on a wholly clean tree. The gap is over-broad or
always-refusing logic, which turns a real defect into a gate that fails legitimate work.

## What this adds

A `SILENCE` table: for each enforced check, a mutation that breaks a *different* check's rule. Two
conditions must hold on that fixture:

1. The check under test reports nothing.
2. At least one other check does fail, so the fixture is genuinely broken and the silence is evidence
   rather than an empty tree.

Each check is paired with a neighbour of a different kind where possible — the frame-magic scan is
paired with a second-reader file whose magic is allowed, the single-reader check with an MCP schema,
the record check with a code file, and the document checks with each other's documents.
`sweep_covers_every_check` now requires both a violation and a silence pair for every check.

## Acceptance test

- `selftest` → 77 of 77, including 19 `silent_<check>` entries.
- Making one check over-broad (also firing on its neighbour's damage) fails `silent_<check>` while
  the sweep entry still passes — proving the two halves catch different drift.
- `check` remains PASS at 19 enforced and `NOT-ENFORCED (4)`.

## What this deliberately does not do

- It does not claim a check inspects the *right* thing; it claims it does not inspect someone
  else's.
- It adds no rule and loosens none; no check is demoted.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0022
