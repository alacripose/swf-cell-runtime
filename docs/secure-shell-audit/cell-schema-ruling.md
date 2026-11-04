# CELL-SCHEMA ruling — recorded 2026-11-02

**Status:** ruling record, not a repo document. It records the owner's answers to
`testport1/docs/CELL-SCHEMA.md` Q-1…Q-6 and what each answer settles. It does not amend the
proposal in place; the proposal stays as the tabled questions, and this file is the disposition.

**Answered by:** the project owner, in session, 2026-11-02. Q-1…Q-5 belong to the format owner and
Q-6 to the project owner; the owner is the same person here, so all six are ruled.

---

## The six answers

| # | Question | Ruling |
|---|---|---|
| Q-1 | One cell row kind or several? | **Several.** Cells, bindings and placements are separate kinds. |
| Q-2 | Field layout and encoding of a cell set | **Canonical declaration text in one field**, line-oriented, tab-separated, re-encoded canonically so spacing cannot move the digest. |
| Q-3 | Shaper/projection vocabulary | **Open, and streamed** — not a fixed enumeration. |
| Q-4 | Ceilings | **Not the 512 KiB / 16 384 pair.** The surface is stateful and real-time, the addressing is **64-bit**, and the geometry reaches **subvoxel** scale. The ceiling must admit that class of surface. Exact numbers still to be given. |
| Q-5 | Binding syntax | **A stable declared identifier and nothing else.** No paths, no expressions, no arithmetic. |
| Q-6 | Does the first mounted surface use cells? | **Yes — cells in the first surface.** |

## What each ruling settles, and what it costs

### Q-1 — several kinds

`KINDS_ARE_TOTAL` grows from four kinds to seven: `row`, `bands`, `materials`, `shell`, plus a cell
kind, a binding kind and a placement kind. Each carries one field (Q-2), so the field-count rule
stays uniform and the existing refusal paths (`UnknownRowKind`, field-count mismatch) cover them.

What it buys: a cell, its binding and its placement can be validated separately, and a placement
that names no declared cell is refused at the placement rather than hidden inside one text blob.
What it costs: three table rows, three format-document lines, and a builder that emits three kinds
instead of one. That is a format-owner act, not an import detail — P2 of the mission carries it.

### Q-2 — canonical text

Unchanged from the proposal: one field per kind, canonical text, re-encoded on write. Comments and
spacing cannot change the digest, and the builder and acceptor still share one parser.

### Q-3 — open and streamed

This is the ruling that changes the proposal's shape. The proposal wanted a closed enumeration with
unknown members refused by name, on the grounds that an open set cannot be validated at build time.
The ruling keeps the refusal but moves the set:

- The vocabulary is **declared by the package, streamed with it**: each package carries its shaper and
  projection names with their own hashes, in a streamed table read before the cells that use them.
- A name the package uses but does not declare is refused `shaper-undeclared` at the reader, by name.
- A name declared but whose definition hash does not match its body is refused
  `shaper-hash-mismatch`.
- Nothing is resolved at render time outside the package's own table, so "open" does not mean
  "ambient": the vocabulary is open across packages and closed within each one.

This is still a format act (the table is a new declared structure, themselves hashed), and it is
compatible with Q-1: the table streams ahead of the cell/binding/placement kinds.

**Open sub-question:** whether the table itself is a fourth new kind or a section of the declaration
text. It must be answered before P6 can parse.

### Q-4 — 64-bit, stateful, real-time, subvoxel

The proposal's 512 KiB / 16 384 pair is **overruled**. It was measured from the only working cell
format in the trees, and that family's 8 × 16 paper/ink grid is not this target (D-1, and the
proposal's own "what must not happen"). The consequences:

- Cell indices and subvoxel coordinates are **64-bit**; a 16-bit or 32-bit count is a false ceiling
  for a subvoxel surface.
- The declaration is **streamed**, because a stateful, real-time surface cannot be required to fit
  one in-memory blob — which is exactly the falsifier the proposal named for Q-2.
- A ceiling still exists and is still refused by name (`cell-ceiling`), but its value must be
  justified by a measured surface, not copied from another family.

**Open sub-question:** the numbers. Until they exist, the importer must not emit a surface it cannot
bound, and the reader must not accept an unbounded declaration.

### Q-5 — identifiers only

Bindings name a declared identifier and nothing else. No paths, no expressions, no arithmetic;
unknown identifier refused by name. Computation stays in the code kind under D-3's budgets, which
Q13 of the mission now makes real.

### Q-6 — cells first

The first mounted surface uses the cell path. H9 therefore waits on this schema, a parser and a
renderer — accepted deliberately, and coherent with the earlier ruling that the taskbar and dashboard
XML lowers into declared cells rather than travelling as a payload.

## What this ruling does not do

- It does not set the Q-4 numbers, or say whether the streamed vocabulary is a kind or a section.
- It does not implement anything. The mission's P2 (hashed identity), P6 (XML → cells) and P9 (code
  kind) carry the work, and each still needs its ticket, branch, exit test and ledger row.
- It does not make `testport1` the runtime owner: the client remains the Qt 6 shell in v2.

## Follow-ups ruled the same day

### Q-3 — carried by a streamed table (confirmed)

The package declares its vocabulary in a **streamed table ahead of the cells**, each name with its own
hash. A name used but not declared is refused `shaper-undeclared`; a declared name whose hash does not
match its body is refused `shaper-hash-mismatch`. Open across packages, closed within each one. The
table's carriage — its own row kind or a section of the declaration — is still the open sub-question,
and it must be answered before P6 parses a window.

### Q-4 — streamed declaration, 64-bit index, per-package byte ceiling

The ruling, in the owner's words, is not just subvoxel: the surface also carries **octree and LOD data
horizons and a tick cadence**. So the ceiling cannot be a global cell count:

- **64-bit** cell index and subvoxel coordinate.
- **Streamed** declaration: no in-memory whole-surface requirement.
- A **per-package byte ceiling**, declared in the package and refused by name when exceeded
  (`cell-ceiling`), replacing the proposal's 512 KiB / 16 384 pair outright.
- Octree nodes, LOD horizons and tick cadence are named in the schema or they are the next erratum:
  a surface that streams LOD data but declares no horizon cannot be validated at mount.

What is still open: the per-package ceiling's default value. It must be justified by a measured
surface, not copied from another family — which is exactly why the proposal refused to invent it.

### Store (asked with the schema, ruled now)

**MongoDB, systemd-managed on the Debian host, credentials from `systemd-creds`.** The owner's rule
that no branch is authoritative over the secure shell's requirements resolves the conflict with
`secure-shell/docs/ARCHITECTURE.md`'s "No MongoDB ... derail" line: that line is a branch position, not
a v2 requirement. `systemd-creds` replaces the plaintext `/etc/secure-shell/store.env` as the master's
location; the XChaCha20-Poly1305 decision and its three derived domains stand unchanged.

## Second round, same day — ceiling and vocabulary carriage

- **Vocabulary carriage settled:** a labelled section at the top of the declaration, read before any
  cell that uses it. The "fourth row kind or a section" question is answered: a section.
- **Ceiling settled:** 1 GiB working size, **4 GiB hard ceiling** per package, 64-bit offsets,
  refused by name. The 512 KiB / 16 384 pair stays overruled.
- **Key topology changed:** one key per package, no master. This supersedes the "one master, three
  derived domain keys" line in the store ruling. The open consequence is ledger and store rows that
  span packages; they need either a store-level key or a per-package key reference on the registry
  row, and that is not decided here.
