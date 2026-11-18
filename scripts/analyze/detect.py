#!/usr/bin/env python3
"""Run /ductus:analyze's deterministic detection steps for one feature
through the runtime and print a compact summary (spec 060, scenario
rule-judging-runs-from-repo-scripts).

usage: python3 -I scripts/analyze/detect.py FEATURE

Runs read-spec (step 1), validate-frontmatter, traverse-deps,
resolve-anchor, check-rule-ids, check-departures, derive-dependencies,
lint-markdown, check-artifacts, check-orphaned-references,
check-unfolded-specs (steps 2-10), resolve-references when the spec has a
references: index (step 13), prune-plan and check-artifact-size (steps 16
and 17). Each result is saved whole, its lists-file merged in, under
<work>/det/<primitive>.json, with <work>/det/meta.json holding HEAD, the
digest of the feature's files, the rule files and the constitution it ran
over, and a digest of the working tree's uncommitted state; each run first
clears the results of the last. split.py and record.py
read them there, and refuse them once any of those has changed since.

Steps 2-10, 13, 16 and 17 render each finding in the tier it carries; this
summary prints them so the host can render and disposition them, with the
notice lines discover-rule-files returns. Steps 11, 12, 14 and 15 are
judgment: split.py, briefs.py, check.py and combine.py.

Exit status: 0 when every primitive returned a result, findings or not; 2
when one returned none, naming it. A primitive that could not run is never
read as one that found nothing.
"""

import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import analyze_lib as lib  # noqa: E402


def steps(feature, rule_files, has_references):
    d = f"specs/{feature}"
    rules = []
    for r in rule_files:
        rules += ["--rule-file", r]
    out = [
        ("validate-frontmatter", ["validate-frontmatter", "--path", f"{d}/spec.md"]),
        ("traverse-deps", ["traverse-deps", "--feature", feature]),
        ("resolve-anchor", ["resolve-anchor", "--path", f"{d}/spec.md",
                            "--markers-path", constitution()]),
        ("check-rule-ids", ["check-rule-ids", "--path", f"{d}/spec.md", *rules]),
        ("check-departures", ["check-departures", "--feature", feature]),
        ("derive-dependencies", ["derive-dependencies"]),
        ("lint-markdown", ["lint-markdown", "--path", d]),
        ("check-artifacts", ["check-artifacts", "--feature", feature]),
        ("check-orphaned-references", ["check-orphaned-references"]),
        ("check-unfolded-specs", ["check-unfolded-specs"]),
        ("prune-plan", ["prune-plan", "--feature", feature]),
        ("check-artifact-size", ["check-artifact-size", "--feature", feature,
                                 "--project-files"]),
    ]
    if has_references:
        out.append(("resolve-references", ["resolve-references", "--feature", feature]))
    return out


def constitution():
    for p in ("framework/constitution.md", ".ductus/constitution.md"):
        if os.path.isfile(p):
            return p
    raise lib.Unreadable("no constitution at framework/constitution.md or .ductus/constitution.md")


def summary(name, r):
    g = r.get
    if name == "read-spec":
        fm = g("frontmatter") or {}
        unticked = [c.get("label") or c.get("text", "")[:30]
                    for c in g("acceptance-criteria") or [] if not c.get("checked")]
        heads = [s.get("heading") for s in g("sections") or []]
        return (f"status={fm.get('status')} open-questions={g('open-questions-count')} "
                f"scenario-open-questions={g('scenario-open-questions-count')} "
                f"applicable-rules-section={'Applicable Rules' in heads} "
                f"references={len(fm.get('references') or [])} unticked={unticked}")
    if name == "traverse-deps":
        bad = [x for x in g("dependencies") or []
               if not x.get("exists", True) or not x.get("compatible", True)]
        return f"dependencies={g('dependencies-count')} bad={bad} cycles={g('cycles')}"
    if name == "resolve-anchor":
        unj = {}
        for ref in g("references") or []:
            if ref.get("unjudged"):
                unj[ref["unjudged"]] = unj.get(ref["unjudged"], 0) + 1
        return f"references={g('references-count')} unresolved={g('unresolved')} unjudged={unj}"
    if name == "check-rule-ids":
        return (f"examined={g('examined')} citations={g('citations-count')} missing={g('missing')} "
                f"deprecated={g('deprecated')} retired={g('retired')} not-installed={g('not-installed')}")
    if name == "check-departures":
        return (f"entries={[e.get('rule') for e in g('entries') or []]} "
                f"findings={g('findings')} unexamined={g('unexamined')}")
    if name == "derive-dependencies":
        return (f"updated={g('updated')} cycles={g('cycles')} unwritable={g('unwritable')} "
                f"unparseable={g('unparseable')}")
    if name == "lint-markdown":
        return f"clean={g('clean')} violations={g('violations')}"
    if name == "check-artifacts":
        skipped = {}
        for s in g("skipped") or []:
            skipped[s.get("reason")] = skipped.get(s.get("reason"), 0) + 1
        found = [(f.get("family"), f.get("severity"), f.get("message")) for f in g("findings") or []]
        return f"findings={found} skipped-by-reason={skipped}"
    if name == "check-unfolded-specs":
        return f"examined={g('examined')} unfolded={g('unfolded')}"
    if name == "prune-plan":
        secs = [(s.get("heading"), (s.get("finding") or {}).get("message"), s.get("decided"))
                for s in g("sections") or []]
        return f"missing={g('missing')} sections={secs}"
    if name == "check-artifact-size":
        over = [(o.get("path"), o.get("message"), o.get("decided")) for o in g("oversized") or []]
        return f"oversized={over} skipped={g('skipped')} notices={g('notices')}"
    if name == "resolve-references":
        return f"references={g('references')} duplicate-repos={g('duplicate-repos')}"
    return f"findings={g('findings')} examined={g('examined')} skipped={g('skipped')}"


def run(argv):
    if len(argv) != 1:
        raise lib.Unreadable("usage: detect.py FEATURE")
    feature = argv[0]
    det = os.path.join(lib.work_dir(feature), "det")
    os.makedirs(det, exist_ok=True)
    for n in os.listdir(det):
        if n.endswith(".json"):  # a run that fails leaves no meta; one with no references, no references result
            os.remove(os.path.join(det, n))
    meta = os.path.join(det, "meta.json")
    rules, _ = lib.call("discover-rule-files", "--all")
    rules_dir = rules.get("rules-dir") or ""
    rule_files = [s if "/" in s else f"{rules_dir}/{s}" for s in rules.get("selected") or []]
    digest = lib.detection_digest(feature, [*rule_files, constitution()])
    tree = lib.tree_state()
    spec, _ = lib.call("read-spec", "--feature", feature)
    lib.write_json(os.path.join(det, "read-spec.json"), spec)
    print(f"== {feature} (results under {det})")
    for note in rules.get("notices") or []:
        print(f"notice: {note}")
    print(f"read-spec: {summary('read-spec', spec)}")
    has_refs = bool((spec.get("frontmatter") or {}).get("references"))
    exits, failed = {"read-spec": 0}, []
    for name, args in steps(feature, rule_files, has_refs):
        try:
            result, code = lib.call(*args, allow_exit=True)
        except lib.Unreadable as e:
            failed.append(name)
            print(f"{name}: NO RESULT — {e}")
            continue
        exits[name] = code
        lib.write_json(os.path.join(det, f"{name}.json"), result)
        print(f"{name}{f' (exit {code})' if code else ''}: {summary(name, result)}")
    if failed:
        raise lib.Unreadable(f"no result from {', '.join(failed)}; the run is not detected")
    lib.write_json(meta, {"head": lib.head(), "digest": digest, "tree": tree, "exits": exits})


if __name__ == "__main__":
    lib.main(run)
