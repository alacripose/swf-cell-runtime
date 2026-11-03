# TICKET-0038 — Frames, ledger rows and store records are sealed, not plaintext

- Owner: implementation agent
- Block: H2 format boundary
- Branch: `ticket/TICKET-0038-aead-seal`
- Ledger: LEDGER-0044
- Depends on: TICKET-0037 (legacy registry), the owner's sealing rulings of 2026-11-02

## What was wrong

Nothing in this repository sealed anything. The frame carried a digest, which proves the bytes are
the bytes and says nothing about who may read them; the ledger and the store rows were plaintext.
The client work cannot start until the read path can answer "is this sealed, and with which key".

## What this builds

`crates/planet-aead`: XChaCha20-Poly1305 with three domains — `planet.frame/1`,
`secure-shell.ledger-row/1`, `secure-shell.store-record/1` — each label used as the AAD and as the
first ingredient of a deterministic nonce, so a blob sealed for one purpose does not open as another
and sealing the same bytes twice is stable. Keys are 32 bytes, parse from 64 hex characters, and a
row can carry `Key::id()` (16 hex characters) without carrying the key.

Refusals by name: `aead-missing`, `aead-key`, `aead-short`, `aead-tag`.

## What this does not do

It does not yet seal anything in the importer, the frame reader or the ledger writer. Wiring is the
next unit, and it is what the window depends on. It does not decide where a key lives on the host;
one key per package was ruled, and the store key for rows that span packages is still open.

## Acceptance test

`cargo test -p planet-aead` exits 0 with 6 tests: round-trip in all three domains, a row that does
not open in another domain, a tampered byte refused, a wrong key refused, stable ciphertext across
two seals, and key parsing that refuses short input by name.

## Closing

- Commit: this unit's commit on `ticket/TICKET-0038-aead-seal` (unmerged; no remote is configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0044
