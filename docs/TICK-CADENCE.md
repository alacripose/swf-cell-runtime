# The tick cadence: the movie's clock, the package's arithmetic, the display's drawing

Status: `present`. Implemented in `crates/planet-package/src/cadence.rs`, used by
`crates/planet-client/src/timeline.rs` and the window binary. The window has not yet been witnessed
drawing a moving picture by the user (`client_witness` stays `NOT-ENFORCED`, AGENTS.md §7).

## The rule this implements

`docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §4.5:

> "The SWF frame rate is part of source semantics. **Do not hardcode `ala-cities`' 20 Hz tick rate.**
> Implement a deterministic scheduler that represents the movie's frame clock exactly (prefer
> rational/integer tick accumulation over floating-point drift), processes gameplay/timeline updates
> in source order, and lets `wgpu` render independently at the display's refresh cadence."

Three consequences, each implemented rather than described:

1. **No 20 Hz.** There is no base rate in the clock. `MAX_CATCH_UP = 4` and the carried fraction come
   from `shploingus-clean/crates/voxel-client/src/clock.rs`, but its `TICK_HZ` does not: this movie's
   rate is the source's own.
2. **Exact integer accumulation.** A SWF header states frames per second as `u16 / 256`. Elapsed time
   arrives as whole nanoseconds. The stream carries `nanos × units` in a `u128` and divides by
   `256 × 10⁹`, so three and a half seconds at 40 fps is exactly 140 ticks. No `f64` enters the
   arithmetic, so nothing rounds and nothing drifts.
3. **Drawing is independent.** `Horizon::Render` has no period. `wgpu` draws at the display's refresh
   cadence (the window is `PresentMode::Fifo`); a draw offers an opportunity and samples the movie
   tick, and it never advances one. A 120 Hz display over a 40 fps movie redraws each frame three
   times; it does not drop or invent a movie frame.

## The stream is bounded by the process, not by a frame clock

The client offers one opportunity per displayed frame. The rate of those offers is whatever the OS
gives the window process — a fast, unpinned process offers a great many, a deprioritised one offers
few, and **neither changes the movie's rate**, because the rate is decided by elapsed time inside the
package manager:

| the process does | the movie does |
| --- | --- |
| 10 000 offers in one second (0.1 ms each) | 40 ticks at 40 fps, `dropped 0` |
| 60 offers in one second | 40 ticks, `dropped 0` |
| 10 offers of 100 ms | 4 ticks each, `dropped 0` |
| one offer carrying 10 s | 4 ticks (the cap), `dropped 396`, counted |

That last row is the point of the cap: a stalled process shows the movie **late**, never fast, and
the ticks it could not carry are counted rather than silently discarded. Because due-ness is
`tick % period == 0` (the family's one-line rule, `voxel-shapers/src/stages.rs:106-118`), starvation
can only delay a rung, never rephase it.

## The horizons

| horizon | period | where it comes from |
| --- | --- | --- |
| `Render` | none — the display's refresh | plan §4.5, "render independently at the display's refresh cadence" |
| `Script` | every tick (one movie frame) | the movie's own frame clock, source order |
| `Logic` | `Water` = 4 ticks | family rung, `voxel-shapers/src/stages.rs:97-104` |
| `Physics` | `Sediment` = 20 ticks | family rung, same |
| (`Daily`) | `TICKS_PER_DAY` | family rung; the day is the unscaled real day here because this package layer has no `REAL_METRES_PER_VOXEL` (`climate.rs:49-50`) — a named difference |

The rungs are the family's and the rule that reads them is the family's. **The assignment of `Logic`
to `Water` and `Physics` to `Sediment` is this port's declaration**, and the base they count in is the
movie's ticks, not the family's 20 Hz — the plan forbids the latter.

## Measured in this tree

`cargo test -q -p planet-package -p planet-client -p swf-import` exits 0, including the twelve tests
in `cadence.rs`. The window run (7 s, killed by the timeout, log kept in `generated/window-cadence.log`
and untracked) printed:

```text
timeline 12 frames at 40 fps (10240 rate units, fixed point), one tick per movie frame, 25000000 ns a
  frame; the window draws at the display's refresh
frames 12 planned into 12 batches
movie tick 1 frame STAGE-0002 ran [script] dropped 0 draw 1 of 1
movie tick 4 frame STAGE-0005 ran [script,logic] dropped 0 draw 16 of 16
movie tick 12 frame STAGE-0001 ran [script,logic] dropped 0 draw 64 of 64
movie tick 20 frame STAGE-0009 ran [script,logic,physics] dropped 0 draw ... of ...
```

183 movie ticks in the run, all twelve carried frames cycling, `dropped 0`, and **more draws than
ticks** (64 draws by movie tick 12): the window is drawing at the display's rate over a 40 fps movie.
`logic` ran on ticks 4, 8, 12, 16, 20 and `physics` on 20 — the family's rungs, on the movie's ticks.

**This is evidence that the package draws, not that the game is witnessed.** No window has been
observed by the user for this run; `client_witness` remains `NOT-ENFORCED` (AGENTS.md §7).

## What is absent, named

- No animation code executes. `Logic` and `Physics` are scheduled and reported; there is no logic or
  physics system in this port yet to ask (`absent`, not implemented).
- No audio synchronisation: the timeline carries `[event]` sound rows, but no mixer consumes them.
- The window's drawn output is not a client witness. A run prints ticks, frames and batches; whether
  the game plays is the user's observation (AGENTS.md §7).
