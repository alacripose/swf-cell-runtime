# TICKET-0023 — Clear a leftover selftest workspace, or stop with a named error

- Owner: implementation agent
- Block: H1 (the gate's own honesty)
- Branch: `ticket/TICKET-0023-workspace-recovery`
- Ledger: LEDGER-0025
- Depends on: TICKET-0019..TICKET-0022

## Defect

`command_selftest` did `remove_tree(work)` then `work.mkdir(parents=True)`. `remove_tree` retries a
read-only failure but swallows a failure it cannot fix, so an interrupted run that left
`target/port-gate-selftest` behind produced an uncaught `FileExistsError: [WinError 183]` traceback,
exit status 1, no record and no diagnostic. Observed during TICKET-0022 verification:

```
File "tools/port_gate.py", line 2328, in command_selftest
    work.mkdir(parents=True)
FileExistsError: [WinError 183] Cannot create a file when that file already exists:
'...\target\port-gate-selftest'
```

The gate's contract says exit 2 means "the gate could not run", with a `port_gate:` message. A
traceback is neither.

## Repair

`prepare_workspace(work)` clears the directory with one retry; if it still exists it raises
`GateError` naming the path and stating that no record was written and no checks ran; a `mkdir`
failure is likewise converted. `command_selftest` calls it. No check, rule, scenario, record field
or schema is touched, and the scenario count stays 115.

## Reproduction

- Stale workspace holding a leftover file, no gate process: `exit=0`,
  `RESULT: PASS (115 of 115 scenarios)`, record written.
- Workspace with a file a live process holds open: `exit=2`,
  `port_gate: selftest workspace could not be cleared: <path>; another run may still hold a file
  inside it. No record was written and no checks were run`.

## Acceptance test

- Both reproductions above, run and quoted.
- `selftest` 115 of 115 twice, byte-identical record; `check` PASS at 19 checks, `NOT-ENFORCED (4)`.
- The record identity changes only because `tools/` is a fixture input; no other committed byte is
  altered by the repair.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0025
