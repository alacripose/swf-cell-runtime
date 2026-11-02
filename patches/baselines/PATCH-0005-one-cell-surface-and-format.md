# PATCH-0005 — one cell surface and one format

| Field | Value |
|---|---|
| Id | PATCH-0005 |
| Block | H1 |
| Status | active baseline (hard gate) |
| Supersedes | — |
| Weakens which | — |

## Required rule

The SWF-derived base game and all streamed mods use the same canonical cell package, mount
path, cell graph, scheduler, and single rendered surface. No SWF compatibility surface, no
alternate mod format, no second executor, no second package store, no second input router,
no second renderer.

The base game is not a special case: it is a root package in the same family, decoded by
the same reader, mounted through the same route, executed by the same scheduler. There is
no raw-movie runtime path in the client, and no compiled artifact may reintroduce one.

## Mechanical evidence

- `tools/port_gate.py` check `format_magics` scans the code tree (`src/`, `crates/`,
  `tools/`, `config/`) for package magic tokens and fails on any token other than the
  authoritative `V4PKG001`; it also fails on second-format identifiers such as `shpkg` or
  `openlock.addons`.
- Because no code tree exists yet, this check currently scans zero files and reports
  `guard: 0 code files present`. It is a guard, not evidence of enforcement over real
  code, and the gate says so rather than reporting a pass.
- `docs/PACKAGE-OWNERSHIP.md` names the single owner for each layer or records it blocked.
- A single surface identity is a code-level invariant; its enforcement requires the client
  crate and is tracked as `not-implemented` in `docs/SCOPE-ACCOUNTING.md`.

## Not allowed under this patch

- A second package magic, row grammar, asset archive, or window/surface format.
- Direct mounting of arbitrary movie bytes at runtime after compilation.
- A Flash player, browser runtime, or compatibility window beside the client.
- Copying the legacy window payload as a second content format.
- Claiming a visual-only success criterion.

## Changing this patch

Requires an explicit user decision plus a superseding patch record.
