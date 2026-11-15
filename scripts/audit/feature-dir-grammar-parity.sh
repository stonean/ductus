#!/usr/bin/env bash
# scripts/audit/feature-dir-grammar-parity.sh — Family 41 of /audit.
#
# The feature-directory grammar the two dogfooded copies carry agrees with the
# one the shipped hook carries (spec 051 AC15; spec 067, task 34).
#
# The runtime's `parse_feature_dir` is the grammar's home, and three shell
# surfaces keep a copy of its alternation because each runs before any binary
# resolves or cannot ask a primitive for its corpus:
#
#   - `framework/bootstrap/hooks/ductus-pre-commit`, the hook adopters get
#     (the reference: Family 22 drives it against both directory forms)
#   - `.githooks/pre-commit`, this repository's own hook
#   - `scripts/lint-frontmatter.sh`, `FEATURE_DIR_RE`
#
# Family 22 reaches the shipped hook only. Each dogfooded copy said it was
# "held by a reader", which is the diligence §design-principles rules out, so
# this family compares the two to the shipped one. The comparison is of the
# grammar's text, the alternation between the leading `^[^/]+/` (or `^`) and
# the trailing `-`, extracted from each file with no regex engine involved. A
# copy that drifts to an equivalent spelling is a finding too: the three are
# kept identical on purpose, so a reader can diff them.
#
# Each copy must hold exactly one grammar. A file that holds none, or two, a
# grammar that extracts empty, and a file that cannot be read are findings,
# never a pass: a parse that found nothing compares equal to nothing. Every
# run ends with a coverage line on stderr naming how many copies it compared.
#
# Bash 3.2 compatible (macOS system bash).

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family feature-dir-grammar

SHIPPED="framework/bootstrap/hooks/ductus-pre-commit"
HOOK_COPY=".githooks/pre-commit"
LINT_COPY="scripts/lint-frontmatter.sh"

# A hook's grep: `grep -E "^[^/]+/<GRAMMAR>-[^/]+/(spec|spec-and-plan)\.md\$"`.
HOOK_HEAD='grep -E "^[^/]+/'
HOOK_TAIL='-[^/]+/(spec|spec-and-plan)\.md\$"'
# The lint's: `FEATURE_DIR_RE='^<GRAMMAR>-'`.
LINT_HEAD="FEATURE_DIR_RE='^"
LINT_TAIL="-'"

compared=0
unread=0
reference=""

# extract FILE HEAD TAIL — set grammar to the text between HEAD and TAIL on
# the one line of FILE holding HEAD, and report a file that cannot yield
# exactly one non-empty grammar. Returns 1 when none was extracted.
extract() {
  local file="$1" head="$2" tail="$3" lines count line
  grammar=""
  if [ ! -r "$file" ]; then
    emit "$file" "cannot be read, so its feature-directory grammar was not compared" "restore the file or retire this family's entry for it"
    unread=$((unread + 1))
    return 1
  fi
  lines="$(grep -F -- "$head" "$file")"
  count="$(printf '%s' "$lines" | grep -c . || true)"
  if [ "$count" -ne 1 ]; then
    emit "$file" "holds $count feature-directory grammars (expected exactly 1), so none was compared" "keep one copy of the alternation in the file, as $SHIPPED holds it"
    unread=$((unread + 1))
    return 1
  fi
  line="${lines#*"$head"}"
  case "$line" in
    *"$tail"*) line="${line%%"$tail"*}" ;;
    *) line="" ;;
  esac
  if [ -z "$line" ]; then
    emit "$file" "its feature-directory grammar extracts empty" "the line no longer matches the form this family reads; update the family with the file"
    unread=$((unread + 1))
    return 1
  fi
  grammar="$line"
}

if extract "$SHIPPED" "$HOOK_HEAD" "$HOOK_TAIL"; then
  reference="$grammar"
  compared=$((compared + 1))
fi

# compare FILE HEAD TAIL — compare FILE's grammar to the shipped one.
compare() {
  if ! extract "$1" "$2" "$3"; then
    return
  fi
  compared=$((compared + 1))
  if [ -n "$reference" ] && [ "$grammar" != "$reference" ]; then
    emit "$1" "feature-directory grammar differs from the shipped hook's ($SHIPPED): has \`$grammar\`, shipped has \`$reference\`" "make the two alternations identical, the shipped hook's being the reference"
  fi
}

compare "$HOOK_COPY" "$HOOK_HEAD" "$HOOK_TAIL"
compare "$LINT_COPY" "$LINT_HEAD" "$LINT_TAIL"

# Three copies is the set this family is written for. A count below it means a
# copy was not compared, and the findings above name which.
echo "feature-dir-grammar-parity: compared $compared of 3 copies ($unread unread)" >&2
if [ "$compared" -lt 3 ] && [ "$unread" -eq 0 ]; then
  emit "$0" "compared $compared of 3 copies with none reported unread" "the family's own accounting is broken"
fi

exit "$drift"
