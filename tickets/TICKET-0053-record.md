# TICKET-0053 — The frame chain closes on the trunk; the record is regenerated to match it

- Owner: implementation agent
- Block: H9 client surface (the headless chain; the witness stays the shell's)
- Branch: `ticket/TICKET-0053-record`
- Ledger: LEDGER-0059
- Depends on: TICKET-0052 (draw batch), TICKET-0051 (picture), TICKET-0050 (cell raster)

## Baseline, proven by running it, not by reading a ticket

- `git rev-parse`: branch `master`, HEAD `0f2a108`, `master` `0f2a108`. `git merge-base --is-ancestor`
  confirms both `05e4586` (picture) and `0f2a108` (draw batch) are reachable from `master`, and that
  HEAD is an ancestor-or-equal of the trunk. Working tree clean.
- `cargo test -p planet-package -p planet-client` exits 0 with batch 7, picture 6, draw 5, scene 6,
  client adopt 12, binding 21, cellframe 19, surface 22, vocabulary 13, plus the smaller-frame
  `cellframe-extent` refusal at the picture and batch layers.
- `python tools/port_gate.py check` **failed**: 1 of 21 (`selftest_record`).
- `python tools/port_gate.py selftest` exits 0, 123 of 123 scenarios.

## The one thing that was actually broken

`artifacts/redacted-test-results/port-gate-selftest.json` carried fixture identity
`sha256:acd23c22…` while the trunk's fixture is `sha256:811590d4…`, so the recorded evidence
described a tree that no longer existed. `fixture_identity` (`tools/port_gate.py`) hashes the raw
bytes of every `FIXTURE_PATHS` input — `AGENTS.md`, `config/`, the docs, `ledger/ledger.jsonl`,
`tools/port_gate.py` — and a checkout rewrote those bytes' line endings after the record was
generated. The measured proof: `tools/port_gate.py` and `ledger/ledger.jsonl` are CRLF in the working
tree while the record was produced when at least one of them was LF.

The repair is the one the message itself names, and the one earlier units already took as their own
`…-record` commits: regenerate the record on the **final** tree, after every fixture input has its
committed byte form. This unit therefore appends its ledger row with the file's own line ending and
regenerates the record last; `check` then passes 21 of 21. The durable alternative — a
`.gitattributes` rule pinning `eol=lf` for the fixture inputs — is deliberately **not** taken here: it
rewrites every fixture file on the next checkout and belongs in its own ticket with its own review.

## Re-read in code, not taken on trust

- `batch::plan` takes `&WindowRaster` and nothing else: it resolves no role (`Quad.colour` is an
  `Rgba` the raster already carried), measures no frame, parses no document or binding, and
  `Quad`/`Batch` hold **no string at all** — no role, no row, no glyph.
- Its refusals are `Refusal::CellframeGlyph { line, window, frame, glyph }` (no batch produced) and,
  from the load before it, `cellframe-extent` naming the window, the frame and the binding row's line.
- `picture::paint` is unchanged and still reads only the raster; `draw::rasters` still resolves roles
  through `Palette`; `palette.rs` is still the only place a role becomes a colour; `font.rs` still
  borrows the package's own `CELL_W_DP`/`CELL_H_DP`.
- `grep -rnE 'Rgba|Palette|rgba|pixel|colour|color' crates/planet-package/src` returns doc comments
  only: the package still carries no colour, palette, raster or rendering concept, and
  `shploingus.cellframe/1`, `check_extent` and the load-time extent refusal are untouched.
- `tests/batch.rs` compares `replay(&batch)` — an independent fill loop over the rectangles — against
  `picture::paint(...).pixels` with `assert_eq!`, for four different glyph runs. It is a byte-for-byte
  comparison, not a restatement of intent.

## The plan, read to settle whether a headless unit remains — it does not

Everything the plan still asks of this chain is assigned to the shell or the user:

- **§5.1 Runtime ownership / §5.2 Rendering path** — the remainder of the renderer is *texture atlas
  or bind-group references*, *layer/depth/mask/filter passes* and *audio device commands*, all of
  which need a `wgpu` device and a surface. Presentation is the shell's step; this agent opens no
  window and depends on no GPU.
- **§4.7 Input and interaction lowering** — the client implements the router, but the hit areas,
  focus, key repeat and button semantics come from the shell's compilation of source behaviour, and
  "verification input is generated only by shell-declared tests or the user". Building a hit-test over
  declared nodes would invent the semantics the plan gives the shell.
- **§6.2 Delivery transaction / §6.4 Unload, replacement, rollback** — "the shell alone executes and
  owns the complete delivery transaction"; an agent "cannot directly validate, stage, commit, unload,
  replace, migrate, or roll back a live package set". `AGENTS.md` §3 says the same.
- **§4.8 Persistence and determinism** — shell-controlled mutation. The one allowance for agent work
  (generic transaction machinery against synthetic fixtures) is a separate lane with its own ticket,
  not the unit after the batch, and nothing here authorises it.
- **§9.2 parity oracle / §9.4 client witness**, and completion **items 6–10** (gameplay loop, save and
  restart, addon streaming, refusal without disturbing the mounted game, the witnessed client) — all
  shell-private evidence the agent cannot produce.
- **H9 — first playable integration witness** stays explicitly with the shell and the user. This
  chain built the headless half of item 5 and stops there.

So this unit invents nothing further: it lands the trunk, regenerates the evidence and records the
boundary above.

## Acceptance test

`cargo test -p planet-package -p planet-client` exits 0 (batch 7, picture 6, draw 5, scene 6, adopt
12, binding 21, cellframe 19, surface 22, vocabulary 13, with both extent refusals); `python
tools/port_gate.py selftest` exits 0 (123 of 123); `python tools/port_gate.py check` exits 0 (21
checks, 4 not enforceable, `client_witness` still `NOT-ENFORCED`).

## Closing

- Commit: this unit's commit, fast-forwarded onto `master`
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0059
- Deliberately open: device presentation (§5.1/§5.2), §4.7 input hit areas, §6.2/§6.4 shell
  transactions, §4.8 persistence, §9.2/§9.4 witness, XML lowering, and H9 for the shell and the user.
