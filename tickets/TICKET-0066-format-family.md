# TICKET-0066 — The declaration names `planet.surface/<32 hex>`, and the upstream family is read, not emitted

Goal, quoted from `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §1:

> "The base game and its mods must become peers in one cell-native package system."

A peer package has to say which grammar it is written in. Since TICKET-0040 it says so with a hash
instead of a counter — but the hash was computed under the **upstream** family name
(`dl-enclave.surface`), which is one of the counter-bearing ids the audit registered as legacy. So a
document this layer emitted presented a legacy family as current: the exact confusion the hashed id
was introduced to end.

## What changed

- `format_id::FAMILY` is `planet.surface`. The plan's ownership table gives the row grammar and the
  `.decl` it frames to this package layer, so the family on the document is this layer's.
- `format_id::LEGACY_FAMILIES` names the upstream family. It is part of the preimage, because which
  legacy heads a reader accepts is a fact about the format, not a reader detail: accepting another
  one changes the id.
- `is_legacy` answers for both families. A numeric id in either one reads as an upgrade
  (`read` → `UpgradedLegacy`) and is refused on emission (`format-id-legacy-emitted`).
- A 32-hex id under **any** family that this reader did not compute is refused
  `format-hash-mismatch`, instead of falling through to `surface-schema`. Before this, only this
  family's hashes reached that refusal; a hash in the upstream family was reported as "not a
  document", which named the wrong problem.
- `Refusal::SurfaceSchema`'s message names the computed id rather than the legacy literal.
- `crates/swf-import/src/emit.rs` no longer imports the unused `SURFACE_SCHEMA`.

## The red suite this unit found on trunk

`cargo test -p swf-import --test placement` failed on `44a0578` with a clean tree, before any edit
here: `a_place_object_2_reads_scale_and_rotation` asked for a scale of 2.0 in a **declared 16-bit**
field. 2.0 is 131,072 in 16.16; a signed 16-bit field tops out at 32,767, so the value truncated to
zero and the reader was blamed for returning what it was given. The fixture now declares 19 bits,
which is what its own comment claims it is testing. Corrected here because a unit cannot land on a
red suite, and recorded rather than quietly fixed.

## Exit test

    cargo test -p planet-package -p swf-import -p planet-client   # exits 0
    python tools/port_gate.py selftest                            # record regenerated
    python tools/port_gate.py check                               # PASS

New tests in `format_id`: the id is `planet.surface/` + 32 hex; the legacy family is in the
preimage and still reads as an upgrade; a numeric id in the new family is legacy too and is refused
on emission; a hash in the upstream family is `format-hash-mismatch`, not `surface-schema`.

## Not done here, and named

- The `V4PKG001` header still carries `schema: u16`. Moving the hashed id into the header replaces
  the numeric schema field and the digest preimage, which AGENTS.md §12 fixes in words; that unit
  needs its own errata and reader, and it is the next one.
- The placement matrix reads `HasScale`, `HasRotate`, then one shared bit count. The format writes
  `HasRotate` **after** the scale pair, each with its own count. The synthetic fixtures agree with
  the reader, so they cannot catch it; a real matrix with rotation would be misread. TICKET-0067.
- `docs/CAMPAIGN-FRONTIER.md` still describes the format as `dl-enclave.surface/1`; the document is
  a working note, and its correction is a doc unit, not this one.
