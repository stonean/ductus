#!/usr/bin/env python3
"""Run every `run:` step of the GitHub workflow files given, locally.

usage: python3 -I scripts/run-workflow-steps.py [--log-dir DIR] WORKFLOW...

Each `run:` step runs in `bash --noprofile --norc -eo pipefail` from the
repository root, or from the `working-directory` its job's `defaults` or the
step itself names, and is recorded PASS or FAIL from its own exit status; the
run continues past a failure, and a failed step's output is in the log file
its line names. A step that cannot run here is reported SKIP with the reason,
never as a pass:

- a step carrying an `if:`, whose condition only a workflow run can decide;
- a step whose script holds a `${{ }}` expression, which only GitHub expands;
- a step whose merged `env:` holds a `${{ }}` expression, which only a workflow
  run expands, so the step would otherwise run without a value CI gives it (a
  secret, say).

A step's environment is the workflow's `env:`, then its job's, then its own,
a later one winning, applied on top of the runner's own environment; every
value that reaches a step is a literal. `uses:` steps run nothing here and are
not counted.

Each step runs in its own process group under a bound: its `timeout-minutes:`,
else its job's, else `DEFAULT_TIMEOUT_MINUTES`, the runner's own choice for a
job that states none and not GitHub's. A positive number, a fraction included,
is accepted; anything else exits 2. When the bound passes, the whole group is
killed and the step is recorded FAIL as timed out.

A step's working directory is resolved, symbolic links followed, and must lie
inside the repository root, compared by whole path components; one outside it
exits 2 before any step runs, since a workflow's text names it (`BE-INPUT-004`).

Each workflow's count of `run:` steps found, run and skipped is printed. The
exit status is 1 when any step FAILs, and also when a workflow yields no
`run:` step, so a parse that found nothing never reads as a pass; 2 for a
usage error or a workflow file that cannot be read.

The parser reads the workflow subset this repository writes: the workflow's
`env:` at the top, jobs at two spaces, `steps:`, `defaults:`, `env:` and
`timeout-minutes:` at four, a step's `- ` at six and its keys at eight, and a
`run: |` block indented past them (spec 079, task 38).
"""
import math
import os
import signal
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# The bound, in minutes, on a step whose own `timeout-minutes:` and job's are
# both absent. The runner's own choice, not GitHub's.
DEFAULT_TIMEOUT_MINUTES = 60


def value(text):
    """A scalar value after `key:`, its surrounding quotes removed."""
    text = text.split(":", 1)[1].strip()
    if len(text) >= 2 and text[0] == text[-1] and text[0] in "'\"":
        text = text[1:-1]
    return text


def indent_of(line):
    return len(line) - len(line.lstrip(" "))


def parse_env(lines, start, key_indent):
    """The `env:` mapping whose lines follow `start`, each at an indent past
    `key_indent`, and the index of the first line after it. A block scalar
    value (`>-`, `|`) is folded or kept by line."""
    env = {}
    i = start
    entry_indent = None
    while i < len(lines):
        line = lines[i]
        if not line.strip() or line.strip().startswith("#"):
            i += 1
            continue
        if indent_of(line) <= key_indent:
            break
        if entry_indent is None:
            entry_indent = indent_of(line)
        if indent_of(line) != entry_indent or ":" not in line:
            i += 1
            continue
        name = line.strip().split(":", 1)[0]
        rest = line.strip().split(":", 1)[1].strip()
        if rest[:1] in ("|", ">"):
            body = []
            i += 1
            while i < len(lines) and (not lines[i].strip() or indent_of(lines[i]) > entry_indent):
                body.append(lines[i].strip())
                i += 1
            body = [b for b in body if b]
            env[name] = ("\n" if rest[0] == "|" else " ").join(body)
            continue
        env[name] = value(line.strip())
        i += 1
    return env, i


def parse(text):
    """The workflow's steps that have a `run:`, in order, each a dict of its
    name, script, `if:`, working directory, merged `env` (the workflow's, the
    job's, then the step's) and raw `timeout` (its own, else its job's, else
    None)."""
    lines = text.split("\n")
    steps = []
    in_jobs = False
    workflow_env = {}
    job = None
    section = None
    step = None
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            i += 1
            continue
        indent = indent_of(line)
        if indent == 0:
            in_jobs = stripped == "jobs:"
            step = None
            if stripped == "env:":
                env, i = parse_env(lines, i + 1, 0)
                workflow_env.update(env)
                continue
            i += 1
            continue
        if not in_jobs:
            i += 1
            continue
        if indent == 2:
            job = {"workdir": None, "env": {}, "timeout": None}
            section, step = None, None
            i += 1
            continue
        if job is None:
            i += 1
            continue
        if indent == 4:
            section = stripped.split(":", 1)[0]
            step = None
            if section == "env" and stripped == "env:":
                env, i = parse_env(lines, i + 1, 4)
                job["env"].update(env)
                continue
            if section == "timeout-minutes":
                job["timeout"] = value(stripped)
            i += 1
            continue
        if section == "defaults":
            if indent == 8 and stripped.startswith("working-directory:"):
                job["workdir"] = value(stripped)
            i += 1
            continue
        if section != "steps":
            i += 1
            continue
        if indent == 6 and stripped.startswith("- "):
            step = {"name": None, "run": None, "if": None, "workdir": None, "job": job,
                    "env": {}, "timeout": None}
            steps.append(step)
            stripped = stripped[2:].strip()
            indent = 8
        if step is None or indent != 8:
            i += 1
            continue
        key = stripped.split(":", 1)[0]
        if key == "env" and stripped == "env:":
            step["env"], i = parse_env(lines, i + 1, 8)
            continue
        if key == "run":
            rest = stripped.split(":", 1)[1].strip()
            if rest in ("|", "|-", "|+", ">", ">-"):
                body = []
                i += 1
                while i < len(lines) and (not lines[i].strip() or indent_of(lines[i]) > 8):
                    body.append(lines[i])
                    i += 1
                while body and not body[-1].strip():
                    body.pop()
                cut = min((indent_of(b) for b in body if b.strip()), default=0)
                step["run"] = "\n".join(b[cut:] for b in body) + "\n"
                continue
            step["run"] = value(stripped) + "\n"
        elif key == "name":
            step["name"] = value(stripped)
        elif key == "if":
            step["if"] = stripped.split(":", 1)[1].strip()
        elif key == "working-directory":
            step["workdir"] = value(stripped)
        elif key == "timeout-minutes":
            step["timeout"] = value(stripped)
        i += 1
    found = [s for s in steps if s["run"] is not None]
    for s in found:
        j = s.pop("job")
        s["job_workdir"] = j["workdir"]
        s["env"] = {**workflow_env, **j["env"], **s["env"]}
        if s["timeout"] is None:
            s["timeout"] = j["timeout"]
    return found


def timeout_minutes(raw):
    """The bound in minutes `raw` names, `DEFAULT_TIMEOUT_MINUTES` when absent,
    None when it is not a positive finite number."""
    if raw is None:
        return DEFAULT_TIMEOUT_MINUTES
    try:
        minutes = float(raw)
    except ValueError:
        return None
    return minutes if math.isfinite(minutes) and minutes > 0 else None


def contained(named):
    """The directory `named` resolves to from the repository root, symbolic
    links followed, or None when that lies outside the root. The comparison is
    by whole path components, so a sibling sharing the root's name as a prefix
    is outside it."""
    root = os.path.realpath(ROOT)
    resolved = os.path.realpath(ROOT / named)
    return Path(resolved) if os.path.commonpath([root, resolved]) == root else None


def run_bounded(script, cwd, env, log, minutes):
    """Run `script` under bash in its own process group, output to `log`.
    Returns its exit status, or None when `minutes` passed and the whole group
    was killed."""
    with open(log, "w") as out:
        proc = subprocess.Popen(
            ["bash", "--noprofile", "--norc", "-eo", "pipefail", "-c", script],
            cwd=cwd,
            env=env,
            stdout=out,
            stderr=subprocess.STDOUT,
            stdin=subprocess.DEVNULL,
            start_new_session=True,
        )
        try:
            return proc.wait(timeout=minutes * 60)
        except subprocess.TimeoutExpired:
            try:
                os.killpg(proc.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            proc.wait()
            return None


def main(argv):
    args = list(argv)
    log_dir = None
    if args[:1] == ["--log-dir"]:
        if len(args) < 2:
            print("usage: run-workflow-steps.py [--log-dir DIR] WORKFLOW...", file=sys.stderr)
            return 2
        log_dir = Path(args[1])
        args = args[2:]
    if not args:
        print("usage: run-workflow-steps.py [--log-dir DIR] WORKFLOW...", file=sys.stderr)
        return 2
    texts = {}
    for workflow in args:
        try:
            texts[workflow] = Path(workflow).read_text()
        except OSError as err:
            print(f"cannot read {workflow}: {err}", file=sys.stderr)
            return 2
    if log_dir is None:
        log_dir = Path(tempfile.mkdtemp(prefix="run-workflow-steps-"))
    log_dir.mkdir(parents=True, exist_ok=True)
    print(f"logs: {log_dir}")
    parsed = {}
    for workflow, text in texts.items():
        parsed[workflow] = parse(text)
        for step in parsed[workflow]:
            if timeout_minutes(step["timeout"]) is None:
                print(
                    f"{workflow}: timeout-minutes {step['timeout']!r} is not a positive number",
                    file=sys.stderr,
                )
                return 2
            named = step["workdir"] or step["job_workdir"] or "."
            step["cwd"] = contained(named)
            if step["cwd"] is None:
                print(
                    f"{workflow}: working-directory {named!r} resolves outside the repository",
                    file=sys.stderr,
                )
                return 2
    totals = {"PASS": 0, "FAIL": 0, "SKIP": 0}
    empty = []
    counts = []
    for workflow in texts:
        base = os.path.basename(workflow)
        steps = parsed[workflow]
        ran = skipped = 0
        for n, step in enumerate(steps):
            label = f"{base} :: {step['name'] or '(unnamed step ' + str(n + 1) + ')'}"
            if step["if"]:
                print(f"SKIP {label} — if: {step['if']}")
                skipped += 1
                totals["SKIP"] += 1
                continue
            if "${{" in step["run"]:
                print(f"SKIP {label} — its script holds a ${{{{ }}}} expression only a workflow run expands")
                skipped += 1
                totals["SKIP"] += 1
                continue
            if any("${{" in v for v in step["env"].values()):
                print(f"SKIP {label} — its env: holds a ${{{{ }}}} expression only a workflow run expands")
                skipped += 1
                totals["SKIP"] += 1
                continue
            workdir = step["cwd"]
            log = log_dir / f"{base}-{n + 1:02d}.log"
            minutes = timeout_minutes(step["timeout"])
            rc = run_bounded(step["run"], workdir, {**os.environ, **step["env"]}, log, minutes)
            ran += 1
            if rc is None:
                print(f"FAIL {label} — timed out after {minutes:g} minutes, log {log}")
                totals["FAIL"] += 1
            elif rc == 0:
                print(f"PASS {label}")
                totals["PASS"] += 1
            else:
                print(f"FAIL {label} — exit {rc}, log {log}")
                totals["FAIL"] += 1
        counts.append(f"{base}: {len(steps)} run step(s) found, {ran} run, {skipped} skipped")
        if not steps:
            empty.append(base)
    for line in counts:
        print(line)
    print(f"summary: PASS {totals['PASS']}, FAIL {totals['FAIL']}, SKIP {totals['SKIP']}")
    for base in empty:
        print(f"{base}: no run step found, so nothing was checked", file=sys.stderr)
    return 1 if totals["FAIL"] or empty else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
