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

  - npm reports a fix that is not a breaking change, on any package it names
    the advisory on;
  - the advisory's package or affected range differs from the entry's;
  - the advisory is no longer reported, or is no longer high or critical.

Exit status: 0 when every high or critical advisory is listed and every entry
is live; 1 when one is not, naming each; 2 when the report or the list cannot
be read, npm runs past NPM_AUDIT_TIMEOUT_SECONDS, or a package npm flags high or
critical reaches no such advisory, which is never a pass. Every run says how
many advisories it judged.
"""

import json
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DEFAULT_ALLOWED = os.path.join(ROOT, "scripts", "npm-audit-allowed.json")
GATING = {"high", "critical"}
# How long `npm audit` may run before the check reports it could not read a
# report (BE-TIMEOUT-001): a registry that never answers fails the step in this
# bound, not at the job's timeout-minutes.
NPM_AUDIT_TIMEOUT_SECONDS = 300
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
        try:
            run = subprocess.run(
                command, cwd=ROOT, capture_output=True, text=True,
                timeout=NPM_AUDIT_TIMEOUT_SECONDS,
            )
        except subprocess.TimeoutExpired as err:
            raise ValueError(f"npm audit ran past {NPM_AUDIT_TIMEOUT_SECONDS} seconds") from err
        text = run.stdout
    report = json.loads(text)
    if not isinstance(report, dict) or not isinstance(report.get("vulnerabilities"), dict):
        raise ValueError("it holds no `vulnerabilities` map")
    for package, vuln in report["vulnerabilities"].items():
        if not isinstance(vuln, dict) or not isinstance(vuln.get("via"), list):
            raise ValueError(f"`{package}` has no `via` list")
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


def fixable(fix):
    """Whether npm reports a fix that is not a breaking change."""
    return fix is True or (isinstance(fix, dict) and not fix.get("isSemVerMajor"))


def reached(package, vulnerabilities, seen=None):
    """The advisory IDs a flagged package reaches: its own, and through each
    package its `via` strings name, theirs."""
    seen = set() if seen is None else seen
    if package in seen or package not in vulnerabilities:
        return set()
    seen.add(package)
    found = set()
    for via in vulnerabilities[package]["via"]:
        if isinstance(via, dict):
            found.add(advisory_id(via))
        else:
            found |= reached(via, vulnerabilities, seen)
    return found


def judge(report, allowed):
    """The failures, the advisories judged and those listed; raises ValueError
    for a report this check cannot read."""
    vulnerabilities = report["vulnerabilities"]
    # Each advisory npm reports, at every package it names it on directly; a
    # package flagged only through another (a string `via`) inherits that
    # one's.
    advisories = {}
    for package, vuln in vulnerabilities.items():
        for via in vuln["via"]:
            if not isinstance(via, dict):
                continue
            ident = advisory_id(via)
            if ident is None:
                raise ValueError(f"an advisory on `{package}` names no advisory ID: {via!r}")
            advisories.setdefault(ident, []).append({
                "package": package,
                "severity": via.get("severity"),
                "range": via.get("range"),
                "fix": vuln.get("fixAvailable"),
                "title": via.get("title", ""),
            })

    # A package npm flags at a gating severity must reach a gating advisory,
    # or the report holds something this check did not read.
    gating = {ident for ident, seen in advisories.items() if any(s["severity"] in GATING for s in seen)}
    for package, vuln in vulnerabilities.items():
        if vuln.get("severity") in GATING and not reached(package, vulnerabilities) & gating:
            raise ValueError(f"`{package}` is {vuln.get('severity')} but reaches no high or critical advisory")

    failures, listed = [], 0
    for ident in sorted(gating):
        seen = advisories[ident]
        first = seen[0]
        entry = allowed.get(ident)
        if entry is None:
            failures.append(
                f"{ident} ({first['severity']}, {first['package']} {first['range']}): {first['title']} — "
                "update or replace the dependency, or list it with why it does not reach this repository"
            )
        elif any(fixable(s["fix"]) for s in seen):
            fix = next(s["fix"] for s in seen if fixable(s["fix"]))
            failures.append(f"{ident} is listed, but npm reports a fix that is not a breaking change ({fix!r}): apply it and remove the entry")
        elif any(s["package"] != entry["package"] or s["range"] != entry["range"] for s in seen):
            reported = ", ".join(f"{s['package']} {s['range']}" for s in seen)
            failures.append(f"{ident} is listed for {entry['package']} {entry['range']}, but npm now reports {reported}: judge the entry again")
        else:
            listed += 1
    for ident in sorted(set(allowed) - set(advisories)):
        failures.append(f"{ident} is listed but no longer reported: remove the entry")
    for ident in sorted((set(allowed) & set(advisories)) - gating):
        failures.append(f"{ident} is listed but no longer high or critical: remove the entry")
    return failures, len(gating), listed


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
        failures, judged, listed = judge(report, allowed)
    except (OSError, ValueError, TypeError, KeyError, AttributeError) as err:
        return fail_to_run(f"the npm audit report cannot be read: {err}")

    print(
        f"check-npm-audit: {judged} high or critical advisory(ies) reported, "
        f"{listed} listed in {os.path.relpath(args['allowed'], ROOT)}, {len(failures)} failing"
    )
    for failure in failures:
        print(f"  {failure}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
