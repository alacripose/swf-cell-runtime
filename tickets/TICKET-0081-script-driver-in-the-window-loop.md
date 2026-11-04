# TICKET-0081 — the script driver: a compiled program reaching the window loop

Status: `present` (driver, tests, window wiring with no program). Carrying a program into a mount is
`blocked` on the shell's ruling (`AGENTS.md` §12).

## Problem

TICKET-0080 gave the runtime a reader and a pure executor for a compiled cell program, and nothing
called it. The window loop chose a frame from the clock alone, so a program's `goto`, `stop`, `play`
and `sound` could never reach a viewer, and a tick moved by a script was indistinguishable from one
the clock earned.

## Change

`crates/planet-client/src/script.rs`:

- `Driver::new(Option<Program>)` and `Driver::run(due_tick) -> Step`, where a `Step` states the tick
  the timeline shows, whether it is playing, the sound characters announced on that tick, and whether
  a program ran at all.
- **No program is stated as no program.** The driver shows the clock's tick, plays, announces nothing,
  and reports `scripted: false`. It never invents a goto, a stop or a sound.
- **A stop freezes the shown tick, not the clock.** The clock keeps earning ticks; the driver runs the
  program from the shown tick while stopped, so a later `play` resumes where the movie stopped rather
  than jumping to wall time.
- **A refused program is reported, not guessed at.** A refusal or a budget overrun sets `refused()`
  with the refusal's code and message, and the driver falls back to the clock's tick.

`crates/planet-client/src/render.rs` runs the driver on the tick the clock earned, before choosing a
carried frame, and prints `sounds [...]` when a tick announces one, plus `scripted` and `playing` on
the frame line. `run_with_timeline` takes `Option<Program>`; `crates/planet-client/src/bin/window.rs`
passes `None` and prints `script none: no admitted code frame in the package`.

## Refused

- Admitting a carried `code` frame at mount. `AGENTS.md` §12 still refuses it `frame-kind-reserved`;
  the driver is ready for a program and the window states that it has none.
- Inventing a program for the movie. A hand-written program standing in for the shell's compilation
  would be a second, uncompiled script and would claim behaviour the package does not carry.
- Driving the audio channel from a `sound` announcement. The driver reports the character; starting a
  clip needs the package's audio table and a `Channel`, which is the next unit.

## Exit test

- `cargo test -p planet-client` passes, including six `script` tests: no program is unscripted, a
  `goto` moves the shown tick off the clock, a `stop` freezes the shown tick while the clock runs,
  `play` resumes from the stopped tick, a sound is announced on the tick that starts it and not after,
  and the same ticks produce the same steps twice.
- `cargo build --bins` exits 0 with the window on the new signature.
- The window still prints `script none` per run: the honest state until admission is ruled on.
