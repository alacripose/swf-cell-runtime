# TICKET-0086 — headless evidence: imported pixels, decoded sounds, and the read-only axis

Status: `present` (evidence run, axis read, sound decode, script refusal). Bitmap fills and per-cell
provenance are `not-implemented` and named below.

## What the run proves, from artifacts this pass produced

`inspect --evidence generated/swf1.pkg artifacts/evidence` mounted the imported package with the
client's own adoption path, drew and painted all 12 carried frames, decoded the frame's sounds and
wrote `artifacts/evidence/evidence.json` plus one PPM per tick.

- Frames drawn: 12 at 40 fps (the movie's own rate).
- Tick 0, frame `STAGE-0001`: 307 200 pixels differ from a blank surface (the same canvas with every
  byte zero), across 2 400 painted cells.
- Attribution: every painted cell names its cell id (`frame/col:row`), its pixel rectangle
  `[x,y,8,16]`, and the imported character whose placement covers it — character 132 at depth 548 on
  tick 0. Frames 1–12 each change 307 200 pixels as well, so every carried frame is drawn, not just
  the first.
- Sounds: 8 imported clips decoded through the client's symphonia channel on tick 0, each with its
  tick, sample offset, rate, channels, frames, samples and peak. One further clip was refused
  `audio-unreadable` with its reason (`no suitable format reader found`) and is printed by the run.
- Script lane: the axis reports `refused` / `frame-kind-reserved`. The probe is the package's own
  bytes with the frame kind set to `code` (2), offered to the real mount, which refused it. The
  missing lane is therefore a recorded refusal, never a silent blank.

## Refused / not-implemented, named

- **Bitmap fills.** The importer answers one neutral colour for a bitmap fill; 18 bitmap characters
  are counted and none are decoded. The evidence attributes sprite and shape cells, never a bitmap's
  pixels, and the JSON states this in its `bitmaps` row.
- **Per-cell provenance.** A frame's `[fill]` rows carry a paper, not the character that painted it,
  so attribution is by covering placement (the deepest matching rectangle), not by the painter.
  Frames 9–12 paint character 137's art under character 132's full-stage cover; the covering
  placement is what the axis can honestly name until the package carries the painter.
- **Client witness.** No window was opened. This is the client draw path over a mounted package, and
  `AGENTS.md` §7 keeps `client_witness` unclaimed. A bot reading the axis inherits the same limit and
  is a second, weaker witness — never counted as a client witness.

## Exit test

- `target/debug/inspect.exe --evidence generated/swf1.pkg artifacts/evidence` exits 0 and writes
  `artifacts/evidence/evidence.json` with 12 ticks.
- Every tick has `changed_pixels` > 0, named cells with pixel rectangles, and a `script` object whose
  `code` is `frame-kind-reserved`.
- Decoded sounds carry `tick` and `sample_offset`; undecodable clips appear as `sound_refused` lines
  with a reason.
- `tools/port_gate.py selftest`, `tools/port_gate.py check` and `tools/health.py` pass on this tree.
