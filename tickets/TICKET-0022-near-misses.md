# TICKET-0022 — Prove each check compares its rule, not something near it

- Owner: implementation agent
- Block: H1 (the gate's own honesty), touching PATCH-0001..0006
- Branch: `ticket/TICKET-0022-near-misses`
- Ledger: LEDGER-0024
- Depends on: TICKET-0019, TICKET-0020, TICKET-0021

## Problem

Violation, silence and decoy prove a check refuses its rule, ignores a neighbour's and ignores a
symptom. A check that reads the **right file with the wrong comparison** passes all three: a count of
"at least six", a directory accepted for a file, a case-insensitive suffix, a string-prefix root
check, a loosened status token, a three-digit ticket accepted as four, a lowercased field label, a
moved citation found anywhere in the document, an abbreviated commit compared by prefix.

## What this adds

A `NEAR_MISS` table with the smallest genuine violation of each check's rule, shaped so exactly that
loosening would accept it — and refused by the real check alone. Examples: a seventh baseline beside
six (a count of ">= 6" accepts); `tickets/TEMPLATE.md` replaced by a directory (an `exists()` test
accepts); `docs/FOOTAGE.SWF` force-added (an exact-case suffix test accepts); `../<root>-evil/` (a
string-prefix root test accepts); `V4PKG002` (a `V4PKG` prefix test accepts); status `"present "`
with a trailing space; `TICKET-001`; `exit test:` lowercased; `present (unverified)`; a citation
removed from its erratum (found elsewhere in the document by a global search); `I-2` absent from the
schema while the model states it; `**Owner** :`; `#### M-4`; an altered proposal marker; a sibling
source column on Q-3; a miscounted `fixture.inputs`; an abbreviated `unit_commit`; `schema.describe.all`
(a permitted prefix); and a parser in `crates/planet-package-evil/` (a `startswith("crates/planet-package")`
test accepts).

Every entry is a real violation, so it must be refused; an entry the check cannot see is a finding,
not something to weaken away. All **19 of 19** enforced checks have a near-miss; **0 exemptions**
were needed, so no check is excused from this half. `sweep_covers_every_check` now requires all four
proofs per check.

## Acceptance test

- `selftest` → 115 of 115: 20 violations, 19 silences, 19 decoys, 19 near misses, coverage.
- A check made wrong-comparison (status accepted by prefix) fails only its `near_<check>` entry, while
  the clean fixture, its violation, silence and decoy entries all pass.
- `check` remains PASS at 19 enforced and `NOT-ENFORCED (4)`.

## What this deliberately does not do

- It does not prove the check reads the artefact a *human* would choose, only the one its rule names.
- It adds no rule and loosens none; no check is demoted.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0024
