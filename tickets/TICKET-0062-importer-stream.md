# TICKET-0062 — The importer's sealed stream: cadence, flush, and a truncation that is never silent

- Owner: implementation agent (importer scope, `AGENTS.md` §1.1 / D-7)
- Block: H2 import boundary (user-authorized streaming proof)
- Branch: `ticket/TICKET-0062-importer-stream`
- Ledger: LEDGER-0068
- Unit: P4 in `docs/testport1_plan_resynthesis.md` §6, which blocks on P3 — landed as `planet-aead`.

## Ownership, quoted

The importer is agent work for this game only, as a streaming proof (D-7), and D-7 says of it: *"It
lowers no shape, timeline or script: those representations are still the format owner's to [decide]."*
So this unit adds carriage of the measurement, never lowering. The cadence and the sealing are the
owner's answers, quoted from §8: *"Per stage, plus every 1,000 records inside a stage"* (Q7) and
sealing from Q6. The refusal this unit implements is §7's: *"A scan or import that holds its output
until exit"* is carried as a refusal, and Q14's *"nothing half-written"* is what the `truncated` row
makes visible instead of invisible.

## What this adds

`crates/swf-import/src/stream.rs` — one JSONL row per 1,000 measured records and per stage, then a
`done` row:

- **Sealed rows.** Each row's payload is `kind\tseq\tdetail`, sealed with `planet_aead` under
  `Domain::StoreRecord` (these rows are what the store receives). The envelope carries the kind, the
  sequence, the sealing key's short public id and the sealed bytes in hex; the reader re-derives the
  plaintext and refuses an envelope that opens as a different row than it claims, so the envelope is not
  trusted on its own.
- **Flushed as written.** Every row is sealed, appended and flushed before the next exists, so a reader
  can see the rows of an import that has not finished.
- **Truncation is a row, and a refusal.** `abort(code)` writes a `truncated` row naming the code; a
  stream whose last row is not `done` reads back as `import-truncated` with the number of sealed rows it
  did find, while the rows themselves stay readable. A killed import leaves evidence, not silence.
- **Closed after the end.** Any row after `done` or `truncated` is refused `stream-order`.

Wiring: `swf-import <input.swf> <out.pkg> <ledger.tsv> [surface-id] [stream.jsonl]`, with the key read
from `PLANET_STREAM_KEY`. A path without a key is refused by name rather than silently dropping the
proof; a key without a path writes no stream. Failures after the stream opens call `abort` with the
refusal code, so the stream and the exit status agree.

## What this does not do

- It does not stream *during* the tag walk: the walk is already materialised, so the rows are written
  from it in 1,000-record chunks. The kill-mid-import property is proved on the reader — an unterminated
  stream is refused — not by interrupting a half-finished walk. Making `tags::walk` incremental is a
  further unit.
- It does not lower shape, timeline or script, does not emit code frames, and adds no store writer (P10).
- It records the measurement stage only; the package and its ledger entry remain the emission record.

## Acceptance test

`cargo test -p swf-import` — 8 stream scenarios: rows are sealed and open in order; a row is on disk
before the stream finishes; a stream cut before its `done` row is refused `import-truncated` while its
rows stay readable; an aborted stream names its code in the last row and refuses a later row with
`stream-order`; a row sealed by another key is refused `stream-order`; a tampered row is refused
`aead-tag`; a line that is not a row is refused `stream-row`; the header names the stream format.

Plus the unchanged suites (import 7, content 5), `cargo test --workspace`, the gate's selftest and
check, and a fresh `git worktree` checkout that reproduces the record identity.

## Closing

- Commit: this unit's commit, fast-forwarded onto `master`; nothing pushed
- Shell gate state: unassigned
- Ledger entry: LEDGER-0068
