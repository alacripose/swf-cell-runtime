# TICKET-0063 — D-7b: the owner widens the importer to lower these games, without carrying their names

- Owner: project owner (decision), implementation agent (execution)
- Block: H2 import boundary
- Branch: `ticket/TICKET-0063-owner-widens-lowering`
- Ledger: LEDGER-0069
- Supersedes for these files only: D-7a's "the lowering stays with the format owner"

## Why this exists

The docs said twice that lowering a shape, a timeline or a script is not agent work: `D-7a` (*"the
lowering stays with the format owner"*) and `SCOPE-ACCOUNTING.md`, which assigns visual, timeline and
interaction lowering to *shell execution*. Meanwhile the streaming format exists for one purpose — an
importer, driven by trained CUA drivers, that auto-translates a movie into the new formats and shows it
accurately on the owner's client.

The owner has now ruled on that tension, verbatim in substance:

> the package streamer format exists so the importer — with trained CUA drivers — can autotranslate all of
> the game successfully to the new formats and display it accurately on the user's client.
>
> the reason for no agent trust is so that when I run the importer on other SWF files later, you don't get
> the game's own names and methods or assets, just the approximation in code for what it needs.
>
> you're allowed to implement as much as you want for the SWF files I add into the project.

So the *trust boundary* is not "do not lower". It is "lower into approximations, and never carry the
movie's own names, strings, identifiers, offsets or assets into code, tickets, ledger rows or commits."

## What this authorizes

For a movie the owner places in `inputs/`, and only for that movie:

1. **Geometry lowering** — shape records and their placement become declared cells/frames the package
   already knows how to draw.
2. **Timeline lowering** — display-list transitions and nested sprite clocks become per-frame documents,
   so the client can advance frames at the declared rate.
3. **Emission of frame documents** beside the package, so the existing draw path (raster → picture →
   batch → window) paints something other than text.

## What it never authorizes

- Carrying source names, action-script identifiers, string tables, tag payload bytes, media bytes or
  offsets into tracked files. Comments and identifiers stay generic (`shape`, `placement`, `frame`).
- A Flash player, a second runtime, renderer, input router or package format (`AGENTS.md` §2).
- Treating an imported game as proof that it plays: the witness stays the owner's (`AGENTS.md` §7).
- Reading any movie other than the one the owner placed.

## Order of work

Each step is its own ticket, commit and exit test, smallest first:

1. Placement parsing: `PlaceObject`/`PlaceObject2` matrices and depths from the tag stream, measured on
   synthetic fixtures, emitted as an approximation inventory (no drawing yet).
2. Geometry: shape edges → filled cells for one frame, emitted as a cellframe document bound to the
   stage window; the client already draws such a document.
3. Timeline: per-frame documents from show-frame and sprite structure; the client gains a clock.
4. Media: bitmap/font/audio kinds, still as approximations rather than carried bytes.

## Acceptance test

The decision is recorded in `docs/DECISIONS-PORT.md` as D-7b, the frontier names lowering as agent work
for owner-supplied movies, and the gate still passes with the source boundary unchanged.

## Closing

- Commit: this unit's commit, fast-forwarded onto `master`; nothing pushed
- Shell gate state: unassigned
- Ledger entry: LEDGER-0069
