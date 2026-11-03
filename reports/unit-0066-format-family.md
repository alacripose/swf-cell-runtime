# Unit 0066 — the format family, measured

Source-safe: no supplied movie, its bytes, identifiers, offsets or media appear in this report.

## Commands and exit status

| Command | Exit |
|---|---|
| `cargo test -p swf-import --test placement` (before any edit) | 101 — `a_place_object_2_reads_scale_and_rotation`, expected scale `(2.0, 2.0)`, received `(0.0, 0.0)` |
| `cargo test -p swf-import --test placement` (after the fixture's declared width is 19 bits) | 0 — 7 passed |
| `cargo test -p planet-package -p swf-import -p planet-client` | 0 |
| `python tools/port_gate.py selftest` | 0 — record regenerated |
| `python tools/port_gate.py check` | 0 — PASS |

## What the replacement id is, and is not

- The id is `planet.surface/<32 hex>`: the family this package layer owns, a slash, and the first 32
  hex characters of the SHA-256 of the line-oriented preimage.
- The preimage holds the family, every accepted legacy family, the reader version, the accepted
  schema numbers, and the sorted refusal set. Reordering the refusal list does not change the id;
  adding a legacy family does.
- The `V4PKG001` magic is unchanged and the header still carries a `u16` schema, so the id travels
  as the declaration's first line, as it did before this unit. That gap is stated in the module doc,
  in the ticket, and in `refused_next`.

## Refusals this unit can answer

| Input | Answer |
|---|---|
| the computed id | read as current |
| `dl-enclave.surface/0`, `/1`, `/2` | read as an upgrade; refused `format-id-legacy-emitted` on emission |
| `planet.surface/2` | legacy as well: upgrade on read, `format-id-legacy-emitted` on emission |
| any 32-hex id this reader did not compute, in either family | `format-hash-mismatch`, naming declared and computed |
| text that is neither | `surface-schema`, naming the computed id |

## The test that was red on trunk

The fixture wrote a 2.0 scale into a field it declared as 16 bits wide. 16.16 fixed point stores 2.0
as 131,072; a signed 16-bit field holds at most 32,767. The truncation was the fixture's, the zero
was the reader obeying what it was told. The fixture now declares 19 bits.

The reader's own field order is a separate defect and is not repaired here: it reads the rotation
flag beside the scale flag, while the format writes the rotation flag after the scale pair. Ticket
0067 owns that, and the synthetic fixtures will have to be rewritten in the same unit or they will
keep agreeing with the reader instead of with the format.
