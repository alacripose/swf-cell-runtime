# TICKET-0068 — The gate record is regenerated on the tree that exists

Goal, quoted from `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §1:

> "The base game and its mods must become peers in one cell-native package system."

This is an erratum unit: no product code changes, and the record it writes is the artifact.

## What happened

`selftest_record` hashes the working tree, and the working tree's bytes depend on when Git last
wrote a file. `ledger/ledger.jsonl` and `reports/*.md` were written with LF by the tools that produce
them; the commit normalised them in the object store; the branch checkout materialised them as CRLF
(`core.autocrlf` on this host). The record was generated between the write and the checkout, so it
described a tree with LF and the checkout then produced a tree with CRLF. The check is correct to
refuse: the evidence no longer matched what it certified.

Three landings died on this before it was named (TICKET-0065, 0066, 0067 all reported
`FAIL selftest_record` immediately after a clean commit), and each was repaired by re-running
selftest, which is the symptom and not the cause.

## What this unit does

- Regenerates `artifacts/redacted-test-results/port-gate-selftest.json` on the tree as checked out,
  after the branch checkout, with a clean working tree.
- Records the ordering rule: the record is generated **last**, after the checkout that materialises
  the tree being committed, never before it.

## Exit test

    python tools/port_gate.py check     # PASS on a clean tree at this commit

## Not done here, and named

The durable repair is a `.gitattributes` that fixes the working tree at LF on every platform, so the
identity cannot depend on the checkout at all. That file changes every path's identity once, which is
a unit of its own; it is the next artifact, not this one.
