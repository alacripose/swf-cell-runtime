# TICKET-0012 — Propose the media kinds' field contents (D-2)

- Owner: implementation agent (proposal and gate), format owner (ruling)
- Block: H7 preparation (media carriage)
- Branch: `ticket/TICKET-0012-media-kinds`
- Ledger: LEDGER-0014
- Depends on: TICKET-0007 (the D-2 ruling)

## Scope (source-safe)

D-2 ruled that bitmaps, fonts and audio each get a kind with one field each and left the field contents
open. This writes `docs/CELL-MEDIA-DECISION.md` as a proposal: five questions, each with a recommended
default, the alternative it rejects and why, an observation that would falsify it, and the owner.

The document records one genuine pressure point rather than resolving it: D-2 fixes one field per kind,
and a reader may need an extent before it can read that field (M-1). It also states what cannot be
proposed — the members of the format sets, and whether a font carries outlines or a baked atlas — both
because they depend on private conversion output.

## Acceptance test

- `docs/CELL-MEDIA-DECISION.md` has `M-1`…`M-5`, each with Recommendation, Rejected, Falsifier and
  Owner, and states its consistency with the cell model, schema and program proposals.
- `config/cell-media.json` holds the question ids, labels and token list, so the document stays the
  only copy of its own wording.
- `python tools/port_gate.py check` → PASS, 13 checks.
- `python tools/port_gate.py selftest` → PASS, 21/21, including `cell_media_owner_removed_refused`,
  `cell_media_wire_token_refused` and `cell_media_question_missing_refused`, with every earlier
  `cell_schema` and `cell_program` refusal still firing with its original message.

## What this deliberately does not do

- It does not rule any question; each keeps its owner.
- It does not fix encodings, format-set members, or glyph representation.
- It does not claim a decoder, a reader, or a surface exists.
- It does not read, name, or describe the source movie or any private shell material.

## Incomplete state (declared, not hidden)

No decoder, no reader for the media kinds, and no client exist. H7 stays blocked on the format owner's
ruling plus the private conversion output this repository cannot see.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0014
