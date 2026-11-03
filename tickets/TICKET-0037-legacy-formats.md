# TICKET-0037 — Legacy format ids are registered, never emitted

- Owner: implementation agent
- Block: H2 format boundary
- Branch: `ticket/TICKET-0037-legacy-formats`
- Ledger: LEDGER-0043
- Depends on: H2 (`docs/PACKAGE-OWNERSHIP.md`), the 2026-11-02 drive audit

## What was wrong

129 numeric format ids exist across the trees. Classified by suffix first they are **78 qualified,
45 unqualified and 6 zero-based**; the audit's earlier split (83 qualified, 46 unqualified, 5
zero-based) counted zero-based ids inside the other two classes. Both readings total 129, and the
registry is the one the gate checks. The owner's ruling is that they are **not refused**: they are legacy formats awaiting update from the
upstream secure-shell v2 store. Nothing in this repository recorded them, so a numeric id could be
emitted by new code with no trace and no owner.

## What this builds

- `config/legacy-formats.json` — the registry, derived from the audit's stream
  (`secure-shell-audit/scan_stream.jsonl`): one row per id with its occurrences, the roots it was seen
  in, `derived: true`, and the rule that classified it (qualified / unqualified / zero-based). The
  4 GiB and per-package decisions are not touched here; this unit only registers identities.
- `tools/port_gate.py` `check_legacy_formats` — scans tracked code and config (`.rs`, `.py`, `.mjs`,
  `.js`, `.ts`, `.json`; skips `.git`, `target`, `generated`, `inputs`, `shell-private`, `captures`,
  `node_modules`, `artifacts`, `logs`, `reports`, and `tools/port_gate.py` itself) for

  `<family>.<noun>/<digits>` and refuses:
  - `legacy-unregistered` — a numeric id in code with no registry row;
  - `legacy-unprovenanced` — a row claiming upstream provenance without naming it, or a `derived`
    row presenting itself as upstream.

  A hashed id is not matched, so the rule is about numeric suffixes only.

## What this does not do

It does not rewrite ids, does not emit anything, and does not decide the upstream store key for a
row: a derived row says where it was seen, not where it came from. That distinction is the reason
`legacy-unprovenanced` exists as its own refusal.

## Acceptance test

`python tools/port_gate.py check` exits 0 with 21 checks; `python tools/port_gate.py selftest` exits 0 with 123 of 123 scenarios, its sweep, silence, decoy and near-miss cases showing that `legacy-unregistered` fires on a planted numeric id and that the check stays silent on registered, hashed and uppercase lookalike ids.
## Closing

- Commit: this unit's commit on `ticket/TICKET-0037-legacy-formats` (unmerged; no remote is
  configured in this checkout, so no PR or merge exists)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0043
