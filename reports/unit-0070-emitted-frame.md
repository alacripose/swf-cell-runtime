# Unit 0070 — the emitted frame, measured

Source-safe: no supplied movie, its bytes, identifiers, offsets or media appear here. The frame below
is the importer's own document, and the counts are the ones it declared.

| Command | Exit |
|---|---|
| `cargo test -p swf-import --test frame_doc` | 0 — 5 passed |
| `cargo test -p swf-import -p planet-package -p planet-client` | 0 |
| `swf-import <input> generated/swf1.pkg generated/mount-ledger.tsv swf1` | 0 — printed `frame generated/swf1.frame` |
| `python tools/port_gate.py selftest` | 0 — 123 of 123 |
| `python tools/port_gate.py check` | 0 — PASS |

## The frame as written

    shploingus.cellframe/1
    frame	STAGE-0001
    	window	stage
    	cols	80
    	rows	30
    	order	10
    	requires	none
    fill	0	0	80	surface
    run	0	0	text	STAGE

## What the client reported after mounting

`window stage — Stage`, `frame STAGE-0001 — 80 by 30, order 10`, `paper surface at (0, 0) for 80
cells`, `text: STAGE`, `raster 80 by 30 cells from STAGE-0001`, `picture 640 by 480 rgba8 (stride
2560)`, `batch 45 quads for STAGE-0001`.

## The refusal that named the field

The first emitted document put the grid in the window's second size pair. The client refused:

    cellframe-extent: frame STAGE-0001 is 80 by 30 and window stage has 640 by 480 of content space

The content space is the first size pair. Corrected there, and `tests/frame_doc.rs` now reads that
field rather than the one the emitter happened to write.

## Bounds

The grid is rounded up from the measured stage and clamped to the format's declared ceilings: 512
columns, 512 rows, 16,384 cells. A stage whose grid would exceed the cell ceiling has its rows
reduced rather than silently truncated, and the frame the writer returns has already been parsed by
the grammar's own reader.
