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
    "docs/CELL-MODEL.md",
    "docs/CELL-SCHEMA.md",
    "docs/CELL-MEDIA-DECISION.md",
    "docs/CELL-PROGRAM-DECISION.md",
    "docs/CELL-TRANSPORT-DECISION.md",
    "docs/RULING-INDEX.md",
    "tickets/TEMPLATE.md",
    ".github/ISSUE_TEMPLATE/ticket.md",
    ".github/pull_request_template.md",
    ".github/workflows/port-gate.yml",
    "ledger/ledger.jsonl",
    "config/output-roots.json",
    "config/format-allowlist.json",
    "config/plan-errata.json",
    "config/cell-schema.json",
    "config/cell-program.json",
    "config/cell-media.json",
    "config/cell-stream.json",
    "config/ruling-index.json",
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
        "check": "alternate_surface_or_runtime",
        "patch": "PATCH-0005",
        "owner": "implementation agent (H9)",
        "blockedBy": "no client crate",
        "note": "Single surface identity is a code invariant; no code tree exists yet.",
    },
    {
        "check": "branch_and_pr_review",
        "patch": "PATCH-0001, PATCH-0002",
        "owner": "repository owner",
        "blockedBy": "no configured remote or review record",
        "note": "Commit-to-ticket-to-ledger references are checked; human review is not.",
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




ERRATUM_HEADING = re.compile(r"^> \*\*ERRATUM (E-\d+)\b", re.M)


def split_errata(text: str) -> tuple[dict[str, str], str]:
    """Return each erratum blockquote and the document with those blocks removed."""
    blocks: dict[str, str] = {}
    spans: list[tuple[int, int]] = []
    matches = list(ERRATUM_HEADING.finditer(text))
    for index, match in enumerate(matches):
        start = match.start()
        stop = matches[index + 1].start() if index + 1 < len(matches) else len(text)
        kept: list[str] = []
        consumed = 0
        for line in text[start:stop].splitlines(keepends=True):
            if not line.startswith(">"):
                break
            kept.append(line)
            consumed += len(line)
        blocks[match.group(1)] = "".join(kept)
        spans.append((start, start + consumed))
    without = text
    for start, stop in reversed(spans):
        without = without[:start] + without[stop:]
    return blocks, without


def check_plan_errata(root: Path) -> list[str]:
    """Each erratum keeps its block, its quote, its citations, and the original it supersedes."""
    manifest_path = root / "config" / "plan-errata.json"
    plan_path = root / "docs" / "SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md"
    if not manifest_path.is_file():
        return ["config/plan-errata.json is missing"]
    if not plan_path.is_file():
        return ["the plan document is missing"]
    try:
        manifest = json.loads(read_text(manifest_path))
    except json.JSONDecodeError as exc:
        return [f"config/plan-errata.json is not valid JSON: {exc}"]
    errata = manifest.get("errata")
    if not isinstance(errata, list) or not errata:
        return ["config/plan-errata.json declares no errata"]

    text = read_text(plan_path)
    blocks, without = split_errata(text)
    head = text[:4000]
    problems: list[str] = []
    for entry in errata:
        if not isinstance(entry, dict):
            problems.append("an errata entry is not an object")
            continue
        erratum = entry.get("id")
        if not isinstance(erratum, str):
            problems.append("an errata entry has no id")
            continue
        block = blocks.get(erratum)
        if block is None:
            problems.append(f"{erratum}: no '> **ERRATUM {erratum}' block in the plan")
            continue
        for label in ("**Superseded", "**Measured:**", "**Status:**"):
            if label not in block:
                problems.append(f"{erratum}: the block is missing its {label!r} part")
        for citation in entry.get("citations", []):
            if citation not in block:
                problems.append(f"{erratum}: no longer cites {citation!r}")
        for phrase in entry.get("superseded", []):
            if phrase not in block:
                problems.append(f"{erratum}: no longer quotes {phrase!r}")
            if phrase not in without:
                problems.append(
                    f"{erratum}: the original wording was removed from the plan; it must "
                    f"stay visible beside the correction ({phrase!r})"
                )
        if erratum not in head:
            problems.append(f"{erratum}: not listed in the errata index at the top of the plan")
    return problems


def sections_by_heading(text: str, pattern: str) -> dict[str, str]:
    """Split a document into the bodies of headings whose first group names them."""
    heading = re.compile(pattern, re.M)
    matches = list(heading.finditer(text))
    sections: dict[str, str] = {}
    for index, match in enumerate(matches):
        start = match.start()
        stop = matches[index + 1].start() if index + 1 < len(matches) else len(text)
        sections[match.group(1)] = text[start:stop]
    return sections


def check_cell_schema(root: Path) -> list[str]:
    """Invariants stay in the model, every schema question keeps its four parts, and no wire token."""
    manifest_path = root / "config" / "cell-schema.json"
    if not manifest_path.is_file():
        return ["config/cell-schema.json is missing"]
    try:
        manifest = json.loads(read_text(manifest_path))
    except json.JSONDecodeError as exc:
        return [f"config/cell-schema.json is not valid JSON: {exc}"]

    model_path = root / str(manifest.get("model", "docs/CELL-MODEL.md"))
    schema_path = root / str(manifest.get("schema", "docs/CELL-SCHEMA.md"))
    problems: list[str] = []
    for path in (model_path, schema_path):
        if not path.is_file():
            problems.append(f"{path.relative_to(root)} is missing")
    if problems:
        return problems

    model = read_text(model_path)
    schema = read_text(schema_path)

    for invariant in manifest.get("invariants", []):
        identifier = invariant.get("id")
        expected = invariant.get("text", "")
        if not expected or expected not in model:
            problems.append(
                f"{identifier}: the model no longer states the invariant {expected!r}"
            )
        if identifier and identifier not in schema:
            problems.append(
                f"{identifier}: the schema proposal does not account for this invariant"
            )

    questions = sections_by_heading(schema, r"^#{2,3} (Q-\d+)\b.*$")
    for question in manifest.get("questions", []):
        section = questions.get(question)
        if section is None:
            problems.append(f"{question}: no '### {question}' section in the schema proposal")
            continue
        for label in manifest.get("questionLabels", []):
            if f"**{label}:**" not in section:
                problems.append(f"{question}: missing its {label!r} part")

    marker = manifest.get("proposalMarker")
    if marker and marker.lower() not in schema.lower():
        problems.append(f"the schema document never says it is a {marker!r}")

    tokens = [str(token) for token in manifest.get("forbiddenTokens", [])]
    if tokens:
        token_pattern = re.compile(
            "|".join(
                rf"\b{re.escape(token)}\b" if token.isalnum() else re.escape(token)
                for token in tokens
            )
        )
        for path, text in ((model_path, model), (schema_path, schema)):
            found = token_pattern.search(text)
            if found:
                problems.append(
                    f"{path.relative_to(root)}: port wire token {found.group(0)!r} is not the "
                    "port's to decide"
                )

    phrases = [str(phrase).lower() for phrase in manifest.get("rasterPhrases", [])]
    exemptions = [str(word).lower() for word in manifest.get("rasterExemptions", [])]
    for path, text in ((model_path, model), (schema_path, schema)):
        for number, line in enumerate(text.splitlines(), start=1):
            lowered = line.lower()
            if not any(phrase in lowered for phrase in phrases):
                continue
            if not any(word in lowered for word in exemptions):
                problems.append(
                    f"{path.relative_to(root)}:{number}: names the family-B raster without "
                    f"rejecting it (expected one of {exemptions})"
                )
    return problems


def check_cell_program(root: Path) -> list[str]:
    """The decision keeps its four parts, labels every budget, and claims no wire format."""
    manifest_path = root / "config" / "cell-program.json"
    if not manifest_path.is_file():
        return ["config/cell-program.json is missing"]
    try:
        manifest = json.loads(read_text(manifest_path))
    except json.JSONDecodeError as exc:
        return [f"config/cell-program.json is not valid JSON: {exc}"]

    document_path = root / str(manifest.get("document", "docs/CELL-PROGRAM-DECISION.md"))
    if not document_path.is_file():
        return [f"{document_path.relative_to(root)} is missing"]
    rel = document_path.relative_to(root)
    document = read_text(document_path)
    problems: list[str] = []

    sections = sections_by_heading(document, r"^#{2,3} (P-\d+)\b.*$")
    for question in manifest.get("questions", []):
        section = sections.get(question)
        if section is None:
            problems.append(f"{question}: no '## {question}' section in the decision document")
            continue
        for label in manifest.get("questionLabels", []):
            if f"**{label}:**" not in section:
                problems.append(f"{question}: missing its {label!r} part")

    marker = manifest.get("proposalMarker")
    if marker and marker.lower() not in document.lower():
        problems.append(f"the decision document never says it is a {marker!r}")

    tokens = [str(token) for token in manifest.get("forbiddenTokens", [])]
    if tokens:
        token_pattern = re.compile(
            "|".join(
                rf"\b{re.escape(token)}\b" if token.isalnum() else re.escape(token)
                for token in tokens
            )
        )
        found = token_pattern.search(document)
        if found:
            problems.append(f"{rel}: port wire token {found.group(0)!r} is not the port's to decide")

    budget_pattern = manifest.get("budgetPattern")
    budget_labels = [str(label).lower() for label in manifest.get("budgetLabels", [])]
    if budget_pattern and budget_labels:
        figures = re.compile(budget_pattern)
        for number, line in enumerate(document.splitlines(), start=1):
            if not figures.search(line):
                continue
            if not any(label in line.lower() for label in budget_labels):
                problems.append(
                    f"{rel}:{number}: a budget is stated without a label; every figure must be "
                    f"marked as one of {budget_labels}"
                )

    for phrase in manifest.get("forbiddenBudgetFrames", []):
        if str(phrase).lower() in document.lower():
            problems.append(
                f"{rel}: a budget is framed as {str(phrase)!r}; every figure here is a proposal "
                "about the class of limit, never a constraint of the imported program"
            )
    return problems


def check_cell_media(root: Path) -> list[str]:
    """The media decision keeps its four parts and proposes no wire format."""
    manifest_path = root / "config" / "cell-media.json"
    if not manifest_path.is_file():
        return ["config/cell-media.json is missing"]
    try:
        manifest = json.loads(read_text(manifest_path))
    except json.JSONDecodeError as exc:
        return [f"config/cell-media.json is not valid JSON: {exc}"]

    document_path = root / str(manifest.get("document", "docs/CELL-MEDIA-DECISION.md"))
    if not document_path.is_file():
        return [f"{document_path.relative_to(root)} is missing"]
    rel = document_path.relative_to(root)
    document = read_text(document_path)
    problems: list[str] = []

    sections = sections_by_heading(document, r"^#{2,3} (M-\d+)\b.*$")
    for question in manifest.get("questions", []):
        section = sections.get(question)
        if section is None:
            problems.append(f"{question}: no '## {question}' section in the decision document")
            continue
        for label in manifest.get("questionLabels", []):
            if f"**{label}:**" not in section:
                problems.append(f"{question}: missing its {label!r} part")

    marker = manifest.get("proposalMarker")
    if marker and marker.lower() not in document.lower():
        problems.append(f"the decision document never says it is a {marker!r}")

    tokens = [str(token) for token in manifest.get("forbiddenTokens", [])]
    if tokens:
        token_pattern = re.compile(
            "|".join(
                rf"\b{re.escape(token)}\b" if token.isalnum() else re.escape(token)
                for token in tokens
            )
        )
        found = token_pattern.search(document)
        if found:
            problems.append(f"{rel}: port wire token {found.group(0)!r} is not the port's to decide")
    return problems


def check_cell_stream(root: Path) -> list[str]:
    """The transport decision keeps its four parts and proposes no wire format."""
    manifest_path = root / "config" / "cell-stream.json"
    if not manifest_path.is_file():
        return ["config/cell-stream.json is missing"]
    try:
        manifest = json.loads(read_text(manifest_path))
    except json.JSONDecodeError as exc:
        return [f"config/cell-stream.json is not valid JSON: {exc}"]

    document_path = root / str(manifest.get("document", "docs/CELL-TRANSPORT-DECISION.md"))
    if not document_path.is_file():
        return [f"{document_path.relative_to(root)} is missing"]
    rel = document_path.relative_to(root)
    document = read_text(document_path)
    problems: list[str] = []

    sections = sections_by_heading(document, r"^#{2,3} (T-\d+)\b.*$")
    for question in manifest.get("questions", []):
        section = sections.get(question)
        if section is None:
            problems.append(f"{question}: no '## {question}' section in the decision document")
            continue
        for label in manifest.get("questionLabels", []):
            if f"**{label}:**" not in section:
                problems.append(f"{question}: missing its {label!r} part")

    marker = manifest.get("proposalMarker")
    if marker and marker.lower() not in document.lower():
        problems.append(f"the decision document never says it is a {marker!r}")

    tokens = [str(token) for token in manifest.get("forbiddenTokens", [])]
    if tokens:
        token_pattern = re.compile(
            "|".join(
                rf"\b{re.escape(token)}\b" if token.isalnum() else re.escape(token)
                for token in tokens
            )
        )
        found = token_pattern.search(document)
        if found:
            problems.append(f"{rel}: port wire token {found.group(0)!r} is not the port's to decide")
    return problems


def check_ruling_index(root: Path) -> list[str]:
    """The index covers every source question once, with that source's owner and disposition."""
    manifest_path = root / "config" / "ruling-index.json"
    if not manifest_path.is_file():
        return ["config/ruling-index.json is missing"]
    try:
        manifest = json.loads(read_text(manifest_path))
    except json.JSONDecodeError as exc:
        return [f"config/ruling-index.json is not valid JSON: {exc}"]

    index_path = root / str(manifest.get("index", "docs/RULING-INDEX.md"))
    if not index_path.is_file():
        return [f"{index_path.relative_to(root)} is missing"]
    rel = index_path.relative_to(root)
    index = read_text(index_path)
    problems: list[str] = []

    dispositions = manifest.get("dispositions", {})
    marker = str(manifest.get("tabledMarker", "none is derivable"))

    def normalise(value: str) -> str:
        return " ".join(value.split()).rstrip(".").strip().lower()

    expected: dict[str, tuple[str, str, str]] = {}
    for source in manifest.get("sources", []):
        source_path = root / str(source)
        if not source_path.is_file():
            problems.append(f"the index names a source that is missing: {source}")
            continue
        sections = sections_by_heading(read_text(source_path), r"^#{2,3} ([QPMT]-\d+)\b.*$")
        for question, section in sections.items():
            owner_match = re.search(r"^\*\*Owner:\*\*\s*(.+)$", section, re.M)
            owner = owner_match.group(1).strip() if owner_match else ""
            if not owner:
                problems.append(f"{question} in {source} has no owner line to index")
                continue
            kind = "tabled" if marker in section else "recommended"
            expected[question] = (owner, dispositions.get(kind, kind), str(source))

    rows: dict[str, tuple[str, str, str, int]] = {}
    for number, line in enumerate(index.splitlines(), start=1):
        stripped = line.strip()
        if not stripped.startswith("|"):
            continue
        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if not cells or not re.fullmatch(r"[QPMT]-\d+", cells[0]):
            continue
        question = cells[0]
        if len(cells) < 5:
            problems.append(
                f"{rel}:{number}: {question} needs id, subject, disposition, owner and source"
            )
            continue
        if question in rows:
            problems.append(f"{rel}:{number}: {question} is indexed more than once")
            continue
        rows[question] = (cells[2], cells[3], cells[4], number)

    for question in sorted(expected):
        source_owner, source_disposition, source_file = expected[question]
        if question not in rows:
            problems.append(f"{question}: a source carries this question and the index omits it")
            continue
        disposition, owner, listed_source, number = rows[question]
        if normalise(source_owner) not in normalise(owner):
            problems.append(
                f"{rel}:{number}: {question} owner {owner!r} does not match the source's "
                f"{source_owner!r}"
            )
        if disposition not in dispositions.values():
            problems.append(
                f"{rel}:{number}: {question} is shown as {disposition!r}; a question may only be "
                f"presented as one of {sorted(dispositions.values())}"
            )
        elif disposition != source_disposition:
            problems.append(
                f"{rel}:{number}: {question} disposition {disposition!r} contradicts the source's "
                f"{source_disposition!r}"
            )
        if listed_source != source_file:
            problems.append(
                f"{rel}:{number}: {question} cites {listed_source!r}, but the source is "
                f"{source_file!r}"
            )

    for question in sorted(set(rows) - set(expected)):
        problems.append(f"{question}: the index carries a question no source carries")

    tokens = [str(token) for token in manifest.get("forbiddenTokens", [])]
    if tokens:
        token_pattern = re.compile(
            "|".join(
                rf"\b{re.escape(token)}\b" if token.isalnum() else re.escape(token)
                for token in tokens
            )
        )
        found = token_pattern.search(index)
        if found:
            problems.append(f"{rel}: port wire token {found.group(0)!r} is not the port's to decide")
    return problems


RECORD_PATH = "artifacts/redacted-test-results/port-gate-selftest.json"


def normalise_problem(problem: str) -> str:
    """Replace throwaway fixture hashes and identities so a rerun records the same words."""
    problem = re.sub(r"sha256:[0-9a-f]{64}", "<fixture-identity>", problem)
    return re.sub(r"\b[0-9a-f]{7,64}\b", "<fixture-commit>", problem)


def fixture_identity(root: Path) -> str:
    """Digest of the inputs a selftest run exercises: stable, and blind to nothing."""
    import hashlib

    digest = hashlib.sha256()
    files: list[Path] = []
    for rel in FIXTURE_PATHS:
        target = root / rel
        if target.is_dir():
            files.extend(path for path in target.rglob("*") if path.is_file())
        elif target.is_file():
            files.append(target)
    for path in sorted(files, key=lambda item: item.relative_to(root).as_posix()):
        relative = path.relative_to(root).as_posix()
        if relative.startswith("artifacts/"):
            continue
        if "__pycache__" in relative or relative.endswith(".pyc"):
            continue
        digest.update(relative.encode("utf-8"))
        digest.update(b"\0")
        digest.update(path.read_bytes())
        digest.update(b"\0")
    return "sha256:" + digest.hexdigest()


def build_record(results: list[dict], identity: str) -> dict:
    """The evidence, as a pure function of the results and the tree they describe."""
    return {
        "tool": "tools/port_gate.py selftest",
        "fixture": {"identity": identity, "inputs": len(FIXTURE_PATHS)},
        "python": sys.version.split()[0],
        "scenarios": len(results),
        "passed": sum(1 for item in results if item["pass"]),
        "results": results,
        "exit_status": 0 if all(item["pass"] for item in results) else 1,
    }


def serialise_record(record: dict) -> str:
    return json.dumps(record, indent=2) + "\n"


def check_selftest_record(root: Path) -> list[str]:
    """The recorded evidence describes this tree and is reproducible from its own results."""
    out = root / RECORD_PATH
    if not out.is_file():
        return [f"{RECORD_PATH} is missing; run python tools/port_gate.py selftest"]
    text = read_text(out)
    try:
        record = json.loads(text)
    except json.JSONDecodeError as exc:
        return [f"{RECORD_PATH} is not valid JSON: {exc}"]

    identity = fixture_identity(root)
    recorded = (record.get("fixture") or {}).get("identity")
    if recorded != identity:
        return [
            f"{RECORD_PATH}: the record describes fixture {recorded!r}, but this tree is "
            f"{identity!r}; re-run selftest so the evidence matches what it certifies"
        ]

    results = record.get("results")
    if not isinstance(results, list):
        return [f"{RECORD_PATH}: the scenario results are missing"]
    if serialise_record(build_record(results, identity)) != text:
        return [
            f"{RECORD_PATH}: the record cannot be rebuilt from its own results, so it carries "
            "something a second run would change; the only permitted inputs are the results and "
            "the fixture identity"
        ]
    return []


LEDGER_PATH = "ledger/ledger.jsonl"


def run_git(root: Path, *args: str) -> tuple[bool, str]:
    """Run git without raising: a check must report, not crash, on bad history."""
    completed = subprocess.run(
        ["git", "-C", str(root), *args], capture_output=True, text=True
    )
    return completed.returncode == 0, completed.stdout


def check_ledger_binding(root: Path) -> list[str]:
    """Each ledger entry resolves to the one commit that carries its unit."""
    ledger_path = root / LEDGER_PATH
    if not ledger_path.is_file():
        return [f"{LEDGER_PATH} is missing"]

    entries: list[dict] = []
    for number, line in enumerate(read_text(ledger_path).splitlines(), start=1):
        if not line.strip():
            continue
        try:
            record = json.loads(line)
        except json.JSONDecodeError:
            return [f"{LEDGER_PATH}:{number}: not valid JSON, so no commit can carry it"]
        if isinstance(record, dict):
            entries.append(record)
    if not entries:
        return [f"{LEDGER_PATH}: no entries to bind"]

    ok, _ = run_git(root, "rev-parse", "--git-dir")
    if not ok:
        return ["git history is unavailable, so no entry can be bound to a commit"]

    ok, ledger_log = run_git(
        root, "log", "--reverse", "--format=commit %H", "-p", "--unified=0", "--", LEDGER_PATH
    )
    if not ok:
        return [f"git history for {LEDGER_PATH} could not be read"]

    introduced: dict[str, list[str]] = {}
    current: str | None = None
    for line in ledger_log.splitlines():
        if line.startswith("commit "):
            current = line.split()[1]
            continue
        if current is None or not line.startswith("+") or line.startswith("+++"):
            continue
        try:
            added = json.loads(line[1:])
        except json.JSONDecodeError:
            continue
        identifier = added.get("id") if isinstance(added, dict) else None
        if isinstance(identifier, str):
            introduced.setdefault(identifier, []).append(current)

    ticket_trees: dict[str, set[str]] = {}

    def ledger_number(identifier: str) -> int:
        match = re.search(r"(\d+)$", identifier)
        return int(match.group(1)) if match else -1

    bound = [identifier for identifier in introduced if identifier in {str(e.get("id")) for e in entries}]
    bound_max = max((ledger_number(identifier) for identifier in bound), default=-1)

    problems: list[str] = []
    for entry in entries:
        identifier = str(entry.get("id", ""))
        ticket = str(entry.get("ticket", ""))
        carriers = introduced.get(identifier, [])

        if not carriers:
            if ledger_number(identifier) > bound_max:
                continue  # the newest entry: the commit that adds it closes the binding
            problems.append(
                f"{identifier}: no commit in history introduces this entry, so nothing carries it"
            )
            continue
        if len(carriers) > 1:
            problems.append(
                f"{identifier}: introduced by {len(carriers)} commits; the binding must be unique"
            )
            continue

        carrier = carriers[0]
        if ticket:
            if carrier not in ticket_trees:
                ok, listing = run_git(root, "ls-tree", "-r", "--name-only", carrier, "tickets/")
                ticket_trees[carrier] = set(listing.splitlines()) if ok else set()
            if not any(
                path.startswith(f"tickets/{ticket}-") for path in ticket_trees[carrier]
            ):
                problems.append(
                    f"{identifier}: the commit that introduces it does not contain a {ticket} "
                    "ticket, so it does not carry this unit"
                )

        claimed = entry.get("unit_commit")
        if isinstance(claimed, str) and claimed:
            ok, _ = run_git(root, "cat-file", "-e", f"{claimed}^{{commit}}")
            if not ok:
                problems.append(f"{identifier}: names a commit that does not exist")
                continue
            ok, _ = run_git(root, "merge-base", "--is-ancestor", claimed, "HEAD")
            if not ok:
                problems.append(
                    f"{identifier}: names a commit that is not an ancestor of HEAD"
                )
                continue
            if claimed != carrier:
                problems.append(
                    f"{identifier}: names a commit that does not carry it; history puts this "
                    "entry in a different commit"
                )
    return problems


def mcp_patch_text(root: Path) -> str:
    """PATCH-0006 is the authority for what the MCP surface may expose."""
    for path in sorted((root / "patches" / "baselines").glob("PATCH-0006-*.md")):
        return read_text(path)
    return ""


def mcp_permitted_operations(root: Path) -> set[str]:
    return set(re.findall(r"`([a-z_]+(?:\.[a-z_]+)+)`", mcp_patch_text(root)))


def mcp_prohibited_terms(root: Path) -> list[str]:
    """The prohibited families, read from the patch's own sentence."""
    text = mcp_patch_text(root)
    match = re.search(r"Nothing else\.(.*?)The CUA Axis driver", text, re.S)
    if not match:
        return []
    terms: list[str] = []
    families = match.group(1).split("No generic tool")[0]
    for chunk in re.split(r",|\bor\b", families):
        term = re.sub(r"^no\s+", "", chunk.strip().strip("."), flags=re.I)
        term = " ".join(term.split()).lower()
        if not term or len(term) > 24 or len(term.split()) > 3:
            continue
        terms.append(term)
    return terms


def mcp_schema_paths(root: Path) -> list[Path]:
    found: list[Path] = []
    for base in ("mcp", "src/mcp"):
        directory = root / base
        if directory.is_dir():
            found.extend(path for path in sorted(directory.rglob("*")) if path.is_file())
    return found


def check_mcp_surface(root: Path) -> list[str]:
    """An MCP schema may expose only PATCH-0006's permitted operations."""
    if not mcp_patch_text(root):
        return [
            "patches/baselines/PATCH-0006-*.md is missing, so the permitted surface cannot be "
            "derived"
        ]
    permitted = mcp_permitted_operations(root)
    banned = mcp_prohibited_terms(root)
    problems: list[str] = []
    for path in mcp_schema_paths(root):
        rel = path.relative_to(root).as_posix()
        lowered = read_text(path).lower()
        for operation in sorted(set(re.findall(r"\b[a-z_]+(?:\.[a-z_]+)+\b", lowered))):
            if operation in permitted:
                continue
            problems.append(
                f"{rel}: exposes {operation!r}, which PATCH-0006 does not permit "
                f"(permitted: {', '.join(sorted(permitted))})"
            )
        for term in banned:
            if " " in term and term in lowered:
                problems.append(f"{rel}: mentions {term!r}, a family PATCH-0006 prohibits")
            elif " " not in term and re.search(rf'["\']?{re.escape(term)}["\']?\s*:', lowered):
                problems.append(
                    f"{rel}: declares a {term!r} entry, a family PATCH-0006 prohibits"
                )
    return problems


def family_a_ownership(root: Path) -> list[tuple[str, str]]:
    """The port's own layer table: layer and the single owner the record names."""
    path = root / "docs" / "PACKAGE-OWNERSHIP.md"
    if not path.is_file():
        return []
    text = read_text(path)
    section = re.search(r"^## 2\..*?(?=^## 3\.)", text, re.S | re.M)
    if not section:
        return []
    lines = section.group(0).splitlines()
    start = next(
        (
            index + 1
            for index, line in enumerate(lines)
            if line.strip().lower().startswith("| layer | owner")
        ),
        None,
    )
    if start is None:
        return []
    rows: list[tuple[str, str]] = []
    for line in lines[start:]:
        stripped = line.strip()
        if not stripped.startswith("|"):
            if rows:
                break
            continue
        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if len(cells) < 2:
            continue
        layer = cells[0]
        if not layer or set(layer) <= {"-", ":", " "}:
            continue
        owner_match = re.match(r"`([^`]+)`", cells[1])
        owner = owner_match.group(1) if owner_match else cells[1]
        rows.append((layer, owner))
    return rows


def package_reader_code_files(root: Path) -> list[str]:
    """Tracked code that references the frame magic, so a second parser can be located."""
    ok, listing = run_git(root, "ls-files")
    if not ok:
        return []
    found: list[str] = []
    for path in listing.splitlines():
        if not (path.startswith("src/") or path.startswith("crates/")):
            continue
        try:
            text = read_text(root / path)
        except (OSError, UnicodeDecodeError):
            continue
        if "V4PKG001" in text:
            found.append(path)
    return found


def check_duplicate_reader(root: Path) -> list[str]:
    """One owner per layer, one reader, and no frame parser outside the named crate."""
    rows = family_a_ownership(root)
    if not rows:
        return [
            "docs/PACKAGE-OWNERSHIP.md names no family-A layer owner, so singularity of the reader "
            "cannot be checked"
        ]

    problems: list[str] = []
    seen: dict[str, str] = {}
    for layer, owner in rows:
        key = " ".join(layer.lower().split())
        if key in seen:
            problems.append(
                f"layer {layer!r} is owned twice ({seen[key]!r} and {owner!r}); one layer has one "
                "owner"
            )
        else:
            seen[key] = owner

    reader_layers = [
        (layer, owner)
        for layer, owner in rows
        if any(word in layer.lower() for word in ("frame", "row", "payload", "declaration"))
    ]
    owners = sorted({owner for _layer, owner in reader_layers})
    if len(owners) > 1:
        problems.append(
            f"the package reader is owned by {len(owners)} crates ({', '.join(owners)}); a second "
            "owner is a second reader"
        )

    authoritative = next(
        (owner for layer, owner in rows if "frame" in layer.lower()), ""
    )
    if not authoritative:
        problems.append("no layer owns the package frame, so the reader is unnamed")

    for path in package_reader_code_files(root):
        if authoritative and not path.startswith(f"crates/{authoritative}/") and not path.startswith(
            f"{authoritative}/"
        ):
            problems.append(
                f"{path}: parses the package frame outside the {authoritative} reader named in "
                "the ownership record"
            )
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
    ("plan_errata", check_plan_errata, "PATCH-0002"),
    ("cell_schema", check_cell_schema, "PATCH-0005"),
    ("cell_program", check_cell_program, "PATCH-0005"),
    ("cell_media", check_cell_media, "PATCH-0005"),
    ("cell_stream", check_cell_stream, "PATCH-0005"),
    ("ruling_index", check_ruling_index, "PATCH-0001, PATCH-0002"),
    ("selftest_record", check_selftest_record, "PATCH-0001"),
    ("ledger_binding", check_ledger_binding, "PATCH-0001"),
    ("mcp_surface", check_mcp_surface, "PATCH-0006"),
    ("duplicate_reader", check_duplicate_reader, "PATCH-0005"),
)

# --------------------------------------------------------------------------- commands


def run_checks(
    root: Path, skip: frozenset[str] = frozenset()
) -> list[tuple[str, str, list[str]]]:
    results = []
    for name, function, patch in CHECKS:
        if name in skip:
            continue
        try:
            problems = function(root)
        except GateError as exc:
            problems = [f"gate error: {exc}"]
        results.append((name, patch, problems))
    return results


def report(results: list[tuple[str, str, list[str]]], root: Path | None = None) -> int:
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
            elif name == "mcp_surface" and root is not None and not mcp_schema_paths(root):
                print(
                    "       guard: no MCP schema present; the rule is decidable, the surface "
                    "is not yet written"
                )
            elif (
                name == "duplicate_reader"
                and root is not None
                and not package_reader_code_files(root)
            ):
                print("       guard: no code tree present; a second reader cannot appear yet")
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
    return report(run_checks(root), root)


# --------------------------------------------------------------------------- selftest

FIXTURE_PATHS = (
    "AGENTS.md",
    ".gitignore",
    ".github",
    "config",
    "docs/PACKAGE-OWNERSHIP.md",
    "docs/PORT-BASELINE.md",
    "docs/SCOPE-ACCOUNTING.md",
    "docs/CELL-MODEL.md",
    "docs/CELL-SCHEMA.md",
    "docs/CELL-MEDIA-DECISION.md",
    "docs/CELL-PROGRAM-DECISION.md",
    "docs/CELL-TRANSPORT-DECISION.md",
    "docs/RULING-INDEX.md",
    "docs/SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md",
    "patches",
    "tickets",
    "ledger",
    "tools",
)

FIXTURE_COMMIT = "TICKET-0001 LEDGER-0001 fixture baseline"


def write_fixture_record(fixture: Path) -> None:
    """Give the fixture a record that describes the fixture, not the checkout it came from."""
    out = fixture / RECORD_PATH
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(
        serialise_record(build_record([], fixture_identity(fixture))), encoding="utf-8"
    )


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
    write_fixture_record(destination)
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


def mutate_missing_erratum(root: Path) -> None:
    path = root / "docs" / "SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md"
    chunks = read_text(path).split("\n\n")
    kept = [chunk for chunk in chunks if not chunk.lstrip().startswith("> **ERRATUM E-3")]
    if len(kept) == len(chunks):
        raise GateError("fixture: the E-3 block was not found before mutation")
    path.write_text("\n\n".join(kept), encoding="utf-8")


def mutate_erratum_original_removed(root: Path) -> None:
    path = root / "docs" / "SWF_CELL_PACKAGE_IMPLEMENTATION_PLAN_v3.md"
    phrase = (
        "`shploingus.cells/1` and `shploingus.cellframe/1` are existing cell "
        "declarations/records within the package system"
    )
    text = read_text(path)
    if phrase not in text:
        raise GateError("fixture: the superseded wording was not found before mutation")
    path.write_text(text.replace(phrase, "", 1), encoding="utf-8")


def mutate_cell_invariant_removed(root: Path) -> None:
    path = root / "docs" / "CELL-MODEL.md"
    text = read_text(path)
    phrase = "**Roles, not colours.**"
    if phrase not in text:
        raise GateError("fixture: the invariant was not found before mutation")
    path.write_text(text.replace(phrase, "**Roles.**", 1), encoding="utf-8")


def mutate_cell_wire_token(root: Path) -> None:
    path = root / "docs" / "CELL-SCHEMA.md"
    text = read_text(path)
    path.write_text(text + "\nThe row is declared as `Kind::Cell`.\n", encoding="utf-8")


def mutate_cell_raster_target(root: Path) -> None:
    path = root / "docs" / "CELL-SCHEMA.md"
    text = read_text(path)
    path.write_text(
        text + "\nThe target layout is an 8 \u00d7 16 paper/ink grid.\n", encoding="utf-8"
    )


def mutate_cell_question_unanswered(root: Path) -> None:
    path = root / "docs" / "CELL-SCHEMA.md"
    text = read_text(path)
    phrase = "**Falsifier:**"
    if phrase not in text:
        raise GateError("fixture: the falsifier label was not found before mutation")
    path.write_text(text.replace(phrase, "**Note:**", 1), encoding="utf-8")


def mutate_cell_program_owner_removed(root: Path) -> None:
    path = root / "docs" / "CELL-PROGRAM-DECISION.md"
    text = read_text(path)
    phrase = "**Owner:**"
    if phrase not in text:
        raise GateError("fixture: the owner label was not found before mutation")
    path.write_text(text.replace(phrase, "**Decided by:**", 1), encoding="utf-8")


def mutate_cell_program_wire_token(root: Path) -> None:
    path = root / "docs" / "CELL-PROGRAM-DECISION.md"
    text = read_text(path)
    path.write_text(text + "\nThe program is carried as `Kind::Program`.\n", encoding="utf-8")


def mutate_cell_program_budget_unlabelled(root: Path) -> None:
    path = root / "docs" / "CELL-PROGRAM-DECISION.md"
    text = read_text(path)
    path.write_text(
        text + "\nEach activation may execute 4096 instructions in total.\n", encoding="utf-8"
    )


def mutate_cell_program_budget_as_constraint(root: Path) -> None:
    path = root / "docs" / "CELL-PROGRAM-DECISION.md"
    text = read_text(path)
    path.write_text(
        text + "\n[proposal] This is the imported program's limit.\n", encoding="utf-8"
    )


def mutate_cell_media_owner_removed(root: Path) -> None:
    path = root / "docs" / "CELL-MEDIA-DECISION.md"
    text = read_text(path)
    phrase = "**Owner:**"
    if phrase not in text:
        raise GateError("fixture: the owner label was not found before mutation")
    path.write_text(text.replace(phrase, "**Decided by:**", 1), encoding="utf-8")


def mutate_cell_media_wire_token(root: Path) -> None:
    path = root / "docs" / "CELL-MEDIA-DECISION.md"
    text = read_text(path)
    path.write_text(text + "\nThe bitmap row is `Kind::Bitmap`.\n", encoding="utf-8")


def mutate_cell_media_question_missing(root: Path) -> None:
    path = root / "docs" / "CELL-MEDIA-DECISION.md"
    text = read_text(path)
    phrase = "## M-4 —"
    if phrase not in text:
        raise GateError("fixture: the M-4 heading was not found before mutation")
    path.write_text(text.replace(phrase, "## Bounds —", 1), encoding="utf-8")


def mutate_cell_stream_owner_removed(root: Path) -> None:
    path = root / "docs" / "CELL-TRANSPORT-DECISION.md"
    text = read_text(path)
    phrase = "**Owner:**"
    if phrase not in text:
        raise GateError("fixture: the owner label was not found before mutation")
    path.write_text(text.replace(phrase, "**Decided by:**", 1), encoding="utf-8")


def mutate_cell_stream_wire_token(root: Path) -> None:
    path = root / "docs" / "CELL-TRANSPORT-DECISION.md"
    text = read_text(path)
    path.write_text(text + "\nEach chunk is `Kind::Chunk`.\n", encoding="utf-8")


def mutate_cell_stream_question_missing(root: Path) -> None:
    path = root / "docs" / "CELL-TRANSPORT-DECISION.md"
    text = read_text(path)
    phrase = "## T-4 —"
    if phrase not in text:
        raise GateError("fixture: the T-4 heading was not found before mutation")
    path.write_text(text.replace(phrase, "## Duplicate delivery —", 1), encoding="utf-8")


def mutate_ruling_index_owner_removed(root: Path) -> None:
    path = root / "docs" / "RULING-INDEX.md"
    lines = read_text(path).splitlines()
    out = []
    hit = False
    for line in lines:
        if line.strip().startswith("| Q-6 |"):
            out.append(line.replace("project owner", ""))
            hit = True
        else:
            out.append(line)
    if not hit:
        raise GateError("fixture: the Q-6 row was not found before mutation")
    path.write_text("\n".join(out) + "\n", encoding="utf-8")


def mutate_ruling_index_id_dropped(root: Path) -> None:
    path = root / "docs" / "RULING-INDEX.md"
    lines = read_text(path).splitlines()
    out = [line for line in lines if not line.strip().startswith("| P-3 |")]
    if len(out) == len(lines):
        raise GateError("fixture: the P-3 row was not found before mutation")
    path.write_text("\n".join(out) + "\n", encoding="utf-8")


def mutate_ruling_index_extra_id(root: Path) -> None:
    path = root / "docs" / "RULING-INDEX.md"
    text = read_text(path)
    path.write_text(
        text
        + "\n| Q-9 | an invented question | recommended, awaiting ruling | format owner | "
        "docs/CELL-SCHEMA.md |\n",
        encoding="utf-8",
    )


def mutate_ruling_index_recommendation_as_ruled(root: Path) -> None:
    path = root / "docs" / "RULING-INDEX.md"
    lines = read_text(path).splitlines()
    out = []
    hit = False
    for line in lines:
        if line.strip().startswith("| P-1 |"):
            out.append(line.replace("recommended, awaiting ruling", "ruled"))
            hit = True
        else:
            out.append(line)
    if not hit:
        raise GateError("fixture: the P-1 row was not found before mutation")
    path.write_text("\n".join(out) + "\n", encoding="utf-8")


def mutate_ruling_index_wire_token(root: Path) -> None:
    path = root / "docs" / "RULING-INDEX.md"
    text = read_text(path)
    path.write_text(text + "\nThe index row is `Kind::Index` .\n", encoding="utf-8")


def mutate_record_stale_fixture(root: Path) -> None:
    path = root / "docs" / "CELL-MEDIA-DECISION.md"
    text = read_text(path)
    path.write_text(text + "\nA paragraph that changes the fixture inputs.\n", encoding="utf-8")


def mutate_record_volatile_field(root: Path) -> None:
    path = root / RECORD_PATH
    record = json.loads(read_text(path))
    record["timestamp"] = "whenever this ran"
    path.write_text(serialise_record(record), encoding="utf-8")


def mutate_ledger_binding_fabricated(root: Path) -> None:
    path = root / LEDGER_PATH
    records = [json.loads(line) for line in read_text(path).splitlines() if line.strip()]
    records[0]["unit_commit"] = "0" * 40
    path.write_text(
        "\n".join(json.dumps(record) for record in records) + "\n", encoding="utf-8"
    )


def mutate_ledger_binding_wrong_commit(root: Path) -> None:
    note = root / "docs" / "drift-note.md"
    note.write_text("a commit that carries no ledger entry\n", encoding="utf-8")
    fixture_commit(root, "TICKET-0001 LEDGER-0001 a commit without a ledger line")
    ok, listing = run_git(root, "rev-parse", "HEAD")
    later_commit = listing.strip() if ok else ""
    if not later_commit:
        raise GateError("fixture: the later commit was not found")
    path = root / LEDGER_PATH
    records = [json.loads(line) for line in read_text(path).splitlines() if line.strip()]
    if len(records) < 2:
        raise GateError("fixture: fewer than two ledger entries")
    records[1]["unit_commit"] = later_commit
    path.write_text(
        "\n".join(json.dumps(record) for record in records) + "\n", encoding="utf-8"
    )


def mutate_mcp_forbidden_operation(root: Path) -> None:
    path = root / "mcp" / "surface.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(
            {"operations": ["schema.describe", "surface.capture"]}, indent=2
        )
        + "\n",
        encoding="utf-8",
    )
    write_fixture_record(root)


def mutate_mcp_permitted_surface(root: Path) -> None:
    path = root / "mcp" / "surface.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(
            {
                "operations": [
                    "schema.describe",
                    "ticket.submit",
                    "secure_cell.get",
                    "test.list_declared",
                    "test.run_declared",
                    "test.result.get",
                ]
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    write_fixture_record(root)


def mutate_duplicate_reader_second_parser(root: Path) -> None:
    path = root / "crates" / "other-reader" / "src" / "frame.rs"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text('const MAGIC: &[u8] = b"V4PKG001";\n', encoding="utf-8")
    git(root, "add", "-A")
    write_fixture_record(root)


def mutate_duplicate_reader_permitted_parser(root: Path) -> None:
    path = root / "crates" / "planet-package" / "src" / "frame.rs"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text('const MAGIC: &[u8] = b"V4PKG001";\n', encoding="utf-8")
    git(root, "add", "-A")
    write_fixture_record(root)


def mutate_duplicate_reader_second_owner(root: Path) -> None:
    path = root / "docs" / "PACKAGE-OWNERSHIP.md"
    text = read_text(path)
    row = (
        "| Package frame | `planet-package` | `frame.rs`: `FRAME_MAGIC`, `FRAME_SCHEMA`, "
        "`FrameKind`, `Package::{encode, decode, read, write}`, `frame_digest`, `file_name`, "
        "`id_and_digest` |"
    )
    if row not in text:
        raise GateError("fixture: the package-frame row was not found before mutation")
    duplicate = "| Package frame | `planet-alt` | `frame.rs`: a second parser |"
    path.write_text(text.replace(row, row + "\n" + duplicate, 1), encoding="utf-8")
    write_fixture_record(root)


def mutate_governance_file_removed(root: Path) -> None:
    path = root / "tickets" / "TEMPLATE.md"
    if not path.is_file():
        raise GateError("fixture: tickets/TEMPLATE.md was not found before mutation")
    path.unlink()


def mutate_second_format_magic(root: Path) -> None:
    path = root / "src" / "alt_format.rs"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text('const MAGIC: &[u8] = b"SHPKG002";\n', encoding="utf-8")


def mutate_scope_block_dropped(root: Path) -> None:
    path = root / "docs" / "SCOPE-ACCOUNTING.md"
    text = read_text(path)
    pattern = re.compile(r"^(\|\s*H3\s*\|\s*)(present|not-implemented|blocked)(\s*\|)", re.M)
    if not pattern.search(text):
        raise GateError("fixture: the H3 status row was not found before mutation")
    path.write_text(pattern.sub(r"\1done\3", text, count=1), encoding="utf-8")


SELFTEST_SCENARIOS = (
    ("valid_fixture", None, None),
    ("tracked_movie_refused", mutate_tracked_movie, "source_exclusion"),
    ("output_escape_refused", mutate_output_escape, "output_roots"),
    ("missing_baseline_refused", mutate_missing_baseline, "baselines_present"),
    ("invalid_ledger_refused", mutate_invalid_ledger, "ledger_valid"),
    ("unticketed_commit_refused", mutate_commit_without_ticket, "commit_refs"),
    ("block_without_exit_test_refused", mutate_block_without_exit_test, "baseline_blocks"),
    ("agent_gate_state_refused", mutate_agent_gate_state, "ledger_valid"),
    ("erratum_removed_refused", mutate_missing_erratum, "plan_errata"),
    ("erratum_original_removed_refused", mutate_erratum_original_removed, "plan_errata"),
    ("cell_invariant_removed_refused", mutate_cell_invariant_removed, "cell_schema"),
    ("cell_wire_token_refused", mutate_cell_wire_token, "cell_schema"),
    ("cell_raster_target_refused", mutate_cell_raster_target, "cell_schema"),
    ("cell_question_unanswered_refused", mutate_cell_question_unanswered, "cell_schema"),
    ("cell_program_owner_removed_refused", mutate_cell_program_owner_removed, "cell_program"),
    ("cell_program_wire_token_refused", mutate_cell_program_wire_token, "cell_program"),
    ("cell_program_budget_unlabelled_refused", mutate_cell_program_budget_unlabelled, "cell_program"),
    ("cell_program_budget_as_constraint_refused", mutate_cell_program_budget_as_constraint, "cell_program"),
    ("cell_media_owner_removed_refused", mutate_cell_media_owner_removed, "cell_media"),
    ("cell_media_wire_token_refused", mutate_cell_media_wire_token, "cell_media"),
    ("cell_media_question_missing_refused", mutate_cell_media_question_missing, "cell_media"),
    ("cell_stream_owner_removed_refused", mutate_cell_stream_owner_removed, "cell_stream"),
    ("cell_stream_wire_token_refused", mutate_cell_stream_wire_token, "cell_stream"),
    ("cell_stream_question_missing_refused", mutate_cell_stream_question_missing, "cell_stream"),
    ("ruling_index_owner_removed_refused", mutate_ruling_index_owner_removed, "ruling_index"),
    ("ruling_index_id_dropped_refused", mutate_ruling_index_id_dropped, "ruling_index"),
    ("ruling_index_extra_id_refused", mutate_ruling_index_extra_id, "ruling_index"),
    ("ruling_index_recommendation_as_ruled_refused", mutate_ruling_index_recommendation_as_ruled, "ruling_index"),
    ("ruling_index_wire_token_refused", mutate_ruling_index_wire_token, "ruling_index"),
    ("selftest_record_stale_refused", mutate_record_stale_fixture, "selftest_record"),
    ("selftest_record_volatile_field_refused", mutate_record_volatile_field, "selftest_record"),
    ("ledger_binding_fabricated_refused", mutate_ledger_binding_fabricated, "ledger_binding"),
    ("ledger_binding_wrong_commit_refused", mutate_ledger_binding_wrong_commit, "ledger_binding"),
    ("mcp_surface_forbidden_operation_refused", mutate_mcp_forbidden_operation, "mcp_surface"),
    ("mcp_surface_permitted_passes", mutate_mcp_permitted_surface, None),
    ("duplicate_reader_second_parser_refused", mutate_duplicate_reader_second_parser, "duplicate_reader"),
    ("duplicate_reader_second_owner_refused", mutate_duplicate_reader_second_owner, "duplicate_reader"),
    ("duplicate_reader_permitted_parser_passes", mutate_duplicate_reader_permitted_parser, None),
)


# One violating mutation per enforced check, with the wording that check alone must emit. Each
# entry is run twice: enabled (the wording must come from this check and no other) and with the
# check skipped (the wording must disappear). A check with no entry here fails the suite.
SWEEP = (
    ("baselines_present", mutate_missing_baseline, "missing baseline patch PATCH-0004-*.md"),
    ("governance_files", mutate_governance_file_removed, "missing tickets/TEMPLATE.md"),
    ("source_exclusion", mutate_tracked_movie, "source movie is tracked in the index"),
    ("output_roots", mutate_output_escape, "output path escapes the project root"),
    (
        "format_magics",
        mutate_second_format_magic,
        "unowned package magic 'SHPKG002' (second format)",
    ),
    ("ledger_valid", mutate_invalid_ledger, "missing exit_test"),
    ("commit_refs", mutate_commit_without_ticket, "subject has no TICKET-NNNN reference"),
    ("baseline_blocks", mutate_block_without_exit_test, "H7: missing Exit test:"),
    (
        "scope_accounting",
        mutate_scope_block_dropped,
        "H3: no status row in SCOPE-ACCOUNTING.md",
    ),
    (
        "plan_errata",
        mutate_erratum_original_removed,
        "the original wording was removed from the plan",
    ),
    ("cell_schema", mutate_cell_question_unanswered, "Q-1: missing its 'Falsifier' part"),
    (
        "cell_program",
        mutate_cell_program_budget_unlabelled,
        "a budget is stated without a label",
    ),
    ("cell_media", mutate_cell_media_question_missing, "M-4: no '## M-4' section"),
    ("cell_stream", mutate_cell_stream_question_missing, "T-4: no '## T-4' section"),
    (
        "ruling_index",
        mutate_ruling_index_id_dropped,
        "P-3: a source carries this question and the index omits it",
    ),
    (
        "selftest_record",
        mutate_record_volatile_field,
        "the record cannot be rebuilt from its own results",
    ),
    ("ledger_binding", mutate_ledger_binding_fabricated, "names a commit that does not exist"),
    ("mcp_surface", mutate_mcp_forbidden_operation, "exposes 'surface.capture'"),
    (
        "duplicate_reader",
        mutate_duplicate_reader_second_parser,
        "parses the package frame outside the planet-package reader",
    ),
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
                    "problems": [normalise_problem(line) for line in problems[:10]],
                    "pass": ok,
                }
            )
            verdict = "PASS" if ok else "FAIL"
            expected = expected_failure or "no failure"
            print(f"  {verdict} {name:<34} expected: {expected}")
            if expected_failure is not None or not ok:
                for problem in problems[:6]:
                    print(f"       - {problem}")

        covered = {check for check, _mutate, _message in SWEEP}
        unproven = [name for name, _function, _patch in CHECKS if name not in covered]
        records.append(
            {
                "scenario": "sweep_covers_every_check",
                "expected_failure": None,
                "failing_checks": [],
                "problems": [f"no sweep entry proves {name} can refuse" for name in unproven],
                "pass": not unproven,
            }
        )
        print(
            f"  {'PASS' if not unproven else 'FAIL'} sweep_covers_every_check"
            f"{'':<15} {len(SWEEP)} of {len(CHECKS)} checks have a violating mutation"
        )
        if unproven:
            failures.append(f"sweep_covers_every_check: unproven {unproven}")

        for check, mutate, expected in SWEEP:
            fixture = work / f"sweep-{check}"
            problems: list[str] = []
            failed: set[str] = set()
            ok = False
            try:
                build_fixture(project_root, fixture)
                mutate(fixture)
                enabled = run_checks(fixture)
                failed = {name for name, _patch, errs in enabled if errs}
                own = [p for name, _patch, errs in enabled if name == check for p in errs]
                fired = [p for p in own if expected in p]
                elsewhere = [
                    p
                    for name, _patch, errs in enabled
                    if name != check
                    for p in errs
                    if expected in p
                ]
                disabled = run_checks(fixture, skip=frozenset({check}))
                residual = [
                    p for _name, _patch, errs in disabled for p in errs if expected in p
                ]
                ok = bool(fired) and not elsewhere and not residual
                problems = fired[:1] or own[:2] or [f"{check} did not refuse the violation"]
                if elsewhere:
                    problems.append(
                        f"the wording came from another check, so it proves nothing about "
                        f"{check}: {elsewhere[0]}"
                    )
                if residual:
                    problems.append(
                        f"the wording survived with {check} disabled, so {check} does not own "
                        f"it: {residual[0]}"
                    )
            except GateError as exc:
                failed = {"gate_error"}
                problems = [f"gate error: {exc}"]
            if not ok:
                failures.append(f"sweep_{check}: {problems[-1]}")
            records.append(
                {
                    "scenario": f"sweep_{check}",
                    "expected_failure": check,
                    "failing_checks": sorted(failed),
                    "problems": [normalise_problem(line) for line in problems[:6]],
                    "pass": ok,
                }
            )
            print(
                f"  {'PASS' if ok else 'FAIL'} sweep_{check:<28} refused by {check}, silent "
                "when disabled"
            )
            for problem in problems[:3]:
                print(f"       - {problem}")
    finally:
        try:
            remove_tree(work)
        except OSError:
            print(f"  warning: could not fully remove {work}")

    identity = fixture_identity(project_root)
    record = build_record(records, identity)
    out = project_root / RECORD_PATH
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(serialise_record(record), encoding="utf-8")

    print()
    for failure in failures:
        print(f"  - {failure}")
    print(f"record: {out.relative_to(project_root)} (fixture {identity})")
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
