# Unit 0090 — the client's own draw path, measured

Status: `present` for the evidence run; `not-implemented` for bitmap fills and per-cell provenance.

## Evidence produced in this pass

`artifacts/evidence/evidence.json` (12 ticks) and `artifacts/evidence/frame-00.ppm` … `frame-11.ppm`
(the pictures themselves; the JSON is the record and `artifacts/evidence/*.ppm` is ignored).

| fact | value |
| --- | --- |
| frames drawn | 12 at 40 fps |
| pixels differing from blank, per tick | 307 200 of 307 200 |
| painted cells, tick 0 | 2 400 |
| characters attributed | 132 at depth 548 |
| decoded sounds on tick 0 | 8, plus one `audio-unreadable` with its reason |
| script lane | `refused` / `frame-kind-reserved` |

The axis is read-only: `Snapshot::read` queries the frame the mount resolved and the raster the
client drew. It advances nothing, decodes nothing and stores nothing.

## Named limits

Bitmap fills remain one neutral colour (18 counted, none decoded), and a frame's fill rows do not
carry their painting character, so attribution is by covering placement. Both are recorded as
`not-implemented` in the evidence JSON and in `docs/MISSION-FRONTIER-2026-11-04.md`.

## What this is not

No window opened, so no client witness. `AGENTS.md` §7 keeps `client_witness` and `client_run_h9`
unclaimed, and a bot consuming the axis does not change that.
