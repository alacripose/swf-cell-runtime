# TICKET-0018 — Enforce the single-reader rule instead of guarding it

- Owner: implementation agent (check), format owner (the reader it names)
- Block: H2 (ownership), replacing the NOT-ENFORCED row `duplicate_package_reader`
- Branch: `ticket/TICKET-0018-duplicate-reader`
- Ledger: LEDGER-0020
- Depends on: TICKET-0002 (the ownership record), TICKET-0017

## Which row, and why this one

`source_access_isolation` names two halves: Git exclusion, which the gate already checks, and
filesystem/AC isolation, which no repository artifact can decide. Enforcing it would either duplicate
`source_exclusion` or claim the unverifiable half.

`duplicate_package_reader` is decidable because PATCH-0005's mechanical evidence says so in its own
words: "`docs/PACKAGE-OWNERSHIP.md` names the single owner for each layer or records it blocked." The
artifact is the ownership record, and the rule is singularity of the reader.

## What the check enforces

- Every layer in the family-A ownership table has exactly one owner; a layer named twice is refused,
  with both owners quoted.
- The reader layers — frame, payload/rows, declaration grammar — share one owner; two owners are a
  second reader.
- A layer must own the package frame at all, so the reader cannot become unnamed.
- Tracked code under `src/` or `crates/` may reference the frame magic only beneath the crate the
  record names. A file emitting `V4PKG001` elsewhere is a second parser, refused by path.
- With no code tree the check reports `guard: no code tree present; a second reader cannot appear
  yet`, the form PATCH-0005 already uses for its own scan, rather than claiming a pass.

## Acceptance test

- `check` → PASS, 19 checks, `NOT-ENFORCED (4)`.
- `selftest` → PASS, 38/38, including `duplicate_reader_second_parser_refused`,
  `duplicate_reader_second_owner_refused` and `duplicate_reader_permitted_parser_passes`.
- Two consecutive `selftest` runs write a byte-identical artifact with a clean worktree.

## What this deliberately does not do

- It does not claim the client adopts one reader; that is a code invariant needing the client crate.
- It does not add a list of forbidden paths beside PATCH-0005.

## Review

- Blocking comments: none
- Reviewer decisions: pending (no remote)

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned
- Ledger entry: LEDGER-0020
