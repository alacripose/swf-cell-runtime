# TICKET-0087 — lower the root's script into the package

Status: `present` for carriage; execution stays `blocked` on the shell's §12 ruling.

## The finding this answers

The user identified why the stage renders blank: `swf1` is a loader. Its root timeline places one
container and the game itself arrives through the file's own action records, so no display-list
change can paint it. The root's frames are the loader's frames, not the game's.

## What this unit does

The importer now carries every `DoAction` (tag 12) and `DoInitAction` (tag 59) record the root
states, byte for byte, as package artifacts:

- `script_records 37 bytes 57259` on `inputs/swf1.swf`;
- 37 `[artifact]` rows named `script/action-<n>-tag<code>.bin`, each with its digest and its bytes;
- the package still mounts and the evidence run still passes over all 12 frames.

The record is carried, not interpreted: this reader does not execute bytecode, and admitting a code
frame at mount remains `frame-kind-reserved` (`AGENTS.md` §12), which the axis still reports.

## Exit test

- `swf-import` prints `script_records 37 bytes 57259` and the frame carries 37 `script/action-*` rows.
- `inspect --evidence` exits 0 with 12 ticks and `script refused frame-kind-reserved`.
- `port_gate.py check` PASS; `health.py` PASS.

## Still open

Executing the carried script needs the canonical program lane compiled by the shell (§4.6) and
admitted at mount. Until then the package holds the loader's behaviour and the client reports it as
carried-but-not-run, rather than pretending the loader is the game.
