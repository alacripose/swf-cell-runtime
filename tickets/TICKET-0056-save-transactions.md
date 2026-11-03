# TICKET-0056 — The shell's save/recovery transaction machinery, as a library it calls

- Owner: implementation agent (mechanics only; the operation stays the shell's)
- Block: H9 (repository work on the runtime contract)
- Branch: `ticket/TICKET-0056-save-transactions`
- Ledger: LEDGER-0062
- Depends on: TICKET-0011 (the ledger's sha256), TICKET-0055 (the LF pin this unit writes inside)

## The boundary, quoted from the plan before any code

`docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §4.8 says, verbatim:

> The shell/runtime alone selects, authorizes, stages, and executes any save migration, package
> replacement, addon unload, state rewrite, rollback, or recovery reconciliation. An agent may implement
> generic transaction machinery against synthetic fixtures through a reviewed code ticket, but it cannot
> inspect live saves, decide migration semantics, initiate a live migration, or perform a destructive
> content operation.

and §0.4:

> Agents MUST NOT delete, overwrite, rewrite, redact, sanitize, quarantine, unload, replace, migrate, or
> otherwise destructively alter source-derived content, mounted game content, saves, or live package
> state.

So the lane is open to this agent, and it is exactly one lane wide: a **library of mechanics** that the
shell calls. This unit therefore implements no storage of its own, reads no save, decides no migration
semantics, and cannot act at all without a `Store` the caller hands it. Every state the plan assigns —
authorization, acceptability, `held`/`absent`/`refused` — stays the shell's.

## What this adds — `crates/planet-save`

- `Record`: a slot, the schema that reads it, the revision it replaces, opaque bytes, and the digest the
  caller **declared**. `Record::new` computes the digest; `Record::declared` carries one that may not
  match, which is how a corrupt save arrives. The framing is the family's: `domain || schema (LE u16) ||
  len (LE u64) || bytes`, hashed with `planet_shell_ledger::sha256` — no second hash, no new dependency.
- `Store`: `load`, `save`, `erase`. The shell supplies the real one; `MemoryStore` is a reference store
  for synthetic fixtures with an injectable write fault, and it is the only store any test uses.
- `Transaction`: `stage` checks slot reservation, schema, digest, duplicate slot and staleness (a
  revision that is not newer than the mounted one) **before** a record is accepted; `commit` writes the
  journal first, then the records, then marks the journal committed and erases it, returning a `Receipt`
  with each slot's pre-operation and post-operation digest; `rollback` restores every pre-operation
  snapshot.
- `journal` / `JournalEntry` / `JournalState`: the journal is a record in the reserved slot
  `planet-save/journal` carrying each slot's **immutable pre-operation snapshot** and its after-digest,
  as text rows so a human can read it.
- `recover`: the named recovery state the plan requires on restart — `Clean`, `Committed(slots)`,
  `RolledBack(slots)` or `Unresolved(slots)`. An unmarked journal is rolled back; a committed one stands
  and is erased; a slot matching neither snapshot is reported, never guessed at.

Codes, all in a `save-*` family because this is a new subsystem with no `cellframe-*` meaning:
`save-schema`, `save-digest`, `save-stale`, `save-duplicate`, `save-slot`, `save-store`, `save-journal`.
Every refusal is a `Result` naming the slot and the rule; none is a panic, a silent skip or a warning.

## One repair found by the tests, not by inspection

My first `commit` erased the journal even when the store was still refusing writes and the
pre-operation snapshots had **not** been restored — a half-applied set with nothing left to reconcile
it. The tests caught it; the commit path now keeps the journal whenever it cannot restore every slot,
and the scenario proves recovery finishes the job once the store returns.

## Fixtures

Every fixture is built in the test file from synthetic bytes; no save file, no live state, no shell
storage is read or written. Nothing new needs a pin, and the two files this unit adds under `tickets/`
and `ledger/` are already covered by TICKET-0055's `text eol=lf` rule and written LF.

## What stays open

- §4.7 input hit areas, §6.2/§6.4 delivery and replacement transactions, §9.2/§9.4 witness, XML
  lowering, H9 and its manual window run.
- §4.8 persistence **proper** and migration: the schema's meaning, what a save contains, and whether a
  migration is authorized are all the shell's decisions; this crate deliberately has no opinion.

## Acceptance test

`cargo test -p planet-save` — 12 scenarios: commit writes and reports digests; an unsupported schema is
refused before any write; a tampered digest is refused; a non-newer revision is refused and the mounted
record stays; a slot staged twice is refused; the journal slot cannot be staged; a store fault leaves
the journal for recovery, which rolls back to the pre-operation bytes; an interrupted commit reconciles
to `RolledBack`; a committed journal reconciles to `Committed` and is erased; an untouched namespace
keeps its bytes and revision; a rollback restores every snapshot; a refusal is a `Result` naming the
slot and the rule.

Plus the unchanged suites: package 9/21/19/16/8/6/22/13, client present 6, batch 7, picture 6, draw 5,
scene 6, adopt 12; `port_gate.py selftest` 123 of 123; `port_gate.py check` 21 checks, 4 not
enforceable; and a fresh `git worktree` checkout whose record identity MATCHES.

## Closing

- Commit: this unit's commit, fast-forwarded onto `master`; nothing pushed
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0062
