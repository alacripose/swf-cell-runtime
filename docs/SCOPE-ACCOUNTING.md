# SCOPE-ACCOUNTING — every named contract item, present or explicitly missing

Plan §0.5 forbids scope loss by omission, simplification, substitution, or an unrecorded
"MVP" interpretation. Every named item below is either `present` or recorded as
`not-implemented` / `blocked` with an owner and a blocking dependency.

Agent-authored implementation vocabulary: `present`, `not-implemented`, `blocked`.
Source-feature gate states (`held` / `absent` / `refused`) are shell-authored and remain
**unassigned** in this repository. Nothing here is a completion claim.

## Block status

| Block | Status | Note |
|---|---|---|
| H1 | present | Scaffold, six baseline patches, templates, ledger, first commits. |
| H2 | blocked | Authoritative package crate absent from checkout. |
| H3 | present | `tools/port_gate.py` enforces decidable invariants and proves refusals. |
| H4 | blocked | Shell-only ingestion; no agent may run or read it. |
| H5 | blocked | Depends on H2 ownership and H4 inventory. |
| H6 | not-implemented | |
| H7 | not-implemented | |
| H8 | not-implemented | |
| H9 | not-implemented | |
| H10 | not-implemented | |

## Package and format layer (H2) — owner: format owner, blocked on authoritative crate

| Item | Status | Blocking dependency |
|---|---|---|
| `V4PKG001` frame header (magic, schema, kind, length, digest, payload) | blocked | crate absence |
| Frame digest construction (domain separation) | blocked | crate absence |
| Cell package row grammar / row schema / row kinds / field counts | blocked | crate absence |
| Cell declaration grammar (`shploingus.cells/1`) | blocked | crate absence |
| Cell frame grammar (`shploingus.cellframe/1`) | blocked | crate absence |
| Asset/artifact reference mechanism | blocked | crate absence |
| Reserved `Code` kind reader and executor contract | blocked | crate absence + format-owner decision |
| Strict decode refusals (bad magic, schema, kind, truncation, overlong, digest, malformed row, unknown required field, bad cell ref, missing asset, duplicate id, trailing bytes) | not-implemented | H2 |
| Single-grammar proof for builder and acceptor | not-implemented | H2 |

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
| `package_owner` adapter (delegates to authoritative reader) | not-implemented | H2 |
| `cell_mount` (validate + mount into one cell graph) | not-implemented | H2 |
| `cell_program` (deterministic instruction set, capabilities, budgets, cancellation) | not-implemented | H5 |
| `cell_runtime` (graph, timeline scheduler, event dispatch, state, save/recovery) | not-implemented | H2/H5 |
| `cell_stream` (mount-v2 consumption, receipts, revisions, staging) | not-implemented | H2 |
| `render_wgpu` (batches, atlases, text, layers/depth/mask/filter, disposable caches) | not-implemented | H2 |
| `client` (`winit` lifecycle, input forwarding, resize, redraw, notices) | not-implemented | — |
| Single surface identity, no second window/renderer/router/scheduler | present as a gate rule | enforcement is guard-only until code exists |

## Addon transaction (H10) — owner: shell

| Step | Status |
|---|---|
| 1. Discover from configured roots / package store | not-implemented |
| 2. Receive exact bytes via mount-v2 | not-implemented |
| 3. Enforce size, count, nesting, time, resource ceilings pre-parse | not-implemented |
| 4. Decode one canonical frame, validate all referenced artifacts and capabilities | not-implemented |
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
| Pre-commit hook and CI workflow running the gate | present |
| Branch + PR review enforcement (remote repository, review records) | not-implemented — no remote is configured in this checkout |
| Commit-hash binding in ledger entries | not-implemented |
| Duplicate-reader / alternate-surface detection on real code | not-implemented — no code tree yet; gate reports it `NOT-ENFORCED` |

## Unassignable here

Source-feature gate states, content acceptability, redaction, quarantine, and any
destructive content operation. **shell gate state: unassigned.**
