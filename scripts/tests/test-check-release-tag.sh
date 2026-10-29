#!/usr/bin/env bash
# Test surface for scripts/check-release-tag.sh.
#
# The check runs only on a pushed tag, so a gate that cannot fail would first
# show it on a release already under way. It is proven here instead, on every
# push: it FAILS on each condition it claims to catch, and a subject it cannot
# compare never reads as one that passed.
#
# Coverage:
#   A. the committed `version` and the tag it names pass (control)
#   B. a tag naming another version fails, naming both
#   C. a tag without the `ductus-v` prefix fails
#   D. a missing `version` fails rather than passing
#   E. an empty `version` fails rather than passing
#   F. no ref fails rather than passing
#
# Usage: scripts/tests/test-check-release-tag.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
CHECK="$REPO_ROOT/scripts/check-release-tag.sh"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# A throwaway tree holding a copy of the check, which reads `version` relative
# to its own location, with `version` set to $2 (no file when $2 is `-`).
new_tree() {
  case_dir="$WORK/$1"
  mkdir -p "$case_dir/scripts"
  cp "$CHECK" "$case_dir/scripts/"
  if [ "$2" != "-" ]; then
    printf '%s' "$2" > "$case_dir/version"
  fi
}

# Run the case's check with the given arguments, capturing status and output.
run_check() {
  out="$(bash "$case_dir/scripts/check-release-tag.sh" "$@" 2>&1)" && code=0 || code=$?
}

echo "A. the committed version"
case_dir="$REPO_ROOT"
run_check "ductus-v$(tr -d '[:space:]' < "$REPO_ROOT/version")"
if [ "$code" -eq 0 ]; then pass "exit 0"; else fail "A: exit $code: $out"; fi

echo "B. a tag naming another version"
new_tree b $'0.60.0\n'
run_check ductus-v0.60.1
if [ "$code" -eq 1 ] && printf '%s' "$out" | grep -q "tag 'ductus-v0.60.1' does not name version 0.60.0"; then
  pass "exit 1 naming both"
else
  fail "B: exit $code: $out"
fi

echo "C. a tag without the prefix"
new_tree c '0.60.0'
run_check v0.60.0
if [ "$code" -eq 1 ]; then pass "exit 1"; else fail "C: exit $code: $out"; fi

echo "D. a missing version"
new_tree d -
run_check ductus-v0.60.0
if [ "$code" -eq 2 ] && printf '%s' "$out" | grep -q 'not found'; then
  pass "exit 2"
else
  fail "D: exit $code: $out"
fi

echo "E. an empty version"
new_tree e $' \n'
run_check ductus-v
if [ "$code" -eq 2 ] && printf '%s' "$out" | grep -q 'empty'; then
  pass "exit 2"
else
  fail "E: exit $code: $out"
fi

echo "F. no ref"
new_tree f '0.60.0'
run_check
if [ "$code" -eq 2 ]; then pass "exit 2"; else fail "F: exit $code: $out"; fi

if [ "$failures" -gt 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "all cases passed"
