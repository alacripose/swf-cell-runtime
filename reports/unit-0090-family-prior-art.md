# Family prior art, recorded from the user (2026-11-04)

The user stated, while unit 0090 was being certified: the `blocked` and `not-implemented` rows of the
mission matrix **exist in family branches** — `openlock` and `ala_civ_bots_prototype` named — and
"we can have our own scratch formats for the shell ruling in the future."

## What this records, and what it does not

- It records provenance: the prior art for the civilian/wishes format, the CUA driver bot model, the
  axis shape, the legacy-format handling and the MCP surface is expected to be in those family
  branches, not invented here.
- It does **not** answer any question in `docs/MISSION-QUESTIONS-220.md`, and it does not change a
  state to `present`: nothing in this repository has been read from those branches, so each row stays
  `blocked` / `not-implemented` until a unit actually implements or imports it.
- It authorises **scratch formats**: a format this repository invents for its own use (the evidence
  artifact kind `headless-evidence/1`, registered in `config/legacy-formats.json`) is a scratch format
  awaiting the shell's ruling, not an upstream key. A scratch format names itself as one and never
  claims an upstream identity.

## Next unit that this unblocks

A prior-art survey ticket: list, from the named branches, what already exists for each matrix row
(axis, civilians/wishes, CUA bot, MCP), and for each either adopt it as the canonical form or write a
scratch format with the differences named. Reading those checkouts needs the user's exact paths and a
ticket, because `AGENTS.md` §1 confines agent reads to this repository.
