# TICKET-0059 — The frontier as a tracked file, H9 named in the gate, and the package a witness needs

- Owner: implementation agent (record-keeping), with the run itself reserved to the user
- Block: H9
- Branch: `ticket/TICKET-0058-input-touch-and-wiring`
- Ledger: LEDGER-0065
- Depends on: TICKET-0057, TICKET-0058

## Why

The campaign's open sections were spread across tickets and conversation, which is how §4.7 was
mistakenly closed one turn and reopened the next. One tracked page now says, for every section, who owns
the next step and the sentence that decides it.

## What this adds

1. `docs/CAMPAIGN-FRONTIER.md` — the frontier table, including the closed sections and the one that is
   blocked on a format decision (node geometry), regenerated at each landed unit.
2. An explicit not-enforced row in `tools/port_gate.py` for the client run, owned by the user and blocked
   on a manual window run, so the one open witness cannot be read as a pass. `NOT-ENFORCED` remains
   `absent`, never `held`.
3. The importer was run against the authorized input, `inputs/swf1.swf`, and wrote an untracked package
   under `generated/`, exiting 0. This is preparation for the witness, not the witness.

## What this does not claim

- A mounted package is not a playable game. The client run, and what it shows, belong to the owner; the
  gate row stays not-enforced until the owner records the run.
- Nothing derived from the source — no digest, byte count, tag count, offset or media inventory — appears
  in this ticket, the ledger row, the report or the commit.

## Acceptance test

- `python tools/port_gate.py selftest` exits 0 with the new row present, and `check` still reports the row
  as not enforceable rather than as a pass.
- The record is regenerated for the committed tree and a fresh `git worktree` checkout reproduces its
  fixture identity with no tracked fixture input carrying CRLF.

## Closing

- Commit: this unit's commit, fast-forwarded onto `master`; nothing pushed
- Shell gate state: unassigned
- Ledger entry: LEDGER-0065
