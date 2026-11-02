# TICKET-0032 — The surface vocabularies are declared tables

- Owner: implementation agent
- Block: H2 (package-format ownership, surface grammar)
- Branch: `ticket/TICKET-0032-vocabulary-tables`
- Ledger: LEDGER-0037
- Depends on: TICKET-0030 (the strict grammar), TICKET-0031 (the mount entry)

## What was wrong

The strict surface grammar checked that `lane`, `rule`, node `kind`, node `role`, `theme role`,
`chrome` and `moves` were **non-empty**, and nothing more. Every value passed. The declared sets are
real — the shell family declares them in `secure-shell/tools/port/surface.mjs` (`LANES`, `RULES`,
`NODE_KINDS`, `THEME_ROLES`, `TYPE_ROLES`, `CHROME`, `MOVES`, `CAPABILITIES`, `CAPABILITY_RANK`) and
refuses `unknown-lane` / `unknown-rule` there — but this port accepted `lane=anything`.

The plan names no section for these tables. AGENTS.md §1–2 and `docs/DECISIONS-PORT.md` D-6 make the
shell family the authority, so the family's file is the text this unit implements: same names, same
order, same counts, same codes.

## What this builds

`crates/planet-package/src/vocabulary.rs`:

| table | entries | refusal code |
| --- | --- | --- |
| `LANES` | 6 — frame, control, tab, world, beacon, motion | `unknown-lane` |
| `RULES` | 10 — fill, full-bleed, centre, centre-wide, right-dock, right-lower, right-stack, bottom-strip, bottom-left, small-right | `unknown-rule` |
| `NODE_KINDS` | 7 — pane, row, field, label, list, table, chart | `unknown-node-kind` |
| `THEME_ROLES` | 2 — surface, hud | `unknown-theme-role` |
| `TYPE_ROLES` | 7 — micro, small, body, lead, title, display, mono | `unknown-type-role` |
| `CHROME` | 3 — minimize, maximize, close | `unknown-chrome` |
| `MOVES` | 2 — drag, resize | `unknown-moves` |
| `CAPABILITIES` | 3 — observe, act, admin (rank 0/1/2) | `unknown-capability` |

`Vocabulary` carries each table's name, code and entries, so a refusal and a test read the same data.
`capability_rank` returns the declared rank, `role_accepts` the union of theme and type roles, and
`list_accepts` a comma-separated list of declared tokens (or `-` for none).

`surface.rs` now reads the tables instead of accepting any non-empty token: lane, rule, node kind,
theme role, chrome, moves, node role, and `requires` (through `Vocabulary::Capability`). A refusal is
`Refusal::UnknownValue { vocabulary, code, found }`, whose `code()` is the table's code and whose
message names the refused value.

## The readings taken, and why

- **Node role is the union of theme and type roles.** The grammar has one role column and the family
  declares two role tables; the union is the narrowest reading that accepts every declared role and
  nothing else. A guessed single table would refuse half the declared values.
- **`chrome` and `moves` are comma-separated token lists, and `-` means none.** The family's example
  document carries `minimize,maximize,close` and `drag,resize`; the existing single-token tests still
  parse. A bare unknown token is refused whole, naming the whole field.
- **`requires` keeps its code.** Its membership now comes from `CAPABILITIES`, but the refusal stays
  `surface-requires` (existing tests pin it); it was not re-coded to `unknown-capability`.
- **Codes for tables the family does not name.** The family declares `unknown-lane` and
  `unknown-rule`; the other six follow the same `unknown-*` shape rather than the `surface-*` row
  codes, because they are about a value, not a row. `surface-*` codes are untouched.
- **The tables that the grammar does not parse are declared but not enforced.** `TYPE_ROLES` is
  consumed through the role union and `CAPABILITIES` through `requires`; nothing in this crate yet
  distinguishes a theme role from a type role in a column, so that distinction stays asserted by tests
  only, not enforced at the boundary.

## What this does not do

- No new row, column, order rule or window limit. The grammar's accept/refuse behaviour changes only
  where a value was previously accepted for being non-empty; every currently-passing document still
  parses.
- No client crate, no executor, no vocabulary beyond the family's eight tables.

## Acceptance test

`crates/planet-package/tests/vocabulary.rs` (12 tests): the counts and order are pinned per table;
every entry of every table is parsed in the column that carries it (lane, rule, theme role, chrome,
moves, node kind, both role tables as a node role, capability as `requires`); seven out-of-vocabulary
values are refused with their seven codes; the refusal names the value; multi-token `chrome`/`moves`
and `-` parse; `capability_rank` matches the ladder; an undeclared `requires` keeps
`surface-requires`.

`cargo test -p planet-package` — 45 existing tests plus 12, none removed or weakened.

`python tools/port_gate.py selftest` then `check` green, four `absent` rows still named.

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0037
