# TICKET-0021 — Prove each check keys on its rule, not on a mutation's symptom

- Owner: implementation agent
- Block: H1 (the gate's own honesty), touching PATCH-0001..0006
- Branch: `ticket/TICKET-0021-decoy-fixtures`
- Ledger: LEDGER-0023
- Depends on: TICKET-0019, TICKET-0020

## Problem

The violation sweep proves a check refuses its own rule's damage. The silence pair proves it does not
refuse a neighbour's. Neither can see a check keyed on the *symptom* a mutation leaves behind: a check
that rejects any document containing `done`, `**Decided by:**`, `## Bounds`, `SHPKG002` or
`surface.capture` passes both, because the symptom and the violation always arrive together.

## What this adds

A `DECOY` table: for every enforced check, a fixture that carries that check's mutation symptom on a
tree where no rule is broken. Examples — a prose sentence saying the work "already done" while every
H1..H10 row keeps a real status; a `**Note:**` label beside every untouched `**Falsifier:**`; an
appendix heading `## Bounds` with no M-number while M-4 stands; a `PATCH-0004.md.orig` backup beside
six intact patches; an untracked `docs/footage.swf`; a `SHPKG002` line in a document the magic scan
does not read; a `surface.capture` sentence outside `mcp/`; a `V4PKG001` note outside `src/` and
`crates/`; a sibling JSON file with a timestamp beside an untouched record; an extra ledger field
holding a fabricated digest.

Each decoy must leave the **whole** tree clean, so any refusal is symptom-keying and nothing else. A
decoy that broke a rule would prove nothing, so the entry fails if any check fires, not just the one
under test. Every enforced check has a decoy; none needed an exemption, and the closest calls are
recorded here rather than smoothed over: the record check's symptom is a sibling file, not the record,
because a timestamp inside the record *is* the violation; the ledger checks' symptom sits in a field
the check does not read, because a fabricated digest in `unit_commit` is the violation.

`sweep_covers_every_check` now demands all three proofs per check, so a check added later cannot ship
with two.

## Acceptance test

- `selftest` → 96 of 96: 20 violation entries, 19 silence entries, 19 decoy entries, coverage.
- A check made symptom-keyed fails only its `decoy_<check>` entry, while `valid_fixture`, its
  `sweep_<check>` entry and its `silent_<check>` entry all still pass.
- `check` remains PASS at 19 enforced and `NOT-ENFORCED (4)`.

## What this deliberately does not do

- It does not prove a check inspects the *right* artefact, only that it does not key on a token.
- It adds no rule and loosens none; no check is demoted.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0023
