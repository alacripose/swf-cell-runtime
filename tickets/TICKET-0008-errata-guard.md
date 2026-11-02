# TICKET-0008 — H3: guard the plan's errata against silent drift

- Owner: implementation agent
- Block: H3
- Branch: `ticket/TICKET-0008-errata-guard`
- Ledger: LEDGER-0010
- Depends on: TICKET-0006 (the errata this guards)

## Scope (source-safe)

Add a mechanical check so an erratum cannot be quietly edited away: its blockquote, its quote of the
superseded wording, its citations, and the original text it supersedes must all remain present. The
manifest names the phrases and citations; the check compares them against the plan. No source movie is
read, named, or described. Nothing was executed outside the gate and its self-test.

## Why a manifest rather than hardcoded strings

The gate would otherwise duplicate the plan's prose, and a second copy of a sentence is exactly the
kind of drift this repository refuses. `config/plan-errata.json` names what must survive; the plan
stays the only copy of the wording.

## What the check proves

For each declared erratum:

1. A `> **ERRATUM <id>` block exists in the plan.
2. The block keeps its `**Superseded` part, its `**Measured:**` part and its `**Status:**` part.
3. Every declared citation appears in the block — so a correction cannot lose the file or symbol that
   proves it.
4. Every declared superseded phrase appears **both** in the block (the quote) and **outside all
   blocks** (the original sentence). Removing the original wording fails the check even when the quote
   survives.
5. The erratum is listed in the errata index at the top of the plan.

## Acceptance test

- `python tools/port_gate.py check` reports **10** checks, including `plan_errata`, and passes.
- `python tools/port_gate.py selftest` reports **10** scenarios and passes, including two that prove
  the new refusal: `erratum_removed_refused` (the E-3 block deleted) and
  `erratum_original_removed_refused` (the §3.1 original sentence deleted, quote left in place).
- `git ls-files | grep -Ei '\.(swf|sfw)$'` — none.

## Deliberate limitation, stated rather than hidden

The check guards **the record**, not the source trees: citations name files in checkouts outside this
repository, and the gate cannot verify they still contain the cited symbol. Re-measuring a citation is
the source-reading job that produced the errata, not something a path check can do. This is recorded
here and in the manifest's `note` rather than claimed as coverage.

## Incomplete state (declared, not hidden)

- No check verifies that a citation's symbol still exists in the external tree.
- The errata's factual corrections are still only as good as LEDGER-0008's reading; nothing re-runs it.
- H4–H10 remain as they were: H4 shell-only, H5 decided (D-3) but unimplemented.

## Excluded from this ticket

Editing the plan's errata; adding checks for claims the errata do not cover; any parser or client code.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0010
