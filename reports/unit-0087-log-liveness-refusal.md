# Unit 0087 — a log may only be polled with proof that it is streaming

Status: `refused` (the blind poll), `present` (the helper and the conventions).

## What happened

A `selftest` was redirected to `generated/selftest.log` and waited on with a loop that only asked
whether a `RESULT:` line had appeared. The loop had no liveness check, so it could not distinguish:

- a run that is working and has not reached its result yet,
- a run that refused before streaming (the 203-byte staging-lock refusal, unit 0086),
- a run whose output never reached the file at all.

All three look identical to a marker poll. The rule that follows is not "never poll"; it is that a
secondary log may be waited on **only by a command that also proves the log is streaming**.

## The convention

- Wait on the run's **exit status**, written by the same shell command (`echo $? > <exit>`).
- Report the log's **byte count and last streamed line** on every report: growth is evidence, and a
  size that never moves is a refusal, not patience.
- Refuse `log-not-streaming` when the log stays at zero bytes past its deadline; refuse
  `wait-expired` at the bound; print a `RESULT:` line on every path.
- Batch independent work into the same response so a wait does not own a whole turn.

`tools/wait_run.py` implements this and is the only sanctioned way to wait on a redirected run's log.
It also states the exit code it read, so a `0`/`1` is never inferred from the absence of a message.

## Not claimed

No client witness and no code-lane claim here. This unit is about how evidence is waited on; the
`selftest` record still has to be regenerated on the tree that carries the code lane and script
driver before `check` can pass.
