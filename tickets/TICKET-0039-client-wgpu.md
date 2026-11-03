# TICKET-0039 — The winit + wgpu client draws the adopted package

- Owner: implementation agent
- Block: H9 client surface
- Branch: `ticket/TICKET-0039-client-wgpu`
- Ledger: LEDGER-0045
- Depends on: TICKET-0036 (`CellMount::adopt`), TICKET-0038 (sealing), the owner's ruling that the
  client is one new binary reading the newest package in `generated/`

## What was missing

The repository had a mount that adopts a package or refuses it whole, and no window. Nothing could
show that an adopted surface reaches a screen, so every claim about the client stopped at a test.

## What this builds

`crates/planet-client/src/bin/window.rs`, one binary:

- it lists `generated/` for `.pkg` files and takes the newest by `(mtime, name)`;
- it reads those bytes and hands them to `planet_client::CellMount::adopt`, so a refusal is the same
  refusal the mount produces, printed with its code and message instead of a blank window;
- the window title is the adopted surface id;
- the body is the first window's `label` and `table` nodes, drawn with glyphon over a wgpu 22 surface
  (`Cache`, `TextAtlas`, `TextRenderer::prepare` with the font system).

Dependencies added to `crates/planet-client`: `winit 0.30`, `wgpu 22`, `glyphon 0.6`, `pollster 0.3`.
Glyphon pulls `cosmic-text`; the text path and the swap chain are the only drawing code.

## What this does not do

It does not play the game, route input, or claim gameplay parity. It draws one window of labels from
a mounted package and nothing else; the taskbar, the cells-as-XML lowering and the second client
branch are separate tickets. It does not seal or unseal: the read path still accepts the unsealed
package the importer emits, which is the wiring unit after this one.

## Acceptance test

- `cargo build -p planet-client --bin window` exits 0.
- `./target/debug/window` stays up for 12 seconds with an empty log while a window is open, then is
  stopped by its recorded pid.
- The drawn text is `adopted()` output: the package path and digest prefix, the surface id, and the
  label nodes of the first window.

The user is the witness of the window itself. This record does not claim the client plays anything.

## Closing

- Commit: this unit's commit on `ticket/TICKET-0039-client-wgpu` (unmerged; no remote is configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0045
