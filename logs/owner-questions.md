# Owner questions — the port, 100 of them

Ask-and-log record. Each question is put to the owner in batches of twenty; the answer is written
under the question in the owner's own words (free text is kept verbatim, an option tick is recorded
as the option's label). Nothing here is source-derived: no movie, identifier, offset or media byte
appears in this file.

Status: all 100 asked and answered (5 batches of 20). Free-text answers are kept verbatim.

---

## A. Media and definition payloads

**Q1. Bitmap payload — verbatim or re-encoded?**
What travels in the package for a bitmap: the original compressed bytes exactly as the file carries
them, or a re-encoded copy?
Answer: **A re-encoded copy.** The package carries what the client can draw, not the source's own bytes.

**Q2. Is any re-encode allowed?**
If a bitmap's format cannot be drawn by the client, may the importer re-encode it, or must itship the original and refuse to draw?
Answer: **Re-encode only when undrawable.**

**Q3. Fonts — embed or rasterise?**
Does a font travel as its original font definition, or as glyph bitmaps produced by the importer?
Answer: **The original font definition.**

**Q4. Glyph coverage.**
Only the glyphs the file actually uses, or the whole font table?
Answer: **The whole font table.**

**Q5. Sound — original codec or transcoded?**
Do sound payloads ship as the file's own compressed bytes, or decoded/transcoded audio?
Answer: **Transcoded audio.**

**Q6. Streaming versus event sound.**
Must the package distinguish a streamed soundtrack from an event sound, or is one sound kind enough?
Answer: **Distinguish both kinds.**

**Q7. Media deduplication.**
Should identical payloads be stored once and referenced, or repeated per use?
Answer: **both, we need to be able to support older package reuse for when similar games use them or other swf files need functions from them**

**Q8. Media ceiling per package.**
Is there a byte ceiling for carried media (4 GiB is the package ceiling; is media separately capped)?
Answer: **No separate cap.**

**Q9. Bitmap alpha.**
How should formats with alpha versus without be represented — a declared flag, or separate kinds?
Answer: **A declared flag.**

**Q10. Bitmap scaling.**
Does the package declare how a bitmap scales (stretch, tile, nine-slice),or does the frame's extent alone decide?
Answer: **Declared per placement.**

**Q11. Definition kinds to measure.**
Which definition tags must be measured for lowering before anything else (buttons, edit text, morph
shapes, fonts)?
Answer: **Buttons, edit text, morph, fonts.**

**Q12. Clip depth.**
A placement with a clip depth masks the depths above it. Should the frame carry that as a mask, or
should the mask be declared and not applied?
Answer: **Apply it as a mask now.**

**Q13. Colour transform.**
Multiply/add colour transforms change a character's colour. Apply them to fills,or declare them and draw untransformed?
Answer: **Apply to fills now.**

**Q14. Blend modes.**
Declare only, or lower into the frame's paper roles?
Answer: **Lower into paper roles.**

**Q15. Filters.**
Drop shadow, blur and glow: declare only at this stage?
Answer: **lower all**

**Q16. Undefined characters.**
A placement naming a character the file never defines: refuse the package, or count it?
Answer: **Draw a placeholder cell.**

**Q17. Sprite nesting depth.**
How deep must sprite composition go before the rest is declared rather than drawn?
Answer: **Unbounded, bounded only by the file.**

**Q18. Which sprite frame.**
A sprite placed once has its own timeline; statically, which of its frames is drawn?
Answer: **it has to support animations and their logic**

**Q19. Media ownership.**
Does the package own the media bytes outright, or reference them in the store by digest?
Answer: **The package owns the bytes.**

**Q20. Unsupported media refusal.**
What should a reader answer when it meets a media kind it cannot draw — a named refusal,or a declared placeholder?
Answer: **A declared placeholder cell.**

---

## B. Display list and geometry

**Q21. When does real geometry replace bounds boxes?**
Shapes are measured as rectangles today. When may their paths be tessellated into cells?
Answer: **Tessellate paths now.**

**Q22. Are bounds boxes ever acceptable as drawing?**
Confirm: a rectangle standing in for a shape's artwork is not acceptable, even as an interim.
Answer: **Never acceptable.**

**Q23. Fill colours to palette roles.**
How do a shape's fill styles map onto the frame's paper roles?
Answer: **Add roles from the file.**

**Q24. Line styles.**
Draw strokes, or declare them?
Answer: **Draw strokes.**

**Q25. Mask layers.**
Are mask layers in scope for the port, or declared only?
Answer: **In scope, draw the mask.**

**Q26. Placements outside the stage.**
Counted and not drawn (current), or clipped to the edge and drawn?
Answer: **Clip and draw.**

**Q27. Depth order.**
Confirm higher depth is in front, so fills are emitted in ascending depth.
Answer: **Higher depth is in front.**

**Q28. Repeated depths.**
The last record at a depth wins for the frame. Confirm.
Answer: **Last record wins.**

**Q29. Rounding.**
Cell positions floor, extents ceil (current). Confirm or change.
Answer: **Floor start, ceil extent.**

**Q30. Skew and rotation.**
Should a rotated or skewed matrix be lowered now, or declared until geometry lands?
Answer: **Lower rotation now.**

**Q31. Sub-cell characters.**
A character smaller than one cell draws one cell. Confirm.
Answer: **One cell minimum.**

**Q32. Overlaps.**
Where placements overlap, does the later (higher) depth overwrite the earlier?
Answer: **Higher depth overwrites.**

**Q33. Move records with no character.**
A move keeps the character already at that depth (current). Confirm.
Answer: **Keep the character.**

**Q34. Lowering cap.**
Is there a maximum number of placements lowered per frame?
Answer: **No cap.**

**Q35. Oversized characters.**
A character larger than the stage: clamp to the window, or count as outside?
Answer: **Clamp to the window.**

**Q36. Shape versus sprite precedence.**
When an id is both, which measurement wins?
Answer: **Sprite extent wins.**

**Q37. Morph shapes.**
Use the start bounds, the end bounds, or declare?
Answer: **Interpolate.**

**Q38. Button states.**
Which state of a button is drawn statically?
Answer: **All states as frames.**

**Q39. Edit text.**
Draw as a box, or declare?
Answer: **Draw box and text.**

**Q40. Dynamic text.**
Declared only, since no script runs yet?
Answer: **Placeholder cell.**

---

## C. Timeline and frames

**Q41. Frame rate source.**
Use the header's declared rate for the clock?
Answer: **cadence tick defined per package from the stream store, client is asynchronous and runs at whatever the user sets the framerate to in the settings package window and wether they want vsync**

**Q42. Late frames.**
When the client falls behind, skip frames or stretch time?
Answer: **Skip frames.**

**Q43. Show-frame boundaries.**
Is a show-frame tag the frame boundary?
Answer: **ShowFrame ends a frame.**

**Q44. Sprite timelines.**
Do sprites advance on their own clocks once the client has one?
Answer: **depends on scripts**

**Q45. Stop and play.**
Honoured when a frame's actions say so, or ignored until an executor exists?
Answer: **Honour via executor.**

**Q46. Looping.**
Loop the timeline, play once, or follow a declared count?
Answer: **Loop.**

**Q47. First frame only?**
Until the clock lands, is the first frame the whole of what is drawn?
Answer: **All frames now.**

**Q48. One document or many.**
One frame document carrying all frames, or one document per frame?
Answer: **One document, many frames.**

**Q49. Package growth.**
Twelve frames of cells: is growth acceptable, or must frames share geometry?
Answer: **Share geometry.**

**Q50. Determinism.**
Must playback be tick-deterministic (same frame every run at the same tick)?
Answer: **Wall clock.**

**Q51. Goto and jumps.**
Frame jumps from actions: in scope with the executor?
Answer: **In scope with the executor.**

**Q52. Sound sync.**
Must sound follow the timeline's own clock?
Answer: **Independent.**

**Q53. Frame labels.**
Carry the file's frame labels into the package?
Answer: **Carry labels.**

**Q54. Preloader frames.**
Import them as ordinary frames?
Answer: **Ordinary frames.**

**Q55. Timeline ceiling.**
Maximum frames a package may carry?
Answer: **Cell ceiling only.**

---

## D. Text, fonts, glyphs

**Q56. Text as cells or as text.**
Does text lower to glyph cells, or travel as declared text the client shapes?
Answer: **Glyph cells.**

**Q57. The client's font.**
The drawing font is the client's own bitmap font (5×7). Is that the only font for shipped text?
Answer: **Embedded fonts required.**

**Q58. Non-ASCII.**
What happens to a glyph the drawing font lacks — refusal (current), replacement, or declared?
Answer: **Declared replacement.**

**Q59. Embedding.**
If fonts travel (Q3), is the client obliged to render them?
Answer: **Must render it.**

**Q60. Text colour.**
Text ink comes from the ink roles; confirm which role is default body text.
Answer: **`text`.**

**Q61. Wrapping and alignment.**
Does the importer wrap text to the window, or does the client?
Answer: **The importer wraps.**

**Q62. Editable text.**
Any text field becomes editable in the port, or declared read-only?
Answer: **Editable fields in the port.**

**Q63. Selection.**
Text selection in scope at all?
Answer: **In scope.**

**Q64. Ligatures and kerning.**
Required, or out of scope for shipped text?
Answer: **Required.**

**Q65. Device fonts.**
Text naming a device font: substitute, refuse, or declare?
Answer: **Substitute a declared font.**

**Q66. Bitmap fonts.**
Supported as a media kind?
Answer: **Supported kind.**

**Q67. Text bounds.**
The measured text rectangle is the placement extent (current). Confirm.
Answer: **Extent is the drawn glyphs.**

**Q68. Missing glyph.**
A run containing one unknown glyph: refuse the whole run, or the frame?
Answer: **Substitute and declare.**

---

## E. Package format and identity

**Q69. Hash into the header now?**
Should the hashed format id move into the `V4PKG001` header, replacing the numeric schema field?
Answer: **Yes, now.**

**Q70. Header layout.**
`magic | id(32 bytes) | kind | length | digest` — approved?
Answer: **Approved.**

**Q71. Digest preimage change.**
The preimage would become `domain || id || length || payload`, amending AGENTS.md §12. Approved?
Answer: **Approved, with an erratum (E-6).**

**Q72. Reader version bump.**
Does moving the id into the header change the reader version and therefore the id itself?
Answer: **Yes, bump it.**

**Q73. Reading old frames.**
Must a reader still read a v1 frame (numeric schema), or refuse it by name?
Answer: **Read it.**

**Q74. Package ceiling.**
4 GiB confirmed, with 1 GiB as the working size?
Answer: **64 gib ceiling, 4 gib working size**

**Q75. Key topology.**
Per-package keys, or one master with derived domains? This was left open.
Answer: **Per-package keys.**

**Q76. Sealed frame, sealed rows.**
Confirm the frame is sealed and contains sealed rows.
Answer: **Confirm.**

**Q77. Legacy ids on read.**
Keep accepting a legacy numeric id as an upgrade (current), or refuse it now?
Answer: **Keep accepting as an upgrade.**

**Q78. Family name.**
`planet.surface` final as the family for this layer's documents?
Answer: **check for openlock, secure shell, enclave, and shploingus, planet was never supposed to be authorative over the updated systems**

**Q79. Window packages.**
Is the stage window its own package, separate from the game's stage package?
Answer: **Own package.**

**Q80. Taskbar and dashboard.**
Two separate window packages, as ruled, or one shell package?
Answer: **Two packages.**

---

## F. Shell v2, store, dependencies

**Q81. Store of record.**
MongoDB confirmed as final, against the branch that calls it a derail?
Answer: **MongoDB final.**

**Q82. Ledger sealing.**
Seal this repository's ledger now, or after the store exists?
Answer: **Now.**

**Q83. Remote.**
Which URL should branches and PRs push to? None is configured.
Answer: **None yet; stay local.**

**Q84. Debian host.**
Is the store host reachable today, and may the importer write to it?
Answer: **Reachable, writable.**

**Q85. Qt client.**
Next mission, as answered — confirm the winit client stays evidence only?
Answer: **Evidence only.**

**Q86. Store versus files.**
Do imported packages land in the store first, or on disk for review first?
Answer: **Disk first, then store.**

**Q87. Registry keys.**
Derived automatically from ids and trees (as answered) — confirm for the v2 store?
Answer: **Derived, marked as derived.**

**Q88. Addon parity.**
What must an addon package prove before it is a peer of the base game?
Answer: **Same frame shape, own digest and receipt.**

**Q89. Cross-package references.**
May one package reference a character or media in another?
Answer: **Yes, by digest.**

**Q90. Store schema ownership.**
Who owns the store's document schema — this repository or the v2 host?
Answer: **The v2 host.**

---

## G. Evidence, gate, witnessing

**Q91. Capture as evidence.**
Is a window capture acceptable mechanical evidence (never a witness)?
Answer: **Yes, mechanical only.**

**Q92. The witness protocol.**
What exactly must you see, and where, before H9 is claimed?
Answer: **A guided checklist you sign off.**

**Q93. Record generation.**
Confirm the gate record is generated on `master` after the tree is final.
Answer: **Confirm.**

**Q94. Commit budget.**
Still 40 files and 4,000 insertions per commit?
Answer: **I don't care about this.**

**Q95. Unit size.**
One unit per turn, or a day's work per review?
Answer: **A day's work per review.**

**Q96. Ledger fields.**
Keep the seven-field row exactly as it stands?
Answer: **Keep it.**

**Q97. Not-enforced rows.**
Accept `NOT-ENFORCED` rows indefinitely, or set a date to close each?
Answer: **Keep them open with owners.**

**Q98. Playtest cadence.**
Every unit, or at milestones?
Answer: **Every unit.**

**Q99. Source boundary.**
Does the media work widen the read of the authorized input, and if so to what?
Answer: **Media bytes too, under the same one-file authorization.** Read to carry them into a package, never committed to the repository.

**Q100. What does "done" mean now?**
Given the display list is not drawn, what single visible outcome would count as the port succeeding?
Answer: **The game playable** — artwork, input, animation and script behaviour in the client.

---

## What these answers change, and the conflicts they create

The answers are logged above; this section is the agent's reading of them against what is currently
coded, ruled and written down. It is not a plan, and nothing here is implemented.

### Preservation versus drawability

- **Q1, Q2, Q5** move from "ship the source's bytes" to "ship a drawable copy": bitmaps re-encoded
  (only when undrawable), sound transcoded. The package is no longer byte-preserving for those two
  kinds, and the ledger's preservation language must say so.
- **Q3, Q4, Q57, Q59** keep fonts as the original definition but carry the whole table, and make the
  client obliged to render embedded fonts. The client's 5×7 bitmap font therefore stops being the
  only shipped text font.
- **Q19 and Q89** say the package owns its bytes but may also reference another package's payload by
  digest. Both, not either — the store needs a content-addressed table plus in-package copies.
- **Q7** says to support reuse across similar games, which is the same content-addressed table.
- **Q99** widens the read of the one authorized file to media bytes. The authorization text in
  AGENTS.md §1.1 currently covers measured structure only, so it needs an amendment before any
  payload is read.

### Rendering, which the client cannot do yet

- **Q12** masks, **Q13** colour transforms, **Q15** all filters, **Q21** tessellated paths, **Q24**
  strokes, **Q25** mask layers, **Q30** rotation/skew, **Q64** kerning and ligatures: every one is a
  renderer feature the client does not have. The current lowering draws rectangles into cells and
  nothing else.
- **Q23 and Q14** want colour roles taken from the file and blends lowered into paper roles, which
  changes the cellframe vocabulary section, not just the importer.
- **Q26, Q35** want clipping rather than counting; **Q31** keeps a one-cell minimum.
- **Q36** says a sprite's extent wins over a shape's, which reverses the precedence currently coded
  in `emit::lower` (shape first).
- **Q37, Q38** want morph interpolation and every button state as its own frame.
- **Q16, Q20, Q40, Q68** want placeholder cells for undefined characters, unsupported media, dynamic
  text and missing glyphs. That is a new declared placeholder the format does not have.

### Time, which the client also does not have

- **Q41** overrides the header's rate: the cadence tick is defined per package from the store, and the
  client runs asynchronously at whatever the user sets in a settings window, with optional vsync.
- **Q47** says all frames are drawn now, not the first one; **Q48, Q49** say one document carries every
  frame with shared geometry.
- **Q50** chooses wall-clock timing, so playback is explicitly not tick-deterministic.
- **Q44** leaves sprite clocks to scripts; **Q52** makes sound independent of the timeline.

### Format identity

- **Q69, Q70, Q71, Q72** approve the header change: `magic | id (32 bytes) | kind | length | digest`,
  preimage `domain || id || length || payload`, reader version bumped, recorded as erratum E-6 in
  `config/plan-errata.json`.
- **Q73, Q77** keep reading a v1 numeric schema and a legacy id as upgrades.
- **Q78** does **not** confirm `planet.surface`: it asks for a check of openlock, secure shell, enclave
  and shploingus ownership, and says planet was never meant to be authoritative. The family name is
  open again, so the id computed today is provisional.
- **Q74** replaces 4 GiB/1 GiB with **64 GiB ceiling, 4 GiB working size**, which contradicts the
  package ceiling in the plan and the reader's payload limit.

### Process

- **Q83** leaves the repository with no remote; `git push` stays impossible and review stays
  `NOT-ENFORCED`.
- **Q84** says the store host is reachable and writable, so the Mongo writer is no longer blocked by
  the host.
- **Q92** replaces "the owner describes it" with a guided checklist the owner signs off; that checklist
  does not exist yet.
- **Q94** says the 40-file/4,000-insertion budget is no longer cared about, while AGENTS.md §11 still
  refuses a commit over it. One of the two has to move, and a rule is not amended by an answer.
- **Q95** chooses a day's work per review, which conflicts with the current one-unit-per-turn cadence
  only in size, not in evidence.
- **Q100** sets the bar at *the game playable*: artwork, input, animation and script behaviour. The
  client today mounts a package, draws one stage paper, and routes pointer events. Nothing in that
  list is met.

---

# Round 2 — the conflicts the first hundred exposed

Questions 101–120. Same log: each answer is written under its question in the owner's words.

**Q101. How far does the widened read go?**
Q99 allows media bytes under the one-file authorization. Does that mean `inputs/swf1.swf` only, or
every game later added to `inputs/`?
Answer: **every game I add until I can verify that importer can be run in a seperate worktree which you will never see**

**Q102. Which bitmap format is the re-encoded copy?**
Q1 chooses a re-encoded copy. What format must it be — PNG, a raw RGBA raster, or something declared?
Answer: **A declared kind, format named per payload.**

**Q103. Which audio format is the transcoded copy?**
Q5 chooses transcoded audio. Ogg/Vorbis, PCM/WAV, Opus?
Answer: **PCM/WAV.**

**Q104. Which font formats may travel?**
Q3/Q4 keep the original font definition with the whole table. Is TrueType the only accepted form, any
original form, or bitmap fonts too (Q66)?
Answer: **Whatever the file carries.**

**Q105. Which digest names a payload for reuse and cross-package reference?**
Q7/Q89 need a content address. SHA-256, BLAKE3, or the package's existing frame digest?
Answer: **BLAKE3.**

**Q106. A cross-package reference whose payload is missing: what happens?**
Refuse the package, or draw the declared placeholder (Q20)?
Answer: **Draw the declared placeholder.**

**Q107. How is the 64 GiB ceiling reached without loading the file whole?**
Streaming reads with 64-bit offsets, a hard reject over the ceiling, or both?
Answer: **Stream with 64-bit offsets, reject over the ceiling.**

**Q108. Which family owns the format id now?**
Q78 reopened `planet.surface` and asked for a check of openlock, secure shell, enclave and shploingus.
Which of those families should the id name?
Answer: **it's a new synthesized format from these four**

**Q109. If the family renames, does `planet.surface/<hash>` stay readable?**
Keep it as a legacy family read as an upgrade (Q77), or drop it?
Answer: **Keep it as a legacy family.**

**Q110. What happens to the commit-budget rule in AGENTS.md §11?**
Q94 says it is not cared about. Remove the rule, raise its numbers, or keep it as written?
Answer: **Keep it as written.**

**Q111. What does a placeholder cell look like?**
Q16/Q20/Q40/Q68 introduce placeholders. Which role paints one — `divider`, `transparent`, or a new
declared role?
Answer: **A new declared role.**

**Q112. Does a placeholder carry text?**
A short label naming what is missing, or a bare pattern?
Answer: **A short label naming what is missing.**

**Q113. How are curved shape paths sampled into 8 × 16 cells?**
Sample at the cell centre, supersample inside the cell, or carry a coverage value?
Answer: **Supersample inside each cell.**

**Q114. May filters be approximated?**
Q15 says lower all. Are cheap approximations (an offset rectangle for a shadow) acceptable, or must
they match the file's parameters exactly?
Answer: **Match the file's parameters exactly.**

**Q115. Are masks cell-granular or finer?**
A mask expressed in the frame's cells, or sub-cell coverage?
Answer: **Sub-cell coverage.**

**Q116. What frame-rate range may the settings window offer?**
Answer: **Any rate.**

**Q117. Is vsync on or off by default?**
Answer: **On by default.**

**Q118. Is the settings window package built in this repository, or later?**
Q41 makes the client's rate come from a settings package window. Does that package belong to this
mission?
Answer: **Later mission.**

**Q119. Who writes the H9 checklist?**
Q92 asks for a guided checklist you sign off. Should the agent draft it for your edits, or do you write
it?
Answer: **The agent drafts it for your edits.**

**Q120. What blocks "playable" first?**
Q100 sets the bar at the game playable. Rank what must land first: shape geometry, the script executor,
input mapping onto the game's controls, or the frame clock?
Answer: **A, then D, then B+C** — geometry, then the clock, then the executor and input together.

---

# Round 3 — naming the synthesis

Questions 121–140.

**Q121. What exact family string should the synthesized format's id name?**
Answer: **`synth.surface`.**

**Q122. What canonical text should the synthesized format's hash cover?**
Answer: **The four ancestor formats' canonical texts, this grammar's, plus reader version and refusals.**

**Q123. May a BLAKE3 crate be added?**
Answer: **Add the crate.**

**Q124. PCM/WAV payloads: what sample rate, depth and channels?**
Answer: **Keep the file's own parameters.**

**Q125. What is the new placeholder role called?**
Answer: **`missing`.**

**Q126. How is the `missing` role coloured?**
Answer: **Declared per package.**

**Q127. Where does sub-cell coverage live in the frame grammar?**
Answer: **A coverage field on each cell record.**

**Q128. How many supersamples per 8 × 16 cell?**
Answer: **Until the result stops changing.**

**Q129. Which filter lands first?**
Answer: **All three together.**

**Q130. What tessellation tolerance, in twips?**
Answer: **Exact to the file's own points.**

**Q131. What must the separate worktree prove before the one-file restriction lifts?**
Answer: **the importer can provide it's own results for the agent in this worktree so the user can see
it's progress on other games without informing this agent's worktree about what the game sees, the
agent does not run in that work tree just the importer shell and whatever swf files the user wants to
test without requiring scripts**

**Q132. Which instruction set should the executor start with?**
Answer: **D-3's bounded set: count, compare, branch.**

**Q133. How should the game's controls be bound to input?**
Answer: **A declared table inside the package.**

**Q134. What is the smallest interaction that counts as playable for H9?**
Answer: **The game's own first scene is traversable.**

**Q135. Must all of the file's frames play?**
Answer: **Yes, all frames.**

**Q136. Is sound playback in this mission?**
Answer: **Yes, both kinds.**

**Q137. When may the importer write to the store?**
Answer: **As soon as an import succeeds.**

**Q138. May a MongoDB driver crate be added?**
Answer: **Yes, a driver crate.**

**Q139. What should the H9 checklist's first items be?**
Answer: **Only the final playable outcome.**

**Q140. Which new gate checks should guard the synthesized format?**
Answer: **Family registered, payload digest present, placeholder role declared.**

---

## Round 3, read against the tree

The synthesis now has a name and a hash: `synth.surface`, hashed over the four ancestors' canonical
texts, this grammar's facts, the reader version and the refusals. That supersedes the provisional
`planet.surface` id, which stays readable as a legacy family and is refused on emission.

What the answers add to the code:

- A BLAKE3 dependency for payload identity (Q105, Q123).
- A `missing` paper role and a per-package colour for it (Q125, Q126), which the cellframe vocabulary
  section has to carry.
- A coverage field on every cell record (Q127), so the cell grammar changes shape, not just its
  vocabulary.
- Supersampling that runs until stable (Q128), tessellation exact to the file's own points (Q130), and
  all three filters in one unit (Q129) — the geometry unit is the largest on the board.
- Sound played in this mission, both kinds (Q136), on PCM/WAV carrying the file's own parameters
  (Q103, Q124).
- Store writes immediately after a successful import (Q137), on the strength of Q84's reachable host,
  which contradicts Q86's "disk first, then store" — Q137 is the later answer and wins, but the
  contradiction is recorded here rather than smoothed over.
- The H9 checklist is one line: the playable outcome (Q139), so the ladder that Q92 implied is a single
  item.

Rules that stay as written: the 40-file / 4,000-insertion budget (Q110), the seven ledger fields
(Q96), `NOT-ENFORCED` rows open with owners (Q97), and evidence generated on `master` after the tree
is final (Q93).

---

# Round 4 — the migration, the digests, the sound, and two contradictions closed

Questions 141–160.

**Q141. Are the units that emit `planet.surface` rewritten in place, or does it stay a legacy-only branch?**
Answer: **Rewrite in place, keep planet readable as legacy.**

**Q142. When a unit tries to emit a legacy id, what does the refusal look like?**
Answer: **A new code that also names the family it should have used.**

**Q143. Packages already emitted with `planet.surface`: re-emit them under `synth.surface`, or leave them as readable legacy artifacts?**
Answer: **Re-emit every existing package.**

**Q144. Which units carry BLAKE3 first — payloads only, or frames and row digests too?**
Answer: **Everything at once.**

**Q145. Are cells digested individually, or only the package and its payloads?**
Answer: **Every cell individually.**

**Q146. Where is a payload digest recorded: the frame's declared text, the ledger row, the store record, or all three?**
Answer: **All three.**

**Q147. Does the frame digest itself switch to BLAKE3, or stay SHA-256?**
Answer: **Switches to BLAKE3 too.**

**Q148. Stream sound: are PCM parameters read from the file or normalized at import?**
Answer: **The file's own parameters.**

**Q149. Event sound: same rule as stream sound, or different?**
Answer: **Same rule.**

**Q150. Channel policy when the file is mono or stereo?**
Answer: **Preserve the file's channel count.**

**Q151. When is the `missing` paper role chosen, as against the existing roles?**
Answer: **Also for any unresolved reference.**

**Q152. Who declares the `missing` role's colour — the importer, the package author, or the client default?**
Answer: **The package declares it; client has a fallback.**

**Q153. How is sub-cell coverage encoded in a cell record?**
Answer: **A per-cell bitmask of subcells.**

**Q154. How does coverage round-trip through the reader — normalized, clipped, or refused when out of range?**
Answer: **Clipped to the valid range.**

**Q155. What does "stable" mean for the supersampling stop condition?**
Answer: **Either, whichever comes first.**

**Q156. What exactly counts as "traversable" for the first scene?**
Answer: **The player can cross the scene's extent with input and the drawing responds.**

**Q157. Which scene is "the first scene"?**
Answer: **The scene with the most placements.**

**Q158. How is a failed playtest recorded?**
Answer: **A refusal in the gate.**

**Q159. Q86 says disk first, then store; Q137 says store as soon as an import succeeds. Which is authoritative?**
Answer: **Q137 — store after a successful import.**

**Q160. Q92 wanted a checklist ladder; Q139 wants one line, the playable outcome. Which is authoritative?**
Answer: **Q139 as the bar, Q92 as the evidence.**

---

## Round 4, read against the tree

**The migration is in place, not beside it.** `planet.surface` and `dl-enclave.surface` are read as
legacy families and never emitted (Q141); every package already on disk is re-emitted under
`synth.surface` (Q143). The refusal for emitting a legacy id gains the family it should have used
(Q142), so `format-id-legacy-emitted` stops being a bare "wrong id" and becomes a correction.

**One digest change, everywhere at once** (Q144): payloads, frames and rows move to BLAKE3 together
(Q147), every cell carries its own digest (Q145), and a payload digest is recorded in the frame's own
text, the ledger row and the store record (Q146). That makes three records agree by construction, and
it makes a cell individually addressable across packages, which is what Q7 and Q89 asked for.

**Sound keeps the file's own parameters** for both kinds (Q148, Q149, Q150): sample rate, bit depth
and channel count are read from the source and declared, never normalized. This settles the shape of
the PCM payload but not who decodes the ancestor's compressed audio into it.

**`missing` is wider than a placeholder** (Q151): any unresolved reference paints it, not only the
four placeholder cases; the package declares its colour and the client has a fallback (Q152).

**Coverage is a per-cell subcell bitmask** (Q153) that the reader clips rather than refuses (Q154),
and supersampling stops on stability or on a hard cap, whichever comes first (Q155).

**Two contradictions are now closed, in the log's own words.** Q137 is authoritative over Q86: the
store is written as soon as an import succeeds, and the earlier disk-first ordering is superseded
(Q159). Q139 is the bar and Q92 is the evidence: H9 is claimed on one line — the playable outcome —
while the checklist is how that outcome is demonstrated (Q160).

**Two new tensions are recorded rather than smoothed over.** Q157 defines "the first scene" as the one
with the most placements, while Q134 calls it "the game's own first scene" — the two can disagree, and
the later answer wins until the user says otherwise. Q158 makes a failed playtest a gate refusal, but
the gate currently decides repository invariants, not play; giving it a playtest verdict is a change
to what the gate is, and it has no evidence source for it yet.

---

# Round 5 — the decoder, the digest preimages, and what a playtest verdict can rest on

Questions 161–180.

**Q161. Who turns the ancestor's compressed audio into the PCM/WAV payload?**
Answer: **Decode in-tree.**

**Q162. Is the decoder a crate or hand-written?**
Answer: **Add a decoder crate.**

**Q163. Where do the re-emitted packages go (Q143)?**
Answer: **Overwrite the existing artifacts.**

**Q164. After re-emission, are the legacy `.pkg` files removed?**
Answer: **Delete them after re-emission.**

**Q165. What is a per-cell digest computed over?**
Answer: **The rendered pixels.**

**Q166. Is the cell digest domain-separated like the frame digest?**
Answer: **Same framing as the frame digest.**

**Q167. Does the payload digest in the ledger change the ledger schema?**
Answer: **Same field, new algorithm, no schema change.**

**Q168. If the schema changes, what happens to the rows already committed?**
Answer: **Migrate every existing row.**

**Q169. How does the store hold a package record?**
Answer: **A new package collection.**

**Q170. If the store is unreachable when an import succeeds, what happens?**
Answer: **Import succeeds, store write is queued.**

**Q171. Can `missing` be replaced later if a script supplies the reference?**
Answer: **It resolves when the reference does.**

**Q172. Is `missing` decided at import time or at mount time?**
Answer: **Decided at mount.**

**Q173. How many subcells does the per-cell coverage bitmask cover?**
Answer: **defined per package per bitmap**

**Q174. Which bit order does the coverage mask use?**
Answer: **Row-major, top-left first, high bit first.**

**Q175. What is the hard cap for the supersampling stop condition (Q155)?**
Answer: **as an option in the settings window package**

**Q176. How is "traversable" (Q156) actually measured?**
Answer: **Manual observation only.**

**Q177. Is "most placements" (Q157) measured on the source survey or on the lowered frame?**
Answer: **both** — on the source survey and on the lowered frame.

**Q178. What is the playtest gate check called and who owns it?**
Answer: **`client_playable`, NOT-ENFORCED, owned by the shell.**

**Q179. Does the family rename get recorded in the plan documents?**
Answer: **A new plan version.**

**Q180. Does AGENTS.md get amended for `synth.surface` and the `missing` role?**
Answer: **Amend it in this mission.**

---

## Round 5, read against the tree

**The audio chain is now an importer duty with a dependency** (Q161, Q162): an ancestor's compressed
audio is decoded in-tree by a decoder crate into the PCM/WAV payload whose rate, depth and channels are
the file's own (Q148–Q150). This is the first dependency that adds a code path the package layer does
not otherwise need, and it is the largest single risk in the media unit.

**Re-emission is destructive by instruction** (Q163, Q164): the new packages overwrite the artifacts
and the legacy `.pkg` files are deleted. Nothing tracked is touched — the artifacts live under
`generated/` — but a delete step in an importer is worth naming in the ticket, because a run that
deletes before it writes loses both.

**The cell digest is over the rendered pixels** (Q165), framed like the frame digest (Q166). That is a
material tension with the id's stability rule (Q122) and with the deterministic re-emission Q131 asks
for: pixels are renderer output, so a cell digest changes when the rasteriser changes even if the
declaration did not. A stronger reader would digest the declared content and keep a pixel digest
separate; the answer as given is recorded, and the tension is stated rather than resolved here.

**The ledger answers contradict each other on their face** (Q167, Q168): no schema change because the
same field carries BLAKE3, and every existing row is migrated. If the field is unchanged and rows are
rewritten, then 77 committed rows lose the record of what they actually hashed with, which is exactly
what §9 exists to prevent. The pair is only coherent if migration means recomputing *and* marking,
which is a schema change; the contradiction is left standing in the log for the owner.

**`missing` is a mount-time decision that can resolve later** (Q171, Q172), so it is not a package
fact at all but a client state — which sits awkwardly beside Q151's "any unresolved reference" and
beside the rule that only the shell assigns states. The colour is still the package's to declare
(Q152).

**Coverage becomes a per-package declaration** (Q173): the subcell grid is named by each bitmap rather
than fixed at the cell size, with row-major, top-left-first, high-bit-first ordering (Q174). A variable
grid means the reader must carry the dimensions with the mask, so the field is a triple, not a word.

**The supersampling cap is a settings option** (Q175), which moves it out of the package and into the
client's settings window — the same window Q116 gave "any rate" and Q117 gave vsync-on.

**The playtest verdict has no automated witness** (Q176): manual observation only. Q178 accordingly
names `client_playable` as `NOT-ENFORCED` with the shell as owner, which keeps the gate honest — an
absent check named and owned, never reported as passing.

**"Most placements" is measured both ways** (Q177), so the first scene is the one that is richest in the
source survey and remains richest after lowering; where the two disagree, the measurement is the
finding, not a tie to break.

**Documentation is in scope** (Q179, Q180): a new plan version for the family rename, and AGENTS.md
amended in this mission for `synth.surface` and the `missing` role.

---

# Round 6 — the three unusable answers, put back as decisions

Questions 181–200. The first three pairs revisit answers that could not be implemented as written; the
rest cover what rounds 1–5 never asked.

**Q181. Q167 said "same field, no schema change"; Q168 said "migrate every existing row". Which is it — rewrite the 77 rows in place with no record, or migrate and mark them?**
Answer: **Migrate and mark the rewritten rows.**

**Q182. If rows are marked, where does the mark live?**
Answer: **Both: a field and a version marker.**

**Q183. Q165 put the cell digest over rendered pixels, against the id's stability rule and deterministic re-emission. Which does it cover?**
Answer: **Two digests: declared content and rendered pixels.**

**Q184. If there are two digests, where does the second one live?**
Answer: **Both on the cell record.**

**Q185. Do the mask triple and the supersample cap land in one grammar revision or two?**
Answer: **Two revisions.**

**Q186. If two revisions, which lands first?**
Answer: **The mask triple.**

**Q187. Six placements measure outside the stage. What happens to them?**
Answer: **Clip them to the frame.**

**Q188. The 26 records the survey cannot read: what do they become?**
Answer: **`missing` cells with a label.**

**Q189. The 32 characters with no measured rectangle: measure them, or declare them missing?**
Answer: **Measure from the file's font metrics.**

**Q190. Button states (up, over, down, hit) — how are they carried?**
Answer: **One role plus a state field.**

**Q191. An unknown script opcode at mount: refuse or skip?**
Answer: **Skip the opcode, count it.**

**Q192. Where do script blocks live?**
Answer: **A separate payload per object.**

**Q193. Where do the client's settings live, and is the settings window itself a package?**
Answer: **Both settings and window are package data.**

**Q194. Are the input table's action names free text or a closed set?**
Answer: **Free text, resolved by the script.**

**Q195. Two actions bound to one input: refused or resolved?**
Answer: **Last declaration wins.**

**Q196. `client_playable` is NOT-ENFORCED, owned by the shell. What evidence may it ever read?**
Answer: **A shell-authored redacted playtest record.**

**Q197. The worktree proof (Q131): how do its results reach this worktree?**
Answer: **A redacted summary the user places in the repo.**

**Q198. Which plan version carries the `synth.surface` rename, and who writes it?**
Answer: **v4, authored by the agent for your review.**

**Q199. Which AGENTS.md sections change for `synth.surface` and the `missing` role?**
Answer: **Sections 2, 3, 9 and 12.**

**Q200. Store documents: one per package or one per payload, and is there a size cap?**
Answer: **One per package, no cap beyond the 64 GiB package ceiling.**

---

## Round 6, read against the tree

**The first three contradictions are closed, and each close changes code.**

The ledger migrates **and marks** (Q181), with both a per-row field and a file-level version marker
(Q182). That supersedes Q167's "no schema change" outright: the ledger gains a field, so the schema
does change, and the 77 existing rows are rewritten with a record of the algorithm that produced
their digest. The risk that made the pair unusable is gone — a migrated row can still be read as what
it was.

The cell carries **two digests** (Q183) **on the same record** (Q184): one over the declared content,
which is stable and rides with the format id, and one over the rendered pixels, which moves with the
rasteriser and says so. Q165 is superseded rather than deleted; the pixel digest is the part of it
that survives.

The mask triple and the supersample cap are **two revisions, mask first** (Q185, Q186). The cap is a
settings-window value (Q175) and a package-declared window (Q193), so it is not frame data and does
not belong in the cellframe grammar's first change; the mask triple is frame data and goes first.

**What rounds 1–5 never asked, now answered.** Off-stage placements are **clipped** (Q187) rather
than dropped or refused, so the six measured outside the stage become visible partial cells. The 26
unreadable records become **`missing` cells with a label** (Q188). The 32 unmeasured characters are
**measured from the file's font metrics** (Q189) instead of being declared missing. Buttons are
**one role plus a state field** (Q190). An unknown opcode is **skipped and counted** (Q191). Scripts
live in **a separate payload per object** (Q192). Settings and the settings window are **both package
data** (Q193). Action names are **free text resolved by the script** (Q194) and a conflicting binding
is **last-declaration-wins** (Q195). The playtest verdict may eventually read **a shell-authored
redacted record** (Q196), and the worktree proof travels as **a redacted summary the user places in
the repo** (Q197). The plan becomes **v4, agent-authored for review** (Q198), and AGENTS.md sections
**2, 3, 9 and 12** change (Q199). Store documents are **one per package, uncapped below 64 GiB**
(Q200).

**Three tensions the answers create, recorded rather than smoothed.**

First, Q188 has the importer emit `missing` cells, while Q172 has `missing` decided at mount and
Q171 has it resolve later when a reference appears. A cell cannot be both an import-time fact and a
mount-time state; the workable reading is that the importer emits a labelled placeholder and the
client decides whether it still needs to paint it.

Second, Q191 skips an unknown opcode and counts it, but every other unknown kind in this reader —
`unknown-lane`, `unknown-rule`, `unknown-node`, `unknown-role`, `unknown-type-role`, `unknown-chrome`,
`unknown-moves`, `unknown-capability` — is a refusal by name. Skipping is a different discipline for
the same class of unknown, and the gate will need to say which one applies where.

Third, Q200 allows a package document with no cap below 64 GiB, but the store's own document limit is
16 MiB. A package document that indexes its payloads cannot be one document at that size, so either
the document references payloads held elsewhere, or the store needs a large-document mechanism. The
answer as given and the store's real limit do not meet.

**Still open after six rounds:** who supplies the font metrics Q189 requires; whether a skipped opcode
is acceptable to the gate or must be a counted, declared fact in the frame; and how a 64 GiB package
is carried by a 16 MiB document.

---

# Round 7 — what it takes to draw

Questions 201–218, asked against the one measured fact that matters: `lower()` produces no cells, so the
client has nothing but stage paper to paint.

**Q201. Where does a character's rectangle come from when the file states none?**
Answer: **The file's own font metric table.**

**Q202. If a character has neither a metric entry nor a readable outline, what then?**
Answer: **A declared fallback rectangle.**

**Q203. How is a text block's extent computed?**
Answer: **Union of the glyph outlines.**

**Q204. The 26 unreadable placement records: read them, or draw around them?**
Answer: **A tolerant second pass that skips trailing fields.**

**Q205. What is the first rendering milestone?**
Answer: **One non-paper cell on screen, from the real package.**

**Q206. May the client paint stage paper it was not given as a cell?**
Answer: **Yes, the window's declared paper may fill the extent.**

**Q207. When cells overlap, what decides the draw order?**
Answer: **The file's placement depth order.**

**Q208. What colour does a lowered cell get?**
Answer: **The file's own fill colour, quantised to the palette.**

**Q209. May the importer emit a synthetic fixture frame with known cells?**
Answer: **Yes, a fixture frame is a test asset.**

**Q210. Is a cell always 8×16 dp, or scaled to fit the window?**
Answer: **Always 8×16 dp, no scaling.**

**Q211. At what size does the client render?**
Answer: **The desktop window's pixel size.**

**Q212. Is a cell drawn as a flat quad or a sampled texture?**
Answer: **A sampled texture.**

**Q213. What must the client print per frame?**
Answer: **A full per-cell dump.**

**Q214. Is clipping at lowering enough for the six off-stage placements?**
Answer: **Clipping at lowering is enough.**

**Q215. Coverage now, or flat cells first?**
Answer: **Coverage in the same revision.**

**Q216. Where does the drawing code live?**
Answer: **A new renderer crate the client calls.**

**Q217. May the client run headlessly for tests?**
Answer: **Window only.**

**Q218. What does "rendering client" mean as a done bar?**
Answer: **A capture shows cells.**

---

## Round 7, read against the tree

**The path out of zero cells.** The survey refuses 26 of the 64 placement records because their
fields do not fit; a tolerant pass that reads the fields it can and skips trailing ones is authorized,
and it is the cheapest route to a non-paper cell, because those records may name characters that the
survey already measured.

**Measurement for text.** A character's extent comes from the file's own font metric
table, a text block's extent from the union of its glyph outlines, and a declared fallback rectangle
covers a character with neither. That supersedes the survey's current shape-or-text-rectangle lookup,
which is why 32 characters measure as nothing.

**Three tensions, recorded.** Q218 accepts a capture as the done bar, while §7 says no capture is a
client witness — the wording is narrower here ("a capture shows cells"), but it must not become the
standing proof of play. Q210 says cells never scale and Q211 says render at the desktop window's pixel
size, so the window's pixel size and the frame's cell grid can disagree; something has to own that
ratio. Q212 (sampled texture) with Q215 (coverage in the same revision) means the first renderer is not
a flat-quad painter at all, and Q216 puts it in a crate of its own.

**What this round does not answer:** which palette Q208 quantises into, and what the declared fallback
rectangle's size is (Q202 names the mechanism, not the numbers).
