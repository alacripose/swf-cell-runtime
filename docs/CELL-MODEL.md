# CELL-MODEL — what a cell is here, and what stays undecided

**Ruling:** D-1 (`docs/DECISIONS-PORT.md`). **Model status:** decided. **Schema status:** undecided.
**Implementation status:** `not-implemented` — this repository has no code tree.

This document fixes the *model* so that work can be scoped without anyone inventing a wire format. It
deliberately does **not** name a `Kind`, a field layout, an encoding, or a shaper vocabulary: those are
format-owner acts, and `docs/PACKAGE-OWNERSHIP.md` §1 says the port owns no bytes.

## 1. What a cell is

A cell is a **declared display unit over data**. It exists so that a surface can be *described* rather
than requiring a renderer written per kind of data. In the measured trees the same seat is held by a
map renderer in `planet-shploingus` and an XML renderer in `openlock`, which draws frames over game
windows — different implementations of one job: turn declared data into drawn output without a bespoke
renderer per data type.

It is **not** a character grid. Family B's 8 × 16 paper/ink raster (`shploingus.cellframe/1`) is that
family's renderer; adopting it would import a renderer as if it were the concept, and PATCH-0005
forbids adopting the family anyway.

## 2. What a cell must be able to do

Taken from the plan's own `Cell frames` row (§3.1) rather than invented here:

| Capability | Meaning |
|---|---|
| typed cell data | the payload carries declared values, not opaque blobs |
| field bindings | a cell names where its values come from; it does not copy them |
| shapers | a declared transformation from data to drawn form |
| projections | a declared choice of what surface form the data takes |
| ordering | a declared draw order, and a declared dependency order |
| dependencies | a cell may require another to be present and resolved first |

## 3. Refusals the model requires

Each of these is a named refusal at build time and at mount time, **all problems reported at once**
rather than the first one — family B's `RefusalSet` behaviour, which is worth keeping even though its
raster is not:

1. **Unbounded extent** — a declaration whose size is not within declared ceilings is refused, never
   truncated.
2. **Empty declaration** — a cell set that draws nothing is refused by name; an absence must be stated
   as one.
3. **Unknown binding** — a field binding naming data that does not exist is refused by name.
4. **Unsatisfied dependency** — a cell requiring an absent cell is refused; it is not drawn degraded.
5. **Duplicate ordinal** — two cells claiming one position in the draw order are refused.
6. **Malformed value** — a value that does not parse as its declared type is refused with the cell and
   the field that produced it.

## 4. Invariants

- **Bounded, not truncated.** Ceilings are declared in the format, and exceeding one is a refusal.
- **Roles, not colours.** A cell names a role the surface resolves; it may not carry a colour that
  would make the package a second palette.
- **Addressed, never ambient.** A cell set addresses one declared destination (a window or panel that
  the same package declares), never chrome, a title bar, or another package's content space.
- **Named by a reference.** A cell set that no row of the same package names is refused rather than
  mounted as furniture — family B's rule, kept because it is about reachability rather than raster.
- **One grammar, two callers.** The builder and the acceptor parse a cell declaration through the same
  function, so a declaration the builder accepts is one the mount accepts.
- **Disposable projections.** Any GPU buffer, atlas, or layout computed from cells is reproducible from
  the package and the session state; caches are never a source of truth.

## 5. Undecided, with the owner of each

| Question | Owner | Where it is decided |
|---|---|---|
| Is it one `Kind` or several (e.g. cells and cell-bindings)? | format owner | new ticket; `rows.rs`-equivalent row plus a format-document line |
| Field layout and encoding of a cell set | format owner | same ticket |
| The shaper and projection vocabulary | format owner | same ticket |
| Ceiling values | format owner | same ticket |
| Binding syntax (how a cell names its data source) | format owner | same ticket |
| Whether the client's first surface uses cells at all | project owner | H9 |

Nothing in this table is answered by this document, and nothing here authorises a parser.

## 6. What this model must not become

- Not a grid raster adopted from family B because it exists.
- Not a second package family, a second renderer, or a per-data-type renderer set — that would restore
  exactly the problem cells exist to remove.
- Not an asset archive: media travel as their own per-type kinds (D-2), not inside cells.
- Not an execution surface: cells declare display. Behaviour is the code kind's problem (D-3), under
  its own budgets.
