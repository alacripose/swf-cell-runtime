# TICKET-0074 — A cell's visual payload is carried, ordered and stated, never invented

Goal, quoted from `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §7:

> "Bitmaps, textures, fonts, audio, and any supported video material are extracted and converted by
> the shell into canonical package assets. Their bytes are content-addressed and referenced through
> the package system's existing asset-reference mechanism."

## What was wrong

The client's dump showed 13 sprites with no art (12 frameless: 371…382, one recordless: 204), and the
pipeline had no way to say so: a sprite with no first-frame placement was drawn as its measured
bounding box, which is a grid standing where the file's picture should be. The picture could also
collapse to one colour, because cells were emitted in display-list order rather than by depth.

## What the owning trees already answer

- `shploingus-clean/crates/voxel-package/src/cellframe.rs` — a frame with no `fill` and no `run` is
  refused by name (`RefusalKind::MalformedRecord`): "an empty frame is a window's content erased, and
  an absence has to be stated as one". Also: "**A frame names a role, never a colour.**"
- `shploingus-clean/crates/voxel-client/src/shell/cells.rs` — "**A role is not a colour here.**"
- `shploingus-clean/crates/voxel-package/src/recipe.rs` — `theme_role` rows: the package says what a
  role is; the client does not supply a palette of its own.
- `shploingus-clean/crates/voxel-client/src/shell/mount.rs` — a mounted package holds its frames in
  `frame_artifacts`, keyed by package-relative path.
- `openlock/shell/alpha-shell/frames/MAP-0001.frame` — real frames fill every content line they
  address, one `fill` row per row.

## What changed

- `[absence]` section in the frame grammar: `sprite<TAB>id<TAB>reason<TAB>rule`, validated (closed
  kind set, one-token reason, law-token rule, no duplicate), with the owning rule cited by name as
  `cellframe::malformed-record`. An empty section is refused, because a heading that states nothing
  was not written.
- `emit::sprite_absences` states `no-display-list`, `no-drawn-record` or `undecoded-display-record`
  for every measured sprite without art, and `frame_bytes` re-reads the frame to prove every row
  survived.
- The importer no longer draws a sprite's bounding box: a sprite with no first-frame placement draws
  nothing and is stated absent.
- Cells carry a painter's path (`Cell::order` = outer depth, then each nested depth) and the frame
  carries them back to front; a sprite's own background can no longer paint over its art.
- A placement with a clip depth is a **mask**, and a placement whose alpha multiplier is zero is
  **invisible**: neither is painted, both are counted.
- The movie is read frame by frame: `ShowFrame` closes a frame, `RemoveObject`/`RemoveObject2` clear a
  depth, and the report names the frame the package ships (`timeline_frame`, `timeline_states`,
  `frame_paint`).
- `inspect --render <pkg> <frame> <out.ppm>` runs the client's own path (adopt → mount → draw →
  paint) and writes the pixels, so the claim is checkable.

## Evidence

```
placements 1 placements; 2400 lowered into cells; 0 without measured bounds;
0 unreadable records; 0 stated absent; 0 masks not drawn; 0 invisible not drawn
timeline_frame 1 of 12 at 40 fps
timeline_states [1, 3, 3, 3, 3, 3, 2, 2, 2, 2, 64, 64]
sprite_absent_total 13
sprite_absent sprite 371 no-display-list rule cellframe::malformed-record   (… 371…382, 204)
raster STAGE-0001 window stage 80x30 cells painted 2400 papers 1 picture 640x480 rgba8
```

`cargo test -q -p planet-package -p swf-import -p planet-client` exits 0.
`python tools/port_gate.py check` prints `RESULT: PASS (21 checks, 5 not enforceable)`.

## What is still open

The picture is one colour because frame 1 of this movie is a single full-stage record; advancing needs
the tick cadence in `TICKET-0075`. Sprite animation beyond a sprite's first frame, clipping, partial
alpha and blend modes are named gaps, not filled in.
