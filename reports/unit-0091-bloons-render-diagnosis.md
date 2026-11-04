# Where Bloons TD 1 stops before it can be played

Status: diagnosis, corrected against the importer's own measurements. Nothing here is claimed
`present`, and nothing here is a client witness.

## The stage timeline places one character, and it is a container

`placements 1 placements; 2400 lowered into cells` — across all 12 frames the stage's display list
places exactly one character: **132, at depth 548**. There is no second stage placement, on any frame.
That is the file's structure, not a reader gap: the timeline walk reports `0 unreadable records`,
`0 read with a tail ignored`, `0 stated absent`.

## That container's children *do* lower

`lower()` recurses into the sprite, and the importer's per-frame probe shows the art arriving:

| frame | papers | painted cells | characters |
| --- | --- | --- | --- |
| 1 | 1 | 2 400 | 132 |
| 2–6 | 2 | 2 430 | 132, 134 |
| 7–10 | 240 | 4 801 | 132, 137, 189 |
| 11–12 | **221** | **19 860** | 132 plus ~50 children (190, 194, 197, 200, 202, 205, 207, 216, 348, 350, 356, 357, …) |

So `sprites 371..382 no-display-list` is **not** what hides the game. Those twelve sprites declare
zero frames in the file (`frames == 0`), and sprite 204 declares frames whose records are only sound.
They were never the art path. The art is 132's own children, and it lowers.

## What actually blanks the picture

Inside the container, the depth-548 child paints **2400 cells on 30 rows on every frame** and it is
the last thing painted: `frame_top 11 character 132 depth 548 order [548] paper fill-99 at (0,29)
80x1`, and the importer's own coverage line reads `draw_coverage character 132 cells 2400`. Because
it is opaque by the reader's own counters — `0 masks not drawn`, `0 invisible not drawn` — the whole
stage resolves to one paper (`visible_papers 1`), which is exactly the `(255,255,255,255)` surface the
client paints.

That is an opaque full-stage child covering its siblings. Whether the file means it as a scrim, a
mask the reader failed to recognise, or a curtain the script removes is **a content judgement this
repository is not allowed to make** (`AGENTS.md` §3: only the shell assigns states and decides
content acceptability). Guessing it away would be inventing a picture the file does not state.

## Bitmaps

18 bitmap characters are counted and carried (258 207 bytes); none is decoded, and `Fill::Bitmap`
answers `(128,128,128)`. This is independent of the blank stage: no stage fill in this movie samples a
bitmap, so decoding them would change nothing visible until the occlusion above is resolved. The
decode is still a real gap and is stated as `not-implemented`.

## Sound

Unchanged and working: 8 clips decode through the client's channel with tick, rate, channels, frames
and sample offset; 1 refuses `audio-unreadable` with its reason.

## The open decision (needs the shell, not me)

Two rulings would each unblock a real step, and both are content decisions:

1. **A scrim rule.** May the importer treat a container's topmost full-stage opaque child as a scrim
   and lower the layers beneath it, naming both layers? If yes, frames 11–12 expose ~50 characters of
   real art immediately, with no reader change.
2. **Code frame admission.** Bloons' behaviour (and the rest of its art, which is scripted) needs the
   canonical program lane admitted at mount; that is `§12`, untouched here.

Until one of those lands, the honest statement is: the imported package carries the art, the lowering
produces it, and an opaque container child covers it.
