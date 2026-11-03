# UNIT-0061 - erratum: a commit carrying two units, and the sequencing rule that caused it

Ticket `tickets/TICKET-0061-erratum-0058-0059.md`, ledger `LEDGER-0067`, block H9.

## What was found

Commit `43463fb` is labelled `TICKET-0059 LEDGER-0065` and contains both TICKET-0058's code and
TICKET-0059's records. No commit cites `LEDGER-0064`, so that row describes real, committed work whose
commit does not name it.

## Why

`tickets/` and `ledger/` are inputs to the gate's selftest record. Writing TICKET-0058's ticket and
LEDGER-0064's row changed the fixture identity, which made the record from TICKET-0057 stale. The
pre-commit hook's `check` then answered `FAIL selftest_record` and refused the first commit; its files were
already staged, and the next plain `git commit` committed the whole index under the second unit's message.

## What changed

- The sequence is now: write ticket and ledger row, regenerate the record, then commit - and a unit never
  spans two commits.
- Commits are made with an explicit pathspec after staging, so a refused commit cannot carry its index
  into the following one.
- TICKET-0058's ticket names the commit that actually carries it instead of claiming its own.

## What was not done

History was not rewritten. The incorrect attribution is corrected in the record rather than erased from
the graph, because tidying a landed commit would remove the evidence this erratum exists to keep.

## Follow-up, named and not taken

`check` passes while a ledger row names no commit. Making `commit_refs` enforce that over the live ledger
is a gate change and needs its own ticket.
