# OWNERSHIP AUDIT — 2026-11-04 — the family branches, and what `testport1` is for

**Status:** audit record. Read-only across the sibling checkouts; it changes no tree but this one.
**Question it answers:** are the things this port (`testport1`, `planet-*`) creates a dying legacy
implementation that another owner will overwrite, and which branches hold the authorities it cites?

## 1. Branch inventory (read 2026-11-04, `git branch -a` per checkout)

| Checkout | Branch checked out | Branches | Notes |
|---|---|---|---|
| `shploingus-clean` | `main` `6049414` (2026-10-26) | `main`, `backup-2026-10-14-pre-flicker-rewrite` (`aef0971`) | one live branch plus one pre-rewrite backup; nothing on a side branch |
| `openlock` | `main` `cb73b7b` (2026-10-30) | `main` (+ `origin`) | clean |
| `secure-shell` | `main` `124b647` (2026-10-31) | `main` (+ `origin`) | clean |
| `dl-enclave` | `main` `262ba89` (2026-10-31) | `main` | clean |
| `shploingus-game` | `shploingus` `d871fd7` (2026-09-28) | single branch | archive lineage |
| `shploingus` | `main` `d6d2160` (2026-09-26) | single branch | MD1 doctrine tree, pre-split |
| `ala_package_designer` | `master` `2b2e48f` (2026-10-18) | single branch | foundry shell, source of the 8 × 16 cell raster |
| `ala_civ_bots_prototype` | `ingest/shploingus-corpus-and-aspiration` `a31cce2` | 8 | **the only checkout on a non-main branch**, one commit ahead of `main` (`2d04c57`); the Java `PackageFrame` lineage |
| `testport1` (this port) | `master` `9ecdbf1` | **55** | every unit is a ticket branch; the port's own history |

`secure-shell-audit` is not a checkout: it is a directory of ruling and audit documents
(`cell-schema-ruling.md`, `testport1_plan_resynthesis.md`, `secure_shell_synthesis.md`, and the two
civilian-expectations records). It is read as the ruling record it says it is.

**No family branch holds a second, newer renderer that this port has failed to notice.** The one
place the format's own renderer lives is `shploingus-clean/main`, and the one place a newer *product*
client is decided is `secure-shell` v2. Those are different answers and the next section is why.

## 2. The ruling that settles the question: this port is the ingest module, not the runtime

From `secure-shell-audit/testport1_plan_resynthesis.md`, §1 (the owner's twenty answers of
2026-11-02):

> `testport1` is the **ingest module of secure-shell v2**, not the runtime owner (Q16, Q17).

and, in the same document's ownership table:

> | Client surface (Qt 6: Core, Gui, Quick/QML, Network, Xml) | v2 client | gives up `winit`/`wgpu` as a
> product surface |
>
> `planet-client` (`package_owner.rs`, `cell_mount.rs`) remains as evidence that the frame is
> adoptable; … `moved to v2` — with the owner named.

`secure-shell-audit/cell-schema-ruling.md` says the same in one sentence: **"It does not make
`testport1` the runtime owner: the client remains the Qt 6 shell in v2."**

So the answer to the question asked is **yes, and by ruling rather than by accident**:

- `planet-client` is **evidence**, not a product surface. Its `winit`/`wgpu` path — `render.rs`,
  `present.rs`, `batch.rs`, `picture.rs`, `draw.rs`, `font.rs` — is a **dying legacy implementation**:
  the port keeps it because it proves the frame is adoptable and drawable, and v2 replaces it with a
  Qt 6 client. That is why building a *second* renderer inside `planet-client` (a family-shaped
  `cells.rs`, linear compositing, its own glyph pipeline) is the wrong investment: it would be a
  third renderer in a tree that has already given up the surface.
- What survives from this port into v2 is the **ingest half**: `planet-package`'s frame, row and
  cellframe readers, the importer, the `[artifact]`/`[audio]`/`[absence]` carriage, and the
  `cellframe` grammar ownership recorded in `docs/PACKAGE-OWNERSHIP.md`.
- What does **not** survive: the port's client surface, its palette, its bitmap font, its raster.
  Any unit aimed at making `planet-client` *look right* is a legacy unit.

## 3. The ceiling question, answered by the same ruling record

`cell-schema-ruling.md`, Q-4 (ruled by the format owner, 2026-11-02):

> **Not the 512 KiB / 16 384 pair.** The surface is stateful and real-time, the addressing is
> **64-bit**, and the geometry reaches **subvoxel** scale. The ceiling must admit that class of
> surface. Exact numbers still to be given.

So the constant `MAX_FRAME_BYTES = 512 * 1024` and `MAX_CELLS = 16_384` — in
`shploingus-clean/crates/voxel-package/src/cellframe.rs` as well as in this port's
`crates/planet-package/src/cellframe.rs` — are **implemented floors, not the ruled ceiling**. The
earlier note in `docs/CELL-SCHEMA.md` that called 512 KiB "the owning reader's ceiling, not this
port's to widen" stated the reader's current constant and mistook it for the format's ruling; that
paragraph is corrected in this unit. The ruled shape is a **64-bit addressed, per-package,
stateful/tick-cadence ceiling whose exact numbers are still owed by the format owner** — which is
also why a 12-frame timeline cannot be dismissed as "too large for the format": it is too large for
the *current constant*, and the constant is the thing the ruling says must change.

## 4. What this audit changes in this port

1. `docs/CELL-SCHEMA.md`: the ceiling paragraph now quotes Q-4 and calls the 512 KiB / 16 384 pair an
   implemented floor with exact numbers owed, rather than the format's answer.
2. `docs/SCOPE-ACCOUNTING.md`'s framing is unchanged; `docs/PORT-BASELINE.md` H9 stays unclaimed —
   the client that will carry H9 is v2's Qt 6 shell, and no agent witness here is a substitute.
3. No new renderer work in `planet-client`. Renderer-shaped units are recorded as evidence-only, and
   the family-shaped raster (`voxel-client/src/shell/cells.rs` + `voxel-ui`) is cited as the design
   the v2 client inherits, not ported a third time here.
4. The timeline still needs carriage (`TICKET-0075`) because ingest is this port's job: a package
   that ships one frame is incomplete ingest regardless of who renders it.

## 5. Evidence limits

- Branch inventory is the checkouts on this machine; a remote branch not fetched here would not
  appear. Every checkout above is in sync with its remote except `ala_civ_bots_prototype`, which is
  one commit ahead of `main` on a feature branch.
- No tree was built, mounted or executed for this audit; it reads branch names, commits and ruling
  documents only.
- The ruling record is the owner's; this audit restates it and does not extend it.
