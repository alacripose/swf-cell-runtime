# TICKET-0040 — The declaration carries a hashed format id, and a legacy id is upgraded in

- Owner: implementation agent
- Block: H2 format boundary
- Branch: `ticket/TICKET-0040-format-hash`
- Ledger: LEDGER-0046
- Depends on: TICKET-0037 (legacy registry), TICKET-0039 (the client that reads the declaration),
  the owner's rulings Q1–Q3 and Q20 of 2026-11-02

## What was wrong

The declaration's first line was `dl-enclave.surface/1`: a family, a noun and a counter. Two
different grammars can both be `/1`, so the line named a format without naming a version of it, and
the reader had no way to say "this is a different surface than the one I implement" except by
comparing a literal.

## What this builds

`crates/planet-package/src/format_id.rs`:

- `id()` is `dl-enclave.surface/<32 hex>`, hashed over a line-oriented preimage: the family, the
  reader version, the accepted schema numbers and the full refusal set, refusals sorted because the
  set is a set (Q2, Q3).
- `read(declared)` answers three ways: the computed id is `Current`; a numeric id for this family is
  `UpgradedLegacy` — **read, not refused**, and re-emitted under the hashed id; any other hash is
  refused `format-hash-mismatch` with `declared` and `found`; anything else is refused
  `surface-schema`.
- `emit(declared)` refuses a legacy id `format-id-legacy-emitted`, naming the legacy id and the id a
  new document must carry.

`Surface::parse` now goes through `read`, so a legacy document mounts as the current format and the
importer emits `planet_package::schema_id()` as the first line. Two refusals are new:
`format-hash-mismatch`, `format-id-legacy-emitted`.

## What this does not do

It does not move the id into the `V4PKG001` header: the frame keeps its `schema`, `kind`, `length`
and digest layout, and the hashed id travels as the declaration's first line. Changing the header is
a separate unit with its own reader. It does not rewrite the upstream store's legacy rows (Q20): the
registry from TICKET-0037 still records them, and `emit` is the only place a legacy id meets a
refusal.

## Acceptance test

`cargo test -p planet-package -p swf-import -p planet-client` exits 0: 8 format-id tests (id shape,
stability, preimage contents, current read, legacy upgrade for `/0`, `/1`, `/2`, foreign hash
refused, non-id text refused, emission refusal), plus the surface, mount, importer and adopt suites
that now cross the new path.

## Closing

- Commit: this unit's commit on `ticket/TICKET-0040-format-hash` (unmerged; no remote is configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0046
