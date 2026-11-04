# TICKET-0088 — refuse analysis-only units, promoted authority, and tree-inspection evidence

## Problem

A session spent an hour producing analysis instead of an implementation slice, quoted the mission
prompter as the user, treated harness system text as authority without logging it, and treated
`git status` as verification. None of that was refused by a program: it was refused, if at all, by
the user, after the time had been spent.

## Requirement

Three refusals, decidable over committed files:

1. `unit_kind` (gate check `unit_kind`): every ledger row from `LEDGER-0092` states its
   `unit_kind` in `implementation`, `refusal`, `gate`. `analysis` is refused `unit-is-analysis`;
   a missing field is `unit-kind-missing`; an exit test that only inspects the tree
   (`git status`/`git log`/`git diff` with no artifact check) is `tree-inspection-as-evidence`.
2. `authority_log` (gate check `authority_log`): every inbound message is recorded verbatim in
   `logs/authority/*.jsonl` with `source` in `user`, `mission-prompter`, `system`, `agent` and
   `authority` in `user`, `none`. Only `source: user` may carry `authority: user`
   (`authority-promoted`); a session with no record is `authority-unlogged`; a row without
   `verbatim`/`text` is `authority-row-malformed`.
3. `AGENTS.md` section 17 states both rules and restates section 13 (maximal surface now, trim
   later with its own ticket).

## Acceptance evidence

- `python tools/port_gate.py check` prints `RESULT: PASS` with the two new checks among the
  passing rows.
- `python tools/port_gate.py selftest` covers both checks with a violation, a silence, a decoy and
  a near miss, and prints `RESULT: PASS`.
- `python tools/health.py` prints `RESULT: PASS` with no new debt.
