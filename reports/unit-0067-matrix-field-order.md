# Unit 0067 — the placement matrix, measured

Source-safe: no supplied movie, its bytes, identifiers, offsets or media appear here.

## Commands and exit status

| Command | Exit |
|---|---|
| `cargo test -p swf-import --test placement` | 0 — 9 passed |
| `cargo test -p planet-package -p swf-import -p planet-client` | 0 — every suite |
| `python tools/port_gate.py selftest` | 0 — record regenerated |
| `python tools/port_gate.py check` | 0 — PASS |

## What the reader now does

- Scale: presence bit, width, two signed values at that width — read only if present.
- Rotation: presence bit, width, two signed values — read only if present, immediately after scale.
- Translation: width, two signed values; a width of zero leaves the pair at identity.

## Why the old tests were green

The fixture and the reader encoded the same non-format order, so both agreed with each other and
neither agreed with the format. The two added tests write a scale without rotation and a rotation
without scale; under the old reader the first returns a zero scale and the second a zero rotation.
They now pass for the right reason.

## Bounds

A matrix is read from the record's own bytes and ends where its last aligned field ends; the record
length is not used to skip trailing data, so a colour transform after the matrix is still refused
`placement-unsupported` rather than silently dropped.
