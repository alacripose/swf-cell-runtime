# TICKET-0010 — Propose the cell schema for the format owner's ruling

- Owner: implementation agent (proposal and gate), format owner (ruling), project owner (Q-6)
- Block: H9 (preparation); completes the schema half of the D-1 follow-up
- Branch: `ticket/TICKET-0010-cell-schema`
- Ledger: LEDGER-0012
- Depends on: TICKET-0009 (the model and its open-questions table)

## Scope (source-safe)

`docs/CELL-MODEL.md` §5 lists six open questions with owners but no content. This turns them into a
schema proposal the format owner can rule on: for each question, a recommended default with rationale,
the alternative it rejects, what would falsify it, and the owner. Where nothing in the plan or the
documents settles the question, the document says so and states exactly what the owner must choose
between, rather than inventing an answer.

The six model invariants become checkable: `config/cell-schema.json` holds them, and `cell_schema`
in the gate refuses a model that drops one, a proposal that omits a question's four parts, a port wire
token in either document, and any naming of the family-B raster that does not also reject it.

## Acceptance test

- `docs/CELL-SCHEMA.md` has a `### Q-1` … `### Q-6` section, each with `Recommendation`, `Rejected`,
  `Falsifier` and `Owner`; Q-6 is tabled as an owner choice with the evidence it needs named as
  unavailable to agents.
- The document says it is a proposal and contains no kind path, integer width or instruction token.
- `python tools/port_gate.py check` → PASS, 11 checks.
- `python tools/port_gate.py selftest` → PASS, 14/14, exit 0, including four new refusals:
  `cell_invariant_removed_refused`, `cell_wire_token_refused`, `cell_raster_target_refused`,
  `cell_question_unanswered_refused`.

## What this deliberately does not do

- It does not rule on any of the six questions; every one keeps its owner.
- It does not fix byte widths, encodings, or the code kind's instruction vocabulary.
- It does not claim a cell is parsed, drawn, or mounted anywhere.
- It does not adopt family B's raster, `shploingus.cells/1`, or `shploingus.cellframe/1`.

## Incomplete state (declared, not hidden)

All six questions await the format owner except Q-6, which awaits the project owner and depends on the
H4 inventory no agent may read. No parser, renderer, or client exists.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0012
