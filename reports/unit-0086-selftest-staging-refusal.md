# Unit 0086 — the selftest that refused in 203 bytes, and the wait that could not see it

Status: `refused` (the poll), `present` (the named refusal and the rule).

## What happened

`python tools/port_gate.py selftest` was started while an earlier `selftest` run still held
`target/port-gate-selftest`. The second run refused immediately:

```text
selftest workspace could not be cleared: …/target/port-gate-selftest; another run may still hold a
file inside it. No record was written and no checks were run
```

That message is 203 bytes, reaches stdout as a `GateError` printed by `main` on **stderr**, and prints
no `RESULT:` line and no record. The gate's record then stayed at the previous tree's digest, so
`check` reported:

```text
FAIL selftest_record  the record describes fixture 'sha256:9f2162…', but this tree is
'sha256:74cf2b…'
```

The agent's next step was `for …; do grep -qE "^RESULT:" generated/selftest.log; sleep 5; done`: a
loop waiting for a marker that the refused run could never print. It consumed the bound, the turn,
and a tool interruption, and the log had already said why.

## The two refusals

**The tool's.** A command that refuses must refuse inside its own output contract. A `GateError`
printed to stderr with no `RESULT:` line is `absent` to every waiter — a script, a hook, an agent —
and `absent` is indistinguishable from `pending` unless someone reads the body.

**The agent's.** A marker is not a fence to wait on; it is evidence to read. Polling a log is not
waiting for work, and a loop that cannot print why it is still waiting is a loop that has stopped
observing.

## Landed

- `tools/port_gate.py`: `prepare_workspace` waits a bounded 30 s for the staging directory to be
  released, printing each attempt, then refuses `selftest-staging-in-use` (or
  `selftest-staging-unwritable` when the directory cannot be created).
- `command_selftest` catches that refusal and prints `REFUSED selftest_did_not_run: …` followed by
  `RESULT: REFUSED (selftest did not run; 0 scenarios)`, exiting non-zero.
- Refusal, record and result lines are flushed, so a redirected log shows them when they happen.
- `AGENTS.md` §15 states the rule: one waitable run, a bounded wait, markers matched with `\r`
  stripped, and no second run of a lock-holding tool.

## Not claimed

The window was not run and no client witness is claimed here: this unit is about the gate's refusal
surface and the agent's wait discipline, not about the movie drawing.
