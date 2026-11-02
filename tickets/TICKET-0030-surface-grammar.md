# TICKET-0030 — The strict `dl-enclave.surface/1` grammar, and the one mount path through it

- Owner: implementation agent
- Block: H2 (package layer)
- Branch: `ticket/TICKET-0030-surface-grammar`
- Ledger: LEDGER-0035
- Depends on: TICKET-0029 (the frame and the boundary check this replaces)

## What was wrong

`mount` accepted a surface it had only scanned: schema row, surface id, final newline, at least one
window. A document carrying a row or key this port does not understand still produced a receipt, so the
port could mount something it could not render, and a caller could rely on a lenient path.

## What this builds

`crates/planet-package/src/surface.rs` — a typed parser, not a scan:

- `Surface { id, requires, windows }`, `Window { …17 declared columns…, nodes, actions }`,
  `Node { id, parent, kind, role, value }`, `Action { verb, tool, target }`.
- Row keys are the closed set `surface`, `requires`, `window`, `node`, `action`; `#` comments are
  skipped and a blank row is refused.
- Refusals by name, each carrying the offender: `surface-row` (unknown row key, or a blank row),
  `surface-field` (wrong column count or value shape, naming row and key), `surface-duplicate` (a key
  twice), `surface-order` (`requires` after a window, a window before the `surface` row, a parent used
  before it is declared), `surface-requires` (not `observe`/`act`/`admin`), `surface-node` and
  `surface-action` (a node or action grouped under a window that is not the current one), and
  `surface-schema` (an older or newer surface schema, refused by name exactly as `frame-schema` refuses
  an older frame).
- `mount` now calls `Surface::parse` and has no other path: a frame carrying an unknown row cannot
  produce a receipt at all. The four boundary refusals keep their shapes (`frame-kind-reserved`,
  `mount-sequence`, `surface-schema` for a surface-id mismatch, `surface-windows` for no windows), so
  the 16 tests from TICKET-0029 still pass unchanged.

## Readings chosen where the plan is silent, named rather than hidden

- **"Key" is the leading row token.** An unknown key is `surface-row`; a row carrying an extra or
  missing positional column is `surface-field` with `columns`.
- **A window that cannot be drawn is refused**: `w` or `h` below 1 is `surface-field`/`extent`.
  `min-w`/`min-h` are only required to be integers.
- **Multiple roots are allowed**; the grammar says a parent must precede its child, not that a window
  has one root, so refusing a second root would be inventing a rule.
- **No lane, rule, node-kind, node-role or theme-role vocabulary is validated**, because this
  repository owns no declared table for them and inventing one would be a second vocabulary. Those
  fields must be non-empty; the vocabulary check belongs to the block that declares the table.

## Still only checked at the boundary

Nothing in `mount` remains lenient, but the grammar's *value* vocabularies (lane, rule, kind, role,
theme-role, content resolution) are shape-checked only. That is the limit this ticket states.

## Acceptance test

`cargo test -p planet-package` — 16 frame/mount tests unchanged, plus the grammar suite: a valid surface
parses into typed windows/nodes/actions and still mounts; unknown row, unknown column, blank row,
duplicate window, duplicate surface row, duplicate node, older schema, newer schema, undeclared parent, a
node under another window, an action under another window, `requires` after a window, a window before the
surface row, unknown requirement, malformed boolean, impossible extent, wrong schema name — each refused
with its own code — and a packed surface with an unknown row refused at mount with `surface-row`.

## Closing

- Merged commit: <sha>
- Shell gate state: unassigned — only the shell assigns held/absent/refused
- Ledger entry: LEDGER-0035
