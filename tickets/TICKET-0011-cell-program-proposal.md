# TICKET-0011 — Propose the cell program's op set, capabilities and budgets (D-3)

- Owner: implementation agent (proposal and gate), format owner (ruling), shell/test harness (witness)
- Block: H5 (preparation)
- Branch: `ticket/TICKET-0011-cell-program`
- Ledger: LEDGER-0013
- Depends on: TICKET-0007 (the D-3 ruling)

## Scope (source-safe)

D-3 ruled the class of executor and left the op set, capability vocabulary, budgets and encoding open.
This writes `docs/CELL-PROGRAM-DECISION.md` as a proposal: five questions, each with a recommended
default, the alternative it rejects and why, an observation that would falsify it, and the owner who
must rule. Where nothing derivable exists — the operation enumeration, which depends on shell-private
compilation — the document says so and states what the owner must decide.

The gate gains `cell_program`, which refuses a question missing any of its four parts, a port wire
token in the document, a budget stated without a label, and a budget framed as a constraint of the
imported program.

## Acceptance test

- `docs/CELL-PROGRAM-DECISION.md` has `P-1`…`P-5`, each with Recommendation, Rejected, Falsifier and
  Owner; every figure carries `[proposal]` or `[class of limit]`.
- `config/cell-program.json` holds the question ids, labels, token list, budget pattern and the
  forbidden budget framings, so the document stays the only copy of its own prose.
- `python tools/port_gate.py check` → PASS, 12 checks.
- `python tools/port_gate.py selftest` → PASS, 18/18, including
  `cell_program_owner_removed_refused`, `cell_program_wire_token_refused`,
  `cell_program_budget_unlabelled_refused`, `cell_program_budget_as_constraint_refused`, with the four
  existing `cell_schema` refusals still firing unchanged.

## What this deliberately does not do

- It does not rule any question; each keeps its owner.
- It does not enumerate the operations, fix an encoding, or propose integer widths.
- It does not claim an executor exists, or that any program runs.
- It does not read, name, or describe the source movie or any shell-private material.

## Incomplete state (declared, not hidden)

No executor, no validator, no compiler, and no client exist. The member list and the compiler contract
are blocked on H4/H5; the plan's `Code` lane stays `not-implemented`.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0013
