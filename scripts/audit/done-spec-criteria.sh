#!/usr/bin/env bash
# scripts/audit/done-spec-criteria.sh — Family 27 of /audit.
#
# A spec at `status: done` that owes work: an unchecked box in its `tasks.md`,
# under any heading, or an unticked acceptance criterion in its `spec.md`. A
# `done` spec is `in-progress` exactly while it holds either (spec 084), and the
# command whose write leaves it owing work reopens it, so reaching this state
# means a hand edit bypassed that or a review's marking step failed.
#
# It happened here. 026 reached `done` in e9262df with AC19 unchecked, and
# every signal stayed green for the whole interval: run-all.sh exited 0,
# `check-artifacts` reported the feature `clean: true`, and CI passed. It was
# found by a hand-written grep while sweeping for remaining work — the same way
# Family 24's residue was found, and the same reason that family exists.
#
# METHOD. The family parses nothing: `check-owed-work --all` reads every `done`
# spec by the runtime's one owed-work test, the one the reopen decision, the
# completion gate and `check-artifacts`' review-state drift read (spec 084), so
# this family cannot drift from them. It reports each `done` spec's unchecked
# boxes and unticked criteria, and each spec the primitive could not judge,
# with its reason; a spec git does not track is skipped and counted, matching
# derive-dependencies' `untracked-skipped`.
#
# The examined count goes to stderr so a clean exit reads as "examined N done
# specs, none owing work" and never as "nothing to examine", and an empty
# enumeration is a finding rather than a pass — the fail-closed direction
# Families 17, 18, and 23 already take.
#
# Requires `python3` and the runtime (`ductus_bin`).

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family done-spec-criteria

if ! command -v python3 >/dev/null 2>&1; then
  emit "(precondition)" "python3 not on PATH — cannot read check-owed-work's result" \
    "install python3 and re-run"
  exit 1
fi

bin="$(ductus_bin)"
if [ -z "$bin" ]; then
  emit "(precondition)" "no ductus runtime reachable — check-owed-work did not run, which is not the same as clean" \
    "build runtime/ (cargo build --release) or run /ductus, then re-run"
  exit 1
fi

# The spec root is adopter-configurable (spec 040); resolve it the way the
# shipped shell does rather than assuming `specs`.
SPECS_ROOT="$(specs_root)"

# The root is committed text. One resolving outside the repository is
# reported and never listed or read (026 plan, BE-INPUT-004).
contained "$ROOT" "$SPECS_ROOT"
case $? in
  1)
    emit "$SPECS_ROOT/" "the configured spec root resolves outside the repository, so no spec was listed or read" \
      "set [paths] specs-root in .ductus/config.toml to a directory inside the repository"
    exit "$drift"
    ;;
  2)
    containment_unchecked "$SPECS_ROOT/"
    exit "$drift"
    ;;
esac

if ! result="$("$bin" check-owed-work --all 2> /dev/null)"; then
  emit "$SPECS_ROOT/" "check-owed-work --all failed — no spec was judged, which is not the same as clean" \
    "run $bin check-owed-work --all directly and read the error"
  exit "$drift"
fi

report="$(
  printf '%s' "$result" | python3 -c '
import json, sys

result = json.load(sys.stdin)
lists = result
if result.get("form") == "file":
    lists = json.load(open(result["lists-file"], encoding="utf-8"))
print("EXAMINED %d" % result["examined-count"])
untracked = 0
for skip in lists.get("skipped", []):
    if skip["reason"] == "untracked":
        untracked += 1
    else:
        print("SKIP\t%s\t%s" % (skip["feature"], skip["reason"]))
print("UNTRACKED %d" % untracked)
for spec in lists.get("specs", []):
    for box in spec["unchecked-boxes"]:
        print("BOX\t%s\t%s" % (spec["feature"], box["line"]))
    for criterion in spec["unticked-criteria"]:
        print("CRITERION\t%s\t%s" % (spec["feature"], criterion.get("label") or "(unlabelled)"))
'
)"

if [ -z "$report" ]; then
  emit "$SPECS_ROOT/" "check-owed-work's result could not be read — the family did not run, which is not the same as clean" \
    "run scripts/audit/done-spec-criteria.sh directly and read the error"
  exit "$drift"
fi

examined="$(printf '%s\n' "$report" | sed -n 's/^EXAMINED //p')"
untracked="$(printf '%s\n' "$report" | sed -n 's/^UNTRACKED //p')"
: "${examined:=0}" "${untracked:=0}"

while IFS="$(printf '\t')" read -r kind feature detail; do
  case "$kind" in
    BOX)
      emit "$SPECS_ROOT/$feature/tasks.md" "spec is status: done but tasks.md holds an unchecked box: $detail" \
        "tick it if the work is done, or reopen the spec to in-progress if it is not"
      ;;
    CRITERION)
      emit "$SPECS_ROOT/$feature/spec.md" "spec is status: done but criterion $detail is unchecked" \
        "tick $detail if the work is done, or reopen the spec to in-progress if it is not"
      ;;
    SKIP)
      case "$detail" in
        outside-project) why="holds a link leaving the project, so nothing under it was read" ;;
        *) why="could not be judged ($detail), which is not the same as owing nothing" ;;
      esac
      emit "$SPECS_ROOT/$feature/" "$why" "fix the spec directory so check-owed-work can read it"
      ;;
  esac
done <<EOF
$(printf '%s\n' "$report" | grep -E '^(BOX|CRITERION|SKIP)	' || true)
EOF

if [ "$examined" -eq 0 ] && [ "$drift" -eq 0 ]; then
  emit "$SPECS_ROOT/" "no specs at status: done were found among the tracked specs — the enumeration is broken, not the corpus" \
    "check the spec root, that specs are committed, and that their frontmatter carries a status: key"
fi

# Scope, on stderr: a clean exit means "these N done specs owe no work", never
# "all specs are complete".
echo "done-spec-criteria: examined $examined spec(s) at status: done under $SPECS_ROOT/ (untracked skipped: $untracked)" >&2

exit "$drift"
