# PATCH-0004 — project-local artifacts

| Field | Value |
|---|---|
| Id | PATCH-0004 |
| Block | H1 |
| Status | active baseline (hard gate) |
| Supersedes | — |
| Weakens which | — |

## Required rule

Every generated output — including shell-private compiler IR, source inventories, package
artifacts, logs, captures, CUA traces, reports, and test results — is written beneath the
repository root. Shell-private outputs remain inaccessible to agent providers. Do not use
`/tmp`, `%TEMP%`, an OS cache, a sibling checkout, or an external scratch directory as an
output destination.

Atomic staging files are created beside their final destination inside the same project
tree, then renamed. Runtime caches live under a project-owned directory or in memory.

## Mechanical evidence

- `config/output-roots.json` declares every output destination and its class.
- `tools/port_gate.py` check `output_roots` canonicalizes each declared path and fails if
  it escapes the project root, is absolute, is a drive-qualified path, or is missing its
  class/visibility annotation. It also requires the shell-input and shell-private classes
  to be Git-ignored.
- `tools/port_gate.py selftest` adds a `../` escape to a fixture configuration and asserts
  the gate refuses it.
- The gate's own self-test scratch directory is project-local
  (`target/port-gate-selftest/`) and is removed after the run.

## Not allowed under this patch

- Any output path outside the checkout.
- Staging to an external temp directory and copying in later.
- Claiming that project-local storage implies agent access: `inputs/`,
  `shell-private/`, and `captures/` are additionally excluded from agent workspace
  mounts and process permissions by the shell.
- Declaring a new output destination without adding it to `config/output-roots.json`.

## Changing this patch

Requires an explicit user decision plus a superseding patch record.
