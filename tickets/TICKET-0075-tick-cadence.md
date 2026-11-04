# TICKET-0075 — The client needs a tick cadence, and the package needs to carry the timeline

## The problem

The package ships **one** frame document, so the client has nothing to advance through: it presents
the frame it happens to carry (frame 1 of 12 for the imported movie, a full-stage record) and never
reaches the frames that hold the scene. The dump looks "wrong" because the movie is idle, not because
the art is missing.

## Why the timeline was not simply inlined

An attempt carried the later frames hex-encoded in the bound frame's `[artifact]` table. The format
refused it:

```
refused: swf-frame the emitted package was refused:
cellframe-bound declaration bytes 1679826 exceeds the format's ceiling of 524288
```

The ceiling is the **owning reader's**, not this port's:

- `shploingus-clean/crates/voxel-package/src/cellframe.rs:74` —
  `pub const MAX_FRAME_BYTES: usize = 512 * 1024;` — "One frame's ceiling. A declaration, not a
  truncation: a frame larger than this is refused."
- `crates/planet-package/src/frame.rs:14` — `PAYLOAD_LIMIT: usize = 4 * 1024 * 1024`, with the frame
  length written as `len (LE u64)`. The u64 is the *encoding* of the length, and the payload is still
  bounded at 4 MiB.

So "a 64-bit ceiling" is the length field, not a licence for an unbounded declaration, and a
timeline of whole documents inside one declaration breaks both bounds. The format's shape is what the
owning trees already do: **a frame is a sibling artifact the package names by id**, and the client's
mount holds them in `frame_artifacts`.

## The unit

1. The importer writes one frame document per timeline frame (`STAGE-0001` … `STAGE-000N`), each
   inside the 512 KiB declaration budget, ordered by the frame's `order` row.
2. `planet-package::cellframe::resolve` binds a window to its named frame **and** to the sibling
   frames that address the same window, ordered; `SurfaceMount::frames_of(window)` answers the
   ordered list while `frame_of` keeps answering the primary frame, so every existing caller is
   unchanged.
3. `planet-client` gains a timeline: `Timeline::at(tick)` selects `tick % frames` in the movie's
   declared frame rate (40 fps for the imported movie), and both binaries tick — `inspect` renders
   chosen ticks to PPM so advancement is a file comparison, and `window` advances on its clock.
4. **No placeholder may fill a tick.** A frame with no art states its absences (`[absence]`) and a
   tick that has nothing to draw says so; it is never given a synthesised grid.

## Exit test

- `cargo test -q -p planet-package -p swf-import -p planet-client` exits 0 with a test that two ticks
  select two different frames from one mounted package.
- `inspect --render --ticks` writes pictures for tick 0 and a later tick, and the two differ (the
  distinct-colour counts and the pixel digests are printed).
- `python tools/port_gate.py check` prints `RESULT: PASS`.
- The user remains the witness that the window advances; no agent may claim playability.
