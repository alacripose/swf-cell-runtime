# SCOPE-ACCOUNTING — every named contract item, present or explicitly missing

Plan §0.5 forbids scope loss by omission, simplification, substitution, or an unrecorded
"MVP" interpretation. Every named item below is either `present` or recorded as
`not-implemented` / `blocked` with an owner and a blocking dependency.

Agent-authored implementation vocabulary: `present`, `not-implemented`, `blocked`.
Source-feature gate states (`held` / `absent` / `refused`) are shell-authored and remain
**unassigned** in this repository. Nothing here is a completion claim.

In the package-layer table, `present` means **the authoritative crate already implements
and owns the item**; it does not mean this repository has code for it. This repository has
no client code yet.

## Block status

| Block | Status | Note |
|---|---|---|
| H1 | present | Scaffold, six baseline patches, templates, ledger, first commits. |
| H2 | present | Both package families read; family A (`V4PKG001`) is the port's frame, family B (`SHPKG002/003`, `cells/1`, `cellframe/1`) is evidence and is not adopted. |
| H3 | present | `tools/port_gate.py` enforces decidable invariants and proves refusals. |
| H4 | blocked | Shell-only ingestion; no agent may run or read it. |
| H5 | blocked | Code-kind executor undecided; depends on a format-owner decision. |
| H6 | not-implemented | |
| H7 | not-implemented | |
| H8 | not-implemented | |
| H9 | not-implemented | |
| H10 | not-implemented | |

## Package and format layer (H2) — owner: format owner, read in `planet-shploingus`

| Item | Status | Owner / evidence |
|---|---|---|
| `V4PKG001` frame header (magic, schema `u16`, kind `u16`, length `u64`, 32-byte digest, payload) | present | `planet-package/src/frame.rs` |
| Payload digest in the header (`domain ‖ schema ‖ len ‖ payload`) | present | `frame.rs`; `planet-core/src/hash.rs` |
| Whole-frame digest in the file name (`<id>-<sha256>.pkg`) | present | `frame.rs` `frame_digest`, `file_name`, `id_and_digest` |
| Payload grammar (`ROW_SCHEMA u16 = 1`, `u32` count, rows) | present | `rows.rs`, `frame.rs` `payload()` |
| Row kinds/families (`row`, `bands`, `materials`, `shell`; declaration/value; `KINDS_ARE_TOTAL`) | present | `rows.rs` |
| Declaration grammar (`.decl`, tab-separated, five panels, canonical re-encode) | present | `shell.rs` |
| Refusal vocabulary (14 frame refusals, 10 declaration refusals) | present | `error.rs`, `shell.rs` |
| Builder (`package build` / `package show`) | present | `src/bin/package.rs` |
| Acceptor / mount (`Mount::watch`: newest by `(mtime, name)`, digest dedup, adopt-or-refuse-whole) | present | `planet-shell/src/mount.rs` |
| Admission witness (package door `present_package`, ledger record) | present | `planet-shell/src/main.rs`; `planet-mcp/src/surface.rs` |
| Media/asset carriage (canonical bytes in row fields; no separate archive) | present | `rows.rs` `Bands`/`Materials` |
| `shploingus.cells/1` declaration grammar | present | Family B: `voxel-package/src/cells.rs` (`DECLARATION_HEADER`, `CellVersion`) |
| `shploingus.cellframe/1` frame grammar, reader and refusals | present | Family B: `voxel-package/src/cellframe.rs` (`HEADER`, `Frame::parse`, `read`, `read_artifacts`, ceilings, `RefusalSet`) |
| Family B frame (`SHPKG002`/`SHPKG003`, `recipe/3`) | present | `voxel-package/src/recipe.rs` (`MAGIC`, `MAGIC_ARTIFACTS`, `HEADER`) |
| Adopting family B as this port's format | blocked | PATCH-0005 forbids a second package family; format-owner decision required |
| A2: `ala_civ_bots_prototype` Java frame reader | present | `PackageFrame.java`: byte-compatible frame, disjoint row kinds (`observation`/`decision`), payload-digest names, no callers and no frame tests |
| Foreign package acceptance | present as a refusal | `UnknownRowKind` and `NameMismatch` refuse another tree's package instead of half-reading it |
| Family-A cell model | not-implemented | Ruled D-1 and scoped in `docs/CELL-MODEL.md`; a schema proposal with recommendations, rejected alternatives, falsifiers and named owners is in `docs/CELL-SCHEMA.md`, awaiting the format owner's ruling |
| `surface.mount/2` transport | blocked | Absent from both families; A adopts via `Mount::watch` + door admission, B reads folders/artifact maps |
| Reserved `Code` kind — frame reader | present | `FrameKind::Code = 2`; encodes via `--code`, decodes |
| Reserved `Code` kind — executor | not-implemented | Ruled D-3: bounded deterministic cell-program instruction set; no executor exists yet, and `planet-shell` still refuses code frames on mount (`mount.rs:111`) |
| Media kinds (bitmap, font, audio) | not-implemented | Ruled D-2: one kind per media type, one field each, each needing a format-document line |
| Streamed transport | not-implemented | Ruled D-4: a new layer with its own byte/digest/receipt decisions; not "mount-v2", which does not exist in either family |

## Shell-only import layer (H4–H5, H7–H8) — owner: shell

| Item | Status | Blocking dependency |
|---|---|---|
| Signature-based input classification (`FWS`/`CWS`/`ZWS`) | not-implemented | shell execution |
| Immutable source record, digest, byte length, provenance | not-implemented | shell execution |
| Full source inventory (header, shapes, display list, timelines, text/fonts, media, scripts, interaction, dependencies) | not-implemented | shell execution |
| Shell-private compiler IR and source-to-cell map | not-implemented | shell execution |
| Visual/media lowering (shapes, transforms, bounds, z-order, masks, hit regions; bitmaps, fonts, audio, video) | not-implemented | shell execution |
| Timeline lowering (display-list ops per transition, nested sprite clocks, rational tick accumulation) | not-implemented | shell execution |
| AVM1/AVM2 semantic compilation into cell programs | not-implemented | shell execution |
| Input/interaction lowering (hit areas, buttons, focus, repeat, capture, drag, wheel, touch, propagation) | not-implemented | shell execution |
| Source oracle and private traces | not-implemented | shell execution |
| Private coverage matrix, zero unclassified features | not-implemented | shell execution |
| Visual baseline store | not-implemented | shell execution |

## Runtime — one surface, one executor (H9)

| Item | Status | Blocking dependency |
|---|---|---|
| `package_owner` adapter (thin call into `planet-package`; owns no bytes) | not-implemented | client crate needed |
| `cell_mount` (`Mount::watch` semantics: digest dedup, adopt-or-refuse-whole) | not-implemented | client crate needed |
| `cell_program` (deterministic instruction set, capabilities, budgets, cancellation) | blocked | H5 executor decision |
| `cell_runtime` (graph, timeline scheduler, event dispatch, state, save/recovery) | not-implemented | H4/H5 |
| `cell_stream` (watched file + door admission; no second transport) | not-implemented | client crate needed |
| `render_wgpu` (batches, atlases, text, layers/depth/mask/filter, disposable caches) | not-implemented | H2 present; client crate needed |
| `client` (`winit` lifecycle, input forwarding, resize, redraw, notices) | not-implemented | — |
| Single surface identity, no second window/renderer/router/scheduler | present as a gate rule | enforcement is guard-only until code exists |

## Addon transaction (H10) — owner: shell

| Step | Status |
|---|---|
| 1. Discover from configured roots / the shell's `packages/` | not-implemented |
| 2. Receive the exact bytes the store names | not-implemented |
| 3. Enforce size, count, nesting, time, resource ceilings pre-parse | not-implemented |
| 4. Decode one canonical frame; validate rows, declaration, media fields, capabilities | not-implemented |
| 5. Resolve dependencies, check base/addon compatibility | not-implemented |
| 6. Reject stale, same-version-different-bytes, duplicate identity, undeclared refs, conflicting extension points | not-implemented |
| 7. Build candidate graph without touching the live graph | not-implemented |
| 8. Run package and migration checks on a staged snapshot | not-implemented |
| 9. Atomic commit at a deterministic tick boundary | not-implemented |
| 10. Preserve prior mounted set and state on any failure | not-implemented |
| 11. Receipt, digest, revision, outcome, user-visible notice | not-implemented |
| Extension points + capability scopes | not-implemented |
| Unload / replace / rollback under shell authorization | not-implemented |

## Persistence and determinism

| Item | Status |
|---|---|
| Saves keyed by game/cell identity, package digest, state schema | not-implemented |
| Unknown-field and foreign-namespace preservation | not-implemented |
| Pre-operation snapshot, staged candidate, before/after digests, audit, atomic commit, rollback | not-implemented |
| Crash-consistent addon replacement (old-valid or new-valid, never mixed) | not-implemented |
| Named recovery state and reconciliation on restart | not-implemented |
| Seeded randomness and private replay trace | not-implemented |

## Agent-facing MCP boundary (H6) — owner: shell

| Operation | Status |
|---|---|
| `schema.describe` | not-implemented |
| `ticket.submit` | not-implemented |
| `secure_cell.get` | not-implemented |
| `test.list_declared` | not-implemented |
| `test.run_declared` | not-implemented |
| `test.result.get` | not-implemented |
| CUA Axis driver (shell/test/user-owned, generic mechanics only) | not-implemented |
| Prohibited surfaces absent (capture, screenshots, pointer, keyboard, text, runtime-advance, probes, selectors, coordinates, CUA handles, package browsing, inventory) | present as a gate rule; enforcement requires H6 |

## Governance (H1, H3) — owner: this repository

| Item | Status |
|---|---|
| Six baseline patches with required rule + mechanical evidence | present |
| Ticket template, issue template, PR template | present |
| Append-only ledger with checkpoint fields | present |
| Drift gate: tracked source binaries, output escape, second magics, commit refs, ledger validity, baseline presence, block exit tests | present |
| Gate self-test proving each refusal | present |
| Cell-schema guard (six invariants, four-part answers, no wire tokens, no raster target) | present | `tools/port_gate.py` `cell_schema` over `config/cell-schema.json` |
| Plan errata guard (block, quote, citations, index entry) | present | `tools/port_gate.py` `plan_errata` over `config/plan-errata.json` |
| Pre-commit hook and CI workflow running the gate | present |
| Branch + PR review enforcement (remote repository, review records) | not-implemented — no remote is configured in this checkout |
| Commit-hash binding in ledger entries | not-implemented |
| Duplicate-reader / alternate-surface detection on real code | not-implemented — no code tree yet; gate reports it `NOT-ENFORCED` |

## Unassignable here

Source-feature gate states, content acceptability, redaction, quarantine, and any
destructive content operation. **shell gate state: unassigned.**
