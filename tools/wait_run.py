#!/usr/bin/env python3
"""One bounded wait for a redirected run, **with proof that its log is streaming**.

`AGENTS.md` §15: a redirected run may only be waited on by a command that also demonstrates, in the
same invocation, that the log is the run's live stream. This is that command.

    python tools/wait_run.py --log generated/selftest.log --exit generated/selftest.exit

What it does, and what it refuses:

- It waits for the run's **exit file** (written by the shell as `echo $? > <exit>`), not for a marker
  it hopes will appear.
- Every report states the log's **byte count and last streamed line**, so growth is evidence rather
  than assumption. A log that never grows past zero within 30 s is refused `log-not-streaming`: the
  run refused before streaming, or nothing was redirected, and waiting longer would be waiting for a
  line that cannot appear.
- The wait is bounded (`--seconds`, default 240). When the bound expires it prints the last real byte
  count and line and exits 4; it prints `RESULT:` on every path, so a caller never has to guess.

Exit codes: 0 the run succeeded, 1 the run failed, 3 `log-not-streaming`, 4 `wait-expired`,
5 the run never wrote an exit code.
"""

import argparse
import os
import sys
import time


def read_tail(path: str, limit: int = 4000) -> tuple[int, list[str]]:
    """The log's size and its last non-empty line (carriage returns stripped)."""
    try:
        with open(path, "rb") as handle:
            handle.seek(0, os.SEEK_END)
            size = handle.tell()
            handle.seek(max(0, size - limit))
            data = handle.read().decode("utf-8", "replace")
    except FileNotFoundError:
        return 0, []
    lines = [line.rstrip("\r") for line in data.splitlines() if line.strip()]
    return size, lines


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description="one bounded wait for a redirected run")
    parser.add_argument("--log", required=True, help="the run's redirected output")
    parser.add_argument("--exit", required=True, help="the file the shell writes the exit code into")
    parser.add_argument("--seconds", type=float, default=240.0, help="bound, in seconds")
    parser.add_argument("--interval", type=float, default=2.0, help="seconds between checks")
    parser.add_argument(
        "--stream-deadline",
        type=float,
        default=30.0,
        help="seconds the log may stay at zero bytes before the wait is refused",
    )
    args = parser.parse_args(argv)

    started = time.monotonic()
    last_report = started
    first_size: int | None = None

    while time.monotonic() - started < args.seconds:
        size, lines = read_tail(args.log)
        if first_size is None:
            first_size = size
        if os.path.exists(args.exit):
            try:
                with open(args.exit, encoding="utf-8") as handle:
                    raw = handle.read().strip()
            except OSError:
                raw = ""
            if raw.isdigit():
                code = int(raw)
                last = lines[-1] if lines else "(empty)"
                print(f"exit: {code}")
                print(f"log: {size} bytes (first read {first_size}); last: {last}")
                print(f"RESULT: {'PASS' if code == 0 else 'FAIL'} (exit {code})", flush=True)
                return 0 if code == 0 else 1
            # The exit file exists but is not written yet: keep waiting, within the same bound.

        if time.monotonic() - started > args.stream_deadline and size == 0:
            print(
                f"RESULT: REFUSED (log-not-streaming; {args.log} stayed at 0 bytes for "
                f"{args.stream_deadline:.0f} s, so no line can appear)",
                flush=True,
            )
            return 3

        now = time.monotonic()
        if now - last_report >= 20.0:
            last = lines[-1] if lines else "(empty)"
            print(f"alive: {size} bytes, last: {last}", flush=True)
            last_report = now
        time.sleep(args.interval)

    size, lines = read_tail(args.log)
    last = lines[-1] if lines else "(empty)"
    print(f"RESULT: REFUSED (wait-expired; {size} bytes, last: {last})", flush=True)
    return 4


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
