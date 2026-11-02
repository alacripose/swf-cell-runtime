# TICKET-0017 — Enforce the MCP surface prohibition instead of guarding it

- Owner: implementation agent (check), shell (H6 surface)
- Block: H6 (boundary), replacing the NOT-ENFORCED row `mcp_prohibited_operations`
- Branch: `ticket/TICKET-0017-mcp-surface`
- Ledger: LEDGER-0019
- Depends on: TICKET-0003 (the gate), TICKET-0016

## The defect

`mcp_prohibited_operations` was a NOT-ENFORCED row with the note "prohibited-operation scan is a guard
until an MCP schema exists". Nothing refused a schema that exposed `surface.capture`, so the rule was
recorded rather than decided.

## What makes this enforceable

`patches/baselines/PATCH-0006` is committed and enumerates both halves in its own words: the permitted
operations (`schema.describe`, `ticket.submit`, `secure_cell.get`, `test.list_declared`,
`test.run_declared`, `test.result.get`) and the prohibited families. The patch already names the check
(`mcp_surface`) and its scan (`mcp/`, `src/mcp/`).

So the check derives its vocabulary from that patch rather than from a new list: a schema file under
`mcp/` or `src/mcp/` may name only the permitted operations, and may not declare a prohibited family.
With no schema present the check reports `guard: no MCP schema present` — which is what PATCH-0006 asks
for — while still being decidable the moment a schema appears.

## Acceptance test

- `check` → PASS, 18 checks, `NOT-ENFORCED (5)`; the row is deleted, not duplicated.
- `selftest` → PASS, 35/35, including `mcp_surface_forbidden_operation_refused` and
  `mcp_surface_permitted_passes`.
- The refusal names the offending operation and lists what is permitted.
- Two consecutive `selftest` runs write a byte-identical artifact with a clean worktree.

## What this deliberately does not do

- It does not write `docs/MCP-SECURE-CELL-CONTRACT.md`, which H6 assigns to the shell.
- It does not claim an MCP server exists, or that its behaviour was witnessed.
- It does not add a second advisory list beside the patch.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0019
