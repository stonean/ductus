#!/usr/bin/env python3
"""Combine the cells' answers into one verdict per rule (spec 060, scenario
rule-judging-runs-from-repo-scripts; docs/developing-ductus.md, Sizing an
analysis's judges).

usage: python3 -I scripts/analyze/combine.py FEATURE

Runs check.py's checks first and refuses on any failure. Then, for each rule,
over the cells of its share, one per scenario group:

- every cell not fired: not fired;
- a departure's rule fired in any cell: waived;
- every cell a finding: a finding, at the first cell's location, in the
  rule's tier as its share gives it;
- some cells a finding and some not: unsettled, since a cell that did not
  read a group's scenarios can miss what addresses a rule there. The host
  settles each with one fresh reader given the rule, the answers and the
  cited sections, and records the outcome in the plan's `settled` map;
  record.py refuses an unsettled rule the map does not name;
- otherwise addressed, with a cite that resolves, citing the spec, plan or
  data model where a cell does, else the first cell's scenario.

A rule counts as judged only when every cell of its share judged it.
Writes <work>/merged.json.

Exit status: 0 written, unsettled rules or not, each one printed; 1 when a
hand-back fails its check; 2 when the manifest cannot be read.
"""

import os
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import analyze_lib as lib  # noqa: E402
import check as checker  # noqa: E402


def merge(m, outs, unresolved=()):
    departures = {e.get("rule") for e in m.get("departures") or []}
    judged, unjudged, verdicts, unsettled, mixed = [], {}, {}, {}, {}
    for letter, share in m["shares"].items():
        cells = [c["cell"] for c in m["cells"] if c["share"] == letter]
        for rid in share["ids"]:
            reasons = [u.get("reason") for c in cells for u in outs[c].get("unjudged") or []
                       if u.get("id") == rid]
            if any(rid not in (outs[c].get("judged") or []) for c in cells):
                unjudged[rid] = "; ".join(r for r in reasons if r) or "not judged by every cell"
                continue
            judged.append(rid)
            answers = {c: (outs[c].get("verdicts") or {}).get(rid) for c in cells}
            kinds = {c: (v or {}).get("verdict") for c, v in answers.items()}
            if len(set(kinds.values())) > 1:
                mixed[rid] = kinds
            fired = {c: v for c, v in answers.items() if v}
            if not fired:
                continue
            if rid in departures:
                verdicts[rid] = {"verdict": "waived"}
                continue
            finding = [c for c, v in fired.items() if v["verdict"] == "finding"]
            if finding and len(finding) < len(cells):
                unsettled[rid] = answers
            elif finding:
                v = fired[finding[0]]
                verdicts[rid] = {"verdict": "finding", "tier": share["tiers"].get(rid),
                                 "at": v["at"], "message": v["message"]}
            else:
                addressed = [c for c, v in fired.items()
                             if v["verdict"] == "addressed" and v["cite"] not in unresolved]
                core = [c for c in addressed
                        if not fired[c]["cite"].partition("#")[0].startswith("scenarios/")]
                verdicts[rid] = {"verdict": "addressed", "cite": fired[(core or addressed)[0]]["cite"]}
    return {"judged": judged, "unjudged": unjudged, "verdicts": verdicts,
            "unsettled": unsettled, "mixed": mixed}


def run(argv):
    if len(argv) != 1:
        raise lib.Unreadable("usage: combine.py FEATURE")
    feature = argv[0]
    m = lib.load_manifest(feature)
    problems, outs, unresolved, _ = checker.check(feature, m)
    failed = sorted(k for k, v in problems.items() if v and k != "step15")
    if failed:
        raise lib.Refused(f"hand-backs fail their check: {', '.join(failed)}; run check.py")
    merged = merge(m, outs, unresolved)
    merged["step15"] = outs.get("step15") if not problems.get("step15") else None
    lib.write_json(os.path.join(lib.work_dir(feature), "merged.json"), merged)
    kinds = [v["verdict"] for v in merged["verdicts"].values()]
    print(f"== {feature}: judged {len(merged['judged'])}, unjudged {len(merged['unjudged'])}; "
          f"addressed {kinds.count('addressed')}, finding {kinds.count('finding')}, "
          f"waived {kinds.count('waived')}, unsettled {len(merged['unsettled'])}")
    for rid, v in merged["verdicts"].items():
        if v["verdict"] == "finding":
            print(f"  finding {rid} ({v['tier']}) at {v['at']}: {v['message']}")
    for rid, kinds in merged["mixed"].items():
        print(f"  cells differ on {rid}: {kinds}{'  UNSETTLED' if rid in merged['unsettled'] else ''}")
    for rid, reason in merged["unjudged"].items():
        print(f"  unjudged {rid}: {reason}")
    if problems.get("step15"):
        print(f"  step 15's hand-back fails its check: {problems['step15']}")


if __name__ == "__main__":
    lib.main(run)
