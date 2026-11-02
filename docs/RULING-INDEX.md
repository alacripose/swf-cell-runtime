# RULING-INDEX — every open question across the five proposals

**Status:** index, **not a decision**. Nothing here is decided. Every row below is copied from a source
document, and the disposition column says only what that source says.

**How to read a row.** The disposition is one of exactly two values. `recommended, awaiting ruling`
means the source offers a default and the named owner must accept or overrule it. `tabled — owner
choice` means the source offers no recommendation and states what the owner must choose between. The
owner column quotes the source's owner line; where a source names two owners for one question, both
appear, because collapsing them would be a decision this index is not entitled to make. The source
column names the document that carries the wording; that document stays the only copy of it.

## The questions

| Id | Subject | Disposition | Owner | Source |
|---|---|---|---|---|
| Q-1 | Is a cell one row kind or several? | recommended, awaiting ruling | format owner | docs/CELL-SCHEMA.md |
| Q-2 | Field layout and encoding of a cell set | recommended, awaiting ruling | format owner | docs/CELL-SCHEMA.md |
| Q-3 | Shaper and projection vocabulary | recommended, awaiting ruling | format owner | docs/CELL-SCHEMA.md |
| Q-4 | Ceiling values | recommended, awaiting ruling | format owner | docs/CELL-SCHEMA.md |
| Q-5 | Binding syntax | recommended, awaiting ruling | format owner | docs/CELL-SCHEMA.md |
| Q-6 | Does the first surface use cells at all? | tabled — owner choice | project owner | docs/CELL-SCHEMA.md |
| P-1 | Which operations exist, and which refuse | recommended, awaiting ruling | format owner | docs/CELL-PROGRAM-DECISION.md |
| P-2 | Capability vocabulary, and what an unknown capability does | recommended, awaiting ruling | format owner | docs/CELL-PROGRAM-DECISION.md |
| P-3 | Budgets: what is bounded, and where the bound is enforced | recommended, awaiting ruling | format owner | docs/CELL-PROGRAM-DECISION.md |
| P-4 | Where a program is declared | recommended, awaiting ruling | format owner | docs/CELL-PROGRAM-DECISION.md |
| P-5 | How determinism is witnessed, not just promised | recommended, awaiting ruling | format owner for the absence of ambient operations; shell/test harness for the witness | docs/CELL-PROGRAM-DECISION.md |
| M-1 | What fields each media kind carries | recommended, awaiting ruling | format owner | docs/CELL-MEDIA-DECISION.md |
| M-2 | Required and optional entries, and what an absent optional entry means | recommended, awaiting ruling | format owner | docs/CELL-MEDIA-DECISION.md |
| M-3 | How a reader treats an unknown or extra key | recommended, awaiting ruling | format owner | docs/CELL-MEDIA-DECISION.md |
| M-4 | Bounds, and where each one is enforced | recommended, awaiting ruling | format owner | docs/CELL-MEDIA-DECISION.md |
| M-5 | How a media object is identified and referenced | recommended, awaiting ruling | format owner | docs/CELL-MEDIA-DECISION.md |
| T-1 | Framing: what a chunk is | recommended, awaiting ruling | format owner | docs/CELL-TRANSPORT-DECISION.md |
| T-2 | Ordering and resume | recommended, awaiting ruling | format owner | docs/CELL-TRANSPORT-DECISION.md |
| T-3 | Receipt: separate from the door admission? | recommended, awaiting ruling | format owner | docs/CELL-TRANSPORT-DECISION.md |
| T-4 | How a receiver proves a revision was applied exactly once | recommended, awaiting ruling | format owner | docs/CELL-TRANSPORT-DECISION.md |
| T-5 | The unit of delivery, and what may travel | recommended, awaiting ruling | format owner | docs/CELL-TRANSPORT-DECISION.md |
| T-6 | Does the base game travel this way? | tabled — owner choice | project owner | docs/CELL-TRANSPORT-DECISION.md |
| T-7 | Failure, staleness and tampering | recommended, awaiting ruling | format owner for transport behaviour; shell for any rollback act | docs/CELL-TRANSPORT-DECISION.md |
| T-8 | Who admits, and what bounds the transport | recommended, awaiting ruling | format owner | docs/CELL-TRANSPORT-DECISION.md |

`docs/CELL-MODEL.md` carries no question ids. Its six invariants are guarded instead, by
`cell_schema` over `config/cell-schema.json`, so the model contributes a check rather than a row.

## Open items that carry no question id

These are the places a source deliberately leaves unknowable. They are table entries rather than gaps,
and none of them is answered here.

| Item | Owner | Source |
|---|---|---|
| The member list of the operation set (inside P-1) | format owner | docs/CELL-PROGRAM-DECISION.md |
| The members of the pixel and sample format sets (inside M-1 and M-4) | format owner | docs/CELL-MEDIA-DECISION.md |
| Whether a font carries outlines or a baked atlas (inside M-1) | format owner | docs/CELL-MEDIA-DECISION.md |
| Whether a self-describing chunk header is a second family (inside T-1) | format owner | docs/CELL-TRANSPORT-DECISION.md |
| Revision numbering when several writers share one store (inside T-4) | format owner | docs/CELL-TRANSPORT-DECISION.md |
| The maximum revision size, deliberately left to the format (inside T-8) | format owner | docs/CELL-TRANSPORT-DECISION.md |

## Recorded conflicts, kept unresolved

Three sources record a pressure point rather than a resolution, and this index keeps them exactly that
way: **M-1** (one field per kind against a reader that may need the extent before the payload), **T-1**
(a resumable chunk that must describe itself against the no-second-family boundary), and **T-8** (a
transport bound that must not become a content limit). Each is an owner choice inside a question that
otherwise carries a recommendation; nothing above resolves any of them, and no wording here softens
what the source says.

## What this index cannot do

Two questions name more than one owner, and this index reproduces both rather than picking a primary:
P-5 splits the operation set from the witness, and T-7 splits transport behaviour from any rollback
act. If a ruling needs a single accountable owner per question, that is itself an owner decision, and
it is not taken here.
