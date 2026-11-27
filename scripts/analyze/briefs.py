#!/usr/bin/env python3
"""Write each judge's brief, the step-15 reader's brief and each scenario
reader's brief from the cells manifest and the templates beside this
script, and print the prompt to start each subagent with (spec 060,
scenario rule-judging-runs-from-repo-scripts).

usage: python3 -I scripts/analyze/briefs.py FEATURE

The templates point each judge at the canonical text, steps 11, 12 and 15
of framework/commands/analyze.md and the reference sections they name, a
scenario reader at framework/reference/analyze.md's Scenario consistency
(advisory), and add only the run's facts: the subjects, the share, the
loaded rule files, the departure entries, the lines the Read tool may cut,
and the JSON check.py reads back.

Exit status: 0 written; 1 a template placeholder was left unfilled; 2 the
manifest or a template could not be read.
"""

import os
import re
import sys

sys.dont_write_bytecode = True
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import analyze_lib as lib  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))


def template(name):
    try:
        with open(os.path.join(HERE, name), encoding="utf-8") as f:
            return f.read()
    except OSError as e:
        raise lib.Unreadable(f"{name}: {e}") from e


def fill(text, values):
    for k, v in values.items():
        text = text.replace("{{" + k + "}}", v)
    left = sorted(set(re.findall(r"\{\{[A-Z0-9_]+\}\}", text)))
    if left:
        raise lib.Refused(f"unfilled placeholders: {left}")
    return text


def subject_lines(m, paths):
    feature = m["feature"]
    return "\n".join(f"- `specs/{feature}/{p}` ({m['subjects'][p]['bytes']:,} bytes)" for p in paths)


def long_note(m, paths):
    hits = [f"`specs/{m['feature']}/{p}` line {', '.join(map(str, m['subjects'][p]['long-lines']))}"
            for p in paths if m["subjects"].get(p, {}).get("long-lines")]
    if not hits:
        return ""
    return (f"These lines run past {lib.LONG_LINE:,} characters, which the Read tool may cut. Read each with "
            "`sed -n '<n>p' <file> | fold -w 1000`: " + "; ".join(hits) + ".")


def departures(m):
    entries = m.get("departures") or []
    if not entries:
        return "the spec holds no departure entry, so no rule is `waived`."
    lines = ["the spec's `## Applicable Rules` holds these well-formed departure entries. "
             "A fired one is `waived`, and one whose trigger does not fire is not fired."]
    for e in entries:
        lines.append(f"  - `{e.get('rule')}`: {(e.get('reason') or '').strip()}")
    return "\n".join(lines)


def rejudged(m, share):
    """The share's rules to judge against the changed sections alone (spec 082, AC7)."""
    ids = [i for i in share["ids"] if i in set(m.get("rejudge") or [])]
    if not ids:
        return "every rule of your share is judged in full."
    sections = ", ".join(f"`{s}`" for s in m.get("changed-sections") or []) or "none"
    return ("these rules of your share keep their stored verdict for every section but the changed "
            "ones, so judge each against the changed sections alone, reading the rest for context: "
            "it fires only when a change makes it fire or removes what addressed it, and a finding is "
            "placed at the changed section that causes it. The others are judged in full.\n"
            f"  - Judged against the changed sections: {', '.join(f'`{i}`' for i in ids)}.\n"
            f"  - Changed sections (`<path>` alone is the text before the file's first heading): {sections}.")


def run(argv):
    if len(argv) != 1:
        raise lib.Unreadable("usage: briefs.py FEATURE")
    m = lib.load_manifest(argv[0])
    base = {"FEATURE": m["feature"], "STATUS": str(m.get("status")), "REPO": os.getcwd(),
            "HEAD": m["head"][:8]}
    judge = template("judge-brief.md")
    prompts = []
    for c in m["cells"]:
        share = m["shares"][c["share"]]
        share_long = ""
        if share.get("long-lines"):
            share_long = (f"Lines of the share past {lib.LONG_LINE:,} characters, read with `sed -n '<n>p' <file> | "
                          f"fold -w 1000`: {', '.join(map(str, share['long-lines']))}.")
        text = fill(judge, {**base, "CELL": c["cell"], "SHARE": c["share"], "OUT": c["out"],
                            "SUBJECTS": subject_lines(m, c["subjects"]),
                            "LONG": long_note(m, c["subjects"]),
                            "SHARE_FILE": share["file"], "SHARE_BYTES": f"{share['bytes']:,}",
                            "SHARE_LONG": share_long,
                            "LOADED": ", ".join(f"`{n}`" for n in m["loaded"]) or "none",
                            "DEPARTURES": departures(m),
                            "REJUDGE": rejudged(m, share)})
        with open(c["brief"], "w", encoding="utf-8") as f:
            f.write(text)
        prompts.append((c["cell"], c["brief"], c["out"]))
    s = m["step15"]
    subj = subject_lines(m, s["subjects"])
    parts, unreadable = s.get("system-parts") or [], s.get("system-parts-unreadable") or []
    if s.get("system"):
        subj += f"\n- `{s['system']}` ({os.path.getsize(s['system']):,} bytes)"
    elif not parts and not unreadable:
        subj += "\n- The project has no `specs/system.md`, so the plan has nothing to contradict there."
    else:
        subj += "\n- The project has no `specs/system.md`, but its parts below are still the file's."
    for p in parts:
        subj += f"\n- `{p}` ({os.path.getsize(p):,} bytes), a part of `specs/system.md`"
    for p in unreadable:
        subj += (f"\n- `{p}`, a part of `specs/system.md` that `check-artifact-size` could not read: do not "
                 "judge the plan against it, and name it in your report as not examined, never as no contradiction")
    text = fill(template("step15-brief.md"), {**base, "OUT": s["out"], "SUBJECTS": subj,
                                              "LONG": long_note(m, s["subjects"])})
    with open(s["brief"], "w", encoding="utf-8") as f:
        f.write(text)
    readers = []
    for r in m["scenario-readers"]:
        text = fill(template("scenario-brief.md"), {**base, "READER": r["reader"], "OUT": r["out"],
                                                    "SUBJECTS": subject_lines(m, r["subjects"]),
                                                    "LONG": long_note(m, r["subjects"])})
        with open(r["brief"], "w", encoding="utf-8") as f:
            f.write(text)
        readers.append((r["reader"], r["brief"], r["out"]))
    print(f"== {m['feature']}: run the step-15 reader and the {len(readers)} scenario reader(s) first, then "
          f"the {len(prompts)} judges, each a fresh subagent with this prompt:")
    for cell, brief, out in [("step15", s["brief"], s["out"])] + readers + prompts:
        print(f"  {cell}: Read {brief} whole and follow it. Write your JSON to {out}. "
              "End your report with END OF REPORT.")


if __name__ == "__main__":
    lib.main(run)
