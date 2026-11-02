# TICKET-0002 — H2: record package-format ownership status

- Owner: format owner (decision), implementation agent (record)
- Block: H2
- Branch: `ticket/TICKET-0002-package-ownership`
- Ledger: LEDGER-0002
- Depends on: TICKET-0001

## Scope (source-safe)

Record `docs/PACKAGE-OWNERSHIP.md` and add the full scope accounting in
`docs/SCOPE-ACCOUNTING.md` so no named contract item can vanish by omission.

## Acceptance test

- `docs/PACKAGE-OWNERSHIP.md` exists, states `blocked`, names the blocking dependency, and
  lists the rejected alternatives with reasons.
- `docs/SCOPE-ACCOUNTING.md` exists, contains a status row for every block H1–H10, and
  contains no `present` claim for a package, cell, runtime, or MCP item.
- `python tools/port_gate.py check` passes `scope_accounting`.

## Exit test evidence

Gate run recorded in `LEDGER-0002`.

## Shell-owned gating and refusal conditions

H2's real exit test cannot pass in this checkout: the authoritative package crate is not
present, so ownership cannot be named from evidence. Per plan §12 the package layer is
recorded as blocked and implementation stops at design evidence. The shell decides when
the crate is supplied and the block may reopen.

## Incomplete state (must be declared, not hidden)

Every package-layer, import-layer, runtime, addon, persistence, and MCP item — enumerated
with owner and blocking dependency in `docs/SCOPE-ACCOUNTING.md`.

## Excluded from this ticket

Any parser, schema, magic constant, digest function, cell grammar, or bytecode grammar.
Writing one here would create the second-reader drift H2 exists to prevent.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0002
