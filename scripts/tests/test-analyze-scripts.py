#!/usr/bin/env python3
"""Test surface for scripts/analyze/ (spec 060, scenario
rule-judging-runs-from-repo-scripts).

Most cases run a script in a fixture project, a git repository under a
temporary directory with one feature, against a stub runtime whose answers
the case sets. Case Z runs the whole chain against the real release binary
instead: the scripts read each list the runtime always writes strictly, so
a field the runtime renames fails Z rather than reading as empty, and Z
makes not-installed, a list the runtime omits when empty, non-empty.
It needs runtime/target/release/ductus, which
framework-checks.yml builds before it runs this, and fails, naming the
build, without one. The evidence each script works is that it fails on
each condition it claims to catch (QUAL-TEST-001), and that a failure is
never an exit 0 (QUAL-CLAIM-001).

Coverage:
  A. detect.py exits 2 when a primitive returns no result, and writes no
     detection record
  B. split.py keeps each rule file whole in one share, judges no reusable
     rule, and keeps verdicts.reusable for the record
  C. split.py groups the scenarios when the subjects overflow a cell, and
     keeps them in one group when they fit
  D. split.py refuses a judged rule no loaded file holds
  E. briefs.py fills every placeholder, names each subject and departure,
     and carries each other-heading's body into the share
  F. check.py passes a whole, well-formed hand-back
  G. check.py fails an ID left unanswered
  H. check.py fails a short read
  I. check.py fails a cite that resolves to no heading
  J. check.py fails a waiver with no departure entry
  K. check.py fails an output older than the manifest
  L. check.py fails a design subject edited after the split
  M. combine.py leaves a rule unsettled when its cells disagree on a finding,
     takes a unanimous finding in its rule's tier, and cites the core subject
  Q. check.py notes, and combine.py passes over, an unresolved cite another
     cell of its share resolves for the same rule
  N. record.py refuses an unsettled rule its plan does not settle
  O. record.py writes judged rules, verdicts, a matched decision's outcome,
     the unexamined targets detection names, and the decided-by git config
     names, and passes process-decisions no restricted for a target excluded
     by construction
  P. record.py refuses when git config names no user.email
  R. a call that runs past ANALYZE_TIMEOUT_SECONDS is stopped, exit 2
  S. a malformed setting or feature name exits 2 and creates no directory,
     and a DUCTUS_BIN naming no executable exits 2 before detection clears
     the last run's results
  T. check.py fails a design subject added after the split
  U. an integrity item step 15 did not check is recorded as
     integrity-not-checked, which sets restricted
  AP. check.py fails a step-15 hand-back that does not say whether the plan
     was judged against the spec's Behavior and criteria, and a contract item
     step 15 did not check is recorded as integrity-not-checked (spec 067, AC7)
  V. record.py refuses a malformed heading a judge reported that its plan
     does not name
  W. record.py refuses detection older than the feature
  X. split.py keeps every cell within --cell-bytes when its scenarios can be
     split further
  Y. check.py fails a hand-back of the wrong shape, never with a traceback
  AA. record.py writes a broken cross-service reference as an Advisory
     finding
  AB. record.py refuses two live findings that share a key
  AC. combine.py refuses while step 15's hand-back fails its check
  AK. split.py shares out verdicts.judge and verdicts.rejudge and keeps
     verdicts.changed-sections, and briefs.py names each share's rules to
     judge against the changed sections (spec 082, AC7)
  AL. record.py counts a rejudged rule's finding at a changed section and
     passes one at an unchanged section as its verdict alone, and covers
     verdicts.judge and verdicts.rejudge (spec 082, AC7)
  AM. split.py reads verdicts.rejudge and verdicts.changed-sections from the
     top level of a file-form result, as it reads verdicts.judge (AC7)
  AN. record.py counts a rejudged rule's finding at an unchanged section
     when its earlier verdict was a waiver (AC7)
  AD. check.py exits 2 when read-section returns no not-found list
  AE. a defect in a script exits 2, never 1, the refused status
  AF. detect.py runs resolve-references only for a spec with references,
     and drops the last run's result when the spec has none
  AG. split.py refuses detection older than an edit to the feature or the
     constitution, or a commit, since
  AH. record.py and split.py exit 2 when a result lacks a list or count the
     runtime always writes, rather than reading it as empty: one case per
     site the scripts read strictly
  AI. split.py refuses detection older than an untracked file added, or
     a tracked file edited, outside the feature
  AJ. split.py exits 2 on a budget flag that is not a whole number of at
     least 1, naming the flag, the value and the bound, and takes 1
  AO. detect.py runs resolve-anchor on each scenario under the spec, tags each
     reference with its file's path, counts the scenario files as examined (0
     for a spec with none), and record.py counts their unjudged references
     unexamined (spec 067, AC8)
  AQ. detect.py merges a scenario's counts into resolve-anchor.json as the
     exec walker does (the kind and form counts, references-count and
     unresolved-count restated, unresolved sorted), reads a scenario file
     whatever the case of its `.md`, and names a scenario it cannot read in
     its output and counts it in anchor-target-unreadable, not examined, and
     record.py records the result's own anchor-target counts unexamined and
     refuses a result missing either
  Z. the whole chain, detect.py through record.py --write, against the real
     release binary, its spec citing a shipped rule the fixture never
     installed, so check-rule-ids' not-installed is non-empty

ANALYZE_SCRIPTS, when set, names the directory of scripts to run, so a
mutant can be run against these cases. Usage:
python3 -I scripts/tests/test-analyze-scripts.py
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
REAL_BIN = os.path.join(REPO, "runtime", "target", "release", "ductus")
F = "001-demo"
EMAIL = "analyst@example.invalid"
failures = 0

STUB = r'''#!/usr/bin/env python3
import json, os, sys, time
d = os.environ["STUB_DIR"]
name = sys.argv[1]
with open(os.path.join(d, "calls.log"), "a") as f:
    f.write(json.dumps(sys.argv[1:]) + "\n")
if name in open(os.path.join(d, "broken")).read().split():
    print("not json"); sys.exit(1)
if name == "resolve-anchor" and "--path" in sys.argv and \
        sys.argv[sys.argv.index("--path") + 1] in open(os.path.join(d, "broken-paths")).read().split():
    print("not json"); sys.exit(1)
if name in open(os.path.join(d, "slow")).read().split():
    time.sleep(5)
if name == "read-section" and os.path.exists(os.path.join(d, "no-not-found")):
    print(json.dumps({"sections-count": 0, "form": "inline"})); sys.exit(0)
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
        {"file": "rules/a-backend.md",
         "other-headings": [{"line": 1, "heading": "# A", "body": "Preamble of the A rules."}], "rules": [
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

FND = {"verdict": "finding", "at": "plan.md#Design", "message": "not covered"}


def ok(name, _detail=""):
    print(f"  PASS  {name}", flush=True)


def fail(name, detail=""):
    global failures
    failures += 1
    print(f"  FAIL  {name} {detail}", file=sys.stderr, flush=True)


def check_(name, good, detail=""):
    (ok if good else fail)(name, detail)


class Project:
    """A fixture project: one feature, a stub runtime answering detection,
    detect.py already run over it."""

    def __init__(self, scenario_bytes=900):
        # The repository, and beside it, outside it, what the run stages and
        # the stub writes, so a run's own files never read as an edit to the tree.
        self.base = tempfile.mkdtemp(prefix="analyze-test-")
        self.root = os.path.join(self.base, "repo")
        self.stub = os.path.join(self.base, "stub")
        self.work = os.path.join(self.base, "work")
        self.plan = os.path.join(self.base, "plan.json")
        os.makedirs(self.stub)
        self.write(f"specs/{F}/spec.md", "---\nstatus: in-progress\n---\n\n# Demo\n\n## Motivation\n\nWhy.\n")
        self.write(f"specs/{F}/plan.md", "# Plan\n\n## Design\n\nHow.\n")
        self.write(f"specs/{F}/tasks.md", "# Tasks\n\n## 1. Build\n\n- [x] Done\n")
        for n in ("one", "two", "three"):
            self.write(f"specs/{F}/scenarios/{n}.md", f"# {n}\n\n## Behavior\n\n" + "x" * scenario_bytes + "\n")
        self.write(".ductus/constitution.md", "# Constitution\n")
        self.write("stub/ductus", STUB)
        os.chmod(os.path.join(self.stub, "ductus"), 0o755)
        for n in ("broken", "slow", "missing-headings", "broken-paths"):
            self.write(f"stub/{n}", "")
        self.answer("process-decisions", {"matched": [], "expired": [], "lapsed": []})
        self.answer("discover-rule-files", {"rules-dir": "rules", "selected": RULES["selected"],
                                            "notices": ["loading rule files: a, b, c"]})
        self.answer("check-departures", {"entries": [{"rule": "RB-001", "reason": "a decided departure"}]})
        self.answer("read-spec", {"frontmatter": {"status": "in-progress"}})
        self.answer("check-artifacts", {"skipped": [{"reason": "ships-to-adopter"}]})
        self.answer("check-rule-ids", {"examined": 3, "citations-count": 0})
        for name in ("check-artifact-size", "check-orphaned-references"):
            self.answer(name, {"skipped": []})
        self.answer("resolve-anchor", {"references": [], "anchor-target-unjudgeable": 0,
                                       "anchor-target-unreadable": 0})
        self.write("rules.json", json.dumps(RULES))
        self.git("init", "-q")
        self.git("config", "user.email", EMAIL)
        self.git("add", ".")
        self.git("-c", "user.name=t", "commit", "-qm", "fixture")
        r = self.run("detect.py", F)
        if r.returncode:
            raise RuntimeError(f"fixture detection failed: {r.stdout}{r.stderr}")

    def write(self, rel, text):
        outside = rel.startswith("stub/") or rel in ("plan.json", "rules.json")
        p = os.path.join(self.base if outside else self.root, rel)
        os.makedirs(os.path.dirname(p), exist_ok=True)
        with open(p, "w") as f:
            f.write(text)

    def answer(self, primitive, value):
        self.write(f"stub/{primitive}.json", json.dumps(value))

    def det(self, name, value):
        with open(os.path.join(self.work, "det", f"{name}.json"), "w") as f:
            json.dump(value, f)

    def env(self, **extra):
        e = {k: v for k, v in os.environ.items() if not k.startswith(("GIT_", "ANALYZE_", "DUCTUS_"))}
        e.update({"DUCTUS_BIN": os.path.join(self.stub, "ductus"), "ANALYZE_WORK": self.work,
                  "STUB_DIR": self.stub, "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"})
        e.update(extra)
        return e

    def git(self, *args):
        subprocess.run(["git", *args], cwd=self.root, env=self.env(), check=True, capture_output=True)

    def run(self, script, *args, **env):
        return subprocess.run([sys.executable, "-I", os.path.join(SCRIPTS, script), *args],
                              cwd=self.root, env=self.env(**env), capture_output=True, text=True)

    def manifest(self):
        with open(os.path.join(self.work, "manifest.json")) as f:
            return json.load(f)

    def split(self, *extra):
        r = self.run("split.py", F, "--rules-json", os.path.join(self.base, "rules.json"), *extra)
        if r.returncode:
            raise RuntimeError(f"split failed: {r.stdout}{r.stderr}")
        return r

    def hand_back(self, cell, verdicts=None, drop=(), short=None, extra=None):
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
               "unjudged": [], "malformed-headings": [], "bytes-read": acct, **(extra or {})}
        with open(c["out"], "w") as f:
            json.dump(out, f)

    def step15(self, unchecked=()):
        s = self.manifest()["step15"]
        acct = {f"specs/{F}/{p}": [os.path.getsize(os.path.join(self.root, "specs", F, p))] * 2
                for p in s["subjects"]}
        with open(s["out"], "w") as f:
            json.dump({"grounding": [], "integrity": [], "decision-reasons": [], "bytes-read": acct,
                       "integrity-checked": {i: i not in unchecked for i in ("code-block", "rationale", "system", "contract")}},
                      f)

    def all_hand_backs(self, verdicts_by_cell=None, unchecked=()):
        time.sleep(1.1)  # an output must be newer than the manifest's whole-second stamp
        for c in self.manifest()["cells"]:
            self.hand_back(c["cell"], (verdicts_by_cell or {}).get(c["cell"]))
        self.step15(unchecked)

    def calls(self, primitive):
        with open(os.path.join(self.stub, "calls.log")) as f:
            return [a for a in (json.loads(x) for x in f) if a[0] == primitive]

    def close(self):
        shutil.rmtree(self.base, ignore_errors=True)


def case(name, fn, **kw):
    try:
        p = Project(**kw)
    except Exception as e:
        fail(name, f"fixture: {e}")
        return
    try:
        fn(p, name)
    except Exception as e:  # a case that raises has not passed
        fail(name, f"raised {type(e).__name__}: {e}")
    finally:
        p.close()


def cell_a(p):
    return p.manifest()["cells"][0]["cell"]


def two_groups(p):
    """Split into one share and several scenario groups; the cells."""
    p.split("--cell-bytes", "6000", "--share-bytes", "100000")
    return [x["cell"] for x in p.manifest()["cells"]]


def a(p, name):
    p.write("stub/broken", "lint-markdown")
    r = p.run("detect.py", F)
    check_(name, r.returncode == 2 and "lint-markdown" in r.stderr
           and not os.path.exists(os.path.join(p.work, "det", "meta.json")), r.stderr)


def b(p, name):
    p.split("--share-bytes", "1500")
    m = p.manifest()
    files = [f for s in m["shares"].values() for f in s["rule-files"]]
    ids = sorted(i for s in m["shares"].values() for i in s["ids"])
    check_(name, len(m["shares"]) > 1 and sorted(files) == sorted(set(files)) and len(files) == 3
           and ids == ["RA-001", "RA-002", "RB-001", "RC-001"]
           and [x["id"] for x in m["reusable"]] == ["RC-002"], json.dumps(m["shares"])[:300])


def c(p, name):
    p.split("--cell-bytes", "1000000")
    one = len(p.manifest()["groups"])
    groups = (two_groups(p) and p.manifest()["groups"])
    flat = sorted(s for g in groups for s in g)
    check_(name, one == 1 and len(groups) > 1 and len(flat) == 3 and len(set(flat)) == 3
           and all(len(x["subjects"]) >= 3 for x in p.manifest()["cells"]), f"one={one} groups={groups}")


def d(p, name):
    rules = json.loads(json.dumps(RULES))
    rules["verdicts"]["judge"].append("RZ-404")
    p.write("rules.json", json.dumps(rules))
    r = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"))
    check_(name, r.returncode == 1 and "RZ-404" in r.stderr, r.stderr)


def rejudging(p):
    """Write a rules.json whose verdicts judge RA-001 and RB-001 in full and
    RA-002 and RC-001 against plan.md#Design alone."""
    rules = json.loads(json.dumps(RULES))
    rules["verdicts"] = {**rules["verdicts"], "reason": None, "judge": ["RA-001", "RB-001"],
                         "rejudge": ["RA-002", "RC-001"], "changed-sections": ["plan.md#Design"]}
    p.write("rules.json", json.dumps(rules))


def ak(p, name):
    rejudging(p)
    p.split("--share-bytes", "1500")
    r = p.run("briefs.py", F)
    m = p.manifest()
    ids = sorted(i for s in m["shares"].values() for i in s["ids"])
    with_ra = next(x for x in m["cells"] if "RA-002" in m["shares"][x["share"]]["ids"])
    without = [x for x in m["cells"] if not set(m["shares"][x["share"]]["ids"]) & {"RA-002", "RC-001"}]
    brief = open(with_ra["brief"]).read()
    check_(name, r.returncode == 0 and ids == ["RA-001", "RA-002", "RB-001", "RC-001"]
           and m["judge"] == ["RA-001", "RB-001"] and m["rejudge"] == ["RA-002", "RC-001"]
           and m["changed-sections"] == ["plan.md#Design"]
           and "Judged against the changed sections: `RA-002`" in brief and "`plan.md#Design`" in brief
           and all("every rule of your share is judged in full" in open(x["brief"]).read() for x in without),
           r.stdout + r.stderr + json.dumps([m.get("rejudge"), ids]))


def al(p, name):
    rejudging(p)
    p.split()
    unchanged = {"verdict": "finding", "at": "plan.md#Other", "message": "over judged text"}
    p.all_hand_backs({cell_a(p): {"RA-002": FND, "RC-001": unchanged}})
    p.write("plan.json", json.dumps({"findings": []}))
    r = p.run("record.py", F, p.plan, "--write")
    wa = (p.calls("write-analysis") or [[]])[0]
    try:
        rv = json.load(open(os.path.join(p.stub, "captured-rule-verdicts-json")))
        fs = json.load(open(os.path.join(p.stub, "captured-findings-json")))
    except OSError:
        rv, fs = {}, []
    keys = sorted(x["message"] for x in fs if x["family"] == "rule-assessment")
    judged = sorted(wa[i + 1] for i, x in enumerate(wa) if x == "--judged-rule")
    check_(name, r.returncode == 0 and keys == ["RA-002: not covered", "RC-002: a reused finding"]
           and rv.get("RC-001", {}).get("at") == "plan.md#Other"
           and judged == ["RA-001", "RA-002", "RB-001", "RC-001"]
           and wa[wa.index("--advisory") + 1] == "2" and "contradiction: RC-001" in r.stdout,
           r.stdout + r.stderr + json.dumps([rv, keys, wa]))


def am(p, name):
    rules = json.loads(json.dumps(RULES))
    reusable = rules["verdicts"]["reusable"]
    rules.update({"judge": ["RA-001", "RB-001"], "rejudge": ["RA-002", "RC-001"],
                  "changed-sections": ["plan.md#Design"], "reusable": reusable})
    rules["verdicts"] = {"reason": None, "judge-count": 2, "rejudge-count": 2,
                         "changed-sections-count": 1, "reusable-count": 1}
    p.write("rules.json", json.dumps(rules))
    p.split()
    m = p.manifest()
    ids = sorted(i for s in m["shares"].values() for i in s["ids"])
    check_(name, ids == ["RA-001", "RA-002", "RB-001", "RC-001"] and m["rejudge"] == ["RA-002", "RC-001"]
           and m["changed-sections"] == ["plan.md#Design"] and [x["id"] for x in m["reusable"]] == ["RC-002"],
           json.dumps([m.get("rejudge"), m.get("changed-sections"), ids]))


def an(p, name):
    rejudging(p)
    rules = json.loads(open(os.path.join(p.base, "rules.json")).read())
    rules["verdicts"]["reusable"].append({"id": "RC-001", "verdict": "waived", "reason": "r",
                                          "decided-by": "x"})
    p.write("rules.json", json.dumps(rules))
    p.split()
    p.all_hand_backs({cell_a(p): {"RC-001": {"verdict": "finding", "at": "plan.md#Other",
                                             "message": "after a waiver"}}})
    p.write("plan.json", json.dumps({"findings": []}))
    r = p.run("record.py", F, p.plan, "--write")
    try:
        fs = json.load(open(os.path.join(p.stub, "captured-findings-json")))
    except OSError:
        fs = []
    keys = sorted(x["message"] for x in fs if x["family"] == "rule-assessment")
    check_(name, r.returncode == 0 and "RC-001: after a waiver" in keys and "contradiction: RC-001" not in r.stdout,
           r.stdout + r.stderr + json.dumps(keys))


def e(p, name):
    p.split("--share-bytes", "1500")
    r = p.run("briefs.py", F)
    m = p.manifest()
    texts = [open(x["brief"]).read() for x in m["cells"]] + [open(m["step15"]["brief"]).read()]
    shares = "".join(open(s["file"]).read() for s in m["shares"].values())
    check_(name, r.returncode == 0 and not any("{{" in t for t in texts)
           and all(f"specs/{F}/{s}" in open(x["brief"]).read() for x in m["cells"] for s in x["subjects"])
           and "`RB-001`: a decided departure" in texts[0] and "Preamble of the A rules." in shares, r.stderr)


def f_(p, name):
    p.split()
    p.all_hand_backs()
    r = p.run("check.py", F)
    check_(name, r.returncode == 0, r.stdout + r.stderr)


def failing_check(mutate, needle, script="check.py"):
    def run(p, name):
        p.split()
        p.all_hand_backs()
        mutate(p)
        r = p.run(script, F)
        check_(name, r.returncode == 1 and needle in r.stdout + r.stderr and "Traceback" not in r.stderr,
               r.stdout + r.stderr)
    return run


def combine_case(p, name):
    cells = two_groups(p)
    verdicts = {cells[0]: {"RA-001": FND, "RA-002": FND,
                           "RC-001": {"verdict": "addressed", "cite": "scenarios/one.md#Behavior"}}}
    for x in cells[1:]:
        verdicts[x] = {"RA-002": FND, "RC-001": {"verdict": "addressed", "cite": "plan.md#Design"}}
    p.all_hand_backs(verdicts)
    r = p.run("combine.py", F)
    out = json.load(open(os.path.join(p.work, "merged.json"))) if r.returncode == 0 else {}
    v = out.get("verdicts", {})
    check_(name, r.returncode == 0 and len(cells) > 1 and "RA-001" in out["unsettled"] and "RA-001" not in v
           and v.get("RA-002", {}).get("tier") == "should"
           and v.get("RC-001", {}).get("cite") == "plan.md#Design", r.stdout + r.stderr)


def q(p, name):
    cells = [x for x in two_groups(p) if x.startswith("A")]
    p.write("stub/missing-headings", "Short\n")
    verdicts = {cells[0]: {"RA-002": {"verdict": "addressed", "cite": "plan.md#Short"}}}
    for x in cells[1:]:
        verdicts[x] = {"RA-002": {"verdict": "addressed", "cite": "scenarios/two.md#Behavior"}}
    p.all_hand_backs(verdicts)
    r = p.run("combine.py", F)
    out = json.load(open(os.path.join(p.work, "merged.json"))) if r.returncode == 0 else {}
    check_(name, r.returncode == 0 and len(cells) > 1
           and out["verdicts"].get("RA-002", {}).get("cite") == "scenarios/two.md#Behavior", r.stdout + r.stderr)


def record_setup(p, settle=True, unchecked=(), plan_extra=None):
    cells = two_groups(p)
    verdicts = {cells[0]: {"RA-001": FND, "RA-002": FND}}
    for x in cells[1:]:
        verdicts[x] = {"RA-002": FND}
    p.all_hand_backs(verdicts, unchecked)
    plan = {"findings": [{"tier": "advisory", "family": "grounding", "message": "a claim",
                          "path": f"specs/{F}/spec.md", "live": True}], **(plan_extra or {})}
    if settle:
        plan["settled"] = {"RA-001": None}
    p.write("plan.json", json.dumps(plan))


def n(p, name):
    record_setup(p, settle=False)
    r = p.run("record.py", F, p.plan, "--write")
    check_(name, r.returncode == 1 and "RA-001" in r.stderr, r.stdout + r.stderr)


def o(p, name):
    record_setup(p)
    p.answer("process-decisions", {
        "matched": [{"key": "grounding — a claim", "outcome": "waived", "reason": "stored", "by-rule": True}],
        "expired": [], "lapsed": []})
    r = p.run("record.py", F, p.plan, "--write")
    wa = (p.calls("write-analysis") or [[]])[0]
    pd = (p.calls("process-decisions") or [[]])[0]
    judged = sorted(wa[i + 1] for i, x in enumerate(wa) if x == "--judged-rule")
    try:
        rv = json.load(open(os.path.join(p.stub, "captured-rule-verdicts-json")))
        fs = json.load(open(os.path.join(p.stub, "captured-findings-json")))
        by = json.load(open(os.path.join(p.stub, "captured-decided-by-json")))
    except OSError:
        rv, fs, by = {}, [], None
    claim = next((x for x in fs if x["family"] == "grounding"), {})
    keys = sorted(x["message"] for x in fs if x["family"] == "rule-assessment")
    check_(name, r.returncode == 0 and judged == ["RA-001", "RA-002", "RB-001", "RC-001"]
           and "RA-001" not in rv and rv.get("RA-002", {}).get("tier") == "should"
           and claim.get("disposition", {}).get("outcome") == "waived" and by == EMAIL
           and keys == ["RA-002: not covered", "RC-002: a reused finding"]
           and "ships-to-adopter=1" in wa and "--restricted" not in pd
           and wa[wa.index("--advisory") + 1] == "3", r.stdout + r.stderr + json.dumps([rv, keys, wa]))


def p_(p, name):
    record_setup(p)
    p.git("config", "--unset", "user.email")
    r = p.run("record.py", F, p.plan, "--write")
    check_(name, r.returncode == 1 and "user.email" in r.stderr, r.stderr)


def r_(p, name):
    p.write("stub/slow", "read-spec")
    r = p.run("detect.py", F, ANALYZE_TIMEOUT_SECONDS="1")
    check_(name, r.returncode == 2 and "ANALYZE_TIMEOUT_SECONDS" in r.stderr, r.stderr)


def s(p, name):
    # `../specs/001-demo` names a real spec.md, so only the name check stops
    # it; with the default work directory it would stage under tmp/specs/.
    tmp = os.path.join(p.base, "tmp")
    os.makedirs(tmp)
    r1 = p.run("detect.py", f"../specs/{F}", ANALYZE_WORK="", TMPDIR=tmp)
    r2 = p.run("detect.py", F, ANALYZE_TIMEOUT_SECONDS="0")
    r3 = p.run("detect.py", F, ANALYZE_WORK="/")
    r4 = p.run("detect.py", F, ANALYZE_TIMEOUT_SECONDS="99999999")
    r5 = p.run("detect.py", F, DUCTUS_BIN=os.path.join(p.base, "no-such-ductus"))
    kept = os.path.exists(os.path.join(p.work, "det", "meta.json"))
    check_(name, r1.returncode == 2 and "not a feature directory name" in r1.stderr
           and not os.path.exists(os.path.join(tmp, "specs"))
           and r2.returncode == 2 and "whole number from 1 to 86400" in r2.stderr
           and r3.returncode == 2 and "ANALYZE_WORK" in r3.stderr
           and r4.returncode == 2 and "whole number from 1 to 86400" in r4.stderr
           and r5.returncode == 2 and "DUCTUS_BIN" in r5.stderr and kept,
           r1.stderr + r2.stderr + r3.stderr + r4.stderr + r5.stderr)


def u(p, name):
    record_setup(p, unchecked=("system",))
    rc = p.run("check.py", F)
    r = p.run("record.py", F, p.plan, "--write")
    wa = (p.calls("write-analysis") or [[]])[0]
    pd = (p.calls("process-decisions") or [[]])[0]
    check_(name, rc.returncode == 0 and "integrity item system was not checked" in rc.stdout
           and r.returncode == 0 and "integrity-not-checked=1" in wa and "--restricted" in pd,
           rc.stdout + r.stdout + r.stderr)


def ap(p, name):
    record_setup(p, unchecked=("contract",))
    rc = p.run("check.py", F)
    r = p.run("record.py", F, p.plan, "--write")
    wa = (p.calls("write-analysis") or [[]])[0]
    out_path = p.manifest()["step15"]["out"]
    out = json.load(open(out_path))
    del out["integrity-checked"]["contract"]
    time.sleep(1.1)
    json.dump(out, open(out_path, "w"))
    rm = p.run("check.py", F)
    check_(name, rc.returncode == 0 and "integrity item contract was not checked" in rc.stdout
           and r.returncode == 0 and "integrity-not-checked=1" in wa
           and rm.returncode == 1 and "integrity-checked does not give each of" in rm.stdout + rm.stderr,
           rc.stdout + r.stdout + r.stderr + rm.stdout + rm.stderr)


def v(p, name):
    record_setup(p)
    m = p.manifest()
    p.hand_back(m["cells"][0]["cell"], {"RA-001": FND, "RA-002": FND},
                extra={"malformed-headings": ["rules/a-backend.md line 1: '# A'"]})
    r1 = p.run("record.py", F, p.plan)
    p.write("plan.json", json.dumps({**json.load(open(p.plan)),
                                     "malformed-headings": ["rules/a-backend.md line 1: '# A'"]}))
    r2 = p.run("record.py", F, p.plan)
    check_(name, r1.returncode == 1 and "malformed" in r1.stderr and r2.returncode == 0, r1.stderr + r2.stderr)


def w_(p, name):
    record_setup(p)
    p.write(f"specs/{F}/tasks.md", "# Tasks\n\n## 1. Build\n\n- [x] Done, and edited\n")
    r = p.run("record.py", F, p.plan, "--write")
    check_(name, r.returncode == 1 and "detect.py" in r.stderr and not p.calls("write-analysis"), r.stderr)


def x(p, name):
    # The share and the spec and plan leave about 1,500 bytes of 6,300 for
    # scenarios of about 925: a first estimate of two groups would put two in
    # one cell, over the budget, so three are needed.
    p.split("--cell-bytes", "6300", "--share-bytes", "100000")
    m = p.manifest()
    check_(name, all(c["bytes"] <= 6300 for c in m["cells"]) and len(m["groups"]) == 3 and not m["notes"],
           json.dumps([(c["cell"], c["bytes"]) for c in m["cells"]]) + json.dumps(m["notes"]))


def aa(p, name):
    p.det("resolve-references", {"references": [{"service": "billing", "spec": "specs/004-x/spec.md",
                                                  "outcome": "broken", "status": None}],
                                 "duplicate-repos": [], "references-count": 1, "duplicate-repos-count": 0})
    record_setup(p)
    r = p.run("record.py", F, p.plan, "--write")
    try:
        fs = json.load(open(os.path.join(p.stub, "captured-findings-json")))
    except OSError:
        fs = []
    check_(name, r.returncode == 0 and any(f["family"] == "cross-service-references" and f["tier"] == "advisory"
                                           and "billing" in f["message"] for f in fs), r.stdout + r.stderr)


def ab(p, name):
    claim = {"tier": "advisory", "family": "grounding", "message": "a claim", "path": f"specs/{F}/spec.md"}
    record_setup(p, plan_extra={"findings": [claim, claim]})
    r = p.run("record.py", F, p.plan)
    check_(name, r.returncode == 1 and "share a key" in r.stderr, r.stderr)


def ac(p, name):
    p.split()
    p.all_hand_backs()
    os.remove(p.manifest()["step15"]["out"])
    r = p.run("combine.py", F)
    check_(name, r.returncode == 1 and "step15" in r.stderr, r.stderr)


def ad(p, name):
    p.split()
    p.write("stub/no-not-found", "")
    p.all_hand_backs({cell_a(p): {"RA-001": FND}})
    r = p.run("check.py", F)
    check_(name, r.returncode == 2 and "read-section returned no not-found list" in r.stderr, r.stdout + r.stderr)


def ae(p, name):
    p.split()
    m = p.manifest()
    m["cells"] = "not a list"
    with open(os.path.join(p.work, "manifest.json"), "w") as f:
        json.dump(m, f)
    r = p.run("check.py", F)
    check_(name, r.returncode == 2 and "the script failed" in r.stderr, r.stderr)


def af(p, name):
    refs = os.path.join(p.work, "det", "resolve-references.json")
    without = os.path.exists(refs) or any(c[0] == "resolve-references" for c in p.calls("resolve-references"))
    p.answer("read-spec", {"frontmatter": {"status": "in-progress",
                                           "references": [{"service": "billing", "spec": "x"}]}})
    p.answer("resolve-references", {"references": [], "duplicate-repos": []})
    r1 = p.run("detect.py", F)
    called = bool(p.calls("resolve-references")) and os.path.exists(refs)
    p.answer("read-spec", {"frontmatter": {"status": "in-progress"}})
    r2 = p.run("detect.py", F)
    check_(name, not without and r1.returncode == 0 and called and r2.returncode == 0 and not os.path.exists(refs),
           r1.stdout + r2.stdout)


def ag(p, name):
    p.write(f"specs/{F}/tasks.md", "# Tasks\n\n## 1. Build\n\n- [x] Done, and edited\n")
    r1 = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"))
    p.run("detect.py", F)
    p.write("other.txt", "unrelated\n")
    p.git("add", "other.txt")
    p.git("-c", "user.name=t", "commit", "-qm", "another")
    r2 = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"))
    p.run("detect.py", F)
    p.write(".ductus/constitution.md", "# Constitution\n\nAmended.\n")
    r3 = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"))
    check_(name, r1.returncode == 1 and "tasks.md" in r1.stderr and r2.returncode == 1 and "HEAD" in r2.stderr
           and r3.returncode == 1 and "constitution.md" in r3.stderr, r1.stderr + r2.stderr + r3.stderr)


STRICT = [  # (script, result, what the runtime always writes, the result without it)
    ("record.py", "check-orphaned-references", "skipped list", {"findings": []}),
    ("record.py", "check-artifacts", "skipped list", {"findings": []}),
    ("record.py", "resolve-anchor", "anchor-target-unjudgeable count", {"references": [], "anchor-target-unreadable": 0}),
    ("record.py", "resolve-anchor", "anchor-target-unreadable count", {"references": [], "anchor-target-unjudgeable": 0}),
    ("record.py", "check-artifact-size", "skipped list", {"oversized": []}),
    ("record.py", "check-rule-ids", "examined count", {"citations-count": 0}),
    ("record.py", "process-decisions", "matched list", {"expired": [], "lapsed": []}),
    ("record.py", "resolve-references", "references list", {"duplicate-repos": []}),
    ("record.py", "resolve-references", "duplicate-repos list", {"references": []}),
    ("split.py", "check-departures", "entries list", {"findings": []}),
    ("record.py", "process-decisions", "expired list", {"matched": [], "lapsed": []}),
    ("record.py", "process-decisions", "lapsed list", {"matched": [], "expired": []}),
]


def ah(p, name):
    bad = []
    for script, result, needle, value in STRICT:
        p.run("detect.py", F)
        record_setup(p)
        if result == "process-decisions":
            p.answer(result, value)
        else:
            p.det(result, value)
        if script == "split.py":
            r = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"))
        else:
            r = p.run("record.py", F, p.plan)
        if not (r.returncode == 2 and needle in r.stderr):
            bad.append(f"{result} {needle}: exit {r.returncode} {r.stderr[-200:]}")
        p.answer("process-decisions", {"matched": [], "expired": [], "lapsed": []})
    check_(name, not bad, "; ".join(bad))


def ai(p, name):
    p.write("notes.txt", "an untracked file, outside the feature\n")
    r = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"))
    p.run("detect.py", F)
    p.write("other.txt", "unrelated\n")
    p.git("add", "other.txt")
    p.git("-c", "user.name=t", "commit", "-qm", "another")
    p.run("detect.py", F)
    p.write("other.txt", "edited, tracked, outside the feature\n")
    r2 = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"))
    check_(name, r.returncode == 1 and "working tree" in r.stderr
           and r2.returncode == 1 and "working tree" in r2.stderr, r.stderr + r2.stderr)


def aj(p, name):
    r1 = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"), "--share-bytes", "0")
    r2 = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"), "--cell-bytes", "lots")
    r3 = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"),
               "--share-bytes", "1", "--cell-bytes", "1")
    check_(name, r1.returncode == 2 and "--share-bytes is '0'; it takes a whole number of at least 1" in r1.stderr
           and r2.returncode == 2 and "--cell-bytes is 'lots'; it takes a whole number of at least 1" in r2.stderr
           and r3.returncode == 0, r1.stderr + r2.stderr + r3.stderr)


def ao(p, name):
    ref = {"anchor": "X", "line": 3, "resolved": True, "kind": "qualified",
           "unjudged": "anchor-target-unjudgeable"}
    p.answer("resolve-anchor", {"references": [ref], "unresolved": ["Gone"], "anchor-target-unjudgeable": 1,
                                "anchor-target-unreadable": 0})
    r = p.run("detect.py", F)
    calls = [c for c in p.calls("resolve-anchor") if "--path" in c][-4:]
    paths = [c[c.index("--path") + 1] for c in calls]
    want = [f"specs/{F}/spec.md"] + [f"specs/{F}/scenarios/{n}.md" for n in ("one", "three", "two")]
    det = json.load(open(os.path.join(p.work, "det", "resolve-anchor.json")))
    tagged = sorted({x["path"] for x in det["references"]})
    record_setup(p)
    r2 = p.run("record.py", F, p.plan, "--write")
    wa = (p.calls("write-analysis") or [[]])[0]
    shutil.rmtree(os.path.join(p.root, "specs", F, "scenarios"))
    r3 = p.run("detect.py", F)
    none = json.load(open(os.path.join(p.work, "det", "resolve-anchor.json")))
    check_(name, r.returncode == 0 and paths == want and all("--markers-path" in c for c in calls)
           and det["scenarios-examined"] == 3 and len(det["references"]) == 4 and tagged == sorted(want)
           and det["unresolved"] == ["Gone", f"specs/{F}/scenarios/one.md: Gone",
                                     f"specs/{F}/scenarios/three.md: Gone", f"specs/{F}/scenarios/two.md: Gone"]
           and r2.returncode == 0 and "anchor-target-unjudgeable=4" in wa
           and r3.returncode == 0 and none["scenarios-examined"] == 0 and len(none["references"]) == 1,
           r.stdout + r.stderr + r2.stderr + r3.stderr + json.dumps([paths, det, wa]))


def aq(p, name):
    ref = {"anchor": "X", "line": 3, "resolved": True, "kind": "qualified",
           "unjudged": "anchor-target-unjudgeable"}
    p.answer("resolve-anchor", {"references": [ref], "unresolved": ["zzz"], "references-count": 1,
                                "unresolved-count": 1, "qualified": 2, "quoted-heading": 1,
                                "anchor-target-unjudgeable": 1, "anchor-target-unreadable": 0})
    p.write(f"specs/{F}/scenarios/Four.MD", "# four\n\n## Behavior\n\nx\n")
    p.write("stub/broken-paths", f"specs/{F}/scenarios/two.md\n")
    r = p.run("detect.py", F)
    det = json.load(open(os.path.join(p.work, "det", "resolve-anchor.json")))
    record_setup(p)
    r2 = p.run("record.py", F, p.plan, "--write")
    wa = (p.calls("write-analysis") or [[]])[0]
    paths = [c[c.index("--path") + 1] for c in p.calls("resolve-anchor") if "--path" in c][-5:]
    check_(name, r.returncode == 0 and f"specs/{F}/scenarios/Four.MD" in paths
           and det["scenarios-examined"] == 3 and det["anchor-target-unreadable"] == 1
           and f"resolve-anchor specs/{F}/scenarios/two.md: UNREADABLE" in r.stdout
           and det["references-count"] == 4 == len(det["references"])
           and det["unresolved-count"] == 4 and det["unresolved"] == sorted(det["unresolved"])
           and det["qualified"] == 8 and det["quoted-heading"] == 4 and det["anchor-target-unjudgeable"] == 4
           and r2.returncode == 0 and "anchor-target-unreadable=1" in wa and "anchor-target-unjudgeable=4" in wa,
           r.stdout + r.stderr + r2.stderr + json.dumps([det, wa]))


def z(name):
    """detect.py through record.py --write against the real release binary,
    in a fixture project holding one rule file and an unfolded staging spec."""
    if not os.access(REAL_BIN, os.X_OK):
        fail(name, f"no runtime at {REAL_BIN}; build it with `cd runtime && cargo build --release`")
        return
    base = tempfile.mkdtemp(prefix="analyze-real-")
    root = os.path.join(base, "repo")
    try:
        def write(rel, text):
            path = os.path.join(root, rel)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w") as f:
                f.write(text)
        write(f"specs/{F}/spec.md", "---\nstatus: in-progress\ndependencies: []\n---\n\n# 001 — Demo\n\n"
              "A demo feature.\n\n## Motivation\n\nWhy it exists, beside `BE-AUTHN-001`.\n\n"
              "## Acceptance Criteria\n\n"
              "- [x] AC1: The demo works\n\n## Open Questions\n\n## Resolved Questions\n")
        write(f"specs/{F}/plan.md", "# 001 — Demo Plan\n\n## Overview\n\nHow it works.\n\n"
              "## Technical Decisions\n\n### One decision\n\nTaken because it is simplest.\n")
        write(f"specs/{F}/tasks.md", "# 001 — Demo Tasks\n\n## 1. Build the demo\n\n- [x] Build it\n\n"
              "- **Done when**: it exists.\n")
        write("specs/feat.1-staging/spec.md", f"---\nstatus: draft\ndependencies: []\nfolds-into: {F}\n---\n\n"
              "# feat.1 — Staging\n\nStaged.\n\n## Acceptance Criteria\n\n- [ ] AC1: It folds\n\n"
              "## Open Questions\n\n## Resolved Questions\n")
        os.makedirs(os.path.join(root, "specs", "rules"))
        shutil.copy(os.path.join(REPO, "framework", "rules", "quality-cross.md"),
                    os.path.join(root, "specs", "rules", "quality-cross.md"))
        write(".ductus/constitution.md", "# Constitution\n")
        env = {k: v for k, v in os.environ.items() if not k.startswith(("GIT_", "ANALYZE_", "DUCTUS_"))}
        env.update({"DUCTUS_BIN": REAL_BIN, "ANALYZE_WORK": os.path.join(base, "work"),
                    "GIT_CONFIG_GLOBAL": os.devnull, "GIT_CONFIG_NOSYSTEM": "1"})
        for args in (["init", "-q"], ["config", "user.email", EMAIL], ["add", "."],
                     ["-c", "user.name=t", "commit", "-qm", "fixture"]):
            subprocess.run(["git", *args], cwd=root, env=env, check=True, capture_output=True)

        def run(script, *args):
            return subprocess.run([sys.executable, "-I", os.path.join(SCRIPTS, script), *args],
                                  cwd=root, env=env, capture_output=True, text=True)
        steps = [run("detect.py", F)]
        steps.append(run("split.py", F))
        steps.append(run("briefs.py", F))
        work = os.path.join(base, "work")
        if any(x.returncode for x in steps):
            fail(name, "".join(f"[{i} exit {x.returncode}] {x.stderr[-600:]}" for i, x in enumerate(steps)))
            return
        m = json.load(open(os.path.join(work, "manifest.json")))
        time.sleep(1.1)
        for c in m["cells"]:
            share = m["shares"][c["share"]]
            acct = {f"specs/{F}/{p}": [os.path.getsize(os.path.join(root, "specs", F, p))] * 2
                    for p in c["subjects"]}
            acct[share["file"]] = [share["bytes"]] * 2
            first = share["ids"][0]
            json.dump({"judged": share["ids"], "unjudged": [], "malformed-headings": [], "bytes-read": acct,
                       "verdicts": {first: {"verdict": "addressed", "cite": "plan.md#One decision"}}},
                      open(c["out"], "w"))
        s15 = m["step15"]
        json.dump({"grounding": [], "integrity": [], "decision-reasons": [],
                   "integrity-checked": {"code-block": True, "rationale": True, "system": True, "contract": True},
                   "bytes-read": {f"specs/{F}/{p}": [os.path.getsize(os.path.join(root, "specs", F, p))] * 2
                                  for p in s15["subjects"]}}, open(s15["out"], "w"))
        steps.append(run("check.py", F))
        steps.append(run("combine.py", F))
        with open(os.path.join(base, "plan.json"), "w") as f:
            json.dump({"findings": []}, f)
        steps.append(run("record.py", F, os.path.join(base, "plan.json"), "--write"))
        record = os.path.join(root, "specs", F, "analysis.md")
        text = open(record).read() if os.path.isfile(record) else ""
        first = m["shares"]["A"]["ids"][0] if m["shares"] else None
        good = (all(x.returncode == 0 for x in steps) and "feat.1-staging" in steps[0].stdout
                and len(m["judge"]) > 0 and sum(len(s["ids"]) for s in m["shares"].values()) == len(m["judge"])
                and "last-run:" in text and f"{first}:" in text and "plan.md#One decision" in text
                and "rule-not-installed" in text)
        check_(name, good, "".join(f"[{i} exit {x.returncode}] {x.stdout[-600:]}{x.stderr[-600:]}"
                                   for i, x in enumerate(steps)))
    finally:
        shutil.rmtree(base, ignore_errors=True)


def main():
    print("analyze scripts:", flush=True)
    case("A. detect.py exits 2 on a primitive with no result", a)
    case("B. split.py keeps rule files whole and judges no reusable rule", b)
    case("C. split.py groups overflowing scenarios, and only then", c)
    case("D. split.py refuses a judged rule no file holds", d)
    case("E. briefs.py fills every placeholder and carries heading bodies", e)
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
    case("R. a call past ANALYZE_TIMEOUT_SECONDS is stopped", r_)
    case("S. a malformed setting or feature name exits 2", s)
    case("T. check.py fails a subject added after the split", failing_check(
        lambda p: p.write(f"specs/{F}/scenarios/four.md", "# four\n\n## Behavior\n\nNew.\n"), "was added"))
    case("U. an unchecked integrity item is recorded and restricts", u)
    case("V. record.py refuses an unacknowledged malformed heading", v)
    case("W. record.py refuses detection older than the feature", w_)
    case("X. split.py keeps each cell within --cell-bytes", x)
    case("Y. check.py fails a hand-back of the wrong shape", failing_check(
        lambda p: p.hand_back(cell_a(p), extra={"verdicts": {"RA-001": ["not", "an", "object"]}}), "verdicts is not"))
    case("AA. record.py writes a broken reference as an Advisory finding", aa)
    case("AB. record.py refuses two findings sharing a key", ab)
    case("AC. combine.py refuses while step 15 fails its check", ac)
    case("AK. split.py and briefs.py carry the rules to judge against changed sections", ak)
    case("AL. record.py counts no finding at an unchanged section", al)
    case("AM. split.py reads rejudge from a file-form result", am)
    case("AN. record.py counts a finding after a waiver", an)
    case("AD. check.py exits 2 on a read-section with no not-found list", ad)
    case("AE. a defect in a script exits 2", ae)
    case("AF. detect.py runs resolve-references only for a spec with references", af)
    case("AG. split.py refuses detection older than an edit or a commit", ag)
    case("AH. record.py and split.py exit 2 on a result missing what the runtime writes", ah)
    case("AI. split.py refuses detection older than an untracked file", ai)
    case("AJ. split.py names the flag, value and bound of a bad budget", aj)
    case("AO. detect.py runs resolve-anchor on each scenario and counts them examined", ao)
    case("AQ. detect.py merges a scenario's counts and names one it cannot read", aq)
    case("AP. step 15 states the plan-against-spec judgment, or it is unchecked", ap)
    z("Z. the whole chain against the real release binary")
    if failures:
        print(f"{failures} failure(s)", file=sys.stderr)
        sys.exit(1)
    print("all passed")


if __name__ == "__main__":
    main()
