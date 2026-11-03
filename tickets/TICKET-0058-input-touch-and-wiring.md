# TICKET-0058 — Touch folded onto the pointer path, and the window loop wired to the router

- Owner: implementation agent (mechanics only)
- Block: H9
- Branch: `ticket/TICKET-0058-input-touch-and-wiring`
- Ledger: LEDGER-0064
- Depends on: TICKET-0057 (the router)

## Ownership, quoted

§4.7: *"Focus, key repeat, pointer capture, dragging, wheel behavior, touch (when present), and event
propagation are implemented in the client."* Touch is named in that list, so it is the client's. What
stays closed is *"Agents have no pointer/keyboard MCP, no CUA control, no arbitrary runtime-advance tool,
and no ability to probe the game"* — so this unit is mechanics and a forwarding loop, and nothing here
drives or observes a running client.

## What this adds

1. **Touch, folded onto the pointer path.** `Event::TouchDown`/`TouchMoved`/`TouchUp` and
   `Routed::Touch { window, cell, id, phase }`. A touch down sets focus and capture exactly as a press
   does, a move is hit-tested through the same capture, and a touch up ends it. There is one state
   machine, not two, and the routed shape still says `Touch` so the caller can tell a finger from a
   mouse without re-deriving geometry.
2. **The window loop forwards events.** `render::run_with_input(scene, quads, router)` builds the same
   `App` as before and adds one method that translates winit's vocabulary — cursor moved/left, mouse
   button, wheel (line and pixel deltas), logical keys, text and touch phases — into the router, printing
   the routed shape. `run_with_batch` is unchanged and still runs without a router.
3. **The binary hands the router over.** `bin/window.rs` builds `Router::new(&adopted.surface)` from the
   declared window rectangles before the adopted surface moves into the scene, and calls
   `run_with_input`; every failure path still runs with no router.

## What this deliberately does not do

- **No node-level propagation.** Nodes declare `id`, `parent`, `kind`, `role`, `value` and no geometry,
  and the frame's drawn rectangles carry no node identity. Deriving node rectangles from the quads was
  the preferred route and is not available: there is nothing in a quad that names a node. Routing a press
  into the node tree would mean inventing the rectangle it lands in, so it stays blocked and is recorded
  as such in `docs/CAMPAIGN-FRONTIER.md`.
- **No drag that mutates layout.** The `moves` field stays a declaration; the router reports.
- **No chrome hit areas.** `chrome` declares a control, not a rectangle.
- **No key map, no game binding, no source hit area.** Those are the shell's compilation.

## Acceptance test

`cargo test -p planet-client --test input` — the eight scenarios of TICKET-0057 plus four touch
scenarios: a touch down focuses and captures, and its move survives leaving the window; a touch outside
every window names no window and moves no focus; a touch move without a down still reports the window
under it; a mouse capture also carries a touch move.

`cargo build -p planet-client --bins` builds both binaries.

## Closing

- Commit: `43463fb`, which carries this unit together with TICKET-0059; the first commit for this unit
  was refused by the pre-commit gate and its staged files were swept into that one. Recorded as
  TICKET-0061 / LEDGER-0067 rather than rewritten.
- Push: none - the repository has no configured remote
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0064
