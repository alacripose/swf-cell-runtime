#!/usr/bin/env python3
"""**Per-unit health, with a ratchet.** One command a test turn runs.

    python tools/health.py            # check, ratchet, write the unit's report
    python tools/health.py --brief    # one line, for a hook or a wrapper

What it does:

1. Runs `tools/port_gate.py check` and reads its `RESULT:` marker, its failing checks and the checks
   it reports as not enforceable.
2. Compares the failures with a **durable ratchet** (`artifacts/health/state.json`): a check that has
   ever been `held` on this project may not fail again. A check that fails while it has never been
   held is a **debt**, recorded with an owner and the unit that opened it, and it does not fail the
   run — but a debt that survives past its unit is reported as still open, never dropped.
3. Writes the unit's report: `reports/health/<date>-<unit>.md` and `.json`, named for the work unit
   (the newest ledger id) and the day, so the record is per unit and per day rather than one rolling
   file that loses which turn produced what.
4. Exits non-zero on a ratchet regression (exit 2), a gate failure that is not a recorded debt (exit
   1), or an unreadable state file (exit 5). Every path prints a `RESULT:` line.

The state is written atomically (temporary file beside the final one, then `os.replace`), so a
disconnected or interrupted run cannot leave a half-written ratchet behind.

Debt policy, stated rather than assumed:

- `selftest_record` is the one **evidence debt** this project allows to fail between runs: the record
  certifies a tree digest, so any edit makes it stale, and the repair is one complete `selftest` run,
  not a rewritten digest. It is recorded as an open debt with its owner and age and never counted as
  held.
- Every other failure is a regression if the check has ever been held, and a new debt if it has not.
"""

import json
import os
import pathlib
import subprocess
import sys
from datetime import datetime, timezone

ROOT = pathlib.Path(__file__).resolve().parent.parent
STATE = ROOT / "artifacts" / "health" / "state.json"
REPORTS = ROOT / "reports" / "health"
PROPOSALS = ROOT / "artifacts" / "health" / "proposals.jsonl"
# The projects this ratchet observes. Only the current repository unless the user has written an
# explicit list: observing another checkout is never inferred from a path.
PROJECTS = ROOT / "config" / "health-projects.json"

# Checks that may fail between runs because their evidence is regenerated per unit.
EVIDENCE_DEBTS = {"selftest_record"}


def projects() -> list[pathlib.Path]:
    """This repository, plus any checkout the user named in `config/health-projects.json`."""
    roots = [ROOT]
    if PROJECTS.is_file():
        try:
            named = json.loads(PROJECTS.read_text(encoding="utf-8")).get("projects", [])
        except (OSError, json.JSONDecodeError):
            named = []
        roots += [pathlib.Path(entry) for entry in named if isinstance(entry, str)]
    return [path for path in roots if path.is_dir()]


def git(root: pathlib.Path, *args: str) -> str:
    proc = subprocess.run(
        ["git", *args], cwd=str(root), capture_output=True, text=True
    )
    return proc.stdout if proc.returncode == 0 else ""


def turn_metrics(root: pathlib.Path) -> dict:
    """How much work this turn did: churn, timing and the tests that moved with it.

    The timestamps are the development times of the changed files, so a turn's span is observed
    rather than assumed; `changed` counts tracked and untracked entries the same way `git status`
    reports them.
    """
    status = [line for line in git(root, "status", "--porcelain").splitlines() if line.strip()]
    changed = [line[3:].strip().strip('"') for line in status]
    insertions = deletions = 0
    for line in git(root, "diff", "--numstat", "HEAD").splitlines():
        parts = line.split("\t")
        if len(parts) >= 2 and parts[0].isdigit() and parts[1].isdigit():
            insertions += int(parts[0])
            deletions += int(parts[1])
    stamps: list[float] = []
    tests = 0
    for relative in changed[:300]:
        path = root / relative
        if not path.is_file():
            continue
        stamps.append(path.stat().st_mtime)
        if path.suffix in {".rs", ".py"}:
            try:
                body = path.read_text(encoding="utf-8", errors="replace")
            except OSError:
                continue
            tests += body.count("#[test]") + body.count("def test_")
    span = round(max(stamps) - min(stamps), 1) if len(stamps) > 1 else 0.0
    return {
        "project": str(root),
        "changed": len(changed),
        "insertions": insertions,
        "deletions": deletions,
        "work_span_seconds": span,
        "tests_touched": tests,
    }


def newest_unit() -> str:
    ledger = ROOT / "ledger" / "ledger.jsonl"
    newest = "LEDGER-0000"
    if ledger.is_file():
        for line in ledger.read_text(encoding="utf-8").splitlines():
            try:
                row = json.loads(line)
            except json.JSONDecodeError:
                continue
            identifier = str(row.get("id", ""))
            if identifier.startswith("LEDGER-") and identifier > newest:
                newest = identifier
    return newest


def read_state() -> dict:
    if not STATE.is_file():
        return {"version": 1, "runs": [], "ever_held": [], "debts": {}}
    try:
        state = json.loads(STATE.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise SystemExit(f"RESULT: REFUSED (health-state-unreadable; {STATE}: {exc})")
    state.setdefault("version", 1)
    state.setdefault("runs", [])
    state.setdefault("ever_held", [])
    state.setdefault("debts", {})
    return state


def write_state(state: dict) -> None:
    STATE.parent.mkdir(parents=True, exist_ok=True)
    temporary = STATE.with_name(STATE.name + ".tmp")
    temporary.write_text(json.dumps(state, indent=2) + "\n", encoding="utf-8")
    os.replace(temporary, STATE)


def gate() -> tuple[str, list[str], list[str], list[str]]:
    """The gate's marker, its failing checks, its held checks, and its not-enforced checks."""
    proc = subprocess.run(
        [sys.executable, str(ROOT / "tools" / "port_gate.py"), "check"],
        capture_output=True,
        text=True,
        cwd=str(ROOT),
    )
    body = (proc.stdout or "") + (proc.stderr or "")
    failing = [
        line.split()[1]
        for line in body.splitlines()
        if line.strip().startswith("FAIL ") and len(line.split()) > 1
    ]
    absent = [
        line.strip()[2:].split()[0]
        for line in body.splitlines()
        if line.strip().startswith("- ") and "[" in line
    ]
    held = [
        line.split()[1]
        for line in body.splitlines()
        if line.strip().startswith("PASS ") and len(line.split()) > 1
    ]
    marker = next(
        (line.strip() for line in body.splitlines() if line.strip().startswith("RESULT:")),
        "RESULT: ABSENT (check printed no RESULT marker)",
    )
    return marker, failing, held, absent


def proposals_for(metrics: dict, failing: list[str], unit: str, new_rows: int) -> list[dict]:
    """What this turn's own measurements say should be a rule, a change or a refusal.

    Each proposal is one rule with the evidence that produced it, so a session's lessons become an
    `AGENTS.md` candidate instead of a memory that ends with the conversation.
    """
    found: list[dict] = []

    def add(identifier: str, rule: str, evidence: str) -> None:
        found.append({"id": identifier, "unit": unit, "rule": rule, "evidence": evidence})

    if metrics["changed"] >= 40 or metrics["insertions"] >= 4_000:
        add(
            "commit-budget",
            "Split a unit whose diff exceeds 40 files or 4,000 insertions into two units, not one "
            "larger commit (AGENTS.md §11).",
            f"this turn changed {metrics['changed']} files with {metrics['insertions']} insertions",
        )
    if metrics["changed"] and metrics["tests_touched"] == 0:
        add(
            "untested-change",
            "A unit that changes behaviour states the test that proves it, or states that it has none "
            "and why.",
            f"{metrics['changed']} files changed and no test function moved with them",
        )
    if new_rows == 0:
        add(
            "unit-without-ledger-row",
            "A unit with no ledger row did not happen: write the row and the ticket with the change "
            "(AGENTS.md §9).",
            f"no ledger row was added since the previous health run for {unit}",
        )
    if "selftest_record" in failing:
        add(
            "evidence-debt",
            "A stale selftest record is an open evidence debt: one complete selftest on the current "
            "tree repairs it, and a rewritten digest does not.",
            "the record does not certify this tree's fixture digest",
        )
    return found


def main(argv: list[str]) -> int:
    brief = "--brief" in argv
    unit = newest_unit()
    now = datetime.now(timezone.utc).replace(microsecond=0)
    run_id = f"{now.date().isoformat()}-{unit}"

    marker, failing, held, absent = gate()
    state = read_state()
    metrics = [turn_metrics(path) for path in projects()]
    ledger_path = ROOT / "ledger" / "ledger.jsonl"
    ledger_rows = len(
        [line for line in ledger_path.read_text(encoding="utf-8").splitlines() if line.strip()]
    )
    previous_rows = int(state.get("ledger_rows", 0))
    new_rows = ledger_rows - previous_rows if previous_rows else ledger_rows
    proposals = proposals_for(metrics[0], failing, unit, new_rows)
    ever_held = set(state["ever_held"])
    debts = state["debts"]

    regressions: list[str] = []
    new_debts: list[str] = []
    for check in failing:
        if check in EVIDENCE_DEBTS:
            debts.setdefault(check, {"owner": "agent", "opened": run_id, "policy": "evidence-debt"})
            continue
        if check in ever_held:
            regressions.append(check)
        else:
            new_debts.append(check)
            debts.setdefault(check, {"owner": "agent", "opened": run_id, "policy": "new-debt"})

    # A debt whose check is held again is repaired, so it closes here: carrying it would be a false
    # open debt, and the ratchet states what is true now rather than collecting history.
    for check in list(debts):
        if check not in failing:
            debts.pop(check, None)

    # The ratchet only tightens: a check that passed once is held forever, and a later failure is a
    # regression rather than a fresh start.
    state["ever_held"] = sorted(ever_held | set(held))
    state["ledger_rows"] = ledger_rows
    state["metrics"] = metrics
    state["proposals"] = (state.get("proposals", []) + proposals)[-80:]
    if proposals:
        PROPOSALS.parent.mkdir(parents=True, exist_ok=True)
        seen: set[str] = set()
        if PROPOSALS.is_file():
            for line in PROPOSALS.read_text(encoding="utf-8").splitlines():
                try:
                    row = json.loads(line)
                except json.JSONDecodeError:
                    continue
                seen.add(f"{row.get('id')}@{row.get('unit')}")
        with PROPOSALS.open("a", encoding="utf-8", newline="\n") as handle:
            for proposal in proposals:
                key = f"{proposal['id']}@{proposal['unit']}"
                if key not in seen:
                    handle.write(json.dumps(proposal, ensure_ascii=False) + "\n")
                    seen.add(key)

    if regressions:
        result = f"RESULT: FAIL (ratchet: {len(regressions)} held check(s) regressed: {', '.join(regressions)})"
        code = 2
    elif not marker.startswith("RESULT: PASS") and not (failing and set(failing) <= EVIDENCE_DEBTS):
        result = f"RESULT: FAIL (gate: {marker})"
        code = 1
    else:
        result = f"RESULT: PASS ({unit} held; {len(debts)} debt(s) open, {len(absent)} not enforceable)"
        code = 0

    state["runs"] = (state["runs"] + [
        {
            "run": run_id,
            "unit": unit,
            "at": now.isoformat(),
            "gate": marker,
            "failing": failing,
            "regressions": regressions,
            "new_debts": new_debts,
            "not_enforced": absent,
            "result": result,
        }
    ])[-40:]
    state["debts"] = debts
    write_state(state)

    REPORTS.mkdir(parents=True, exist_ok=True)
    report = REPORTS / f"{run_id}.md"
    lines = [
        f"# Health — {unit} — {now.date().isoformat()}",
        "",
        f"- gate: `{marker}`",
        f"- failing: {', '.join(failing) if failing else 'none'}",
        f"- regressions: {', '.join(regressions) if regressions else 'none'}",
        f"- new debts: {', '.join(new_debts) if new_debts else 'none'}",
        f"- not enforceable: {len(absent)} ({', '.join(absent) if absent else 'none'})",
        f"- result: `{result}`",
        "",
        "## Open debts",
        "",
    ]
    if debts:
        for check, debt in sorted(debts.items()):
            lines.append(
                f"- `{check}` opened {debt.get('opened')} by {debt.get('owner')} "
                f"({debt.get('policy')})"
            )
    else:
        lines.append("- none")
    lines += ["", "## Work measured this turn", ""]
    for metric in metrics:
        lines.append(
            f"- `{metric['project']}`: {metric['changed']} files changed, "
            f"+{metric['insertions']}/-{metric['deletions']}, observed development span "
            f"{metric['work_span_seconds']} s, tests touched {metric['tests_touched']}"
        )
    lines += ["", "## Proposals for AGENTS.md", ""]
    if proposals:
        for proposal in proposals:
            lines.append(
                f"- `{proposal['id']}`: {proposal['rule']} — evidence: {proposal['evidence']}"
            )
    else:
        lines.append("- none this turn")
    lines += [
        "",
        "## What this is not",
        "",
        "A gate PASS is not a client witness and not a playable game (`AGENTS.md` §7). This report "
        "states what the repository's checks held on this unit's tree; the window, the mount and the "
        "movie remain the shell's and the user's evidence.",
        "",
    ]
    report.write_text("\n".join(lines), encoding="utf-8")
    (REPORTS / f"{run_id}.json").write_text(
        json.dumps(
            {
                "unit": unit,
                "at": now.isoformat(),
                "gate": marker,
                "failing": failing,
                "regressions": regressions,
                "new_debts": new_debts,
                "not_enforced": absent,
                "metrics": metrics,
                "proposals": proposals,
                "debts": debts,
                "result": result,
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )

    if brief:
        print(f"{run_id}: {result}")
    else:
        print(f"health: {run_id} at {now.isoformat()}")
        print(f"  {marker}")
        if failing:
            print(f"  failing: {', '.join(failing)}")
        if regressions:
            print(f"  regressions: {', '.join(regressions)}")
        for metric in metrics:
            print(
                f"  work: {metric['changed']} files, +{metric['insertions']}/-{metric['deletions']}, "
                f"span {metric['work_span_seconds']} s, tests touched {metric['tests_touched']}"
            )
        for proposal in proposals:
            print(f"  proposal {proposal['id']}: {proposal['rule']}")
        print(f"  report: {report.relative_to(ROOT)}")
        print(result, flush=True)
    return code


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
