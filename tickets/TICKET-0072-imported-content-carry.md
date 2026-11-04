# TICKET-0072 — Imported content reaches the client as content: sound payloads and every placed sprite

Goal, quoted from `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §7 (media extraction):

> "Bitmaps, textures, fonts, audio, and any supported video material are extracted and converted by
> the shell into canonical package assets. Their bytes are content-addressed and referenced through
> the package system's existing asset-reference mechanism."

Two gaps stood between the mounted package and drawn, audible content. Sounds were **measured** but
never carried: the inventory counted `media_sound 9 / 20668 bytes` and no audio byte entered the
format, so no client sink could consume one. And five placements of two sprites had no measured
extent, so they were counted `unmeasured` and drew nothing. Both are closed here, in one unit, because
"the package mounts" is not "the client renders it".

## What changed

### The audio channel (`planet-package`)

The cellframe grammar had no audio channel, so one was added where the other streamed channel is
documented: a `[audio]` section at the top of the declaration, terminated by one blank line, one row
per sound —

    sound|stream<TAB>id<TAB>format<TAB>rate<TAB>samples<TAB>hex

— carrying the character's **payload bytes verbatim**, not a count of them. It is additive (a frame
with no section is silent), stripped before the owning grammar reads the frame, and refuses an
empty, odd-length or non-hex payload and a duplicate `kind`+`id` by name (`cellframe-row`,
`cellframe-duplicate`). `SoundClip`, `payload_digest` (FNV-1a 64) and `decode_hex` live beside it;
`docs/CELL-SCHEMA.md` records the section beside the vocabulary section it mirrors.

### The carry and its identity assertion (`swf-import`)

`content::sound_clip` reads each `DefineSound` character — id, the `SoundFormat` byte, the declared
sample count, and the payload from byte seven of the tag — and each `SoundStreamBlock`, into
`Inventory.sounds`. `emit::frame_document` writes them as the section, and `emit::frame_bytes`
**re-parses the document it just wrote** and refuses `audio-payload` unless every carried payload
equals the source bytes, so a truncation cannot reach a client. The report prints, per sound,
`sound_source <id> <bytes> <digest> …` and `sound_carried <id> <bytes> <digest> match true`, and one
`sound_total`.

Two corrections came out of the work: the `SoundFormat` bit order was wrong (the byte is
compression in the high nibble, rate in bits 2–3), which had labelled MP3 payloads `nellymoser` at
5512 Hz; and the tolerant placement pass now reads `PlaceObject3` (tag 70), which is how sprite 209
places its content.

### The client sink (`planet-client`)

`audio::Sink` consumes a frame's clips whole, refuses `audio-empty` rather than queueing silence, and
reports kind, id, format, rate, byte count and payload digest for each. `src/bin/inspect.rs` is the
headless consumption surface: it parses the frame, draws it, and runs the sink, printing one
`sound_sink` row per payload.

### Sprite placement, from the file's own facts

- **Sprite 209** places a shape with `PlaceObject3`, which the reader skipped; `place_object_3_tolerant`
  reads the second flags byte, the class-name string when the record declares one, then the same
  character and matrix fields, and is wired into the first-frame read, the extent measurement and the
  tolerant survey. No character id is special-cased.
- **Sprite 204's** decoded body is a sound stream block and nothing else — no display record at all —
  so it is reported as decoded with zero cells instead of `unmeasured`. The report now separates
  `frameless sprites` (declared with zero frames: 371–382) from `recordless sprites` (frames but no
  display record: 204), because the two are different facts about the file.
- The proof line is now **per placed character** with the cells it added, and distinguishes
  `decoded`, `decoded-offscreen`, `placeholder` and `unmeasured`. Before this, a placement that could
  not be measured was reported `decoded-offscreen`, which was the wrong claim.

### The frame's cell ceiling

Lowering the newly placed geometry pushed the frame to exactly 16 384 cell rows — the format's
ceiling — so later placements were being dropped in silence. `rasterise` now emits one `fill` per
same-colour stretch of a row, which is what the format's run row is for, and `Lowering.painted`
states the covered cell count (22 014 cells in 2 309 fills). The picture is unchanged; the count the
summary states is now the number of cells covered, not the number of rows.

## Exit test

    cargo test -q -p planet-package -p swf-import -p planet-client        # exits 0
    cargo run -q -p swf-import --bin swf-import -- \
        inputs/swf1.swf generated/swf1.pkg generated/mount-ledger.tsv swf1
    cargo run -q -p planet-client --bin inspect -- generated/swf1.frame
    python tools/port_gate.py check                                       # RESULT: PASS

Measured on the authorized input:

    shape_geometry 138 regions 573
    placements 65 placements; 22014 lowered into cells; 5 outside the stage;
               0 without measured bounds; 0 unreadable records; 3 read with a tail ignored
    placeholder 0
    unmeasured 0
    unmeasured sprites 0 unknown 0
    empty sprites 13 [371…382, 204]   frameless sprites 12 [371…382]   recordless sprites 1 [204]
    proof_placement kinds: 56 decoded, 9 decoded-offscreen
    sound_total 9 20612
    sound_carried <id> … match true            (for all nine)
    inspect: frame STAGE-0001 cols 80 rows 30 fills 2310 painted 21561 unresolved 0
    inspect: sound_sink_total 9 20612

20 612 is 20 668 (measured `media_sound`) minus the eight seven-byte `DefineSound` headers: the
payloads, not their counts, and the digests agree at three points — the SWF tag, the frame that was
written, and the client sink.

## Not claimed

- No sample is decoded or played. The sink consumes, holds and reports payloads; a decoder and a
  device backend are the next unit, and nothing here pretends to audio output.
- Sprite 204's own nested `SoundStreamBlock` is not carried; the carried set is the nine inventoried
  top-level sounds, and carrying nested audio is a separate decision with its own count.
- `client_witness` and `client_run_h9` stay `NOT-ENFORCED`: the inspection binary is mechanical
  evidence of what a client consumed, not a human witness of a window.
- The 3 records read with a tail ignored and the 5 placements outside the stage are unchanged and
  still declared.
