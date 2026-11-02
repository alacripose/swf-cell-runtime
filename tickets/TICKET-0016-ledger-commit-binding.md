# TICKET-0016 — Bind every ledger entry to the commit that carries its unit

- Owner: implementation agent
- Block: H1 (governance), replacing the NOT-ENFORCED row `ledger_commit_hash_binding`
- Branch: `ticket/TICKET-0016-ledger-binding`
- Ledger: LEDGER-0018
- Depends on: TICKET-0003 (the gate), TICKET-0015 (deterministic record)

## The defect

Every ledger entry names a ticket and a block, but nothing ties it to history. `git log` could show a
commit whose message cites `LEDGER-0012` while the entry it claims to produce lives in some other
commit, and the gate would not notice. `ledger_commit_hash_binding` was therefore listed NOT-ENFORCED
with the note "a ledger entry cannot be bound to a hash it is committed alongside".

## Why the obvious fix is circular, and what is done instead

Storing the carrying commit inside the entry cannot work: the entry is written before that commit
exists, and any hash written into it would name a different commit or nothing at all. A placeholder
such as `"pending"` would satisfy a check while proving nothing.

So the binding is **derived from history** and then enforced:

- The gate reads every commit that touched `ledger/ledger.jsonl` and records which ids each commit
  introduces. Each entry must resolve to exactly one such commit.
- That commit must add the ticket file the entry names, so "carries the unit" is a fact about the diff
  rather than a claim in the message.
- An entry may optionally store `unit_commit`. When present it is verified: the commit must exist, be an
  ancestor of `HEAD`, and be the same commit the derivation produced. Wrong, absent or unreachable
  values are refused by name.
- The one permitted unbound entry is a newest one not yet committed, which is what the pre-commit hook
  sees; anything older that cannot be resolved is refused.

The honest path needs no repair: entries stay as they are, and the binding appears as soon as the commit
lands.

## Fixture determinism

The check runs inside the self-test, whose fixtures are throwaway repositories with different hashes
every run. Its messages therefore cite entry ids and tickets, never commit hashes, and any hash that
does appear is normalised by `normalise_problem` before it is recorded. `serialise_record()` must still
reproduce the record byte for byte.

## Acceptance test

- `check` → PASS, 17 checks, `NOT-ENFORCED (6)` — the binding row is gone, not duplicated.
- `selftest` → PASS, 33/33, including `ledger_binding_fabricated_refused` and
  `ledger_binding_wrong_commit_refused`.
- Two consecutive `selftest` runs write a byte-identical artifact and leave `git status --porcelain`
  empty.
- Every earlier refusal still reports its original message.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0018
