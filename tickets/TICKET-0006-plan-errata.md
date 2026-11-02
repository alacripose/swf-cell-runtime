# TICKET-0006 — H2: errata against the plan's §3.1 and §12

- Owner: implementation agent (errata), format owner (the open decisions the errata name)
- Block: H2
- Branch: `ticket/TICKET-0006-plan-errata`
- Ledger: LEDGER-0008
- Depends on: TICKET-0004, TICKET-0005

## Scope (source-safe)

Bring the governing plan into agreement with the measured source where it contradicts it, without
rewriting its intent: each correction is inserted as an attributed erratum immediately beside the
superseded wording, and the original text stays visible. No source movie is read, named, or
described. Nothing was built, mounted, or run.

## Errata recorded

| Id | Superseded claim | Measured | Verdict |
|---|---|---|---|
| E-1 | `shploingus.cells/1` and `shploingus.cellframe/1` are "within the package system" | Owned by `voxel-package` in a different frame family (`SHPKG002/003`, `recipe/3`); `planet-package` has no such kind (`rows.rs` `Kind::ALL`) | contradiction, corrected; cell raster is an **open decision** |
| E-2 | `surface.mount/2` is the transport/receipt boundary | No mount-v2 code in the authoritative tree; adoption is `Mount::watch` + door admission (`planet-shell/src/mount.rs`, `main.rs`) | contradiction, corrected; transport beyond the watched file **blocked** |
| E-3 | the code-kind reader status is unknown | The code **frame reader** exists (`FrameKind::Code`, `decode`, `--code`); the **executor** does not, and the shell refuses code frames on adoption (`mount.rs` `FrameKind::Code`) | contradiction corrected on the reader; executor **open** (H5) |
| E-4 | an "existing asset-reference mechanism" | None in the port's family; media ride in one-field `Kind::Bands`/`Kind::Materials` rows (`rows.rs`, `bin/package.rs`); `SHPKG003` has a table, in the other family | contradiction, corrected; media kind **open** |

## Acceptance test

- `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md`: each erratum quotes the wording it supersedes,
  states the measured behaviour, cites the file and symbol that settles it, and labels its status.
- No claim in §3.1 or §12 contradicts the measured source after the errata; the original sentences
  are still present.
- Open decisions are marked undecided (cell raster, executor, media kind, transport), not answered.
- `python tools/port_gate.py check` passes; `python tools/port_gate.py selftest` passes as before.

## Verification run

- `python tools/port_gate.py check` — PASS (9 checks; 7 not enforceable).
- `python tools/port_gate.py selftest` — PASS (8 of 8 scenarios).
- `git ls-files | grep -Ei '\.(swf|sfw)$'` — none.
- Anchors were located by search before editing (`plan:254`, `plan:819`), so no sentence was replaced
  in place: every erratum is an insertion beside the original.

## Incomplete state (declared, not hidden)

- E-1's cell raster, E-3's executor, E-4's media kind, and any transport beyond the watched file are
  **undecided** and recorded as blocked. The errata do not settle them.
- H4 shell ingestion remains blocked; no agent may run it.
- The errata are agent-authored records. Only the shell assigns gate states.

## Excluded from this ticket

Editing the source trees to match the plan; adopting a second package family; writing a parser; any
claim about what a mount or executor does at runtime (none was executed).

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0008
