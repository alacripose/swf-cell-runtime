# PACKAGE-OWNERSHIP — H2 status: BLOCKED

**Implementation status:** `blocked`. No parser, schema, row grammar, cell declaration
grammar, cellframe reader, digest layout, or bytecode representation has been written, and
none may be written from this document.

**Owner of the decision:** format owner of the authoritative package crate, with the shell.
**Blocking dependency:** the authoritative `planet-package` / cell-package Rust source and
its reader/writer must be present in the target environment. It is not present in this
checkout.

## Evidence for the block

The plan (§3.1, §12) requires the implementation agent to locate the authoritative crate
and read its actual reader/writer before writing a parser. What exists in this checkout:

| Path | Content |
|---|---|
| `docs/` | Plan only. |
| `inputs/` | Shell-only source input. Not readable by agents; not inspected. |
| `patches/`, `tickets/`, `ledger/`, `tools/`, `config/` | Governance scaffold created by H1/H3. |
| Rust workspace (`Cargo.toml`, `src/`, `crates/`) | Absent. |
| `shploingus.*` / `V4PKG001` implementation code | Absent. |

Consequence per §12: ownership cannot be named from evidence, so the package layer stays
`blocked`. It is **not** recreated from memory, from the plan's prose, or from the Java
reference reader. The plan's description of `V4PKG001` is design evidence, not a
specification this repository may implement against.

## What is known, and what it is not authority for

Design evidence carried in the plan (to be confirmed against the crate, never implemented
from this document):

- `V4PKG001`: eight-byte magic, frame schema, frame kind, payload length, domain-separated
  digest, payload; strict decode of row schema, row kinds, field counts, trailing bytes.
- `shploingus.cells/1` declares a package's cells/frames and their relationships.
- `shploingus.cellframe/1` carries typed cell data, field bindings, shapers, projections,
  ordering, dependencies.
- The package `Code` kind exists but is reserved until a reader exists.
- `surface.mount/2` is the stream/receipt transport, not a second content schema.

None of the above fixes a byte layout, a field ordering, a digest construction, or a
grammar. All of it requires the crate.

## Reserved alternatives, and why

| Alternative | Decision |
|---|---|
| Copy the Java `PackageFrame.java` reader into Rust | Rejected. It is a reference implementation, not a second authority; a competing Rust reader is drift. |
| Copy `dl-openshell`'s `.shpkg` / `openlock.addons/2` window payload | Rejected. Reuse its staging, versioning, discovery, bounds, atomic swap and refusal *mechanics*, not its content format. |
| Invent a provisional Rust frame reader "to unblock" | Rejected. Producing a second magic/grammar is the exact drift H2 exists to prevent. |
| Treat this document as the schema | Rejected. It records a block, not a format. |

## Work that may proceed while H2 is blocked

- Governance, gate, ledger and ticket mechanics (H3), which do not touch package bytes.
- Synthetic fixtures and generic contracts that name no format detail.
- Receipt/ledger/audit data structures that carry opaque ids and digests only.

## Work that must not proceed

- Any frame/row/cell/code parser or writer.
- Any `V4PKG001` constant, magic, or digest function.
- Any mount or cell-graph implementation that assumes a row grammar.
- Any claim about cell mounting, rendering, or gameplay parity.

## Exit condition

Supplying the authoritative crate and recording: the exact frame and row readers/writers,
the cell declaration and cellframe owner, the mount-v2 owner, the selected source paths,
the rejected alternatives, and evidence that builder and acceptor share one grammar. Until
then this file stays `blocked` and the package layer stays unimplemented.
