# TICKET-0033 — A node role records which declared table it came from

- Owner: implementation agent
- Block: H2 (surface grammar)
- Branch: `ticket/TICKET-0033-role-split`
- Ledger: LEDGER-0039
- Depends on: TICKET-0032 (the vocabulary tables)

## What was wrong

TICKET-0032 refused an undeclared node role as `unknown-node-role` from the union of `THEME_ROLES`
and `TYPE_ROLES`, and said so plainly: the split between the two tables was asserted by tests only.
A consumer holding a parsed `Node` could not tell a theme role from a type role, so the declaration's
distinction existed in the tables and nowhere in the parsed type.

## What this builds

`vocabulary::RoleClass { Theme, Type }` with `classify_role(&str) -> Option<RoleClass>`, and
`Node::role_class` set at parse time by the same lookup that decides acceptance. The refusal is
unchanged: an undeclared role is still `Refusal::UnknownValue { vocabulary: "node role", code:
"unknown-node-role", found }`, and `Node::role` still carries the declared string.

## The reading taken, and the one refused

`dl-enclave` PACKAGE-FORMAT §2.5 says a node's `role` is "a theme role ... or a type role ...". It
declares no mapping from node kind to role table, and the shell's validator (`tools/port/surface.mjs`)
enforces exactly that union. So the unit records the class the declaration makes and invents no
per-kind rule: a `label` may carry a theme role, a `pane` a type role, and both are accepted, because
that is what the text says. A proposed mapping from kind to class was refused as invention.

`tests/role_class.rs` proves the classification is total and unambiguous: every theme entry classifies
`Theme`, every type entry classifies `Type`, an undeclared role classifies as nothing, and the two
tables are disjoint (a name in both would make the class ambiguous and is refused by the test).

## What this does not do

- No new row, column, limit or order rule; no refusal code changed or added.
- No client crate, no executor.

## Acceptance test

`cargo test -p planet-package` — 58 existing tests plus 6 in `tests/role_class.rs`; every entry of both
role tables parses with its class recorded, disjointness holds, and an undeclared role is refused
`unknown-node-role` naming the value.

`python tools/port_gate.py selftest` then `check` green, four `absent` rows still named.

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0039
