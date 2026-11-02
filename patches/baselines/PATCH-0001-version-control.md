# PATCH-0001 — version control

| Field | Value |
|---|---|
| Id | PATCH-0001 |
| Block | H1 |
| Status | active baseline (hard gate) |
| Supersedes | — |
| Weakens which | — |

## Required rule

Initialize Git immediately; commit small coherent units early and often. Every commit
cites its ticket and its ledger entry. Work happens on ticket branches and is reviewed
through pull requests. Do not accumulate a large uncommitted implementation and do not
squash away the useful sequence.

## Mechanical evidence

- An initial baseline commit exists before any implementation work, and it contains the
  rules rather than product code.
- `tools/port_gate.py` check `commit_refs`: every commit subject carries a `TICKET-NNNN`
  and a `LEDGER-NNNN` reference, both of which resolve to existing records.
- Commit-to-ticket and commit-to-ledger references are therefore verifiable from
  `git log` alone.
- `.github/pull_request_template.md` exists and requires the ticket link, the ledger
  entry, the review record, and the exit test that was run.
- Commit-size/drift: the scaffold repository is three small commits, not one.

## Not allowed under this patch

- Implementation commits before the baseline commit.
- A commit without a ticket or ledger reference.
- Direct-to-main implementation of nontrivial change.
- A single giant initial commit that hides the sequence.

## Residual gap (recorded, not waived)

Branch/PR review enforcement needs a remote repository and a review record. No remote is
configured in this checkout, so the gate verifies commit references but cannot verify a
review. Tracked in `docs/SCOPE-ACCOUNTING.md`.

## Changing this patch

Requires an explicit user decision plus a superseding patch record. An implementation
agent cannot amend the baseline it is measured against.
