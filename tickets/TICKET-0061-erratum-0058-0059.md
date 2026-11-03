# TICKET-0061 — Erratum: one commit carries two units, and LEDGER-0064 names no commit

- Owner: implementation agent
- Block: H9
- Branch: `ticket/TICKET-0061-erratum-0058-0059`
- Ledger: LEDGER-0067
- Depends on: TICKET-0058, TICKET-0059

## What is actually in history

Commit `43463fb` is written as `TICKET-0059 LEDGER-0065`, and it carries **both** units: the touch and
wiring files (`input.rs`, `render.rs`, `bin/window.rs`, `tests/input.rs`), TICKET-0058's ticket and report,
and TICKET-0059's gate row, frontier page and report. No commit message cites `LEDGER-0064`.

So LEDGER-0064 is a true row describing real work, and its work is committed - but it does not name a
commit that carries it, which is exactly what AGENTS.md §9 requires. This ticket records that instead of
rewriting the trunk, because rewriting a landed commit to tidy attribution would destroy the audit trail
the rule exists to protect.

## Why it happened

1. `tickets/` and `ledger/` are fixture inputs for the gate's selftest record. Adding TICKET-0058's ticket
   and LEDGER-0064's row changed the fixture identity, so the record committed at TICKET-0057 was stale
   the moment those files existed.
2. The pre-commit hook ran `port_gate.py check`, which answered `FAIL selftest_record` (1 of 21 checks), and
   the commit was refused. I had planned to regenerate the record in the *second* step, which was one step
   too late.
3. The refused commit had already staged its files. The next step staged more and ran a plain
   `git commit`, which commits the whole index - so the second commit swept up the first unit's files and
   its message never mentions them.

## What changes

- **One unit, one commit, record regenerated last.** A unit's ticket, ledger row and regenerated record
  are written before its own commit, never after.
- **Commit with an explicit pathspec.** `git add <paths> && git commit -- <paths>` confines a commit to the
  paths named, so a refused commit cannot leak its index into the next one.
- `tickets/TICKET-0058-input-touch-and-wiring.md` now names `43463fb` and says it is co-located with
  TICKET-0059, rather than claiming a commit of its own.

## A gate gap, named and not closed here

`port_gate.py check` answered `PASS` while LEDGER-0064 named no commit, so the attribution rule is enforced
for the selftest's own fixtures but not over the live ledger. Closing it means teaching `commit_refs` to
require every ledger row to appear in a commit message - a gate change with its own ticket, not a silent
addition here.

## What this does not do

- It does not rewrite history, squash `43463fb`, or move files between commits.
- It does not claim the missing attribution is invisible: the next reader sees it here.

## Acceptance test

- `python tools/port_gate.py selftest` exits 0, `check` exits 0, and the record describes the committed tree.
- A fresh `git worktree` checkout reproduces the record identity exactly.
- `git log --all --grep=LEDGER-0067` finds this unit's commit after it is merged.

## Closing

- Commit: this unit's commit, fast-forwarded onto `master`; nothing pushed
- Shell gate state: unassigned
- Ledger entry: LEDGER-0067
