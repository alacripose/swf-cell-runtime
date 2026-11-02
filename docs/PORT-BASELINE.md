# PORT-BASELINE — H1–H10 block contract

One artifact per block. A block is not authorized until the previous block's ledger entry
exists and can be verified (`ledger/ledger.jsonl`, checked by `tools/port_gate.py`).

Status vocabulary in this document is implementation vocabulary: `present`,
`not-implemented`, `blocked`. Gate states (`held`, `absent`, `refused`) are shell-assigned
and appear here only as `unassigned` until the shell records them.

Current position: **H1–H3 present; H4+ not started.**

### H1 — project scaffold and patch baselines

- Artifact: `patches/baselines/` + this document.
- Exit test: Git initialized with the baseline commit landed before any implementation; all six baseline patches exist with their required rule and mechanical evidence; `.swf`/`.sfw` exclusion and the project-root output gate pass; issue/PR templates and the ledger are usable.
- Refuses: import or render work before the rules are committed; a source movie in Git; outputs outside the checkout; one giant initial commit.
- Status: present.

### H2 — package-format ownership

- Artifact: `docs/PACKAGE-OWNERSHIP.md`.
- Exit test: names the authoritative package crate, its exact frame and row readers/writers, the cell declaration and cellframe owner, the mount-v2 owner, the selected repository code paths, the rejected alternatives, and evidence that builder and acceptor share grammar.
- Refuses: new magic, a new `secure-shell.*` game format, a second reader, or copying the Java reader as a competing Rust parser.
- Status: present — three implementations that claim this frame were read at the supplied checkouts: `planet-package` (authoritative), `shploingus-clean`'s `voxel-package` (the real owner of `shploingus.cells/1` and `shploingus.cellframe/1`), and `ala_civ_bots_prototype`'s Java `PackageFrame` (same frame bytes, disjoint row vocabulary, payload-digest names). Family A is the port's frame; family B and the Java port are evidence and are not adopted. `docs/PACKAGE-OWNERSHIP.md` records the ownership, the decision, four corrections to the plan, and one correction to this repository's own earlier claim that the cell grammars had no reader.

### H3 — drift gate and repository workflow

- Artifact: `tools/port_gate.py`.
- Exit test: mechanically detects tracked `.swf`/`.sfw`, outputs outside the project root, second package magics in code, unrecorded format tags, missing ticket/PR references in commit history, invalid ledger entries, missing baseline patches, and blocks without exit tests; `selftest` proves each refusal.
- Refuses: a prose-only plan that cannot reject drift; direct-to-main implementation; untracked work accumulating outside the queue.
- Status: present (decidable checks enforced; remainder listed by the gate as `NOT-ENFORCED` with owner and blocking dependency).

### H4 — shell-owned SWF source inventory

- Artifact: shell-private source inventory and source-map store; ticket-scoped redacted secure-cell definitions.
- Exit test: shell-only read-only parse; complete source names, offsets, URLs, script identifiers, media and dependency graph remain shell-private; the agent receives only opaque work ids, canonical artifact digests, and coarse redacted classifications.
- Refuses: agent inspection of raw source; agent-visible inventory files; source identifiers in tickets or logs; lowering before shell inventory.
- Status: blocked — requires the shell to run ingestion; no agent may perform it.

### H5 — shell compiler contract and code-kind reader

- Artifact: `docs/CELL-PROGRAM-DECISION.md` plus shell-private compiler/test artifacts.
- Exit test: format-owner decision and generic code-kind implementation contract; the shell privately builds source-derived AVM1/AVM2 coverage and test vectors; agent work uses synthetic fixtures and generic schemas only.
- Refuses: smuggling code into data rows; a second runtime; exposing source-derived vectors; claiming script parity before the reserved code lane has a reader.
- Status: blocked — the instruction set is ruled (**D-3**: bounded deterministic cell program), so the decision that held this block is answered; what remains is the compiler contract that needs H4's shell-private inventory plus an executor that does not exist. Implementation: `not-implemented`.

### H6 — ticket/secure-cell/test-only MCP pilot

- Artifact: `docs/MCP-SECURE-CELL-CONTRACT.md` and shell-private test records.
- Exit test: the agent submits a ticket and receives only its secure cell; it can request only already-declared shell tests and read redacted semantic results; CUA Axis, controls, captures, oracle, controller and judge stay shell/test/user-side.
- Refuses: agent screenshots, observations, CUA handles, pointer/keyboard/runtime tools, invented probes, source inventory, or game-specific MCP logic.
- Status: not-implemented.

### H7 — visual baseline freeze, shell/test/user only

- Artifact: shell-private client/source visual baseline store.
- Exit test: shell/test/user witness captures at declared resolutions; images, frame data, comparison artifacts and capture identities stay shell-private; the agent receives only a redacted semantic assertion and canonical artifact digest.
- Refuses: agent viewing or taking screenshots; agent access to captures; screenshot-driven agent retuning; source captures in tickets or PRs.
- Status: not-implemented.

### H8 — shell-owned semantic coverage matrix

- Artifact: shell-private full coverage matrix plus redacted aggregate report.
- Exit test: every source feature maps to its canonical cell implementation, shell status, and private witness; zero unclassified source features; the agent receives aggregate/redacted completion assertions only.
- Refuses: agent-visible source locations or feature identifiers; a percentage that hides missing semantics; treating a shell `absent` state as a pass.
- Status: not-implemented.

### H9 — first playable integration witness

- Artifact: one end-to-end base-game integration path inside the full package/runtime contract.
- Exit test: the shell builds and mounts the canonical package; user or shell-declared tests exercise the client; the shell verifies the state transition and emits a redacted result.
- Refuses: agent-driven probes or observation; treating the slice as the final product; browser-only witness; a second surface; agent-side game behavior.
- Status: not-implemented.

### H10 — streamed addon transaction witness

- Artifact: one addon cell package plus shell-owned transaction/test record.
- Exit test: the shell delivers the addon through the same stream/frame format, verifies an extension, tests stale and tampered replacement, and exercises unload/rollback while preserving base-game state; the shell/test Axis driver performs the witness.
- Refuses: a separate `.shpkg`/`openlock.addons/2` route; an alternate renderer; agent-triggered replacement, unload, or migration; state reset; game-specific MCP logic.
- Status: not-implemented.

## 90-minute checkpoint fields

Recorded per block in `ledger/ledger.jsonl`. The shell owns the authoritative checkpoint
and gate state; an agent may not mark a source feature `held`, `absent`, or `refused`.

1. User goal sentence, verbatim.
2. The one artifact changed in this block.
3. Last shell-assigned gate state.
4. Drift signal observed, if any.
5. Explicitly refused work for the next block.
6. Next artifact.
7. Ledger entry id for the completed block.
