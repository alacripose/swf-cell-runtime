# AGENTS.md — operating rules for this repository

This repository ports an SWF game into one Rust `winit` + `wgpu` cell-package runtime.
The governing plan is `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md`; the block
sequence and its exit tests are in `docs/PORT-BASELINE.md`.

Read this file before touching anything. It is not style guidance: every rule below is
a hard gate (`patches/baselines/`), and `tools/port_gate.py` enforces the parts that are
mechanically decidable today.

## 1. The source movie is not agent-readable

The supplied SWF/SFW lives in `inputs/`. It is a shell-only trust boundary. Do not read,
open, list, parse, hex-dump, decompress, inventory, summarize, name, or describe it, and
do not read any representation derived from it.

Do not read or write: `inputs/`, `shell-private/`, `captures/`.

Do not create agents, scripts, greps, or tools that reach those paths. Do not ask the
user to paste source bytes, source names, offsets, URLs, script identifiers, decoded
media, or compiler IR into the conversation.

Agents work only from: approved package/cell contracts, generic implementation
specifications, synthetic fixtures, opaque ticket/secure-cell identifiers, and
shell-authored redacted test results.

## 2. One format, one surface, one runtime

The base game and every addon are the same canonical cell package (`V4PKG001` frame plus
the cell declaration/frame grammar owned by the package layer). No second package magic,
no `.shpkg` / `openlock.addons/*` content format, no Flash player or browser beside the
client, no second renderer, input router, scheduler, or package store. See
`docs/PACKAGE-OWNERSHIP.md` for the current ownership status.

## 3. States belong to the shell

Only the shell assigns `held`, `absent`, or `refused`, decides content acceptability,
quarantines records, or performs destructive content operations (unload, replacement,
migration, rollback, redaction). Agents do not emit those states and do not judge source
content.

For implementation tracking, agents use only `present`, `not-implemented`, `blocked`
(see `docs/SCOPE-ACCOUNTING.md`). Those are implementation facts, never gate states.

## 4. Workflow: ticket → branch → small commits → PR

1. Every step, defect, unknown, and design decision gets a ticket in `tickets/`.
2. Branch from main as `ticket/TICKET-XXXX-short-name`.
3. Commit small coherent units. Every commit message starts with its ticket id and the
   ledger entry id it produces, e.g. `TICKET-0007 LEDGER-0007 add frame digest check`.
4. Open a PR using `.github/pull_request_template.md`. Unresolved blocking comments
   block merge. Comments record source-safe reasoning only.
5. Merge only after the gate passes. Close the ticket with the merged commit, the shell's
   gate state (not yours), and the ledger entry.

No direct-to-main implementation, no giant uncommitted change sets, no squashing away
the audit trail.

## 5. Output locality

Every write resolves beneath the repository root. No `/tmp`, `%TEMP%`, OS cache, sibling
checkout, or external scratch directory. Configured destinations live in
`config/output-roots.json`; atomic staging directories must be created beside the final
destination inside the project tree.

Generated packages go to `generated/`, shell-private material to `shell-private/`, raw
captures to `captures/`. None of those are committed. Redacted, source-safe results may
go to `artifacts/`, `reports/`, and `logs/` and are versioned.

## 6. Never commit a movie

`*.swf` and `*.sfw` are never tracked, at any stage, for any reason. The intent is the
canonical cell package, never a rebuilt movie.

## 7. Proof

A headless parser test passing is not proof the client mounted and played the game. The
client is the test surface for mounting, input, gameplay, and addon claims; execution and
observation belong to the shell/test harness or the user. Never claim a port works from
code inspection alone, and never claim an unenforced invariant is enforced.

## 8. Gate

```sh
python tools/port_gate.py check      # repository invariants
python tools/port_gate.py selftest   # proves the gate rejects bad states
```

Install the pre-commit hook once after cloning:

```sh
cp tools/pre-commit .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit
```

Checks that cannot be decided yet are listed by the gate as `NOT-ENFORCED` with an owner
and blocking dependency. They are never reported as passing.
