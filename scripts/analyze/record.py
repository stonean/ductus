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
                            the host's verdict for each rule the cells left
                            unsettled (null for not fired), and any it
                            overrides, each printed
   "dispositions": {KEY: disposition},
                            a disposition for a generated finding, by its key
   "malformed-headings": [...],
                            each malformed heading a judge reported, which
                            the host has judged and recorded as a finding or
                            set aside
   "unexamined": {REASON: N}}
                            targets the host could not judge, beside those
                            the run's results name

The hand-backs are checked and merged again here, as check.py and
combine.py do, so what is recorded is what the judges wrote. Each fired
rule's finding, and each verdicts.reusable finding, is written as
`rule-assessment — RULE: message`, in the Blocking tier for a MUST-tier
rule and the Advisory tier otherwise; a finding of a rule judged against
the changed sections alone, placed at a section verdicts.changed-sections
does not name, is passed only as its verdict, which write-analysis records
as a contradiction counted in no tier (spec 082, AC7). Each broken cross-service reference
and each repository the registry holds twice is an Advisory finding. The
unexamined targets are those detect.py's results name, an integrity item
step 15 did not check, and the plan's; any the run could not read sets
process-decisions' restricted, so a stored decision is retained rather than
expired. A finding a stored decision matches takes its stored outcome.
decided-by is the repository's `git config user.email`.

Without --write it prints the record it would write and the write-analysis
call; with --write it makes the call.

Exit status: 0 written, or the dry run printed; 1 refused: a hand-back
fails its check, a rule is unsettled, a reported malformed heading is not
in the plan, detection predates an edit or a commit, two findings share a key,
no decided-by, or write-analysis refused; 2 an input could not be read.
"""

import collections
import os
import sys
import time

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import analyze_lib as lib  # noqa: E402
import check as checker  # noqa: E402
import combine  # noqa: E402


def key(f):
    return f.get("decision-key") or f"{f['family']} — {f['message']}"


def rule_finding(feature, rid, v, tier):
    path = v["at"].partition("#")[0]
    return {"tier": "blocking" if tier == "must" else "advisory", "family": "rule-assessment",
            "message": f"{rid}: {v['message']}", "path": f"specs/{feature}/{path}", "live": True}


def load(det, name):
    p = os.path.join(det, f"{name}.json")
    if not os.path.isfile(p):
        raise lib.Unreadable(f"{p} does not exist; run detect.py first")
    return lib.read_json(p)


def listed(result, field, name):
    """A list the runtime always writes, so one absent is a renamed field,
    never an empty list."""
    if not isinstance(result.get(field), list):
        raise lib.Unreadable(f"{name}'s result has no {field} list")
    return result[field]


def unexamined(det):
    """The unexamined targets detect.py's results name, by reason."""
    un = collections.Counter()
    for s in listed(load(det, "check-artifacts"), "skipped", "check-artifacts"):
        un[s["reason"]] += 1
    anchors = load(det, "resolve-anchor")
    for reason in ("anchor-target-unjudgeable", "anchor-target-unreadable"):
        if not isinstance(anchors.get(reason), int):
            raise lib.Unreadable(f"resolve-anchor's result has no {reason} count")
        un[reason] += anchors[reason]  # a scenario detect.py could not read is in the second
    ids = load(det, "check-rule-ids")
    if not isinstance(ids.get("examined"), int):
        raise lib.Unreadable("check-rule-ids' result has no examined count")
    un["rule-not-installed"] += len(ids.get("not-installed") or [])  # absent when empty
    if ids["examined"] == 0 and ids.get("citations-count"):
        un["rule-citations-not-checked"] += ids["citations-count"]
    un["artifact-unreadable"] += len(listed(load(det, "check-artifact-size"), "skipped", "check-artifact-size"))
    un["referrer-unreadable"] += len(listed(load(det, "check-orphaned-references"), "skipped",
                                            "check-orphaned-references"))
    return +un


def reference_findings(feature, det):
    """Step 13's Advisory findings: each broken reference and each repository
    the registry holds under two aliases; the unknowns are printed, never
    findings (framework/reference/analyze.md, Cross-service references)."""
    p = os.path.join(det, "resolve-references.json")
    if not os.path.isfile(p):
        return []
    r = lib.read_json(p)
    out, spec = [], f"specs/{feature}/spec.md"
    for ref in listed(r, "references", "resolve-references"):
        if ref.get("outcome") == "broken":
            out.append({"tier": "advisory", "family": "cross-service-references", "path": spec, "live": True,
                        "message": f"reference to {ref.get('service')} {ref.get('spec')} is broken: the "
                                   "service's checkout holds no such spec"})
        elif ref.get("outcome") != "ok":
            print(f"reference {ref.get('service')} {ref.get('spec')}: {ref.get('outcome')} (informational)")
    for dup in listed(r, "duplicate-repos", "resolve-references"):
        out.append({"tier": "advisory", "family": "cross-service-references", "path": spec, "live": True,
                    "message": f"the [services] registry holds {dup.get('repo')} under "
                               f"{', '.join(dup.get('aliases') or [])}"})
    return out


def demoted(m, rid, v):
    """Whether a finding is a contradiction alone (spec 082, AC7): its rule
    was judged against the changed sections only, its earlier verdict was not
    a waiver, and it is placed at `<path>#<Heading>` that
    verdicts.changed-sections does not name. That a heading the list does not
    name is unchanged, present in both digests alike as the runtime's
    `SectionDelta::unchanged_at` requires, holds only because run() has
    already refused every `at` that does not resolve (lib.unresolved_cites):
    a heading that exists now and is not named changed was in the last record
    too."""
    if rid not in set(m.get("rejudge") or []):
        return False
    if any(r.get("id") == rid and r.get("verdict") == "waived" for r in m.get("reusable") or []):
        return False
    path, _, heading = (v.get("at") or "").partition("#")
    key = f"{path.strip()}#{heading.strip()}"
    return bool(heading.strip()) and key not in set(m.get("changed-sections") or [])


def run(argv):
    write = "--write" in argv
    argv = [a for a in argv if a != "--write"]
    if len(argv) != 2:
        raise lib.Unreadable("usage: record.py FEATURE PLAN.json [--write]")
    feature, plan_path = argv
    w = lib.work_dir(feature)
    det = os.path.join(w, "det")
    m = lib.load_manifest(feature)
    plan = lib.read_json(plan_path)
    if not isinstance(plan, dict):
        raise lib.Unreadable(f"{plan_path} is not a JSON object")

    lib.check_detection_current(feature, det)
    result = checker.check(feature, m)
    if checker.failures(result):
        raise lib.Refused(f"hand-backs or subjects fail their check: {checker.failures(result)}")
    merged = combine.merge(m, result["outs"], result["unresolved"])
    reported = sorted({h for hs in result["malformed"].values() for h in hs})
    unacknowledged = sorted(set(reported) - set(plan.get("malformed-headings") or []))
    if unacknowledged:
        raise lib.Refused(f"judges reported malformed headings the plan's malformed-headings does not "
                          f"name: {unacknowledged}; a malformed rule heading is a Blocking finding "
                          "(framework/reference/analyze.md, Rules)")

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
    if set(merged["judged"]) | set(merged["unjudged"]) != set(m["judge"]) | set(m.get("rejudge") or []):
        raise lib.Refused("judged and unjudged rules do not cover verdicts.judge and verdicts.rejudge")

    findings = [dict(f) for f in plan.get("findings") or []]
    for rid, v in sorted(verdicts.items()):
        if v["verdict"] == "finding" and demoted(m, rid, v):
            print(f"contradiction: {rid} judged a finding at {v.get('at')}, a section unchanged since its "
                  "verdict, so it counts in no tier")
        elif v["verdict"] == "finding":
            findings.append(rule_finding(feature, rid, v, v.get("tier")))
    for r in m.get("reusable") or []:
        if r.get("verdict") == "finding":
            findings.append(rule_finding(feature, r["id"], r, r.get("tier")))
    findings += reference_findings(feature, det)
    keys = [key(f) for f in findings if f.get("live", True)]
    dupes = sorted(k for k, n in collections.Counter(keys).items() if n > 1)
    if dupes:
        raise lib.Refused(f"two live findings share a key: {dupes}")
    planned = {f.get("message") for f in plan.get("findings") or []}
    for kind in ("grounding", "integrity", "decision-reasons"):
        for item in result["outs"]["step15"].get(kind) or []:
            if item.get("message") not in planned:
                print(f"step 15 {kind}, not in the plan: {item.get('path')}: {item.get('message')}")

    un = unexamined(det) + collections.Counter(plan.get("unexamined") or {})
    un["integrity-not-checked"] += len(result["unchecked"])
    un = +un
    restricted = lib.is_restricted(un)
    lib.write_json(os.path.join(w, "fired.json"), keys)
    decide = ["process-decisions", "--feature", feature, "--record", "analysis",
              "--fired-json", os.path.join(w, "fired.json")]
    decided, _ = lib.call(*decide, *(["--restricted"] if restricted else []))
    for name in ("matched", "expired", "lapsed"):
        listed(decided, name, "process-decisions")
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

    email = lib.git("config", "user.email")
    if not email:
        raise lib.Refused("git config user.email is empty, so no decided-by can be recorded")
    tiers = collections.Counter(f["tier"] for f in findings if f.get("live", True))
    lib.write_json(os.path.join(w, "findings.json"), findings)
    lib.write_json(os.path.join(w, "expired.json"),
                   (decided.get("expired") or []) + (decided.get("lapsed") or []))
    lib.write_json(os.path.join(w, "decided-by.json"), email)
    lib.write_json(os.path.join(w, "verdicts.json"), verdicts)
    cmd = ["write-analysis", "--feature", feature,
           "--analyzed-at", time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "--analyzed-against", lib.head(),
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
          f"advisory {tiers.get('advisory', 0)}; unexamined {dict(un)}{' (restricted)' if restricted else ''}; "
          f"{len(merged['judged'])} rules judged, {len(verdicts)} fired; "
          f"{len(decided.get('expired') or [])} decisions expire")
    for f in findings:
        print(f"  [{f['tier']}] {'' if f.get('live', True) else '(gone) '}{key(f)[:140]} "
              f"=> {f.get('disposition', {}).get('outcome')}")
    if not write:
        print("dry run; pass --write to record it")
        return
    p = lib.run_cmd([lib.binary(), *cmd])
    if p.returncode:
        raise lib.Refused(f"write-analysis exited {p.returncode}: {(p.stderr or p.stdout).strip()[:600]}")
    print(p.stdout.strip()[:600])


if __name__ == "__main__":
    lib.main(run)
