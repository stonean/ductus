#!/usr/bin/env python3
"""Split the rules an analysis judges into judge shares, and the design
subjects into scenario groups, and write the cells manifest the later
scripts read (spec 060, scenario rule-judging-runs-from-repo-scripts).

usage: python3 -I scripts/analyze/split.py FEATURE
           [--share-bytes N] [--cell-bytes N] [--rules-json FILE]

The rules are those discover-rule-files' verdicts.judge lists (spec 082),
from `discover-rule-files --all --rules --verification --feature FEATURE`,
or from FILE, a result saved from that call. Each verdicts.reusable entry is
kept in the manifest for record.py to report again, unjudged.

Shares keep each rule file whole and are balanced by the bytes of their
rendered text: the fewest that keep each share within --share-bytes
(default lib.DEFAULT_SHARE_BYTES). A cell is one share read beside the spec,
plan and data model and a group of scenarios; when the subjects do not fit
one cell's --cell-bytes (default lib.DEFAULT_CELL_BYTES) the scenarios are
split into the fewest groups balanced by bytes that keep each cell within
it, and every share is judged against every group. A share or cell that
cannot be brought within its budget, a rule file or a scenario larger than
it, is named in a note. Step 15 has its own reader.

Needs detect.py's results, current for the feature as it stands, whose
check-departures entries the briefs name. Prints discover-rule-files'
notice lines. Writes <work>/manifest.json and <work>/share-<X>.md beside it.

Exit status: 0 written; 1 refused (a judged rule in no loaded file, a rule
file that could not be read, detection older than an edit or a commit); 2 an input
could not be read.
"""

import math
import os
import sys
import time

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import analyze_lib as lib  # noqa: E402

def render_file(path, rules, other_headings):
    out = [f"# FILE {path}",
           f"(other-headings in this file, i.e. headings that open no rule: {len(other_headings)})"]
    for h in other_headings:
        out.append(f"- other-heading line {h.get('line')}: {h.get('heading')!r}")
        for line in (h.get("body") or "").strip("\n").splitlines():
            out.append(f"    | {line}")
    out.append("")
    for r in rules:
        out += [f"## {r['id']} — tier: {r.get('tier') or 'none'} — heading: {r.get('heading')}", "",
                "**Statement:**", "", (r.get("statement") or "(no Statement)").strip(), "",
                "**Verification:**", "", (r.get("verification") or "(no Verification)").strip(), ""]
    return "\n".join(out) + "\n"


def balance(items, bins):
    """Greedy: each (key, size) item, largest first, into the lightest bin."""
    out = [[] for _ in range(bins)]
    loads = [0] * bins
    for key, size in sorted(items, key=lambda x: (-x[1], x[0])):
        i = loads.index(min(loads))
        out[i].append(key)
        loads[i] += size
    return out, loads


def fit(items, bins, limit, extra=0):
    """The fewest bins from `bins` up, one per item at most, whose loads plus
    extra stay within limit; with the loads, for a note when even that
    cannot."""
    bins = max(1, min(bins, len(items)))
    while True:
        out, loads = balance(items, bins)
        if max(loads) + extra <= limit or bins >= len(items):
            return out, loads
        bins += 1


def long_lines(path):
    with open(path, encoding="utf-8") as f:
        return [n for n, line in enumerate(f, 1) if len(line.rstrip("\n")) > lib.LONG_LINE]


def budget_flag(args, flag):
    """The flag's value as a whole number of at least lib.MIN_BUDGET_BYTES;
    anything else exits 2 naming the flag, the value and the bound."""
    raw = args[flag]
    try:
        n = int(raw)
    except ValueError:
        n = None
    if n is None or n < lib.MIN_BUDGET_BYTES:
        raise lib.Unreadable(f"{flag} is {raw!r}; it takes a whole number of at least {lib.MIN_BUDGET_BYTES}")
    return n


def run(argv):
    args = {"--share-bytes": str(lib.DEFAULT_SHARE_BYTES), "--cell-bytes": str(lib.DEFAULT_CELL_BYTES),
            "--rules-json": None}
    if not argv or argv[0].startswith("--"):
        raise lib.Unreadable("usage: split.py FEATURE [--share-bytes N] [--cell-bytes N] [--rules-json FILE]")
    feature, rest = argv[0], argv[1:]
    while rest:
        if rest[0] not in args or len(rest) < 2:
            raise lib.Unreadable(f"unknown or valueless option {rest[0]}")
        args[rest[0]] = rest[1]
        rest = rest[2:]
    share_bytes, cell_bytes = budget_flag(args, "--share-bytes"), budget_flag(args, "--cell-bytes")
    d = lib.feature_dir(feature)
    w = lib.work_dir(feature)
    lib.check_detection_current(feature, os.path.join(w, "det"))
    departures_path = os.path.join(w, "det", "check-departures.json")
    departures = lib.read_json(departures_path).get("entries")
    if not isinstance(departures, list):
        raise lib.Unreadable("check-departures' result has no entries list")
    spec_path = os.path.join(w, "det", "read-spec.json")
    status = (lib.read_json(spec_path).get("frontmatter") or {}).get("status") \
        if os.path.isfile(spec_path) else None

    if args["--rules-json"]:
        found = lib.whole(lib.read_json(args["--rules-json"]))
    else:
        found, _ = lib.call("discover-rule-files", "--all", "--rules", "--verification",
                            "--feature", feature)
    verdicts = found.get("verdicts")
    if isinstance(verdicts, dict) and "judge" not in verdicts and "judge" in found:
        # Past the read size the lists file holds verdicts' two lists at its top level.
        verdicts = {**verdicts, "judge": found["judge"], "reusable": found.get("reusable") or []}
    if not isinstance(verdicts, dict) or "judge" not in verdicts:
        raise lib.Unreadable("discover-rule-files returned no verdicts.judge; pass --feature with --all and --rules")
    if found.get("unreadable"):
        raise lib.Refused(f"rule files could not be read: {found['unreadable']}; their rules are "
                          "neither judged nor reusable, so the run cannot cover them")
    for note in found.get("notices") or []:
        print(f"notice: {note}")
    judge = set(verdicts["judge"])
    rules_dir = found.get("rules-dir") or ""
    files, placed = [], set()
    for rf in found.get("rule-files") or []:
        rules = [r for r in rf.get("rules") or [] if r.get("id") in judge]
        placed |= {r["id"] for r in rules}
        if rules:
            text = render_file(rf["file"], rules, rf.get("other-headings") or [])
            files.append({"file": rf["file"], "rules": rules, "text": text,
                          "bytes": len(text.encode("utf-8"))})
    missing = sorted(judge - placed)
    if missing:
        raise lib.Refused(f"verdicts.judge names rules no loaded file holds: {missing}")

    notes = []
    total = sum(f["bytes"] for f in files)
    groups_of_files = []
    if files:
        groups_of_files, loads = fit([(f["file"], f["bytes"]) for f in files],
                                     math.ceil(total / share_bytes), share_bytes)
        if max(loads) > share_bytes:
            notes.append(f"a share holds {max(loads)} bytes, over --share-bytes {share_bytes}: "
                         "a rule file is larger than a share")
    by_file = {f["file"]: f for f in files}
    shares = {}
    for i, names in enumerate(groups_of_files):
        letter = chr(ord("A") + i)
        names = sorted(names)
        text = f"# Rule share {letter}\n\n" + "\n".join(by_file[n]["text"] for n in names)
        share_file = os.path.join(w, f"share-{letter}.md")
        with open(share_file, "w", encoding="utf-8") as f:
            f.write(text)
        ids = [r["id"] for n in names for r in by_file[n]["rules"]]
        shares[letter] = {"file": share_file, "bytes": os.path.getsize(share_file),
                          "rule-files": names, "ids": ids,
                          "tiers": {r["id"]: r.get("tier") for n in names for r in by_file[n]["rules"]},
                          "long-lines": long_lines(share_file)}

    subjects = lib.design_subjects(feature)
    core = [(p, b) for p, b in subjects if not p.startswith("scenarios/")]
    scen = [(p, b) for p, b in subjects if p.startswith("scenarios/")]
    core_bytes, scen_bytes = sum(b for _, b in core), sum(b for _, b in scen)
    max_share = max((s["bytes"] for s in shares.values()), default=0)
    if not scen or core_bytes + max_share + scen_bytes <= cell_bytes:
        groups = [[p for p, _ in scen]]
    else:
        budget = max(1, cell_bytes - core_bytes - max_share)
        parts, loads = fit(scen, math.ceil(scen_bytes / budget), cell_bytes, core_bytes + max_share)
        groups = [sorted(g, key=str.lower) for g in parts if g]
        if max(loads) + core_bytes + max_share > cell_bytes:
            notes.append(f"a cell holds {max(loads) + core_bytes + max_share} bytes, over --cell-bytes "
                         f"{cell_bytes}: the spec, plan and data model ({core_bytes}) beside the largest "
                         f"share ({max_share}) and one scenario group do not fit")

    run_id = time.strftime("%Y%m%dT%H%M%SZ", time.gmtime())
    sizes = dict(subjects)
    cells = []
    for letter in shares:
        for gi, group in enumerate(groups, 1):
            cell = letter if len(groups) == 1 else f"{letter}{gi}"
            subj = [p for p, _ in core] + group
            cells.append({"cell": cell, "share": letter, "group": gi, "subjects": subj,
                          "bytes": shares[letter]["bytes"] + sum(sizes[p] for p in subj),
                          "brief": os.path.join(w, f"brief-{cell}.md"),
                          "out": os.path.join(w, f"out-{cell}-{run_id}.json")})
    s15 = [p for p in ("spec.md", "plan.md") if p in sizes]
    system = "specs/system.md" if os.path.isfile("specs/system.md") else None
    head = lib.head()
    manifest = {
        "feature": feature, "status": status, "head": head, "run": run_id,
        "created": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "work": w,
        "rules-dir": rules_dir, "loaded": [os.path.basename(s) for s in found.get("selected") or []],
        "judge": sorted(judge), "reusable": verdicts.get("reusable") or [],
        "verdicts-reason": verdicts.get("reason"),
        "departures": departures, "shares": shares, "groups": groups, "cells": cells,
        "subjects": {p: {"bytes": b, "sha256": lib.sha256(os.path.join(d, p)),
                         "long-lines": long_lines(os.path.join(d, p))} for p, b in subjects},
        "step15": {"subjects": s15, "system": system,
                   "brief": os.path.join(w, "brief-step15.md"),
                   "out": os.path.join(w, f"out-step15-{run_id}.json")},
        "notes": notes,
    }
    lib.write_json(lib.manifest_path(feature), manifest)
    print(f"== {feature}: {len(judge)} rules to judge, {len(manifest['reusable'])} reusable verdicts "
          f"({verdicts.get('reason') or 'current'}); {len(shares)} shares x {len(groups)} scenario groups "
          f"= {len(cells)} cells, plus a step-15 reader")
    for letter, s in shares.items():
        print(f"  share {letter}: {len(s['ids'])} rules, {s['bytes']} bytes, {', '.join(s['rule-files'])}")
    for c in cells:
        print(f"  cell {c['cell']}: {c['bytes']} bytes, {len(c['subjects'])} subjects")
    for n in notes:
        print(f"  note: {n}")
    print(f"manifest: {lib.manifest_path(feature)}")


if __name__ == "__main__":
    lib.main(run)
