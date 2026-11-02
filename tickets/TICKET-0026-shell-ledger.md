# TICKET-0026 — The shell's commit boundary, built from the shell family's own ledger

- Owner: implementation agent
- Block: H1 (gate) and the foundation the shell's later blocks commit through
- Branch: `ticket/TICKET-0026-shell-ledger`
- Ledger: LEDGER-0030 (this repository's port ledger)
- Depends on: D-6 (authority: the shell family, not the planet tree)

## Why this is the first shell code

`secure-shell/docs/ARCHITECTURE.md` puts the backend in charge of "the package stream, the record,
the ledger, and the events". `secure-shell/docs/LEDGER.md` states the rule the family keeps: "The
ledger is the project's commit and audit boundary. A change is landed when its entry is appended and
fsynced; the entry and the evidence are one record." `dl-enclave` D17 repeats it: "Proposals never
mutate state; one typed commit point, with its audit record." Nothing built later — mount, parity,
client witness — can be accounted for before that boundary exists.

`planet-shploingus` is not the authority for the shell. It is evidence about a package reader.

## What this unit adds

`crates/planet-shell-ledger`, a dependency-free Rust crate:

- One entry per change, with every field the family's ledger names as owned: `operation_id`, `at`,
  `actor`, `kind`, `authority_ref`, `subject`, `digest`, `evidence` (command, exit status, check
  count, surface, digest), `outcome`.
- A SHA-256 chain with `seq`, `prev`, `hash` from `genesis`; `verify()` refuses a dropped, reordered
  or edited entry.
- A caller-supplied `expected_seq`: a writer that saw a different head is refused as `stale-seq`
  instead of being silently rebased.
- An idempotent `operation_id`: a retry returns `Existing(seq)` and appends nothing.
- Stable refusal codes for missing authority, missing evidence, empty subject, a digest that is not
  64 lowercase hex, unrepresentable field content, a wrong schema, and an unreadable line.
- SHA-256 implemented in-tree and checked against the standard vectors, so the chain rests on no
  dependency a reader cannot audit.

## What this unit does not do

- No package bytes, no mount, no parity checker, no HTTP/SSE boundary, no client, no fsync path yet:
  the durable form is one tab-separated line per entry and the file writer arrives with the mount
  record that needs it.
- No source access of any kind; every test uses synthetic digests.

## Acceptance test

`cargo test -p planet-shell-ledger` — genesis linkage, stale-seq refusal, idempotent retry, missing
authority, missing evidence, non-SHA-256 digest, uppercase digest, edited entry, dropped first entry,
reordered pair, durable round trip.

`python tools/port_gate.py selftest` then `check` — both stay green.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0030
