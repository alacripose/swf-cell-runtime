# AGENTS.md proposals — 2026-11-04

Each proposal is a rule, the evidence that produced it from this session, and the ticket it would
need. A proposal is not a rule until a ticket lands it in `AGENTS.md`. The health ratchet records the
same items in `artifacts/health/proposals.jsonl`.

## P-1 — Code frames are admitted only with a written admission contract

**Proposed rule.** Before `kind code` may be admitted at mount, three things exist and are cited in
the ticket: a schema addendum naming the program's schema and its refusals, a document stating what
the executor may and may not do per tick, and the shell's ruling that code frames may travel in a
mounted package. Until then the frame stays refused `frame-kind-reserved` by name.

**Evidence.** §12 refuses the frame; the reader and executor now exist (`program.rs`,
`script.rs`) and nothing admits them; the window prints `script none` rather than implying the movie
is scripted.

**Would need.** `TICKET-0085`.

## P-2 — A refusal that cannot be seen is not a refusal

**Proposed rule.** Every command that can refuse prints `RESULT: REFUSED (<code>)` (or
`REFUSED <code>: …`) on stdout and exits non-zero, and matches markers with carriage returns
stripped. A refusal that reaches only stderr or a traceback is a defect with a ticket.

**Evidence.** A staging-lock refusal printed 203 bytes to stderr with no marker, and a waiting agent
spent minutes on a line that could not appear.

**Would need.** `TICKET-0082` (landed) extended by an output-framing check.

## P-3 — Polling a log requires streaming evidence in the same command

**Proposed rule.** A redirected run may be waited on only by a command that also proves the log is
the live stream: wait on the run's exit file, report byte count and last line, refuse
`log-not-streaming` on a silent log, and refuse `wait-expired` at the bound. Batch independent edits
and checks into the same response instead of letting a wait own a turn.

**Evidence.** A `grep` loop could not distinguish a working run from one that refused before
streaming; `tools/wait_run.py` now makes the distinction printable.

**Would need.** `TICKET-0083` (landed).

## P-4 — Every test turn runs the health ratchet

**Proposed rule.** `python tools/health.py` is run before a turn is reported done. A check ever held
may not fail again; a never-held failure is a debt with an owner; `selftest_record` is the one
evidence debt and is repaired by one complete selftest run, never a rewritten digest. The unit's
report is written per unit and per day, and proposals go to `artifacts/health/proposals.jsonl`.

**Evidence.** Health was read by eye, so a regression was indistinguishable from a first failure, and
the session re-derived its own process lessons each turn.

**Would need.** `TICKET-0084` (landed).

## P-5 — No training format is adopted without a research record

**Proposed rule.** Adopting a modern training or interaction format for civilians, bots or wishes
requires a research record under `docs/research/` naming the format, the source read, the licence,
the failure modes, and the fixture that proves it. A unit that adopts a format without that record is
refused by the health ratchet as `unresearched-format` until the record exists.

**Evidence.** The mission now names "modern training formats" for civilians and CUA bots; nothing in
the repository states which formats are in scope, and adopting one from memory is the drift this
repository's rules exist to prevent.

**Would need.** a new ticket plus a `tools/health.py` rule.

## P-6 — The client axis is one address space, not a second runtime

**Proposed rule.** The client axis is a read-only address space over the mounted package — movie
tick, frame identity, cell coordinates, announced sounds and refusals — and it may not schedule,
draw, decode or store content. Anything that would make it a second runtime is refused under §2.

**Evidence.** Bots and reviewers need a quotable position ("tick 40, cell 3,2"), and the repository
has none; the risk is that the axis grows into a scheduler beside the cadence.

**Would need.** a new ticket, plus a document defining the axis.

## P-7 — A wish may not carry gate states

**Proposed rule.** A wish, a civilian and any CUA bot report only implementation facts
(`present`/`not-implemented`/`blocked`); `held`/`absent`/`refused` remain the shell's. A wish that
names a gate state is refused `wish-gate-state`.

**Evidence.** §3 forbids agents emitting gate states, and the wishing surface is a new place for that
mistake to enter.

**Would need.** a new ticket once the wishes format has an owner.

## P-8 — A bot witness is never a client witness

**Proposed rule.** A CUA-driver bot's report, an axis dump and a screenshot are artifacts, not a
client witness. `client_witness` and `client_run_h9` stay `NOT-ENFORCED` until the user runs the
client and reports what appears.

**Evidence.** §7: a headless parser or driver passing is not proof the client mounted and played.

**Would need.** a new ticket alongside the bot proof.
