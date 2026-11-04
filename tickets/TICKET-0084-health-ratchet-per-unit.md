# TICKET-0084 — a health ratchet per work unit, measuring the turn and proposing rules

Status: `present` (`tools/health.py`, `artifacts/health/state.json`, per-unit reports, proposals).

## Problem

Health was checked by running `port_gate.py check` and reading it by eye. Nothing recorded which
checks had held before, so a regression looked exactly like a first failure; nothing measured what a
turn did; and the lessons a session learned about its own process lived only in the conversation and
died with it.

## Change

`tools/health.py`:

- **Runs the gate** and reads its `RESULT:` marker, failing checks, held checks and not-enforced rows.
- **Ratchets.** `artifacts/health/state.json` records every check that has ever been held. A held
  check that fails is a regression (exit 2). A never-held check that fails is a debt with an owner and
  the unit that opened it (does not fail the run, never reported as held). `selftest_record` is the
  declared evidence debt: any edit stales it, and one complete `selftest` run repairs it — a rewritten
  digest does not.
- **Reports per unit and per day** to `reports/health/<date>-<ledger>.md` and `.json`: gate marker,
  failing, held, regressions, open debts, not-enforced rows and the turn's measurements.
- **Measures the turn** across the projects it observes: files changed, insertions and deletions, the
  observed development span from the changed files' timestamps, test functions touched, ledger rows
  added.
- **Proposes rules** from those measurements into `artifacts/health/proposals.jsonl` (deduplicated per
  unit): over-budget commit, behaviour change with no test, unit with no ledger row, open evidence
  debt. Each proposal carries the evidence that produced it and the report carries the block to paste
  into `AGENTS.md`; a proposal is not a rule until a ticket lands it.
- **Observes only what the user named**: this checkout, plus paths in `config/health-projects.json`.
  Nothing outside the repository is read unless the user listed it.
- **Writes state atomically** (temporary file beside the final one, then `os.replace`), and prints a
  `RESULT:` line on every path, including `health-state-unreadable`.

## Refused

- Counting a debt as a pass. A check that has never held is named with an owner, and the run's result
  says how many are open.
- Clearing the ratchet on a green run. The held set only grows.
- Inverting a report into a claim about the game: a gate PASS is not a client witness (`AGENTS.md` §7).
- Reading another checkout by inference. Only configured paths are observed.

## Exit test

- `python tools/health.py` prints `RESULT: PASS` or `RESULT: FAIL (ratchet: …)`, writes
  `reports/health/<date>-<ledger>.md` and `.json`, and updates `artifacts/health/state.json`.
- A second run with a check removed from the held set cannot clear it: the held list only grows.
- With a stale selftest record the run reports one open evidence debt and the report names it.
- `python tools/wait_run.py --log generated/selftest.log --exit generated/selftest.exit` printed
  `RESULT: PASS (exit 0)` for the run that regenerated the record (`RESULT: PASS (128 of 128
  scenarios)`).
