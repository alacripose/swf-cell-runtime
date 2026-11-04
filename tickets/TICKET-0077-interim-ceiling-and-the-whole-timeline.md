# TICKET-0077 — The interim ceiling admits the package, and the package carries the whole timeline

## The two blockers this removes

1. **The byte cap refused the package.** A package carrying its timeline measured 1 679 826 bytes of
   hex in one declaration, past `MAX_FRAME_BYTES = 512 * 1024`.
2. **Ingest carried 1 of 12 frames**, because the timeline had no carriage: a client could show the
   first picture forever and had no cadence to advance.

## What changed

- **The ceiling is an interim floor, in both readers.** `crates/planet-package/src/cellframe.rs` and
  `shploingus-clean/crates/voxel-package/src/cellframe.rs` now declare
  `MAX_FRAME_BYTES = 8 * 1024 * 1024`. The ruling that makes this legitimate is the owner's, not this
  port's (`secure-shell-audit/cell-schema-ruling.md`, Q-4): the 512 KiB / 16 384 pair is **not** the
  ceiling, addressing is 64-bit, geometry is subvoxel, and the exact numbers are **still owed**.
  `docs/CELL-SCHEMA.md` records it as a floor that moves when the owner gives the number, never as the
  ruling. `MAX_CELLS` moved in this port (16 384 → 65 536) because its reader counts a frame's cells
  as the sum of its fill runs; the family reader's `MAX_CELLS` is a declared grid ratio
  (`MAX_COLS * MAX_ROWS / 16`) and was **not** moved.
- **The timeline is carried, not named away.** `[timeline]` rows name each later frame's `order` and
  the artifact that carries it; each artifact is a frame document in the same grammar, written by
  `emit::timeline_document` with its own id and order. The bound frame's `order` is 1, so the mount's
  ordered answer is the movie's order.
- **The mount binds what the bound frame carries.** `cellframe::resolve` reads each `[timeline]`
  artifact through `CellFrame::parse`, extent-checks it against the same window, and binds it beside
  the frame that named it; `SurfaceMount::frames_of(window)` answers them sorted by `order`, and
  `frame_of` still answers the bound frame, so every existing caller is unchanged.
- **Sounds are scheduled, not just carried.** `placement::timeline` records `StartSound` (15) and
  `SoundStreamBlock` (45) per frame; the package states them as `[event]` rows
  (`sound<TAB>frame<TAB>audio-id`), validated against the audio table. A stream block has no character
  id, so it is named by the audio table's own `-`.
- **The client has a cadence.** `planet_client::Timeline::of(adopted, window)` answers the frames, and
  `frame_at(tick)` wraps. It selects a carried frame; it never synthesises one between two.

## Measured evidence

```
declaration_bytes 740802          (was 524288 ceiling; the earlier hex-in-one-row attempt was 1679826)
timeline_documents 12
audio_events 1                    (the movie's stream block; it carries no StartSound)
timeline_frames 11 artifacts 20   (the bound frame's 9 audio clips + 11 carried frames)
tick 0..11 frame STAGE-0001..STAGE-0012 cells 2400 pixels 307200
```

`crates/swf-import/tests/end_to_end.rs` runs the importer on the real movie and asserts all of the
above through `planet_client`'s own adopt, mount, timeline, draw and paint path — 12 frames, real
filled pixels, and every event naming a carried payload.

## What this does **not** prove, and what is still wrong with the picture

- **The picture is flat white, and the reason is not the tick.** All twelve frames paint one colour
  and one digest. The movie's scene is **18 bitmap characters, 258 207 bytes**, which this importer
  does not decode, and a white cover character (132, depth 548, 2400 cells) sits in front of the
  stage in every frame. Carrying the timeline is real; it is not yet the game's art.
- **The proof is headless.** No window drew this package in any assertion — see
  `TICKET-0078`. A window binary run on this machine printed `drawn 2400 cells in STAGE-0001` and
  cells `255 255 255 255`, which is a headless-equivalent report, not an observed screen.
- `[event]` timing is frame order, not a millisecond offset; sub-frame sound offsets are not carried.

## Acceptance

- `cargo test -q -p planet-package -p swf-import -p planet-client` exits 0.
- The importer prints `timeline_documents 12`, `audio_events 1`, and writes a frame document over
  512 KiB that the grammar reads.
- `python tools/port_gate.py check` prints `RESULT: PASS`.
