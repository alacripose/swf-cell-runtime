# TICKET-0079 — the movie's exact frame clock, in the package, with drawing independent of it

Status: `present` (implemented and tested headlessly; no client witness claimed)

## Problem

The client advanced the movie with `tick_at(seconds) = floor(seconds × fps)` — a float conversion per
call, no carried remainder, and a tick that could be derived only by re-measuring. Two defects follow:

1. **The clock was a float.** `floor(seconds × fps)` loses and re-earns sub-frame time on every call,
   and the family's `voxel-client/src/clock.rs` records the mould: seconds held as `f64` and divided
   by `1.0 / 20.0` came out **one tick short** at 3.5 s.
2. **The tick rate was coupled to loop participation.** A tick per loop turn (or per `Wait` wake) makes
   the movie's rate a function of process priority and compositor behaviour, which is exactly what the
   plan forbids: "Let `wgpu` render independently at the display's refresh cadence."

The plan is explicit and was being ignored (`docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §4.5):

> "The SWF frame rate is part of source semantics. Do not hardcode `ala-cities`' 20 Hz tick rate.
> Implement a deterministic scheduler that represents the movie's frame clock exactly (prefer
> rational/integer tick accumulation over floating-point drift), processes gameplay/timeline updates
> in source order, and lets `wgpu` render independently at the display's refresh cadence."

## Change

`crates/planet-package/src/cadence.rs` (new) owns the cadence:

- `FrameRate` — the source's fixed-point rate, `units / 256`, never a float.
- `TickStream::offer(nanos)` — one **opportunity** per displayed frame; elapsed nanoseconds become
  ticks by integer accumulation (`nanos × units` over `256 × 10⁹`), with the sub-frame remainder
  carried as an integer. `MAX_CATCH_UP = 4` bounds one offer; `dropped` counts what it refused.
- `Cadence` / `Horizons` — the family's rungs (`EveryTick`, `Water` ×4, `Sediment` ×20, `Daily`),
  due by `tick % period == 0`; `Script` is every tick (one movie frame, in source order); `Render` has
  no period and is never due, because drawing is not on the ladder.
- `MAX_CATCH_UP`, the carried remainder and counting a refusal are `shploingus-clean`'s
  `voxel-client/src/clock.rs`; the rung numbers are `voxel-shapers/src/stages.rs:97-104`. **The 20 Hz
  base is not taken**, per the plan.

`crates/planet-client/src/timeline.rs` and `render.rs`: the window offers one opportunity per redraw
with the measured elapsed nanoseconds and draws the frame the tick selects. `ControlFlow::Poll` keeps
the loop's rate the process's, not a frame clock's; the `Fifo` present mode remains the display's own
refresh. A draw never advances a tick.

## Refused

- A hardcoded 20 Hz base clock. The plan forbids it and the movie states 40 fps.
- A float tick computation. `f64` remains only for printing and for `interpolation()`.
- A second draw cadence constant invented in the client. The display's refresh already decides it.
- Claiming the movie plays. The window's output is evidence that the package draws; the user is the
  witness.

## Exit test

- `crates/planet-package/src/cadence.rs` unit tests: the ladder's numbers and nesting; 10 000
  opportunities in one second are 40 movie ticks with `dropped 0`; 60 offers and 10 offers of 100 ms
  reach the same 40 ticks; 35 offers of 100 ms at 40 fps are exactly 140 ticks with no remainder; a
  quarter frame four times is one tick; a 10 s offer is 4 ticks with 396 counted; a 120 Hz display
  sees frames `0..40` in order with none skipped; drawing is never due and advances nothing.
- `cargo test -q -p planet-package -p planet-client -p swf-import` exits 0.
- The window run prints `movie tick N frame ID ran [...] dropped 0 draw N of M`, showing draws
  outnumbering ticks.
- `python tools/port_gate.py check` reports PASS (recorded in the ledger row).
