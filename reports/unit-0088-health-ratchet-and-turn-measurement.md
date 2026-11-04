# Unit 0088 — the ratchet, the per-unit report, and proposals that survive a session

Status: `present`.

## What it does

`python tools/health.py` is the test-turn command. It runs the gate, compares the result with the
durable ratchet, measures the turn, writes the unit's report, prints proposals, and exits on
regression.

- **Ratchet.** `artifacts/health/state.json` holds every check that has ever been held. A held check
  that fails is a regression (exit 2); a never-held check that fails is a debt with an owner, and it
  is never counted as held. `selftest_record` is the one evidence debt: any edit stales the record and
  one complete `selftest` run repairs it.
- **Per unit and per day.** `reports/health/<date>-<ledger>.md` and `.json` — the newest ledger id
  names the unit, the day names the file, so a report never loses which turn produced it.
- **Turn measurement.** Files changed, insertions and deletions, the observed development span from
  the changed files' timestamps, test functions touched, ledger rows added, per observed project.
- **Proposals.** An over-budget commit, a behaviour change with no test, a unit with no ledger row, or
  an open evidence debt becomes a proposal in `artifacts/health/proposals.jsonl` with the evidence
  that produced it, and the report carries the block to paste into `AGENTS.md`. A proposal is not a
  rule until a ticket lands it.
- **Scope.** This checkout, plus only the paths the user listed in `config/health-projects.json`.
  Another checkout is never inferred.
- **Atomic state.** Temporary file beside the final one, then `os.replace`: an interrupted turn cannot
  leave a half-written ratchet.

## Evidence from this turn

`python tools/wait_run.py --log generated/selftest.log --exit generated/selftest.exit` reported the
selftest's exit code, byte count and last streamed line; the run it waited on ended
`RESULT: PASS (128 of 128 scenarios)` with exit `0`, so the record now certifies this tree.

## Not claimed

A gate PASS is not a client witness and not a playable game (`AGENTS.md` §7). The window was not run
in this unit; the movie, the mount and the frame timing remain the shell's and the user's evidence.
