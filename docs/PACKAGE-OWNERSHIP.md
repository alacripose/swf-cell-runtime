# PACKAGE-OWNERSHIP — H2: ownership named from source, both package families

**Status:** `present`. **Block:** H2. **Recorded:** 2026-11-02. **Supersedes** the `blocked`
record of TICKET-0002 / LEDGER-0002, and corrects one claim of TICKET-0004 / LEDGER-0005.

**Sources read (read-only, no build, no mount, no execution):**

- `planet-shploingus` — `crates/planet-package/{src/frame.rs, rows.rs, shell.rs, error.rs, lib.rs, bin/package.rs, tests/tool.rs}`, `crates/planet-core/src/{hash.rs, canon.rs}`, `crates/planet-shell/{src/mount.rs, main.rs, Cargo.toml}`, workspace `Cargo.toml`.
- `shploingus-clean` — `crates/voxel-package/src/{recipe.rs, cells.rs, cellframe.rs, lib.rs}`, `crates/voxel-client/src/shell/{mount.rs, cells.rs}`, `crates/axis/verify/src/subject.rs`.
- `secure-shell` — `docs/FORMAT-SOURCES-AUDIT.md` (§3), `server/test/ingest.test.mjs`.
- `openlock` — `src/Openlock/Shell/ShellPackage.cs`, `tools/gen_shell_data.py`, `clientaddon/panorama/scripts/openlock_shell_data.js`.

Nothing was built, mounted, or run. This is a decision record: it fixes **who owns what**, so
that one reader exists. It is not an implementation, and no parser is written here.

## 1. There are two package families, not one

This is the fact the plan could not see, and it changes H2.

| | **Family A** — `planet-shploingus` | **Family B** — `shploingus-clean` |
|---|---|---|
| Frame magic | `V4PKG001` | `SHPKG002` (data), `SHPKG003` (with artifact table) |
| Frame layout | magic · schema `u16` · kind `u16` · length `u64` · 32-byte digest · payload | magic · length `u32` · 32-byte digest · exact source |
| Declarations | one frame, typed row kinds (`row`, `bands`, `materials`, `shell`) | sibling text artifacts: `shploingus.recipe/3`, `shploingus.cells/1`, `shploingus.cellframe/1` |
| Cell drawing | none | **yes** — `cellframe` (§4) |
| Mount | `planet-shell::mount::Mount::watch` + door admission | folder read / artifact map (`recipe::artifacts_in`) |
| Status | current game shell | abandoned post-split library (its own recipe header is pinned at `recipe/3`) |

**Family A is this port's frame.** Family B is prior-generation evidence. Adopting family B's
frame, recipe, or cell grammar *as the port's format* would be a second package family, which
PATCH-0005 refuses. What family B supplies is the **semantics** the port must reproduce inside
family A's rows or a future `Kind` — not a second wire format.

## 2. Ownership, family A (the port's frame)

| Layer | Owner | Authority |
|---|---|---|
| Package frame | `planet-package` | `frame.rs`: `FRAME_MAGIC`, `FRAME_SCHEMA`, `FrameKind`, `Package::{encode, decode, read, write}`, `frame_digest`, `file_name`, `id_and_digest` |
| Payload/rows | `planet-package` | `rows.rs`: `ROW_SCHEMA`, `Kind`, `Family`, `Row`, `KINDS_ARE_TOTAL` |
| Declaration grammar | `planet-package` | `shell.rs`: `ShellDeclaration::{parse, encode, row}`, `PANELS`, `DeclarationError` |
| Refusals | `planet-package` | `error.rs` (14 named), `shell.rs` (10 named) |
| Identity/hash | `planet-core` | `hash.rs`: `DOMAIN_PACKAGE = b"planet-package-v1"`, `digest(domain, schema, bytes)`; `canon.rs`: little-endian `put_*` |
| Builder | `planet-package` binary | `bin/package.rs`: `package build|show` |
| Acceptor/mount | `planet-shell` | `src/mount.rs`: `Mount::watch`, `Reload`; `src/main.rs`: watch + `present_package` witness |

**One grammar, two callers.** The builder parses a `.decl` with `ShellDeclaration::parse` then
calls `declaration.row("shell")`; the mount takes the `Kind::Shell` field and calls the **same**
function. The crate states it: *"the tool that builds a shell package and the shell that mounts one
read a declaration through the same function."*

**Decision: depend, never vendor.** The port consumes `planet-package` + `planet-core` as path
dependencies. A local framing parser is a second reader — the drift PATCH-0005 refuses.

### The frame

```
magic 8 "V4PKG001" | schema u16 LE =1 | kind u16 LE (1 data, 2 code)
| length u64 LE | 32-byte payload digest | payload
head = 52 bytes; payload = u16 ROW_SCHEMA · u32 row count · rows
```

Two digests over two preimages — never conflated:

1. **Header digest:** `digest(DOMAIN_PACKAGE, FRAME_SCHEMA, payload)` — the schema is inside the
   preimage, so a schema bump changes the digest, not just the header.
2. **Name digest:** `frame_digest(bytes) = digest(DOMAIN_PACKAGE, FRAME_SCHEMA, bytes)`, and
   `Package::read` refuses a mismatch (`NameMismatch`).

`digest(domain, schema, bytes)` = SHA-256(`domain ‖ schema_le ‖ len_le(u64) ‖ bytes`).
Name is `<id>-<digest>.pkg`; an id that could be a path is `BadId`; a name without a digest is
`NamelessDigest`. Whole frame or nothing: `Truncated`, `BadLength`, `TrailingBytes`.

### Rows

`ROW_SCHEMA=1`, `u32` count, then `put_bytes(kind) · put_bytes(subject) · put_u32(field count) · put_bytes(field)*`.

| Kind | Family | Fields |
|---|---|---|
| `row` | declaration | 1 |
| `bands` | value | 1 |
| `materials` | value | 1 |
| `shell` | declaration | 1 |

`KINDS_ARE_TOTAL` makes a kindless table a compile error. **A new media type is a new `Kind`** — a
row plus a format-document line, format-owner decision. There is no separate asset archive, and
media travels as canonical bytes in row fields.

### The `.decl` grammar

Tab-separated; `#` comments and blank lines ignored; trailing `\r` stripped.
`window <title>` (2 fields, exactly one) · `destination <panel> <mark> <title>` (4 fields, panel ∈
`[world, light, axis, changes, about]`, mark exactly one character, at least one destination, no
panel twice). `encode()` is canonical, so a comment- or spacing-only edit re-encodes to identical
bytes and therefore to an identical digest. Ten named refusals: `NotText`, `UnknownVerb`,
`FieldCount`, `Empty`, `UnknownPanel`, `DuplicatePanel`, `MarkLength`, `NoWindow`, `TwoWindows`,
`NoDestinations`.

### The mount

`Mount::watch(dir)`: newest `*.pkg` by `(mtime, name)`; digest-deduped, a refusal counting as
answered; a mid-write file skipped and retried; `Package::read` whole or refused; `kind == Code`
refused (`mount.rs:111`); first `Kind::Shell` row parsed; success replaces the declaration in force
and reports the artifact. `main.rs` then presents it at the package door, so the reload leaves an
admission on the ledger. **The receipt is that admission plus the ledger record** — there is no
separate receipt object to invent. A refusal leaves the previous declaration untouched.

### The reserved code kind

`FrameKind::Code = 2` is declared, written by `--code`, and decoded. `planet-shell` **refuses** a
code frame on the mount path; `planet-mcp` will present one (`surface.rs:450`). **No executor
exists.** So the missing piece is the executor, not the reader — H5 stays blocked on that decision.

## 3. Ownership, family B (prior generation; evidence, not adoption)

| Layer | Owner | Authority |
|---|---|---|
| Package frame | `voxel-package` | `recipe.rs`: `MAGIC = b"SHPKG002"`, `MAGIC_ARTIFACTS = b"SHPKG003"`, `HEADER = "shploingus.recipe/3"`; unknown magic refused, other headers refused |
| Cell declaration | `voxel-package` | `cells.rs`: `DECLARATION_HEADER = "shploingus.cells/1"`, `CellVersion { Elements = 0, Holo = 1 }`, `IMPLEMENTED` list; `shploingus.cells/0` refused by file and line |
| Cell frames | `voxel-package` | `cellframe.rs`: `HEADER = "shploingus.cellframe/1"`, `Frame::parse`, `read`, `read_artifacts`, `Declared`, `Package` |
| Format subject (measured) | `axis-verify` | `subject.rs`: `FORMAT` with `format: Some("shploingus.cellframe/1")`, witness `display-packages/alpha-shell/frames/MAP-0001.frame` |
| Raster/placement | `voxel-client` | `shell/cells.rs` (paint/`frame`), `shell/mount.rs` (`CellVersion`, `Vec<Declared>`, `frame_space`) |

## 3A. A second claimed family-A implementation — `ala_civ_bots_prototype`

`src/main/java/dev/ensouled/packages/PackageFrame.java` (300 lines) states it is *"ported from
`planet-package/src/frame.rs` and `rows.rs` — the same magic, the same schema numbers, the same
domain-separated digest and the same row grammar."* Measured against the Rust source, three of those
four hold.

| Property | `planet-package` (Rust) | `PackageFrame` (Java) | Same bytes? |
|---|---|---|---|
| Magic, schema, kind, length, digest, payload | `V4PKG001`, `u16`, `u16`, `u64`, 32 | identical | **yes** |
| Digest preimage | `domain ‖ schema_le ‖ len_le ‖ bytes`, `DOMAIN_PACKAGE` | identical (`planet-package-v1`, `le16`, `le64`) | **yes** |
| Payload head | `u16 ROW_SCHEMA`, `u32` count | identical | **yes** |
| Field framing | `put_bytes` = `u64` length + bytes | `putBytes` = `le64` length + bytes | **yes** |
| Row kinds | `row`, `bands`, `materials`, `shell` — 1 field each | `observation`, `decision` — 13 fields each | **no: disjoint vocabularies** |
| File name | `<id>-<whole-frame digest>.pkg`, verified on read (`NameMismatch`) | `<id>-<payload digest>.pkg`, never verified on read | **no: different digest, no read-side check** |
| Refusals | typed `PackageError`; `BadLength{declared,found}` distinct from `TrailingBytes{trailing}` | `IllegalArgumentException` string codes; short and long frames are both `truncated` | **no** |

So the frame is byte-compatible and the package is not. The consequence is the useful part:
**`V4PKG001` identifies a frame layout, not a readable package.** A package from either tree decodes
its header on the other side and is then refused on `UnknownRowKind` — the correct outcome, and one
that already exists. A Java-written file additionally fails the name check by construction.

Two facts about the Java port, both from the tree:

- **Nothing calls it.** `PackageFrame` appears only in its own file: no production class and no test
  references it. The Java package loop that is live runs on `DecisionPackage`/`AxisProof`.
- **It has no frame-format tests.** `FrameParserTest` covers the prose frame parser, not
  `PackageFrame`. Its compatibility is asserted in a comment and unverified even on its own side.

**Verdict: design evidence, not an authority and not a second reader to adopt.** What it contributes
is *content vocabulary* — the §47.2 envelope (`message_id`, `conversation_id`, `sender`,
`sender_name`, `recipients`, `channel`, `scope`, `speech_act`, `utterance`, `content_hash`, `tick`,
`dimension`, `position`) and the observation/decision split — which belongs in a family-A row design,
never in a frame.

## 4. What `shploingus.cellframe/1` actually specifies

This is the only place in any tree where "cells" is a real grammar with a reader, so it is the
evidence base for whatever representation the port eventually needs.

**A frame is a sibling artifact, named by id from a `row`** — the recipe grammar is untouched
because `recipe/3` has no frame rows (R-64.1: a document needing a field the schema lacks is a
refusal, not a schema change). A package that declares a frame and no row naming it is refused
rather than mounted as furniture.

```text
shploingus.cellframe/1
frame <id>          window <window-id>   cols <u32>   rows <u32>   order <u32>
                    requires <frame-id>[,...]|none      [feature <world-feature>]
fill  <col> <row> <cols> <paper>
run   <col> <row> <ink>  <text>
```

- **Ceilings, declared not truncated:** `MAX_FRAME_BYTES = 512 KiB`, `MAX_COLS = MAX_ROWS = 512`,
  `MAX_CELLS = 16_384`, `CELL_W_DP = 8`, `CELL_H_DP = 16`.
- **Every key required**, unknown key refused by name, duplicate key refused by name; `KEYS` is
  `[window, cols, rows, order, requires]` and `OPTIONAL_KEYS` is `[feature]`.
- **A frame names a role, never a colour:** `Paper`/`Ink` are parsed by word; an unknown name is a
  refusal, not a default. A frame must not become a second palette.
- **Bounded by its window:** `window` is a declared window id and the extent comes from
  `recipe::WindowSpec::geometry`; a frame that does not fit is refused by cell count rather than
  drawn through chrome. A `fill`/`run` past `cols`/`rows` is refused with the cell and row.
- **An empty frame is refused by name** — an absence has to be stated as one.
- **Refusals accumulate:** `Frame::parse` returns `RefusalSet` with every problem
  (`RefusalKind::{MalformedRecord, UnadmittedFieldSpec, …}`), not the first one.
- **Two files are one declaration:** `frames/<id>.frame` (machine) + `frames/<id>.md` (record), the
  split `layer.rs` already uses; `Frame::render` prints both, and a record missing a required patch
  section is refused by that section's name.

## 5. Corrections

Four to the plan, and one to this repository's own record. They are recorded as explicit errata **E-1–E-4** in `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §3.1 and §12, with the superseded wording left visible beside each correction.

1. **`shploingus.cells/1` is not family A's declaration format.** It is owned by
   `voxel-package/cells.rs` in family B. The plan's claim that it is an artifact "within the package
   system" is right about existence and wrong about which system.
2. **`shploingus.cellframe/1` has a full reader, writer, and refusal set** — in family B
   (§4). LEDGER-0005 said "no reader or writer anywhere"; that claim was **wrong**, made from three
   trees when the owner was in a fourth. It is corrected here, and the over-claim is named as drift
   in LEDGER-0007 rather than quietly edited away.
3. **`surface.mount/2` does not exist in either family.** Family A adopts through
   `Mount::watch` + door admission; family B reads a folder or an artifact map. The string appears
   only in openlock/dl-enclave declarations.
4. **The `cell/4` tag in the secure-shell census is not a format tag.** The only `cell/4`
   occurrence in the workspace's `.rs` sources is an arithmetic comment in
   `voxel-client/src/pattern.rs:890` (`footprint / (cell/4)`). The real headers are
   `shploingus.recipe/3`, `shploingus.cells/1`, `shploingus.cellframe/1`.
5. **Frame identity is two digests** (family A) and **family B's frame is magic · `u32` length ·
   digest · exact source** — neither is the other, and no `V4PKG001` reader can read `SHPKG002`.

## 6. Consequences for the port

- `package_owner`: thin adapter over `planet-package` + `planet-core`. No framing code.
- `cell_mount`: `Mount::watch` semantics — newest by `(mtime, name)`, digest dedup, adopt-or-refuse
  whole, never a partial mount.
- `cell_stream`: the transport is the watched file plus the door admission. A different transport is
  a new, explicitly decided layer — not a claim that mount-v2 exists.
- The declaration surface is `.decl`, and `PANELS` is a real constraint: the ported game draws
  inside one of five panels or the format owner widens the set.
- **Cell drawing is a format-owner decision, not an import detail.** The honest options are
  (a) a new family-A `Kind` carrying a cell raster with family B's semantics (ceilings, role not
  colour, window-bounded, empty refused, all problems reported), or (b) no cell raster at all. What
  is not available is adopting family B's file/grammar wholesale.

## 7. Exit condition — met, with the fork recorded

Named from source: family A's frame, rows, declaration, refusals, builder, mount and admission;
family B's frame, cell declaration, cellframe grammar, and its format subject. Builder and acceptor
share one grammar in both families. Rejected alternatives are recorded: vendoring a local reader,
adopting family B's magics, and reconstructing a transport that does not exist.

Named from source: three implementations that claim this frame — the authoritative Rust pair, the
family-B grammar, and the Java port that shares the bytes and not the row table — with the reason each
is or is not adopted.

**Still blocked:** the family-A cell representation decision (§6), the code-kind executor (H5),
the `shploingus.cells/1` version-0 path (refused by family B itself, so it stays refused here), and
any streaming transport beyond the watched file.

## 8. Rulings (2026-11-02)

Section 6's bullet offering "a new family-A `Kind` carrying a cell raster with family B's semantics
(ceilings, role not colour, window-bounded, empty refused, all problems reported), or no cell raster at
all" is **superseded**, and the bullet stays above so the correction is readable rather than rewritten.

The project owner ruled that a cell here is a **declared display unit over data**, not a character
grid: in the measured trees a cell mechanism stands in place of a data renderer — a map renderer in
`planet-shploingus`, an XML renderer in `openlock` — so the family-B 8 × 16 paper/ink raster is that
family's renderer choice, not the concept. §3.1's own `Cell frames` row ("typed cell data, field
bindings, shapers, projections, ordering, and dependencies") is the shape the ruling restores.

Media (per-type kinds), the code executor (bounded deterministic cell program) and the streamed
transport (a new layer, not "mount-v2") are ruled in the same record: `docs/DECISIONS-PORT.md`
D-1–D-4. All four are decisions, not implementations, and none changes the ownership named above.
