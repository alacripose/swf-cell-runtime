# TICKET-0057 — The client's input router, as generic mechanics over the declared surface

- Owner: implementation agent (mechanics only; the running client stays the shell's and the user's)
- Block: H9 (repository work on the runtime contract)
- Branch: `ticket/TICKET-0057-input-router`
- Ledger: LEDGER-0063
- Depends on: TICKET-0048 (client adoption), TICKET-0052 (the drawn batch), TICKET-0055 (the LF pin)

## The ownership rule, quoted before any code

`docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §4.7, which the previous report dismissed in one
clause without quoting:

> The shell compiles source interaction semantics into one event graph owned by the cell surface …
> The cell runtime must preserve the source SWF's hit-area and button semantics. Focus, key repeat,
> pointer capture, dragging, wheel behavior, touch (when present), and event propagation are
> implemented in the client. Verification input is generated only by shell-declared tests or the user.
> Agents have no pointer/keyboard MCP, no CUA control, no arbitrary runtime-advance tool, and no ability
> to probe the game.

and §0.3:

> The agent-facing MCP surface is limited to schema discovery and ticket, secure-cell, and declared-test
> operations. It MUST NOT expose general observation or control of the running client.

Read together: the *router* is client code the plan asks for, and what is closed is the agent driving or
observing a **running** client. §9.2 closes the oracle and its traces, not the routing mechanics. So the
lane is open, exactly this wide: generic mechanics over the surface's declared geometry, fed by synthetic
events, with no probe, no live input, and no game binding invented.

## What this adds — `crates/planet-client/src/input.rs`

- `Event`: pointer moved/pressed/released/left, wheel, key pressed/released, text — the window layer's
  vocabulary, not the game's.
- `Routed`: `Nothing`, `Left`, `Hover { window, cell }`, `Press`, `Release { …, captured }`,
  `Wheel { …, delta }`, `Key { window, key, repeat }`, `Text { window, glyph }`. Every variant names a
  window id; none names a game action, because the action bindings are the shell's compilation.
- `Router::new(&Surface)` reads the declared window rectangles (`x`, `y`, `w`, `h` in cells) and the
  format's own `CELL_W`/`CELL_H` to map a pixel to a window and a cell. It re-parses nothing and
  re-measures no frame.
- Behaviour the plan names: **hit-test** by declared rectangle (not by declaration order); **focus** set
  by a press; **pointer capture** so a release outside still reaches the window the press landed on;
  **key repeat** reported from the held set rather than guessed; **wheel** routed to the window under
  the pointer; and a press on nothing moving no focus and capturing nothing.

## One repair, and it was mine

The first fixture put both `window` rows before the `node` row; the surface grammar attaches a node to
the window parsed immediately before it, so it refused `SurfaceNode { window: "main", node: "root" }`.
The code was right and the fixture was wrong — the node now sits under the window it names. Recorded
because it is exactly the kind of thing a "make the test pass" reflex would have hidden.

## What this does not do

- No game action binding, no key map, no hit-area semantics from the source game: those are the shell's
  compilation, and this module deliberately has no opinion about them.
- No live input, no probe, no window, no device. Every event in the tests is constructed in the test.
- Nothing in `planet-package` changes; `shploingus.cellframe/1`, `check_extent` and the load-time extent
  refusal are untouched.

## What stays open

- §6.2/§6.4 delivery, replacement and rollback: the shell alone executes those transactions.
- §4.8 persistence proper and migration: schema meaning and migration authority are the shell's.
- §9.2/§9.4 witness and parity oracle: shell-private evidence, redacted results only.
- XML lowering; and H9, which needs the owner's manual window run.

## Acceptance test

`cargo test -p planet-client --test input` — 8 scenarios: a pointer inside a window reports that window
and its cell; a pointer outside every window routes nothing and leaving is `Left`; two windows route by
the point, not by declaration order; a press focuses and captures so a release outside still reaches it;
a wheel goes to the window under the pointer with its delta; the second press of a held key is a repeat;
text and keys name the focused window and no window before a press; a press outside every window routes
nothing and keeps the focus.

Plus the unchanged suites (client present 6, batch 7, picture 6, draw 5, scene 6, adopt 12; package
9/21/19/16/8/6/22/13; save 12), `port_gate.py selftest` 123 of 123, `port_gate.py check` 21 checks with 4
not enforceable, and a fresh `git worktree` checkout whose record identity MATCHES with no fixture input
carrying CRLF.

## Closing

- Commit: this unit's commit, fast-forwarded onto `master`; nothing pushed
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0063
