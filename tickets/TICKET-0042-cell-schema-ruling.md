# TICKET-0042 — The cell schema questions are answered, and Q-5 stays open

- Owner: project owner (answers), implementation agent (record)
- Block: H1 declarations
- Branch: `ticket/TICKET-0042-cell-schema-ruling`
- Ledger: LEDGER-0048
- Depends on: `docs/CELL-SCHEMA.md` proposal, the owner's answers of 2026-11-03, and the trees the
  owner asked be read before answering Q-1

## What was wrong

Six questions in `docs/CELL-SCHEMA.md` were still open, so P6 (XML lowered into declared cells) could
not start, and the only thing the importer could do with a window definition was refuse it. The
questions had also been answered once from memory of the neighbouring trees, which is the drift the
checkpoint rule exists for: the owner asked what those trees actually say a cell is.

## What this records

Read from the sources, with paths:

- `openlock/shell/alpha-shell/frames/MAP-0001.frame` opens `shploingus.cellframe/1`: `frame MAP-0001`,
  `window map-plan`, `cols 56`, `rows 22`, `order 10`, `requires none`, `feature all`, two
  `field` bindings (`elevation terrain::elevation_m`, `biome ecology::biome_at`), `shaper profile32 32`,
  `projection orthographic`, `centre`, `cell 32`, `extent`, `pitch`, then `fill` and `run` rows whose
  paper and ink are roles.
- `openlock/shell/alpha-shell/package.cells` and `shploingus-clean/display-packages/biomes/package.cells`
  are two lines: `shploingus.cells/1` and `cells 1` — which drawing of the shell the package is a
  package of.
- `shploingus-clean/crates/voxel-package/src/cellframe.rs` names the frame's own bounds: a 512 KiB
  declaration, a 512 by 512 grid, 16 384 cells, and cells sized by the format.

Answers recorded in `docs/CELL-SCHEMA.md` under "What the owner ruled, 2026-11-03": Q-1 several row
kinds inside a frame (keys, field bindings, typed draw rows); Q-2 a typed array of values per cell;
Q-3 an open vocabulary streamed in a labelled section at the top of the declaration; Q-4 4 GiB per
package with 64-bit addressing, frame bounds retained; Q-6 cells in the first surface.

## What this does not do

**Q-5 is not ruled.** A binding naming a declared field id and nothing else remains a recommendation,
and the lowering may not depend on it. No frame grammar is implemented by this unit, no XML is
lowered, and the numeric containment bounds from
`docs/cellular_generativity_axis_civilian_expectations.md` §7.2 are not yet wired into
`crates/planet-client/src/xml.rs` (which is a parity format, not the Qt reader).

## Acceptance test

`python tools/port_gate.py check` exits 0 with `cell_schema` passing: every question keeps its
Recommendation, Rejected, Falsifier and Owner parts, the document still calls itself a proposal, no
port wire token appears, and the family-B raster appears only where it is rejected.

## Closing

- Commit: this unit's commit on `ticket/TICKET-0042-cell-schema-ruling` (unmerged; no remote configured)
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0048
