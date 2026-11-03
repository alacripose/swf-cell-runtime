# TICKET-0065 — The window's text was clipped away by an empty bounds rect and an unset viewport

- Owner: implementation agent
- Block: H9 client visibility
- Branch: `ticket/TICKET-0065-text-visibility`
- Ledger: LEDGER-0071
- Found by: the owner, who reported the window showed nothing

## What was wrong, twice

1. `render.rs` passed `TextBounds::default()` to glyphon's `TextArea`. In glyphon 0.6 that default is a
   zero-sized rectangle, so every glyph was clipped: the scene's eight lines were shaped and then never
   drawn.
2. `viewport.update(&queue, Resolution { .. })` was never called, so the text atlas viewport was unset.
   Even with correct bounds the pass would have had nothing to map glyphs into.

Both bugs were invisible to the headless tests: nothing in this repository can render glyphon text
without a device, so `present`, `draw`, `batch` and `picture` all passed while the window stayed blank.

## The fix

`State::new` and `State::resize` now update the viewport to the surface size, and `render` bounds the
text to the surface rather than to a zero rect.

## Evidence

- Before: a captured window showed the client's clear colour only, with the scene holding eight lines
  (confirmed by dumping the mounted scene).
- After: the same window draws those lines — surface id, digest prefix, the window and its frame
  statement, frame rate, tag classes, shape union, sprite counts and media totals.
- The scene's fourth line says the truth loudly: the stage window carries no frame, because no frame
  document exists yet. Text visibility is not drawing the movie.

## What this does not do

It does not lower anything, and it does not make the game appear: the picture pass still has an empty
batch until a frame document exists. It also adds no headless test for glyphon behaviour, because the
honest test needs a device; what is recorded is the owner's observation and the capture.

## Acceptance test

`cargo test -p planet-client` unchanged and passing (input 12, present 6, batch 7, picture 6, draw 5,
scene 6, adopt 12), `cargo build -p planet-client --bins`, the gate's selftest and check, and a fresh
checkout identity probe.

## Closing

- Commit: this unit's commit, fast-forwarded onto `master`; nothing pushed
- Shell gate state: unassigned
- Ledger entry: LEDGER-0071
