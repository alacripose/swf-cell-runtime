# TICKET-0093 — commit the whole `docs/` tree, including the secure-shell audit material

## Problem

Every `docs/` file the port depends on is tracked, but the shell-side audit material that was
produced beside them is not: `docs/secure-shell-audit/` (nine files, 3 702 lines) sits untracked, so a
clone of the public repository is missing the pages the plan and the rulings are synthesised from.
The user asked for the entire `docs/` folder to be committed to the public repository.

## Requirement

- `docs/secure-shell-audit/` — the cell-schema ruling, the two civilian-expectations pages, the plan
  resynthesis, the secure-shell synthesis, and the scan tool with its state, stream and log — added to
  version control as they stand.
- No file is edited, redacted or moved to make the commit smaller: the material is committed because
  the user asked for it, and a partial commit would be the same absence at a smaller size.
- The commit cites this ticket and `LEDGER-0097`, because the gate refuses a subject without both.
- Nothing is added for the files that are already tracked: `docs/` outside this directory was clean.

## Acceptance evidence

- `git ls-files docs | wc -l` counts 31 tracked files (22 already tracked, 9 added), and
  `git status --porcelain docs` is empty.
- `python tools/port_gate.py check` exits 0 with `RESULT: PASS`.
- `python tools/health.py` exits 0 with `RESULT: PASS`.
- `git rev-parse HEAD` equals `git rev-parse origin/main` after the push.

## What this does not claim

The audit material is the shell's reading of the family repositories, not evidence that this port
renders or plays the movie. `client_witness` and `client_run_h9` stay `NOT-ENFORCED`, and no sentence
here is a claim that the game works.
