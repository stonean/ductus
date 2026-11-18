#!/usr/bin/env python3
"""Test surface for scripts/analyze/ (spec 060, scenario
rule-judging-runs-from-repo-scripts).

Each case runs a script in a fixture project, a git repository under a
temporary directory with one feature, against a stub runtime whose answers
the case sets, so no release build is needed. The evidence each script works
is that it fails on each condition it claims to catch (QUAL-TEST-001), and
that a failure is never an exit 0 (QUAL-CLAIM-001).

Coverage:
  A. detect.py exits 2 when a primitive returns no result
  B. split.py keeps each rule file whole in one share, judges only
     verdicts.judge, and keeps verdicts.reusable for the record
  C. split.py groups the scenarios when the subjects overflow a cell, and
     keeps them in one group when they fit
  D. split.py refuses a judged rule no loaded file holds
  E. briefs.py fills every placeholder and names each subject and departure
  F. check.py passes a whole, well-formed hand-back
  G. check.py fails an ID left unanswered
  H. check.py fails a short read
  I. check.py fails a cite that resolves to no heading
  J. check.py fails a waiver with no departure entry
  K. check.py fails an output older than the manifest
  L. check.py fails a design subject edited after the split
  Q. check.py notes, and combine.py passes over, an unresolved cite
     another cell of its share resolves for the same rule
  M. combine.py leaves a rule unsettled when its cells disagree on a finding,
     takes a unanimous finding in its rule's tier, and cites the core subject
  N. record.py refuses an unsettled rule its plan does not settle
  O. record.py writes judged rules, verdicts, a matched decision's outcome,
     and the decided-by git config names
  P. record.py refuses when git config names no user.email

ANALYZE_SCRIPTS, when set, names the directory of scripts to run, so a
mutant can be run against these cases.

Usage: python3 -I scripts/tests/test-analyze-scripts.py
"""

import json
import os
import shutil
import subprocess
import sys
import tempfile
import time

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SCRIPTS = os.environ.get("ANALYZE_SCRIPTS") or os.path.join(REPO, "scripts", "analyze")
F = "001-demo"
EMAIL = "analyst@example.invalid"
failures = 0

STUB = r'''#!/usr/bin/env python3
import json, os, sys
d = os.environ["STUB_DIR"]
name = sys.argv[1]
with open(os.path.join(d, "calls.log"), "a") as f:
    f.write(json.dumps(sys.argv[1:]) + "\n")
if name in open(os.path.join(d, "broken")).read().split():
    print("not json"); sys.exit(1)
if name == "read-section":
    missing = open(os.path.join(d, "missing-headings")).read().splitlines()
    asked = [sys.argv[i + 1] for i, a in enumerate(sys.argv) if a == "--heading"]
    print(json.dumps({"sections-count": 0, "not-found": [h for h in asked if h in missing],
                      "form": "inline"}))
    sys.exit(0)
if name == "write-analysis":
    for flag in ("--rule-verdicts-json", "--findings-json", "--decided-by-json"):
        src = sys.argv[sys.argv.index(flag) + 1]
        with open(src) as s, open(os.path.join(d, "captured" + flag[1:]), "w") as t:
            t.write(s.read())
    print(json.dumps({"written": True})); sys.exit(0)
p = os.path.join(d, name + ".json")
print(open(p).read() if os.path.isfile(p) else "{}")
'''

RULES = {
    "rules-dir": "rules",
    "selected": ["a-backend.md", "b-cross.md", "c-frontend.md"],
    "rule-files": [
        {"file": "rules/a-backend.md", "other-headings": [{"line": 1, "heading": "# A"}], "rules": [
            {"id": "RA-001", "heading": "### RA-001", "tier": "must", "statement": "S " * 400,
             "verification": "V " * 400},
            {"id": "RA-002", "heading": "### RA-002", "tier": "should", "statement": "S",
             "verification": "V"}]},
        {"file": "rules/b-cross.md", "other-headings": [], "rules": [
            {"id": "RB-001", "heading": "### RB-001", "tier": "must", "statement": "S " * 300,
             "verification": "V " * 300}]},
        {"file": "rules/c-frontend.md", "other-headings": [], "rules": [
            {"id": "RC-001", "heading": "### RC-001", "tier": "should", "statement": "S " * 300,
             "verification": "V " * 300},
            {"id": "RC-002", "heading": "### RC-002", "tier": "should", "statement": "S",
             "verification": "V"}]},
    ],
    "verdicts": {"reason": "subjects-changed", "judge": ["RA-001", "RA-002", "RB-001", "RC-001"],
                 "reusable": [{"id": "RC-002", "verdict": "finding", "tier": "should",
                               "at": "spec.md#Motivation", "message": "a reused finding"}]},
}


def ok(name, _detail=""):
    print(f"  PASS  {name}", flush=True)


def fail(name, detail=""):
    global failures
    failures += 1
    print(f"  FAIL  {name} {detail}", file=sys.stderr, flush=True)


class Project:
    def __init__(self):
        self.root = tempfile.mkdtemp(prefix="analyze-test-")
        self.stub = os.path.join(self.root, "stub")
        self.work = os.path.join(self.root, "work")
        os.makedirs(self.stub)
        os.makedirs(os.path.join(self.work, "det"))
        d = os.path.join(self.root, "specs", F, "scenarios")
        os.makedirs(d)
        self.write(f"specs/{F}/spec.md", "---\nstatus: in-progress\n---\n\n# Demo\n\n## Motivation\n\nWhy.\n")
        self.write(f"specs/{F}/plan.md", "# Plan\n\n## Design\n\nHow.\n")
        for n in ("one", "two", "three"):
            self.write(f"specs/{F}/scenarios/{n}.md", f"# {n}\n\n## Behavior\n\n" + "x" * 900 + "\n")
        self.write(".ductus/constitution.md", "# Constitution\n")
        self.write("stub/ductus", STUB)
        os.chmod(os.path.join(self.stub, "ductus"), 0o755)
        self.write("stub/broken", "")
        self.write("stub/missing-headings", "")
        self.write("stub/process-decisions.json", json.dumps({"matched": [], "expired": [], "lapsed": []}))
        self.write("rules.json", json.dumps(RULES))
        self.det("check-departures", {"entries": [{"rule": "RB-001", "reason": "a decided departure"}]})
        self.det("read-spec", {"frontmatter": {"status": "in-progress"}})
        self.det("check-artifacts", {"skipped": [{"reason": "ships-to-adopter"}]})
        for n in ("resolve-anchor", "check-artifact-size", "check-orphaned-references"):
            self.det(n, {})
        self.det("check-rule-ids", {"examined": 3, "citations-count": 0})
        self.git("init", "-q")
        self.git("config", "user.email", EMAIL)
        self.git("add", ".")
        self.git("-c", "user.name=t", "commit", "-qm", "fixture")

    def write(self, rel, text):
        p = os.path.join(self.root, rel)
        os.makedirs(os.path.dirname(p), exist_ok=True)
        with open(p, "w") as f:
            f.write(text)

    def det(self, name, value):
        with open(os.path.join(self.work, "det", f"{name}.json"), "w") as f:
            json.dump(value, f)

    def env(self):
        e = {k: v for k, v in os.environ.items() if not k.startswith("GIT_")}
        e.update({"DUCTUS_BIN": os.path.join(self.stub, "ductus"), "ANALYZE_WORK": self.work,
                  "STUB_DIR": self.stub, "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"})
        return e

    def git(self, *args):
        subprocess.run(["git", *args], cwd=self.root, env=self.env(), check=True, capture_output=True)

    def run(self, script, *args):
        return subprocess.run([sys.executable, "-I", os.path.join(SCRIPTS, script), *args],
                              cwd=self.root, env=self.env(), capture_output=True, text=True)

    def manifest(self):
        with open(os.path.join(self.work, "manifest.json")) as f:
            return json.load(f)

    def split(self, *extra):
        return self.run("split.py", F, "--rules-json", os.path.join(self.root, "rules.json"), *extra)

    def hand_back(self, cell, verdicts=None, drop=(), short=None):
        """Write a cell's output: every share ID judged but those in drop,
        every subject and the share read whole but the path in short."""
        m = self.manifest()
        c = next(x for x in m["cells"] if x["cell"] == cell)
        share = m["shares"][c["share"]]
        paths = [f"specs/{F}/{p}" for p in c["subjects"]]
        acct = {p: [os.path.getsize(os.path.join(self.root, p))] * 2 for p in paths}
        acct[share["file"]] = [share["bytes"]] * 2
        if short:
            acct[short][0] -= 10
        out = {"cell": cell, "judged": [i for i in share["ids"] if i not in drop],
               "verdicts": {k: v for k, v in (verdicts or {}).items() if k in share["ids"]},
               "unjudged": [], "malformed-headings": [], "bytes-read": acct}
        with open(c["out"], "w") as f:
            json.dump(out, f)

    def step15(self):
        m = self.manifest()
        s = m["step15"]
        acct = {f"specs/{F}/{p}": [os.path.getsize(os.path.join(self.root, "specs", F, p))] * 2
                for p in s["subjects"]}
        with open(s["out"], "w") as f:
            json.dump({"grounding": [], "integrity": [], "decision-reasons": [], "bytes-read": acct,
                       "integrity-checked": {"code-block": True, "rationale": True, "system": True}}, f)

    def all_hand_backs(self, verdicts_by_cell=None):
        time.sleep(1.1)  # an output must be newer than the manifest's whole-second stamp
        for c in self.manifest()["cells"]:
            self.hand_back(c["cell"], (verdicts_by_cell or {}).get(c["cell"]))
        self.step15()

    def close(self):
        shutil.rmtree(self.root, ignore_errors=True)


def case(name, fn):
    p = Project()
    try:
        fn(p, name)
    except Exception as e:  # a case that raises has not passed
        fail(name, f"raised {type(e).__name__}: {e}")
    finally:
        p.close()


def a(p, name):
    p.write("stub/broken", "lint-markdown")
    p.write("stub/discover-rule-files.json", json.dumps({"rules-dir": "rules", "selected": []}))
    p.write("stub/read-spec.json", json.dumps({"frontmatter": {"status": "in-progress"}}))
    r = p.run("detect.py", F)
    (ok if r.returncode == 2 and "lint-markdown" in r.stderr else fail)(name, r.stderr)


def b(p, name):
    r = p.split("--share-bytes", "1500")
    m = p.manifest() if r.returncode == 0 else {}
    files = [f for s in m.get("shares", {}).values() for f in s["rule-files"]]
    ids = sorted(i for s in m.get("shares", {}).values() for i in s["ids"])
    good = (r.returncode == 0 and len(m["shares"]) > 1 and sorted(files) == sorted(set(files))
            and len(files) == 3 and ids == ["RA-001", "RA-002", "RB-001", "RC-001"]
            and [x["id"] for x in m["reusable"]] == ["RC-002"])
    (ok if good else fail)(name, r.stderr + json.dumps(m.get("shares"))[:300])


def c(p, name):
    r1 = p.split("--cell-bytes", "1000000")
    one = len(p.manifest()["groups"]) if r1.returncode == 0 else -1
    r2 = p.split("--cell-bytes", "6000", "--share-bytes", "100000")
    m = p.manifest() if r2.returncode == 0 else {}
    groups = m.get("groups") or []
    flat = sorted(s for g in groups for s in g)
    good = (one == 1 and len(groups) > 1 and len(flat) == 3 and len(set(flat)) == 3
            and all(len(x["subjects"]) >= 3 for x in m["cells"]))
    (ok if good else fail)(name, f"one={one} groups={groups} {r2.stderr}")


def d(p, name):
    rules = json.loads(json.dumps(RULES))
    rules["verdicts"]["judge"].append("RZ-404")
    p.write("rules.json", json.dumps(rules))
    r = p.split()
    (ok if r.returncode == 1 and "RZ-404" in r.stderr else fail)(name, r.stderr)


def e(p, name):
    p.split("--share-bytes", "1500")
    r = p.run("briefs.py", F)
    m = p.manifest()
    texts = [open(x["brief"]).read() for x in m["cells"]] + [open(m["step15"]["brief"]).read()]
    good = (r.returncode == 0 and not any("{{" in t for t in texts)
            and all(f"specs/{F}/{s}" in open(x["brief"]).read() for x in m["cells"] for s in x["subjects"])
            and "`RB-001`: a decided departure" in texts[0])
    (ok if good else fail)(name, r.stderr)


def f_(p, name):
    p.split()
    p.all_hand_backs()
    r = p.run("check.py", F)
    (ok if r.returncode == 0 else fail)(name, r.stdout + r.stderr)


def failing_check(mutate, needle):
    def run(p, name):
        p.split()
        p.all_hand_backs()
        mutate(p)
        r = p.run("check.py", F)
        (ok if r.returncode == 1 and needle in r.stdout else fail)(name, r.stdout + r.stderr)
    return run


def cell_a(p):
    return p.manifest()["cells"][0]["cell"]


def combine_case(p, name):
    p.split("--cell-bytes", "6000", "--share-bytes", "100000")
    m = p.manifest()
    cells = [x["cell"] for x in m["cells"]]
    fnd = {"verdict": "finding", "at": "plan.md#Design", "message": "not covered"}
    verdicts = {cells[0]: {"RA-001": fnd, "RA-002": fnd,
                           "RC-001": {"verdict": "addressed", "cite": "scenarios/one.md#Behavior"}}}
    for x in cells[1:]:
        verdicts[x] = {"RA-002": fnd, "RC-001": {"verdict": "addressed", "cite": "plan.md#Design"}}
    p.all_hand_backs(verdicts)
    r = p.run("combine.py", F)
    out = json.load(open(os.path.join(p.work, "merged.json"))) if r.returncode == 0 else {}
    v = out.get("verdicts", {})
    good = (r.returncode == 0 and len(cells) > 1 and "RA-001" in out["unsettled"] and "RA-001" not in v
            and v.get("RA-002", {}).get("tier") == "should"
            and v.get("RC-001", {}).get("cite") == "plan.md#Design")
    (ok if good else fail)(name, r.stdout + r.stderr)


def q(p, name):
    p.split("--cell-bytes", "6000", "--share-bytes", "100000")
    cells = [x["cell"] for x in p.manifest()["cells"] if x["share"] == "A"]
    p.write("stub/missing-headings", "Short\n")
    verdicts = {cells[0]: {"RA-002": {"verdict": "addressed", "cite": "plan.md#Short"}}}
    for x in cells[1:]:
        verdicts[x] = {"RA-002": {"verdict": "addressed", "cite": "scenarios/two.md#Behavior"}}
    p.all_hand_backs(verdicts)
    r = p.run("combine.py", F)
    out = json.load(open(os.path.join(p.work, "merged.json"))) if r.returncode == 0 else {}
    good = (r.returncode == 0 and len(cells) > 1
            and out["verdicts"].get("RA-002", {}).get("cite") == "scenarios/two.md#Behavior")
    (ok if good else fail)(name, r.stdout + r.stderr)


def record_setup(p, settle):
    p.split("--cell-bytes", "6000", "--share-bytes", "100000")
    cells = [x["cell"] for x in p.manifest()["cells"]]
    fnd = {"verdict": "finding", "at": "plan.md#Design", "message": "not covered"}
    verdicts = {cells[0]: {"RA-001": fnd, "RA-002": fnd}}
    for x in cells[1:]:
        verdicts[x] = {"RA-002": fnd}
    p.all_hand_backs(verdicts)
    p.run("combine.py", F)
    plan = {"findings": [{"tier": "advisory", "family": "grounding", "message": "a claim",
                          "path": f"specs/{F}/spec.md", "live": True}]}
    if settle:
        plan["settled"] = {"RA-001": None}
    p.write("plan.json", json.dumps(plan))


def n(p, name):
    record_setup(p, settle=False)
    r = p.run("record.py", F, "plan.json", "--write")
    (ok if r.returncode == 1 and "RA-001" in r.stderr else fail)(name, r.stdout + r.stderr)


def o(p, name):
    record_setup(p, settle=True)
    p.write("stub/process-decisions.json", json.dumps({
        "matched": [{"key": "grounding — a claim", "outcome": "waived", "reason": "stored", "by-rule": True}],
        "expired": [], "lapsed": []}))
    r = p.run("record.py", F, "plan.json", "--write")
    calls = [json.loads(x) for x in open(os.path.join(p.stub, "calls.log"))]
    wa = next((x for x in calls if x[0] == "write-analysis"), [])
    judged = sorted(wa[i + 1] for i, x in enumerate(wa) if x == "--judged-rule")
    try:
        rv = json.load(open(os.path.join(p.stub, "captured-rule-verdicts-json")))
        fs = json.load(open(os.path.join(p.stub, "captured-findings-json")))
        by = json.load(open(os.path.join(p.stub, "captured-decided-by-json")))
    except OSError:
        rv, fs, by = {}, [], None
    claim = next((x for x in fs if x["family"] == "grounding"), {})
    keys = sorted(x["message"] for x in fs if x["family"] == "rule-assessment")
    good = (r.returncode == 0 and judged == ["RA-001", "RA-002", "RB-001", "RC-001"]
            and "RA-001" not in rv and rv.get("RA-002", {}).get("tier") == "should"
            and claim.get("disposition", {}).get("outcome") == "waived" and by == EMAIL
            and keys == ["RA-002: not covered", "RC-002: a reused finding"]
            and "--blocking-findings" in wa and wa[wa.index("--advisory") + 1] == "3")
    (ok if good else fail)(name, r.stdout + r.stderr + json.dumps([rv, keys]))


def p_(p, name):
    record_setup(p, settle=True)
    p.git("config", "--unset", "user.email")
    r = p.run("record.py", F, "plan.json", "--write")
    (ok if r.returncode == 1 and "user.email" in r.stderr else fail)(name, r.stderr)


def main():
    print("analyze scripts:", flush=True)
    case("A. detect.py exits 2 on a primitive with no result", a)
    case("B. split.py keeps rule files whole and judges only verdicts.judge", b)
    case("C. split.py groups overflowing scenarios, and only then", c)
    case("D. split.py refuses a judged rule no file holds", d)
    case("E. briefs.py fills every placeholder", e)
    case("F. check.py passes a whole, well-formed hand-back", f_)
    case("G. check.py fails an unanswered ID", failing_check(
        lambda p: p.hand_back(cell_a(p), drop=p.manifest()["shares"]["A"]["ids"][:1]), "no answer"))
    case("H. check.py fails a short read", failing_check(
        lambda p: p.hand_back(cell_a(p), short=f"specs/{F}/plan.md"), "read"))
    case("I. check.py fails an unresolved cite", failing_check(
        lambda p: (p.write("stub/missing-headings", "Gone\n"), p.hand_back(cell_a(p), {
            p.manifest()["shares"]["A"]["ids"][0]: {"verdict": "addressed", "cite": "plan.md#Gone"}})),
        "names no heading"))
    case("J. check.py fails a waiver with no departure", failing_check(
        lambda p: p.hand_back(cell_a(p), {"RA-001": {"verdict": "waived"}, "RA-002": {"verdict": "waived"},
                                          "RC-001": {"verdict": "waived"}}), "no departure entry"))
    case("K. check.py fails an output older than the manifest", failing_check(
        lambda p: os.utime(p.manifest()["cells"][0]["out"], (0, 0)), "predates"))
    case("L. check.py fails a subject edited after the split", failing_check(
        lambda p: p.write(f"specs/{F}/plan.md", "# Plan\n\n## Design\n\nChanged.\n"), "changed since"))
    case("M. combine.py settles agreement and leaves disagreement unsettled", combine_case)
    case("Q. combine.py takes the cite that resolves", q)
    case("N. record.py refuses an unsettled rule", n)
    case("O. record.py writes the run", o)
    case("P. record.py refuses with no user.email", p_)
    if failures:
        print(f"{failures} failure(s)", file=sys.stderr)
        sys.exit(1)
    print("all passed")


if __name__ == "__main__":
    main()
