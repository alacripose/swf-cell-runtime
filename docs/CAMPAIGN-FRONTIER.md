# Campaign frontier

Regenerated at each landed unit. Every row names who owns the next step and the plan sentence that
decides it. "Agent" means implementation work this repository's agent may do; it never means the agent
drove, observed or vouched for a running client.

Last regenerated at: TICKET-0059 / LEDGER-0065.

## What this shell is for

The shell streams SWF *games* through the one cell-package format (`V4PKG001` payload, `dl-enclave.surface/1`
declaration, `shploingus.cellframe/1` frames). It is not a Flash emulator and does not reproduce a Flash
player: source interaction semantics are compiled by the shell into the package, and the client draws and
routes what the package declares.

## Frontier

| Section | State | Owner of the next step | Rule that decides it |
| --- | --- | --- | --- |
| §4.7 focus, key repeat, pointer capture, wheel, touch | built | agent (done) | "Focus, key repeat, pointer capture, dragging, wheel behavior, touch (when present), and event propagation are implemented in the client." |
| §4.7 event propagation into nodes | blocked | format owns it | Nodes declare `id`, `parent`, `kind`, `role`, `value` and no geometry; drawn rectangles carry no node identity. Routing a press into a node would invent the rectangle. |
| §4.7 source hit areas and button semantics | shell | shell | "The shell compiles source interaction semantics into one event graph owned by the cell surface." |
| §4.7 verification input | shell, then user | shell / user | "Verification input is generated only by shell-declared tests or the user." |
| Running the client (H9) | not enforceable here | user | "Agents have no pointer/keyboard MCP, no CUA control, no arbitrary runtime-advance tool, and no ability to probe the game." |
| §6.2/§6.4 delivery, replacement, rollback | shell | shell | "The shell alone executes and owns the complete delivery transaction." |
| §4.8 persistence mechanics | built | agent (done) | "An agent may implement generic transaction machinery against synthetic fixtures through a reviewed code ticket." |
| §4.8 schema meaning, migration, destructive operations | shell | shell | "it cannot inspect live saves, decide migration semantics, initiate a live migration, or perform a destructive content operation." |
| §9.2/§9.4 oracle and witness | shell, private | shell | Shell-private oracle runs; agents receive redacted results only. |
| XML lowering, second surface format | closed | nobody | "No second package magic, package row grammar, asset archive, or window/surface format." |
| Push to a remote | blocked | user | The repository has no configured remote; the owner must supply one before a push can exist. |

## Working rules learned the hard way

- Scratch scripts go under `target/`, which the gate skips, never under `tools/`, which the fixture
  identity hashes. A scratch file left there makes a record that describes an uncommitted tree
  (TICKET-0060).
- `port_gate.py check` does not compare the record's identity with the tree; only a fresh `git worktree`
  checkout does. Both are required evidence.

## Gates

- `python tools/port_gate.py check` - repository invariants. `PASS`, `NOT-ENFORCED` (absent) and `FAIL`
  are `held`, `absent` and `refused`; a `NOT-ENFORCED` row is never a pass.
- `python tools/port_gate.py selftest` - proves the gate refuses bad states.

## Current artefacts

- `crates/planet-client/src/input.rs` - the router: pointer, wheel, key repeat, capture, touch.
- `crates/planet-client/src/render.rs` - the one device path, now forwarding platform events to the router.
- `crates/planet-save` - save transaction mechanics over synthetic fixtures.
- `generated/` - untracked output: the importer's package and any frame documents beside it.

## Next action

The owner runs the client against the generated package and reports what appears. Until then the H9
row stays not-enforced, and no claim that the game plays is made by anyone here.
