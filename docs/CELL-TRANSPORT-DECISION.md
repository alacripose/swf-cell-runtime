# CELL-TRANSPORT-DECISION — a proposal for the format owner, not a ruling

**Status:** proposal. **Nothing here is decided.** D-4 ruled that the port gains a streamed transport
as a new layer, explicitly not "mount-v2", and fixed the boundary: the transport moves canonical
package bytes and may not define them, become a second package family, a second mount store, or a
second content schema. Today's measured route stays `Mount::watch` over a directory plus the package
door admission. This document proposes the eight things D-4 left open.

**Consistency with the committed proposals.** No contradiction with `docs/CELL-MODEL.md`,
`docs/CELL-SCHEMA.md`, `docs/CELL-PROGRAM-DECISION.md` or `docs/CELL-MEDIA-DECISION.md` was found: the
transport carries frames, so cells, media and programs travel inside their own kinds untouched. Two
pressure points are recorded rather than resolved — T-1, where a resumable chunk must describe itself
without becoming a second format, and T-8, where a transport bound must not quietly become a content
limit. T-3 also has to agree with an existing measured statement rather than a proposal: the ownership
record already says the receipt is the door admission plus the ledger record, with no separate receipt
object to invent.

## T-1 — Framing: what a chunk is

**Recommendation:** no new magic and no new row kind. A delivery is one canonical package file; the
transport cuts it into chunks at declared boundaries, each chunk tagged with a revision identity, a
chunk index and the chunk count in a control message beside the bytes, and the receiver verifies the
reassembled bytes against the package's own frame digest and name before anything else looks at them.
Rationale: every proposal to give chunks their own header ends in a second family that a reader must
decode before it can hand over a package — exactly what D-4 forbids. Content integrity is already
carried by the frame, so the transport only needs to say where it is in a byte sequence.

**Rejected:** a chunk envelope with its own magic, version and row vocabulary. Even a small one creates
a second thing on the wire that is not a package, and every tool that reads packages would have to
learn it. Also rejected: a transport row kind inside a canonical frame, which defines content rather
than moving it.

**Falsifier:** resumption after a restart with no surviving control state. A receiver that must rebuild
its position from the bytes alone needs the boundaries to be self-describing, which means a header —
and then the question is whether a transport-only header counts as a second family. That is an owner
choice, not something this document resolves.

**Owner:** format owner.

## T-2 — Ordering and resume

**Recommendation:** one revision is delivered whole, in index order, and a gap refuses the revision
rather than reassembling around it; resume continues from the last contiguous chunk of the same
revision only, and a new revision invalidates an unfinished one. Rationale: the mount already adopts or
refuses whole (`Mount::watch` semantics), so a transport that applied partial content would create a
state the mount is specified to make impossible.

**Rejected:** out-of-order reassembly with holes, and per-chunk application. Both let a partially
delivered addon exist, which is the interrupted state H10's witness is meant to prove cannot survive.

**Falsifier:** a revision large enough that resending all of it after one lost chunk is unacceptable. A
selective retransmit of missing chunks inside the same revision would keep the rule; anything that
orders chunks across revisions does not.

**Owner:** format owner.

## T-3 — Receipt: separate from the door admission?

**Recommendation:** no new receipt object. The transport reports transport facts — bytes transferred,
digest verified — and the content receipt remains the door admission plus its ledger row. Rationale:
the ownership record already states that this is what the receipt is, and a second receipt type would
be a second authority claiming the content was adopted.

**Rejected:** a transport receipt type asserting application, signed by the transport. The transport
does not write the store and cannot know whether the door admitted anything.

**Falsifier:** a sender that must learn the receiver *mounted* a revision without reading the receiver's
ledger. That needs a receipt carrying the admission identity back, which is a distinct object with its
own failure modes; the owner rules on whether it exists.

**Owner:** format owner.

## T-4 — How a receiver proves a revision was applied exactly once

**Recommendation:** revision identity plus store state, not transport bookkeeping. Each delivery names a
monotonic revision for one package identity; the store records the triple of package identity,
revision and content digest atomically with adoption. A resent revision whose digest matches that
record is a duplicate and is acknowledged without re-adopting; the same revision number with a
different digest is refused as conflicting. Exactly-once is then an effect on the store that anyone can
verify by reading it.

**Rejected:** transport-level deduplication keys or acknowledgements. The transport can crash after
acknowledging and before the store is written, so an acknowledgement proves nothing about what was
adopted.

**Falsifier:** two receivers sharing one store, where a per-package revision number gives no total
order. That needs an order the store assigns, and the owner decides its shape.

**Owner:** format owner.

## T-5 — The unit of delivery, and what may travel

**Recommendation:** the unit is one canonical package, byte-identical to what a watched file would
contain. The transport does not transform, transcode, or reconstruct, and it carries no deltas or
patches. A code frame travels as bytes and is still refused at mount while no executor exists, so the
transport cannot become a route around that refusal. Rationale: D-4's boundary sentence is exactly
this — move bytes, define none.

**Rejected:** deltas or compressed reconstructions, which would require the receiver to own a second
content schema to rebuild a package, and a bypass mode that hands executable content straight to a
consumer without the mount's refusal.

**Falsifier:** a delivery size where whole-package transfer is impractical — a very large addon on a
narrow link. That is a compression question for the owner, and the answer may not be a second content
form.

**Owner:** format owner.

## T-6 — Does the base game travel this way?

**Recommendation:** none is derivable, and none is offered. This depends on how the base game is
deployed and on inventory this repository may not read; nothing in the plan or the committed proposals
settles it. What follows is the choice, priced.

1. **One path for base game and addons.** Every content change uses the transport. Fewer branches in
   the mount, and the "one mount path" constraint is satisfied literally — but the first mounted
   surface then depends on the transport existing, which pushes H9's witness later.
2. **Addons first, base game on the watched file.** H9 can be witnessed sooner, and the transport is
   added beside a working route — but two routes for content exist for a while, which the constraint
   warns against, and the second route must be retired deliberately rather than left to drift.

**Rejected:** deciding blind. Picking either without the deployment facts would be a guess presented as
a ruling, and the two errors cost differently: option 1 can stall the only witness that matters,
option 2 can leave a second delivery route in place indefinitely.

**Falsifier:** for option 1, evidence that the base game cannot be delivered this way at all. For
option 2, evidence that the addon path is the only path the game ever exercises, which would make the
watched file dead weight from the start.

**Owner:** project owner.

## T-7 — Failure, staleness and tampering

**Recommendation:** every failure refuses the whole revision and leaves the declaration in force
untouched: a chunk that fails the digest check refuses the revision as tampered and names the chunk; a
revision older than the one recorded is refused as stale rather than applied; an interrupted transfer
leaves nothing behind, because the transport never writes the store. A tampered revision that happens
to reassemble to a valid package is not tampering at all — it is a different package, and the frame's
own identity is what says so.

**Rejected:** applying a stale revision to "roll back", which turns a delivery ordering mistake into
content rollback — and rollback is the shell's act, with its own witness, not a transport default.

**Falsifier:** a legitimate need to deliver an older revision deliberately, such as a revert. That is a
rollback operation with its own record, and the owner must decide whether the transport may carry it at
all.

**Owner:** format owner for transport behaviour; shell for any rollback act.

## T-8 — Who admits, and what bounds the transport

**Recommendation:** the transport hands verified bytes to the package door and holds no persistent
state; only the door writes the store and the ledger, so the transport can never be a second mount
store. Its own bound is an in-flight buffer of at most one package, with chunk size proposed at
64 KiB `[proposal]` and the maximum revision size deliberately not defined by the transport — it is
whatever the format already allows, so the transport cannot become a content limit by accident.

**Rejected:** streaming chunks directly into the store or into a staging area the transport owns. Both
create partial state outside the door, which is the second mount store D-4 forbids.

**Falsifier:** a delivery that must exceed one package in flight, such as several addons pushed
together. That needs explicit queuing with its own ordering rule, and the owner decides whether the
queue belongs to the transport or to the door.

**Owner:** format owner.

## What this cannot settle, and why

- **The base game's delivery route (T-6).** Owner choice, priced above; the facts needed are not
  agent-readable.
- **Whether a transport-only chunk header is a second family (T-1).** Owner choice, priced above.
- **Any revision numbering the store assigns when several writers exist (T-4).** Owner choice; the
  shape depends on the store, which is the shell's.

## What this must not become

- Not a second package family, second reader, second mount store, or second content schema (PATCH-0005).
- Not a bypass of the mount's refusal of code frames, and not an executor by another name.
- Not a receipt authority standing beside the ledger.
- Not a delivery route whose partial state a crash can leave behind.
