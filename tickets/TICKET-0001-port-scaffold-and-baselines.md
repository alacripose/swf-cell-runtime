# TICKET-0001 — H1: port scaffold and patch baselines

- Owner: implementation agent (scaffold), shell (gate state)
- Block: H1
- Branch: `main` (initial baseline commit; PATCH-0001 requires it before implementation)
- Ledger: LEDGER-0001
- Depends on: none

## Scope (source-safe)

Create the repository baseline: `AGENTS.md`, `.gitignore`, the six baseline patch
records, this ticket system, the issue and PR templates, the append-only ledger, the
output-root declaration, and `docs/PORT-BASELINE.md`.

No source material is read, named, parsed, or summarized. No package, cell, or runtime
code is written.

## Acceptance test

`python tools/port_gate.py check` passes for the checks owned by this ticket:
`baselines_present`, `governance_files_present`, `source_exclusion` (ignore rules present
and nothing tracked), `output_roots` (all declared paths contained), `baseline_blocks`
(every block has artifact, exit test, and refused work).

## Exit test evidence

Gate run recorded in `LEDGER-0001` and reproduced by
`python tools/port_gate.py check`.

## Shell-owned gating and refusal conditions

The shell assigns the block's gate state. The scaffold's own claim is limited to
`present` for the files it created.

## Incomplete state (must be declared, not hidden)

- Branch/PR review enforcement: no remote configured. Owner: repository owner. Tracked in
  `docs/SCOPE-ACCOUNTING.md`.
- Commit-hash binding in ledger entries: not implemented. Owner: repository owner.

## Excluded from this ticket

Package parsing, cell grammar, mount, runtime, rendering, ingestion, MCP.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha of the scaffold commit series>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0001
