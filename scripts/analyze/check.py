#!/usr/bin/env python3
"""Check every judge's and the step-15 reader's hand-back before it is used
(spec 060, scenario rule-judging-runs-from-repo-scripts;
framework/reference/analyze.md, Rule judging in a subagent).

usage: python3 -I scripts/analyze/check.py FEATURE

For each cell in the manifest it checks that:

- the output file exists, parses, and was written after the manifest;
- every ID of the cell's share has exactly one answer, judged or unjudged,
  and every verdict is of a judged ID;
- each verdict is well formed: addressed with a cite, finding with an at
  and a message, waived only for a rule with a departure entry, and a
  departure's rule never addressed or a finding;
- each cite and at resolves to a heading of a design subject, by
  read-section's exact match, the one write-analysis resolves with;
- the read account names every subject and the share file, each read
  whole, at its size on disk now;
- no design subject changed since split.py measured it.

The step-15 hand-back is checked for its read account and for the three
integrity items. Writes <work>/check.json.

Exit status: 0 when every hand-back passes; 1 naming each failure, so the
host asks that judge to finish (SendMessage) and runs this again; 2 when
the manifest cannot be read.
"""

import json
import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import analyze_lib as lib  # noqa: E402

KINDS = ("addressed", "finding", "waived")


def read_out(path, created_epoch):
    if not os.path.isfile(path):
        return None, "no output file"
    if os.path.getmtime(path) < created_epoch:
        return None, "output file predates this run's manifest"
    try:
        with open(path, encoding="utf-8") as f:
            return json.load(f), None
    except (OSError, ValueError) as e:
        return None, f"output does not parse: {e}"


def read_account(out, expected):
    """Problems with the bytes-read account for the repo-relative paths in
    expected: each present, read whole, at its size on disk now."""
    acct = out.get("bytes-read") if isinstance(out.get("bytes-read"), dict) else {}
    problems = []
    for p in expected:
        entry = acct.get(p)
        size = os.path.getsize(p) if os.path.isfile(p) else None
        if not (isinstance(entry, list) and len(entry) == 2):
            problems.append(f"no read account for {p}")
        elif entry[1] != size:
            problems.append(f"{p} reported at {entry[1]} bytes, but is {size}")
        elif entry[0] < size:
            problems.append(f"{p} read {entry[0]} of {size} bytes")
    return problems


def check(feature, m):
    """(problems by cell, outputs by cell, unresolved cites, notes by cell).

    A cell with no problem has []. An addressed verdict whose cite resolves
    to no heading is a problem unless another cell of its share cites a
    heading that resolves for the same rule, which combine.py then takes: a
    note, never a pass for the unresolved cite itself.
    """
    d = lib.feature_dir(feature)
    created = _epoch(m["created"])
    problems, outs = {}, {}
    for p, info in m["subjects"].items():
        path = os.path.join(d, p)
        if lib.sha256(path) != info["sha256"]:
            problems.setdefault("subjects", []).append(
                f"{p} changed since split.py measured it; re-run split.py and briefs.py, "
                f"then re-judge (docs/developing-ductus.md, Sizing an analysis's judges)")
    departures = {e.get("rule") for e in m.get("departures") or []}
    cites = {}
    for c in m["cells"]:
        cell = c["cell"]
        out, err = read_out(c["out"], created)
        if err:
            problems[cell] = [err]
            continue
        outs[cell] = out
        ps = []
        ids = set(m["shares"][c["share"]]["ids"])
        judged = set(out.get("judged") or [])
        unjudged = {u.get("id") for u in out.get("unjudged") or [] if isinstance(u, dict)}
        if judged & unjudged:
            ps.append(f"both judged and unjudged: {sorted(judged & unjudged)}")
        if (judged | unjudged) != ids:
            if ids - judged - unjudged:
                ps.append(f"no answer for {sorted(ids - judged - unjudged)}")
            if (judged | unjudged) - ids:
                ps.append(f"answers for rules outside the share: {sorted((judged | unjudged) - ids)}")
        verdicts = out.get("verdicts") if isinstance(out.get("verdicts"), dict) else {}
        for rid, v in sorted(verdicts.items()):
            kind = v.get("verdict") if isinstance(v, dict) else None
            if rid not in judged:
                ps.append(f"{rid}: a verdict for a rule not in judged")
            if kind not in KINDS:
                ps.append(f"{rid}: verdict {kind!r} is none of {KINDS}")
            elif kind == "waived" and rid not in departures:
                ps.append(f"{rid}: waived, but the spec holds no departure entry for it")
            elif kind != "waived" and rid in departures:
                ps.append(f"{rid}: {kind}, but its departure entry makes a fired one waived")
            elif kind == "addressed" and not v.get("cite"):
                ps.append(f"{rid}: addressed with no cite")
            elif kind == "finding" and not (v.get("at") and v.get("message")):
                ps.append(f"{rid}: a finding needs an at and a message")
            ref = v.get("cite") if kind == "addressed" else v.get("at") if kind == "finding" else None
            if ref:
                cites.setdefault(ref, []).append((cell, rid))
        expected = [f"{d}/{p}" for p in c["subjects"]] + [m["shares"][c["share"]]["file"]]
        ps += read_account(out, [p for p in expected if not os.path.isabs(p)])
        share_file = m["shares"][c["share"]]["file"]
        acct = out.get("bytes-read") if isinstance(out.get("bytes-read"), dict) else {}
        share_entry = acct.get(share_file) or acct.get(os.path.basename(share_file))
        size = os.path.getsize(share_file)
        if not (isinstance(share_entry, list) and len(share_entry) == 2 and share_entry[0] >= size):
            ps.append(f"the share file was not reported read whole ({size} bytes)")
        problems[cell] = ps
    bad = set(lib.unresolved_cites(feature, list(cites)))
    share_of = {c["cell"]: c["share"] for c in m["cells"]}
    resolving = {(share_of[cell], rid) for ref, uses in cites.items() if ref not in bad
                 for cell, rid in uses if outs[cell]["verdicts"][rid].get("verdict") == "addressed"}
    notes = {}
    for ref in sorted(bad):
        for cell, rid in cites[ref]:
            line = f"{rid}: {ref} names no heading of a design subject"
            if outs[cell]["verdicts"][rid].get("verdict") == "addressed" \
                    and (share_of[cell], rid) in resolving:
                notes.setdefault(cell, []).append(line + "; another cell's cite resolves")
            else:
                problems[cell].append(line)
    s = m["step15"]
    out, err = read_out(s["out"], created)
    if err:
        problems["step15"] = [err]
    else:
        outs["step15"] = out
        expected = [f"{d}/{p}" for p in s["subjects"]] + ([s["system"]] if s.get("system") else [])
        ps = read_account(out, expected)
        checked = out.get("integrity-checked") if isinstance(out.get("integrity-checked"), dict) else {}
        for item in ("code-block", "rationale", "system"):
            if item not in checked:
                ps.append(f"integrity item {item} has no checked state")
        for key in ("grounding", "integrity", "decision-reasons"):
            if not isinstance(out.get(key), list):
                ps.append(f"{key} is not a list")
        problems["step15"] = ps
    return problems, outs, bad, notes


def _epoch(stamp):
    import calendar
    import time
    return calendar.timegm(time.strptime(stamp, "%Y-%m-%dT%H:%M:%SZ"))


def run(argv):
    if len(argv) != 1:
        raise lib.Unreadable("usage: check.py FEATURE")
    feature = argv[0]
    m = lib.load_manifest(feature)
    problems, outs, _, notes = check(feature, m)
    lib.write_json(os.path.join(lib.work_dir(feature), "check.json"), problems)
    failed = {k: v for k, v in problems.items() if v}
    for c in [x["cell"] for x in m["cells"]] + ["step15"]:
        o = outs.get(c) or {}
        if c == "step15":
            counts = {k: len(o.get(k) or []) for k in ("grounding", "integrity", "decision-reasons")}
        else:
            kinds = [v.get("verdict") for v in (o.get("verdicts") or {}).values() if isinstance(v, dict)]
            counts = {"judged": len(o.get("judged") or []), "unjudged": len(o.get("unjudged") or []),
                      **{k: kinds.count(k) for k in KINDS},
                      "malformed-headings": len(o.get("malformed-headings") or [])}
        print(f"{c}: {'FAIL' if problems.get(c) else 'ok'} {counts}")
        for p in problems.get(c) or []:
            print(f"   {p}")
        for n in notes.get(c) or []:
            print(f"   note: {n}")
    for p in problems.get("subjects") or []:
        print(f"subjects: {p}")
    if failed:
        raise lib.Refused(f"{len(failed)} hand-back(s) failed: {', '.join(sorted(failed))}")


if __name__ == "__main__":
    lib.main(run)
