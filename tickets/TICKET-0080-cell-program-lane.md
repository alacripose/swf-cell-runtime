# TICKET-0080 — the code lane's reader: a deterministic cell program, read and executed

Status: `present` (reader, grammar, executor, tests). Admission into a mount is `not-implemented` and
blocked on a shell ruling (`AGENTS.md` §12).

## Problem

A movie that carries pictures and no behaviour has carried its loading screen and stopped there.
`docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §4.6 says so directly, and it also fixes the split
of work:

> "The shell determines whether the source uses AVM1, AVM2, or both. It alone reads action records,
> script bytes, source identifiers, names, class tables, string tables, imports/exports, and runtime
> references. It compiles the required source semantics into the canonical cell-program representation
> and retains the detailed source-to-program mapping privately."

`docs/PORT-BASELINE.md` fixes what the agent side may build: "format-owner decision and generic
code-kind implementation contract; the shell privately builds source-derived AVM1/AVM2 coverage and
test vectors; agent work uses synthetic fixtures and generic schemas only."

Before this ticket the repository had no code-kind reader at all: `planet_package::Kind::Code`
existed as a frame kind and every reader refused it, so there was no representation for the shell to
compile into and no executor for the runtime to run.

## Change

`crates/planet-package/src/program.rs`:

- **The grammar.** A code payload is `shploingus.cellprogram`, a `version` row (schema 1), a
  `program` row, then `op` rows. Operations are `goto`, `next`, `wait`, `stop`, `play`, `sound`,
  `end` — a bounded, deterministic vocabulary of timeline effects, not a general-purpose language, so
  the executor's cost is the operation count.
- **The reader.** `Program::parse` validates every row and refuses by name with the line:
  `program-header`, `program-version` (with `found` and `supported`), `program-identity`,
  `program-row`, `program-opcode`, `program-operand`, `program-length`, `program-budget`.
- **The executor.** `Program::run(start)` returns an `Execution` — final tick, whether the timeline
  plays, the announced sound characters, the steps taken, whether an `end` was reached. It is pure:
  no clock, no I/O, no device, so the same program from the same tick produces the same execution
  everywhere.

The representation carries no source identifier, offset, string table or decoded media. A payload
that contained one would be a row this grammar does not declare, and is refused as `program-row`.

## Refused

- Admitting `kind code` at mount. `AGENTS.md` §12 keeps it refused `frame-kind-reserved`; changing
  that needs the shell's ruling, and this ticket does not take it.
- Compiling AVM1/AVM2 here. The shell compiles; a second reader of the source would be the second
  runtime §2 forbids.
- Any source-derived name, offset or vector in this tree. Tests use synthetic programs only.
- Claiming script parity. The lane reads and executes a program; it does not claim the game's own
  behaviour is present until the shell's compiled program is carried and admitted.

## Exit test

- `cargo test -p planet-package` passes, including twelve `program` tests: reading in order, a
  picture refused as a program, another schema refused with both numbers, unknown operation with its
  line, missing/extra/unreadable operand, missing identity, undeclared row, a run's tick and sounds,
  determinism across two runs, an unfinished program stated as unfinished, an over-long program
  refused rather than truncated, and a `wait` that advances the tick without a clock.
- `python tools/port_gate.py check` reports PASS (recorded in the ledger row).
