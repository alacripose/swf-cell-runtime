# TICKET-0076 — This port is the ingest module; the client surface is v2's, and the ceiling is owed

## The problem this ticket records

Two claims this port was working from are wrong, and both are settled by the ruling record the owner
already made on 2026-11-02 (`secure-shell-audit/`, read in full for this ticket):

1. **`testport1` is not the runtime owner.** `secure-shell-audit/testport1_plan_resynthesis.md` §1:
   "`testport1` is the **ingest module of secure-shell v2**, not the runtime owner (Q16, Q17)", and
   its ownership table gives up `winit`/`wgpu` as a product surface in favour of a Qt 6 client
   (Core, Gui, Quick/QML, Network, Xml). `cell-schema-ruling.md` repeats it: "It does not make
   `testport1` the runtime owner: the client remains the Qt 6 shell in v2."
2. **The 512 KiB / 16 384 pair is not the format's ceiling.** `cell-schema-ruling.md` Q-4: "**Not the
   512 KiB / 16 384 pair.** The surface is stateful and real-time, the addressing is **64-bit**, and
   the geometry reaches **subvoxel** scale. The ceiling must admit that class of surface. Exact
   numbers still to be given."

## What follows

- `planet-client`'s renderer (`render.rs`, `present.rs`, `batch.rs`, `picture.rs`, `draw.rs`,
  `font.rs`) is **evidence that the frame is adoptable**, not a surface to grow. A unit that makes
  it prettier — a second raster, a linear-space compositor, a family-shaped `cells.rs` — is a legacy
  unit, and this ticket refuses it rather than doing it quietly.
- The family's renderer (`shploingus-clean/crates/voxel-client/src/shell/cells.rs` and
  `voxel-client/src/voxel-ui`) is the **design v2 inherits**; it is cited, not ported again here.
- The ceiling constants (`MAX_FRAME_BYTES = 512 * 1024`, `MAX_CELLS = 16_384` in
  `crates/planet-package/src/cellframe.rs`, same numbers in
  `shploingus-clean/crates/voxel-package/src/cellframe.rs`) are **implemented floors** with the ruled
  numbers **owed by the format owner**. A refusal that quotes them says "the current constant", never
  "the format's ceiling".
- Ingest stays this port's job, and it is not finished: the timeline carriage (`TICKET-0075`) matters
  whoever renders it, because a package that ships one frame of twelve is incomplete ingest.

## Acceptance evidence

- `docs/OWNERSHIP-AUDIT-2026-11-04-family-branches-and-v2.md` records the branch inventory of every
  family checkout, the two ruling quotations, and what each changes here.
- `docs/CELL-SCHEMA.md` no longer presents 512 KiB / 16 384 as the ruled ceiling; it quotes Q-4 and
  names the numbers as owed.
- `python tools/port_gate.py check` prints `RESULT: PASS`.
- `cargo test -q -p planet-package -p swf-import -p planet-client` exits 0.

## What this ticket does not do

It does not delete `planet-client`, does not claim v2's client exists, does not supply the owed
ceiling numbers, and does not claim the game plays. H9 stays unclaimed until the shell or the user
witnesses it.
