# TICKET-0035 — The importer converts this movie's structure into a canonical package

- Owner: implementation agent, under the authorization recorded in `AGENTS.md` §1.1 and `D-7`
- Block: H2 import boundary (user-authorized streaming proof)
- Branch: `ticket/TICKET-0035-swf-importer`
- Ledger: LEDGER-0041
- Depends on: TICKET-0029 (mount), TICKET-0031 (mount entry), TICKET-0034 (adoption)

## What was wrong

The tree could mount a package nobody could produce. The importer was off-agent by rule, so the
pipeline had no source side at all.

## What this builds

`crates/swf-import`:

- `header::read` — signature from the bytes (`FWS`/`CWS`/`ZWS`), version, declared length, stage rect,
  frame rate and count; `CWS` bodies are inflated, `ZWS` is refused `swf-compression` because no LZMA
  decoder is carried yet.
- `tags::walk` / `tags::counts` — the length-prefixed tag stream, short and long lengths, ending at
  code 0; a body that overruns is refused `swf-truncated`.
- `emit::package_frame` — the measured facts become a `dl-enclave.surface/1` document (one `world`
  window sized from the stage, a stage pane, and label nodes carrying the frame and tag counts), which
  is validated by `planet_package::Surface::parse` before it is framed as `V4PKG001`.
- `src/bin/swf-import.rs` — reads a file, writes the package, mounts and records it through
  `mount_and_record`, and prints the measurements, receipt digest and sequence.

## The lowering that is deliberately absent

Tag bodies are counted, not interpreted. Shapes, sprites, the timeline and AVM1/2 need
representations the format has not declared (`docs/CELL-SCHEMA.md` Q-1…Q-6 tabled; media M-1…M-5;
D-3 pending), so lowering them here would be the kind→role mistake again. The importer converts what
is declared and refuses to invent the rest; this is the gap `docs/DECISIONS-PORT.md` D-7 records.

## Real run (this game, measured 2026-11-02)

`cargo run -p swf-import --bin swf-import -- inputs/swf1.swf generated/swf1.pkg generated/mount-ledger.tsv swf1`

- zlib, version 8, declared 841492 bytes, decompressed 841484
- stage 640×480 px, 12 frames at 40 fps, 600 tags in 27 tag classes
- emitted package: 352 bytes; digest `f73f6144ba9fcfefa07282b9c707435e360320d46d220e91508a3b20c87a63f5`
- ledger `generated/mount-ledger.tsv`: one `mount` entry, sequence 1, `mount/<surface>` subject,
  `landed`

The package proves the streaming path — digest, receipt, durable entry, adoption — and nothing about
gameplay. The user is the witness.

## Acceptance test

`crates/swf-import/tests/import.rs`, 7 synthetic tests (no supplied bytes): header and tag stream
measured; a zlib body read; the emitted package mounts and its document carries the measured 550×400
stage and "12 frames at 40 fps"; an unknown signature refused `swf-signature`; `ZWS` refused
`swf-compression`; an overrunning tag refused `swf-truncated`; a three-byte file refused `swf-truncated`.

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0041
