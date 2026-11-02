# DECISIONS-PORT — the four rulings, and exactly how far each one reaches

**Recorded:** 2026-11-02. **Source of authority:** the project owner, answering the four questions the
errata left open (`docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md`, errata E-1–E-4).
**Status of everything here:** decided, **not implemented**. No client, no parser, and no executor
exists in this repository; nothing below is a claim that anything runs.

A ruling answers *what the port must do*. It does not create code, and it does not close an H block.
Each entry therefore names what it settles, what it explicitly does not, and what stays blocked.

---

## D-1 — a cell is a declared display unit over data, not a character grid

**Ruling.** The port's cells are not grids. In the measured trees a cell mechanism stands **in place
of a data renderer** — `planet-shploingus` has a map renderer, `openlock` has an XML renderer that
draws frames over game windows. The port's cells are the same class of thing: a declared unit of
display bound to data, so a surface can be described rather than requiring a bespoke renderer.

**Supersedes, and why.** `docs/PACKAGE-OWNERSHIP.md` §6 previously offered "a new family-A `Kind`
carrying a cell raster with family B's semantics, or no cell raster at all". That framing was **wrong**
and is superseded: it imported `shploingus.cellframe/1`'s 8 × 16 paper/ink grid as though the grid
were what "cell" means. The grid is *family B's renderer choice*, not the concept. Family B remains
evidence for how a declared display format is bounded and refused — ceilings, required keys, role
rather than colour, window-bounded extents, empty refused, all problems reported at once — but its
raster is not the port's model.

**What the plan already said, and this restores.** §3.1's `Cell frames` row described cell frames as
"typed cell data, field bindings, shapers, projections, ordering, and dependencies". That is a
declared display over data. The erratum's narrowing to a raster contradicted the plan's own intent;
this ruling restores it.

**Settles:** the model — declarations bound to data, interpreted generically, replacing per-data-type
renderers.
**Does not settle:** the schema. The `Kind` name, field layout, and the shaper/projection vocabulary
are still undecided and need a format-owner ticket. Nothing here authorises implementing one.
**Consequence:** the port does not adopt `shploingus.cells/1` or `shploingus.cellframe/1` as its
format (PATCH-0005), and does not grow a paper/ink grid "because the other tree had one".

## D-2 — media travel as per-type kinds

**Ruling.** Bitmaps, fonts and audio each get their own `Kind` (one field each), rather than a single
media kind with a type tag, and rather than raw bytes in `bands`/`materials`.

**Settles:** the shape of media carriage — one kind per class, each a row in `rows.rs` plus a line in
the format document, `KINDS_ARE_TOTAL` covering them.
**Does not settle:** field contents for each kind (encoding, indexing, colour space, loop points).
**Consequence:** three format-owner lines instead of one, and a refusal that can name the media class
it rejected. No asset archive is introduced; `SHPKG003`'s artifact table stays unadopted.

## D-3 — the reserved code kind executes a bounded deterministic cell program

**Ruling.** A code frame carries a cell program executed under a closed instruction set with declared
budgets: instruction and memory limits, recursion/stack limits, cancellation, capability checks, and
deterministic failure reporting. It runs no host, filesystem, network, or native code.

**Settles:** the class of executor — plan §3.3's own design, now ruled rather than proposed.
**Does not settle:** the op-code set, the encoding, the capability vocabulary, or the budget numbers.
Those belong in `docs/CELL-PROGRAM-DECISION.md` (H5), which does not exist yet.
**Consequence:** the frame reader for code already exists (`FrameKind::Code`, `--code`); the executor
does not, and `planet-shell` refuses code frames on adoption (`mount.rs:111`). Parity remains
unclaimed until an executor exists and is witnessed.

## D-4 — a streamed transport is added, as a new layer

**Ruling.** The port gains a streamed transport for package delivery. It is a new layer with its own
decisions, and it is **not** "mount-v2", because no mount-v2, `surface.mount/2`, or receipt type
exists in either family.

**Settles:** that a transport beyond the watched file is in scope, and that its identity is new.
**Does not settle:** framing (reuse `V4PKG001` frames as chunks, or a declared envelope), ordering and
resume semantics, whether it carries a receipt separate from the door admission, or how a receiver
proves it applied a revision exactly once. Each needs a ticket and a decision record.
**Consequence, and the line that must not be crossed:** the transport may not become a second package
family, a second mount store, or a second content schema. It moves canonical package bytes; it does
not define them. Today the measured route remains `Mount::watch` over a directory plus the package-door
admission, and the ledger row that admission leaves.

---

## What these rulings do not do

- They do not implement anything. Every affected row in `docs/SCOPE-ACCOUNTING.md` reads
  `not-implemented`, not `present`.
- They do not close H5, H6–H10, or the shell-only blocks: H4 ingestion still belongs to the shell, and
  no agent may run or read it.
- They do not weaken PATCH-0005: no second package family, no second reader, no second surface, and no
  grid adopted from family B because it happened to exist.
- They do not authorise a parser. A format owner still writes the schema, and this repository still has
  no code tree.
