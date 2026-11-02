# TICKET-0027 — Land the chain durably

- Owner: implementation agent
- Block: H1
- Branch: `ticket/TICKET-0027-ledger-landing`
- Ledger: LEDGER-0031
- Depends on: TICKET-0026

## What was wrong

TICKET-0026's crate documented that "an append that is not landed until it is synced", and the code
had no sync path at all: the chain lived in memory and `render()` returned a string. That is a claim
the code did not keep, and `secure-shell/docs/LEDGER.md` is explicit — "A change is landed when its
entry is appended and fsynced; the entry and the evidence are one record."

## What this unit adds

`crates/planet-shell-ledger/src/landing.rs`:

- `land`: render, write a `.staging` sibling in the destination's own directory, `sync_all`, then
  rename over the destination, then sync the parent where the platform allows it. A refused render
  writes nothing at all, so the destination keeps its previous bytes.
- `load`: verify on read; a missing file is an empty chain, a broken one is a refusal.
- `staging_path`: the staging sibling, so the naming is testable rather than implied.

## Acceptance test

`cargo test -p planet-shell-ledger` — 18 tests, adding: a landed chain reloads and verifies; staging
does not survive a landed write; a missing log reads as empty; an edited file refuses with
`broken-chain`; an unrepresentable entry creates neither log nor staging file; landing twice keeps one
chain.

Two limits are named rather than hidden: the durable form is one tab-separated line per entry, and
`rename` is atomic over a file replacement on this filesystem but the parent sync is best-effort.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: 498c923 on branch `ticket/TICKET-0027-ledger-landing`, unit 962ef9c
- Shell gate state: unassigned - only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0031
