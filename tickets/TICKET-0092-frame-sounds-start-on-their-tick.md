# TICKET-0092 — a carried frame's sounds start on the tick that shows it

## Problem

`LEDGER-0095` proved the package carries the movie's bitmaps. It also carries its audio: the mount
resolves each timeline frame's `[event]` rows into that frame's own `audio` list, and the client's
`audio` module decodes a clip into real samples. Nothing joined the two. `inspect` decoded a frame's
clips for a report and threw the channel away; the window decoded nothing at all. A mounted movie was
therefore silent however many payloads it carried, and the frontier page said so: "driver announces
characters; nothing hands them to `audio.rs`'s channel".

## Requirement

- `planet_client::playback::Score` — every carried frame's clips decoded **once**, queued into the
  client's own `Channel` on the tick the frame is shown. A frame starts its sounds when the shown
  frame changes, not on every redraw; a wrap back to a frame is a new visit and starts again.
- A frame carrying no clips starts with an empty list rather than a fabricated sound, and an index the
  score does not carry starts nothing at all.
- `Score::DEVICE` states the limit in the line that reports a start: no audio device is opened,
  samples are rendered and counted.
- The window loop advances the score when the shown frame changes and prints
  `sound_start tick … frame … sounds […] samples … device … played_samples …`.
- `inspect --evidence` prints one `sound_start` line per starting frame, renders that frame's samples
  and prints the peak, and ends with `sound_score clips … unreadable … starts … device …`.

## Acceptance evidence

- `cargo test -p planet-client --lib` passes, including seven `playback` scenarios (a start queues its
  samples; a redraw starts nothing; a wrap starts again; an empty frame starts with an empty list; an
  index outside the score starts nothing; rendering a started frame yields samples and a peak; the
  same visits produce the same starts).
- `cargo build -p planet-client --bins` exits 0.
- `inspect --evidence generated/swf1.pkg artifacts/evidence` prints `sound_start` lines naming the
  frame and the sound characters, and the closing `sound_score` line.
- `port_gate.py check`, `port_gate.py selftest` and `health.py` pass.
