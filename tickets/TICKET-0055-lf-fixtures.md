# TICKET-0055 — The gate's fixture inputs are pinned to LF, so its record survives a checkout

- Owner: implementation agent
- Block: H9 client surface (repository evidence, not rendering)
- Branch: `ticket/TICKET-0055-lf-fixtures`
- Ledger: LEDGER-0061
- Depends on: TICKET-0053 (the record repair this unit finishes)

## The defect this closes

`fixture_identity` in `tools/port_gate.py` hashes the **raw bytes** of every `FIXTURE_PATHS` input.
`core.autocrlf=true` made a checkout rewrite those files to CRLF, so a record generated before that
checkout certified a tree that no longer existed — the failure TICKET-0053 repaired by hand. The
approved policy is LF for the fixture inputs only; this unit applies it.

## What was measured first

- 91 fixture inputs across `AGENTS.md`, `.gitignore`, `.github/`, `config/`, the eleven named
  `docs/` files, `patches/`, `tickets/`, `ledger/` and `tools/`.
- 45 of them still carried CRLF, totalling 10,694 lines — over both the 40-file and the 4,000-insertion
  commit cap, so the work is split into cap-sized commits rather than one large one.
- `git config --get core.autocrlf` → `true`; there was no `.gitattributes`.

## What this does

`.gitattributes` pins `text eol=lf` for those paths, so the index stores LF and a checkout reproduces
LF — the bytes the record is regenerated against. Every CRLF fixture input is then rewritten to LF in
the working tree and committed in batches that each stay at or under 40 files and 4,000 insertions.
The selftest record is regenerated **last**, once every fixture input has its final byte form, and a
fresh `git worktree` checkout is used to prove the committed record's identity matches what a checkout
actually produces.

## No exception is needed, and here is the proof

Nothing in the gate parses a fixture input's line endings: `grep -n '\\r' tools/port_gate.py` finds no
rule that reads carriage returns, and the only eol-sensitive thing in the tool is the byte hash itself.
The inputs are Markdown, JSON, JSONL, Python and patch files, none of which require CRLF; the patch
files are diffs that git applies by content. So all 91 inputs are pinned, with no named exception.

## What stays open

- Running the window by hand and the H9 on-screen witness — the shell's and the user's.
- §4.7 input hit areas, §6.2/§6.4 shell transactions, §4.8 persistence, §9.2/§9.4 witness, XML
  lowering.

## Acceptance test

Every fixture input has LF in the working tree and the index; `.gitattributes` pins them; each commit
is at or under the 40-file / 4,000-insertion cap; `cargo test -p planet-package -p planet-client`
exits 0 (present 6, batch 7, picture 6, draw 5, scene 6, adopt 12, binding 21, cellframe 19, surface
22, vocabulary 13); `port_gate.py selftest` exits 0 (123 of 123) and `port_gate.py check` exits 0 (21
checks, 4 not enforceable); and a fresh worktree checkout reproduces the record's fixture identity.

## Closing

- Commit: this unit's commits, fast-forwarded onto `master`
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0061
