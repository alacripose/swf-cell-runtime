# TICKET-0067 — The placement matrix is read in the format's field order

Goal, quoted from `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §1:

> "The base game and its mods must become peers in one cell-native package system."

D-7b lets the importer lower placement records into cells. It cannot lower what it misreads, and this
reader misread every matrix that carried rotation.

## The defect

The matrix record writes each optional pair with its own presence bit and its own width, in this
order: scale present, scale width, scale pair; rotation present, rotation width, rotation pair;
translation width, translation pair.

`placement::matrix` read both presence bits **first**, then one width shared by scale and rotation.
Two consequences:

1. A record with a scale and no rotation had its rotation flag read out of the scale's width field,
   so the scale pair was decoded at the wrong width and came back truncated. `TICKET-0066` found the
   first symptom of this (a fixture asking for 2.0 in 16 bits), and correcting the fixture alone
   would have hidden the cause.
2. A record whose scale and rotation widths differ — which the format permits and writers emit —
   could not be read at all.

The synthetic fixtures agreed with the reader, which is why twelve green tests did not catch it: both
sides of the test were the same mistake.

## What changed

- `placement::matrix` reads scale and rotation as independent optional pairs, each with its own
  width immediately before its own values, and translation after them.
- `tests/placement.rs::scaled` writes the format's order, with the declared width stated once.
- Two tests are added that the old reader cannot pass:
  `a_matrix_may_carry_a_scale_with_no_rotation` (scale 2.0, rotation absent, translation asserted)
  and `a_matrix_may_carry_a_rotation_with_no_scale` (rotation 0.25 turn, scale absent, scale read as
  identity).

## Exit test

    cargo test -p swf-import --test placement     # 9 passed, 0 failed
    cargo test -p planet-package -p swf-import -p planet-client   # exits 0

## Not done here, and named

- Only `PlaceObject`/`PlaceObject2` matrices are proved. A colour transform, ratio, name or clip
  depth following the matrix is still not read, and the records that carry them are refused rather
  than skipped (`placement-unsupported`), which stays correct until those fields are lowered.
- Nothing consumes the placement list yet: `place_object_2` fills a display list, and no cell is
  emitted from it. That is the next artifact, not this one.
