# PATCH-0003 — source and build-artifact boundary

| Field | Value |
|---|---|
| Id | PATCH-0003 |
| Block | H1 |
| Status | active baseline (hard gate) |
| Supersedes | — |
| Weakens which | — |

## Required rule

Never commit the original, intermediate, or finished source-movie files (`*.sfw` or
`*.swf`). The source movie is a local, immutable, read-only shell input; its digest, byte
length, signature, and project-relative location are recorded only in shell-private
provenance, not in agent-visible Git files or comments. The intended product is the
canonical cell package, not a rebuilt movie.

Nothing in this repository may name the source file. Access isolation is enforced by the
shell; Git exclusion is not a substitute for it.

## Mechanical evidence

- `.gitignore` excludes `*.swf`, `*.sfw`, `inputs/`, `shell-private/`, `captures/`.
- `tools/port_gate.py` check `source_exclusion` inspects `git ls-files` and fails if any
  tracked path ends in `.swf` or `.sfw`, case-insensitively; it also fails if the required
  ignore rules are missing.
- `tools/pre-commit` runs the gate before every commit, so a force-added movie is
  rejected at commit time.
- `.github/workflows/port-gate.yml` runs the same gate in CI.
- `tools/port_gate.py selftest` stages a synthetic movie file in a throwaway fixture
  repository and asserts the gate refuses it.

## Not allowed under this patch

- Tracking or force-adding a movie, at any stage, under any name.
- Committing source digest, filename, or path into agent-visible files.
- Writing a converted or "finished" movie.
- Relying on `.gitignore` alone for the agent access boundary.

## Changing this patch

Requires an explicit user decision plus a superseding patch record.
