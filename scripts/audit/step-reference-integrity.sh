#!/usr/bin/env bash
# scripts/audit/step-reference-integrity.sh — Family 34 of /audit.
#
# A `step N` reference in a command file that names a numbered step the file
# does not have. Command files number their Instructions steps and refer to
# them in prose — "settled in step 4", "the confirmation in step 3 carries" —
# and the numbers are the only binding between a reference and its target.
#
# THE CHECK IS A PRIMITIVE; THIS IS THE ENTRY POINT — the Family 30 shape, and
# what the contract in README.md requires: the check is deterministic and
# parses markdown structure, so it is `check-step-references` in the runtime
# and this script resolves the binary, calls it, and renders through `emit`.
#
# WHAT IT CATCHES, AND WHAT IT DOES NOT. This is the important part, because
# the motivating incident is mostly *outside* its reach and the family must not
# be read as covering it.
#
# Spec 054 removed two steps from `specify.md` and one from `consolidate.md`,
# which renumbered everything after them and left four stale references. Of
# those four, this family would have caught exactly **one**:
#
#   - "create-feature in step 6" after it moved to 5 — NOT caught. Step 6
#     still exists (it is `writeSpecBody`), so the reference resolves; it just
#     resolves to the wrong step.
#   - "refused by the primitive in step 6" — NOT caught, same reason.
#   - "the confirmation in step 4 carries" after it moved to 3 — NOT caught.
#     Step 4 exists (`rewrite-spec-links`).
#   - "even though step 5 established the same fact", inside step 5 — CAUGHT.
#     It had been step 6 pointing at step 5 and became a self-reference, which
#     is why self-reference is a finding of its own rather than folded into
#     existence.
#
# Renumbering shifts references onto other *existing* steps, and deciding that
# "step 6" should now read "step 5" needs the semantics of what each step does.
# Asserting that would mean matching prose against step content — a heuristic,
# and a family that fires falsely on a correct reference is worse than the
# silence it replaces (the standard 045 applied when it rejected the
# criterion-supersession check at 455 pairs). So this family asserts only what
# is exact, and says so rather than implying the wider guarantee.
#
# Three findings, because the repairs differ: `unresolved` (names a number the
# file lacks), `self-reference` (resolves, so an existence check passes it),
# and `discontinuous` (ascends from 1 but skips numbers — the residue of a
# partial removal, which makes every later reference ambiguous rather than
# wrong).
#
# SUBJECT — the Instructions section of `framework/commands/*.md` plus the two
# bootstrap procedures. A numbered list outside Instructions restarts at 1 and
# is a different list, so its mentions are counted and not resolved (a
# command's reference is its own file under
# framework/reference/, outside the step subject, which the second subject below reads);
# a file whose Instructions holds several lists rather than one procedure
# (`amend.md` restarts under each subsection, `status.md` uses three one-item
# lists) is named in `not-a-procedure` rather than examined, because resolving
# against a merged set would invent findings. Both counts go to stderr so a
# clean exit is never read as "every step reference in the corpus resolves".
#
# Generated copies under a host's commands directory are not a subject: they
# carry whatever their source carries, and the repair is a ductus release.
#
# An unreachable runtime is a finding, not a silent pass. So is an empty
# examined set — a family that examined nothing reports clean. So is a
# rendering that cannot run: the findings are rendered by python, and a python
# that is missing, that crashes on output that is not JSON, or that meets a
# finding without the fields it reads would otherwise print nothing and leave
# the examined-set guard above unrun, which reads as clean. Its exit status is
# tested, and a non-zero one is a finding.
#
# SECOND SUBJECT — the `##` sections of each framework/reference/<command>.md
# that its command reaches (spec 026, from 071's scenario
# reference-sections-are-reached-by-a-step). An agent reads a section only where
# a step names it, so one no step reaches is never read: an `unreached-section`
# finding, unless it carries an `audit:kept` marker with a stated reason
# (`empty-exemption`, `stale-exemption` and `misplaced-exemption` keep the
# marker honest). A reference with no section is counted apart, not passed.
#
# Past `[artifacts] read-size-bytes` the primitive moves its six lists to the
# file its `lists-file` names, relative to the repository root where this
# family runs (026 task 6, BE-PAYLOAD-001). The render reads them there, and a
# file it cannot read, or one lacking a list, is a `(precondition)` finding
# saying nothing was checked, never an empty result.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family step-references

drift=0

if ! command -v python3 >/dev/null 2>&1; then
  emit "(precondition)" "python3 not on PATH — cannot render the check-step-references result" \
    "install python3 and re-run"
  exit 1
fi

bin="$(ductus_bin)"
if [ -z "$bin" ]; then
  emit "runtime" \
    "no ductus binary found (.ductus/bin/ductus, runtime/target/release/ductus, or PATH)" \
    "build the runtime (cargo build --release --manifest-path runtime/Cargo.toml) or install it"
  exit 1
fi

result="$("$bin" check-step-references 2>/dev/null)"
if [ -z "$result" ]; then
  emit "runtime" \
    "check-step-references produced no output" \
    "run '$bin check-step-references' directly to see the failure"
  exit 1
fi

# The render is captured, never streamed into the loop, so its exit status is
# observable: a substitution inside a heredoc discards it, and a python that
# died before printing anything would have rendered as zero findings.
render_findings() {
  printf '%s' "$result" | python3 -c '
import json, sys

d = json.load(sys.stdin)

# Past [artifacts] read-size-bytes the lists are in the file lists-file names
# (026 task 6): read them there, or say nothing was checked. A list the file
# lacks would read as none found, so it halts too.
if d.get("form") == "file":
    try:
        with open(d["lists-file"], encoding="utf-8") as handle:
            lists = json.load(handle)
        missing = [key for key in ("findings", "examined", "with-steps", "not-a-procedure", "skipped", "reachability")
                   if not isinstance(lists.get(key), list)]
        if missing:
            raise ValueError("it holds no {} list".format(", ".join(missing)))
        d.update(lists)
    except (KeyError, TypeError, OSError, ValueError) as err:
        print("\t".join(["(precondition)",
                         "check-step-references moved its lists to a file that could not be read ({}), so no step reference was checked".format(err),
                         "run check-step-references directly and read the file its lists-file names"]))
        raise SystemExit(0)

FIX = {
    "unresolved": "renumber the reference to the step it means, or restore the step it named",
    "self-reference": "a step referring to its own number is renumbering residue — point it at the step it meant",
    "discontinuous": "renumber the Instructions list so it runs 1..n without gaps",
    "unreached-section": "cite the section from the step that needs it, delete it if it only restates, or mark it with an audit:kept comment stating the reader outside the command that keeps it",
    "empty-exemption": "state the reason in the audit:kept comment, or remove it",
    "stale-exemption": "remove the audit:kept comment: the section is reached",
    "misplaced-exemption": "move the audit:kept comment to the line directly beneath its ## heading, or remove it",
}

for f in d.get("findings", []):
    loc = f["file"] if not f.get("line") else "{}:{}".format(f["file"], f["line"])
    print("\t".join([loc, f["message"], FIX.get(f["kind"], "correct the reference")]))

for s in d.get("skipped", []):
    print("\t".join([s["path"], "could not be examined: " + s["reason"],
                     "make the file readable, or remove it from the subject"]))

if not d.get("examined"):
    print("\t".join(["framework/commands", d.get("guidance") or "no command sources examined",
                     "run from the ductus repository root"]))

sys.stderr.write(
    "step-references: {} file(s) examined, {} carrying one procedure; "
    "{} not one procedure (not resolved: {}); "
    "{} reference(s) outside the Instructions section were counted, not resolved. "
    "A reference that renumbering shifted onto a DIFFERENT existing step is not "
    "detectable here and is not claimed. "
    "Reference sections: {} judged in {} reference(s), {} exempt, {} unreached, "
    "{} reference(s) with no section (not a pass: nothing to reach).\n".format(
        len(d.get("examined", [])),
        len(d.get("with-steps", [])),
        len(d.get("not-a-procedure", [])),
        ", ".join(d.get("not-a-procedure", [])) or "none",
        d.get("references-out-of-subject", 0),
        d.get("sections-examined", 0),
        len(d.get("reachability", [])),
        d.get("sections-exempt", 0),
        d.get("sections-unreached", 0),
        d.get("references-without-sections", 0),
    )
)
'
}

if ! rendered="$(render_findings)"; then
  emit "runtime" \
    "the check-step-references result could not be rendered (python3 exited non-zero) — nothing was judged, which is not the same as clean" \
    "run '$bin check-step-references' directly and confirm it prints the JSON this family reads (findings with file, message and kind; examined)"
  exit 1
fi

while IFS=$'\t' read -r location message fix; do
  [ -z "$location" ] && continue
  emit "$location" "$message" "$fix"
done <<EOF
$rendered
EOF

exit "$drift"
