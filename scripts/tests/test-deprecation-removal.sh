#!/usr/bin/env bash
# Test surface for scripts/audit/deprecation-removal.sh (Family 40).
#
# The family is the release gate that holds a deprecated rule to its removal
# target, so the evidence it works is that it FAILS on each condition it
# claims to catch, and that a comparison it cannot make never reads as one
# that passed (spec 026, scenario family-40-deprecated-rules-are-removed-on-time).
#
# Coverage:
#   A. the committed tree passes, with a coverage line in either form (control)
#   B. a target above version passes, at the minor and at the patch number
#   C. a target equal to version fails, naming the rule and its citing files
#   D. a target below version fails, compared numerically (0.9.0 < 0.10.0)
#   E. a label wrapped onto a second line is read whole
#   F. the target is matched in any case
#   G. a label naming no target, two targets, or a target that is not
#      MAJOR.MINOR.PATCH fails
#   H. a deprecating version that is not MAJOR.MINOR.PATCH fails
#   I. a label outside any rule section fails, including one under a ## heading
#      that follows a rule
#   J. a label that does not open its line (a blockquote) fails
#   K. two labels in two files are both read and counted
#   L. no label at all passes, and says there was nothing to compare
#   M. a missing or malformed version fails rather than passing vacuously
#   N. no tracked rule file, or a tracked file that cannot be read, fails
#
# Usage: scripts/tests/test-deprecation-removal.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
FAMILY="$REPO_ROOT/scripts/audit/deprecation-removal.sh"
LIB="$REPO_ROOT/scripts/audit/lib.sh"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# make_tree NAME VERSION RULE-BODY [SECOND-RULE-BODY] — a fixture repository
# holding the family, its lib, a `version` file (omitted when VERSION is
# empty), up to two rule files (the first omitted when RULE-BODY is empty)
# and a spec citing BE-AUTH-009. Prints the directory.
make_tree() {
  local dir="$WORK/$1"
  mkdir -p "$dir/scripts/audit" "$dir/framework/rules" "$dir/specs/001-x"
  cp "$FAMILY" "$LIB" "$dir/scripts/audit/"
  if [ -n "$2" ]; then printf '%s\n' "$2" > "$dir/version"; fi
  if [ -n "$3" ]; then printf '%s\n' "$3" > "$dir/framework/rules/fixture-backend.md"; fi
  if [ -n "${4:-}" ]; then printf '%s\n' "$4" > "$dir/framework/rules/fixture-cross.md"; fi
  printf '# 001\n\nCites `BE-AUTH-009`.\n' > "$dir/specs/001-x/spec.md"
  git -C "$dir" init -q
  git -C "$dir" add -A
  echo "$dir"
}

# run_family DIR — run the fixture's copy; sets out, err and code.
run_family() {
  set +e
  out="$(bash "$1/scripts/audit/deprecation-removal.sh" 2> "$WORK/stderr")"
  code=$?
  set -e
  err="$(cat "$WORK/stderr")"
}

# rule LABEL [ID] — one rule file holding one rule whose label paragraph is LABEL.
rule() {
  printf '# Rules\n\n## BE-AUTH — Authentication\n\n### %s\n\n%s\n\n> Comparison MUST be constant-time.\n\n**Verification:** none.\n' "${2:-BE-AUTH-009}" "$1"
}

label() { printf '**DEPRECATED in %s:** moved to `SEC-AUTH-001`; removed in %s.' "$1" "$2"; }

# expect NAME CODE PATTERN — the last run exited CODE and its findings match
# PATTERN (an extended regex; empty means no finding at all).
expect() {
  if [ "$code" -ne "$2" ]; then fail "$1: exit $code, want $2: $out"; return; fi
  if [ -z "$3" ]; then
    if [ -n "$out" ]; then fail "$1: want no finding: $out"; else pass "$1"; fi
    return
  fi
  if printf '%s' "$out" | grep -qE -- "$3"; then pass "$1"; else fail "$1: no finding matched /$3/: $out"; fi
}

echo "A. the committed tree passes"
set +e
real_err="$(bash "$FAMILY" 2>&1 > /dev/null)"
real_code=$?
set -e
if [ "$real_code" -eq 0 ] \
  && printf '%s' "$real_err" | grep -qE 'rule file\(s\) examined; (no deprecation label|[0-9]+ deprecation label\(s\) found)'; then
  pass "exit 0 and a coverage line: $real_err"
else
  fail "exit $real_code, stderr: $real_err"
fi

echo "B. a target above version passes"
run_family "$(make_tree above-minor 0.61.0 "$(rule "$(label 0.60.0 0.62.0)")")"
expect "target 0.62.0, version 0.61.0" 0 ""
run_family "$(make_tree above-patch 0.62.0 "$(rule "$(label 0.60.0 0.62.1)")")"
expect "target 0.62.1, version 0.62.0" 0 ""

echo "C. a target equal to version fails, naming the rule and its citing files"
run_family "$(make_tree equal 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")"
expect "the finding at the label's line" 1 \
  'framework/rules/fixture-backend.md:7 [|] BE-AUTH-009 is deprecated in 0.60.0 and removed in 0.62.0, and version is 0.62.0'
expect "the citing spec named" 1 'specs/001-x/spec.md'

echo "D. a target below version fails, compared numerically"
run_family "$(make_tree below-major 1.0.0 "$(rule "$(label 0.60.0 0.62.0)")")"
expect "target 0.62.0, version 1.0.0" 1 'removed in 0.62.0, and version is 1.0.0'
run_family "$(make_tree below-minor 0.10.0 "$(rule "$(label 0.8.0 0.9.0)")")"
expect "target 0.9.0, version 0.10.0" 1 'removed in 0.9.0, and version is 0.10.0'
run_family "$(make_tree below-patch 0.62.10 "$(rule "$(label 0.60.0 0.62.9)")")"
expect "target 0.62.9, version 0.62.10" 1 'removed in 0.62.9, and version is 0.62.10'

echo "E. a label wrapped onto a second line is read whole"
run_family "$(make_tree wrapped 0.62.0 "$(rule "$(printf '**DEPRECATED in 0.60.0:** moved to `SEC-AUTH-001`;\nremoved in `0.62.0`.')")")"
expect "the target read from the second line" 1 'removed in 0.62.0, and version is 0.62.0'

echo "F. the target is matched in any case"
run_family "$(make_tree any-case 0.62.0 "$(rule '**DEPRECATED in 0.60.0:** moved to `SEC-AUTH-001`. Removed in 0.62.0.')")"
expect "Removed in 0.62.0" 1 'removed in 0.62.0, and version is 0.62.0'

echo "G. a label naming no target, two targets, or a malformed one fails"
run_family "$(make_tree no-target 0.60.0 "$(rule '**DEPRECATED in 0.60.0:** moved to `SEC-AUTH-001`.')")"
expect "no target" 1 'names no removal target'
run_family "$(make_tree two-targets 0.59.0 "$(rule '**DEPRECATED in 0.60.0:** replaces BE-OLD-001 (removed in 0.58.0); removed in 0.62.0.')")"
expect "two targets" 1 'names more than one removal target \(0.58.0, 0.62.0\)'
run_family "$(make_tree short-target 0.60.0 "$(rule '**DEPRECATED in 0.60.0:** moved; removed in 0.62.')")"
expect "target 0.62" 1 'removal target 0.62 is not MAJOR.MINOR.PATCH'

echo "H. a deprecating version that is not MAJOR.MINOR.PATCH fails"
run_family "$(make_tree bad-deprecated 0.60.0 "$(rule '**DEPRECATED in v0.60:** moved; removed in 0.62.0.')")"
expect "DEPRECATED in v0.60" 1 'DEPRECATED in v0.60, which is not MAJOR.MINOR.PATCH'

echo "I. a label outside any rule section fails"
run_family "$(make_tree no-section 0.60.0 "$(printf '# Rules\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "before any rule heading" 1 'outside any rule section'
run_family "$(make_tree after-category 0.60.0 "$(printf '# Rules\n\n## BE-XX\n\n### BE-XX-001\n\n> A MUST.\n\n## BE-YY\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "under a ## heading that follows a rule" 1 'framework/rules/fixture-backend.md:11 [|] a deprecation label outside any rule section'

echo "J. a label that does not open its line fails"
run_family "$(make_tree blockquote 0.60.0 "$(rule "> $(label 0.60.0 0.62.0)")")"
expect "a blockquoted label" 1 'does not open its line'

echo "K. two labels in two files are both read and counted"
run_family "$(make_tree two-files 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")" "$(rule "$(label 0.60.0 0.62.0)" SEC-AUTH-002)")"
expect "the first file's label" 1 'fixture-backend.md:7 [|] BE-AUTH-009'
expect "the second file's label" 1 'fixture-cross.md:7 [|] SEC-AUTH-002'
if printf '%s' "$err" | grep -q '2 of 2 rule file(s) examined; 2 deprecation label(s) found, 2 compared'; then
  pass "both counted: $err"
else
  fail "counts: $err"
fi

echo "L. no label passes and says there was nothing to compare"
run_family "$(make_tree no-label 0.62.0 "$(rule 'Nothing is deprecated here.')")"
expect "no finding" 0 ""
if printf '%s' "$err" | grep -q 'no deprecation label, so nothing to compare'; then pass "the distinct coverage line"; else fail "stderr: $err"; fi

echo "M. a missing or malformed version fails"
run_family "$(make_tree no-version '' "$(rule "$(label 0.60.0 0.62.0)")")"
expect "no version file" 1 '^deprecation-removal [|] version [|] missing'
run_family "$(make_tree short-version 0.62 "$(rule "$(label 0.60.0 0.62.0)")")"
expect "version 0.62" 1 "first line '0.62' is not MAJOR.MINOR.PATCH"
if printf '%s' "$err" | grep -q 'nothing compared'; then pass "a coverage line on the early exit"; else fail "stderr: $err"; fi

echo "N. no tracked rule file, or one that cannot be read, fails"
run_family "$(make_tree no-rules 0.60.0 '')"
expect "no tracked rule file" 1 'no tracked rule file'
d="$(make_tree unreadable 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")" "$(rule 'Nothing is deprecated here.' SEC-AUTH-002)")"
rm "$d/framework/rules/fixture-backend.md"
run_family "$d"
expect "a tracked file deleted from the working tree" 1 'fixture-backend.md [|] tracked but missing or unreadable'
if printf '%s' "$err" | grep -q '1 of 2 rule file(s) examined'; then pass "counted as not examined: $err"; else fail "stderr: $err"; fi

if [ "$failures" -ne 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "all cases passed"
