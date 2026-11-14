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
- a step whose script holds a `${{ }}` expression, which only GitHub expands.

A step's `env:` is not applied: in these workflows every value is such an
expression. `uses:` steps run nothing here and are not counted.

Each workflow's count of `run:` steps found, run and skipped is printed. The
exit status is 1 when any step FAILs, and also when a workflow yields no
`run:` step, so a parse that found nothing never reads as a pass; 2 for a
usage error or a workflow file that cannot be read.

The parser reads the workflow subset this repository writes: jobs at two
spaces, `steps:` and `defaults:` at four, a step's `- ` at six and its keys at
eight, and a `run: |` block indented past them (spec 079, task 38).
"""
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def value(text):
    """A scalar value after `key:`, its surrounding quotes removed."""
    text = text.split(":", 1)[1].strip()
    if len(text) >= 2 and text[0] == text[-1] and text[0] in "'\"":
        text = text[1:-1]
    return text


def indent_of(line):
    return len(line) - len(line.lstrip(" "))


def parse(text):
    """The workflow's steps that have a `run:`, in order, each a dict of its
    name, script, `if:` and working directory."""
    lines = text.split("\n")
    steps = []
    in_jobs = False
    job_workdir = None
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
            i += 1
            continue
        if not in_jobs:
            i += 1
            continue
        if indent == 2:
            job_workdir, section, step = None, None, None
            i += 1
            continue
        if indent == 4:
            section = stripped.split(":", 1)[0]
            step = None
            i += 1
            continue
        if section == "defaults":
            if indent == 8 and stripped.startswith("working-directory:"):
                job_workdir = value(stripped)
            i += 1
            continue
        if section != "steps":
            i += 1
            continue
        if indent == 6 and stripped.startswith("- "):
            step = {"name": None, "run": None, "if": None, "workdir": None, "job_workdir": job_workdir}
            steps.append(step)
            stripped = stripped[2:].strip()
            indent = 8
        if step is None or indent != 8:
            i += 1
            continue
        key = stripped.split(":", 1)[0]
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
        i += 1
    return [s for s in steps if s["run"] is not None]


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
    totals = {"PASS": 0, "FAIL": 0, "SKIP": 0}
    empty = []
    counts = []
    for workflow, text in texts.items():
        base = os.path.basename(workflow)
        steps = parse(text)
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
            workdir = ROOT / (step["workdir"] or step["job_workdir"] or ".")
            log = log_dir / f"{base}-{n + 1:02d}.log"
            with open(log, "w") as out:
                rc = subprocess.run(
                    ["bash", "--noprofile", "--norc", "-eo", "pipefail", "-c", step["run"]],
                    cwd=workdir,
                    stdout=out,
                    stderr=subprocess.STDOUT,
                    stdin=subprocess.DEVNULL,
                ).returncode
            ran += 1
            if rc == 0:
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
