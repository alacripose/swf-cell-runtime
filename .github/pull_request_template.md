## Ticket

TICKET-NNNN

## Ledger

LEDGER-NNNN

## What changed

Source-safe description of the diff. No source identifiers, paths, offsets, URLs, script
names, decoded media, inventory references, screenshots, or captures.

## Exit test

- [ ] `python tools/port_gate.py check` — result:
- [ ] `python tools/port_gate.py selftest` — result (only when the gate changed):
- [ ] Ticket-specific acceptance test — command and result:

## Baseline conformance

- [ ] PATCH-0001 small coherent commits, each citing ticket and ledger
- [ ] PATCH-0002 ticket exists, comments record decisions source-safely
- [ ] PATCH-0003 no movie tracked, nothing force-added
- [ ] PATCH-0004 every new output path declared in `config/output-roots.json`
- [ ] PATCH-0005 one format, one surface, one runtime — no second magic/reader/surface
- [ ] PATCH-0006 no capture/input/observation/probe surface added for agents

## Incomplete state

What this PR does not deliver, with owner and blocking dependency.

## Review

- Blocking comments: (must be resolved before merge)
- Reviewer decisions:
- Gate state assigned by shell:

## Squash policy

Preserve the commit series. Do not squash away attributable audit trail.
