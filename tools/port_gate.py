#!/usr/bin/env python3
"""port_gate -- mechanical drift gate for the SWF -> cell-package port.

Enforces the parts of patches/baselines/PATCH-0001..0006 that are decidable from
repository state alone. Anything that cannot be decided yet is printed as
NOT-ENFORCED with an owner and a blocking dependency. It is never reported as a pass.

Usage:
    python tools/port_gate.py check        # verify this repository
    python tools/port_gate.py selftest     # prove the gate refuses bad states

Exit status: 0 = every decidable check passed, 1 = at least one failed,
2 = the gate could not run (missing configuration).
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
from datetime import date
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent.parent

BASELINE_COUNT = 6
BLOCKS = [f"H{n}" for n in range(1, 11)]
STATUS_TOKENS = ("present", "not-implemented", "blocked")
SHELL_GATE_STATES = ("held", "absent", "refused")

LEDGER_REQUIRED = (
    "id",
    "timestamp",
    "block",
    "ticket",
    "artifact",
    "exit_test",
    "evidence",
    "status",
    "shell_gate_state",
)

GOVERNANCE_FILES = (
    "AGENTS.md",
    ".gitignore",
    "docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md",
    "docs/PORT-BASELINE.md",
    "docs/PACKAGE-OWNERSHIP.md",
    "docs/SCOPE-ACCOUNTING.md",
    "tickets/TEMPLATE.md",
    ".github/ISSUE_TEMPLATE/ticket.md",
    ".github/pull_request_template.md",
    ".github/workflows/port-gate.yml",
    "ledger/ledger.jsonl",
    "config/output-roots.json",
    "config/format-allowlist.json",
    "tools/pre-commit",
)

# Invariants the baseline patches require but this tool cannot decide from repository
# state. Reported on every run so an unenforced rule can never be mistaken for a pass.
GUARD_NOTE = "guard: not run"

NOT_ENFORCED = (
    {
        "check": "source_access_isolation",
        "patch": "PATCH-0003",
        "owner": "shell",
        "blockedBy": "filesystem/workspace AC isolation lives outside the repository",
        "note": "Git exclusion is checked; exclusion of inputs/ and shell-private/ from "
        "agent mounts and process permissions is not verifiable here.",
    },
    {
        "check": "duplicate_package_reader",
        "patch": "PATCH-0005",
        "owner": "format owner (H2 present)",
        "blockedBy": "this repository has no code tree or crate manifest yet",
        "note": "The authoritative reader is named in docs/PACKAGE-OWNERSHIP.md. A second "
        "reader cannot be detected here until this repository has code to scan.",
    },
    {
        "check": "alternate_surface_or_runtime",
        "patch": "PATCH-0005",
        "owner": "implementation agent (H9)",
        "blockedBy": "no client crate",
        "note": "Single surface identity is a code invariant; no code tree exists yet.",
    },
    {
        "check": "mcp_prohibited_operations",
        "patch": "PATCH-0006",
        "owner": "shell (H6)",
        "blockedBy": "no MCP implementation",
        "note": "Prohibited-operation scan is a guard until an MCP schema exists.",
    },
    {
        "check": "branch_and_pr_review",
        "patch": "PATCH-0001, PATCH-0002",
        "owner": "repository owner",
        "blockedBy": "no configured remote or review record",
        "note": "Commit-to-ticket-to-ledger references are checked; human review is not.",
    },
    {
        "check": "ledger_commit_hash_binding",
        "patch": "PATCH-0001",
        "owner": "repository owner",
        "blockedBy": "ledger entries reference tickets, not commit hashes",
        "note": "A ledger entry cannot be bound to a hash it is committed alongside.",
    },
    {
        "check": "client_witness",
        "patch": "PATCH-0005",
        "owner": "shell/test harness",
        "blockedBy": "no client, no shell-declared tests",
        "note": "No repository-local test can witness a mounted client surface.",
    },
)


# --------------------------------------------------------------------------- helpers


class GateError(RuntimeError):
    """The gate itself cannot run; this is not a repository verdict."""


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace")


def git(root: Path, *args: str) -> subprocess.CompletedProcess:
    proc = subprocess.run(
        ["git", *args],
        cwd=str(root),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    if proc.returncode != 0:
        raise GateError(f"git {' '.join(args)} failed in {root}: {proc.stderr.strip()}")
    return proc


def is_repo(root: Path) -> bool:
    return (root / ".git").exists()


def ticket_exists(root: Path, ticket_id: str) -> bool:
    return any((root / "tickets").glob(f"{ticket_id}-*.md")) or (
        root / "tickets" / f"{ticket_id}.md"
    ).is_file()


def ledger_ids(root: Path) -> set[str]:
    path = root / "ledger" / "ledger.jsonl"
    ids: set[str] = set()
    if not path.is_file():
        return ids
    for line in read_text(path).splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            record = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(record, dict) and isinstance(record.get("id"), str):
            ids.add(record["id"])
    return ids


def remove_tree(path: Path) -> None:
    """Remove a directory tree, clearing the read-only flag Git sets on objects (Windows)."""
    if not path.exists():
        return

    def retry(function, target, _exc) -> None:
        try:
            os.chmod(target, 0o700)
            function(target)
        except OSError:
            pass

    if sys.version_info >= (3, 12):
        shutil.rmtree(path, onexc=retry)
    else:  # pragma: no cover - older interpreters
        shutil.rmtree(path, onerror=lambda f, t, e: retry(f, t, e))


def section_after(text: str, heading_pattern: str) -> str | None:
    match = re.search(heading_pattern, text, re.M)
    if not match:
        return None
    rest = text[match.end() :]
    following = re.search(r"^### H\d+\b", rest, re.M)
    return rest[: following.start()] if following else rest


# --------------------------------------------------------------------------- checks


def check_baselines_present(root: Path) -> list[str]:
    """PATCH-0001..0006 exist, with the rule and its mechanical evidence."""
    problems: list[str] = []
    directory = root / "patches" / "baselines"
    if not directory.is_dir():
        return ["patches/baselines/ is missing"]
    names = sorted(p.name for p in directory.glob("PATCH-*.md"))
    for number in range(1, BASELINE_COUNT + 1):
        prefix = f"PATCH-{number:04d}"
        matches = [n for n in names if n.startswith(prefix)]
        if not matches:
            problems.append(f"missing baseline patch {prefix}-*.md")
        for name in matches:
            text = read_text(directory / name)
            for section in ("## Required rule", "## Mechanical evidence"):
                if section not in text:
                    problems.append(f"{name}: missing section {section!r}")
    if len(names) != BASELINE_COUNT:
        problems.append(
            f"expected {BASELINE_COUNT} baseline patches, found {len(names)}: "
            + ", ".join(names)
        )
    return problems


def check_governance_files(root: Path) -> list[str]:
    """Templates, ledger, baseline record and policy files are usable."""
    return [f"missing {rel}" for rel in GOVERNANCE_FILES if not (root / rel).is_file()]


def check_source_exclusion(root: Path) -> list[str]:
    """No movie is tracked, and the ignore rules that keep one out are present."""
    problems: list[str] = []
    ignore_file = root / ".gitignore"
    if not ignore_file.is_file():
        return [".gitignore is missing"]
    ignore_text = read_text(ignore_file)
    for rule in ("*.swf", "*.sfw", "inputs/", "shell-private/", "captures/"):
        if rule not in ignore_text:
            problems.append(f".gitignore is missing exclusion rule {rule!r}")
    if not is_repo(root):
        problems.append("not a git repository: tracked-file check cannot run")
        return problems
    for path in git(root, "ls-files", "-z").stdout.split("\0"):
        if path.lower().endswith((".swf", ".sfw")):
            problems.append(f"source movie is tracked in the index: {path}")
    return problems


def check_output_roots(root: Path) -> list[str]:
    """Every declared output destination resolves inside the project root."""
    problems: list[str] = []
    config_path = root / "config" / "output-roots.json"
    if not config_path.is_file():
        return ["config/output-roots.json is missing"]
    try:
        config = json.loads(read_text(config_path))
    except json.JSONDecodeError as exc:
        return [f"config/output-roots.json is not valid JSON: {exc}"]

    entries = config.get("outputRoots")
    if not isinstance(entries, list) or not entries:
        return ["config/output-roots.json declares no outputRoots"]

    root_real = os.path.realpath(root)
    ignore_file = root / ".gitignore"
    ignore_text = read_text(ignore_file) if ignore_file.is_file() else ""
    declared_classes: set[str] = set()

    for entry in entries:
        if not isinstance(entry, dict):
            problems.append("outputRoots entry is not an object")
            continue
        path = entry.get("path")
        klass = entry.get("class")
        access = entry.get("agentAccess")
        if not isinstance(path, str) or not path:
            problems.append("outputRoots entry is missing a path")
            continue
        if not klass or not access:
            problems.append(f"{path}: missing class or agentAccess annotation")
        else:
            declared_classes.add(klass)
        if os.path.isabs(path) or re.match(r"^[A-Za-z]:", path):
            problems.append(f"{path}: absolute output path refused")
            continue
        resolved = os.path.realpath(os.path.join(root, path))
        try:
            common = os.path.commonpath([root_real, resolved])
        except ValueError:
            problems.append(f"{path}: output path escapes the project root")
            continue
        if os.path.normcase(common) != os.path.normcase(root_real):
            problems.append(f"{path}: output path escapes the project root")
            continue
        if entry.get("gitIgnored"):
            segment = path.split("/")[0] + "/"
            if segment not in ignore_text:
                problems.append(f"{path}: declared gitIgnored but {segment!r} is not in .gitignore")

    if "shell-input" not in declared_classes:
        problems.append("no shell-input output root is declared for the supplied input")
    if "shell-private" not in declared_classes:
        problems.append("no shell-private output root is declared")
    return problems


def check_ledger_valid(root: Path) -> list[str]:
    """Ledger entries are complete, unique, ticket-linked, and agent-safe."""
    problems: list[str] = []
    path = root / "ledger" / "ledger.jsonl"
    if not path.is_file():
        return ["ledger/ledger.jsonl is missing"]
    seen: set[str] = set()
    for lineno, line in enumerate(read_text(path).splitlines(), start=1):
        line = line.strip()
        if not line:
            continue
        try:
            record = json.loads(line)
        except json.JSONDecodeError as exc:
            problems.append(f"ledger line {lineno}: invalid JSON ({exc})")
            continue
        if not isinstance(record, dict):
            problems.append(f"ledger line {lineno}: not an object")
            continue
        missing = [key for key in LEDGER_REQUIRED if key not in record]
        if missing:
            problems.append(f"ledger line {lineno}: missing {', '.join(missing)}")
        entry_id = record.get("id")
        if not isinstance(entry_id, str) or not re.fullmatch(r"LEDGER-\d{4}", entry_id):
            problems.append(f"ledger line {lineno}: id is not LEDGER-NNNN")
        elif entry_id in seen:
            problems.append(f"ledger line {lineno}: duplicate id {entry_id}")
        else:
            seen.add(entry_id)
        status = record.get("status")
        if status is not None and status not in STATUS_TOKENS:
            problems.append(
                f"ledger line {lineno}: status {status!r} is not one of {STATUS_TOKENS}"
            )
        gate_state = record.get("shell_gate_state")
        if isinstance(gate_state, str) and gate_state in SHELL_GATE_STATES:
            problems.append(
                f"ledger line {lineno}: an agent may not assign the shell gate state "
                f"{gate_state!r}"
            )
        ticket = record.get("ticket")
        if isinstance(ticket, str) and re.fullmatch(r"TICKET-\d{4}", ticket):
            if not ticket_exists(root, ticket):
                problems.append(f"ledger line {lineno}: unknown ticket {ticket}")
        block = record.get("block")
        if isinstance(block, str) and block not in BLOCKS:
            problems.append(f"ledger line {lineno}: block {block!r} is not H1..H10")
    if not seen:
        problems.append("ledger has no entries")
    return problems


def check_commit_refs(root: Path) -> list[str]:
    """Every commit resolves to an existing ticket and an existing ledger entry."""
    if not is_repo(root):
        return ["not a git repository: commit reference check cannot run"]
    problems: list[str] = []
    known_ledger = ledger_ids(root)
    log = git(root, "log", "--format=%H%x1f%s").stdout
    commits = [line for line in log.splitlines() if line.strip()]
    if not commits:
        return ["repository has no commits: the baseline commit is missing"]
    for line in commits:
        sha, _, subject = line.partition("\x1f")
        short = sha[:8]
        ticket = re.search(r"TICKET-\d{4}", subject)
        ledger = re.search(r"LEDGER-\d{4}", subject)
        if not ticket:
            problems.append(f"{short}: subject has no TICKET-NNNN reference")
        elif not ticket_exists(root, ticket.group(0)):
            problems.append(f"{short}: references unknown ticket {ticket.group(0)}")
        if not ledger:
            problems.append(f"{short}: subject has no LEDGER-NNNN reference")
        elif ledger.group(0) not in known_ledger:
            problems.append(f"{short}: references unknown ledger entry {ledger.group(0)}")
    return problems


def check_baseline_blocks(root: Path) -> list[str]:
    """Every H1..H10 block names its artifact, exit test, and refused work."""
    path = root / "docs" / "PORT-BASELINE.md"
    if not path.is_file():
        return ["docs/PORT-BASELINE.md is missing"]
    text = read_text(path)
    problems: list[str] = []
    for block in BLOCKS:
        body = section_after(text, rf"^### {block}\b.*$")
        if body is None:
            problems.append(f"{block}: missing from PORT-BASELINE.md")
            continue
        for field in ("Artifact:", "Exit test:", "Refuses:"):
            if field not in body:
                problems.append(f"{block}: missing {field}")
    return problems


def check_scope_accounting(root: Path) -> list[str]:
    """The scope accounting covers every block and no gate state is agent-assigned."""
    path = root / "docs" / "SCOPE-ACCOUNTING.md"
    if not path.is_file():
        return ["docs/SCOPE-ACCOUNTING.md is missing"]
    text = read_text(path)
    problems: list[str] = []
    rows = dict(
        re.findall(
            r"^\|\s*(H\d{1,2})\s*\|\s*(present|not-implemented|blocked)\s*\|",
            text,
            re.M,
        )
    )
    for block in BLOCKS:
        if block not in rows:
            problems.append(f"{block}: no status row in SCOPE-ACCOUNTING.md")
    if "shell gate state: unassigned" not in text.lower():
        problems.append("SCOPE-ACCOUNTING.md must record that shell gate states are unassigned")
    return problems


def check_format_magics(root: Path) -> list[str]:
    """Guard: only the authoritative package magic appears in code."""
    config_path = root / "config" / "format-allowlist.json"
    if not config_path.is_file():
        return ["config/format-allowlist.json is missing"]
    try:
        config = json.loads(read_text(config_path))
    except json.JSONDecodeError as exc:
        return [f"config/format-allowlist.json is not valid JSON: {exc}"]

    allowed = set(config.get("allowedPackageMagics", []))
    forbidden = [str(token) for token in config.get("forbiddenIdentifiers", [])]
    pattern = re.compile(str(config.get("magicPattern", r"[A-Z0-9]{2,10}PKG[0-9]{3}")))
    directories = [
        root / str(name) for name in config.get("scannedDirectories", []) if str(name)
    ]
    extensions = set(config.get("scannedExtensions", []))

    problems: list[str] = []
    scanned = 0
    for directory in directories:
        if not directory.is_dir():
            continue
        for file in directory.rglob("*"):
            if not file.is_file() or file.suffix not in extensions:
                continue
            scanned += 1
            text = read_text(file)
            rel = str(file.relative_to(root)).replace(os.sep, "/")
            for magic in pattern.findall(text):
                if magic not in allowed:
                    problems.append(f"{rel}: unowned package magic {magic!r} (second format)")
            for token in forbidden:
                if token in text:
                    problems.append(f"{rel}: forbidden alternate format identifier {token!r}")

    global GUARD_NOTE
    if scanned == 0:
        GUARD_NOTE = (
            "guard: no code tree present, 0 files scanned "
            "(NOT evidence of enforcement over code)"
        )
    else:
        GUARD_NOTE = f"guard: {scanned} code files scanned"
    return problems


CHECKS = (
    ("baselines_present", check_baselines_present, "PATCH-0001"),
    ("governance_files", check_governance_files, "PATCH-0001, PATCH-0002"),
    ("source_exclusion", check_source_exclusion, "PATCH-0003"),
    ("output_roots", check_output_roots, "PATCH-0004"),
    ("format_magics", check_format_magics, "PATCH-0005"),
    ("ledger_valid", check_ledger_valid, "PATCH-0001, PATCH-0002"),
    ("commit_refs", check_commit_refs, "PATCH-0001, PATCH-0002"),
    ("baseline_blocks", check_baseline_blocks, "PATCH-0001"),
    ("scope_accounting", check_scope_accounting, "PATCH-0001"),
)


# --------------------------------------------------------------------------- commands


def run_checks(root: Path) -> list[tuple[str, str, list[str]]]:
    results = []
    for name, function, patch in CHECKS:
        try:
            problems = function(root)
        except GateError as exc:
            problems = [f"gate error: {exc}"]
        results.append((name, patch, problems))
    return results


def report(results: list[tuple[str, str, list[str]]]) -> int:
    failed = 0
    print(f"port_gate: {len(results)} checks against {PROJECT_ROOT}")
    for name, patch, problems in results:
        if problems:
            failed += 1
            print(f"  FAIL {name:<20} [{patch}]")
            for problem in problems:
                print(f"       - {problem}")
        else:
            print(f"  PASS {name:<20} [{patch}]")
            if name == "format_magics":
                print(f"       {GUARD_NOTE}")
    print()
    print(f"NOT-ENFORCED ({len(NOT_ENFORCED)}) - required by the baselines, not decidable here:")
    for item in NOT_ENFORCED:
        print(f"  - {item['check']} [{item['patch']}]")
        print(f"      owner: {item['owner']} | blocked by: {item['blockedBy']}")
        print(f"      {item['note']}")
    print()
    if failed:
        print(f"RESULT: FAIL ({failed} of {len(results)} checks)")
        return 1
    print(f"RESULT: PASS ({len(results)} checks, {len(NOT_ENFORCED)} not enforceable)")
    return 0


def command_check(root: Path) -> int:
    return report(run_checks(root))


# --------------------------------------------------------------------------- selftest

FIXTURE_PATHS = (
    "AGENTS.md",
    ".gitignore",
    ".github",
    "config",
    "docs/PACKAGE-OWNERSHIP.md",
    "docs/PORT-BASELINE.md",
    "docs/SCOPE-ACCOUNTING.md",
    "docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md",
    "patches",
    "tickets",
    "ledger",
    "tools",
)

FIXTURE_COMMIT = "TICKET-0001 LEDGER-0001 fixture baseline"


def build_fixture(project_root: Path, destination: Path) -> None:
    destination.mkdir(parents=True, exist_ok=True)
    for rel in FIXTURE_PATHS:
        source = project_root / rel
        target = destination / rel
        if not source.exists():
            raise GateError(f"cannot build fixture: {rel} is missing from the project")
        target.parent.mkdir(parents=True, exist_ok=True)
        if source.is_dir():
            shutil.copytree(source, target)
        else:
            shutil.copy2(source, target)
    git(destination, "init", "-q")
    git(destination, "add", "-A")
    git(
        destination,
        "-c",
        "user.name=port-gate",
        "-c",
        "user.email=port-gate@example.invalid",
        "commit",
        "-q",
        "-m",
        FIXTURE_COMMIT,
    )


def fixture_commit(root: Path, message: str, force_add: tuple[str, ...] = ()) -> None:
    git(root, "add", "-A")
    if force_add:
        git(root, "add", "-f", *force_add)
    git(
        root,
        "-c",
        "user.name=port-gate",
        "-c",
        "user.email=port-gate@example.invalid",
        "commit",
        "-q",
        "-m",
        message,
    )


def mutate_tracked_movie(root: Path) -> None:
    movie = root / "inputs" / "selftest-placeholder.swf"
    movie.parent.mkdir(parents=True, exist_ok=True)
    movie.write_text("synthetic placeholder, not a movie\n", encoding="utf-8")
    fixture_commit(root, "TICKET-0001 LEDGER-0001 force-add a movie", force_add=("inputs",))


def mutate_output_escape(root: Path) -> None:
    path = root / "config" / "output-roots.json"
    config = json.loads(read_text(path))
    config["outputRoots"].append(
        {"path": "../outside-scratch/", "class": "redacted", "agentAccess": "read"}
    )
    path.write_text(json.dumps(config, indent=2) + "\n", encoding="utf-8")


def mutate_missing_baseline(root: Path) -> None:
    for candidate in (root / "patches" / "baselines").glob("PATCH-0004-*.md"):
        candidate.unlink()


def mutate_invalid_ledger(root: Path) -> None:
    path = root / "ledger" / "ledger.jsonl"
    entry = {
        "id": "LEDGER-0001",
        "timestamp": date.today().isoformat(),
        "block": "H1",
        "ticket": "TICKET-0001",
        "artifact": "fixture",
        "evidence": "none",
        "status": "present",
        "shell_gate_state": "unassigned",
    }
    path.write_text(json.dumps(entry) + "\n", encoding="utf-8")


def mutate_commit_without_ticket(root: Path) -> None:
    note = root / "docs" / "drift-note.md"
    note.write_text("unreferenced change\n", encoding="utf-8")
    fixture_commit(root, "scaffold tweaks")


def mutate_block_without_exit_test(root: Path) -> None:
    path = root / "docs" / "PORT-BASELINE.md"
    text = read_text(path)
    body = section_after(text, r"^### H7\b.*$") or ""
    broken = body.replace("- Exit test:", "- (removed)")
    path.write_text(text.replace(body, broken), encoding="utf-8")


def mutate_agent_gate_state(root: Path) -> None:
    path = root / "ledger" / "ledger.jsonl"
    lines = [line for line in read_text(path).splitlines() if line.strip()]
    records = [json.loads(line) for line in lines]
    records[0]["shell_gate_state"] = "held"
    path.write_text(
        "\n".join(json.dumps(record) for record in records) + "\n", encoding="utf-8"
    )


SELFTEST_SCENARIOS = (
    ("valid_fixture", None, None),
    ("tracked_movie_refused", mutate_tracked_movie, "source_exclusion"),
    ("output_escape_refused", mutate_output_escape, "output_roots"),
    ("missing_baseline_refused", mutate_missing_baseline, "baselines_present"),
    ("invalid_ledger_refused", mutate_invalid_ledger, "ledger_valid"),
    ("unticketed_commit_refused", mutate_commit_without_ticket, "commit_refs"),
    ("block_without_exit_test_refused", mutate_block_without_exit_test, "baseline_blocks"),
    ("agent_gate_state_refused", mutate_agent_gate_state, "ledger_valid"),
)


def command_selftest(project_root: Path) -> int:
    work = project_root / "target" / "port-gate-selftest"
    remove_tree(work)
    work.mkdir(parents=True)

    failures: list[str] = []
    records: list[dict] = []
    print(f"port_gate selftest: fixtures under {work}")

    try:
        for name, mutate, expected_failure in SELFTEST_SCENARIOS:
            fixture = work / name
            problems: list[str] = []
            failed: set[str] = set()
            try:
                build_fixture(project_root, fixture)
                if mutate is not None:
                    mutate(fixture)
                results = run_checks(fixture)
                failed = {check for check, _patch, errs in results if errs}
                problems = [p for _check, _patch, errs in results for p in errs]
            except GateError as exc:
                failed = {"gate_error"}
                problems = [f"gate error: {exc}"]
            ok = (not failed) if expected_failure is None else (expected_failure in failed)
            if not ok:
                if expected_failure is None:
                    failures.append(f"{name}: expected a clean fixture, failed {sorted(failed)}")
                else:
                    failures.append(
                        f"{name}: expected {expected_failure!r} to fail, failing checks "
                        f"were {sorted(failed) or 'none'}"
                    )
            records.append(
                {
                    "scenario": name,
                    "expected_failure": expected_failure,
                    "failing_checks": sorted(failed),
                    "problems": problems[:10],
                    "pass": ok,
                }
            )
            verdict = "PASS" if ok else "FAIL"
            expected = expected_failure or "no failure"
            print(f"  {verdict} {name:<34} expected: {expected}")
            if expected_failure is not None or not ok:
                for problem in problems[:6]:
                    print(f"       - {problem}")
    finally:
        try:
            remove_tree(work)
        except OSError:
            print(f"  warning: could not fully remove {work}")

    record = {
        "tool": "tools/port_gate.py selftest",
        "timestamp": date.today().isoformat(),
        "python": sys.version.split()[0],
        "scenarios": len(records),
        "passed": sum(1 for item in records if item["pass"]),
        "results": records,
        "exit_status": 0 if not failures else 1,
    }
    out = project_root / "artifacts" / "redacted-test-results" / "port-gate-selftest.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")

    print()
    for failure in failures:
        print(f"  - {failure}")
    print(f"record: {out.relative_to(project_root)}")
    if failures:
        print(f"RESULT: FAIL ({len(failures)} scenarios)")
        return 1
    print(f"RESULT: PASS ({record['passed']} of {record['scenarios']} scenarios)")
    return 0


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description="port drift gate")
    parser.add_argument(
        "command",
        nargs="?",
        default="check",
        choices=("check", "selftest"),
        help="check = verify this repository, selftest = prove the gate refuses bad states",
    )
    parser.add_argument(
        "--root",
        default=str(PROJECT_ROOT),
        help="repository root to check (default: this checkout)",
    )
    args = parser.parse_args(argv)
    root = Path(args.root).resolve()
    try:
        if args.command == "selftest":
            return command_selftest(root)
        return command_check(root)
    except GateError as exc:
        print(f"port_gate: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
