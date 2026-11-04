#!/usr/bin/env python3
"""Streaming, read-only inventory scanner for the secure shell v2 audit.

Every root is flushed to secure_shell_synthesis.md and scan_stream.jsonl the
moment it finishes, so truncation is visible while the scan runs instead of
being discovered at the end. Never enters inputs/, shell-private/ or captures/.
"""
from __future__ import annotations

import json
import os
import re
import sys
import time
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE / "secure_shell_synthesis.md"
STREAM = HERE / "scan_stream.jsonl"
STATE = HERE / "scan_state.json"

D = Path("C:/Users/ala/Documents")
ROOTS = [
    D / "testport1", D / "shploingus-clean", D / "shploingus-game", D / "shploingus",
    D / "openlock", D / "secure-shell", D / "dl-enclave", D / "ala_civ_bots_prototype",
    D / "ala_package_designer", D / "agent_god_files",
    Path("F:/planet-shploingus"), Path("F:/shploingus-mod-sources"),
    Path("F:/ala-cities-temp"), Path("F:/agent_god_build"), Path("F:/tmp_db"),
]
DRIVES = [Path(f"{d}:/") for d in "CDEFGH"]

SKIP = {
    ".git", "node_modules", "target", "dist", "build", "out", ".venv", "venv",
    "__pycache__", ".next", ".cache", ".gradle", ".tools", ".idea", "AppData",
    "Windows", "ProgramData", "Program Files", "Program Files (x86)", "$RECYCLE.BIN",
    "System Volume Information", "SteamLibrary", "inputs", "shell-private", "captures",
    "WpSystem", "WpSystemBackup", "Recovery", "Config.Msi", "$WinREAgent",
    "secure-shell-audit", "openlock.reference",
}
TEXT = {".md", ".rs", ".js", ".mjs", ".cjs", ".ts", ".java", ".json", ".toml",
        ".yml", ".yaml", ".ps1", ".sh", ".sml", ".gradle", ".xml", ".cfg", ".qml", ".py"}

MAX_FILE = 512 * 1024
ROOT_BUDGET = 80.0
DRIVE_BUDGET = 15.0
TOTAL_BUDGET = 780.0

ID_RE = re.compile(r'["`]([a-z][a-zA-Z0-9._-]{2,48}/[0-9]{1,3})["`]')
HASH_ID_RE = re.compile(r'\b([a-z][a-z0-9-]{1,30}\.[a-z][a-z0-9-]{1,30}/[0-9a-f]{8,64})\b')
MAGIC_RE = re.compile(r'b?"([A-Z0-9]{1,12}PKG[0-9]{3})"')
ALGO_RE = re.compile(r"^(SHA|HMAC|AES|BLAKE|MD5|TestVet)", re.I)
MONGO_RE = re.compile(r'mongodb://|MongoClient|getDatabase\(|getCollection\(')
LEDGER_RE = re.compile(r"ledger\.jsonl|surface\.mount|planet-shell\.ledger")
AXIS_RE = re.compile(r"axis-grant|civilian-unwitnessed|CivilianRefusal|cua-action|AxisGate")
CUA_RE = re.compile(r"\bCUA\b|cua_|computer-use")
QT_RE = re.compile(r"QGuiApplication|QQmlApplicationEngine|QXmlStreamReader|find_package\(Qt6|qt_add_qml", re.I)
REG_RE = re.compile(r"format[-_ ]registry|registry\.json|registry of formats|version hash", re.I)

started = time.time()
files_seen = 0
totals = {"formats": Counter(), "magics": Counter(), "hash_ids": Counter(), "xml": Counter()}
stream = STREAM.open("w", encoding="utf-8")
report = OUT.open("w", encoding="utf-8")


def emit(line: str) -> None:
    report.write(line + "\n")
    report.flush()
    os.fsync(report.fileno())


def emit_json(record: dict) -> None:
    stream.write(json.dumps(record, sort_keys=True) + "\n")
    stream.flush()
    os.fsync(stream.fileno())


def walk(root: Path, budget: float, max_depth: int = 8) -> dict:
    global files_seen
    root_start = time.time()
    local = {"root": str(root), "files": 0, "truncated": False, "formats": Counter(),
             "hash_ids": Counter(), "magics": Counter(), "stores": [], "axis": [],
             "qt": [], "xml": 0, "notes": []}
    if not root.exists():
        local["truncated"] = True
        local["notes"].append("absent")
        return local
    for dirpath, dirnames, filenames in os.walk(root, onerror=lambda e: local["notes"].append(f"walk: {e}")):
        if time.time() - root_start > budget or time.time() - started > TOTAL_BUDGET:
            local["truncated"] = True
            local["notes"].append(f"budget reached at {dirpath}")
            break
        dirnames[:] = [d for d in dirnames if d not in SKIP and not d.startswith(".git")]
        if dirpath[len(str(root)):].count(os.sep) >= max_depth:
            dirnames[:] = []
        for name in filenames:
            local["files"] += 1
            files_seen += 1
            p = Path(dirpath) / name
            ext = p.suffix.lower()
            if ext == ".xml":
                local["xml"] += 1
                totals["xml"][str(root)] += 1
                continue
            if ext not in TEXT:
                continue
            try:
                if p.stat().st_size > MAX_FILE:
                    continue
                text = p.read_text(encoding="utf-8", errors="replace")
            except OSError as exc:
                local["notes"].append(f"{p}: {exc}")
                continue
            rel = str(p)
            for ident in ID_RE.findall(text):
                local["formats"][ident] += 1
                totals["formats"][ident] += 1
            for ident in HASH_ID_RE.findall(text):
                local["hash_ids"][ident] += 1
                totals["hash_ids"][ident] += 1
            for magic in MAGIC_RE.findall(text):
                local["magics"][magic] += 1
                totals["magics"][magic] += 1
            mongo = MONGO_RE.search(text)
            if mongo:
                snippet = text[max(0, mongo.start() - 30):mongo.start() + 110].replace("\n", " ")
                local["stores"].append(f"{rel} :: {snippet.strip()}")
            elif LEDGER_RE.search(text):
                local["stores"].append(rel)
            if AXIS_RE.search(text):
                local["axis"].append(rel)
            if QT_RE.search(text):
                local["qt"].append(rel)
    local["stores"] = local["stores"][:60]
    local["axis"] = local["axis"][:60]
    local["qt"] = local["qt"][:40]
    local["seconds"] = round(time.time() - root_start, 1)
    return local


def repo_scan(root: Path) -> list:
    repos = []
    for dirpath, dirnames, _ in os.walk(root):
        if time.time() - started > TOTAL_BUDGET:
            return repos
        dirnames[:] = [d for d in dirnames if d not in SKIP]
        if dirpath[len(str(root)):].count(os.sep) >= 3:
            dirnames[:] = []
        try:
            is_repo = (Path(dirpath) / ".git").exists()
        except OSError:
            is_repo = False
        if is_repo:
            wf = Path(dirpath) / ".github" / "workflows"
            repos.append({"path": dirpath,
                          "workflows": sorted(x.name for x in wf.glob("*")) if wf.is_dir() else [],
                          "docs": [n for n in ("AGENTS.md", "VOWS.md", "CHECKPOINT.md", "CONTEXT.md")
                                   if (Path(dirpath) / n).exists()]})
            dirnames[:] = []
    return repos


def stream_root(local: dict, repos: list) -> None:
    own = Counter({k: v for k, v in local["formats"].items() if not ALGO_RE.match(k)})
    emit(f"\n## `{local['root']}`")
    emit(f"- files: {local['files']} · seconds: {local['seconds']} · "
         f"**truncated: {'yes' if local['truncated'] else 'no'}** · xml: {local['xml']}")
    if repos:
        for r in repos:
            emit(f"- repo `{r['path']}` docs={r['docs'] or '**none**'} workflows={r['workflows'] or '**none**'}")
    if own:
        emit("- identifiers: " + ", ".join(f"`{k}`×{v}" for k, v in own.most_common(25)))
    if local["hash_ids"]:
        emit("- hash-named ids: " + ", ".join(f"`{k}`" for k in local["hash_ids"])[:400])
    if local["magics"]:
        emit("- magics: " + ", ".join(f"`{k}`×{v}" for k, v in local["magics"].most_common()))
    if local["stores"]:
        emit("- store sites:")
        for s in local["stores"]:
            emit(f"  - `{s}`")
    if local["axis"]:
        emit("- axis/civ sites: " + ", ".join(f"`{a}`" for a in local["axis"][:12]))
    if local["qt"]:
        emit("- qt sites: " + ", ".join(f"`{q}`" for q in local["qt"][:8]))
    for n in local["notes"][:4]:
        emit(f"- note: {n}")
    record = {k: local[k] for k in ("root", "files", "seconds", "truncated", "xml")}
    record["identifiers"] = dict(own.most_common(50))
    record["magics"] = dict(local["magics"])
    record["stores"] = local["stores"][:20]
    record["axis_sites"] = len(local["axis"])
    record["repos"] = repos
    emit_json(record)


def main() -> int:
    emit("# secure shell v2 — streamed inventory")
    emit(f"Started {time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())}. "
         f"Per-root budget {ROOT_BUDGET:.0f}s, total {TOTAL_BUDGET:.0f}s. "
         "Each root is flushed as it completes; `truncated: yes` is a refusal, not a pass.")
    for root in ROOTS:
        repos = repo_scan(root)
        found = walk(root, ROOT_BUDGET)
        stream_root(found, repos)
    for d in DRIVES:
        if time.time() - started > TOTAL_BUDGET:
            emit(f"\n## `{d}` skipped (total budget exhausted)")
            continue
        if d.exists():
            stream_root(walk(d, DRIVE_BUDGET, max_depth=2), repo_scan(d))
    numeric = {k: v for k, v in totals["formats"].items()
               if not ALGO_RE.match(k) and re.search(r"/[0-9]+$", k)}
    emit("\n## Totals")
    emit(f"- files inspected: {files_seen} · elapsed: {time.time() - started:.0f}s")
    emit(f"- own numeric identifiers: {len(numeric)} distinct, {sum(numeric.values())} occurrences")
    emit(f"- hash-named identifiers: {len(totals['hash_ids'])}")
    emit(f"- magics: {dict(totals['magics'])}")
    emit(f"- xml files: {sum(totals['xml'].values())}")
    emit("\n## Refusals this scan itself demonstrates")
    emit("- `audit-stream-truncated`: a root hit its budget and still produced a partial row.")
    emit("- `audit-gap-silent`: an absent root is reported as absent, never as zero findings.")
    emit("- `audit-unprovenanced`: a count without file:line is not accepted as evidence.")
    emit("- `audit-unstreamed`: output held until exit hides truncation and is refused.")
    STATE.write_text(json.dumps({"files": files_seen, "seconds": round(time.time() - started, 1),
                                 "formats": totals["formats"], "magics": totals["magics"],
                                 "hash_ids": totals["hash_ids"]}, indent=2), encoding="utf-8")
    stream.close()
    report.close()
    print(f"done in {time.time() - started:.1f}s files={files_seen}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
