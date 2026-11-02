# CELL-ADDON-DECISION — the inferred shell, its packaged delivery, and its parity check

**Status:** proposal. **Nothing here is implemented in this repository.** D-5 records the owner's
statement; this document separates what the other projects were *observed* to do from what is still a
recommendation, and names the questions that remain for owners to answer.

**The owner's statement, verbatim:** "the shell is inferred from the other projects, delivered as a streamed packaged addon that parity checks itself"

## C — the three obligations the statement imposes

### C-1 — the shell is inferred from the other projects

**Obliges:** the shell's behaviour is evidence to be inferred from the other projects, so every claim
about the shell cites an artifact in one of them and is marked observed or not proven.
**Forbids:** inferring shell behaviour from this repository alone, inventing it, or treating an
inference as a reading of the source movie.

### C-2 — delivery is a streamed packaged addon

**Obliges:** delivery is a streamed packaged addon, carrying canonical package bytes, so an addon is
the same package frame as the base content and travels over the transport D-4 established.
**Forbids:** a second manifest, a separate window format, a second package family for addons, or a
side channel beside the stream.

### C-3 — the addon parity checks itself

**Obliges:** the addon parity checks itself, before any consumer sees it, so a parity result is part of
what delivery carries rather than something a reviewer asserts afterwards.
**Forbids:** inventing a parity format here, claiming parity in this repository, or reporting parity as
verified when only the intent to check it has been recorded.

## E — what the other projects actually show

Each entry cites one artifact and states only what it supports. "Observed" means the artifact says it
happened; "not proven" means the artifact itself says it did not.

- **E-1** `openlock/README.md` — a UI shell that mounts as a normal addon and declares its windows from a package: "runs over the live game ... mounts as a normal `pak01_dir.vpk` addon and does not require any server-side plugin or custom game mode", with window geometry, titles, lanes, opacity, pins and elevation all coming from the package.
- **E-2** `openlock/examples/motd-addon/README.md` — one package frame declares a window; no second manifest, no separate window format: "It uses the package frame and loader dl-openshell already ships; there is no second manifest, no separate window format, and no client script."
- **E-3** `openlock/README.md` — an addon is the same package family, digest checked before use: a package is a real `SHPKG002`/`SHPKG003` frame and "the frame's digest is checked before its `shploingus.recipe/3` source is read". Toasts and versioned addon delivery are named there and in `openlock/docs/DELIVERY.md`, which records the v1 addon proofs.
- **E-4** `openlock/docs/DELIVERY.md` — the delivery record separates evidence from claims: versioned replacement, atomic apply-or-reject and observer toasts are listed, and the document states that in its latest pass "the v1 → v2 window swap and the observer toast were NOT observed in the running client"; they were observed in the deterministic harness and in an earlier live pass. Recorded as not proven live, not as proven.
- **E-5** `openlock/.scratch/parity-current.json` — a parity checkpoint that records a shell state per cell, with a commit and a witness: `"state": "BLOCKED"` overall, each cell carrying `"state"` of `held`, a `commit`, a `dirty` flag, a `witness` and a `sha256`.
- **E-6** `ala_civ_bots_prototype/AGENTS.md` — the consumer refuses on parity: "`./gradlew build` refuses when the ingest proof, the Mongo projection or the installed-jar parity does not hold", and the refusal may not be bypassed with a skip flag.
- **E-7** `secure-shell/README.md` — a shell with a package stream and a ledger in which an unshown change has not landed: "a Debian-hosted backend, a forked Openlock window manager and taskbar, a package stream, and a project-activity ledger", and "a change the client cannot show has not been announced".
- **E-8** `dl-enclave/README.md` — a shell shipped as its own installable image, with the client and MCP surface as separate artifacts, and no implementation claimed: "contains **no enclave implementation**, and that is deliberate".

**What the evidence supports.** The shell is a consumer of packages, not their author: it mounts a
package, declares windows from it, checks its digest, and records what it admitted (E-1, E-3, E-7).
Addons are that same frame with no second manifest (E-2). Parity is a state with a witness, produced
by a checker and consumed as a refusal by a downstream build (E-5, E-6). None of that says this
repository's shell exists.

## A — what the statement leaves open

### A-1 — What evidence counts as inferring the shell from the other projects?

**Recommendation:** an artifact in one of those projects, cited by path, with the claim it supports
kept separate from the claim it refuses; a behaviour with no such artifact is recorded as blocked.
**Rejected:** reading the source movie or any representation of it, and inferring shell behaviour from
this repository's own documents, which were written after the port began.
**Falsifier:** an inference the shell contradicts when it next runs. That does not make recording the
inference wrong; it means the evidence was insufficient and each claim must then name its gap.
**Owner:** project owner.

### A-2 — Is an addon a separate artifact from the base package?

**Recommendation:** no new artifact class. E-2 shows an addon is the same frame with its own
declaration; the port keeps that, distinguished by declared content rather than a second magic.
**Rejected:** an addon container with its own header, and an addon directory beside the package store,
both of which PATCH-0005 forbids as a second format and a second store.
**Falsifier:** an addon that must be applied atomically with base content, where a partially delivered
addon leaves a state no single package can express.
**Owner:** format owner.

### A-3 — What does parity compare, and where does its result live?

**Recommendation:** parity compares the bytes the consumer received against the package's own frame
digest and name — the integrity E-3 shows the shell already checks — and the result lives with the
delivery facts; E-5's per-cell state with a witness is the shape to follow rather than a new receipt.
**Rejected:** a parity manifest inside the addon, and a parity verdict written into the ledger by the
addon itself; the ledger row belongs to the shell's admission (E-7).
**Falsifier:** a delivery that legitimately differs from its package — re-encoding, compression, or a
transport transform — where byte parity is the wrong comparison.
**Owner:** format owner, with the shell for admission.

### A-4 — Who may declare parity satisfied?

**Recommendation:** the addon may compute and report parity for itself; only the shell may treat that
report as a gate state, and E-6 shows a downstream build refusing on parity rather than trusting a
declaration. This repository records neither.
**Rejected:** an agent or the gate declaring parity, and a parity result that bypasses admission
because it arrived inside the delivered content.
**Falsifier:** a self-check the shell must trust without being able to re-derive it, which would make
the addon the authority over its own admission.
**Owner:** shell.

## What this does not do

- It does not implement a shell, an addon loader, a stream or a parity checker, and does not claim one.
- It does not read `inputs/`, name the source movie, or derive anything from it.
- It does not decide A-1..A-4; each keeps a recommendation, a rejected alternative, a falsifier and an
  owner.
