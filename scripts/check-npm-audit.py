#!/usr/bin/env python3
"""Fail on a high or critical advisory in the npm lint tooling, unless the
committed list names it (spec 022 task 51; 022's plan, Provenance).

usage: python3 -I scripts/check-npm-audit.py [--package-lock-only]
                                             [--report FILE] [--allowed FILE]

`npm audit --json` (or the report FILE a test passes) is read, and every
advisory it reports at high or critical severity must be named in
scripts/npm-audit-allowed.json (or FILE), with its advisory ID, its package,
its affected range and why it does not reach this repository. That list is
BE-DEPS-001's documented exception, for an advisory no release fixes yet. An
entry fails the check again, so it goes from the list, when:

  - npm reports a fix for its package that is not a breaking change;
  - the advisory's affected range differs from the one the entry records;
  - the advisory is no longer reported.

Exit status: 0 when every high or critical advisory is listed and every entry
is live; 1 when one is not, naming each; 2 when the report or the list cannot
be read, which is never a pass. Every run says how many advisories it judged.
"""

import json
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DEFAULT_ALLOWED = os.path.join(ROOT, "scripts", "npm-audit-allowed.json")
GATING = {"high", "critical"}
ADVISORY_ID = re.compile(r"^GHSA(-[0-9a-z]{4}){3}$")


def fail_to_run(message):
    print(f"check-npm-audit: {message}; nothing was judged", file=sys.stderr)
    return 2


def advisory_id(via):
    url = via.get("url") or ""
    tail = url.rstrip("/").rsplit("/", 1)[-1]
    return tail if ADVISORY_ID.match(tail) else None


def load_allowed(path):
    with open(path, encoding="utf-8") as handle:
        data = json.load(handle)
    entries = data.get("allowed") if isinstance(data, dict) else None
    if not isinstance(entries, list):
        raise ValueError("it holds no `allowed` list")
    allowed = {}
    for entry in entries:
        if not isinstance(entry, dict):
            raise ValueError("an entry is not an object")
        for field in ("advisory", "package", "range", "reason"):
            value = entry.get(field)
            if not isinstance(value, str) or not value.strip():
                raise ValueError(f"an entry has no `{field}`: {entry!r}")
        if not ADVISORY_ID.match(entry["advisory"]):
            raise ValueError(f"`{entry['advisory']}` is not an advisory ID")
        if entry["advisory"] in allowed:
            raise ValueError(f"`{entry['advisory']}` is listed twice")
        allowed[entry["advisory"]] = entry
    return allowed


def read_report(args):
    if args["report"] is not None:
        with open(args["report"], encoding="utf-8") as handle:
            text = handle.read()
    else:
        command = ["npm", "audit", "--json"]
        if args["package-lock-only"]:
            command.append("--package-lock-only")
        # npm audit exits non-zero whenever it reports a vulnerability, so its
        # status says nothing here: the report it prints is what is judged.
        run = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
        text = run.stdout
    report = json.loads(text)
    if not isinstance(report, dict) or not isinstance(report.get("vulnerabilities"), dict):
        raise ValueError("it holds no `vulnerabilities` map")
    return report


def parse_args(argv):
    args = {"report": None, "allowed": DEFAULT_ALLOWED, "package-lock-only": False}
    rest = list(argv)
    while rest:
        flag = rest.pop(0)
        if flag == "--package-lock-only":
            args["package-lock-only"] = True
        elif flag in ("--report", "--allowed") and rest:
            args[flag[2:]] = rest.pop(0)
        else:
            raise ValueError(f"unknown argument `{flag}`")
    return args


def main(argv):
    try:
        args = parse_args(argv)
    except ValueError as err:
        return fail_to_run(str(err))
    try:
        allowed = load_allowed(args["allowed"])
    except (OSError, ValueError) as err:
        return fail_to_run(f"the list {args['allowed']} cannot be read: {err}")
    try:
        report = read_report(args)
    except (OSError, ValueError) as err:
        return fail_to_run(f"the npm audit report cannot be read: {err}")

    # Each advisory npm reports, at the package it names directly; a package
    # flagged only through another (a string `via`) inherits that one's.
    advisories = {}
    for package, vuln in report["vulnerabilities"].items():
        for via in vuln.get("via", []):
            if not isinstance(via, dict):
                continue
            ident = advisory_id(via)
            if ident is None:
                return fail_to_run(f"an advisory on `{package}` names no advisory ID: {via!r}")
            advisories[ident] = {
                "package": package,
                "severity": via.get("severity"),
                "range": via.get("range"),
                "fix": vuln.get("fixAvailable"),
                "title": via.get("title", ""),
            }

    failures, judged, listed = [], 0, 0
    for ident, adv in sorted(advisories.items()):
        if adv["severity"] not in GATING:
            continue
        judged += 1
        entry = allowed.get(ident)
        if entry is None:
            failures.append(
                f"{ident} ({adv['severity']}, {adv['package']} {adv['range']}): {adv['title']} — "
                "update or replace the dependency, or list it with why it does not reach this repository"
            )
            continue
        fix = adv["fix"]
        if fix is True or (isinstance(fix, dict) and not fix.get("isSemVerMajor")):
            failures.append(f"{ident} is listed, but npm reports a fix that is not a breaking change ({fix!r}): apply it and remove the entry")
            continue
        if entry["package"] != adv["package"] or entry["range"] != adv["range"]:
            failures.append(
                f"{ident} is listed for {entry['package']} {entry['range']}, "
                f"but npm now reports {adv['package']} {adv['range']}: judge the entry again"
            )
            continue
        listed += 1
    for ident in sorted(set(allowed) - set(advisories)):
        failures.append(f"{ident} is listed but no longer reported: remove the entry")

    # A package npm flags at a gating severity while no gating advisory was
    # found means the report holds something this check did not read.
    if judged == 0:
        for package, vuln in report["vulnerabilities"].items():
            if vuln.get("severity") in GATING:
                return fail_to_run(f"`{package}` is {vuln.get('severity')} but no gating advisory was found")

    print(
        f"check-npm-audit: {judged} high or critical advisory(ies) reported, "
        f"{listed} listed in {os.path.relpath(args['allowed'], ROOT)}, {len(failures)} failing"
    )
    for failure in failures:
        print(f"  {failure}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
