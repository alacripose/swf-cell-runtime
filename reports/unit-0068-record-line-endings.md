# Unit 0068 — the gate record and line endings

Source-safe: no supplied movie, its bytes, identifiers, offsets or media appear here.

| Command | Exit |
|---|---|
| `python tools/port_gate.py selftest` (after the checkout, clean tree) | 0 — 123 of 123 |
| `python tools/port_gate.py check` | 0 — PASS |

The record is generated on the tree that will be committed, after the branch checkout. Before this
unit the order was inverted, which is why three landings reported `FAIL selftest_record` on a tree
whose content was correct.

The durable repair — `.gitattributes` fixing `eol=lf` — is named in the ticket and not attempted
here, because it re-hashes every path once and needs its own record.
