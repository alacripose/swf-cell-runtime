# TICKET-0073 — Sound payloads travel as package artifacts, and the client plays decoded samples

Goal, quoted from `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §7:

> "Bitmaps, textures, fonts, audio, and any supported video material are extracted and converted by
> the shell into canonical package assets. Their bytes are content-addressed and referenced through
> the package system's existing asset-reference mechanism."

## What was wrong, and what the other trees already answer

TICKET-0072 carried the sound payloads as hex spliced into a streaming section of the frame
document. That was a shape invented inside this port. The trees that own the format already carry
payloads a different way, and this unit adopts theirs:

- `shploingus-clean/crates/voxel-package/src/recipe.rs` — a package carries its own files as an
  **artifact table**: a count, then `path` and `bytes` for each, `ARTIFACT_PATH_MAX = 256`,
  `ARTIFACT_BYTES_MAX = 4 MiB`, `ARTIFACT_COUNT_MAX = 1024`, and a path that must be package-relative
  (no leading `/`, no `\`, no `.`/`..`, no empty part), validated on compile **and** on load.
- `shploingus-clean/crates/voxel-package/src/cellframe.rs` — the frame grammar never splices media
  into a drawing row: a frame is a **sibling artifact the package names by id**.
- `shploingus-clean/crates/voxel-client/src/shell/mount.rs` — the mounted client holds the package's
  files in `frame_artifacts`, keyed by package-relative path.
- `ala_civ_bots_prototype/.../PackageFrame.java` — one frame for data and code, content-addressed,
  with payload-digest names: a payload is named and hashed, not inlined into a row vocabulary.
- `shploingus-clean/crates/voxel-audio` — samples are `Vec<f32>` with one declared rate ("declared,
  and the same on both ends of the pipe"), and the device wheel is deliberately **absent** so the
  path stays headless-testable, with the owed wheel named in the crate doc rather than stubbed.

## What changed

`planet-package`:

- `[artifact]` section, `path<TAB>digest<TAB>hex`, terminated by a blank line, opening the
  declaration. Paths are checked by `artifact_path_is_well_formed`, a restatement of family B's rule;
  the ceilings are `ARTIFACT_PATH_MAX` / `ARTIFACT_BYTES_MAX` / `ARTIFACT_COUNT_MAX`, family B's
  numbers rather than wider ones; a payload over a ceiling is `cellframe-bound`, never truncated; a
  duplicate path is refused by name; and the row's digest is **checked against its bytes on read**
  (`artifact-digest`), so byte identity is a parse-time fact.
- `[audio]` rows now **name** an artifact: `sound|stream<TAB>id<TAB>artifact-path<TAB>format<TAB>rate<TAB>samples`.
  A row naming a path the table does not carry is refused `cellframe-row` with `artifact-reference`.
  `SoundClip` gains `artifact` and keeps `bytes`, resolved from the table at parse, so a consumer
  still receives the payload itself.
- `docs/CELL-SCHEMA.md` records the table, the ceilings, the path rule and the reference row, and
  names each source above rather than presenting the shape as this port's invention.

`swf-import`: `frame_document` emits `[artifact]` rows (`audio/<id>.<format>`, or `audio/stream.<format>`
for the block that states no id) and `[audio]` reference rows; `frame_bytes` re-parses what it wrote
and still refuses unless every resolved clip's bytes equal the source character's payload.

`planet-client`: unchanged in shape — `Sink` decodes with a real MPEG decoder, `Channel` renders the
samples, and the device wheel stays owed rather than stubbed. Its tests take the new `artifact` field.

## Exit test

    cargo test -q -p planet-package -p swf-import -p planet-client   # exits 0
    cargo run -q -p swf-import --bin swf-import -- \
        inputs/swf1.swf generated/swf1.pkg generated/mount-ledger.tsv swf1
    cargo run -q -p planet-client --bin inspect -- generated/swf1.frame
    python tools/port_gate.py check

Measured: `placements 65 … 0 without measured bounds`, `placeholder 0`, `unmeasured 0`,
`empty sprites 13 / frameless sprites 12 / recordless sprites 1 [204]`, `sound_total 9 20612`; the
frame opens with `[artifact]` and 8 `sound` rows plus 1 `stream` row; `inspect` decodes 8 of 9
(114 048 frames / 114 048 samples at 11 025 Hz mono, peaks 0.66–1.00) and reports the 4-byte stream
block as `sound_unreadable … audio-unreadable` with the decoder's reason, then `sound_played` renders
114 048 frames with peak 1.0.

## Not claimed

- No device: nothing opens a sound card, and the wheel is named owed, as `voxel-audio` names it.
- The 4-byte stream block is genuinely undecodable on its own; it is reported, not replaced.
- `client_witness` / `client_run_h9` remain `NOT-ENFORCED`.
