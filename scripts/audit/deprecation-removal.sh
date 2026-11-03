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
# Subject: every tracked framework/rules/*.md. A rule is deprecated as the
# runtime's check-rule-ids decides it (`section_is_deprecated`,
# runtime/src/primitives/check_rule_ids.rs): its section runs from its
# `##`-`####` ID heading to the next heading at its level or above, and any
# line in it containing `**DEPRECATED` is its label. Fenced blocks are not
# special-cased, since that parse does not special-case them. Each label must
# open its line as `**DEPRECATED in X.Y.Z:**`, and its paragraph must name the
# target once as `removed in X.Y.Z`, in any case
# (specs/008-security-rules/data-model.md, the Deprecated field):
#
#   40a  a malformed label: one outside a rule section, a second in one
#        section, one not opening its line, a version that is not
#        MAJOR.MINOR.PATCH, or no target or more than one
#   40b  a target at or below `version`, naming every tracked file citing the ID
#
# This re-reads what the runtime already parses. A primitive returning each
# label will replace the parse below, and this script will become its entry
# point (spec 067, scenario deprecation-labels-are-read-by-the-runtime).
#
# No label at all is not a finding: nothing is deprecated, so there is nothing
# to compare. An unreadable `version`, no tracked rule file, a tracked rule
# file that cannot be read, and a parse that cannot run are findings, since
# each leaves a comparison unmade. Every run ends with a coverage line on
# stderr.
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
current=$(head -n 1 "$VERSION_FILE" | tr -d '[:space:]')
if ! printf '%s' "$current" | grep -qE "$SEMVER_RE"; then
  emit "$VERSION_FILE" "first line '$current' is not MAJOR.MINOR.PATCH, so no removal target can be compared with it" \
    "write one MAJOR.MINOR.PATCH line (Family 20)"
  coverage "nothing compared: $VERSION_FILE is not MAJOR.MINOR.PATCH"
  exit "$drift"
fi

# NUL-separated, so a path git would quote is read as it is.
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
done < <(git -c core.quotePath=false ls-files -z -- 'framework/rules/*.md' 2> /dev/null)

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

# parse_labels VERSION FILE... — one record per line, fields joined by the
# ASCII unit separator:
#   M  location  message  fix                 a malformed label (40a)
#   D  file  line  id  deprecated-in  target  a target at or below VERSION (40b)
#   C  found  compared                        the counts, last
# A function, not `$(python3 - <<EOF ...)`: bash 3.2 misparses a heredoc
# inside a command substitution.
parse_labels() {
  python3 - "$@" << 'PY'
import re
import sys

current = tuple(int(p) for p in sys.argv[1].split("."))
SEP = "\x1f"
RULE = re.compile(r"#{2,4}\s+([A-Z]{2,5}-[A-Z][A-Z0-9]+-\d{3,4})\b")
LABEL = re.compile(r"\*\*DEPRECATED in ([^:*]*):\*\*")
TARGET = re.compile(r"removed in\s+`?([0-9A-Za-z][0-9A-Za-z.+-]*)", re.IGNORECASE)
SEMVER = re.compile(r"\d+\.\d+\.\d+")
FORM = "specs/008-security-rules/data-model.md, the Deprecated field"
found = 0
compared = 0


def atx(line):
    # The runtime's parse_atx_heading: leading whitespace trimmed, one to six
    # hashes, then a space or the end of the line.
    text = line.lstrip()
    level = 0
    while level < len(text) and level < 6 and text[level] == "#":
        level += 1
    if level == 0:
        return None
    after = text[level:]
    if after and not after.startswith(" "):
        return None
    return level


def out(*fields):
    print(SEP.join(str(f) for f in fields))


def version(text):
    return tuple(int(p) for p in text.split("."))


for name in sys.argv[2:]:
    with open(name, encoding="utf-8") as handle:
        lines = handle.read().splitlines()
    rule = None
    level = 0
    labels_in_rule = 0
    for index, line in enumerate(lines):
        where = "%s:%d" % (name, index + 1)
        heading = RULE.match(line)
        if heading:
            rule = heading.group(1)
            level = atx(line)
            labels_in_rule = 0
            continue
        hashes = atx(line)
        if hashes is not None:
            if rule is not None and hashes <= level:
                rule = None
            continue
        if "**DEPRECATED" not in line:
            continue
        found += 1
        if rule is None:
            out("M", where, "a deprecation label outside any rule section, so it names no rule to remove",
                "move the label directly under its rule's heading (%s)" % FORM)
            continue
        labels_in_rule += 1
        if labels_in_rule > 1:
            out("M", where, "a second deprecation label in %s's section" % rule,
                "keep one label under the heading (%s)" % FORM)
            continue
        opening = LABEL.match(line)
        if not opening:
            out("M", where, "%s's deprecation label does not open its line as **DEPRECATED in X.Y.Z:**" % rule,
                "start the line with the label, outside any blockquote or indent (%s)" % FORM)
            continue
        deprecated_in = opening.group(1).strip("`")
        if not SEMVER.fullmatch(deprecated_in):
            out("M", where, "%s's label reads DEPRECATED in %s, which is not MAJOR.MINOR.PATCH" % (rule, deprecated_in),
                "name the deprecating release as MAJOR.MINOR.PATCH")
            continue
        paragraph = [line]
        for following in lines[index + 1:]:
            if not following.strip() or atx(following) is not None:
                break
            paragraph.append(following)
        targets = [t.rstrip(".") for t in TARGET.findall(" ".join(paragraph))]
        if not targets:
            out("M", where, "%s is deprecated in %s, and its label names no removal target, so nothing holds a release to removing it" % (rule, deprecated_in),
                "add removed in X.Y.Z to the label (%s)" % FORM)
            continue
        if len(targets) > 1:
            out("M", where, "%s's label names more than one removal target (%s)" % (rule, ", ".join(targets)),
                "name the target once, as removed in X.Y.Z (%s)" % FORM)
            continue
        if not SEMVER.fullmatch(targets[0]):
            out("M", where, "%s's removal target %s is not MAJOR.MINOR.PATCH" % (rule, targets[0]),
                "name the target as removed in X.Y.Z (%s)" % FORM)
            continue
        compared += 1
        if version(targets[0]) <= current:
            out("D", name, index + 1, rule, deprecated_in, targets[0])

out("C", found, compared)
PY
}

if ! command -v python3 > /dev/null 2>&1; then
  emit "scripts/audit/deprecation-removal.sh" "python3 is not on PATH, so no deprecation label was read" \
    "install python3, which this family's parse needs"
  coverage "nothing compared: python3 is missing"
  exit "$drift"
fi

if ! records=$(parse_labels "$current" "${readable[@]}"); then
  emit "scripts/audit/deprecation-removal.sh" "the label parse exited non-zero, so no deprecation label was compared" \
    "run the family directly and read the python error on stderr"
  coverage "nothing compared: the label parse failed"
  exit "$drift"
fi

sep=$(printf '\037')
found=""
compared=""
while IFS="$sep" read -r kind a b c d e; do
  case "$kind" in
    M) emit "$a" "$b" "$c" ;;
    D)
      cites=$(git -c core.quotePath=false grep -l -w -F -- "$c" 2> /dev/null)
      cite_count=$(printf '%s\n' "$cites" | grep -c .)
      cite_list=$(printf '%s\n' "$cites" | grep . | tr '\n' ' ' | sed 's/ $//')
      emit "$a:$b" "$c is deprecated in $d and removed in $e, and version is $current, but the rule is still in $a" \
        "remove $c from $a, and re-point or classify each of the $cite_count tracked file(s) citing it ($cite_list); a decision record, a changelog entry or an audit record keeps the old ID"
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

if [ "$found" -eq 0 ]; then
  coverage "${#readable[@]} of ${tracked} rule file(s) examined; no deprecation label, so nothing to compare with version ${current}"
else
  coverage "${#readable[@]} of ${tracked} rule file(s) examined; ${found} deprecation label(s) found, ${compared} compared with version ${current}"
fi

exit "$drift"
