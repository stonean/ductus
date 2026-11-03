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
# subdirectory, as the runtime's rule discovery does not. A rule is deprecated
# as the runtime's check-rule-ids decides it (`section_is_deprecated`,
# runtime/src/primitives/check_rule_ids.rs): its section runs from its
# `##`-`####` ID heading to the next heading at its level or above, so a
# deeper heading, another rule's included, leaves it open, and the first line
# in it containing `**DEPRECATED` is its label. The parse splits lines,
# trims whitespace and reads heading levels as the runtime does, and fenced
# blocks are not special-cased, since that parse does not special-case them.
# Each label must open its line as `**DEPRECATED in X.Y.Z:**`, and its
# paragraph, which ends at a blank line, a heading, a block quote, a
# `**Field:**` line, a list item or a fence, must name the target once as
# `removed in X.Y.Z`, in any case (specs/008-security-rules/data-model.md,
# the Deprecated field). Only a `removed in` followed by something that
# starts like a version is a target, so "removed in favour of" is prose:
#
#   40a  a malformed label: one outside a rule section, a second in one
#        section, one not opening its line or not in the form, a version that
#        is not MAJOR.MINOR.PATCH, or no target or more than one
#   40b  a target at or below `version`, naming every tracked file citing the ID
#
# This re-reads what the runtime already parses. A primitive returning each
# label will replace the parse below, and this script will become its entry
# point (spec 067, scenario deprecation-labels-are-read-by-the-runtime).
#
# No label at all is not a finding: nothing is deprecated, so there is nothing
# to compare. A missing, unreadable or malformed `version`, a git that cannot
# list the tracked files, no tracked rule file, a tracked rule file that cannot
# be read or is not UTF-8, and a parse that cannot run are findings, since
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

# parse_labels VERSION FILE... — one record per line, fields joined by the
# ASCII unit separator:
#   U  file                                   a rule file that is not UTF-8
#   M  location  message  fix                 a malformed label (40a)
#   D  file  line  id  deprecated-in  target  a target at or below VERSION (40b)
#   C  found  compared                        the counts, last
# A function, not `$(python3 - <<EOF ...)`: bash 3.2 misparses a heredoc
# inside a command substitution. `-I` keeps the repository root off
# sys.path, so no file there shadows the standard library.
parse_labels() {
  python3 -I - "$@" << 'PY'
import re
import sys

current = tuple(int(p) for p in sys.argv[1].split("."))
SEP = "\x1f"
# Rust's char::is_whitespace, which str.isspace() and Python's \s do not
# match: both also take U+001C to U+001F.
WS = ("\t\n\x0b\x0c\r \x85\xa0       "
      "         　")
# The runtime's heading_id_regex, run over the whole file as it is: a match
# opens at a line start, and its whitespace may run onto the next line.
RULE = re.compile(r"^#{2,4}[%s]+([A-Z]{2,5}-[A-Z][A-Z0-9]+-\d{3,4})\b" % re.escape(WS), re.MULTILINE)
LABEL = re.compile(r"\*\*DEPRECATED in ([^:*]*):\*\*")
FIELD = re.compile(r"\*\*[^*]+:\*\*")
# A line that opens a list item or a fence, either of which ends a paragraph.
BLOCK = re.compile(r"(?:[-*+]|1[.)])(?:[ \t]|$)|```|~~~")
TARGET = re.compile(r"\bremoved\s+in\s+`?(v?[0-9][0-9A-Za-z.+-]*)", re.IGNORECASE)
SEMVER = re.compile(r"[0-9]+\.[0-9]+\.[0-9]+")
FORM = "specs/008-security-rules/data-model.md, the Deprecated field"
found = 0
compared = 0


def rust_lines(text):
    # str::lines: split at each \n, dropping one \r before it, with no empty
    # last line. A lone \r, a form feed or U+2028 does not split a line.
    parts = text.split("\n")
    last = parts.pop()
    lines = [p[:-1] if p.endswith("\r") else p for p in parts]
    if last:
        lines.append(last)
    return lines


def atx(line):
    # The runtime's parse_atx_heading: leading whitespace trimmed, one to six
    # hashes, then a space or the end of the line.
    text = line.lstrip(WS)
    level = 0
    while level < len(text) and level < 6 and text[level] == "#":
        level += 1
    if level == 0:
        return None
    after = text[level:]
    if after and not after.startswith(" "):
        return None
    return level


def version(text):
    return tuple(int(p) for p in text.split("."))


def possessive(rules):
    return " and ".join(rules) + "'s"


for name in sys.argv[2:]:
    # A file the runtime's read_text cannot decode is one it cannot read:
    # reported and not examined, and the other files are still read.
    try:
        with open(name, encoding="utf-8", newline="") as handle:
            text = handle.read()
    except UnicodeDecodeError:
        print(SEP.join(("U", name)))
        continue
    lines = rust_lines(text)
    records = []

    def report(index, *fields):
        records.append((index, fields))

    def where(index):
        return "%s:%d" % (name, index + 1)

    headings = []
    offset = 0
    index = 0
    for match in RULE.finditer(text):
        index += text.count("\n", offset, match.start())
        offset = match.start()
        headings.append((index, match.group(1)))

    marked = [i for i, line in enumerate(lines) if "**DEPRECATED" in line]
    found += len(marked)
    covered = set()
    owners = {}
    for start, rule in headings:
        # section_is_deprecated: a heading the runtime cannot read the level
        # of (a tab after the hashes) is taken as level 4.
        level = atx(lines[start]) or 4
        in_section = []
        for i in range(start + 1, len(lines)):
            hashes = atx(lines[i])
            if hashes is not None and hashes <= level:
                break
            if "**DEPRECATED" in lines[i]:
                in_section.append(i)
        covered.update(in_section)
        if not in_section:
            continue
        owners.setdefault(in_section[0], []).append(rule)
        for extra in in_section[1:]:
            report(extra, "M", where(extra),
                   "a second deprecation label in %s's section, after the one at line %d" % (rule, in_section[0] + 1),
                   "keep one label, directly under the heading (%s)" % FORM)

    # A rule's own heading line is not in its section, so a label there is
    # outside it, as check-rule-ids reads it.
    for i in marked:
        if i in covered:
            continue
        report(i, "M", where(i), "a deprecation label outside any rule section, so it deprecates no rule",
               "move the label directly under its rule's heading (%s)" % FORM)

    for i, rules in owners.items():
        who = possessive(rules)
        line = lines[i]
        opening = LABEL.match(line)
        if not opening:
            if line.startswith("**DEPRECATED"):
                report(i, "M", where(i), "%s deprecation label is not in the form **DEPRECATED in X.Y.Z:**" % who,
                       "write the label as **DEPRECATED in X.Y.Z:**, naming the deprecating release (%s)" % FORM)
            else:
                report(i, "M", where(i), "%s deprecation label does not open its line" % who,
                       "start the line with the label, outside any blockquote or indent (%s)" % FORM)
            continue
        deprecated_in = opening.group(1).strip("`")
        if not SEMVER.fullmatch(deprecated_in):
            report(i, "M", where(i), "%s label reads DEPRECATED in %s, which is not MAJOR.MINOR.PATCH" % (who, deprecated_in),
                   "name the deprecating release as MAJOR.MINOR.PATCH (%s)" % FORM)
            continue
        paragraph = [line]
        for following in lines[i + 1:]:
            trimmed = following.strip(WS)
            if (not trimmed or atx(following) is not None
                    or trimmed.startswith(">") or FIELD.match(trimmed)
                    or BLOCK.match(trimmed)):
                break
            paragraph.append(following)
        targets = [t.rstrip(".") for t in TARGET.findall(" ".join(paragraph))]
        if not targets:
            report(i, "M", where(i), "%s label names no removal target, so nothing holds a release to removing it" % who,
                   "add removed in X.Y.Z to the label (%s)" % FORM)
            continue
        if len(set(targets)) == 1 and len(targets) > 1:
            report(i, "M", where(i), "%s label names its removal target %s more than once" % (who, targets[0]),
                   "name the target once, as removed in X.Y.Z (%s)" % FORM)
            continue
        if len(targets) > 1:
            report(i, "M", where(i), "%s label names more than one removal target (%s)" % (who, ", ".join(targets)),
                   "name the target once, as removed in X.Y.Z (%s)" % FORM)
            continue
        if not SEMVER.fullmatch(targets[0]):
            report(i, "M", where(i), "%s removal target %s is not MAJOR.MINOR.PATCH" % (who, targets[0]),
                   "name the target as removed in X.Y.Z (%s)" % FORM)
            continue
        compared += 1
        if version(targets[0]) <= current:
            for rule in rules:
                report(i, "D", name, i + 1, rule, deprecated_in, targets[0])

    for _, fields in sorted(records, key=lambda record: record[0]):
        print(SEP.join(str(f) for f in fields))

print(SEP.join(("C", str(found), str(compared))))
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
undecodable=0
while IFS="$sep" read -r kind a b c d e; do
  case "$kind" in
    U)
      emit "$a" "not UTF-8, so its deprecation labels were not examined" \
        "re-save $a as UTF-8, which the runtime's check-rule-ids reads"
      undecodable=$((undecodable + 1))
      ;;
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

examined=$((${#readable[@]} - undecodable))
if [ "$examined" -eq 0 ]; then
  coverage "nothing compared: none of ${tracked} tracked rule file(s) could be read"
elif [ "$found" -eq 0 ]; then
  coverage "${examined} of ${tracked} rule file(s) examined; no deprecation label, so nothing to compare with version ${current}"
else
  coverage "${examined} of ${tracked} rule file(s) examined; ${found} deprecation label(s) found, ${compared} compared with version ${current}"
fi

exit "$drift"
