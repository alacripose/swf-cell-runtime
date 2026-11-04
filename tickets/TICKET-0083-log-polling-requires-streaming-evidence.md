# TICKET-0083 — polling a log requires streaming evidence in the same command

Status: `present` (`tools/wait_run.py`, `AGENTS.md` §15 conventions).

## Problem

An agent waited on a redirected `selftest` log with a `grep` loop. The loop had no way to tell a run
that was working from a run that had refused before printing anything, so it burned its bound on a
log that could not produce the line it wanted. §15 already said "one waitable command"; it did not
say how a secondary log may be read at all, and the missing rule is the one that let the loop exist.

## Change

- `tools/wait_run.py`: one bounded wait on a run's **exit file**, reporting the log's byte count and
  last streamed line on every report. A log that stays at zero bytes past `--stream-deadline`
  (default 30 s) is refused `log-not-streaming`; a bound that expires is refused `wait-expired`; both
  print a `RESULT:` line and exit non-zero. A finished run prints its exit code, the byte count and
  the last line, then `RESULT: PASS|FAIL (exit n)`.
- `AGENTS.md` §15: polling a secondary log is allowed only with liveness evidence in the same
  command — wait on exit status, state bytes and last line, refuse a silent log, and batch the
  independent work instead of spending a turn on the wait alone.

## Refused

- Waiting on a marker in a log that has not been shown to stream. Zero bytes is `absent`, and
  `absent` may not be waited on.
- Treating a wait's expiry as a result. It is refused evidence, reported with the last real line.
- Reporting a run as passing from a log's absence of errors. Only the exit status and the body are
  evidence.

## Exit test

- `python tools/wait_run.py --log /dev/null --exit /nonexistent --seconds 35` refuses
  `log-not-streaming` and exits 3.
- `python tools/wait_run.py --log generated/selftest.log --exit generated/selftest.exit --seconds 300`
  prints the run's exit code, byte count, last line and a `RESULT:` line.
- `python tools/port_gate.py check` prints `RESULT: PASS (22 checks, 5 not enforceable)` once the
  selftest record certifies this tree.
