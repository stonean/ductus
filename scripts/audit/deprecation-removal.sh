#!/usr/bin/env bash
# scripts/audit/deprecation-removal.sh — Family 40 of /audit.
#
# A rule ductus ships is retired in two steps (constitution §rules): a
# `**DEPRECATED in {version}:**` label names the release that deprecates it
# and its removal target, and the rule is removed in that target release.
# Nothing held a release to the second step. No task or check compared a
# label's target with `version`, so the removal depended on whoever cut the
# release remembering it, which §design-principles rules out. This family
# fails once `version` reaches a label's target while the rule is still in
# its file, so the release audit stops the release that reaches the target
# (spec 026, scenario family-40-deprecated-rules-are-removed-on-time; spec
# 077, task 8).
#
# Subject: every tracked framework/rules/*.md, not descending into a
# subdirectory, as the runtime's rule discovery does not. The labels are read
# by the runtime's `read-deprecation-labels`, which shares `check-rule-ids`'
# reader (spec 067, scenario deprecation-labels-are-read-by-the-runtime): this
# script keeps the comparison with `version` and the list of files citing a
# due rule, and renders the primitive's JSON through python. A label is read
# as the primitive's description states: its rule's section, a label opening
# its line as `**DEPRECATED in X.Y.Z:**`, and one target named once as
# `removed in X.Y.Z` in its paragraph (specs/008-security-rules/data-model.md,
# the Deprecated field):
#
#   40a  a malformed label: one outside a rule section, a second in one
#        section, one not opening its line or not in the form, a version that
#        is not MAJOR.MINOR.PATCH, or no target or more than one
#   40b  a target at or below `version`, naming every tracked file citing the ID
#
# No label at all is not a finding: nothing is deprecated, so there is nothing
# to compare. A missing, unreadable or malformed `version`, a git that cannot
# list the tracked files, no tracked rule file, a tracked rule file that cannot
# be read or is not UTF-8, and a read that cannot run (no runtime, no python3, a
# primitive that fails) are findings, since
# each leaves a comparison unmade. A file that cannot be read or decoded is not
# counted as examined, and the other files' labels are still compared. Every
# run ends with a coverage line on stderr.
#
# Bash 3.2 compatible (macOS system bash).

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family deprecation-removal

VERSION_FILE="version"
SEMVER_RE='^[0-9]+\.[0-9]+\.[0-9]+$'

coverage() {
  echo "deprecation-removal: $1" >&2
}

if [ ! -f "$VERSION_FILE" ]; then
  emit "$VERSION_FILE" "missing, so no deprecation label's removal target can be compared with it" \
    "restore $VERSION_FILE, one MAJOR.MINOR.PATCH line (Family 20)"
  coverage "nothing compared: $VERSION_FILE is missing"
  exit "$drift"
fi
if [ ! -r "$VERSION_FILE" ]; then
  emit "$VERSION_FILE" "not readable, so no deprecation label's removal target can be compared with it" \
    "make $VERSION_FILE readable"
  coverage "nothing compared: $VERSION_FILE is not readable"
  exit "$drift"
fi
current=$(head -n 1 "$VERSION_FILE" | tr -d '[:space:]')
if ! printf '%s' "$current" | grep -qE "$SEMVER_RE"; then
  emit "$VERSION_FILE" "first line '$current' is not MAJOR.MINOR.PATCH, so no removal target can be compared with it" \
    "write one MAJOR.MINOR.PATCH line (Family 20)"
  coverage "nothing compared: $VERSION_FILE is not MAJOR.MINOR.PATCH"
  exit "$drift"
fi

# A git that cannot run, or a directory that is not a work tree, lists
# nothing, which must not read as a tree with no rule file.
if ! git rev-parse --is-inside-work-tree > /dev/null 2>&1; then
  emit "framework/rules/" "git could not list the tracked rule files (git is not on PATH, or this is not a work tree), so the family examined nothing" \
    "run the family inside the repository, with git on PATH"
  coverage "nothing compared: git could not list the tracked rule files"
  exit "$drift"
fi

# NUL-separated, so a path git would quote is read as it is. The glob magic
# keeps `*` from matching a `/`.
tracked=0
readable=()
while IFS= read -r -d '' f; do
  tracked=$((tracked + 1))
  if [ -f "$f" ] && [ -r "$f" ]; then
    readable+=("$f")
  else
    emit "$f" "tracked but missing or unreadable, so its deprecation labels were not examined" \
      "restore $f, or remove it from the index"
  fi
done < <(git -c core.quotePath=false ls-files -z -- ':(glob)framework/rules/*.md' 2> /dev/null)

if [ "$tracked" -eq 0 ]; then
  emit "framework/rules/" "no tracked rule file, so the family examined nothing" \
    "restore framework/rules/*.md, or fix this family's enumeration"
  coverage "nothing compared: no tracked rule file"
  exit "$drift"
fi
if [ "${#readable[@]}" -eq 0 ]; then
  coverage "nothing compared: none of ${tracked} tracked rule file(s) could be read"
  exit "$drift"
fi

# render_labels JSON — one record per finding, fields joined by the ASCII
# unit separator, from the primitive's result:
#   U  file                                   a rule file that could not be read
#   M  location  message  fix                 a malformed label (40a)
#   L  file  line  rules  deprecated-in  target
#                                             a label naming a target, for the
#                                             comparison with version (40b)
#   C  found  compared                        the counts, last
# A function, not `$(python3 - <<EOF ...)`: bash 3.2 misparses a heredoc
# inside a command substitution. `-I` keeps the repository root off
# sys.path, so no file there shadows the standard library.
render_labels() {
  python3 -I - "$1" << 'PY'
import json
import sys

SEP = "\x1f"
FORM = "specs/008-security-rules/data-model.md, the Deprecated field"
result = json.loads(sys.argv[1])
for name in result["unreadable"]:
    print(SEP.join(("U", name)))
compared = 0
for label in result["labels"]:
    where = "%s:%d" % (label["file"], label["line"])
    for earlier in label["follows"]:
        print(SEP.join((
            "M", where,
            "a second deprecation label in %s's section, after the one at line %d" % (earlier["rule"], earlier["first-line"]),
            "keep one label, directly under the heading (%s)" % FORM)))
    if not label["rules"]:
        if not label["follows"]:
            print(SEP.join((
                "M", where, "a deprecation label outside any rule section, so it deprecates no rule",
                "move the label directly under its rule's heading (%s)" % FORM)))
        continue
    bad = label.get("malformed")
    if bad:
        print(SEP.join(("M", where, bad["reason"], bad["fix"])))
        continue
    compared += 1
    print(SEP.join(("L", label["file"], str(label["line"]), " ".join(label["rules"]),
                    label["deprecated-in"], label["removal-target"])))
print(SEP.join(("C", str(len(result["labels"])), str(compared))))
PY
}

# at_or_below A B — A is not above B, both MAJOR.MINOR.PATCH, compared
# numerically.
at_or_below() {
  local a1 a2 a3 b1 b2 b3
  IFS=. read -r a1 a2 a3 << EOF_A
$1
EOF_A
  IFS=. read -r b1 b2 b3 << EOF_B
$2
EOF_B
  [ "$a1" -ne "$b1" ] && { [ "$a1" -lt "$b1" ]; return; }
  [ "$a2" -ne "$b2" ] && { [ "$a2" -lt "$b2" ]; return; }
  [ "$a3" -le "$b3" ]
}

bin="$(ductus_bin)"
if [ -z "$bin" ]; then
  emit "scripts/audit/deprecation-removal.sh" "the ductus runtime is not reachable, so no deprecation label was read" \
    "run /ductus to acquire the runtime, or build it with cargo build --release in runtime/"
  coverage "nothing compared: the runtime is not reachable"
  exit "$drift"
fi
if ! command -v python3 > /dev/null 2>&1; then
  emit "scripts/audit/deprecation-removal.sh" "python3 is not on PATH, so no deprecation label was read" \
    "install python3, which this family renders the primitive's result with"
  coverage "nothing compared: python3 is missing"
  exit "$drift"
fi

rule_args=()
for f in "${readable[@]}"; do
  rule_args+=(--rule-file "$f")
done
if ! result=$("$bin" read-deprecation-labels "${rule_args[@]}" 2> /dev/null); then
  emit "scripts/audit/deprecation-removal.sh" "$bin read-deprecation-labels exited non-zero, so no deprecation label was compared" \
    "run $bin read-deprecation-labels directly and read its error"
  coverage "nothing compared: read-deprecation-labels failed"
  exit "$drift"
fi
if ! records=$(render_labels "$result"); then
  emit "scripts/audit/deprecation-removal.sh" "the label parse exited non-zero, so no deprecation label was compared" \
    "run the family directly and read the python error on stderr"
  coverage "nothing compared: the label parse failed"
  exit "$drift"
fi

sep=$(printf '\037')
found=""
compared=""
undecodable=0
while IFS="$sep" read -r kind a b c d e; do
  case "$kind" in
    U)
      emit "$a" "not UTF-8, so its deprecation labels were not examined" \
        "re-save $a as UTF-8, which the runtime reads"
      undecodable=$((undecodable + 1))
      ;;
    M) emit "$a" "$b" "$c" ;;
    L)
      # a: file  b: line  c: rules  d: deprecated-in  e: target
      if at_or_below "$e" "$current"; then
        for rule_id in $c; do
          cites=$(git -c core.quotePath=false grep -l -w -F -- "$rule_id" 2> /dev/null)
          cite_count=$(printf '%s\n' "$cites" | grep -c .)
          cite_list=$(printf '%s\n' "$cites" | grep . | tr '\n' ' ' | sed 's/ $//')
          emit "$a:$b" "$rule_id is deprecated in $d and removed in $e, and version is $current, but the rule is still in $a" \
            "remove $rule_id from $a, and re-point or classify each of the $cite_count tracked file(s) citing it ($cite_list); a decision record, a changelog entry or an audit record keeps the old ID"
        done
      fi
      ;;
    C)
      found="$a"
      compared="$b"
      ;;
  esac
done << EOF
$records
EOF

if [ -z "$found" ]; then
  emit "scripts/audit/deprecation-removal.sh" "the label parse printed no counts, so its result cannot be trusted" \
    "run the family directly and read its output"
  coverage "nothing compared: the label parse printed no counts"
  exit "$drift"
fi

examined=$((${#readable[@]} - undecodable))
if [ "$examined" -eq 0 ]; then
  coverage "nothing compared: none of ${tracked} tracked rule file(s) could be read"
elif [ "$found" -eq 0 ]; then
  coverage "${examined} of ${tracked} rule file(s) examined; no deprecation label, so nothing to compare with version ${current}"
else
  coverage "${examined} of ${tracked} rule file(s) examined; ${found} deprecation label(s) found, ${compared} compared with version ${current}"
fi

exit "$drift"
