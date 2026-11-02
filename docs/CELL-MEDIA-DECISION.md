# CELL-MEDIA-DECISION — a proposal for the format owner, not a ruling

**Status:** proposal. **Nothing here is decided.** D-2 ruled that bitmaps, fonts and audio each get
their own kind with one field each, rather than a single kind with a type tag, and rather than raw
bytes inside `bands` or `materials`. This document proposes the field contents D-2 left open: what each
kind carries, what is required and what an absent optional entry means, how an unknown key is treated,
which bounds exist and where they are enforced, and how a media object is identified and referenced.

**Consistency with the earlier proposals.** No contradiction with `docs/CELL-MODEL.md`,
`docs/CELL-SCHEMA.md` or `docs/CELL-PROGRAM-DECISION.md` was found. The cell text convention (Q-2)
governs cell declarations and never governs media payloads: a media payload is binary and never shares
a kind with a declaration. One pressure point does exist and is recorded where it belongs rather than
resolved here — see M-1: D-2 fixed one field per kind, and a reader may need an extent before it can
read the field it belongs to.

**What cannot be proposed here.** No part of this document needs the shell's private conversion
material, and where a field's members depend on it the question is tabled rather than answered.

## M-1 — What fields each media kind carries

**Recommendation:** one payload field per kind, with a fixed-length preamble inside that payload
carrying what a reader must know before it can trust the rest. The row kind already supplies the media
class, so the preamble needs no tag of its own. In order: the payload length, then the kind's own
extent — two sides and a pixel format for a bitmap, a glyph count and an em size for a font, a sample
count, a channel count and a sample format for audio. The format identifiers come from closed sets,
the same discipline as shapers and capabilities. Rationale: D-2 fixed one field per kind and the row
header already fixes the field count, so everything else must live inside the single field; a
fixed-length preamble lets a reader check the declared extent against the bytes it actually has
before allocating.

**Rejected:** a second declared field for the extent. It changes the row shape D-2 fixed, and adding a
field per kind is a format change larger than the question being asked.

**Falsifier:** a reader that must know the extent before it can read the payload at all — a bitmap
whose field cannot be buffered whole. Then a preamble inside the payload cannot be reached without
buffering everything, a second declared field is required, and D-2's "one field each" must be revisited
by the owner. This is the recorded pressure point between D-2 and its reader, and it is not resolved
here.

**Owner:** format owner.

## M-2 — Required and optional entries, and what an absent optional entry means

**Recommendation:** the payload and its preamble are required and non-empty; everything else is
optional by key, and absent means the declared default, never "unknown". A font with no kerning entry
has no kerning; audio with no loop points does not loop; a bitmap with no palette is not indexed. An
empty required payload refuses by name. Rationale: two ways to express absence — a missing key and an
empty value — leave a reader's default unknowable, so only one way is allowed.

**Rejected:** absent meaning "the reader decides". The same package would then display differently on
two surfaces, which is the non-determinism the program budgets exist to prevent.

**Falsifier:** a media class where absent and explicitly-empty are genuinely different, such as a font
that must distinguish "no kerning data" from "kerning present but empty". That class needs an explicit
sentinel, and the one-way-to-be-absent rule needs a stated exception for it.

**Owner:** format owner.

## M-3 — How a reader treats an unknown or extra key

**Recommendation:** refuse the media row and quote the key. Because each kind carries one field, an
"extra field" can only be an unknown key in the payload's preamble or trailer, and the reader applies
the same name-and-refuse rule already used for unknown operations and unknown capabilities: refusal at
validation, before anything is decoded.

**Rejected:** ignoring unknown keys in the name of forward compatibility. A reader that ignores what it
does not understand cannot tell a newer package from a corrupted one, and it will display the first
while reporting the second as fine.

**Falsifier:** a future revision that must add optional keys older readers are expected to accept.
Refusal then becomes wrong for exactly those keys, the preamble needs a declared minor revision a
reader may accept, and that is a format act with a version rule for the owner to rule on.

**Owner:** format owner.

## M-4 — Bounds, and where each one is enforced

**Recommendation:** every extent is declared and refused above, never truncated, and every bound is
enforced at validation before decode — a bound checked during decode has already allocated.

- bitmap payload ≤ 8 MiB `[proposal]`, each side ≤ 4096 `[proposal]`
- font payload ≤ 4 MiB `[proposal]`, glyphs ≤ 16 384 `[proposal]`, the same order as the cell ceiling
- audio payload ≤ 32 MiB `[proposal]`, samples ≤ 4 194 304 `[proposal]`
- preamble and trailer lengths fixed per kind, so the payload can be located without parsing it
  `[proposal]`

A refusal names the kind, the bound and the observed value.

**Rejected:** one media ceiling shared by all three kinds. A glyph atlas and an audio stream differ in
natural size by orders of magnitude, so a single ceiling would either refuse ordinary audio or admit an
absurd bitmap.

**Falsifier:** a legitimate surface that needs a bitmap wider than the declared side bound, or audio
longer than the payload ceiling. The figure then rises with a stated reason, and the bound keeps its
refusal.

**Owner:** format owner.

## M-5 — How a media object is identified and referenced

**Recommendation:** a media row is identified by the frame's own name, which the frame digest already
provides, and cells reference it by that declared identifier; a reference to an identifier no row
declares refuses by name at validation. No separate index, and no identifier the package does not
derive from its own bytes.

**Rejected:** an asset index or artifact table mapping names to offsets. That is `SHPKG003`'s design,
which stays unadopted (PATCH-0005); it also creates a second naming authority beside the frame digest.

**Falsifier:** one bitmap shared by several packages. A per-package digest identifier cannot name it
across packages, so a declared external reference would be needed — that is a package dependency, not a
media field, and whoever rules on package composition must decide it.

**Owner:** format owner.

## What this cannot settle, and why

- **The members of the pixel and sample format sets.** A closed set is recommended, but its members
  depend on what the surface can upload and on what the private conversion emits; the surface side is
  not agent-readable, so the members are the owner's to write.
- **Whether a font carries outlines or a baked atlas.** Both fit the same field, and the choice changes
  the decoder and the bounds. It depends on private conversion output, so it is tabled rather than
  guessed. The two cost differently: outlines need a rasteriser in the client, an atlas needs texture
  space and a bake step before first use.

## What this must not become

- Not an asset archive: three named kinds, never a table of blobs (PATCH-0005).
- Not a second naming authority beside the frame digest.
- Not a decoder whose output differs between machines; that would make the determinism witness
  meaningless.
- Not a host for executable content: media is data, and the code lane is the only place behaviour
  lives.
