#!/usr/bin/env bash
# Test surface for scripts/audit/feature-dir-grammar-parity.sh (Family 41).
#
# The family holds the two dogfooded copies of the feature-directory grammar
# (`.githooks/pre-commit`, `scripts/lint-frontmatter.sh`) to the shipped
# hook's (spec 067, task 34; spec 051 AC15). The evidence it works is that it
# FAILS on a diverged copy of each, and that a parse that finds no grammar never
# reads as a pass. Each fixture is a tree holding the family, its lib and the
# three files, so no real copy is ever diverged.
#
# Coverage:
#   A. the committed tree passes, its coverage line naming 3 of 3 copies
#   B. a diverged .githooks/pre-commit fails, naming that file
#   C. a diverged scripts/lint-frontmatter.sh fails, naming that file
#   D. a diverged shipped hook fails, naming each dogfooded copy, since both
#      now differ from the reference
#   E. a copy holding no grammar, or two, a grammar that extracts empty and a
#      missing file each fail rather than pass, and the coverage line counts
#      the copies compared
#   F. an identical copy edited elsewhere in the file still passes (control)
#
# Usage: scripts/tests/test-feature-dir-grammar-parity.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
FAMILY="scripts/audit/feature-dir-grammar-parity.sh"
LIB="scripts/audit/lib.sh"
SHIPPED="framework/bootstrap/hooks/ductus-pre-commit"
HOOK=".githooks/pre-commit"
LINT="scripts/lint-frontmatter.sh"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# make_tree NAME — a fixture holding the family, its lib and the three files.
make_tree() {
  local dir="$WORK/$1" f
  for f in "$FAMILY" "$LIB" "$SHIPPED" "$HOOK" "$LINT"; do
    mkdir -p "$dir/$(dirname "$f")"
    cp "$REPO_ROOT/$f" "$dir/$f"
  done
  echo "$dir"
}

# run_family DIR — run the fixture's copy; sets out, err and code.
run_family() {
  set +e
  out="$("$BASH" "$1/$FAMILY" 2> "$WORK/stderr")"
  code=$?
  set -e
  err="$(cat "$WORK/stderr")"
}

# diverge FILE — change the sequential alternation's four-digit arm in FILE.
diverge() {
  sed 's/\[1-9\]\[0-9\]\[0-9\]\[0-9\]+/[0-9][0-9][0-9][0-9]+/' "$1" > "$1.new"
  if cmp -s "$1" "$1.new"; then
    echo "diverge: $1 holds no arm to change" >&2
    exit 1
  fi
  mv "$1.new" "$1"
}

# expect NAME CODE [FINDING-SUBSTRING...] — check the last run.
expect() {
  local name="$1" want="$2" ok=1 needle
  shift 2
  [ "$code" -eq "$want" ] || ok=0
  for needle in "$@"; do
    case "$out$err" in *"$needle"*) ;; *) ok=0 ;; esac
  done
  if [ "$ok" -eq 1 ]; then pass "$name"; else
    fail "$name (exit $code)"
    printf '%s\n%s\n' "$out" "$err" >&2
  fi
}

# A. the committed tree
t="$(make_tree control)"
run_family "$t"
expect "A. the tree passes, 3 of 3 copies compared" 0 "compared 3 of 3 copies"

# A'. the real tree, not a copy
set +e
real_out="$("$BASH" "$REPO_ROOT/$FAMILY" 2> "$WORK/stderr")"
real_code=$?
set -e
if [ "$real_code" -eq 0 ] && grep -q "compared 3 of 3" "$WORK/stderr"; then
  pass "A. the real tree passes"
else
  fail "A. the real tree (exit $real_code): $real_out"
fi

# B. a diverged .githooks/pre-commit
t="$(make_tree b)"; diverge "$t/$HOOK"; run_family "$t"
expect "B. a diverged .githooks/pre-commit fails" 1 "$HOOK"

# C. a diverged lint
t="$(make_tree c)"; diverge "$t/$LINT"; run_family "$t"
expect "C. a diverged scripts/lint-frontmatter.sh fails" 1 "$LINT"

# D. a diverged shipped hook
t="$(make_tree d)"; diverge "$t/$SHIPPED"; run_family "$t"
expect "D. a diverged shipped hook fails against both copies" 1 "$HOOK" "$LINT"

# E1. no grammar
t="$(make_tree e1)"; grep -v 'grep -E "^\[^/\]+/' "$t/$HOOK" > "$t/new" && mv "$t/new" "$t/$HOOK"
run_family "$t"
expect "E. a copy holding no grammar fails" 1 "holds 0 feature-directory grammars" "compared 2 of 3"

# E2. two grammars
t="$(make_tree e2)"; grep 'grep -E "^\[^/\]+/' "$t/$HOOK" > "$t/dup"; cat "$t/dup" >> "$t/$HOOK"
run_family "$t"
expect "E. a copy holding two grammars fails" 1 "holds 2 feature-directory grammars" "compared 2 of 3"

# E3. a grammar that extracts empty
t="$(make_tree e3)"
sed "s/^FEATURE_DIR_RE=.*/FEATURE_DIR_RE='^-'/" "$t/$LINT" > "$t/new" && mv "$t/new" "$t/$LINT"
run_family "$t"
expect "E. a grammar that extracts empty fails" 1 "extracts empty" "compared 2 of 3"

# E4. a missing file, and the shipped one
t="$(make_tree e4)"; rm "$t/$LINT"; run_family "$t"
expect "E. a missing lint fails" 1 "$LINT" "compared 2 of 3"
t="$(make_tree e5)"; rm "$t/$SHIPPED"; run_family "$t"
expect "E. a missing shipped hook fails" 1 "$SHIPPED" "compared 2 of 3"

# F. an edit elsewhere in the file does not matter
t="$(make_tree f)"; printf '\n# a trailing comment\n' >> "$t/$HOOK"; run_family "$t"
expect "F. an unrelated edit still passes" 0 "compared 3 of 3 copies"

if [ "$failures" -ne 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "all cases passed"
