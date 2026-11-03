# UNIT-0058 - touch folded onto the pointer path, and the window loop wired to the router

Ticket `tickets/TICKET-0058-input-touch-and-wiring.md`, ledger `LEDGER-0064`, block H9.

## What changed

- `crates/planet-client/src/input.rs`: `Event::TouchDown`/`TouchMoved`/`TouchUp`, `Routed::Touch` with a
  `TouchPhase`, and `Router::window_cells()` reporting the declared rectangles the router hit-tests.
  Touch shares the pointer's state machine - down sets focus and capture, move is hit-tested through the
  capture, up releases it - and is still reported as a touch so a caller can tell a finger from a mouse.
- `crates/planet-client/src/render.rs`: the event loop forwards cursor, mouse-button, wheel, keyboard and
  touch events to a router and prints the routed shape; `run_with_input` takes a router and
  `run_with_batch` still runs without one.
- `crates/planet-client/src/bin/window.rs`: builds the router from the adopted surface's declared window
  rectangles and hands it to the window, so the run prints what each event routed to.
- `crates/planet-client/tests/input.rs`: four touch scenarios (down focuses and captures; a move that
  leaves the window keeps the capture and reports no cell; a touch on nothing names no window and moves no
  focus; a mouse capture also carries a touch move).

## What did not change

- No game binding, key map, hit area or node rectangle is invented. Node-level propagation stays
  unimplemented because nodes declare no geometry and drawn rectangles carry no node identity.
- `planet-package` is untouched; no format, schema or vocabulary changes.
- No window was opened by a test, and no test produces input from a live client.

## Checks

- `cargo test -p planet-client` - input 12, present 6, batch 7, picture 6, draw 5, scene 6, adopt 12, all
  passing; one pre-existing `atlas_cache` dead-field warning in `render.rs`.
- `cargo build -p planet-client --bins` - both binaries build.
- `python tools/port_gate.py selftest` and `check` - see UNIT-0059's record for the numbers at the commit.
