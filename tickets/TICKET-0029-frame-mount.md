# TICKET-0029 — The `V4PKG001` frame and its mount receipt

- Owner: implementation agent
- Block: H2 (package schema and frame)
- Branch: `ticket/TICKET-0029-frame-mount`
- Ledger: LEDGER-0034
- Depends on: TICKET-0026 (the SHA-256 the frame is framed with), D-6, plan §13.5

## What this builds

`crates/planet-package`, the port's own package layer:

- `frame.rs` — the `V4PKG001` frame: `magic (8) | schema (LE u16) | kind (LE u16) | len (LE u64) |
  payload digest (32) | payload`, with the payload digest framed `domain || schema (LE u16) ||
  len (LE u64) || payload`, which is the family's framing (`planet-core/src/hash.rs`) and the rule plan
  §13.5 fixed.
- `mount.rs` — the receipt in the shape `secure-shell` vendors as `surface.mount/2`, with no field
  added: `surface`, `sequence`, `digest`, `state`, `windows`, `at`.

## Refusals, each with a stable code

`frame-magic`, `frame-schema` (with `found` and `supported` — an older frame is refused by name, never
converted), `frame-kind` (the closed set `decl(1)`/`code(2)`), `frame-truncated`, `frame-length`,
`frame-digest`, `package-payload-limit` (4 MiB), `frame-kind-reserved` (a `code` frame packs and is
refused at mount), `mount-sequence`, `surface-schema`, `surface-windows`.

## Limits named rather than hidden

- `kind` is in the header and in the whole-frame digest, **not** in the payload preimage. §13.5 fixes the
  preimage as domain, schema, length, payload; two frames with the same payload and different kinds
  therefore share a payload digest and differ in their frame digest. Changing this is a schema decision,
  not an edit.
- The surface document is checked at its boundary only — schema row, surface id, final newline, at least
  one window. Unknown rows and unknown keys inside `dl-enclave.surface/1` are a later unit, and this unit
  does not claim to refuse them.
- No signature, no attestation, no ledger write: the receipt carries a digest and nothing signs it. The
  `code` kind stays reserved because no reader exists.

## Acceptance test

`cargo test -p planet-package` — round trip, the framed digest recomputed independently, older schema
refused by name, another magic refused, closed kind set, truncation, declared length mismatch, altered
payload, payload cap, code packs and mount refuses it, sequence, a non-surface payload, a missing final
newline, a mismatched surface id, and a surface with no windows.

`python tools/port_gate.py selftest` then `check` — held for every check, the four `absent` rows named.

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0034
