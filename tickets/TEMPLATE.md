# TICKET-NNNN — one-line artifact or defect

- Owner: <name/role>
- Block: H1–H10
- Branch: `ticket/TICKET-NNNN-short-name`
- Ledger: LEDGER-NNNN
- Depends on: TICKET-NNNN (or "none")

## Scope (source-safe)

What this ticket changes, described without source identifiers, paths, offsets, URLs,
script names, decoded media, or inventory references.

## Acceptance test

The executable check that decides this ticket. Name the command and the assertion. A
prose claim is not an acceptance test.

## Exit test evidence

What the run produced, where the durable record is, and its digest if applicable.

## Shell-owned gating and refusal conditions

What the shell must gate before this work is accepted, and which refusal states exist for
it. The agent records the condition; the shell alone assigns the state.

## Incomplete state (must be declared, not hidden)

Every part of the named contract this ticket does not deliver, with owner and blocking
dependency. "Nothing outstanding" is a claim that needs evidence.

## Excluded from this ticket

Work deliberately not done here, so it cannot disappear by omission.

## Review

- Blocking comments: <none / list>
- Reviewer decisions: <accepted/rejected with reason>

## Closing

- Merged commit: <sha>
- Shell gate state: <assigned by shell>
- Ledger entry: LEDGER-NNNN
