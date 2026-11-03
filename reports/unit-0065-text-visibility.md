# UNIT-0065 - text visibility

Ticket `tickets/TICKET-0065-text-visibility.md`, ledger `LEDGER-0071`, block H9.

## The two bugs

`TextBounds::default()` is a zero-sized clip rectangle in glyphon 0.6, so all text was clipped; and
`Viewport::update` was never called, so the atlas viewport was unset. Either alone is enough to show a
blank window over the correct clear colour, which is exactly what the owner saw.

## Evidence

A dump of the mounted scene showed eight lines and 360 bytes of text, so the scene was never empty. After
the fix a desktop capture of the window shows those eight lines drawn in white over the clear colour.

The fourth line is the honest summary of the current state: the stage window carries no frame, because
the importer has never emitted a frame document. Text visibility is not the game appearing.

## Checks

`cargo test -p planet-client` (input 12, present 6, batch 7, picture 6, draw 5, scene 6, adopt 12),
`cargo build -p planet-client --bins`, `port_gate.py selftest` and `check`, fresh-checkout identity probe:
statuses in LEDGER-0071.
