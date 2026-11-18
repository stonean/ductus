#!/usr/bin/env python3
"""Record an analysis: step 18's process-decisions and step 21's
write-analysis for one feature, through the runtime (spec 060, scenario
rule-judging-runs-from-repo-scripts).

usage: python3 -I scripts/analyze/record.py FEATURE PLAN.json [--write]

PLAN.json holds what the host decided:

  {"findings": [...],       the run's other findings, from detect.py,
                            step 14, step 15 and the host's own items, each
                            {tier, family, message, path, live, and
                            optionally disposition and decision-key}
   "settled": {RULE: verdict or null},
                            the host's verdict for each rule combine.py left
                            unsettled (null for not fired), and any it
                            overrides, each printed
   "dispositions": {KEY: disposition},
                            a disposition for a rule finding, by its key
   "unexamined": {REASON: N}}
                            targets the host could not judge, beside those
                            detect.py's results name

Each fired rule's finding is written as `rule-assessment — RULE: message`,
in the Blocking tier for a MUST-tier rule and the Advisory tier otherwise,
from merged.json and from each verdicts.reusable finding. A finding a
stored decision matches takes its stored outcome. decided-by is the
repository's `git config user.email`.

Without --write it prints the record it would write and the write-analysis
call; with --write it makes the call.

Exit status: 0 written, or the dry run printed; 1 refused: a hand-back
fails its check, a rule is unsettled, a design subject changed, no
decided-by, or write-analysis refused; 2 an input could not be read.
"""

import collections
import os
import subprocess
import sys
import time

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import analyze_lib as lib  # noqa: E402
import check as checker  # noqa: E402


def key(f):
    return f.get("decision-key") or f"{f['family']} — {f['message']}"


def rule_finding(feature, rid, v, tier):
    path = v["at"].partition("#")[0]
    return {"tier": "blocking" if tier == "must" else "advisory", "family": "rule-assessment",
            "message": f"{rid}: {v['message']}", "path": f"specs/{feature}/{path}", "live": True}


def unexamined(det):
    def load(name):
        p = os.path.join(det, f"{name}.json")
        if not os.path.isfile(p):
            raise lib.Unreadable(f"{p} does not exist; run detect.py first")
        return lib.read_json(p)
    un = collections.Counter()
    for s in load("check-artifacts").get("skipped") or []:
        un[s["reason"]] += 1
    for ref in load("resolve-anchor").get("references") or []:
        if ref.get("unjudged"):
            un[ref["unjudged"]] += 1
    ids = load("check-rule-ids")
    un["rule-not-installed"] += len(ids.get("not-installed") or [])
    if ids.get("examined") == 0 and ids.get("citations-count"):
        un["rule-citations-not-checked"] += ids["citations-count"]
    un["artifact-unreadable"] += len(load("check-artifact-size").get("skipped") or [])
    un["referrer-unreadable"] += len(load("check-orphaned-references").get("skipped") or [])
    return +un


def run(argv):
    write = "--write" in argv
    argv = [a for a in argv if a != "--write"]
    if len(argv) != 2:
        raise lib.Unreadable("usage: record.py FEATURE PLAN.json [--write]")
    feature, plan_path = argv
    w = lib.work_dir(feature)
    m = lib.load_manifest(feature)
    plan = lib.read_json(plan_path)
    merged_path = os.path.join(w, "merged.json")
    if not os.path.isfile(merged_path):
        raise lib.Unreadable(f"{merged_path} does not exist; run combine.py first")
    merged = lib.read_json(merged_path)

    problems, _, _, _ = checker.check(feature, m)
    failed = {k: v for k, v in problems.items() if v}
    if failed:
        raise lib.Refused(f"hand-backs or subjects fail their check: {failed}")
    if os.path.isfile(os.path.join(w, "det", "resolve-references.json")):
        raise lib.Refused("the spec has a references: index, whose outcomes this script does not "
                          "classify yet; extend it before recording")

    tiers_of = {rid: t for s in m["shares"].values() for rid, t in s["tiers"].items()}
    verdicts = dict(merged["verdicts"])
    settled = plan.get("settled") or {}
    left = sorted(set(merged["unsettled"]) - set(settled))
    if left:
        raise lib.Refused(f"unsettled rules the plan's settled map does not name: {left}")
    for rid, v in settled.items():
        if rid not in merged["judged"]:
            raise lib.Refused(f"settled names {rid}, which this run did not judge")
        print(f"settled {rid}: {v} (cells: {merged['unsettled'].get(rid) or merged['verdicts'].get(rid)})")
        if v is None:
            verdicts.pop(rid, None)
        elif v.get("verdict") == "finding":
            verdicts[rid] = {**v, "tier": tiers_of.get(rid)}
        else:
            verdicts[rid] = v
    refs = [v.get("cite") or v.get("at") for v in verdicts.values() if v.get("cite") or v.get("at")]
    bad = lib.unresolved_cites(feature, refs)
    if bad:
        raise lib.Refused(f"verdicts cite headings that do not resolve: {bad}")
    if set(merged["judged"]) | set(merged["unjudged"]) != set(m["judge"]):
        raise lib.Refused("judged and unjudged rules do not cover verdicts.judge")

    findings = [dict(f) for f in plan.get("findings") or []]
    for rid, v in sorted(verdicts.items()):
        if v["verdict"] == "finding":
            findings.append(rule_finding(feature, rid, v, v.get("tier")))
    for r in m.get("reusable") or []:
        if r.get("verdict") == "finding":
            findings.append(rule_finding(feature, r["id"], r, r.get("tier")))
    keys = [key(f) for f in findings if f.get("live", True)]
    dupes = sorted(k for k, n in collections.Counter(keys).items() if n > 1)
    if dupes:
        raise lib.Refused(f"two live findings share a key: {dupes}")

    lib.write_json(os.path.join(w, "fired.json"), keys)
    decided, _ = lib.call("process-decisions", "--feature", feature, "--record", "analysis",
                          "--fired-json", os.path.join(w, "fired.json"))
    matched = {d["key"]: d for d in decided.get("matched") or []}
    lapsed = {d["key"] for d in decided.get("lapsed") or []}
    dispositions = plan.get("dispositions") or {}
    for note in decided.get("notices") or []:
        print(f"notice: {note}")
    for f in findings:
        if not f.get("live", True):
            continue
        k = key(f)
        if k in matched:
            d = matched[k]
            f["disposition"] = {x: d[x] for x in ("outcome", "reason", "target", "task", "work-item",
                                                  "by-rule") if d.get(x)}
            f["decision-key"] = k
        elif k in dispositions:
            f["disposition"] = dispositions[k]
        elif "disposition" not in f:
            f["disposition"] = {"outcome": "undispositioned"}
        if k in lapsed and k not in dispositions:
            print(f"lapsed: {k} needs a new disposition in the plan's dispositions map")

    email = subprocess.run(["git", "config", "user.email"], capture_output=True, text=True).stdout.strip()
    if not email:
        raise lib.Refused("git config user.email is empty, so no decided-by can be recorded")
    tiers = collections.Counter(f["tier"] for f in findings if f.get("live", True))
    un = unexamined(os.path.join(w, "det")) + collections.Counter(plan.get("unexamined") or {})
    rule_verdicts = {rid: v for rid, v in verdicts.items()}

    lib.write_json(os.path.join(w, "findings.json"), findings)
    lib.write_json(os.path.join(w, "expired.json"),
                   (decided.get("expired") or []) + (decided.get("lapsed") or []))
    lib.write_json(os.path.join(w, "decided-by.json"), email)
    lib.write_json(os.path.join(w, "verdicts.json"), rule_verdicts)
    head = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
    cmd = [lib.binary(), "write-analysis", "--feature", feature,
           "--analyzed-at", time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "--analyzed-against", head,
           "--hard-fail", str(tiers.get("hard-fail", 0)),
           "--blocking-findings", str(tiers.get("blocking", 0)), "--advisory", str(tiers.get("advisory", 0)),
           "--findings-json", os.path.join(w, "findings.json"),
           "--expired-decisions-json", os.path.join(w, "expired.json"),
           "--decided-by-json", os.path.join(w, "decided-by.json"),
           "--rule-verdicts-json", os.path.join(w, "verdicts.json")]
    for reason, n in sorted(un.items()):
        cmd += ["--unexamined-reason", f"{reason}={n}"]
    if not un:
        cmd += ["--unexamined", "0"]
    for rid in merged["judged"]:
        cmd += ["--judged-rule", rid]

    print(f"== {feature}: hard-fail {tiers.get('hard-fail', 0)}, blocking {tiers.get('blocking', 0)}, "
          f"advisory {tiers.get('advisory', 0)}; unexamined {dict(un)}; {len(merged['judged'])} rules "
          f"judged, {len(rule_verdicts)} fired; {len(decided.get('expired') or [])} decisions expire")
    for f in findings:
        print(f"  [{f['tier']}] {'' if f.get('live', True) else '(gone) '}{key(f)[:140]} "
              f"=> {f.get('disposition', {}).get('outcome')}")
    if not write:
        print("dry run; pass --write to record it")
        return
    p = subprocess.run(cmd, capture_output=True, text=True)
    print(p.stdout.strip()[:2000] or p.stderr.strip()[:2000])
    if p.returncode:
        raise lib.Refused(f"write-analysis exited {p.returncode}: {p.stderr.strip()[:600]}")


if __name__ == "__main__":
    lib.main(run)
