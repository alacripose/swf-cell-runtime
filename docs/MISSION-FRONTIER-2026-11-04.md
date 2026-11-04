# Mission frontier — 2026-11-04

Status words are the implementation facts `AGENTS.md` §3 allows: `present`, `not-implemented`,
`blocked`. They are never gate states.

## What is present

| axis | artifact | evidence |
| --- | --- | --- |
| Package content → cells | `crates/swf-import`, `crates/planet-package` | frames differ per movie frame; `frame_paint` counts recorded in `generated/import.log` |
| Cadence | `crates/planet-package/src/cadence.rs` | 12 tests, exact integer clock, `docs/TICK-CADENCE.md` |
| Code lane reader + executor | `crates/planet-package/src/program.rs` | 12 tests on synthetic programs, `docs/CELL-PROGRAM.md` |
| Script driver in the window loop | `crates/planet-client/src/script.rs` | 6 tests; window prints `script none` because no program is admitted |
| Refusal discipline | `tools/port_gate.py`, `tools/wait_run.py`, `AGENTS.md` §15 | `RESULT: REFUSED (selftest did not run; 0 scenarios)`; liveness-checked waits |
| Health ratchet | `tools/health.py`, `artifacts/health/state.json`, `reports/health/` | per-unit reports, proposals, `RESULT: PASS (… 0 debt(s) open …)` |

## What is not implemented, and what blocks it

| axis | state | blocker |
| --- | --- | --- |
| Bitmap fills as content | `not-implemented` | `shape.rs` returns a grey for `Fill::Bitmap`; decoding needs a decision on the media lane |
| Frame sounds start on their tick | `planet_client::playback::Score` | 7 tests; a frame's carried clips are queued on the tick that shows it; no audio device is opened |
| Sound device output | `not-implemented` | the channel renders and counts samples; handing them to hardware needs a device dependency and a shell-declared run |
| Code frame admission | `blocked` | `AGENTS.md` §12 refuses `frame-kind-reserved`; admission is the shell's ruling |
| Client axis | `not-implemented` | no address space for tick/cell/sprite a reviewer or bot can point at (questions 101–130) |
| Civilians and wishes format | `not-implemented` | ownership undecided: this repo, the shell, or the civilian system (questions 131–155) |
| CUA driver node bot control proof | `not-implemented` | no node-side client, no headless witness path (questions 156–180) |
| Secure shell v2 civilian system 1 model | `blocked` | upstream dependency; what this example must prove is not yet written down (questions 181–192) |
| Legacy formats and other games | `not-implemented` | only the one movie is authorized under `AGENTS.md` §1.1; fixtures for other games need a ruling (questions 193–206) |
| Drag-and-drop importer | `not-implemented` | no entry surface; copy/refuse rules for a dropped file are undefined (questions 202–204) |
| MCP surface | `not-implemented` | `mcp_surface` check passes by naming the rule; the surface itself is absent (questions 207–216) |
| Training-format research refusal | `not-implemented` | no ratchet line requires a research record before a format is adopted (questions 217–220) |

## The one thing that unlocks the most

The movie's own behaviour. Every remaining visual question — the white cover, sprite frames beyond the
movie's twelve, sounds timed by scripts — reduces to "the movie's scripts decide it". The lane that
would carry them (`code`) is read and executed by this repository and still refused at mount by
`AGENTS.md` §12. Admission is a ruling, not an implementation: it needs a schema addendum, a
documented executor contract, and the shell's decision that code frames may travel in a mounted
package.

Until that ruling, the honest statement is: the client shows the package's frames over the movie's
own cadence, and no script of the movie runs.

## Ordering that does not waste a turn

1. Sound handoff (driver announcement → channel) — no ruling needed, proves a second content kind.
2. Client axis in read-only form (tick, frame id, cells, sounds, refusals) — the substrate every bot
   and reviewer question depends on.
3. Bitmap fills as real content — removes the largest remaining visual lie.
4. Code-frame admission — after the shell's ruling, then scripts select frames and time sounds.
5. Civilians/wishes format and the CUA bot proof — only meaningful once the axis exists.
