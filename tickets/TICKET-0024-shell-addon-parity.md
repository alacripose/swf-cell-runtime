# TICKET-0024 — Record the shell/addon ruling and guard it with four proofs

- Owner: implementation agent
- Block: H1 (the gate's own honesty); records a decision that reaches H4, H6 and D-4
- Branch: `ticket/TICKET-0024-shell-addon-parity`
- Ledger: LEDGER-0026
- Depends on: TICKET-0019..TICKET-0023

## What the owner said

"the shell is inferred from the other projects, delivered as a streamed packaged addon that parity
checks itself"

This is a decision about the shell's provenance, delivery and verification. It is recorded, not
implemented: the shell is shell-owned, `inputs/` is unreadable to agents, and no addon, parity check or
shell exists in this tree.

## What this unit adds

- `docs/DECISIONS-PORT.md` **D-5**, recording the statement verbatim, what it settles and what it does
  not.
- `docs/CELL-ADDON-DECISION.md`: three obligations (C-1 shell behaviour is inferred from the other
  projects; C-2 delivery is a streamed packaged addon carrying canonical package bytes; C-3 the addon
  parity checks itself before any consumer sees it) and four open questions (A-1 what evidence counts,
  A-2 whether an addon is a separate artifact, A-3 what parity compares and where its result lives,
  A-4 who may declare parity satisfied), each with a recommendation, a rejected alternative, a
  falsifier and an owner. Nothing is marked decided beyond D-5's statement.
- `config/cell-addon.json` guarding it, and the `addon_parity` check: the statement stays verbatim,
  each commitment stays stated, every question keeps its four parts, the document still says it is a
  proposal, and it never claims the shell or its parity as done.
- A `SCOPE-ACCOUNTING.md` row reading `not-implemented` for the whole item.
- Four proofs for the new check, because coverage demands them: a violation (statement reworded), a
  silence pair (a governance file removed, `addon_parity` silent), a decoy (the mutation's own phrase
  present in prose while the statement stands) and a near-miss (the statement's first letter
  capitalised, which only an exact comparison refuses).

## Acceptance test

- `selftest` 119 of 119, with `sweep_addon_parity`, `silent_addon_parity`, `decoy_addon_parity` and
  `near_addon_parity` all passing and coverage reporting 20 of 20 checks with four proofs each.
- `check` PASS at 20 checks, `NOT-ENFORCED (4)` with the same four rows and blockers.
- The record stays deterministic and free of timestamps and absolute paths.

## What this deliberately does not do

- It does not infer shell behaviour, read the source boundary, invent a parity format, or claim parity.
- It does not demote a check or loosen a rule; the four `NOT-ENFORCED` rows are untouched.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0026
