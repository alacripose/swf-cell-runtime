# SWF-to-Cell Package Port: Security-Hardened Theoretical Implementation Framework

**Target:** Preserve the original SWF game's playable behavior while porting it into the Rust `winit` + `wgpu` client and making gameplay extension possible through the streamed package/addon path proven in Openlock.

**Governing constraint:** One surface. One package format. One cell runtime. The SWF is source input to the package compiler; it is not a second runtime, a sidecar application, a compatibility window, or an image-conversion target.

**Revision:** v3 — trust-boundary and maximal-contract correction.

**Errata (2026-11-02):** four claims in §3.1 and §12 are superseded by measured source — **E-1** `shploingus.cells/1` and `shploingus.cellframe/1` ownership, **E-2** the `surface.mount/2` transport, **E-3** the code-kind reader status, **E-4** the asset-reference mechanism, **E-5** the remaining "existing mount-v2" references. Each erratum keeps the original wording visible beside the correction and cites the file that settles it. Full ownership record: `docs/PACKAGE-OWNERSHIP.md`; evidence: `LEDGER-0008`. Open decisions are marked undecided, not answered.

**Rulings (2026-11-02):** the four decisions the errata named as open were put to the project owner and answered. **D-1** a cell is a *declared display unit over data*, not a character grid — the family-B `cellframe` 8 × 16 paper/ink raster is that family's renderer, standing where a map renderer or an XML renderer stands in other trees; **D-2** media travel as per-type kinds (bitmap, font, audio), one field each; **D-3** the reserved code kind executes a bounded deterministic cell-program instruction set; **D-4** the port gains a streamed transport as a new layer with its own decisions, explicitly not "mount-v2", which does not exist. Where an erratum's *Status* line still calls one of these open, this block supersedes the status; each erratum's correction of fact stands unchanged. Rulings are decisions, not implementations. Full record: `docs/DECISIONS-PORT.md`; ledger entry `LEDGER-0009`.

**Status:** Architecture proposal based on a read-only audit of the four named repositories. No repository was modified, built, or executed for this audit. The actual SWF has not been supplied to this architecture audit, so its ActionScript version, tag inventory, game-specific APIs, and unsupported dependencies remain to be measured by the shell after the user supplies it. Those measurements will remain shell-private and will not be exposed to an implementation agent.

## 0. Normative trust boundary and maximal-system convention

### 0.1 Source movie and every source-derived representation are shell-only

The original SWF/SFW file, its decompressed bytes, and every representation derived from it remain inside the shell's trusted import boundary. The shell alone may read, open, parse, inspect, execute in a reference player, classify, inventory, transform, compile, compare, or visually observe the source movie.

The agent provider MUST NOT receive or be able to retrieve:

- Raw or decompressed source bytes, decoded source media, or source-player output.
- Source tag names/types as individually identifiable records, symbol names, frame labels, strings, asset/file names, URLs, source paths, source offsets, or dependency names.
- ActionScript bytecode, source script/class/function names, class tables, string tables, symbol tables, or source-to-cell mappings.
- Detailed source inventories, compiler IR, source-derived test vectors, source-oracle traces, screenshots, frame captures, or recordings.
- Any other source-derived identifier or content from which those details can be reconstructed.

The shell retains the immutable source, its signature and full provenance, all detailed inventories, source identities, offsets, dependency relationships, source maps, decoded media, source-player baselines, and oracle comparisons. Those records stay in shell-private storage under the project root and are not mounted into an agent workspace or returned by MCP. Git-ignore rules alone are not an access boundary: the shell must enforce filesystem/process isolation so an agent provider cannot read the input or shell-private directories.

The agent may work only from canonical package contracts, generic implementation specifications, synthetic fixtures, opaque secure-cell work items, and shell-authored redacted test results. The importer, source analyzer, compiler, source player, and oracle are not agent tasks and are never exposed as agent tools.

### 0.2 Internal cell ticket -> shell-issued secure cell

Before an agent receives a work unit or a test result, it submits an internal cell ticket. The shell validates the ticket and issues a capability-scoped secure cell. A secure cell may contain only the minimum information required for that ticket:

- Opaque ticket, work-item, cell, and artifact identifiers.
- Digests of the canonical package/cell artifacts relevant to the ticket; the original source-file digest remains shell-held.
- The ticket's allowed implementation operations and capability descriptors.
- Shell-declared test identifiers and the permitted result fields.
- Redacted, coarse classifications and abstract state-transition contracts that do not expose source identifiers or reconstructable content.

A secure cell MUST NOT contain source names, script identifiers, URLs, offsets, detailed provenance, decoded assets, source-derived traces, or a source-to-cell mapping. It is not a copy of the source inventory. It is the only agent-visible content/work representation.

Every MCP call beyond schema discovery requires a valid ticket id. Unticketed calls are refused by the shell. The agent cannot issue, alter, or self-authorize its own capabilities or secure-cell contents.

### 0.3 Agent MCP is ticket/secure-cell/test-only; CUA is shell/test/user-owned

The agent-facing MCP surface is limited to schema discovery and ticket, secure-cell, and declared-test operations. It MUST NOT expose general observation or control of the running client. In particular, agents have no screenshot, frame/video capture, surface observation, pointer, keyboard, text-input, arbitrary runtime-advance, selector, coordinate, probe, or free-form action primitive.

The permitted agent-facing operation family is limited to:

1. Submit an internal cell ticket.
2. Retrieve the shell-issued secure cell for that ticket.
3. List tests already declared by the shell for that ticket.
4. Request execution of one of those declared tests by the shell.
5. Retrieve the shell-produced, redacted result for an authorized test.

The shell/test harness may use the abstract CUA Axis driver internally to execute shell-declared tests or user-authorized witness flows. The driver may use generic operating-system input and private observation internally, but it is not an agent tool and does not disclose its captures, observations, raw input traces, or CUA session to an agent. It must not hardcode game-specific controls or recreate gameplay logic. Test plans are shell-owned and derived/approved inside the trusted boundary; agents cannot invent probes or action sequences.

The user may interact with the running client directly. User witness and shell/test evidence remain separate from agent observation: the agent receives only the redacted test result explicitly attached to its secure cell.

### 0.4 Shell-only refusal, content, and mutation authority

Only the shell/runtime may assign or emit `held`, `absent`, or `refused` states, decide content acceptability, quarantine or withhold a source record, or authorize a content operation. Those terms in this document describe shell-authored states and gates; they are never agent decisions, agent recommendations, or agent-returned statuses. Agents MUST NOT judge whether source content is acceptable or emit those states.

Agents MUST NOT delete, overwrite, rewrite, redact, sanitize, quarantine, unload, replace, migrate, or otherwise destructively alter source-derived content, mounted game content, saves, or live package state. Agents may submit ordinary implementation code changes through the ticket/branch/PR process, but the shell controls validation, merge gates, package production, mounting, live-state mutation, and all content-changing transactions. An agent's inability to complete work is an implementation failure or unresolved request; the shell alone determines the resulting state.

The shell does not sanitize the source in place. It preserves the immutable original and decides whether content is withheld from an agent boundary. Any approved transformation, migration, replacement, unload, or rollback is a shell-owned operation with explicit authorization, a staged candidate, pre/post digests, an audit record, and rollback behavior. An agent cannot trigger or perform that operation directly.

### 0.5 Maximal contract now; verification slices are not product scope

Build the full named system contract in this plan. Every named package state, refusal code, store, path, capability, source-coverage class, ticket/secure-cell transition, importer/compiler responsibility, runtime subsystem, addon transaction, persistence rule, and verification path must be implemented or explicitly recorded by the shell as missing with an owner and blocking dependency. No item may disappear through omission, simplification, substitution, or an unrecorded “MVP” interpretation.

The H1-H10 sequence is an ordered engineering and evidence-gathering discipline, not permission to ship thin slices, stop after the first playable slice, defer a named subsystem indefinitely, or reduce the final product to whatever happens to work first. A vertical slice is an integration witness within the full contract. Missing functionality remains visibly missing and blocks any claim of completion. Scope reduction requires a separate, explicit user decision and a reviewed ticket/patch; an implementation agent cannot trim the contract unilaterally.

This maximal-system rule requires full scope accounting now. It does not authorize unreviewed implementation, bypassing gates, or exposing source information to an agent.

---

## 1. Governing architecture

The base game and its mods must become peers in one cell-native package system. The base SWF is compiled into the canonical cell package representation. Addons are compiled or authored into that same representation. The running shell mounts one composed cell surface and executes one cell runtime.

```text
                    TRUSTED SHELL BOUNDARY
                 original SWF/SFW (read-only)
                                |
                                v
                 shell-only parse + inventory
             (names, offsets, URLs, scripts, assets)
                                |
                                v
                shell-only semantic compilation
                  +-------------+-------------+
                  |             |             |
              visual IR     timeline IR    program IR
                  |             |             |
                  +-------------+-------------+
                                |
                                v
                  CANONICAL CELL PACKAGE
                       V4PKG001 frame
                                |
                 +--------------+--------------+
                 |                             |
           base-game cells              addon/mod cells
                 |                             |
                 +--------------+--------------+
                                |
                                v
                    ONE PACKAGE STREAM / MOUNT
                 existing mount-v2 transport path
                                |
                                v
                       ONE CELL SURFACE
                                |
                                v
                   ONE CELL EXECUTION RUNTIME
                                |
                  +-------------+-------------+
                  |             |             |
               winit          wgpu       audio / input
                  |             |             |
                  +-------------+-------------+
                                |
                                v
                     PLAYABLE BASE GAME + MODS

        Separate, shell-only verification path:
        source oracle + client + CUA Axis -> private tests
        private tests -> redacted semantic result -> secure cell

        Agent path:
        ticket -> secure cell -> implementation proposal/PR
        No source path, CUA, screenshot, input, or probe path.
```

The SWF is not loaded by a Flash Player beside the Rust game. The trusted shell importer reconstructs source semantics and writes them into the canonical cell package. This entire import, source-analysis, and lowering path is off-agent. At runtime, the cell engine executes those semantics as part of the same world/surface that executes addon behavior.

This is a semantic port, not a screenshot port. It must preserve the game's state transitions, timing, input behavior, collision rules, scoring/progression, media, and persistence—not merely reproduce its first frame.

## 2. Repository audit and what each project contributes

### 2.1 `dl-openshell`: the strongest evidence for streamed addon delivery

Repository: [alacripose/dl-openshell](https://github.com/alacripose/dl-openshell)

Relevant source:

- [`AddonPackages.cs`](https://github.com/alacripose/dl-openshell/blob/main/src/Openlock/Shell/AddonPackages.cs)
- [`ShpPackage.cs`](https://github.com/alacripose/dl-openshell/blob/main/src/Openlock/Shell/ShpPackage.cs)
- [`ShellPackage.cs`](https://github.com/alacripose/dl-openshell/blob/main/src/Openlock/Shell/ShellPackage.cs)
- [`ShellMount.cs`](https://github.com/alacripose/dl-openshell/blob/main/src/Openlock/Shell/ShellMount.cs)
- [`openlock_delivery.js`](https://github.com/alacripose/dl-openshell/blob/main/clientaddon/panorama/scripts/openlock_delivery.js)
- [`package.cells`](https://github.com/alacripose/dl-openshell/blob/main/shell/alpha-shell/package.cells)
- [`MAP-0001.frame`](https://github.com/alacripose/dl-openshell/blob/main/shell/alpha-shell/frames/MAP-0001.frame)

**Proven patterns worth preserving:**

1. Addon discovery supports configured roots and game search paths, including packages embedded in VPK archives.
2. Package identity includes a package name, version, source, digest, and declared windows/artifacts.
3. Older versions are rejected; same-version/different-content replacement is rejected; a valid newer version replaces the prior package only after the entire candidate has staged successfully.
4. The live package catalog is changed atomically. A failed replacement leaves the previous package mounted.
5. The client validates the incoming payload before applying it. The delivery module has explicit ceilings for payload bytes, node count, and tree depth.
6. Rebuilding and closing windows is queued and time-sliced rather than allowing a large delivery to monopolize a frame.
7. Refusals and delivery outcomes are surfaced instead of silently falling back.

**Important limitation found in the actual source:** `ShellPackage.ReadCells` currently recognizes `shploingus.cells/1` but explicitly refuses it because that host only draws composed panels. `ReadFrames` recognizes the `shploingus.cellframe/1` header and explicitly refuses the frame's cell fields/shapers/projections. `ShellContent` also marks `streamer` as refused because no mountable streamer source exists on that build. Therefore, Openlock proves the *addon delivery and atomic replacement mechanics*, but the current C# shell is not proof that cell frames are already mounted and rendered by that host.

**Porting consequence:** reuse Openlock's staging, versioning, discovery, bounds, atomic swap, and user-visible refusal behavior. Do not copy its legacy `.shpkg` / `openlock.addons/2` window payload as a second content format for this project. Base game content and mod content must arrive as the same canonical cell packages.

### 2.2 `secure-shell`: package ownership, mount discipline, and execution order

Repository: [alacripose/secure-shell](https://github.com/alacripose/secure-shell)

Relevant source:

- [`docs/FIRST-TEN-HOURS.md`](https://github.com/alacripose/secure-shell/blob/main/docs/FIRST-TEN-HOURS.md)
- [`docs/ARCHITECTURE.md`](https://github.com/alacripose/secure-shell/blob/main/docs/ARCHITECTURE.md)
- [`docs/DECISIONS.md`](https://github.com/alacripose/secure-shell/blob/main/docs/DECISIONS.md)
- [`docs/SELECTIONS.md`](https://github.com/alacripose/secure-shell/blob/main/docs/SELECTIONS.md)
- [`docs/LEDGER.md`](https://github.com/alacripose/secure-shell/blob/main/docs/LEDGER.md)
- [`docs/FORMAT-SOURCES-AUDIT.md`](https://github.com/alacripose/secure-shell/blob/main/docs/FORMAT-SOURCES-AUDIT.md)

The first-ten-hours document is an implementation discipline, not a feature backlog. Each block has one artifact, an exit test, a refusal gate, and a list of work that is not allowed in that block. A 90-minute checkpoint records the user's verbatim goal, changed artifact, previous gate state, drift signal, refused work, next artifact, and ledger entry. A block without its ledger entry does not authorize the next block. These ordered blocks organize construction and verification of the complete target; they do not authorize deferring or omitting any named subsystem.

The secure-shell decisions explicitly select `surface.mount/2` as the stream of record, disallow a new package/window format, and state that the client is the only valid test surface. A browser/Node fixture can assist development but cannot witness a mounted client surface. The source audit describes the authoritative package frame family as `V4PKG001`, with schema, kind, payload length, and a domain-separated digest; it also records that the code frame kind is reserved until a reader exists.

**Porting consequence:** keep one cell surface and one canonical package frame. Use the existing mount-v2 stream/receipt path as the transport and mount boundary, not as a second content representation. Keep the first-ten-hours gates, ledger, and client-witness rule. Do not create a new `secure-shell.*` game format, another mount store, or a parallel display.

### 2.3 `ala-civ-bots-prototype`: package framing, verified delivery, and long-lived state

Repository: [alacripose/ala_civ_bots_prototype](https://github.com/alacripose/ala_civ_bots_prototype)

Relevant source:

- [`PackageFrame.java`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/src/main/java/dev/ensouled/packages/PackageFrame.java)
- [`StreamClient.java`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/src/main/java/dev/ensouled/client/StreamClient.java)
- [`SocialStream.java`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/src/main/java/dev/ensouled/core/SocialStream.java)
- [`docs/IMPLEMENTATION_STATUS.md`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/docs/IMPLEMENTATION_STATUS.md)
- [`docs/UPSTREAM_ALIGNMENT.md`](https://github.com/alacripose/ala_civ_bots_prototype/blob/main/docs/UPSTREAM_ALIGNMENT.md)

The Java `PackageFrame` implementation documents the `V4PKG001` frame header: eight-byte magic, frame schema, frame kind, payload length, digest, and payload. It validates magic, schema, kind, exact declared length, digest, row schema, row kinds, field counts, and trailing bytes. The stream client checks that the served payload is for the expected agent and that its digest and window references are valid before drawing it.

The implementation-status record also makes a relevant distinction between a connected client and a persistent being: stable identity, durable continuity, write-ahead state, explicit recovery states, and reconciliation are separate requirements. That discipline is useful for game saves and mod-owned state too: a mounted package must not erase or silently recreate live gameplay state during an update.

**Porting consequence:** use this project as evidence for strict frame decoding, digest verification, per-recipient stream separation where applicable, and durable lifecycle/state handling. The Java frame reader is a reference implementation, not a reason to add a second Rust format reader beside the authoritative Rust crate.

### 2.4 `ala-cities`: the Rust graphics/windowing foundation

Repository: [alacripose/ala-cities](https://github.com/alacripose/ala-cities)

Relevant source:

- [`Cargo.toml`](https://github.com/alacripose/ala-cities/blob/main/Cargo.toml)
- [`src/main.rs`](https://github.com/alacripose/ala-cities/blob/main/src/main.rs)
- [`src/render.rs`](https://github.com/alacripose/ala-cities/blob/main/src/render.rs)
- [`src/ui.rs`](https://github.com/alacripose/ala-cities/blob/main/src/ui.rs)
- [`src/session.rs`](https://github.com/alacripose/ala-cities/blob/main/src/session.rs)

`Cargo.toml` already uses `winit 0.30.13` and `wgpu 30.0.0`. `main.rs` demonstrates the `ApplicationHandler` event lifecycle, input routing, resize handling, a fixed-step simulation separate from an uncapped render loop, and ticket/governor-mediated world changes. `render.rs` owns a `wgpu::Surface`, adapter/device/queue, render pipelines, depth texture, image texture/bind group, GPU buffers, and Glyphon text rendering.

**Porting consequence:** use `ala-cities` as the Rust window/GPU implementation reference and likely code donor after a recorded selection. Do not inherit its city simulation as the SWF game's simulation. Its fixed 20 Hz loop is an example of simulation/render separation, not an instruction to force every SWF to 20 Hz; the imported game's declared timeline rate and timing behavior must be preserved.

### 2.5 Audit conclusion

The four projects supply complementary evidence, but none of them alone is the complete target:

| Concern | Best source | What it proves | What remains to implement |
|---|---|---|---|
| Package frame/integrity | `ala_civ_bots_prototype` and secure-shell's format audit | `V4PKG001` framing and strict decode expectations | Rust integration with the authoritative package crate and the SWF-derived row set |
| Cell declarations/frames | `dl-openshell` package examples | Existing `shploingus.cells/1` and `shploingus.cellframe/1` artifacts exist | A reader, mount path, cell graph, and renderer that actually consume them |
| Addon streaming | `dl-openshell` | Discovery, validation, version rules, atomic replacement, bounded delivery | Same mechanics applied to canonical cell packages, not a second `.shpkg` content format |
| Mount/receipts/ledger | `secure-shell` | Existing mount-v2 path, receipts, durable ledger, client witness | Game-package integration through that path |
| Rust window/GPU | `ala-cities` | Practical `winit`/`wgpu` client foundation | Cell-driven scene construction and game runtime integration |
| Base-game semantics | Shell-owned SWF ingestion and source oracle | Not available to this audit; must be measured by the shell after input is supplied | Full shell-private inventory, semantic reconstruction, and redacted test contracts |

---

## 3. The one-format contract

### 3.1 Format ownership

The implementation agent must locate the authoritative `planet-package` / cell-package crate and its actual reader/writer in the workspace before writing a parser. The secure-shell format audit identifies `V4PKG001` as the framed package ingress and identifies the `.decl`/cell declaration grammar as belonging with the package kind, so builder and acceptor share one grammar.

The expected responsibilities are:

| Layer | Authority | Responsibility |
|---|---|---|
| Package frame | Existing `V4PKG001` owner | Magic, schema, kind, payload length, domain-separated digest, exact decode/refusal behavior |
| Cell package rows | Existing package row grammar | Typed records that describe cell content, cell programs, dependencies, and package composition, using only schema already owned or explicitly approved by its owner |
| Cell declaration | Existing cell declaration owner | Declares the package's cells/frames and their relationships |
| Cell frames | Existing `shploingus.cellframe/1` owner | Typed cell data, field bindings, shapers, projections, ordering, and dependencies |
| Stream/mount | Existing `surface.mount/2` path | Delivers and receipts the package bytes/identity; it does not invent a second content schema |
| Surface/runtime | One Rust client | Mounts and executes the cells; there is no parallel Flash surface or addon surface |

`surface.mount/2` is the transport/receipt boundary, not a second game package. `V4PKG001` is the package frame. `shploingus.cells/1` and `shploingus.cellframe/1` are existing cell declarations/records within the package system. Detailed source-derived provenance is held outside agent-visible package manifests; secure cells receive only opaque artifact identities and digests. The implementation must not add another package magic, another window declaration format, or a new `secure-shell.*` game payload format.

> **ERRATUM E-1 — supersedes the paragraph above (measured 2026-11-02).**
>
> **Superseded wording, kept visible:** *"`shploingus.cells/1` and `shploingus.cellframe/1` are existing cell declarations/records within the package system."* The same assumption appears in the `Cell declaration` and `Cell frames` table rows above.
>
> **Measured:** the grammars exist, but not in the package system this section means. `planet-package` declares four row kinds — `row`, `bands`, `materials`, `shell` — and no cell or cellframe kind (`crates/planet-package/src/rows.rs`: `Kind::ALL`, `Kind::COUNT = 4`, `KINDS_ARE_TOTAL`); its `lib.rs` exposes only `error`, `frame`, `rows`, `shell`. The grammars are owned by `voxel-package` in `shploingus-clean`: `crates/voxel-package/src/cells.rs` (`DECLARATION_HEADER = "shploingus.cells/1"`, `CellVersion { Elements = 0, Holo = 1 }`) and `crates/voxel-package/src/cellframe.rs` (`HEADER = "shploingus.cellframe/1"`, `Frame::parse`, `read`, `read_artifacts`). They belong to a different frame family — `SHPKG002`/`SHPKG003` over a `shploingus.recipe/3` document (`crates/voxel-package/src/recipe.rs`: `MAGIC`, `MAGIC_ARTIFACTS`, `HEADER`) — not to `V4PKG001`.
>
> **What the port uses instead:** the declaration surface of its own family is the `.decl` grammar in `crates/planet-package/src/shell.rs` (`ShellDeclaration::parse`, `PANELS`), which the builder and the mount both call.
>
> **Status:** contradiction — corrected. Whether the port grows a family-A cell raster at all is an **open decision**, not settled here (`docs/SCOPE-ACCOUNTING.md`: "Family-A cell raster representation: blocked"; `docs/PACKAGE-OWNERSHIP.md` §6). Adopting the other family's grammar is refused by PATCH-0005.

> **ERRATUM E-2 — supersedes the first sentence above and the `Stream/mount` row of the table (measured 2026-11-02).**
>
> **Superseded wording, kept visible:** *"`surface.mount/2` is the transport/receipt boundary, not a second game package."* and the table's *"Stream/mount | Existing `surface.mount/2` path"*.
>
> **Measured:** no `surface.mount`, `mount/2`, mount-v2 or receipt type exists in the authoritative tree; a search of `crates/` in `planet-shploingus` for `surface.mount`, `mount/2`, `mount_v2` and `MountReceipt` returns nothing. Adoption is a watched file: `crates/planet-shell/src/mount.rs` `Mount::watch` takes the newest `*.pkg` by `(mtime, name)`, deduplicates by frame digest, reads it whole with `Package::read`, refuses the code kind (`if package.kind == FrameKind::Code`), and parses the first `Kind::Shell` row through `ShellDeclaration::parse`. `crates/planet-shell/src/main.rs` then presents the adopted package at the host door (`present_package`); that admission plus the ledger row is the receipt.
>
> **Status:** contradiction — corrected. There is no mount-v2 byte/digest/receipt contract to conform to, so §12 item 5 has no object. A transport beyond the watched file would be a new layer with its own decision, and it remains **blocked** (`docs/SCOPE-ACCOUNTING.md`: "Streaming transport beyond the watched file"). The `surface.mount/2` string survives only in openlock and dl-enclave declarations.

### 3.2 One package family for base game and addons

The base game is one root package (or an explicitly dependency-closed set of packages if the authoritative package model requires multiple frames). Every mod/addon is another package in the same family. Both use the same package decoder, digest rules, schema rules, cell grammar, mount route, and runtime APIs.

The base package declares the stable gameplay surface and extension points. Addons extend those declared points through cells and permitted actions. An addon is not an alternate renderer or a second script runtime. It is additional cell content and behavior admitted by the same package reader and scheduler.

The package identity must be content-addressed. Package name/version are metadata; neither substitutes for the byte digest. A package that changes bytes without a valid version transition is refused by the shell. A package with an unsupported schema is refused by the shell with a named code; it is not opportunistically converted while mounting. Agents do not make either decision or emit the refusal state.

### 3.3 The reserved code-kind gate

The secure-shell decision record says the package `Code` kind is reserved until a reader exists. A playable SWF cannot be achieved by ignoring this: the source's ActionScript contains behavior, not decoration.

The port must therefore implement the predeclared code-kind reader as part of the same cell package runtime, before claiming script-driven gameplay support. The code payload should contain a deterministic, bounded cell-program representation compiled from the SWF's action semantics. The package reader validates it; the cell scheduler executes it under declared capabilities and resource budgets.

This is not a new package format or a separate runtime domain. It is the implementation of the already-reserved code lane inside the same package frame and cell runtime. Until that reader and executor exist, the shell records the code-kind path as `absent` or `refused`; no agent may smuggle executable code into an undocumented data row or claim full gameplay parity from a visual-only package. Only the shell assigns those states.

The exact row schema and bytecode/IR representation are format-owner decisions. The agent may work on the generic reader/executor contract from repository code, approved schemas, and synthetic fixtures. Any candidate is submitted through a ticket and reviewed by the format owner; the shell records the selected grammar. The agent does not derive the grammar from the SWF or receive source-derived test vectors. Builder and runtime must share the same parser/validator.

---

## 4. SWF ingestion: shell-compiles the game into cells

### 4.1 Input acceptance and source identity — shell-only

The user supplies the base SWF/SFW file to the shell. The shell's trusted importer—not an implementation agent, agent tool, MCP call, or agent-accessible process—reads the bytes and identifies the real file signature independently of the filename extension. For a standard SWF, the shell distinguishes uncompressed (`FWS`), zlib-compressed (`CWS`), and LZMA-compressed (`ZWS`) bodies. Version, declared file length, frame rate, frame count, stage rectangle, compression, and tag-stream bounds are measured and retained in shell-private provenance.

The shell preserves the original input as an immutable, read-only source file. The shell records its full digest, byte length, detected signature/version, compression mode, local path, parser build identity, and import lineage in a shell-private source record. These details are not embedded in agent tickets, PR comments, agent-visible logs, MCP responses, or the agent workspace. The agent receives no source path or original-source digest. Where an agent needs artifact identity, the secure cell carries only an opaque source/work token and the digest of the canonical cell/package artifact relevant to its ticket.

The input remains beneath the repository root in a Git-ignored and access-isolated directory. It is not merely hidden from Git: the agent provider's filesystem/workspace mount MUST exclude `inputs/` and every shell-private directory. The importer never rewrites the original file in place. No converted or finished SWF is produced or committed. A file that is not a supported SWF is classified and refused by the shell based on its actual signature; an extension is not evidence of file format.

### 4.2 Source inventory before conversion — shell-owned and shell-held

The shell performs the complete read-only inventory and classifies the source's header, shape data, display list, timelines, text/fonts, media, scripts, interaction, and external dependencies. It records exact source identities, tag and symbol names, source offsets, dependency links, script/class/function identifiers, URLs, ordering, decoded asset references, and any other source-derived detail required for correct compilation and audit.

The complete inventory is stored only in shell-private project storage. It is not exposed to agents in whole or in part. The shell may issue a secure cell containing only the minimum opaque work id, digest, coarse redacted classification, capability boundary, and abstract implementation contract required for the ticket. A redacted classification must not preserve a source name, offset, URL, script identifier, or reconstructable detail.

Unknown, unsupported, or policy-blocked source content is evaluated only by the shell. The shell records the applicable `held`, `absent`, or `refused` state and reason in its private record, and decides whether any safe, non-reconstructive work item can be issued. Agents are never asked to judge, classify, sanitize, or decide the acceptability of source content. They do not receive detailed source locations or individual unknown-tag records.

### 4.3 Compile-time intermediate representation — shell-private

A compiler IR is permitted because it exists only inside the trusted import boundary and is not mounted or streamed as a second runtime or package format. The IR may contain source-derived symbols, names, offsets, scripts, assets, event bindings, and dependency relationships because the shell needs those details to preserve semantics. Every such IR object, source map, intermediate report, and diagnostic remains shell-private and inaccessible to agent providers.

Illustrative private data model:

```text
ShellPrivateSourceMovie
  source_record
  header_and_timing
  source_symbols_and_names
  timelines_and_display_list_ops
  scripts_and_class_metadata
  decoded_asset_records
  source_input_bindings
  audio_and_media_events
  dependency_graph_with_source_locations

ShellPrivateCellCompilation
  canonical_cell_declarations
  canonical_cell_frames
  canonical_cell_programs
  canonical_asset_blobs
  extension_points
  source_to_cell_map
  unsupported_and_refusal_records

AgentSecureCell
  opaque_ticket_id
  opaque_work_and_cell_ids
  canonical_artifact_digests
  allowed_operations_and_capabilities
  declared_test_ids
  redacted_classifications
  abstract_state_transition_contract
```

Only the canonical package representation is emitted to the runtime. Private IR snapshots and reports, where reproducibility requires them, are persisted beneath the repository root in a shell-private directory; they are excluded from Git, agent mounts, MCP responses, PR attachments, and agent-visible logs. The agent sees neither the IR nor the source-to-cell map.

### 4.4 Visual and media lowering — shell-owned transformation

The shell lowers SWF vector shapes into cell-native geometry/shaper inputs and projections. It preserves meaningful scene objects, transforms, bounds, z/depth ordering, masks, hit regions, and display-list semantics. It does not flatten the game into screenshots or a video sequence.

Bitmaps, textures, fonts, audio, and any supported video material are extracted and converted by the shell into canonical package assets. Their bytes are content-addressed and referenced through the package system's existing asset-reference mechanism. If the package owner does not expose an asset-reference mechanism for a required media type, that is a named format gap to resolve before implementation—not a reason to invent an independent asset archive or send the asset to an agent for manual conversion.

The renderer may upload decoded resources to GPU/device caches, but those caches are disposable projections of mounted package assets, not an alternate source of truth. The shell owns every source-derived media transformation and any mutation of derived content. Agents do not alter, replace, redact, sanitize, or discard source or derived game assets.

Morph shapes, gradients, filters, masks, alpha, color transforms, and blend modes each require an explicit lowering or a shell-recorded unsupported state. Exact semantic preservation is the default. Approximation is not an importer fallback and is not selected by an agent. A fidelity waiver requires explicit user authorization, a named affected behavior, a documented expected difference, a shell-owned test, and a reviewed ticket/patch. Without that authorization, the shell holds the affected output rather than silently approximating or changing content.

### 4.5 Timeline lowering — shell-owned semantic compilation

The shell converts a SWF timeline into a cell-owned sequence of state transitions, not a video file. Each frame transition records the display-list operations and events required to reach the next state. Nested sprites remain independently addressable timeline entities with their own clock/state where source semantics require it. Source labels, identifiers, and exact frame-to-source mappings remain private.

The SWF frame rate is part of source semantics. Do not hardcode `ala-cities`' 20 Hz tick rate. Implement a deterministic scheduler that represents the movie's frame clock exactly (prefer rational/integer tick accumulation over floating-point drift), processes gameplay/timeline updates in source order, and lets `wgpu` render independently at the display's refresh cadence.

The scheduler must define behavior for multiple timeline advances in one display frame, dropped render frames, pause, single-step, stop, goto-frame, nested sprite clocks, and stream-audio synchronization. Those behaviors are exercised by shell-declared tests; agents do not advance or observe the running timeline directly.

### 4.6 ActionScript and gameplay lowering — shell-only source analysis and compilation

The shell determines whether the source uses AVM1, AVM2, or both. It alone reads action records, script bytes, source identifiers, names, class tables, string tables, imports/exports, and runtime references. It compiles the required source semantics into the canonical cell-program representation and retains the detailed source-to-program mapping privately.

The agent does not inspect ActionScript or source-derived script/function/class identifiers and is not asked to reverse-engineer the game's behavior. It receives only an opaque secure-cell program handle, the approved generic code-kind contract, capability descriptors, declared test identifiers, and a redacted state-transition contract. Any source-specific feature classification or test vector remains shell-held. The shell's source oracle validates semantic parity; only redacted test outcomes may be returned to an agent.

The cell runtime remains the only executor. There is no embedded Flash Player, browser page, or second compatibility window. The AVM-derived instructions are a source-language compatibility layer compiled by the shell into cell programs and run by the cell scheduler; they do not own rendering, input, package loading, or the surface.

### 4.7 Input and interaction lowering — runtime path; tests are shell-owned

The shell compiles source interaction semantics into one event graph owned by the cell surface:

```text
winit WindowEvent
      |
      v
cell input router
      |
      +--> pointer / hit-test / focus
      +--> keyboard / text input
      +--> game action bindings
      +--> cell program event queue
      |
      v
cell state transition + shell-authorized provenance/ticket
```

The cell runtime must preserve the source SWF's hit-area and button semantics. Focus, key repeat, pointer capture, dragging, wheel behavior, touch (when present), and event propagation are implemented in the client. Verification input is generated only by shell-declared tests or the user. Agents have no pointer/keyboard MCP, no CUA control, no arbitrary runtime-advance tool, and no ability to probe the game. The renderer never invents a second click path around the cell event router.

### 4.8 Persistence and determinism — shell-controlled mutation

The package defines initial game state and schema; the running game owns mutable session state. Saves are keyed to stable game/cell identities, package digest, and state schema. Unknown fields and unrelated base-game namespaces must be preserved. The original SWF and shell source records remain immutable throughout.

The shell/runtime alone selects, authorizes, stages, and executes any save migration, package replacement, addon unload, state rewrite, rollback, or recovery reconciliation. An agent may implement generic transaction machinery against synthetic fixtures through a reviewed code ticket, but it cannot inspect live saves, decide migration semantics, initiate a live migration, or perform a destructive content operation. Live changes require an explicitly authorized shell transaction with an immutable pre-operation snapshot, schema/version checks, staged candidate, before/after digests, durable audit entry, atomic commit, and rollback path.

A crash during addon replacement must leave either the old valid package/state or the new valid package/state, never a mixed half-mounted set. On restart, the shell/runtime reports a named recovery state and reconciles in-flight operations before resuming gameplay. Randomness must be seeded and recorded when deterministic replay is required. Time, event order, input sequence, and package version form part of a shell-private replay trace; agents receive only the redacted result of the declared replay test.
---

## 5. One Rust cell runtime and one surface

### 5.1 Runtime ownership

The Rust client should use `winit` for the OS window/event lifecycle and `wgpu` for GPU presentation. The cell runtime owns the actual game semantics and surface graph.

Suggested crate boundaries (names are provisional; they are not new wire formats):

| Crate/module | Sole responsibility |
|---|---|
| `package_owner` adapter | Calls the authoritative package reader/writer; no duplicate framing parser |
| `cell_mount` | Validates and mounts package rows/frames; produces one mounted cell graph |
| `swf_import` | Shell-only trusted import boundary; never exposed to agent providers |
| `swf_lowering` | Shell-only semantic lowering boundary; emits canonical package/code payloads, never source names to agents |
| `cell_program` | Executes the approved deterministic cell-program instruction set and capability checks |
| `cell_runtime` | Owns cell graph, timeline scheduler, event dispatch, state, save/recovery, and gameplay transitions |
| `cell_stream` | Consumes the existing mount-v2 delivery path, tracks receipts/revisions, and stages updates |
| `render_wgpu` | Projects the mounted cell graph into GPU draw batches, textures, fonts, and audio output |
| `client` | `winit` application lifecycle, input forwarding, resize, redraw, and user-visible notices |

These are code ownership boundaries inside one executable/surface, not independent applications or runtime domains. Consolidate modules if the existing workspace architecture calls for it; do not create empty crates merely to match this table.

### 5.2 Rendering path

The renderer reads only mounted cell state. It should transform cell projections/shapers into:

- Static and dynamic mesh/quad batches.
- Texture atlas or bind-group references for package assets.
- Font/text runs and glyph buffers.
- Layer/depth/mask/filter passes.
- Audio device commands derived from cell events.

`ala-cities` is a useful reference for GPU surface/device creation, buffer capacity management, image texture binding, text rendering, resize handling, and separation of world geometry from HUD geometry. Its renderer should be selected or adapted only after the project records the exact source files and what is intentionally not reused.

The cell package remains authoritative. GPU buffers, decoded asset caches, and display batches are disposable projections and must be reproducible from mounted package/state data.

### 5.3 Single-surface rule

There is one cell surface for:

- The base game's visual tree and interactive state.
- Addon/mod cells.
- Shared HUD, menus, inventory, map, overlays, and notifications.
- Package delivery/refusal notices.
- Developer inspection and proof output where declared by the surface.

No Flash window, browser window, second renderer, or mod-owned top-level surface is allowed. An addon may declare cell content and actions within the existing surface contract; it may not create its own display loop or independent input router.

---

## 6. Addons and mods: reuse Openlock's proven delivery mechanics in the one format

### 6.1 Addon package model

Each addon is a normal canonical cell package with:

- Stable package id and explicit version (agent-facing references remain opaque).
- Content digest and shell-controlled provenance; any source-derived provenance is reduced to opaque ids/digests and redacted classifications before it can enter a secure cell.
- Dependencies and compatible base-package identity/version range.
- Declared cells/frames and any approved cell-program content.
- Declared extension points it consumes or contributes.
- Requested capabilities and their bounded scopes.
- Optional state schema declaration and, where needed, a shell-reviewed migration proposal. The declaration does not authorize or execute a migration; only the shell may approve and run a live migration.

The shell derives and records the base package's stable extension points, such as event hooks, item/asset registries, level/scene registries, UI slots, gameplay rules, and data-driven configuration surfaces. The detailed mapping remains shell-private; agents receive only opaque extension-point handles and redacted capability contracts. Do not invent a generic plugin API unrelated to the base game's architecture.

### 6.2 Delivery transaction

The shell alone executes and owns the complete delivery transaction below. Agents and addons cannot directly validate, stage, commit, unload, replace, migrate, or roll back a live package set. An agent may submit a candidate addon artifact through an authorized ticket, but only the shell validates and applies it.

Port the following Openlock behavior to canonical cell packages:

1. Discover a package from configured/local roots or the server's package store.
2. Receive the exact package bytes through the existing mount-v2 stream path.
3. Enforce size, count, nesting, time, and resource ceilings before expensive parsing or allocation.
4. Decode the single canonical package frame; validate magic, schema, kind, declared length, digest, row grammar, cell declarations, referenced assets, and capability declarations.
5. Resolve dependencies and check compatibility with the mounted base-game package and existing addons.
6. Reject stale versions, same-version/different-content packages, duplicate identities, undeclared references, and conflicting extension-point ownership.
7. Build a complete candidate cell graph without changing the live graph.
8. Run package and state-migration checks against a staged snapshot.
9. Commit the new mounted set atomically at a deterministic cell/tick boundary.
10. Preserve the prior mounted set and its state if any validation, migration, or commit step fails.
11. Record the receipt, digest, revision, outcome, and user-visible notice; publish the change to the shell.

A package that adds zero visible windows can still change game data or behavior; therefore a delivery result must not be inferred from a window-count delta. The shell's package catalog revision and actual mounted digest determine whether state changed.

### 6.3 Mod execution permissions

A mod's cell program receives only the capabilities declared for its extension points: read/write specific game registries, subscribe to named events, add assets, contribute UI cells, alter allowed rules, or persist namespaced state. It does not receive arbitrary filesystem, process, network, or native-library access by default.

The base package owns the stable extension API. A mod cannot replace the package reader, renderer, input router, mount code, or core scheduler. The shell rejects such a request; an agent or mod does not issue refusal states or content judgments.

### 6.4 Unload, replacement, rollback, and state continuity

Unload, replacement, migration, rollback, and removal are shell-owned destructive operations. An agent or addon may not request or execute them directly. The shell accepts such an operation only through an explicitly authorized ticket/user action, after validating scope, package identity, dependency closure, state compatibility, and the preserved pre-operation snapshot. The shell stages the complete candidate, records pre/post digests, commits atomically at a deterministic boundary, and retains a rollback path. A failed operation leaves the old mounted package and state unchanged.

The runtime tracks stable identities for cells, programs, assets, and addon-owned state, while detailed source identities remain shell-private. Package reload must not reset the player's session, discard unrelated save data, or create duplicate timers/listeners. Timers, subscriptions, entities, and scheduled tasks owned by a replaced addon are handled only by the shell's approved transaction: they are migrated explicitly under a declared migration policy or retired with a recorded outcome. No agent decides what content to discard, rewrite, redact, or sanitize.

---

## 7. Security, proof, and audit discipline

The implementation inherits secure-shell's evidence-first rules:

- Read and record the authoritative package/runtime source code before writing adapters. This means the relevant repository implementation, not the SWF/SFW source movie; the movie remains shell-only.
- One owner for each format/parser/store/runtime layer.
- No second package reader and no new magic/version tag without an explicit format-owner decision.
- Only the shell/runtime fails closed and emits refusal records. Agents do not refuse tickets or content and do not sanitize, redact, delete, or rewrite content. A refusal names the package, record, opaque source reference, code, and shell-held reason. The agent receives only the redacted secure-cell outcome authorized by its ticket.
- No silent no-op for an unsupported SWF instruction or addon action.
- Every build, package, mount, migration, test, and refusal is recorded in the project ledger.
- The client is the test surface for claims that concern mounted rendering, actual input, gameplay, and addon behavior. Execution and observation are performed by the shell/test harness or user, never by an agent CUA session.
- A headless test is useful for parser/VM/package unit properties, but it is not proof that the client mounted and played the game correctly. Raw client evidence remains shell-held; agent-visible results are redacted semantic assertions only.

The code-kind implementation is a security boundary. It needs deterministic instruction budgets, memory limits, recursion/stack limits, cancellation, capability checks, and deterministic failure reporting. A malicious or malformed mod must not hang the render loop or mutate half the live game state.

For gameplay changes, the runtime should use typed commands/tickets or the equivalent project-authorized operation boundary. The action is validated before mutation and the resulting world/state is read back. The ledger is evidence of the operation; a screenshot or an agent's confidence is not.

---

### 7.1 Project establishment: mandatory patch baselines

Before importing or implementing game behavior, establish a small set of normative patch baselines inside the new repository. These are not retrospective notes. They are the project's first accepted constraints and must be checked by the repository's own gate.

| Baseline patch | Required rule | Mechanical evidence |
|---|---|---|
| `patches/baselines/PATCH-0001-version-control.md` | Initialize Git immediately; commit small coherent units early and often. Every commit cites its ticket and ledger entry. Work happens on ticket branches and is reviewed through pull requests; do not accumulate a large uncommitted implementation or squash away the useful sequence. | Initial baseline commit exists before implementation work; commit-size/drift gate; commit-to-ticket and commit-to-ledger references; PR checklist and review record. |
| `patches/baselines/PATCH-0002-ticket-and-review-discipline.md` | Every implementation step, defect, shell-issued refusal, unknown, and design decision gets a ticket. Use PRs for scoped changes and use ticket/PR comments continuously to record source-safe findings, evidence, questions, decisions, and review responses. Only the shell creates refusal-state records or decides content status. Comments supplement—not replace—durable code, tests, patch records, or ledger entries. | Each active work item has a ticket; each nontrivial change links its ticket and PR; unresolved blocking review comments block merge; completed tickets point to the merged commit and shell-authored witness. |
| `patches/baselines/PATCH-0003-source-and-build-artifact-boundary.md` | Never commit the original, intermediate, or finished source-movie files (`*.sfw` or `*.swf`). The source SWF is a local, immutable, read-only shell input; its digest, byte length, signature, and project-relative location are recorded only in shell-private provenance, not in agent-visible Git files or comments. The intended product is the canonical cell package, not a rebuilt SWF. | `.gitignore` and a pre-commit/CI gate reject every `*.sfw` and `*.swf` path from the Git index; filesystem/process policy excludes the input from agent workspaces; shell audit confirms the private source record matches the local input. |
| `patches/baselines/PATCH-0004-project-local-artifacts.md` | Every generated output, including shell-private compiler IR, source inventories, package artifacts, logs, captures, CUA traces, reports, and test results, is written beneath the repository root. Shell-private outputs remain inaccessible to agent providers. Do not use `/tmp`, `%TEMP%`, an OS cache, a sibling checkout, or an external scratch directory as an output destination. | A path-containment gate canonicalizes every configured output path and refuses paths outside the project root; ACL/workspace tests prove shell-private paths are not agent-readable; build/test/MCP configuration uses project-relative destinations. |
| `patches/baselines/PATCH-0005-one-cell-surface-and-format.md` | The SWF-derived base game and all streamed mods use the same canonical cell package, mount path, cell graph, scheduler, and single rendered surface. No SWF compatibility surface, alternate mod format, or second executor. | Architecture gate checks one package owner, one cell runtime, one surface identity, and no raw-SWF execution path. |
| `patches/baselines/PATCH-0006-abstract-mcp-cua-axis.md` | Establish an agent-facing MCP surface limited to ticket, secure-cell, and shell-declared-test operations. The CUA Axis driver is shell/test/user-owned and is never exposed to agents. It uses generic observation/input mechanics internally, without game-specific control scripts, guessed selectors, hardcoded gameplay logic, or a parallel implementation of the SWF. Source-specific test plans and observations stay shell-private. | MCP schema tests prove there are no agent-facing capture, observation, pointer, keyboard, runtime-advance, CUA, or probe tools. Axis tests prove generic mechanics and shell-only access. The client witness and raw action trace remain shell-private; the agent receives only a redacted semantic test result. |

These six patches are established in the first project baseline and treated as hard gates, not aspirational style guidance. A change that weakens one requires a new, explicit user decision and a superseding patch; an implementation agent cannot silently amend the baseline it is being measured against.

#### Git, tickets, pull requests, and comments

The required working loop is:

1. Create the repository, `AGENTS.md`, `.gitignore`, ticket templates, PR template, baseline patch records, and the first ledger record. Commit this scaffold immediately.
2. Create one issue/ticket for each bounded artifact or defect. Give each task a named owner, source-safe scope, acceptance test, shell-owned refusal/gating conditions, and dependencies. Do not put source-derived identifiers or private inventory references in agent-visible tickets.
3. Create a branch from the current mainline for each ticket or tightly coupled ticket group. Use ticket ids in branch names, commits, and PR titles.
4. Commit each coherent step as soon as its own tests and evidence exist. Small corrective commits are preferred over one end-of-day commit. A partial implementation may be committed only when its incomplete state and failing/absent checks are explicit.
5. Open a pull request for each nontrivial change. Keep PRs narrow enough to review against their ticket and baseline patch. Do not merge unresolved blocking comments.
6. Use issue and PR comments as the live deliberation record: post repository code paths/line references, source-safe command summaries, alternatives, questions, reviewer objections, decisions, and links to ledger events as they occur. SWF paths, source offsets, URLs, script names, and shell-private inventory references are prohibited from agent-visible comments. Do not leave the reasoning only in private agent context.
7. Merge only after the required reviewer and automated gates pass. Preserve the commit series; do not squash away the audit trail where the repository policy requires each unit to remain attributable.
8. Close the ticket with the merged commit, the shell-assigned final gate state (`held`, `absent`, or `refused`), a redacted client-test result or explicit absence, and the ledger event. The agent cannot set the gate state or attach raw witness material.

A ticket comment or PR discussion is not itself proof that code works. The executable test, mounted-client witness, digest, and ledger record remain authoritative.

#### Source SWF and output-directory contract

The source file is stored under the shell-only `inputs/` directory, preserving the supplied filename and bytes. The exact path and filename are held only in the shell-private source record and are not put in agent-visible documentation, tickets, comments, or logs. The shell identifies the binary signature during ingestion; it does not rename bytes merely to satisfy a suffix. Never write a converted or "finished" SWF.

All generated material stays inside the checkout, for example:

```text
project/
  AGENTS.md
  .gitignore
  docs/
  patches/
    baselines/
  tickets/
  baselines/                    # source-safe synthetic fixtures/contracts only
    synthetic/
    generic-test-contracts/
  inputs/                       # supplied SWF/SFW; exact name/path shell-private
  shell-private/                # project-local; ACL-isolated; never agent-mounted or committed
    source-records/
    inventories-and-source-maps/
    decoded-assets/
    compiler-ir/
    canonical-package-artifacts/
    package-provenance/
    source-oracle/
    visual-baselines/
    raw-test-traces/
    detailed-logs/
  artifacts/                    # agent-safe or explicitly redacted artifacts only
    redacted-coverage-summary.json
    secure-cell-contracts/
    redacted-test-results/
  generated/                    # canonical packages; shell/runtime only, no agent filesystem access
    cell-packages/
  reports/                      # redacted/source-safe reports only
  logs/                          # redacted/source-safe logs only
  captures/                      # shell/test/user only; never agent-readable
  target/                        # ordinary local build output; project-local; no source payloads
```

The directory names are illustrative, not a new package format. The output-root policy is the invariant: all writes resolve beneath the project root. If atomic replacement needs a staging file, create it beside the final destination inside the same project tree, then rename it atomically. Do not stage to an external temporary directory and copy it in later. Runtime caches must also be under a project-owned directory or kept in memory.

Ignore rules may keep large generated artifacts out of Git, but they must not redirect them outside the repository. Access controls must additionally keep `inputs/`, `shell-private/`, and any raw `captures/` directory inaccessible to agent providers; Git exclusion alone is insufficient. Keep manifests, schemas, reports, baselines, test definitions, patch records, and required reproducibility metadata versioned. Review binary package outputs individually for size, licensing, and whether they belong in Git. `*.swf` is never an exception to that rule.

#### 7.2 Ticket-mediated MCP and secure-cell test boundary

The MCP server is a shell-owned boundary for issuing secure work and requesting shell-declared tests. It is not a game-control API, source-inspection API, package browser, CUA bridge, or observation surface for agents.

The agent-facing MCP schema is limited to:

- `schema.describe`: schema discovery only; no runtime or source information.
- `ticket.submit`: submit an internal cell ticket with requested scope, implementation operations, and capability needs. The shell decides what may be granted.
- `secure_cell.get`: retrieve the shell-issued secure cell for that ticket.
- `test.list_declared`: list only shell-declared test IDs and redacted descriptions already authorized for that ticket.
- `test.run_declared`: request that the shell execute one listed test. The agent cannot provide input values, action sequences, selectors, coordinates, probes, or a substitute test body.
- `test.result.get`: retrieve the shell-produced redacted semantic result attached to that ticket.

These are the only agent-facing operation families. `package.inspect`, generic surface descriptions, source inventory endpoints, general evidence browsing, and all other observation or control endpoints are absent. Any package identity needed by the agent is included in the secure cell as an opaque id and canonical artifact digest. Any refusal classification is shell-authored and read-only; the agent cannot create or alter it.

The agent-facing MCP surface MUST NOT expose `surface.capture`, screenshots, frame/video capture, visual observation, `input.pointer`, `input.keyboard`, text entry, `runtime.advance`, event injection, CUA session handles, arbitrary probes, free-form test definitions, or game-specific selectors/coordinates/action maps. It must not expose a generic tool that can be repurposed to perform those actions indirectly.

The CUA Axis driver is a separate shell/test-owned component. It may be used internally by the shell to execute a predeclared test plan or a user-authorized witness flow against the actual Rust client. Its inputs are test plans issued by the shell; its observations and captures remain private to the shell/test system. It returns only the test's redacted semantic result. Neither the driver nor its CUA session is callable, controllable, inspectable, or observable by an agent provider.

The Axis driver remains abstract: it contains generic window/input/observation mechanics only. Game-specific expectations live in shell-held test declarations generated from or checked against the private source inventory; those declarations are not agent-visible and do not become hardcoded game logic in the MCP server or driver. Agents cannot create probes, read the live surface, or perform a judge/controller loop. The shell separates test execution from evidence evaluation, and neither role is delegated to the agent.

**Agent work loop:**

1. Submit a ticket that identifies the generic implementation task and requested capabilities without source identifiers.
2. Retrieve the shell-issued secure cell and operate only on the approved code/package contract and synthetic fixtures.
3. Submit implementation changes through the ticket branch and pull request process.
4. Request only tests already declared for the ticket.
5. Read only the redacted semantic test result returned by the shell.

**Shell/test work loop:**

1. Resolve the source record and full test specification inside shell-private storage.
2. Run the client and, where authorized, the source player/oracle and internal CUA Axis driver.
3. Keep screenshots, captures, raw inputs, source traces, names, offsets, URLs, and detailed state diffs inside shell-private storage.
4. Compare observable semantics and compute a redacted result with an opaque test id, pass/fail or shell status, canonical artifact digest, and only the minimum abstract state assertion needed by the ticket.
5. Attach that result to the secure cell; never attach raw or reconstructable source evidence.

No game-specific button names, coordinates, sequences, level rules, score rules, item semantics, action maps, guessed object paths, source function names, or source-derived test vectors may be hardcoded into agent-facing MCP code or the Axis driver. The driver is generic; all source-specific control and oracle knowledge stays within the shell/test boundary.

The client witness is real, but it is not agent observation. The shell/test system witnesses the mounted surface; the agent receives only the declared, redacted result.

### 7.3 Trust-boundary data classes and visibility matrix

| Data class | Shell/runtime | Agent provider | Required handling |
|---|---|---|---|
| Original SWF/SFW bytes and decompressed body | Read-only access | No access | Local under `inputs/`; shell credential/ACL boundary; never Git-tracked |
| Full source inventory, tags, names, offsets, URLs, scripts, dependency graph | Full access | No access | Shell-private store; never ticketed, logged, or attached to a PR |
| Decoded source media, source-player output, screenshots, captures, raw input traces | Full access when authorized | No access | Shell-private; no MCP capture or CUA session exposure |
| Compiler IR and source-to-cell maps | Full access | No access | Shell-private intermediate representation; not mounted or streamed |
| Canonical package/cell artifacts | Full access | Ticket-limited opaque digest/handles only | One canonical format; secure-cell capability checks |
| Secure-cell work contract | Issued/controlled | Ticket-scoped access | Opaque ids, relevant canonical digests, allowed operations, declared tests, redacted classifications |
| Source-oracle/test declarations and detailed vectors | Full access | Test IDs only | Shell/test-owned; never agent-invented or source-reconstructable |
| Test result | Full result | Redacted semantic assertion only | Shell-generated; no raw trace/image/detailed diff |
| `held` / `absent` / `refused` status | Assigns and records | At most a generic read-only gate outcome, without private reason or source reference | Shell-authored state; never an agent judgment or output |
| Package unload/replacement/state migration/rollback | Executes transaction | No direct invocation, proposal approval, or live-state access | Shell-authorized by user/policy, staged, digested, auditable, reversible |

This table is normative. Any additional data returned to an agent requires a reviewed schema change and an explicit source-safety analysis. “Redacted” is a shell operation; the agent does not receive unredacted material and is not asked to redact it.
---

## 8. First-ten-hours execution sequence adapted to this port

This sequence adopts the structure of `secure-shell/docs/FIRST-TEN-HOURS.md`: one artifact per block, an exit test for every block, an explicit refusal list, a 90-minute checkpoint, and a ledger entry before the next block. It does not authorize parallel feature work. If the original secure-shell H1–H10 gates are active in the target repository, they remain authoritative; this table specializes the same discipline to the SWF game port.
H1–H10 sequence does not authorize trimming. Each block must instantiate every state/refusal/path named for that artifact, or explicitly record it as missing.

| Block | One artifact | Exit test / required evidence | Refuses |
|---|---|---|---|
| **H1 — project scaffold and patch baselines** | `patches/baselines/` plus `docs/PORT-BASELINE.md` | Git initialized and first small commit landed before implementation; all six baseline patches exist; `.swf` exclusion and project-root output gate pass; issue/PR templates and ledger are usable. | Starting import/render work before the rules are committed; storing source SWF in Git; writing outputs outside the checkout; a single giant initial commit. |
| **H2 — package-format ownership** | `docs/PACKAGE-OWNERSHIP.md` | Names the authoritative package crate, exact frame and row readers/writers, cell declaration/frame owner, mount-v2 owner, selected repository code paths (not SWF source paths), rejected alternatives, and evidence that builder and acceptor share grammar. | New magic, new `secure-shell.*` game format, second reader, or copying the Java reader as a competing Rust parser. |
| **H3 — drift gate and repository workflow** | `tools/port_gate` (or existing gate extension) | Mechanically detects duplicate package readers, alternate surfaces, unrecorded format tags, tracked `.swf` files, output paths outside the project root, missing ticket/PR references, oversized commits, and plan steps without exit tests. | A prose-only plan that cannot reject drift; direct-to-main implementation; untracked work accumulating outside the queue. |
| **H4 — shell-owned SWF source inventory** | Shell-private source inventory and source-map store; ticket-scoped redacted secure-cell definitions | Shell-only read-only parse; complete source names, offsets, URLs, script identifiers, media, and dependency graph remain shell-private. Agent receives only opaque work ids, canonical artifact digests, and coarse redacted classifications. | Agent inspection of raw SWF; agent-visible inventory files; source identifiers in tickets/logs; lowering before shell inventory. |
| **H5 — shell compiler contract and code-kind reader** | `docs/CELL-PROGRAM-DECISION.md` plus shell-private compiler/test artifacts | Format-owner decision and generic code-kind implementation contract; shell privately builds source-derived AVM1/AVM2 coverage and test vectors. Agent work uses synthetic fixtures and generic schemas only; the agent never receives source-derived instruction samples or identifiers. | Smuggling code into data rows, inventing a second runtime, exposing source-derived vectors, or claiming script parity before the reserved code lane has a reader. |
| **H6 — ticket/secure-cell/test-only MCP pilot** | `docs/MCP-SECURE-CELL-CONTRACT.md` and shell-private test records | Agent submits a ticket and receives only its secure cell; it can request only already-declared shell tests and read redacted semantic results. CUA Axis, controls, captures, source oracle, controller, and judge remain shell/test/user-side. | Agent screenshots, observations, CUA handles, pointer/keyboard/runtime tools, invented probes, source inventory or game-specific MCP logic. |
| **H7 — visual baseline freeze — shell/test/user only** | Shell-private client/source visual baseline store | Shell/test/user witness captures at declared resolutions; all images, frame data, comparison artifacts, and capture identities remain shell-private. Agent receives only a redacted semantic assertion and canonical artifact digest, never an image or capture reference. | Agent viewing/taking screenshots; agent access to captures; screenshot-driven agent retuning; source captures in tickets or PRs. |
| **H8 — shell-owned semantic coverage matrix** | Shell-private full coverage matrix plus redacted aggregate report | Every source feature maps to its canonical cell implementation, shell status, and private witness; exact source locations and identities remain shell-held. Zero unclassified source features. Agent receives aggregate/redacted completion assertions only. | Agent-visible source locations or feature identifiers; reporting a percentage that hides missing semantics; treating a shell `absent` state as a pass. |
| **H9 — first playable integration witness** | One end-to-end base-game integration path within the full package/runtime contract | Shell builds/mounts the canonical package; user or shell-declared tests exercise the client; the shell verifies the state transition and emits a redacted result. This is an integration witness, not permission to limit product scope to one slice. | Agent-driven probes or observation; treating the slice as the final product; browser-only witness; second surface; hardcoded agent-side game behavior. |
| **H10 — streamed addon transaction witness** | One addon cell package plus shell-owned transaction/test record | Shell delivers the addon through the same stream/frame format, verifies an extension, tests stale/tampered replacement, and exercises authorized unload/rollback while preserving base-game state. The shell/test Axis driver performs the witness; the agent sees only redacted test results. | Separate `.shpkg`/`openlock.addons/2` route, alternate renderer, agent-triggered replacement/unload/migration, state reset, or game-specific MCP logic. |

### 90-minute checkpoint fields

At the end of each block, the shell records exactly the checkpoint discipline from `FIRST-TEN-HOURS.md`. Agent-authored notes may document source-safe implementation work, but the shell alone assigns gate states and records source-derived evidence:

1. User goal sentence, verbatim.
2. The one artifact changed in this block.
3. Last shell-assigned gate state: `held`, `absent`, or `refused` (never an agent judgment).
4. Drift signal observed, if any.
5. Explicitly refused work for the next block.
6. Next artifact.
7. Ledger entry id for the completed block.

The next block is not authorized until the entry exists and can be verified. The shell owns the authoritative checkpoint and gate state; an agent may not mark a source feature `held`, `absent`, or `refused`.

---

## 9. Required test matrix

### 9.0 Project governance, source exclusion, and output locality

- The repository has a first commit before implementation work and a series of small, attributable commits thereafter.
- Every active work item has a ticket; every nontrivial change has a PR; commit, ticket, PR, comments, and ledger references resolve to one another.
- Blocking PR comments prevent merge; reviewer responses and accepted/rejected decisions are preserved in the PR thread.
- Git-index inspection finds no original, intermediate, or finished `*.sfw` or `*.swf` file.
- The shell-private source digest manifest matches the local, read-only input file; source bytes, source digest, and local source path are not required in Git or exposed to the agent provider for reproducibility claims.
- Every configured output path is canonicalized and proven to remain beneath the project root, including shell-private source inventory/oracle destinations, shell-only capture destinations, logs, reports, compiler output, generated cell packages, and build staging. Project-local storage does not imply agent access: `inputs/` and shell-private paths are excluded from agent workspace mounts and process permissions.
- Atomic staging files are created within the destination's project-local directory; no output depends on `/tmp`, `%TEMP%`, an OS cache, a sibling repository, or an external scratch tree.
- A deliberately configured outside-root output path is refused by the gate.
- Deliberately staged `.sfw` and `.swf` files are rejected by the pre-commit and CI checks.

### 9.0.1 MCP and CUA Axis abstraction

- MCP requires a valid internal cell ticket for every non-schema call.
- The shell returns a secure cell containing only opaque identifiers, canonical artifact digests, allowed capabilities, declared test IDs, and redacted classifications/contracts.
- Agent-facing MCP contains only ticket, secure-cell, and declared-test operations; it has no package-browsing, source-inventory, screenshot, capture, visual-observation, pointer, keyboard, runtime-advance, CUA-session, arbitrary-probe, or free-form input tools.
- Declared tests are authored/approved and executed by the shell; agents may request only a test already declared for their ticket.
- The CUA Axis driver is not agent-accessible. It uses generic mechanics internally under shell/test/user authorization and contains no game-specific logic.
- All screenshots, captures, raw input traces, source-oracle material, and detailed client observations remain shell-private. They are never attached to agent tickets, MCP results, logs, or PRs.
- Client witness is produced by user/tests/shell. The agent receives only a redacted semantic assertion and canonical artifact digest.

### 9.1 Package and cell-format conformance

- Known-good frame round-trips to identical payload bytes and digest.
- Bad magic, unsupported schema, unknown kind, truncation, overlong payload, mismatched digest, malformed row, unknown required field, bad cell reference, missing asset, duplicate id, and trailing bytes each produce a named shell refusal record; agents cannot emit or alter those records.
- Builder and runtime reader accept/reject the same declaration and cell grammar.
- A code-kind package cannot mount until the code reader and execution gate are implemented and registered.
- A package with a valid digest but unsupported code capability is refused by the shell; digest validity alone never implies safety.

### 9.2 SWF semantic parity — shell/test-owned oracle

The actual SWF is the reference oracle, but it is available only inside the shell/test trust boundary. A trusted source player or source-behavior harness may be run by the shell to produce private deterministic traces. The Rust cell port is tested against those traces inside the shell. Agents never receive the source movie, source-player session, oracle traces, source captures, raw input sequences, source identifiers, or detailed diffs.

The shell compares observable semantics across the complete required feature set, including:

- Stage dimensions, frame rate, and timeline progression.
- Sprite/frame and nested timeline transitions.
- Display-list ordering, transforms, masks, hit areas, and color changes.
- Button states, pointer transitions, keyboard controls, focus, and event ordering.
- Script values, branch outcomes, timers, and state transitions.
- Collision/contact behavior, scoring, inventory, lives, checkpoints, menus, and level transitions.
- Audio event order, loop, stop, volume, and stream synchronization.
- Save/load, restart, and deterministic replay.

The source-side test plan, feature-to-source mapping, baseline captures, and full result trace stay shell-private. An agent receives only the declared test id, a redacted pass/fail or shell-assigned state, a digest of the relevant canonical artifact, and a minimal abstract assertion such as “timeline transition contract satisfied.” It does not receive raw values, names, screenshots, offsets, URLs, or a diff from which source details can be reconstructed.

Visual comparison is necessary but insufficient. A game that looks right while accepting the wrong inputs or losing state is not a successful port. The oracle is a shell/test authority, not an agent-readable specification or agent-visible evidence source.

### 9.3 Streaming and addon transaction tests

- Add a new addon; confirm one package revision and one mounted cell-set change.
- Deliver a newer version; confirm one atomic replacement.
- Deliver an older version; confirm refusal and the previous version remains mounted.
- Deliver same version with different bytes; confirm refusal.
- Tamper with the frame; confirm digest refusal before live state mutation.
- Deliver a package with a missing dependency or conflicting extension point; confirm explicit refusal.
- Force failure during staging, migration, and commit; confirm rollback leaves the old mounted set and save state unchanged.
- Remove an addon; confirm only its owned contributions are removed.
- Disconnect/reconnect during delivery; confirm catch-up from the authoritative mount/ledger state without duplicate application.
- Deliver an addon with no visible window but with valid game-state content; confirm success is derived from package state, not window count.
- Stress large asset packages; confirm bounded work per frame and no render-loop starvation.

### 9.4 Client witness — shell-private evidence with redacted result

The real Rust client must run the checks over the mounted cell surface. The shell/test record retains:

- Exact command, environment identity, and exit status.
- Check count and machine-readable output digest.
- Package digest, mount receipt/revision, surface id, and mounted package set.
- Private input trace and resulting game-state digest for gameplay tests.
- Private source-baseline/capture identity and source-oracle comparison record for parity tests.
- Any screenshots, frame captures, video, raw state diffs, or source-derived references needed by the shell's audit.

None of those raw witness materials is exposed to the agent provider. The agent receives only the ticket-authorized test id, redacted semantic result, and canonical artifact digest. A headless parser test can pass while the client still fails to mount the content; those are separate shell-recorded claims and must be reported separately.

---

## 10. Completion criteria

The port is not complete when it displays the SWF's first frame. It is complete only when all of the following are witnessed:

1. The shell holds a reproducible, content-addressed import record for the immutable original SWF; the source digest, path, and full provenance remain private.
2. The shell has classified all required source tags, assets, timeline constructs, script constructs, external dependencies, and game-specific behaviors in its private coverage record.
3. The base game is emitted through the canonical cell package format; no raw-SWF runtime path exists in the client.
4. The cell package reader and code-kind executor are the only runtime path for both base game and addons.
5. One Rust `winit`/`wgpu` client mounts and renders one cell surface.
6. The player can perform the game's meaningful original interaction loop and observe the correct state transitions.
7. Gameplay state survives save/restart and package replacement according to the declared persistence rules.
8. At least one addon is streamed in through the same package format and changes gameplay through a declared extension point.
9. Invalid, stale, malicious, or incompatible packages are refused without changing the currently mounted game.
10. The client has witnessed the actual gameplay and addon paths; backend tests and screenshots alone are not substituted for that witness.
11. The project begins with the six mandatory patch baselines, a ticket/PR/commit/ledger workflow, and mechanical gates for their invariants.
12. The source and every generated output remain project-local, while no `.swf` binary is ever committed.
13. The agent-facing MCP surface is limited to ticket/secure-cell/declared-test operations; the shell/test-owned CUA Axis driver privately verifies the real client through generic mechanics and returns only redacted semantic results.
14. No agent provider can access raw SWF/SFW bytes, source-derived names/functions/URLs/offsets, detailed inventories, compiler IR, decoded media, source-player output, screenshots, raw traces, or source-to-cell mappings.
15. Every agent work unit is mediated by an internal cell ticket and a shell-issued secure cell containing only opaque ids, canonical artifact digests, allowed capabilities, declared test ids, and redacted classifications/contracts.
16. Only the shell assigns `held`, `absent`, or `refused` states and decides content acceptability.
17. Only the shell may perform source-derived transformations, content redaction/withholding, package unload/replacement, state migration, rollback, or other destructive content operations.
18. The full named contract is implemented or explicitly recorded as missing; H1-H10 verification slices do not authorize product-scope reduction.

---

## 11. Explicit non-goals and prohibited shortcuts

- No SWF-to-PNG, screenshot sequence, or video conversion as the game port.
- No embedded Flash Player, browser runtime, or second compatibility application.
- No split where the base game uses one runtime and addons use another.
- No second package magic, package row grammar, asset archive, or window/surface format.
- No direct mounting of arbitrary SWF bytes in the runtime after compilation.
- No unrestricted execution of ActionScript or native addon code.
- No second renderer, input router, scheduler, or package store.
- No visual-only success criterion.
- No silent no-op for unsupported tags/actions/APIs.
- No destructive hot reload that resets the game or silently discards addon/base state.
- No claiming that the current `dl-openshell` cell parser is already functional: its current code explicitly refuses cell raster/frame mounting.
- No claiming a successful port from source inspection alone: the actual SWF and a live client witness are required.
- No committing the source, intermediate, or finished `.sfw`/`.swf` binaries. Full source digests, filenames/paths, inventories, source maps, and provenance remain shell-private; only source-safe canonical artifact digests or explicitly redacted provenance summaries may be versioned.
- No outputs outside the repository root, including OS temp folders, external scratch locations, or sibling checkouts.
- No direct-to-main implementation, giant unreviewed change sets, or nontrivial changes without a ticket and pull request.
- No game-specific behavior, key map, selector, coordinate recipe, or gameplay assertion hardcoded into the MCP server or CUA Axis driver.
- No MCP/CUA action without a ticket, declared action set, and durable evidence record.
- No agent reading, parsing, inventorying, naming, or summarizing the source SWF/SFW or any source-derived representation.
- No agent access to source paths, raw/decompressed bytes, source names, offsets, URLs, script identifiers, decoded media, detailed IR, source maps, or source-oracle data.
- No agent screenshots, frame captures, video captures, surface observation, pointer/keyboard control, runtime advance, CUA session, or invented probes.
- No agent refusal state, content-acceptability judgment, sanitization, redaction, deletion, rewriting, quarantine, unload, replacement, migration, or destructive state operation.
- No raw source content, source-derived identifiers, source captures, or reconstructable source evidence in agent-facing MCP, secure cells, tickets, logs, or PRs.
- No use of Git-ignore rules as a substitute for excluding `inputs/` and shell-private storage from agent filesystem/workspace access.
- No interpreting the first playable integration witness as permission to omit the remaining named system contract.


## 12. First implementation decision the agent must resolve

Before writing an adapter, package parser, or code-kind reader, an implementation agent may inspect the authoritative package crate identified by the secure-shell format audit (repository implementation source only; never the SWF/SFW source movie). The format owner and shell must establish the exact, current schema for:

1. `V4PKG001` frame data/code kinds and row grammar;
2. `shploingus.cells/1` and `shploingus.cellframe/1` ownership and supported features;
3. the existing artifact/asset reference mechanism;
4. the cell-program/code-kind reader status;
5. the mount-v2 byte/digest/receipt contract.

The audit of the four public repositories does not expose the full authoritative `planet-package` Rust source. The secure-shell audit identifies it in the broader workspace, but that source must be read directly in the target environment before the agent writes a parser, schema, or bytecode grammar. If it cannot be located, the shell records the package layer as `absent` and implementation stops at the design evidence; the agent does not recreate it from memory. The shell-held SWF inventory is not made available as a substitute.

> **ERRATUM E-3 — supersedes §12 item 4's premise and §3.3's "until a reader exists" (measured 2026-11-02).**
>
> **Superseded premise, kept visible:** item 4 asks for *"the cell-program/code-kind reader status"* as an open unknown, and §3.3 states the code kind "is reserved until a reader exists" and that "until that reader and executor exist, the shell records the code-kind path as `absent` or `refused`".
>
> **Measured:** the **frame reader for code already exists.** `FrameKind` declares `Code = 2` and `from_u16` maps it, `Package::decode` accepts both kinds, and `package build --code` writes one (`crates/planet-package/src/frame.rs`, `src/bin/package.rs`). What does not exist is an **executor**: nothing in the tree interprets a code frame's payload as behaviour, the shell refuses a code frame on the adoption path (`crates/planet-shell/src/mount.rs`, `if package.kind == FrameKind::Code` — refused, declaration in force untouched), and the door will still present one (`crates/planet-mcp/src/surface.rs`, the `FrameKind::Code` case).
>
> **Consequence:** the missing piece is the executor, not the reader. §3.3's rule — no script parity until an executor exists — stands unchanged.
>
> **Status:** contradiction — corrected on the reader's existence. The executor's shape, instruction budgets and capability set are an **open decision** (H5, **blocked**; `docs/PORT-BASELINE.md` H5). Nothing here authorises one.

> **ERRATUM E-4 — supersedes §12 item 3 and §4.4's "existing asset-reference mechanism" (measured 2026-11-02).**
>
> **Superseded wording, kept visible:** *"the existing artifact/asset reference mechanism"* (item 3) and §4.4's *"referenced through the package system's existing asset-reference mechanism"*.
>
> **Measured:** the port's family has no asset-reference mechanism to reuse. Media travel as canonical bytes inside a single row field: `Kind::Bands` and `Kind::Materials` are `Family::Value` kinds with exactly one field each (`crates/planet-package/src/rows.rs`), and `package build --bands <path>` / `--materials <path>` read a file's bytes straight into that field (`src/bin/package.rs`). A media type with no kind has nowhere to go. The nearest artifact table is in the other family — `SHPKG003` adds one (`crates/voxel-package/src/recipe.rs`: `MAGIC_ARTIFACTS`, `artifacts_in`) — which is precisely the second format this plan forbids adopting.
>
> **Consequence:** §4.4's own escape clause — "if the package owner does not expose an asset-reference mechanism for a required media type, that is a named format gap to resolve before implementation" — now has its answer: this **is** that gap. Adding a `Kind` is a format-owner act (a row in `rows.rs` plus a line in the format document), never an import detail.
>
> **Status:** contradiction — corrected in that no mechanism exists. Which kind(s) carry media is an **open decision** for the format owner, recorded as blocked in `docs/SCOPE-ACCOUNTING.md`.

**Errata status note.** §12's closing condition — read the authoritative crate directly before writing a parser — was satisfied at H2 (TICKET-0004, TICKET-0005): `planet-package`, `planet-core`, `voxel-package` and the Java `PackageFrame` were read, and `docs/PACKAGE-OWNERSHIP.md` records the ownership and the compatibility measurements. Its fallback sentence ("the shell records the package layer as `absent`") is therefore not in force: the layer is `present`, and the paragraph below is retained as the condition that was met, not as a pending instruction.

**Final architecture rule:** the SWF is compiled into the cell package; the cell package is mounted into the one surface; the one cell runtime executes both the base game's behavior and streamed addon behavior. Openlock contributes the proven delivery transaction mechanics, secure-shell contributes the stream/receipt/evidence discipline, the civ-bots prototype contributes strict framed-package and state-continuity examples, and ala-cities contributes the Rust window/GPU foundation. None of those projects licenses a second runtime or a second content format.

> **ERRATUM E-5 — supersedes the remaining "existing mount-v2" references (measured 2026-11-02).**
>
> **Superseded:** "existing mount-v2 transport path"; "Use the existing mount-v2 stream/receipt path as the transport and mount boundary"; "Consumes the existing mount-v2 delivery path"; "Receive the exact package bytes through the existing mount-v2 stream path."; "the mount-v2 byte/digest/receipt contract."
>
> **Measured:** no mount-v2 byte/digest/receipt contract exists in the authoritative tree.
> `crates/planet-shell/src/mount.rs` adopts a watched file (`Mount::watch`), and
> `crates/planet-package/src/frame.rs` carries the frame digest. `docs/DECISIONS-PORT.md` D-4
> therefore rules the streamed transport a new layer, explicitly not mount-v2; E-2 corrected §3.1,
> and the sentences quoted above in §1, §2.5, §5.1, §6.2 and §12 instruct the opposite. They are
> superseded by D-4 and E-2, and the original wording stays visible above.
>
> **Status:** contradiction — corrected. The transport remains a new, undecided layer
> (`docs/SCOPE-ACCOUNTING.md`), and no implementation may assume a mount-v2 path.
---

## 13. The shell family's operating discipline (added 2026-11-02)

`secure-shell/docs/FIRST-TEN-HOURS.md` and `secure-shell/docs/PLAN.md` are the authority for *how* work
is sequenced in this repository (D-6). Four rules change the blocks below; none changes a block's
subject. The plan body and the errata above are unchanged, because an addition is not a correction.

### 13.1 A unit with no ledger entry did not happen

Every block ends with a `ledger/ledger.jsonl` row naming the command, its exit status, the check count
and a digest — not a paragraph saying it was done. `exit_test` and `evidence` are the fields that hold
them, and a row whose `evidence` names no runnable command is incomplete.

### 13.2 The checkpoint's seven fields

Before a block starts: the goal sentence **quoted** from this plan, the one artifact, the last gate's
answer in its three-way words, the drift signal, the refused list for the next block, the next artifact,
and the ledger id just written. The rows already carry five (`goal`, `artifact`, `drift_signal`,
`refused_next`, `next_artifact`); the quoted sentence and the gate's answer are carried inside `goal`
and `exit_test` so no row needs a new key.

### 13.3 The gate answers three ways, and only `refused` fails

`held` — checked and true. `absent` — the subject does not exist yet, named with what was looked for; it
is a state, not a pass and not a failure. `refused` — checked and false. `tools/port_gate.py`'s `PASS` /
`NOT-ENFORCED` / `FAIL` map onto these exactly, and this plan uses the shell's words from here. The four
`NOT-ENFORCED` rows are `absent`, and none of them may be read as `held`.

### 13.4 Gates this repository does not have yet

| Gate | Refuses | Owner block |
|---|---|---|
| `squash` | a commit over 40 files or 4,000 insertions | H1 |
| `step-without-exit` | a plan step with no exit test | H2 |
| `lost-object` | a dangling commit or blob left by an amend or a reset, named | H1 |
| `witness-cannot-see` | a `test` row whose claim cannot be seen by the surface it cites | H9 |
| `unshipped-artifact` | a dirty tracked file under a path a workflow publishes | H10 |

Each is `absent` today and is named here rather than omitted, which is the difference between a gate
that has not been written and one that has silently passed.

### 13.5 The digest framing is the family's, not this plan's to invent

`planet-core/src/hash.rs` frames a digest as `domain || schema (LE u16) || len (LE u64) || bytes`, and
`planet-package/src/frame.rs` keeps the schema inside the payload preimage. For `V4PKG001` (§2.1) that
means: the length is in the preimage, so a truncated payload cannot collide with a longer one; an older
frame is refused **by name** (`frame-schema` with `found` and `supported`), never converted; and `kind`
is the closed set `decl`/`code`, where a `code` frame packs and is refused `frame-kind-reserved` at
mount.

### 13.6 Order inside the blocks, where the family learned it the hard way

The token baseline is frozen **before** the first render (the family's H7 before H9), because a token
written after a render records whatever that render happened to be. The end-to-end window is witnessed
**before** the window that reads code (the family's S4 before S5), because the second needs a working
stream underneath it.

### 13.7 What stays refused for the whole port

A browser or Node result reported as a client witness; a second display, window format, or palette; a
second package magic anywhere; MongoDB or a game-server-only host; a star or grade printed before the
cell that carries it exists; and a step with no exit test.
