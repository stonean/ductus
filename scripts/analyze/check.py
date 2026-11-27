#!/usr/bin/env python3
"""Check every judge's, the step-15 reader's and each scenario reader's
hand-back before it is used (spec 060, scenario
rule-judging-runs-from-repo-scripts; framework/reference/analyze.md, Rule
judging in a subagent).

usage: python3 -I scripts/analyze/check.py FEATURE

It refuses at once, before reading any hand-back, when a loaded rule file
has changed or is gone since split.py ran, since what the judges judged is
then not the rule as it stands. For each
cell in the manifest it checks that:

- the output file exists, parses, and was written after the manifest;
- its fields have the shapes the brief asks for;
- every ID of the cell's share has exactly one answer, judged or unjudged,
  and every verdict is of a judged ID;
- each verdict is well formed: addressed with a cite, finding with an at
  and a message, waived only for a rule with a departure entry, and a
  departure's rule never addressed or a finding;
- each cite and at resolves to a heading of a design subject, by
  read-section's exact match, the one write-analysis resolves with. An
  addressed verdict whose cite does not resolve is a note rather than a
  failure when another cell of its share cites a heading that resolves for
  the same rule, which combine.py takes;
- the read account names every subject and the share file, each read
  whole, at its size on disk now;
- the design subjects are the ones split.py measured: none edited, added
  or removed since.

The step-15 hand-back is checked for its read account, each part of
specs/system.md split.py listed to read included, and its shape. An
integrity item it reports unchecked is printed, and record.py records it
under integrity-not-checked. Each scenario reader's hand-back is checked for
its shape, a judgment of every scenario of its group and of none outside
it, each finding at one of them with a check of context or behavior and at
most one for each scenario and check, and its read account, every scenario
of its group read whole (framework/reference/analyze.md, Scenario
consistency (advisory)). Each judge's malformed-headings are printed, and
record.py refuses them until the plan file names each. Writes
<work>/check.json.

Exit status: 0 when every hand-back passes; 1 naming each failure, so the
host asks that judge to finish (SendMessage) and runs this again, or naming
the rule file that changed, which needs detect.py, split.py, briefs.py and
the judges again; 2 when the manifest cannot be read.
"""

import calendar
import json
import os
import sys
import time

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import analyze_lib as lib  # noqa: E402

INTEGRITY = ("code-block", "rationale", "system", "contract")
SCENARIO_CHECKS = ("context", "behavior")


def read_out(path, created_epoch):
    if not os.path.isfile(path):
        return None, "no output file"
    if os.path.getmtime(path) < created_epoch:
        return None, "output file predates this run's manifest"
    try:
        with open(path, encoding="utf-8") as f:
            out = json.load(f)
    except (OSError, ValueError) as e:
        return None, f"output does not parse: {e}"
    if not isinstance(out, dict):
        return None, "output is not a JSON object"
    return out, None


def shape(out, kind="cell"):
    """Problems with the hand-back's field shapes, found before any is used:
    kind is a judge's cell, step15 or scenarios."""
    ps = []
    acct = out.get("bytes-read")
    if not isinstance(acct, dict) or not all(
            isinstance(v, list) and len(v) == 2 and all(isinstance(n, int) for n in v)
            for v in acct.values()):
        ps.append("bytes-read is not a map of path to [bytes read, size]")
    if kind == "scenarios":
        if not isinstance(out.get("judged"), list) or not all(isinstance(x, str) for x in out["judged"]):
            ps.append("judged is not a list of scenario paths")
        found = out.get("findings")
        if not isinstance(found, list) or not all(
                isinstance(f, dict) and all(isinstance(f.get(k), str) for k in ("path", "check", "message"))
                for f in found):
            ps.append("findings is not a list of {path, check, message}")
        return ps
    if kind == "step15":
        for key in ("grounding", "integrity", "decision-reasons"):
            if not isinstance(out.get(key), list) or not all(isinstance(x, dict) for x in out[key]):
                ps.append(f"{key} is not a list of objects")
        checked = out.get("integrity-checked")
        if not isinstance(checked, dict) or not all(isinstance(checked.get(i), bool) for i in INTEGRITY):
            ps.append(f"integrity-checked does not give each of {INTEGRITY} true or false")
        return ps
    if not isinstance(out.get("judged"), list) or not all(isinstance(x, str) for x in out["judged"]):
        ps.append("judged is not a list of rule IDs")
    unjudged = out.get("unjudged", [])
    if not isinstance(unjudged, list) or not all(
            isinstance(u, dict) and isinstance(u.get("id"), str) for u in unjudged):
        ps.append("unjudged is not a list of {id, reason}")
    verdicts = out.get("verdicts", {})
    if not isinstance(verdicts, dict) or not all(
            isinstance(v, dict) and all(isinstance(v.get(k, ""), str)
                                        for k in ("verdict", "cite", "at", "message"))
            for v in verdicts.values()):
        ps.append("verdicts is not a map of rule ID to {verdict, cite | at and message}")
    if not isinstance(out.get("malformed-headings", []), list):
        ps.append("malformed-headings is not a list")
    return ps


def read_account(out, expected):
    """Problems with the bytes-read account for the paths in expected: each
    present, read whole, at its size on disk now. An absolute path, the share
    file, may be reported under its basename."""
    acct = out["bytes-read"]
    problems = []
    for p in expected:
        entry = acct.get(p)
        if entry is None and os.path.isabs(p):
            entry = acct.get(os.path.basename(p))
        size = os.path.getsize(p) if os.path.isfile(p) else None
        if entry is None:
            problems.append(f"no read account for {p}")
        elif entry[1] != size:
            problems.append(f"{p} reported at {entry[1]} bytes, but is {size}")
        elif entry[0] < size:
            problems.append(f"{p} read {entry[0]} of {size} bytes")
    return problems


def check(feature, m):
    """{problems, outs, unresolved, notes, malformed, unchecked}: problems and
    notes by cell, a cell with no problem holding []; outputs by cell; the
    cites that resolve to no heading; each cell's malformed headings; and
    the integrity items step 15 left unchecked."""
    d = lib.valid_feature(feature)
    created = calendar.timegm(time.strptime(m["created"], "%Y-%m-%dT%H:%M:%SZ"))
    problems, outs, notes, malformed = {}, {}, {}, {}
    now = dict(lib.design_subjects(feature))
    for p in sorted(set(now) | set(m["subjects"])):
        if p not in m["subjects"]:
            problems.setdefault("subjects", []).append(f"{p} was added since split.py ran, so no judge read it")
        elif p not in now:
            problems.setdefault("subjects", []).append(f"{p} was removed since split.py ran")
        elif lib.sha256(os.path.join(d, p)) != m["subjects"][p]["sha256"]:
            problems.setdefault("subjects", []).append(f"{p} changed since split.py measured it")
    if problems.get("subjects"):
        problems["subjects"].append("re-run split.py and briefs.py, then re-judge "
                                    "(.ductus/docs/agents/developing-ductus.md, Sizing an analysis's judges)")
    departures = lib.departure_rules(m)
    cites = {}
    for c in m["cells"]:
        cell = c["cell"]
        out, err = read_out(c["out"], created)
        ps = [err] if err else shape(out)
        problems[cell] = ps
        if ps:
            continue
        outs[cell] = out
        ids = set(m["shares"][c["share"]]["ids"])
        judged = set(out["judged"])
        unjudged = {u["id"] for u in out.get("unjudged", [])}
        if judged & unjudged:
            ps.append(f"both judged and unjudged: {sorted(judged & unjudged)}")
        if ids - judged - unjudged:
            ps.append(f"no answer for {sorted(ids - judged - unjudged)}")
        if (judged | unjudged) - ids:
            ps.append(f"answers for rules outside the share: {sorted((judged | unjudged) - ids)}")
        for rid, v in sorted(out.get("verdicts", {}).items()):
            kind = v.get("verdict")
            if rid not in judged:
                ps.append(f"{rid}: a verdict for a rule not in judged")
            problem = lib.verdict_problem(rid, v, departures)
            if problem:
                ps.append(problem)
            ref = v.get("cite") if kind == "addressed" else v.get("at") if kind == "finding" else None
            if ref:
                cites.setdefault(ref, []).append((cell, rid))
        ps += read_account(out, [f"{d}/{p}" for p in c["subjects"]] + [m["shares"][c["share"]]["file"]])
        if out.get("malformed-headings"):
            malformed[cell] = [str(x) for x in out["malformed-headings"]]
    bad = set(lib.unresolved_cites(feature, list(cites)))
    share_of = {c["cell"]: c["share"] for c in m["cells"]}
    resolving = {(share_of[cell], rid) for ref, uses in cites.items() if ref not in bad
                 for cell, rid in uses if outs[cell]["verdicts"][rid].get("verdict") == "addressed"}
    for ref in sorted(bad):
        for cell, rid in cites[ref]:
            line = f"{rid}: {ref} names no heading of a design subject"
            if outs[cell]["verdicts"][rid].get("verdict") == "addressed" \
                    and (share_of[cell], rid) in resolving:
                notes.setdefault(cell, []).append(line + "; another cell's cite resolves")
            else:
                problems[cell].append(line)
    unchecked = []
    s = m["step15"]
    out, err = read_out(s["out"], created)
    ps = [err] if err else shape(out, "step15")
    if not ps:
        outs["step15"] = out
        ps = read_account(out, [f"{d}/{p}" for p in s["subjects"]] + ([s["system"]] if s.get("system") else [])
                          + list(s.get("system-parts") or []))
        unchecked = [i for i in INTEGRITY if out["integrity-checked"][i] is False]
        for i in unchecked:
            notes.setdefault("step15", []).append(
                f"integrity item {i} was not checked; record.py records it under integrity-not-checked")
    problems["step15"] = ps
    for r in m["scenario-readers"]:
        problems[r["reader"]] = scenario_problems(d, r, created, outs)
    return {"problems": problems, "outs": outs, "unresolved": bad, "notes": notes,
            "malformed": malformed, "unchecked": unchecked}


def scenario_problems(d, r, created, outs):
    """Problems with scenario reader r's hand-back, which is put in outs
    under its name once its shape passes: every scenario of its group judged
    and read whole, none outside it, and each finding at one of them, about
    its context or behavior, once for each scenario and check."""
    out, err = read_out(r["out"], created)
    ps = [err] if err else shape(out, "scenarios")
    if ps:
        return ps
    outs[r["reader"]] = out
    group, judged = set(r["subjects"]), set(out["judged"])
    if group - judged:
        ps.append(f"no judgment of {sorted(group - judged)}")
    if judged - group:
        ps.append(f"judgments of scenarios outside this reader's group: {sorted(judged - group)}")
    seen = set()
    for f in out["findings"]:
        at = (f["path"], f["check"])
        if f["path"] not in group:
            ps.append(f"a finding at {f['path']}, outside this reader's group")
        if f["check"] not in SCENARIO_CHECKS:
            ps.append(f"{f['path']}: check {f['check']!r} is none of {SCENARIO_CHECKS}")
        if not f["message"].strip():
            ps.append(f"{f['path']}: a finding needs a message")
        if at in seen:
            ps.append(f"{f['path']}: two {f['check']} findings")
        seen.add(at)
    return ps + read_account(out, [f"{d}/{p}" for p in r["subjects"]])


def failures(result):
    return {k: v for k, v in result["problems"].items() if v}


def run(argv):
    if len(argv) != 1:
        raise lib.Unreadable("usage: check.py FEATURE")
    feature = argv[0]
    m = lib.load_manifest(feature)
    lib.check_rule_files_current(feature, m)
    result = check(feature, m)
    lib.write_json(os.path.join(lib.work_dir(feature), "check.json"),
                   {k: result[k] for k in ("problems", "notes", "malformed", "unchecked")})
    outs, problems = result["outs"], result["problems"]
    readers = [r["reader"] for r in m["scenario-readers"]]
    for c in [x["cell"] for x in m["cells"]] + ["step15"] + readers:
        o = outs.get(c) or {}
        if c == "step15":
            counts = {k: len(o.get(k) or []) for k in ("grounding", "integrity", "decision-reasons")}
        elif c in readers:
            counts = {k: len(o.get(k) or []) for k in ("judged", "findings")}
        else:
            kinds = [v.get("verdict") for v in (o.get("verdicts") or {}).values()]
            counts = {"judged": len(o.get("judged") or []), "unjudged": len(o.get("unjudged") or []),
                      **{k: kinds.count(k) for k in lib.VERDICT_KINDS}}
        print(f"{c}: {'FAIL' if problems.get(c) else 'ok'} {counts}")
        for p in problems.get(c) or []:
            print(f"   {p}")
        for n in result["notes"].get(c) or []:
            print(f"   note: {n}")
        for h in result["malformed"].get(c) or []:
            print(f"   malformed heading reported: {h}")
    for p in problems.get("subjects") or []:
        print(f"subjects: {p}")
    if failures(result):
        raise lib.Refused(f"{len(failures(result))} hand-back(s) failed: {', '.join(sorted(failures(result)))}")


if __name__ == "__main__":
    lib.main(run)
