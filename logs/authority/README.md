# Authority log

Every inbound message of a session is recorded here, verbatim, one JSON object per line:

    {"logged_at": "...", "unit": "LEDGER-0092", "source": "user",
     "authority": "user", "verbatim": true, "text": "..."}

- `source`: `user`, `mission-prompter`, `system`, or `agent`.
- `authority`: `user` only when `source` is `user`; otherwise `none`.
- `verbatim`: `true` when `text` is the message as received, `false` when it is an excerpt and
  the row says so in `note`.

The mission prompter is not the user. A prompter or system line states a goal; it is recorded and
classified here and never quoted as the user's requirement. `tools/port_gate.py check` reads
these files: a promoted row is refused `authority-promoted`, and a session directory with no
recorded session is refused `authority-unlogged`.
