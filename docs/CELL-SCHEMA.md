# CELL-SCHEMA — a proposal for the format owner, not a ruling

**Status:** proposal. **Nothing here is decided.** Every question below carries a recommendation the
format owner may accept or overrule, the alternative it rejects, what would falsify it, and the owner
who must rule.

**Inputs:** `docs/CELL-MODEL.md` (the model and its six invariants), `docs/DECISIONS-PORT.md` D-1 (cells
are a declared display unit over data, not a grid), plan §3.1 (`Cell frames`: typed cell data, field
bindings, shapers, projections, ordering, dependencies), `docs/PACKAGE-OWNERSHIP.md` §1 (this
repository owns no bytes).

## How the six invariants map to refusals

The model states six invariants as prose. This proposal makes each one a refusal the builder and the
acceptor both perform, through the same function, so a declaration one accepts is one the other
accepts.

| Invariant | Who refuses | What the refusal names |
|---|---|---|
| I-1 Bounded, not truncated | builder and acceptor | the ceiling exceeded and the measured value |
| I-2 Roles, not colours | builder and acceptor | the colour literal and the role it should have named |
| I-3 Addressed, never ambient | acceptor | the destination that is not a declared window or panel of the same package |
| I-4 Named by a reference | acceptor | the declaration no row of the package names |
| I-5 One grammar, two callers | architecture, checked by the gate | the second parser, if one is ever added |
| I-6 Disposable projections | renderer, by construction | nothing at build time; a cache is rebuilt, never trusted |

## Q-1 — Is a cell one row kind or several?

**Recommendation:** one kind, carrying one canonical declaration text per destination as its single
field. Rationale: family A already models a declaration as canonical text in one field (the `shell`
row), which gives whitespace-insensitive identity for free and keeps the kind table total; a binding
or a placement with no cell is not a thing, so splitting them would create rows that can only be
refused.

**Rejected:** separate kinds for cells, bindings and placements. It multiplies the kind table, and each
new kind needs its own field-count and its own refusal path for no gain in expressiveness.

**Falsifier:** a real surface needs to reference a cell declared by another package without
re-declaring it. Text-in-one-field cannot validate a cross-package reference at build time, so that
requirement would force a second kind.

**Owner:** format owner.

## Q-2 — Field layout and encoding of a cell set

**Recommendation:** canonical declaration text, line-oriented, tab-separated between a verb and its
fields — the shape the `shell` declaration already uses — re-encoded canonically on write so comments
and spacing cannot change the digest. The field layout is therefore "one field: the declaration"; the
declaration's own line grammar is the thing the owner specifies.

**Rejected:** a typed array of values per cell. `rows.rs` fixes one field count per kind, so a
variable-length record would need a new row shape, which is a format change larger than the question.

**Falsifier:** a surface whose cell set cannot be held in memory as one declaration (streaming a map
larger than the declaration ceiling) — then a chunked layout, and this recommendation fails.

Integer widths and byte encodings are deliberately **not** proposed here: this repository owns no
bytes, and fixing them would be the port inventing a wire format rather than consuming one.

**Owner:** format owner.

## Q-3 — Shaper and projection vocabulary

**Recommendation:** a closed enumeration, versioned with the declaration, with a total table in the
format — the same discipline as the five panels and the four row kinds. An unknown shaper is refused by
name, never defaulted.

**Rejected:** an open, free-form vocabulary resolved at runtime. That is a second extension mechanism
beside the package system, and it cannot be validated at build time.

**Falsifier:** source semantics need a projection that cannot be expressed as a member of a closed set.
The answer then is a recorded format act that adds a member, never a runtime escape hatch — so the
recommendation survives the falsifier as a rule about *how* the set grows.

**Owner:** format owner.

## Q-4 — Ceiling values

**Recommendation:** a declaration ceiling of 512 KiB and a cell ceiling of 16 384, with the grid
addressed as columns by rows bounded by the same cell ceiling. Rationale: these are the measured
ceilings of the only working cell format in the trees, which makes them evidence about the class of
limit rather than an aesthetic choice; they are numbers, not a grammar, so taking them does not adopt
that family's format.

**Rejected:** no declared ceiling, with allocation following the declaration. That is how a malformed
package becomes an out-of-memory report instead of a named refusal.

**Falsifier:** a legitimate surface (a large map, a dense panel) that needs more than these bounds.
Then the ceiling rises with a measured justification, and the refusal keeps existing.

**Owner:** format owner.

## Q-5 — Binding syntax

**Recommendation:** a binding names its data by a stable declared identifier and nothing else; the
acceptor resolves it and refuses an unknown identifier by name. No paths, no expressions, no
arithmetic.

**Rejected:** an expression or path language in bindings. That is a second interpreter with its own
budget problem, and the code kind (D-3) is where behaviour belongs.

**Falsifier:** a surface that needs a value computed from other values. If that becomes necessary, the
computation belongs in the code kind under its budgets, not in the binding, so the recommendation holds
only while bindings stay declarative — which is the point.

**Owner:** format owner.

## Q-6 — Does the first surface use cells at all?

**Recommendation:** none is derivable from the documents available here, and none is offered. The
answer depends on what the imported game's first screen actually contains, and that lives in the
shell-private inventory (H4), which no agent may read or ask for. What follows is the choice, stated
plainly rather than dressed as a recommendation.

**The choice, as the owner must put it:**

1. **Cells in the first surface.** The first surface is rebuilt on the cell path, so H9 cannot be
   witnessed until this schema is ruled, a parser exists, and a renderer draws it. Later, smaller.
2. **Shapes and declared layout first, cells after.** H9 can be witnessed sooner with a surface that
   does not depend on cells; the cell path is added behind the same surface when the schema is ruled.

**Rejected:** deciding blind. Picking either option without the inventory would be a guess presented
as a ruling, and the two errors cost very differently: cells-first can stall the only witness that
matters, and layout-first can mean building the surface twice.

**Falsifier:** for option 1, an inventory showing the first screen carries no data-driven display, in
which case cells-first buys nothing early. For option 2, an inventory showing the first screen *is*
data-driven display, in which case deferring cells means building that surface twice.

**Owner:** project owner.

## What this proposal cannot settle, and will not pretend to

- Byte widths and encodings — the format owner's, and named as open in Q-2.
- The op vocabulary of the code kind — a different decision (D-3), deliberately out of scope here.
- The number of shapers or projections that will eventually be needed — only use can say, and no
  surface exists yet.

## What must not happen

The family-B raster is not the target: its 8 × 16 paper/ink grid is rejected here as a model, and that
family's grammar stays unadopted (PATCH-0005). A schema proposal that restates that raster as the thing
to build toward has failed this document.
