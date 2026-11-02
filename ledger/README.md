# Ledger — append-only evidence record

One JSON object per line, append-only. Each entry is the checkpoint record for one block
or one bounded artifact (see `docs/PORT-BASELINE.md`, "90-minute checkpoint fields").

Required fields: `id`, `timestamp`, `block`, `ticket`, `artifact`, `exit_test`,
`evidence`, `status`, `shell_gate_state`.

- `status` is implementation vocabulary only: `present`, `not-implemented`, `blocked`.
- `shell_gate_state` is `unassigned` until the shell records `held` / `absent` /
  `refused`. An agent must never write one of those values here; the gate rejects it.
- `evidence` points at a project-local, source-safe record. Never a source path, source
  digest, capture, or raw trace.

Entries are never edited or removed. A correction is a new entry that references the one
it supersedes.
