# TICKET-0028 — Adopt the shell family's operating discipline in the plan and AGENTS.md

- Owner: implementation agent
- Block: H1 (gate) and every block after it
- Branch: `ticket/TICKET-0028-plan-discipline`
- Ledger: LEDGER-0033
- Authority: `secure-shell/docs/FIRST-TEN-HOURS.md`, `secure-shell/docs/PLAN.md` (D-6)

## Why

The shell family records six drifts that all have one shape: "the durable record was written after the
work, by memory or by summary, instead of at the moment of the act." `FIRST-TEN-HOURS.md` answers it with
a checkpoint every 90 minutes, per-hour refusals, and gates that answer `held` / `absent` / `refused`.
This repository ran its gate strictly but never wrote that discipline down, so a unit could pass the gate
while its ledger row was reconstructed afterwards from memory — which is what happened to the first two
shell units here.

## What changed

- `AGENTS.md` §9: a unit with no ledger entry did not happen; the seven checkpoint fields, with the goal
  quoted from the plan rather than restated.
- `AGENTS.md` §10: the gate's three answers, and that `NOT-ENFORCED` is `absent`, never `held`.
- `AGENTS.md` §11: the `squash` budget — 40 files or 4,000 insertions — as a refusal.
- `AGENTS.md` §12: the frame digest framing (domain, schema, length, payload) and its three
  consequences: the length in the preimage, an older frame refused by name, and `kind` closed with `code`
  reserved at mount.
- Plan §13.1–13.7: the same rules where the blocks are, the five gates this tree lacks named `absent`
  with owners, the ordering rules (baseline before render; end-to-end window before the code reader),
  and the refusal list for the whole port.

## What did not change

No block subject, no decision, and no erratum. The plan body and E-1…E-5 are untouched: this is an
addition at the end, which is why the gate still guards five errata rather than six.

## Acceptance test

`python tools/port_gate.py selftest` then `check` — the record must describe the tree that carries this
ticket, and the gate must answer `held` for all 20 checks with the four `absent` rows still named.

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0033
