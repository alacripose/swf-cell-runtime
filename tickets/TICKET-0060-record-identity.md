# TICKET-0060 — Regenerate the gate record after my own scratch scripts polluted the fixture identity

- Owner: implementation agent
- Block: H9
- Branch: `ticket/TICKET-0060-record-identity`
- Ledger: LEDGER-0066
- Depends on: TICKET-0059

## What went wrong

`tools/port_gate.py` computes the fixture identity over `FIXTURE_PATHS`, and that list includes `tools/`
as a directory. During TICKET-0058/0059 I left three scratch scripts in `tools/` (`land.py` and two
patch scripts) while the selftest wrote `artifacts/redacted-test-results/port-gate-selftest.json`, so the
committed identity covers files that are not part of the repository. The working tree then agreed with
the record, and `port_gate.py check` answered `PASS`, but a fresh `git worktree` checkout did not
reproduce the identity: **MISMATCH**, the one check that proves the record describes the committed tree.

`check` does not compare the record's identity with the tree, so it cannot catch this. The fresh-checkout
probe does, which is why AGENTS.md §9's digest is worth producing rather than trusting.

## The fix

1. Delete the scratch scripts from `tools/` (they were never committed).
2. Regenerate the record with the tree in the state a checkout would produce.
3. Commit the corrected record alone, so the diff names exactly the evidence that changed.

## The durable lesson

Scratch work goes under `target/`, which the gate skips, not under `tools/`, which it hashes. This ticket
is the reason that sentence now exists in `docs/CAMPAIGN-FRONTIER.md`.

## Acceptance test

- `python tools/port_gate.py selftest` exits 0 and rewrites the record.
- `python tools/port_gate.py check` exits 0.
- A fresh `git worktree` checkout of HEAD reproduces the record's identity exactly, with no tracked
  fixture input carrying CRLF, and the working tree is clean.

## Closing

- Commit: this unit's commit, fast-forwarded onto `master`; nothing pushed
- Shell gate state: unassigned
- Ledger entry: LEDGER-0066
