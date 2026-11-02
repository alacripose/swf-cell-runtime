# TICKET-0004 — H2: name package-format ownership from the authoritative crate

- Owner: format owner (decision), implementation agent (evidence and record)
- Block: H2
- Branch: `ticket/TICKET-0004-package-ownership`
- Ledger: LEDGER-0005
- Depends on: TICKET-0002 (supersedes its `blocked` state)

## Scope (source-safe)

Read the supplied repository checkouts and record, from their source, who owns each package/cell
layer: frame, rows, declaration grammar, refusals, builder, acceptor, admission witness, and hash.
No source movie is read, named, or described; this ticket touches only implementation source in
the supplied repositories, read-only.

Repositories read: `planet-shploingus` (authoritative), `secure-shell` (format audit),
`openlock` (the legacy cell/cellframe claimant). Nothing was built, mounted, or run.

## Acceptance test

H2's exit test from `docs/PORT-BASELINE.md`:

- The authoritative crate is named, with its exact frame and row readers/writers
  (`planet-package/src/frame.rs`, `rows.rs`), the declaration grammar owner (`shell.rs`), the
  refusal vocabulary (`error.rs`), the hash owner (`planet-core/src/hash.rs`), the mount owner
  (`planet-shell/src/mount.rs`), and the builder (`src/bin/package.rs`).
- Builder and acceptor share one grammar: `ShellDeclaration::parse` is called by both, and
  `declaration.row("shell")` is the only place the field shape is decided.
- Rejected alternatives are recorded: vendoring a local reader; adopting openlock's
  `shploingus.cells/1` / `shploingus.cellframe/1`; treating `surface.mount/2` as existing.
- `python tools/port_gate.py check` passes, including `scope_accounting`.

## Exit test evidence

`docs/PACKAGE-OWNERSHIP.md` — ownership table, byte layout, two digests, mount semantics, code-kind
status, and five corrections to the plan's premises, each citing a file and symbol in the source.

## Corrections this ticket records

1. `shploingus.cells/1` is not the package declaration format; it exists only in openlock's C#,
   Python, and shipped JS. The `.decl` grammar in `planet-package/src/shell.rs` is the owner.
2. `shploingus.cellframe/1` has no reader or writer in any of the three trees.
3. `surface.mount/2` does not exist in the authoritative tree; adoption is `Mount::watch` plus
   door admission.
4. Frame identity is two digests (payload in the header, whole frame in the name).
5. Media rides in row fields; a new media type is a new `Kind`, not a new archive.

## Incomplete state (declared, not hidden)

- `shploingus.cells/1` and `shploingus.cellframe/1`: blocked, no owner anywhere. A format-owner
  decision is required if they are wanted; they must not be reconstructed here.
- Code-kind **executor**: blocked (H5). The frame reader for code exists; nothing runs its contents,
  and the shell refuses code frames on the mount path.
- Streaming transport beyond the watched file: blocked; would be a new layer with its own decision.
- New media kind: blocked on the format owner.

## Excluded from this ticket

Writing any parser, magic constant, digest function, row/declaration grammar, mount code, or test
vector in this repository. Reading the supplied source movie in any form.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0005
