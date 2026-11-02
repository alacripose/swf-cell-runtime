# CELL-PROGRAM-DECISION — a proposal for the format owner, not a ruling

**Status:** proposal. **Nothing here is decided.** D-3 ruled the *class* of executor: a code frame
carries a cell program run under a closed instruction set, with declared budgets, capability checks,
cancellation, and deterministic failure reporting, and with no host, filesystem, network, or native
code. This document proposes the parts D-3 left open: the op set, the capability vocabulary, the
budgets, where a program is declared, and how determinism is witnessed. Each recommendation is the
format owner's to accept or overrule.

**Budget labels.** Every figure below carries one of two labels. `[proposal]` means a number chosen
here for the owner to rule on. `[class of limit]` means an order-of-magnitude anchor taken from a
comparable declared bound elsewhere in this port, not a measurement. This repository contains no
measurement of the imported program and no agent may produce one, so no figure here is evidence about
that program.

## P-1 — Which operations exist, and which refuse

**Recommendation:** a closed set organised as six families — arithmetic, comparison, memory (load and
store against a declared region), control flow (branch, call, return), display (emit a cell value, set
a declared field), and package (read a declared table entry, request a declared capability). Each
operation is total over its declared operand types, and validation happens before execution: an
unknown operation, a wrong operand count, or a mismatched operand type refuses the program by name.
Five operations are absent by construction rather than by policy — nothing reads a file, spawns a
process, opens a socket, evaluates source text, or calls into native code. There is also no operation
that loads another package. The member list itself is the owner's to write, because it must cover
whatever the shell's private compilation emits, and that mapping is not agent-readable.

**Rejected:** an extensible set with a host-provided call operation. It makes what a program can do
depend on the host's configuration instead of the frame's contents, which is the second-runtime hole
PATCH-0005 closes, and it makes validation unanswerable from the package alone.

**Falsifier:** the shell's compilation needs a behaviour that cannot be expressed by composing
members of these six families — a timing primitive, or an object-model operation with no arithmetic
encoding. That means the set is too small. The answer is then a recorded format act that adds a
member, never a host escape, so the recommendation survives only in that form.

**Owner:** format owner.

## P-2 — Capability vocabulary, and what an unknown capability does

**Recommendation:** a closed vocabulary in which one name grants one verb over one declared object
class — read a cell field, write a declared state slot, subscribe to a named event, emit display
values. Capabilities are declared by the package, granted at activation, and never granted
implicitly. An unknown capability name refuses the *program* and quotes the name; a capability the
destination does not declare refuses it and quotes both the capability and the destination. Refusal,
not degradation, because a program whose capabilities are silently reduced behaves differently on
different surfaces, which is exactly the non-determinism the budgets exist to prevent.

**Rejected:** resolving capability names at runtime with defaults, or ignoring unknown ones. Ignoring
an unknown name turns a typo into a silent privilege change, and defaulting grants access the package
never declared.

**Falsifier:** a legitimate mod needs two verbs over one object — read and then update the same
registry. A single name per capability would then force either a combinatorial vocabulary or a grant
coarse enough to permit the verbs it did not need. If that happens, a capability becomes a
verb-and-object pair, and the owner rules on the shape.

**Owner:** format owner.

## P-3 — Budgets: what is bounded, and where the bound is enforced

**Recommendation:** five bounds, each enforced at a named point rather than "somewhere":

- instructions per activation: 65 536 `[proposal]` — enforced during execution.
- call depth: 64 `[proposal]` — enforced during execution.
- operand stack depth: 256 `[proposal]` — enforced during execution.
- program working memory: 4 MiB `[class of limit]` — enforced during execution, from an allocator the
  executor owns.
- program size: 512 KiB `[class of limit]` — enforced at validation, before the program reaches the
  queue.

Exceeding any bound is a deterministic failure: execution stops at the operation that would cross it,
reports which bound and the observed count, and publishes no partial display update. There is no
wall-clock bound and no timeout, because a time-based cutoff makes failure depend on the machine,
which contradicts deterministic failure reporting. Cancellation is a separate shell act with its own
report; it is not budget exhaustion and must not be reported as one.

**Rejected:** a single fuel counter covering instructions and memory together. It cannot say which
bound was crossed, and the report is then useless to whoever must fix the program.

**Falsifier:** a program compiled from the shell's private path that legitimately needs more than one
of these bounds for a screen the base game actually has. The figure then rises with a stated reason;
the bound and its refusal stay.

**Owner:** format owner.

## P-4 — Where a program is declared

**Recommendation:** inside the already-reserved code lane, as that frame's single field. The frame
reader for code already exists, and D-3 says a code frame carries the program, so the carriage is
settled by the ruling rather than proposed here. No new kind is added.

**Rejected:** a program kind of its own, which would create a second path able to carry executable
content — one the mount would then have to refuse separately — and embedding a program in a cell
declaration, which puts executable content where the cell validator expects display data.

**Falsifier:** a package that must carry two programs with independent lifecycles in one frame. One
field cannot hold them, and the field would become a table with its own ordering rule.

**Owner:** format owner.

## P-5 — How determinism is witnessed, not just promised

**Recommendation:** determinism is demonstrated by re-execution from a recorded start state. The same
program, the same initial package state and the same input event sequence must produce the same
sequence of display values and the same final budget counter on two runs, compared literally, with the
counters exposed by the executor for the witness to read. The absence of ambient inputs is structural
rather than promised: no clock, entropy source, environment, or host state appears in the op set
(P-1), so there is nothing for a run to differ on.

**Rejected:** treating "no time operation exists" as the whole argument. The set could later admit a
host-supplied value, and only a witness that replays a recorded start state would notice.

**Falsifier:** a program that legitimately needs a timer, such as an animation. The port must then
supply a virtual tick counter that the witness can replay; if that counter is ever fed by real time,
determinism is lost and this recommendation fails.

**Owner:** format owner for the absence of ambient operations; shell/test harness for the witness.

## What cannot be settled here

- **The member list of the op set.** It must cover what the shell's private compilation emits, and
  that mapping is not agent-readable (H4, H5). What is derived above is the six families, the
  validation order, and the five absences; the enumeration is the owner's.
- **Whether the reserved code lane is populated for the base game at all.** That is H5's compiler
  contract and depends on the same private inventory. This document does not assume it.

## What this must not become

- Not a host escape: no operation may reach outside the package's declared state.
- Not a second executor: the cell runtime stays the only one.
- Not a wall-clock budget, and not a native compilation of the instruction set — D-3 forbids native
  code, and both would make failure machine-dependent.
- Not code smuggled into a data row: the plan names that as the failure this lane exists to prevent,
  and the program lives in the code lane or nowhere.
