# TICKET-0078 — Every proof in this port is headless, and the game has never drawn a window

## The failure, stated plainly

The work that carries the timeline, the interim ceiling and the sound events proves things about
**the package** and about **the client's draw path**. It does not prove the game draws anything.

- `crates/planet-client/src/bin/inspect.rs` adopts, mounts, draws and paints — into a buffer. It
  creates no window and presents no surface.
- `crates/swf-import/tests/end_to_end.rs` runs the importer on the real movie and paints all twelve
  frames through `planet_client::draw` and `picture` — into memory.
- The window binary (`planet-client --bin window`) exists and mounts the newest package, but no run
  of it has been observed drawing the imported package on a screen. A process that exits 0 is not a
  witness; a screenshot of a white buffer is not the game.

`docs/PORT-BASELINE.md` H9 says the same thing in the baseline's own words: the client is the test
surface, a human runs it and reports what appears, and H9 stays unclaimed until then. Every ledger
row in this port carries `shell_gate_state: unassigned` for that reason.

## What this ticket changes

1. **The headless limit is printed where the proof runs.** `inspect --render` prints
   `headless true window never opened — evidence the package draws, not a client witness`, and the
   end-to-end test prints the same class of line before it asserts anything. A reader of the output
   cannot mistake it for a window run.
2. **This ticket and its ledger row record the attempt and the outcome of an actual window run**, so
   "no witness" is a measured fact rather than an assumption: whatever the window binary printed on
   this machine is quoted in the row.
3. **No claim moves.** H9 stays unclaimed, `client_witness` stays a `NOT-ENFORCED` row, and no agent
   message, test name or ledger `status` may say the game plays.

## What would close it

A shell or user run of the window client against the imported package, with what appeared on screen
reported back — including "it showed a white rectangle", which is a true and useful witness. The
package's own limit is separate and known: the movie's scene is 18 bitmap characters (258 207 bytes)
this importer does not decode, and a white cover character sits on top of the stage, so the expected
witness today is a white window, not the game.

## Acceptance

- `inspect --render` prints the headless line, and the end-to-end test prints it.
- The ledger row quotes the window binary's actual output on this machine.
- `cargo test -q -p planet-package -p swf-import -p planet-client` exits 0.
- `python tools/port_gate.py check` prints `RESULT: PASS`.
- The user remains the only witness for H9.
