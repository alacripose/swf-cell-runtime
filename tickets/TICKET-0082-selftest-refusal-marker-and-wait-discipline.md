# TICKET-0082 — a refusal must print its own marker; a wait must not be a `grep` loop

Status: `present` (named refusal, bounded lock wait, flushed markers, `AGENTS.md` §15).

## Problem

`tools/port_gate.py selftest` refused because an earlier run still held
`target/port-gate-selftest`, and printed that refusal only as a `GateError` on stderr: no `RESULT:`
line, no record, `0` checks. A waiter looping on `grep -E "^RESULT:"` therefore waited for a marker
that could not appear. The refusal was correct; its **output contract** was not, and the poll that
followed it was not a wait.

Reproduction, without needing a second process: hold the staging directory (or leave it behind) and
run `selftest`; before this ticket the log ends with the stderr message and no marker.

## Change

- `prepare_workspace` retries removal a bounded 30 s, printing each attempt, then refuses by code:
  `selftest-staging-in-use` (a live holder or an interrupted run's leftover) or
  `selftest-staging-unwritable` (the directory cannot be created).
- `command_selftest` catches that refusal and prints `REFUSED selftest_did_not_run: …` and
  `RESULT: REFUSED (selftest did not run; 0 scenarios)`, returning non-zero. A refused run is
  therefore matchable, and `0 scenarios` is stated rather than implied.
- Refusal, record and `RESULT:` lines are flushed, so a redirected log carries them when they happen
  rather than when the buffer drains.
- `AGENTS.md` §15 states the rule this refusal taught: one waitable command with a bounded timeout; a
  missing marker is `absent`, not `pending`; markers matched with `\r` stripped; never a second run of
  a lock-holding tool; a waiter states its bound and stops when it expires.

## Refused

- A silent retry of the second run. The lock is the only thing preventing two runs from corrupting the
  fixture tree; the answer is to name the refusal, not to wait it out forever.
- Reporting the stale-record `FAIL` as a code failure. It is `absent`: the record describes a tree the
  selftest never certified, and the repair is one complete run, not a rewritten digest.
- Treating a `grep` loop's expiry as evidence either way. `absent` is not a verdict.

## Exit test

- Hold the staging directory (a second `selftest` in another shell, or a leftover directory) and run
  `python tools/port_gate.py selftest`: within the bound it prints
  `RESULT: REFUSED (selftest did not run; 0 scenarios)` and exits non-zero.
- `python tools/port_gate.py selftest` on a free tree runs to completion and prints
  `RESULT: PASS (… of … scenarios)`, writing the record.
- `python tools/port_gate.py check` prints `RESULT: PASS (22 checks, 5 not enforceable)`.
