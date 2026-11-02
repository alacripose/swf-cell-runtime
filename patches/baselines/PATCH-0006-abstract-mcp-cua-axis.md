# PATCH-0006 — abstract MCP and CUA Axis

| Field | Value |
|---|---|
| Id | PATCH-0006 |
| Block | H1 |
| Status | active baseline (hard gate) |
| Supersedes | — |
| Weakens which | — |

## Required rule

The agent-facing MCP surface is limited to schema discovery plus ticket, secure-cell, and
shell-declared-test operations:

`schema.describe`, `ticket.submit`, `secure_cell.get`, `test.list_declared`,
`test.run_declared`, `test.result.get`.

Nothing else. No capture, screenshot, frame or video capture, visual observation, pointer,
keyboard, text entry, runtime advance, event injection, CUA session handle, probe,
selector, coordinate, action map, package browser, or source inventory. No generic tool
that can be repurposed into those actions.

The CUA Axis driver is shell/test/user-owned. It uses generic observation and input
mechanics internally, contains no game-specific controls or gameplay logic, and is never
callable, controllable, inspectable, or observable by an agent. Its captures, observations,
and raw input traces stay shell-private. The agent receives only the redacted semantic
result of a declared test.

Secure cells carry only opaque ticket/work/cell identifiers, canonical artifact digests,
allowed operations and capability descriptors, declared test ids, and redacted
classifications or abstract state-transition contracts. They never carry source names,
script identifiers, URLs, offsets, detailed provenance, decoded assets, traces, or a
source-to-cell mapping.

## Mechanical evidence

- Every non-schema MCP call requires a valid ticket id; unticketed calls are refused.
  Implementation status: `not-implemented` (H6), recorded in
  `docs/SCOPE-ACCOUNTING.md`.
- The permitted and prohibited operation families above are the entire allowed surface; a
  schema review that adds any prohibited family must be recorded as a baseline change.
- `tools/port_gate.py` check `mcp_surface` inspects any commit that adds an MCP schema
  file under `mcp/` or `src/mcp/` for prohibited operation names. Because no MCP
  implementation exists yet, the check reports `guard: no MCP schema present` rather than
  a pass.
- Source-safety of the gate itself: this repository contains no source identifiers, so no
  test can leak them.

## Not allowed under this patch

- Agent observation of the running client, by any mechanism, including an agent-authored
  probe or a judge/controller loop.
- Game-specific button names, coordinates, sequences, level rules, score rules, item
  semantics, or guessed object paths in agent-facing code or in the driver.
- Attaching screenshots, captures, raw traces, or detailed diffs to tickets, PRs, logs, or
  MCP results.
- Agents judging content acceptability or emitting `held` / `absent` / `refused`.

## Changing this patch

Requires an explicit user decision plus a superseding patch record and a source-safety
analysis.
