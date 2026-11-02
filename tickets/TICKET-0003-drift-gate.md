# TICKET-0003 — H3: drift gate and repository workflow

- Owner: implementation agent
- Block: H3
- Branch: `ticket/TICKET-0003-port-gate`
- Ledger: LEDGER-0003
- Depends on: TICKET-0001, TICKET-0002

## Scope (source-safe)

`tools/port_gate.py`, `tools/pre-commit`, and `.github/workflows/port-gate.yml`: the
mechanical drift gate for the baseline patches, plus a self-test that proves each refusal.

## Acceptance test

- `python tools/port_gate.py check` exits 0 on this repository.
- `python tools/port_gate.py selftest` exits 0 and demonstrates a refusal for: a tracked
  source movie, an output path escaping the project root, a missing baseline patch, an
  invalid ledger entry, a commit with no ticket reference, and a block missing its exit
  test.
- Checks the tool cannot decide are printed as `NOT-ENFORCED` with owner and blocking
  dependency, never as passes.

## Exit test evidence

`artifacts/redacted-test-results/port-gate-selftest.json` (scenario list with pass/fail and
the exit status of the run), recorded in `LEDGER-0003`.

## Shell-owned gating and refusal conditions

The gate refuses repository states. It does not refuse source content, judge
acceptability, or assign `held` / `absent` / `refused` to any feature.

## Incomplete state (must be declared, not hidden)

- Duplicate-reader and alternate-surface detection over a real code tree: the code tree
  does not exist yet, so the check is a guard scanning zero files. Owner: implementation
  agent, blocked on H2.
- MCP prohibited-operation scan: blocked on H6.
- Remote/PR review verification: blocked on a configured remote.

## Excluded from this ticket

Any package, cell, runtime, or client code. Any enforcement the gate claims but does not
perform.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0003
