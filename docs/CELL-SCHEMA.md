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
**Narrowed by the owning reader, 2026-11-03:** `cellframe.rs` refuses a field name it does not
sample, refuses a second source for the same field, and refuses prose where a law token belongs,
so the declarative form is the format's, not this proposal's preference. The recommendation above
stands unchanged; what changes is that it is the reader's own refusal set rather than only a
design choice.


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

## What the owner ruled, 2026-11-03

The six questions above were put to the owner with this document in front of them. Where the owner
answered, the answer is recorded here; the sections above stay as the proposal they were, so a later
reader can see both the question and the answer. Two of the answers were checked against the trees
rather than taken from this document, because the owner asked for that first.

- **Q-1 — the cell is a frame, not a single row kind.** Read from the sources, not proposed here:
  `shploingus.cellframe/1` (`openlock/shell/alpha-shell/frames/MAP-0001.frame`) is a grid of cells
  addressed to one declared window's content space. It carries typed frame keys (`window`, `cols`,
  `rows`, `order`, `requires`), declarative field bindings (`field elevation terrain::elevation_m`,
  `field biome ecology::biome_at`), a shaper and a projection (`shaper profile32 32`,
  `projection orthographic`), and typed draw rows (`fill`, `run`) whose colours are roles, never
  literals. `package.cells` (`shploingus.cells/1`) is two lines and says which drawing of the shell a
  package is a package of. **Ruled: several row kinds — frame keys, field bindings, typed draw rows —
  not one cell kind.** A binding or a run with no frame is not a thing, which is the reason the
  single-kind recommendation above was withdrawn rather than amended.
- **Q-2 — a typed array of values per cell** (owner). It matches the `fill` and `run` columns the
  trees already write: position, extent, role and text, each in its own field.
- **Q-3 — the vocabulary is open, and it is streamed**, in a labelled section at the top of the
  declaration (owner). A value the section does not declare is refused by name against that section,
  not against a closed enumeration owned by this document.
- **Q-4 — 4 GiB per package, 64-bit addressing** (owner, restated). That supersedes the ceilings
  proposed above; the frame-level numbers the trees already carry stay as the frame's own bounds: a
  512 KiB declaration, a 512 by 512 grid, 16 384 cells, so a frame is bounded even inside a package
  that may grow to the package ceiling.
- **Q-5 — still open.** "A binding names a declared field id and nothing else" is the recommendation;
  the owner did not rule it, and nothing in this document may be read as a ruling on it. The lowering
  may not depend on an answer here until one is recorded.
- **Q-6 — cells in the first surface** (owner). It is consistent with the resynthesis answer Q9: the
  windows' XML is converted into declared cells inside the package, never carried as a payload.

### The artifact table and the audio references

**This shape is family B's, not one invented here.** `shploingus-clean/crates/voxel-package/src/recipe.rs`
carries a package's own files as an artifact table — a count, then `path` and `bytes` for each, with
`ARTIFACT_PATH_MAX = 256`, `ARTIFACT_BYTES_MAX = 4 MiB`, `ARTIFACT_COUNT_MAX = 1024`, and a path that
must be package-relative (no leading `/`, no `\`, no `.`/`..`, no empty part), checked on compile
**and** on load. `cellframe.rs` in that tree never splices media into a drawing row: a frame is a
sibling artifact the package names by id, and `voxel-client/src/shell/mount.rs` holds the mounted
package's files in `frame_artifacts`, keyed by package-relative path.

So a declaration opens with the package's files and the frame names them:

```text
[artifact]
audio/sound-7.mp3<TAB>54d7de0b6d4b1c42<TAB>7d06ffe3...

audio
sound<TAB>7<TAB>audio/sound-7.mp3<TAB>mp3<TAB>11025<TAB>1784
stream<TAB>-<TAB>audio/stream.mp3<TAB>mp3<TAB>0<TAB>0

shploingus.cellframe/1
frame<TAB>MAP-0001
```

`[artifact]` is `path<TAB>digest<TAB>hex`, terminated by one blank line. The path is family B's
package-relative name and the ceilings are its ceilings, restated rather than widened; the payload is
its bytes verbatim, lower-case hex, even length, never empty. `digest` is the payload's
`payload_digest` (FNV-1a 64, the same function the shell frame uses for its payloads), stated by the
row and **checked against the bytes on read**, so `artifact-digest` refuses a table whose bytes are
not the bytes it describes. A path outside the family's rules is `cellframe-row`; a duplicate path is
refused by name; the count and byte ceilings are refusals (`cellframe-bound`), never truncations.

`[audio]`, which follows, holds one row per sound and **names an artifact rather than carrying one**:
`sound` is a definition character and carries its id, `stream` is a stream block and states `-`,
`format` and `rate` are the source's declared encoding, `samples` its declared sample count. A row
naming a path the table does not carry is refused `cellframe-row` with `artifact-reference`; a
duplicate id of the same kind is refused by name. The bytes a consumer receives are the artifact's,
resolved at parse, so a count of audio can still never stand in for audio. A declaration with no
`[artifact]` and no `[audio]` carries no sound, which is the state a silent frame is in.

### The XML reader this repository has, and the one it does not

The Qt reader belongs to secure-shell v2; nothing here implements or lowers it. `testport1` has one
XML reader, `crates/planet-client/src/xml.rs`, written as the second client format so the two
formats are compared on one package. It is a declarative surface in the sense
`docs/cellular_generativity_axis_civilian_expectations.md` §7.2 requires: an allowlist of elements
and attributes, refusals by name for anything else, and no script or style path. The numeric
containment bounds that document names are not yet wired into that reader, and that is a named gap
rather than a claim.

### The v2 addendum's rule, and the reader that answers Q-1 and Q-5

`docs/cellular_generativity_axis_civilian_expectations_prototype_rechecked.md` section 13 binds this
port to the v2 streamed inventory, and it states its own precondition in CGA-21: a cellframe and a
cell set use their owning versioned readers, and a guessed grammar is a refusal condition. Section
13.10 lists the reads that must happen first. So the frame grammar is recorded here from the owning
source rather than from a sample frame:

- `shploingus-clean/crates/voxel-package/src/cellframe.rs` — the reader for `shploingus.cellframe/1`.
  A header, then `frame<TAB>id`; the required keys `window`, `cols`, `rows`, `order` and `requires`;
  the optional key `feature`; the draw rows `fill<TAB>col<TAB>row<TAB>cols<TAB>paper` and
  `run<TAB>col<TAB>row<TAB>ink<TAB>text`, whose paper and ink are roles and never colours; and,
  optional together, the sampling rows `field`, `shaper`, `projection`, `centre`, `cell`, `extent` and
  `pitch`. The bounds are the format's: a 512 KiB declaration, a 512 by 512 grid, 16 384 cells, and a
  cell size the format states. An unknown key, a duplicate key, a missing key, a row past the grid and
  a frame that draws nothing are each refused by name in one pass.
- `shploingus-clean/crates/voxel-package/src/cells.rs` — `package.cells` is a header and one `cells`
  line selecting which drawing the package is a package of. The older declaration header is refused
  at its file and line instead of being read under the newer rules, a declaration with no drawing
  line is refused, and a package carrying no such file at all is an absence the caller states.

**Q-1 is answered by that grammar**: several row kinds inside one frame — typed keys, field bindings
and typed draw rows — not one cell row kind. **Q-5 is answered as far as the format answers it**: a
binding is a declared field name plus a qualified law token (`elevation` answered by
`terrain::elevation_m`). The reader refuses a field name it does not sample and refuses a second
source for one field, so a path or an expression has nothing to bind to, and there is no second
syntax to refuse because the grammar admits none.

**What this document does not settle, and will not invent:** the exact contracts of `V4PKG002`,
`SHPKG009`, `secure-shell.served/2`, `secure-shell.ingest/1`, `secure-shell.admission/1`,
`civilian.axis-cells/1`, `civilian.decision/1`, `civilian.ticket/1`, `system1-agent/1`,
`minecraft.cua-action/1`, `ensouled.frame/1` and `ensouled.action/1`. Section 13.10 names those as
reads that precede implementation. Until they are read in this repository they are integration
anchors, not permission to invent a schema, and no unit may cite them as an implemented contract.

**Two gaps this record names rather than closes.** The numeric containment bounds the earlier
expectations doc asks of a declarative surface — bytes, node count, depth, text length — are still
not wired into `crates/planet-client/src/xml.rs`, which has named refusals but no budgets. And the
format id reader upgrades a legacy numeric id to the computed one; CGA-21 refuses a *silent* version
conversion, so that upgrade has to stay visible in the record it produces, not only in the value it
returns.

## The visual payload: what a cell draws, and what it says it does not draw

The import carries a cell's visual payload in the same shape the owning trees use, and the parts
that were this port's own habit were corrected against them:

- **A frame names a role, never a colour.** `shploingus-clean/crates/voxel-package/src/cellframe.rs`
  (`//! **A frame names a role, never a colour.**`) and `voxel-client/src/shell/cells.rs`
  ("**A role is not a colour here.**") state the rule; `recipe.rs`'s `theme_role` field is where a
  package says what a role *is*. This port keeps it exactly that way: a `fill` row names a paper
  role, and the `[vocabulary]` `colour` rows are the package's own role→RGB declarations — the same
  role-not-colour indirection, with the package, not the client, saying what the role means.
- **The painter's order is the placement path.** A stage placement carries a depth; a sprite's
  children carry their own depths *inside* the sprite's slot. Cells are drawn back to front by that
  whole path (`emit::Cell::order`), because flattening a sprite's children onto the sprite's depth
  let a sprite's own background paint over its art — a flat picture where the file's art was.
- **A mask is not a colour, and an invisible record paints nothing.** A placement that declares a
  clip depth states the depths it clips, not a picture; a placement whose colour transform sets the
  alpha multiplier to zero is invisible. Neither is drawn, and both are counted in
  `Lowering::masked` / `Lowering::transparent`. Clipping and partial alpha are named gaps, not
  guessed colours.
- **A sprite is never its bounding box.** A sprite whose first frame places nothing draws nothing;
  a measured rectangle around a scene is a grid where the picture should be. The frame states the
  absence instead (below).
- **A timeline is frames, not one composited display list.** `cellframe.rs` refuses a frame with no
  `fill` and no `run` ("an empty frame is a window's content erased, and an absence has to be stated
  as one"), and its frames are *sibling artifacts the package names by id*, which
  `voxel-client/src/shell/mount.rs` holds in `frame_artifacts`. This port therefore reads the movie
  frame by frame, applying `RemoveObject`/`RemoveObject2` so a character the file takes away is not
  composited into a later frame, and it states which frame it ships. Each later frame is carried as a
  sibling artifact the `[timeline]` section names by `order`, and the sounds the movie **starts** are
  a `[event]` row each — `sound<TAB>frame<TAB>audio-id`, naming a payload the `[audio]` table already
  carries. An event is not a payload: a package that carried sounds and not the frames that start
  them would hold audio nothing can place in time.
- **What the package does not carry is stated, not drawn.** The streamed `[absence]` section is one
  row per character with no art: `sprite<TAB>id<TAB>reason<TAB>rule`, where `reason` is
  `no-display-list`, `no-drawn-record` or `undecoded-display-record`, and `rule` is the owning rule
  as a law token — `cellframe::malformed-record`, the `RefusalKind` the owning reader uses when it
  refuses a frame that draws nothing. A client reads these and prints them; it draws nothing for
  them. The section exists because the alternative the format forbids is an empty frame standing in
  for erased content.

**The 512 KiB / 16 384 pair is an implemented floor, and the ruled ceiling is owed.** The format
owner ruled on it directly (`secure-shell-audit/cell-schema-ruling.md`, Q-4, 2026-11-02):

> **Not the 512 KiB / 16 384 pair.** The surface is stateful and real-time, the addressing is
> **64-bit**, and the geometry reaches **subvoxel** scale. The ceiling must admit that class of
> surface. Exact numbers still to be given.

Both readers carried the old numbers — `MAX_FRAME_BYTES = 512 * 1024` and `MAX_CELLS = 16_384` in
`shploingus-clean/crates/voxel-package/src/cellframe.rs` and, restated, in this port's
`crates/planet-package/src/cellframe.rs`. Both now carry the **interim floor**
`MAX_FRAME_BYTES = 8 MiB` and `MAX_CELLS = 65_536`, which admits the ruled class — a stateful,
real-time, 64-bit addressed surface whose frames carry a timeline — while the owner's exact numbers
are owed. It is a **floor, never the ruling**: a refusal that quotes it says "the interim floor", and
the number moves when the owner gives the real one. The package frame's payload length is a 64-bit
little-endian field bounded here by `PAYLOAD_LIMIT = 4 MiB` in `crates/planet-package/src/frame.rs`,
so "64-bit" is the encoding of the length while the budget is a number still owed. A timeline of
whole frame documents hex-encoded into one declaration was refused by the current constant (measured:
1 679 826 bytes for 12 frames); that refusal is evidence about the constant, not a verdict on the
shape, so the timeline travels as **sibling frame documents**, one per frame — the shape the owning
trees already use — and `TICKET-0075` carries the client tick over them. The port's own client is
**ingest evidence, not the runtime**: `secure-shell-audit/testport1_plan_resynthesis.md` §1 records
`testport1` as the ingest module of secure-shell v2, whose client is the Qt 6 shell, and
`docs/OWNERSHIP-AUDIT-2026-11-04-family-branches-and-v2.md` records what that changes here.
