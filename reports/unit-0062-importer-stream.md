# UNIT-0062 - the importer's sealed stream

Ticket `tickets/TICKET-0062-importer-stream.md`, ledger `LEDGER-0068`, block H2 (streaming proof), unit P4.

## What changed

- `crates/swf-import/src/stream.rs`: rows sealed with `planet_aead` under the store-record domain, one
  per 1,000 measured records and per stage, flushed as written, closed by `done`; `abort(code)` writes a
  `truncated` row naming the code. The reader re-derives each row's plaintext and refuses an envelope
  that opens as a different row than the one it claims.
- `crates/swf-import/src/bin/swf-import.rs`: an optional stream path and `PLANET_STREAM_KEY`; a path
  without a key is refused by name, and every failure after the stream opens marks it truncated with the
  refusal's own code.
- `crates/swf-import/tests/stream.rs`: eight synthetic scenarios, including the two that matter —
  a row is readable before the stream finishes, and an unterminated stream is refused `import-truncated`
  while still yielding its sealed rows.

## What did not change

No shape, timeline or script is lowered; no code frame is emitted; there is no store writer. The tag
walk is still materialised, so the stream is written from the measurement rather than during it, and the
ticket says so rather than implying an interrupted walk was ever exercised.

## Checks

- `cargo test -p swf-import`: stream 8, import 7, content 5, all passing.
- `cargo test --workspace`, `port_gate.py selftest` and `check`, and a fresh-checkout identity probe:
  recorded in LEDGER-0068 with their statuses.
