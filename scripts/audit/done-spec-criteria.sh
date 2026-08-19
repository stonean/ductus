#!/usr/bin/env bash
# scripts/audit/done-spec-criteria.sh — Family 27 of /audit.
#
# A spec at `status: done` whose Acceptance Criteria still carry an unchecked
# box. The completion gate is supposed to make this unreachable —
# framework/commands/implement.md marks each verified criterion and refuses to
# propose the transition while any remains unchecked — so reaching `done` with
# one unticked means the gate was bypassed or its marking step failed.
#
# It happened here. 026 reached `done` in e9262df with AC19 unchecked, and
# every signal stayed green for the whole interval: run-all.sh exited 0,
# `check-artifacts` reported the feature `clean: true`, and CI passed. It was
# found by a hand-written grep while sweeping for remaining work — the same way
# Family 24's residue was found, and the same reason that family exists.
#
# The direction of the failure is what makes it worth a check. An unchecked
# criterion under a `done` spec reads as work that was never finished, so
# either the spec is lying about being complete or the criterion is lying about
# being unmet. Both are cheap to repair and neither announces itself.
# `check-artifacts` deliberately does not cover it: its criterion-path-existence
# family asks whether a *checked* criterion's paths exist, the opposite
# direction.
#
# METHOD. Exact rather than heuristic — the subject is a checkbox in a known
# section of a file with known frontmatter, so nothing is inferred:
#
#   27a Enumerate tracked spec.md files under the spec root via `git ls-files`.
#       Untracked specs are skipped and counted, matching derive-dependencies'
#       `untracked-skipped`; silently ignoring one would let a spec escape the
#       check by never having been committed.
#   27b Read `status:` from the frontmatter block only. The token appears in
#       prose across the corpus (026's own scenarios contain it), so a repo-wide
#       grep would match documents *describing* the state.
#   27c Within a `done` spec, scan only the Acceptance Criteria section, skip
#       fenced blocks, and report each `- [ ]` with its file:line and label.
#
# Only `done` specs are examined. A spec at draft / clarified / planned /
# in-progress is *expected* to carry unchecked criteria — that is what those
# states mean — and flagging them would make this fire on every spec in flight
# and be ignored within a day.
#
# NOT a finding: a spec with no Acceptance Criteria section at all. Section
# completeness is /ductus:analyze's artifact-tier concern, and duplicating it
# would cross the boundary audit.md §Notes draws between the two commands.
#
# The examined count goes to stderr so a clean exit reads as "examined N done
# specs, all criteria checked" and never as "nothing to examine", and an empty
# enumeration is a finding rather than a pass — the fail-closed direction
# Families 17, 18, and 23 already take.
#
# Requires `python3`.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family done-spec-criteria

if ! command -v python3 >/dev/null 2>&1; then
  emit "(precondition)" "python3 not on PATH — cannot parse spec frontmatter" \
    "install python3 and re-run"
  exit 1
fi

# The spec root is adopter-configurable (spec 040); resolve it the way the
# shipped shell does rather than assuming `specs`.
SPECS_ROOT="specs"
if [ -f .ductus/config.toml ]; then
  configured="$(
    sed -n 's/^[[:space:]]*specs-root[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' \
      .ductus/config.toml | head -1
  )"
  [ -n "$configured" ] && SPECS_ROOT="$configured"
fi

files="$(git ls-files -- "$SPECS_ROOT/*/spec.md" 2>/dev/null)"
if [ -z "$files" ]; then
  emit "$SPECS_ROOT/" "no tracked spec.md files found — the enumeration is broken, not the corpus" \
    "check the spec root and that specs are committed"
  exit "$drift"
fi

# Untracked specs are invisible to `git ls-files`. Count them so the scope line
# states what went unexamined rather than implying full coverage.
untracked="$(git ls-files --others --exclude-standard -- "$SPECS_ROOT/*/spec.md" 2>/dev/null | grep -c . || true)"

report="$(
  printf '%s\n' "$files" | python3 -c '
import sys, re

paths = [p for p in sys.stdin.read().splitlines() if p]
examined = 0
findings = []

for path in paths:
    try:
        with open(path, encoding="utf-8") as fh:
            lines = fh.read().splitlines()
    except OSError as exc:
        findings.append((path, 0, "", "unreadable: %s" % exc))
        continue

    # 27b — status from the frontmatter block only, never a loose grep.
    if not lines or lines[0].strip() != "---":
        continue
    status = None
    for i in range(1, len(lines)):
        if lines[i].strip() == "---":
            break
        m = re.match(r"^status:\s*(\S+)\s*$", lines[i])
        if m:
            status = m.group(1)
    if status != "done":
        continue
    examined += 1

    # 27c — Acceptance Criteria section only, fences skipped.
    in_section = False
    in_fence = False
    for idx, line in enumerate(lines, start=1):
        if re.match(r"^```", line):
            in_fence = not in_fence
            continue
        if in_fence:
            continue
        if re.match(r"^##\s+", line):
            in_section = bool(re.match(r"^##\s+Acceptance Criteria\s*$", line))
            continue
        if not in_section:
            continue
        if re.match(r"^\s*-\s+\[ \]", line):
            label = ""
            m = re.search(r"\[ \]\s*(?:\*\*)?(AC\d+)", line)
            if m:
                label = m.group(1)
            findings.append((path, idx, label, ""))

print("EXAMINED %d" % examined)
for path, line, label, err in findings:
    if err:
        print("ERROR\t%s\t%d\t%s" % (path, line, err))
    else:
        print("FINDING\t%s\t%d\t%s" % (path, line, label or "(unlabelled)"))
'
)"

if [ -z "$report" ]; then
  emit "$SPECS_ROOT/" "criterion scan produced no output — it did not run, which is not the same as clean" \
    "run scripts/audit/done-spec-criteria.sh directly and read the error"
  exit "$drift"
fi

examined="$(printf '%s\n' "$report" | sed -n 's/^EXAMINED //p')"
: "${examined:=0}"

if [ "$examined" -eq 0 ]; then
  emit "$SPECS_ROOT/" "no specs at status: done were found among the tracked specs — the status parse is broken, not the corpus" \
    "confirm spec frontmatter carries a status: key and re-run"
  exit "$drift"
fi

while IFS="$(printf '\t')" read -r kind path line detail; do
  case "$kind" in
    FINDING)
      emit "$path:$line" "spec is status: done but criterion $detail is unchecked" \
        "tick $detail if the work is done, or reopen the spec to in-progress if it is not"
      ;;
    ERROR)
      emit "$path:$line" "$detail" "fix the file so the criterion scan can read it"
      ;;
  esac
done <<EOF
$(printf '%s\n' "$report" | grep -E '^(FINDING|ERROR)	' || true)
EOF

# Scope, on stderr: a clean exit means "these N done specs have every criterion
# checked", never "all specs are complete".
echo "done-spec-criteria: examined $examined spec(s) at status: done under $SPECS_ROOT/ (untracked skipped: $untracked)" >&2

exit "$drift"
