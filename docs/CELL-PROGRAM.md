# The code lane: a deterministic cell program, compiled by the shell and executed by the runtime

Status: `present` for the reader, the grammar and the executor; `not-implemented` for admission into a
mounted package (a `code` frame is still refused `frame-kind-reserved`, `AGENTS.md` §12), and for
source compilation (shell-only).

## Who reads the source, and who reads the program

`docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md` §4.6:

> "The shell determines whether the source uses AVM1, AVM2, or both. It alone reads action records,
> script bytes, source identifiers, names, class tables, string tables, imports/exports, and runtime
> references. It compiles the required source semantics into the canonical cell-program representation
> and retains the detailed source-to-program mapping privately."

`docs/PORT-BASELINE.md`, the code-lane row:

> "format-owner decision and generic code-kind implementation contract; the shell privately builds
> source-derived AVM1/AVM2 coverage and test vectors; agent work uses synthetic fixtures and generic
> schemas only."
>
> Refuses: "smuggling code into data rows; a second runtime; exposing source-derived vectors; claiming
> script parity before the reserved code lane has a reader."

So this repository owns **the representation and its reader**; the shell owns the compilation. The
program below is what a compiled script looks like, and nothing in it carries a source name, an
offset, a string table, or a decoded asset. A package that carried any of those would be a different
artifact, and this reader would refuse it as a row it does not declare.

## The program

A code payload is a UTF-8 document: one header line, a `version` row, a `program` row, then one `op`
row per operation. Fields are tab-separated.

```text
shploingus.cellprogram
version	1
program	loader-advance
op	goto	0
op	sound	7
op	wait	40
op	next
op	stop
op	end
```

| operation | operand | what it does to the execution |
| --- | --- | --- |
| `goto` | absolute movie tick | sets the tick |
| `next` | — | advances one movie frame |
| `wait` | movie ticks | holds the tick forward by that many |
| `stop` | — | stops the timeline (`playing = false`) |
| `play` | — | resumes it |
| `sound` | sound character | announces the package's audio character to start |
| `end` | — | closes the program |

Refusals, by name (the same shape the frame reader uses, `AGENTS.md` §12):

| code | when |
| --- | --- |
| `program-header` | the payload does not open with the header — a picture is not a program |
| `program-version` | a schema this reader does not speak, with `found` and `supported` |
| `program-identity` | no `program` row, or an identifier that is empty or more than one word |
| `program-row` | a row the grammar does not declare |
| `program-opcode` | an operation this reader does not execute, with its line |
| `program-operand` | a missing, extra or unreadable operand, with its line |
| `program-length` | more operations than the declared ceiling |
| `program-budget` | an execution that ran past its step budget |

## Why it is deterministic, and why that is the point

`Program::run(start)` is pure: no clock, no I/O, no device, no allocation beyond the result. The same
program from the same tick produces the same `Execution` on every machine, in every process, at every
priority. That is what makes a scripted tick checkable in a replay: the execution is either the
compiled program's or it is not, and the difference is visible without a window.

## What is absent, named

- **Admission.** `mount` still refuses `kind code` (`frame-kind-reserved`), per `AGENTS.md` §12. The
  lane is read and executed by the runtime and its tests; a package does not yet carry one into a
  mount. Changing that is a ruling, not an implementation detail (`tickets/TICKET-0080-cell-program-lane.md`).
- **Compilation.** No AVM1 or AVM2 is compiled here. The shell compiles; this repository reads the
  result. `docs/SCOPE-ACCOUNTING.md` keeps "AVM1/AVM2 semantic compilation into cell programs" as
  `not-implemented`, owned by shell execution.
- **A program per frame.** The client's driver (`crates/planet-client/src/script.rs`) runs one
  program **once**, on the tick it is presented, and the clock carries the timeline afterwards: a
  program states where the timeline starts, whether it plays, and which sounds it starts, and it is
  not re-executed per tick because a re-executed `sound` would announce sixty times a second. A script
  that changes per frame is a package carrying a program per frame, which needs admission.
- **Admission of a program into the window.** The driver is wired into the window loop and the window
  prints `script none: no admitted code frame in the package` — the honest state while `AGENTS.md` §12
  still refuses `kind code` at mount.
- **Sound.** `sound` announces a character; nothing yet hands that name to the client's audio channel.
