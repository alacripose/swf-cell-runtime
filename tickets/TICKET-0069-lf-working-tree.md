# TICKET-0069 — The working tree is LF on this host, so identity stops moving

Goal, quoted from `docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §1:

> "The base game and its mods must become peers in one cell-native package system."

TICKET-0068 regenerated the record after the checkout. That was necessary and not sufficient: the
checkout itself rewrote `ledger/ledger.jsonl` and `reports/*.md` to CRLF because this host has
`core.autocrlf=true`, so the next record described another tree again. The check was right both times.

## What this unit does

- Sets `core.autocrlf=false` in this repository and re-materialises the working tree from the index,
  so every checkout produces the bytes the record hashes.
- Regenerates `artifacts/redacted-test-results/port-gate-selftest.json` **after** that
  re-materialisation, and commits it with this ticket.

## Exit test

    python tools/port_gate.py selftest   # 123 of 123
    python tools/port_gate.py check      # PASS, and PASS again after a branch checkout

## Not done here, and named

The repository-local setting does not travel: a clone on another host inherits its own
`core.autocrlf`. The portable repair is a `.gitattributes` with `* text=auto eol=lf`, which re-hashes
every path once and therefore needs its own record. Named, not done.
