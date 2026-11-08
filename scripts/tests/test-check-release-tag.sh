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
#   G. an entry under `## [Unreleased]` fails, counting it
#   H. an `## [Unreleased]` section holding only prose passes
#   I. a missing runtime/CHANGELOG.md fails rather than passing
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

# A throwaway tree holding a copy of the check, which reads `version` and
# runtime/CHANGELOG.md relative to its own location, with `version` set to $2
# (no file when $2 is `-`) and the changelog to $3 (a released one when absent,
# no file when `-`).
RELEASED=$'# Changelog\n\n## [0.60.0] — 2026-11-08\n\n### Added\n\n- an entry\n'
new_tree() {
  case_dir="$WORK/$1"
  mkdir -p "$case_dir/scripts" "$case_dir/runtime"
  cp "$CHECK" "$case_dir/scripts/"
  if [ "$2" != "-" ]; then
    printf '%s' "$2" > "$case_dir/version"
  fi
  if [ "${3:-}" != "-" ]; then
    printf '%s' "${3:-$RELEASED}" > "$case_dir/runtime/CHANGELOG.md"
  fi
}

# Run the case's check with the given arguments, capturing status and output.
run_check() {
  out="$(bash "$case_dir/scripts/check-release-tag.sh" "$@" 2>&1)" && code=0 || code=$?
}

echo "A. the committed version, with a released changelog"
committed="$(tr -d '[:space:]' < "$REPO_ROOT/version")"
new_tree a "$committed"
run_check "ductus-v$committed"
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

echo "G. an entry under [Unreleased]"
new_tree g '0.60.0' $'# Changelog\n\n## [Unreleased]\n\nProse.\n\n### Added\n\n- a pending entry\n\n## [0.59.11] — 2026-10-17\n\n- shipped\n'
run_check ductus-v0.60.0
if [ "$code" -eq 1 ] && printf '%s' "$out" | grep -q "2 entry line(s) still sit under '## \[Unreleased\]'"; then
  pass "exit 1 counting the entry and its heading"
else
  fail "G: exit $code: $out"
fi

echo "H. an [Unreleased] section holding only prose"
new_tree h '0.60.0' $'# Changelog\n\n## [Unreleased]\n\nNothing yet.\n\n## [0.60.0] — 2026-11-08\n\n### Added\n\n- shipped\n'
run_check ductus-v0.60.0
if [ "$code" -eq 0 ]; then pass "exit 0"; else fail "H: exit $code: $out"; fi

echo "I. a missing changelog"
new_tree i '0.60.0' -
run_check ductus-v0.60.0
if [ "$code" -eq 2 ] && printf '%s' "$out" | grep -q 'CHANGELOG.md: not found'; then
  pass "exit 2"
else
  fail "I: exit $code: $out"
fi

if [ "$failures" -gt 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "all cases passed"
