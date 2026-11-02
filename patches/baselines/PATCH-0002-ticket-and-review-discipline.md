# PATCH-0002 — ticket and review discipline

| Field | Value |
|---|---|
| Id | PATCH-0002 |
| Block | H1 |
| Status | active baseline (hard gate) |
| Supersedes | — |
| Weakens which | — |

## Required rule

Every implementation step, defect, shell-issued refusal, unknown, and design decision gets
a ticket. Use PRs for scoped changes and ticket/PR comments continuously to record
source-safe findings, evidence, questions, decisions, and review responses. Only the shell
creates refusal-state records or decides content status. Comments supplement — never
replace — durable code, tests, patch records, or ledger entries.

## Mechanical evidence

- Each active work item has a file in `tickets/` with owner, scope, acceptance test,
  refusal/gating conditions, dependencies, and ledger links.
- `tools/port_gate.py` check `commit_refs` proves each commit resolves to an existing
  ticket.
- `tools/port_gate.py` check `ledger_valid` proves each ledger entry resolves to an
  existing ticket and carries its exit test and evidence.
- `.github/ISSUE_TEMPLATE/ticket.md` and `.github/pull_request_template.md` exist and
  require the same fields, so the tracker and the file-based record cannot diverge
  silently.
- Unresolved blocking review comments block merge: the PR template carries both a
  blocking-comment field and a reviewer-response field.

## Not allowed under this patch

- Untracked work: changes with no ticket.
- Leaving reasoning only in private agent context.
- An agent marking a content status, refusal, or gate state.
- Treating a comment or PR discussion as proof that code works.

## Residual gap (recorded, not waived)

The review record itself needs a remote repository. Nothing in this checkout can verify
that a human reviewed a diff; the gate verifies the reference chain only.

## Changing this patch

Requires an explicit user decision plus a superseding patch record.
