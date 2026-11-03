# TICKET-0054 — The client presents a frame's batch through the one surface

- Owner: implementation agent
- Block: H9 client surface (presentation attempt; the witness stays with the shell and the user)
- Branch: `ticket/TICKET-0054-present`
- Ledger: LEDGER-0060
- Depends on: TICKET-0053 (trunk and record), TICKET-0052 (draw batch), TICKET-0051 (picture)

## Baseline, proven by running it first

- `git`: branch `master`, HEAD `afe412f`, `master` equal to it, working tree clean, `05e4586`,
  `0f2a108` and `afe412f` all reachable from the trunk.
- `cargo test -p planet-package -p planet-client` exited 0 at batch 7, picture 6, draw 5, scene 6,
  client adopt 12, binding 21, cellframe 19, surface 22, vocabulary 13.
- `python tools/port_gate.py check` PASS (21 checks, 4 not enforceable); `selftest` PASS (123 of 123).

## The decision this unit follows

Asked to choose, the project owner chose: attempt GPU presentation behind a headless-safe boundary,
allow a window on manual runs only, keep tests and the gate headless, and pin LF for the gate's fixture
inputs. This unit is the presentation attempt. The LF pinning is **not** in this commit: it
renormalises the byte form of roughly ninety tracked fixture inputs, which the 40-file commit cap turns
into three commits followed by a record regeneration, so it is its own unit with its own ticket rather
than a footnote to a rendering change.

## What this adds

**`crates/planet-client/src/present.rs`** — the boundary, and the whole of it is pure data:
`Instance { rect: [f32; 4], colour: [f32; 4] }`, `instances(&[Quad], (u32, u32)) -> Vec<Instance>`,
`colour(Rgba) -> [f32; 4]` and `bytes(&[Instance]) -> Vec<u8>`. Pixel rectangles become clip-space
instances by an exact rule (`x' = 2x/w - 1`, `y' = 1 - 2y/h`, `height` carried negative because the
picture's origin is top left and clip space grows upwards), colours are each palette byte over 255
with no gamma, premultiplication or colour-space claim, and the byte layout is the one the vertex
buffer declares: eight little-endian floats, `rect` then `colour`, 32 bytes per instance. Order is the
batch's order. A hole produces no instance, because a hole is what the canvas already is.

**`crates/planet-client/src/render.rs`** — the one renderer gains a quad pass inside the same render
pass and surface as its text: a small WGSL shader (`vertex`/`fragment`), an instance-step vertex
buffer, alpha blending, the batch drawn before the text. `run_with_batch(scene, quads)` is the entry
point that carries the frame's rectangles; `run(scene)` still exists and presents an empty batch. The
projection is recomputed on every resize, so a resize never draws a stale projection.

**`crates/planet-client/src/bin/window.rs`** — collects each drawn frame's batch and hands it to
`run_with_batch`, so the window the package opens shows the picture it drew, with the scene's text over
it.

## What is proved, and what is only compiled

`cargo test` proves the boundary completely and headlessly: the projection is exact and reversible,
the origin and the `y` flip are right, the colour rule is the palette's bytes over 255, the bytes are
the declared layout, a mounted package presents exactly the batch it drew in order, and a hole
presents nothing.

The upload and the draw are **not** proved here: they need a real adapter and a visible surface. This
repository's tests and the gate never construct a device, and nothing in this unit claims the client
mounts or plays — H9 stays with the shell and the user, who run the binary by hand.

## Codes

No `cellframe-*` code was minted, and presentation has no refusal of its own: it consumes the batch
that `batch::plan` already refuses-clean, so there is nothing left for it to refuse.

## What stays open

- Running the window by hand (the owner's), and the H9 on-screen witness (the shell's and the user's).
- The approved LF pinning for the gate's fixture inputs — its own unit, three commits under the cap.
- §4.7 input hit areas, §6.2/§6.4 shell transactions, §4.8 persistence, §9.2/§9.4 witness, XML
  lowering, and `planet-package`'s freedom from colour, which this unit does not touch.

## Acceptance test

`cargo test -p planet-package -p planet-client` exits 0 with present 6, batch 7, picture 6, draw 5,
scene 6, adopt 12, binding 21, cellframe 19, surface 22, vocabulary 13; `port_gate.py selftest` exits
0 (123 of 123); `port_gate.py check` exits 0 (21 checks, 4 not enforceable).

## Closing

- Commit: this unit's commit, fast-forwarded onto `master`
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0060
