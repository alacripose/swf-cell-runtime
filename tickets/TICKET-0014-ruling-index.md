# TICKET-0014 — Index every open question across the five proposals, and guard the index

- Owner: implementation agent (index and gate), format owner and project owner (each ruling)
- Block: none; serves the ruling pass across H5, H7, H9, H10
- Branch: `ticket/TICKET-0014-ruling-index`
- Ledger: LEDGER-0016
- Depends on: TICKET-0009 … TICKET-0013 (the five proposals)

## Scope (source-safe)

The five proposals carry 24 open questions between them, which makes a ruling pass a tour of five
files. This aggregates them into `docs/RULING-INDEX.md`: id, one-line subject, disposition as the source
has it, owner as the source names it, and source file. Nothing is marked decided, the three recorded
conflicts stay unresolved, and the items a source tables rather than invents appear as rows.

`ruling_index` in the gate refuses an index that omits a source's id, carries an id no source has,
loses or contradicts an owner or disposition, presents a question as ruled, or smuggles a wire token.

## Acceptance test

- The index has one row per source question (24) and a second table for items without an id.
- `python tools/port_gate.py check` → PASS, 15 checks.
- `python tools/port_gate.py selftest` → PASS, 29/29, including the five `ruling_index_*` refusals,
  with every earlier refusal still reporting its original message.
- The governance and fixture lists are printed before and after the insertion, with every existing
  entry kept.

## What this deliberately does not do

- It does not rule, resolve, or harmonise anything.
- It does not paraphrase an owner or a disposition into a tidier form.
- It does not read, name, or describe the source movie or any private shell material.

## Incomplete state (declared, not hidden)

All 24 questions remain open. No client, parser, decoder, executor or transport exists.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0016
