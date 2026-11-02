# TICKET-0005 — H2: two package families and three claimed implementations

- Owner: format owner (decision), implementation agent (evidence and record)
- Block: H2
- Branch: `ticket/TICKET-0005-package-landscape`
- Ledger: LEDGER-0007
- Depends on: TICKET-0004 (corrects its claim about cell grammar ownership)

## Scope (source-safe)

Read `shploingus-clean` and `ala_civ_bots_prototype` read-only and record what each owns, correcting
TICKET-0004's claim that the cell grammars had no reader anywhere. No source movie is read, named, or
described. Nothing was built, mounted, or run.

## What the trees show

1. **Two package families.** Family A: `planet-shploingus` — `V4PKG001`, typed row kinds, `.decl`
   shell declaration, `Mount::watch` adoption plus door admission. Family B: `shploingus-clean` —
   `SHPKG002`/`SHPKG003`, `recipe/3`, and the only real `shploingus.cells/1` and
   `shploingus.cellframe/1` grammars in any tree.
2. **A third implementation claiming family A.** `ala_civ_bots_prototype`'s
   `PackageFrame.java` shares the frame bytes exactly (magic, schema, digest preimage with domain and
   length, `u16` payload schema, `u32` count, `u64`-prefixed fields) but carries a **disjoint row
   vocabulary** (`observation`/`decision`, 13 fields each) and writes the **payload digest** into the
   file name where Rust writes the **whole-frame digest** and verifies it.
3. **`cell/4` is not a format tag.** The only `.rs` occurrence is an arithmetic comment in
   `voxel-client/src/pattern.rs:890`.

## Decision recorded

The port's frame is family A (`V4PKG001`, `planet-package`). Family B and the Java port are evidence,
not formats to adopt: one package family is a hard gate (`PATCH-0005`). Because the Java port proves
`V4PKG001` identifies a **frame layout** and not a readable package, the correct behaviour for a
foreign package is refusal (`UnknownRowKind`, `NameMismatch`) — which already exists and needs no
invention. Cell drawing in the port is an open format-owner decision: a new family-A `Kind` carrying
a cell raster with family B's semantics, or no cell raster at all.

## Acceptance test

- `docs/PACKAGE-OWNERSHIP.md` names both families and all three implementations, with the
  compatibility table measured from source rather than from either tree's comments.
- The earlier over-claim is corrected in place and named as drift in the ledger, not edited away.
- `python tools/port_gate.py check` passes, including `scope_accounting`.

## Evidence

| Claim | Source |
|---|---|
| Family A frame, rows, declaration, mount | `planet-package/src/{frame,rows,shell,error}.rs`, `planet-shell/src/mount.rs` |
| Field framing is `u64` on both sides | `planet-core/src/canon.rs` `put_bytes`; `PackageFrame.java` `putBytes` |
| Family A name digest is whole-frame and verified | `frame.rs` `file_name`, `Package::read` (`NameMismatch`) |
| Java name digest is payload-only and unverified | `PackageFrame.java` `digestHex`, `name`, `decode` |
| Java row vocabulary is disjoint | `PackageFrame.java` `RowKind`, `FIELDS` |
| Family B frame and cell grammars | `voxel-package/src/{recipe,cells,cellframe}.rs` |
| Format subject measured through cellframe | `axis/verify/src/subject.rs` `FORMAT` |

## Incomplete state (declared, not hidden)

- Family-A cell raster representation: blocked on the format owner.
- Code-kind executor: blocked (H5); the frame reader exists, nothing executes its contents.
- `shploingus.cells/0`: refused by family B itself; stays refused here.
- Streaming transport beyond the watched file: blocked; would be a new, decided layer.
- Java-port interoperability: not implemented and not desired; the port refuses foreign packages
  rather than reading them.

## Excluded from this ticket

Writing any parser or grammar; adopting family B's magics; adopting the Java reader; reading the
source movie.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0007
