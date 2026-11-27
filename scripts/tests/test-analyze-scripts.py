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
  AR. split.py takes step 15's parts from check-artifact-size --project-files:
     each measured part of specs/system.md to read, each unreadable one apart,
     no other file's part, and exits 2 on a result with no parts list;
     briefs.py names each; check.py fails a step-15 read account missing a
     part; record.py writes each part-findings entry as an Advisory
     project-parts finding, and counts an unreadable part once, under
     artifact-unreadable (spec 087)
  AS. split.py gives the scenarios one reader when they fit --cell-bytes,
     several covering each scenario once when they do not, and none for a
     spec with no scenario; briefs.py fills each reader's brief from
     scenario-brief.md, naming its scenarios and Scenario consistency
     (advisory), and prints its prompt
  AT. check.py fails a scenario reader's hand-back that leaves a scenario
     unjudged, reads one short, gives a check other than context or
     behavior, or two findings for one scenario and check, and exits 2 on a
     manifest with no scenario-readers list
  AU. combine.py carries each scenario finding, and record.py writes it as
     an Advisory scenario-consistency finding keyed by its scenario and check,
     and refuses while a scenario reader's hand-back is missing
  AV. check.py, combine.py and record.py refuse a rule file edited or removed
     after split.py rendered the shares, once detection is re-run, and a
     manifest with no rule-file-digest map
  BT. record.py refuses a loaded rule file edited after the split whose
     rules are all reusable, so no share holds it
  AW. split.py refuses a judged or rejudged rule whose tier is neither must
     nor should, and lets an unjudged one be
  AX. record.py records a MUST-tier rule finding Blocking and a SHOULD-tier
     one Advisory, in the counts it passes write-analysis, and exits 2 on a
     tier of neither
  AY. split.py exits 2 on a result whose selected is absent or no list
  AZ. record.py refuses a settled verdict that is a finding with no at, a
     waiver with no departure entry, of an unknown kind, an addressed with no
     cite, or no object
  BA. record.py refuses a settled rule the run did not judge
  BB. record.py refuses a settled verdict whose cite resolves to no heading
  BC. check.py fails a departure's rule answered addressed or a finding
  BD. check.py fails a rule answered both judged and unjudged
  BE. check.py fails answers for rules outside the cell's share
  BF. check.py names a design subject removed after the split
  BG. check.py fails a scenario finding outside the reader's group
  BH. check.py fails a scenario judgment outside the reader's group
  BI. check.py fails a scenario finding with an empty message
  BJ. record.py counts a referrer check-orphaned-references could not read
     unexamined, which sets restricted
  BK. record.py counts the citations a check-rule-ids that examined none
     leaves unchecked unexamined, which sets restricted
  BL. combine.py records a fired departure's rule as waived
  BM. split.py notes a share larger than --share-bytes
  BN. briefs.py names the share's lines past 2,000 characters
  BO. briefs.py names a subject's lines past 2,000 characters
  BP. split.py refuses when a rule file could not be read
  BQ. record.py refuses a primitive's non-zero exit that carries a result
  BR. split.py records the loaded rule files, and briefs.py names them
  BS. check.py fails a hand-back that is not a JSON object, never with a
     traceback
  Z. the whole chain, detect.py through record.py --write, against the real
     release binary, its spec citing a shipped rule the fixture never
     installed, so check-rule-ids' not-installed is non-empty, and its
     system.md an index with a linked part and an unlinked one: both are
     step 15's subjects, and the unlinked one is a project-parts finding;
     its one scenario has a reader, whose finding the record holds as a
     scenario-consistency finding
     the manifest lists the loaded rule files, and every tier it holds is
     must or should, one at least must

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
if name in open(os.path.join(d, "failing")).read().split():
    p = os.path.join(d, name + ".json")
    print(open(p).read() if os.path.isfile(p) else "{}"); sys.exit(3)
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
        for rf in RULES["rule-files"]:
            self.write(rf["file"], f"# {rf['file']}\n")
        self.write("stub/ductus", STUB)
        os.chmod(os.path.join(self.stub, "ductus"), 0o755)
        for n in ("broken", "slow", "failing", "missing-headings", "broken-paths"):
            self.write(f"stub/{n}", "")
        self.answer("process-decisions", {"matched": [], "expired": [], "lapsed": []})
        self.answer("discover-rule-files", {"rules-dir": "rules", "selected": RULES["selected"],
                                            "notices": ["loading rule files: a, b, c"]})
        self.answer("check-departures", {"entries": [{"rule": "RB-001", "reason": "a decided departure"}]})
        self.answer("read-spec", {"frontmatter": {"status": "in-progress"}})
        self.answer("check-artifacts", {"skipped": [{"reason": "ships-to-adopter"}]})
        self.answer("check-rule-ids", {"examined": 3, "citations-count": 0})
        self.answer("check-artifact-size", {"skipped": [], "parts": [], "part-findings": []})
        self.answer("check-orphaned-references", {"skipped": []})
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

    def step15(self, unchecked=(), skip=()):
        """Write step 15's output: every subject, specs/system.md and each part
        split.py listed to read, read whole, but the paths in skip."""
        s = self.manifest()["step15"]
        acct = {f"specs/{F}/{p}": [os.path.getsize(os.path.join(self.root, "specs", F, p))] * 2
                for p in s["subjects"]}
        for p in ([s["system"]] if s.get("system") else []) + list(s.get("system-parts") or []):
            if p not in skip:
                acct[p] = [os.path.getsize(os.path.join(self.root, p))] * 2
        with open(s["out"], "w") as f:
            json.dump({"grounding": [], "integrity": [], "decision-reasons": [], "bytes-read": acct,
                       "integrity-checked": {i: i not in unchecked for i in ("code-block", "rationale", "system", "contract")}},
                      f)

    def scenarios(self, findings=(), drop=(), short=None):
        """Write each scenario reader's output: every scenario of its group
        judged and read whole but those in drop and the path in short, with
        each (scenario, check, message) in findings at the reader whose group
        holds it."""
        for r in self.manifest()["scenario-readers"]:
            acct = {f"specs/{F}/{s}": [os.path.getsize(os.path.join(self.root, "specs", F, s))] * 2
                    for s in r["subjects"]}
            if short in acct:
                acct[short][0] -= 10
            with open(r["out"], "w") as f:
                json.dump({"reader": r["reader"], "judged": [s for s in r["subjects"] if s not in drop],
                           "findings": [{"path": s, "check": c, "message": msg} for s, c, msg in findings
                                        if s in r["subjects"]],
                           "bytes-read": acct}, f)

    def all_hand_backs(self, verdicts_by_cell=None, unchecked=(), scenario_findings=()):
        time.sleep(1.1)  # an output must be newer than the manifest's whole-second stamp
        for c in self.manifest()["cells"]:
            self.hand_back(c["cell"], (verdicts_by_cell or {}).get(c["cell"]))
        self.step15(unchecked)
        self.scenarios(scenario_findings)

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
    ("record.py", "check-artifact-size", "skipped list", {"oversized": [], "part-findings": []}),
    ("record.py", "check-artifact-size", "part-findings list", {"oversized": [], "skipped": []}),
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


LINKED, LOOSE, GONE = (f".ductus/docs/system/{n}.md" for n in ("linked", "loose", "gone"))
PARTS = [
    {"path": LINKED, "project-file": "specs/system.md", "state": "measured", "bytes": 1, "linked": True},
    {"path": LOOSE, "project-file": "specs/system.md", "state": "measured", "bytes": 1, "linked": False},
    {"path": GONE, "project-file": "specs/system.md", "state": "unreadable"},
    {"path": ".ductus/docs/agents/howto.md", "project-file": "AGENTS.md", "state": "measured", "bytes": 1,
     "linked": True},
]
UNLINKED = {"kind": "unlinked-part", "path": LOOSE, "message": "loose.md is a part specs/system.md does not link"}


def ar(p, name):
    p.write("specs/system.md", "# System\n\nIn parts: [linked](../.ductus/docs/system/linked.md).\n")
    for part in ("system/linked.md", "system/loose.md", "agents/howto.md"):
        p.write(f".ductus/docs/{part}", f"# {part}\n\nContent.\n")
    p.git("add", ".")
    p.git("-c", "user.name=t", "commit", "-qm", "parts")
    p.answer("check-artifact-size", {"skipped": [{"path": GONE, "reason": "artifact-unreadable"}],
                                     "parts": PARTS, "part-findings": [UNLINKED]})
    det = p.run("detect.py", F)
    p.split()
    s = p.manifest()["step15"]
    briefs = p.run("briefs.py", F)
    brief = open(s["brief"]).read()
    time.sleep(1.1)  # an output must be newer than the manifest's whole-second stamp
    for c in p.manifest()["cells"]:
        p.hand_back(c["cell"])
    p.scenarios()
    p.step15(skip=(LOOSE,))
    short = p.run("check.py", F)
    p.step15()
    whole = p.run("check.py", F)
    p.write("plan.json", json.dumps({"findings": []}))
    rec = p.run("record.py", F, p.plan, "--write")
    wa = (p.calls("write-analysis") or [[]])[0]
    try:
        fs = json.load(open(os.path.join(p.stub, "captured-findings-json")))
    except OSError:
        fs = []
    found = [(f["tier"], f["message"], f["path"]) for f in fs if f["family"] == "project-parts"]
    p.answer("check-artifact-size", {"skipped": [], "part-findings": []})
    p.run("detect.py", F)
    bare = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"))
    check_(name, det.returncode == 0 and f"part-findings=[('unlinked-part', '{LOOSE}')]" in det.stdout
           and s["system"] == "specs/system.md" and s["system-parts"] == [LINKED, LOOSE]
           and s["system-parts-unreadable"] == [GONE]
           and briefs.returncode == 0 and f"`{LINKED}` (" in brief and f"`{LOOSE}` (" in brief
           and f"`{GONE}`, a part of `specs/system.md` that `check-artifact-size` could not read" in brief
           and "howto.md" not in brief
           and short.returncode == 1 and f"no read account for {LOOSE}" in short.stdout
           and whole.returncode == 0
           and rec.returncode == 0 and found == [("advisory", UNLINKED["message"], LOOSE)]
           and wa[wa.index("--advisory") + 1] == "2" and "artifact-unreadable=1" in wa
           and not any(x.startswith("integrity-not-checked") for x in wa)
           and bare.returncode == 2 and "no parts list" in bare.stderr,
           det.stdout + briefs.stderr + short.stdout + whole.stdout + rec.stdout + rec.stderr + bare.stderr
           + json.dumps([s, found, wa]))


SCENARIOS = [f"scenarios/{n}.md" for n in ("one", "three", "two")]


def as_(p, name):
    p.split()
    one = p.manifest()["scenario-readers"]
    briefs = p.run("briefs.py", F)
    brief = open(one[0]["brief"]).read() if one and os.path.isfile(one[0]["brief"]) else ""
    p.split("--cell-bytes", "1000")
    many = p.manifest()["scenario-readers"]
    flat = sorted(s for r in many for s in r["subjects"])
    shutil.rmtree(os.path.join(p.root, "specs", F, "scenarios"))
    p.run("detect.py", F)
    p.split()
    none = p.manifest()["scenario-readers"]
    check_(name, len(one) == 1 and one[0]["subjects"] == SCENARIOS
           and briefs.returncode == 0 and "{{" not in brief and "§Scenario consistency (advisory)" in brief
           and all(f"`specs/{F}/{s}` (" in brief for s in SCENARIOS)
           and "scenario reader(s) first" in briefs.stdout and one[0]["brief"] in briefs.stdout
           and len(many) == 3 and flat == sorted(SCENARIOS) and none == [],
           briefs.stdout + briefs.stderr + json.dumps([one, many, none]))


def at(p, name):
    p.split()
    p.all_hand_backs()
    whole = p.run("check.py", F)
    bad = {}
    for label, kw in (("unjudged", {"drop": ("scenarios/two.md",)}),
                      ("short", {"short": f"specs/{F}/scenarios/one.md"}),
                      ("check", {"findings": [("scenarios/one.md", "edge-cases", "m")]}),
                      ("twice", {"findings": [("scenarios/one.md", "context", "a"),
                                              ("scenarios/one.md", "context", "b")]})):
        p.scenarios(**kw)
        r = p.run("check.py", F)
        bad[label] = (r.returncode, r.stdout + r.stderr)
    p.scenarios()
    mpath = os.path.join(p.work, "manifest.json")
    m = p.manifest()
    del m["scenario-readers"]
    with open(mpath, "w") as f:
        json.dump(m, f)
    old = p.run("check.py", F)
    check_(name, whole.returncode == 0 and "scenarios-1: ok" in whole.stdout
           and bad["unjudged"][0] == 1 and "no judgment of ['scenarios/two.md']" in bad["unjudged"][1]
           and bad["short"][0] == 1 and f"specs/{F}/scenarios/one.md read" in bad["short"][1]
           and bad["check"][0] == 1 and "check 'edge-cases' is none of" in bad["check"][1]
           and bad["twice"][0] == 1 and "two context findings" in bad["twice"][1]
           and old.returncode == 2 and "no scenario-readers list" in old.stderr,
           whole.stdout + json.dumps(bad) + old.stderr)


def au(p, name):
    record_setup(p)
    p.scenarios([("scenarios/one.md", "context", "has no Context section")])
    comb = p.run("combine.py", F)
    merged = json.load(open(os.path.join(p.work, "merged.json"))) if comb.returncode == 0 else {}
    r = p.run("record.py", F, p.plan, "--write")
    wa = (p.calls("write-analysis") or [[]])[0]
    try:
        fs = json.load(open(os.path.join(p.stub, "captured-findings-json")))
    except OSError:
        fs = []
    found = [f for f in fs if f["family"] == "scenario-consistency"]
    os.remove(p.manifest()["scenario-readers"][0]["out"])
    gone = p.run("record.py", F, p.plan)
    check_(name, comb.returncode == 0 and [f["path"] for f in merged.get("scenarios", [])] == ["scenarios/one.md"]
           and r.returncode == 0 and found == [{
               "tier": "advisory", "family": "scenario-consistency", "path": f"specs/{F}/scenarios/one.md",
               "live": True, "message": "scenarios/one.md: has no Context section",
               "decision-key": f"scenario-consistency — specs/{F}/scenarios/one.md: context",
               "disposition": {"outcome": "undispositioned"}}]
           and wa[wa.index("--advisory") + 1] == "4" and "3 scenarios judged" in r.stdout
           and gone.returncode == 1 and "scenarios-1" in gone.stderr,
           comb.stdout + comb.stderr + r.stdout + r.stderr + gone.stderr + json.dumps([found, wa]))


def edit_reader(p, fn):
    """Rewrite the first scenario reader's output through fn."""
    out_path = p.manifest()["scenario-readers"][0]["out"]
    with open(out_path) as f:
        out = json.load(f)
    fn(out)
    with open(out_path, "w") as f:
        json.dump(out, f)


def captured(p, name):
    try:
        with open(os.path.join(p.stub, f"captured-{name}")) as f:
            return json.load(f)
    except OSError:
        return None


def av(p, name):
    p.split()
    p.all_hand_backs()
    p.write("plan.json", json.dumps({"findings": []}))
    rule = "rules/a-backend.md"
    p.write(rule, "# rules/a-backend.md\n\nAn edit after the judges read the share.\n")
    p.run("detect.py", F)  # detection is current again: only the shares predate the edit
    edited = {s: p.run(s, F, *extra) for s, extra in (("check.py", ()), ("combine.py", ()),
                                                      ("record.py", (p.plan,)))}
    p.write(rule, f"# {rule}\n")
    p.run("detect.py", F)
    restored = [p.run("check.py", F), p.run("combine.py", F), p.run("record.py", F, p.plan)]
    os.remove(os.path.join(p.root, rule))
    p.run("detect.py", F)
    gone = p.run("check.py", F)
    m = p.manifest()
    del m["rule-file-digest"]
    with open(os.path.join(p.work, "manifest.json"), "w") as f:
        json.dump(m, f)
    old = p.run("check.py", F)
    check_(name, all(r.returncode == 1 and rule in r.stderr and "has changed" in r.stderr
                     and "split.py" in r.stderr and "Traceback" not in r.stderr for r in edited.values())
           and all(r.returncode == 0 for r in restored)
           and gone.returncode == 1 and f"{rule} is gone" in gone.stderr
           and old.returncode == 2 and "rule-file-digest" in old.stderr,
           json.dumps({k: (v.returncode, v.stderr[-200:]) for k, v in edited.items()})
           + "".join(r.stderr[-200:] for r in restored) + gone.stderr + old.stderr)


def bt(p, name):
    # RD-001 is the only rule of d-cross.md and is reusable: no share holds the file.
    rules = json.loads(json.dumps(RULES))
    rules["selected"] = RULES["selected"] + ["d-cross.md"]
    rules["rule-files"].append({"file": "rules/d-cross.md", "other-headings": [], "rules": [
        {"id": "RD-001", "heading": "### RD-001", "tier": "should", "statement": "S", "verification": "V"}]})
    rules["verdicts"]["reusable"].append({"id": "RD-001", "verdict": "finding", "tier": "should",
                                          "at": "spec.md#Motivation", "message": "carried over"})
    p.write("rules.json", json.dumps(rules))
    p.write("rules/d-cross.md", "# rules/d-cross.md\n")
    p.answer("discover-rule-files", {"rules-dir": "rules", "selected": rules["selected"], "notices": []})
    p.run("detect.py", F)
    p.split()
    p.all_hand_backs()
    p.write("plan.json", json.dumps({"findings": []}))
    p.write("rules/d-cross.md", "# rules/d-cross.md\n\nEdited after the split.\n")
    p.run("detect.py", F)
    r = p.run("record.py", F, p.plan, "--write")
    check_(name, "rules/d-cross.md" in p.manifest()["rule-file-digest"]
           and r.returncode == 1 and "rules/d-cross.md has changed" in r.stderr and not p.calls("write-analysis"),
           r.stdout + r.stderr)


def aw(p, name):
    bad = []
    for label, rejudge, rid, tier in (("judged", False, "RA-001", "may"), ("no tier", False, "RB-001", None),
                                      ("rejudged", True, "RA-002", "may")):
        if rejudge:
            rejudging(p)
        else:
            p.write("rules.json", json.dumps(RULES))
        rules = json.load(open(os.path.join(p.base, "rules.json")))
        for rf in rules["rule-files"]:
            for r in rf["rules"]:
                if r["id"] == rid:
                    r["tier"] = tier
        p.write("rules.json", json.dumps(rules))
        r = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"))
        if not (r.returncode == 1 and f"{rid} has tier {tier!r}" in r.stderr and "Traceback" not in r.stderr):
            bad.append(f"{label}: exit {r.returncode} {r.stderr[-200:]}")
    rules = json.loads(json.dumps(RULES))
    rules["rule-files"][2]["rules"][1]["tier"] = None  # RC-002 is reusable, judged by nobody
    p.write("rules.json", json.dumps(rules))
    r = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"))
    check_(name, not bad and r.returncode == 0, "; ".join(bad) + r.stderr)


def ax(p, name):
    p.split()
    p.all_hand_backs({"A": {"RA-001": FND, "RA-002": FND}})
    p.write("plan.json", json.dumps({"findings": []}))
    r = p.run("record.py", F, p.plan, "--write")
    wa = (p.calls("write-analysis") or [[]])[0]
    tiers = {x["message"]: x["tier"] for x in captured(p, "findings-json") or [] if x["family"] == "rule-assessment"}
    m = p.manifest()
    m["shares"]["A"]["tiers"]["RA-001"] = "may"
    with open(os.path.join(p.work, "manifest.json"), "w") as f:
        json.dump(m, f)
    calls = len(p.calls("write-analysis"))
    odd = p.run("record.py", F, p.plan, "--write")
    check_(name, r.returncode == 0
           and tiers == {"RA-001: not covered": "blocking", "RA-002: not covered": "advisory",
                         "RC-002: a reused finding": "advisory"}
           and wa[wa.index("--blocking-findings") + 1] == "1" and wa[wa.index("--advisory") + 1] == "2"
           and odd.returncode == 2 and "RA-001 has tier 'may'" in odd.stderr and "Traceback" not in odd.stderr
           and len(p.calls("write-analysis")) == calls,
           r.stdout + r.stderr + json.dumps([tiers, wa]) + odd.stderr)


def ay(p, name):
    bad = []
    for label, value in (("absent", None), ("a string", "a-backend.md")):
        rules = json.loads(json.dumps(RULES))
        if value is None:
            del rules["selected"]
        else:
            rules["selected"] = value
        p.write("rules.json", json.dumps(rules))
        r = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"))
        if not (r.returncode == 2 and "no selected list" in r.stderr and "Traceback" not in r.stderr):
            bad.append(f"{label}: exit {r.returncode} {r.stderr[-200:]}")
    check_(name, not bad, "; ".join(bad))


def az(p, name):
    record_setup(p)
    bad = []
    for label, value, needle in (
            ("a finding with no at", {"verdict": "finding", "message": "m"}, "RA-001: a finding needs an at and a message"),
            ("a waiver with no departure entry", {"verdict": "waived"}, "RA-001: waived, but the spec holds no departure"),
            ("an unknown verdict", {"verdict": "bogus"}, "RA-001: verdict 'bogus' is none of"),
            ("an addressed with no cite", {"verdict": "addressed"}, "RA-001: addressed with no cite"),
            ("a verdict that is no object", "finding", "RA-001: a verdict is an object")):
        p.write("plan.json", json.dumps({"findings": [], "settled": {"RA-001": value}}))
        r = p.run("record.py", F, p.plan, "--write")
        if not (r.returncode == 1 and f"settled {needle}" in r.stderr and "Traceback" not in r.stderr
                and not p.calls("write-analysis")):
            bad.append(f"{label}: exit {r.returncode} {r.stderr[-200:]}")
    p.write("plan.json", json.dumps({"findings": [], "settled": {"RA-001": {**FND, "message": "settled"}}}))
    good = p.run("record.py", F, p.plan, "--write")
    check_(name, not bad and good.returncode == 0 and p.calls("write-analysis"),
           "; ".join(bad) + good.stderr)


def ba(p, name):
    record_setup(p)
    p.write("plan.json", json.dumps({"findings": [], "settled": {"RA-001": None, "RZ-9": None}}))
    r = p.run("record.py", F, p.plan)
    check_(name, r.returncode == 1 and "settled names RZ-9, which this run did not judge" in r.stderr, r.stderr)


def bb(p, name):
    record_setup(p)
    p.write("stub/missing-headings", "Gone\n")
    p.write("plan.json", json.dumps({"findings": [], "settled": {"RA-001": {**FND, "at": "plan.md#Gone"}}}))
    r = p.run("record.py", F, p.plan)
    check_(name, r.returncode == 1 and "verdicts cite headings that do not resolve" in r.stderr
           and "plan.md#Gone" in r.stderr, r.stdout + r.stderr)


def bc(p, name):
    p.split()
    p.all_hand_backs()
    bad = []
    for v in ({"verdict": "addressed", "cite": "plan.md#Design"}, FND):
        p.hand_back(cell_a(p), {"RB-001": v})
        r = p.run("check.py", F)
        if not (r.returncode == 1 and "RB-001: " + v["verdict"] + ", but its departure entry makes a fired one waived"
                in r.stdout):
            bad.append(f"{v['verdict']}: exit {r.returncode} {r.stdout[-200:]}")
    check_(name, not bad, "; ".join(bad))


def bj_bk(result, value, entry):
    def run(p, name):
        record_setup(p)
        p.det(result, value)
        r = p.run("record.py", F, p.plan, "--write")
        wa = (p.calls("write-analysis") or [[]])[0]
        pd = (p.calls("process-decisions") or [[]])[0]
        check_(name, r.returncode == 0 and entry in wa and "--restricted" in pd, r.stdout + r.stderr + json.dumps(wa))
    return run


def bl(p, name):
    p.split()
    p.all_hand_backs({"A": {"RB-001": {"verdict": "waived"}}})
    r = p.run("combine.py", F)
    out = json.load(open(os.path.join(p.work, "merged.json"))) if r.returncode == 0 else {}
    check_(name, r.returncode == 0 and out["verdicts"].get("RB-001") == {"verdict": "waived"}, r.stdout + r.stderr)


def bm(p, name):
    p.split("--share-bytes", "100")
    notes = p.manifest()["notes"]
    check_(name, any("over --share-bytes 100" in n and "a rule file is larger than a share" in n for n in notes),
           json.dumps(notes))


def bn(p, name):
    rules = json.loads(json.dumps(RULES))
    rules["rule-files"][0]["rules"][1]["statement"] = "L" * 2500
    p.write("rules.json", json.dumps(rules))
    p.split()
    r = p.run("briefs.py", F)
    brief = open(p.manifest()["cells"][0]["brief"]).read()
    check_(name, r.returncode == 0 and p.manifest()["shares"]["A"]["long-lines"]
           and "Lines of the share past 2,000 characters" in brief, r.stderr + brief[-600:])


def bo(p, name):
    p.write(f"specs/{F}/plan.md", "# Plan\n\n## Design\n\n" + "y" * 2500 + "\n")
    p.run("detect.py", F)
    p.split()
    r = p.run("briefs.py", F)
    brief = open(p.manifest()["cells"][0]["brief"]).read()
    check_(name, r.returncode == 0 and "These lines run past 2,000 characters" in brief
           and f"`specs/{F}/plan.md` line 5" in brief, r.stderr + brief[-600:])


def bp(p, name):
    rules = json.loads(json.dumps(RULES))
    rules["unreadable"] = ["rules/gone-backend.md"]
    p.write("rules.json", json.dumps(rules))
    r = p.run("split.py", F, "--rules-json", os.path.join(p.base, "rules.json"))
    check_(name, r.returncode == 1 and "rule files could not be read" in r.stderr and "gone-backend" in r.stderr,
           r.stderr)


def bq(p, name):
    record_setup(p)
    p.write("stub/failing", "process-decisions")
    r = p.run("record.py", F, p.plan, "--write")
    check_(name, r.returncode == 1 and "process-decisions exited 3" in r.stderr and not p.calls("write-analysis"),
           r.stdout + r.stderr)


def br(p, name):
    p.split()
    r = p.run("briefs.py", F)
    brief = open(p.manifest()["cells"][0]["brief"]).read()
    check_(name, r.returncode == 0 and p.manifest()["loaded"] == RULES["selected"]
           and "`a-backend.md`, `b-cross.md`, `c-frontend.md`" in brief, r.stderr + brief[-600:])


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
              "- **Done when**: it exists.\n\n## 2. The edge (scenario edge)\n\n"
              "- [x] Implement the behavior described in `scenarios/edge.md`\n\n"
              "- **Done when**: the edge is handled.\n")
        write(f"specs/{F}/scenarios/edge.md", "---\nsection: \"Acceptance Criteria\"\n---\n\n# Edge\n\n"
              "## Context\n\nAn edge.\n\n## Behavior\n\n- The demo handles it.\n")
        write("specs/feat.1-staging/spec.md", f"---\nstatus: draft\ndependencies: []\nfolds-into: {F}\n---\n\n"
              "# feat.1 — Staging\n\nStaged.\n\n## Acceptance Criteria\n\n- [ ] AC1: It folds\n\n"
              "## Open Questions\n\n## Resolved Questions\n")
        os.makedirs(os.path.join(root, "specs", "rules"))
        shutil.copy(os.path.join(REPO, "framework", "rules", "quality-cross.md"),
                    os.path.join(root, "specs", "rules", "quality-cross.md"))
        write(".ductus/constitution.md", "# Constitution\n")
        write("specs/system.md", "# System\n\nThe architecture, in parts: [the one](../.ductus/docs/system/linked.md).\n")
        write(".ductus/docs/system/linked.md", "# Linked part\n\nArchitecture.\n")
        write(".ductus/docs/system/unlinked.md", "# Unlinked part\n\nMore architecture.\n")
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
        read15 = {f"specs/{F}/{p}": [os.path.getsize(os.path.join(root, "specs", F, p))] * 2
                  for p in s15["subjects"]}
        for p in ([s15["system"]] if s15.get("system") else []) + list(s15.get("system-parts") or []):
            read15[p] = [os.path.getsize(os.path.join(root, p))] * 2
        json.dump({"grounding": [], "integrity": [], "decision-reasons": [],
                   "integrity-checked": {"code-block": True, "rationale": True, "system": True, "contract": True},
                   "bytes-read": read15}, open(s15["out"], "w"))
        for r in m["scenario-readers"]:
            json.dump({"reader": r["reader"], "judged": r["subjects"],
                       "findings": [{"path": "scenarios/edge.md", "check": "context",
                                     "message": "its Context names no situation"}],
                       "bytes-read": {f"specs/{F}/{p}": [os.path.getsize(os.path.join(root, "specs", F, p))] * 2
                                      for p in r["subjects"]}}, open(r["out"], "w"))
        steps.append(run("check.py", F))
        steps.append(run("combine.py", F))
        with open(os.path.join(base, "plan.json"), "w") as f:
            json.dump({"findings": []}, f)
        steps.append(run("record.py", F, os.path.join(base, "plan.json"), "--write"))
        record = os.path.join(root, "specs", F, "analysis.md")
        text = open(record).read() if os.path.isfile(record) else ""
        first = m["shares"]["A"]["ids"][0] if m["shares"] else None
        tiers = {t for s in m["shares"].values() for t in s["tiers"].values()}
        good = (all(x.returncode == 0 for x in steps) and "feat.1-staging" in steps[0].stdout
                and len(m["judge"]) > 0 and sum(len(s["ids"]) for s in m["shares"].values()) == len(m["judge"])
                and "last-run:" in text and f"{first}:" in text and "plan.md#One decision" in text
                and "rule-not-installed" in text
                and s15.get("system-parts") == [".ductus/docs/system/linked.md", ".ductus/docs/system/unlinked.md"]
                and "project-parts" in text and ".ductus/docs/system/unlinked.md" in text
                and [r["subjects"] for r in m["scenario-readers"]] == [["scenarios/edge.md"]]
                and "scenario-consistency" in text and "its Context names no situation" in text
                and m["loaded"] and "must" in tiers and tiers <= {"must", "should"})
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
    case("AR. step 15 reads specs/system.md's parts, and each part finding is Advisory", ar)
    case("AS. split.py and briefs.py give the scenarios their readers", as_)
    case("AT. check.py holds each scenario reader to every scenario of its group", at)
    case("AU. record.py writes each scenario reader's finding as Advisory", au)
    case("AV. a rule file edited or removed after the split stops check, combine and record", av)
    case("BT. a loaded rule file with only reusable rules, edited after the split, stops record.py", bt)
    case("AW. split.py refuses a judged or rejudged rule whose tier is neither must nor should", aw)
    case("AX. record.py records a MUST-tier finding Blocking and a SHOULD-tier one Advisory", ax)
    case("AY. split.py exits 2 on a result with no selected list", ay)
    case("AZ. record.py refuses a settled verdict that fails the checks a cell's does", az)
    case("BA. record.py refuses a settled rule the run did not judge", ba)
    case("BB. record.py refuses a settled verdict whose cite resolves to no heading", bb)
    case("BC. check.py fails a departure's rule answered addressed or a finding", bc)
    case("BD. check.py fails a rule answered both judged and unjudged", failing_check(
        lambda p: p.hand_back(cell_a(p), extra={"unjudged": [{"id": "RA-001", "reason": "r"}]}),
        "both judged and unjudged"))
    case("BE. check.py fails answers for rules outside the cell's share", failing_check(
        lambda p: p.hand_back(cell_a(p), extra={"judged": p.manifest()["shares"]["A"]["ids"] + ["RZ-9"]}),
        "answers for rules outside the share"))
    case("BF. check.py names a design subject removed after the split", failing_check(
        lambda p: os.remove(os.path.join(p.root, "specs", F, "scenarios", "two.md")),
        "scenarios/two.md was removed since split.py ran"))
    case("BG. check.py fails a scenario finding outside the reader's group", failing_check(
        lambda p: edit_reader(p, lambda o: o["findings"].append(
            {"path": "scenarios/nine.md", "check": "context", "message": "m"})),
        "a finding at scenarios/nine.md, outside this reader's group"))
    case("BH. check.py fails a scenario judgment outside the reader's group", failing_check(
        lambda p: edit_reader(p, lambda o: o["judged"].append("scenarios/nine.md")),
        "judgments of scenarios outside this reader's group"))
    case("BI. check.py fails a scenario finding with an empty message", failing_check(
        lambda p: edit_reader(p, lambda o: o["findings"].append(
            {"path": "scenarios/one.md", "check": "context", "message": "  "})),
        "scenarios/one.md: a finding needs a message"))
    case("BJ. record.py records a referrer it could not read as unexamined and restricted", bj_bk(
        "check-orphaned-references", {"skipped": [{"path": "specs/x.md", "reason": "unreadable"}]},
        "referrer-unreadable=1"))
    case("BK. record.py records citations no rule check examined as unexamined and restricted", bj_bk(
        "check-rule-ids", {"examined": 0, "citations-count": 2}, "rule-citations-not-checked=2"))
    case("BL. combine.py records a fired departure's rule as waived", bl)
    case("BM. split.py notes a share larger than --share-bytes", bm)
    case("BN. briefs.py names the share's lines past 2,000 characters", bn)
    case("BO. briefs.py names a subject's lines past 2,000 characters", bo)
    case("BP. split.py refuses when a rule file could not be read", bp)
    case("BQ. a primitive's non-zero exit with a result is refused, not read", bq)
    case("BR. split.py records the loaded rule files and briefs.py names them", br)
    case("BS. check.py fails a hand-back that is not a JSON object, never with a traceback", failing_check(
        lambda p: open(p.manifest()["cells"][0]["out"], "w").write("[1]"), "output is not a JSON object"))
    z("Z. the whole chain against the real release binary")
    if failures:
        print(f"{failures} failure(s)", file=sys.stderr)
        sys.exit(1)
    print("all passed")


if __name__ == "__main__":
    main()
