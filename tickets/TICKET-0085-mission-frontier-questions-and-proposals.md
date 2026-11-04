# TICKET-0085 — the mission frontier: 220 questions, a frontier map, and eight proposals

Status: `present` (question log, frontier map, proposals). No code changes; no admission taken.

## Problem

The mission widened from "render the imported content" to a system: client axis, civilians and a
wishes format, a CUA-driver node bot proof, secure-shell v2, legacy package formats, dependency
handling, an MCP surface, drag-and-drop import, streaming, and a health ratchet that refuses
training-format adoption without research. None of that was written down as questions or as an honest
frontier, so every turn would have re-derived scope from the prompt.

## Change

- `docs/MISSION-QUESTIONS-220.md` — 220 numbered questions in eleven groups (rendering 1–30, sprites
  31–50, scripts 51–80, sounds 81–100, client axis 101–130, civilians/wishes 131–155, CUA driver
  156–180, secure shell v2 181–192, packages/dependencies 193–206, MCP/streaming/other games
  207–216, health and training-format research 217–220), each answerable in place. A question is not
  a decision.
- `docs/MISSION-FRONTIER-2026-11-04.md` — what is present with its artifact, and what is
  `not-implemented` or `blocked` with its blocker, using only the implementation facts §3 allows.
- `docs/AGENTS-PROPOSALS-2026-11-04.md` — eight proposals, each a rule with the session evidence that
  produced it and the ticket it would need: code-frame admission contract, refusal markers,
  streaming-evidence polling, the health ratchet, the training-format research refusal, the axis as
  one address space, wishes may not carry gate states, and a bot witness is never a client witness.

## Refused

- Answering the questions on the user's behalf. They are questions because the answer changes the
  contract.
- Adopting any training format before the research record exists (P-5).
- Treating the frontier map as a scope reduction: §13 says build the maximal system, so the map names
  what is missing rather than trimming it.
- Claiming the mission is complete because it is documented.

## Exit test

- The question log contains exactly 220 numbered questions, every group's range matching its count.
- The frontier map names an artifact for every `present` row and a blocker for every `blocked` row.
- `python tools/health.py` prints `RESULT: PASS` on this tree with zero open debts.
- `python tools/port_gate.py check` prints `RESULT: PASS (22 checks, 5 not enforceable)`.
