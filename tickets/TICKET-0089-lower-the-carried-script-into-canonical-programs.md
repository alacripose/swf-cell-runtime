# TICKET-0089 — lower the carried script into canonical programs and report the opcode set

## Problem

The loader's 37 action records were carried byte for byte and nothing read them. "Carried but not
run" is not a pass, and a hand-written list of Bloons-specific opcodes would not carry the next
game. The package needs a canonical, game-independent program per record, and the importer must say
exactly which opcodes it understood and which it refused.

## Requirement

- `crates/swf-import/src/avm1.rs`: a table-driven AVM1 action decoder that emits one canonical
  instruction per record (`op <hex> <name> <len> <payload>`), refuses an unknown opcode by number
  (`avm1-opcode-unimplemented`), a truncated record (`avm1-record-truncated`), a push whose framing
  does not terminate (`avm1-push-unterminated`, `avm1-push-truncated`) or whose value type is not
  implemented (`avm1-push-type-unimplemented`), and starts a `DoInitAction` record after its
  two-byte character id rather than at byte zero.
- The importer carries `script/program-<n>-tag<code>.txt` beside the raw `.bin` record for every
  record it can lower, and prints per-opcode counts plus `script_refused` lines for the rest.

## Acceptance evidence

- `swf-import` prints `script_lower records 37 decoded 37 refused 0 opcodes 18` with the per-opcode
  histogram (3863 `push`, 1415 `get-variable`, 810 `jump`, 741 `set-variable`, 741 `if`, 37
  `define-function`), and the package carries the program artifacts.
- `cargo test -p swf-import` passes, including the decoder's own tests.
- `inspect --evidence` still exits 0 over the imported package.
