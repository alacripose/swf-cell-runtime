# Unit 0069 — the working tree, measured

Source-safe: no supplied movie, its bytes, identifiers, offsets or media appear here.

| Command | Exit |
|---|---|
| `git config core.autocrlf false` + re-materialise from index | 0 |
| `python tools/port_gate.py selftest` | 0 — 123 of 123 |
| `python tools/port_gate.py check` (on `master`, after the branch checkout) | 0 — PASS |

Four landings reported `FAIL selftest_record` for one cause: the record hashes working-tree bytes and
the checkout changed them. The record is now generated after the tree is final, and the setting that
changed it is off for this repository.
