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
#   B. a target above version passes, at the minor and at the patch number,
#      its coverage line naming the version, which is the first line of
#      `version`
#   C. a target equal to version fails, naming the rule and counting and
#      naming each tracked file citing its ID as a word
#   D. a target below version fails, compared numerically (0.9.0 < 0.10.0)
#   E. a label wrapped onto further lines is read whole, a wrap inside the
#      phrase included
#   F. the target is matched in any case: capitalised, all caps and mixed
#   G. a label naming no target (counted, not compared), two targets, or a
#      target that is not MAJOR.MINOR.PATCH (short, suffixed or prefixed)
#      fails
#   H. a deprecating version that is not MAJOR.MINOR.PATCH (short, suffixed
#      or prefixed) fails, and a backticked one is read
#   I. a label outside any rule section fails, and is counted, including one
#      in another form, a blockquoted, an indented and a mid-line one, one
#      under a ## heading that follows a rule and one on a rule's own heading
#      line
#   J. a label that does not open its line (a blockquote, an indent, a
#      mid-line mention) fails, and one opening it in another form says so
#   K. two labels in two files are both read and counted
#   L. no label at all passes, and says there was nothing to compare with
#      the version
#   M. a missing, unreadable or malformed (short, long or prefixed) version
#      fails rather than passing vacuously, with a coverage line
#   N. no tracked rule file, a tracked file that is deleted or unreadable
#      (not counted as examined, while a readable file's label is compared),
#      no tracked file that can be read, and git unable to list the tracked
#      files (no work tree, or no git on PATH) each fail, with a coverage line
#   O. two deprecated rules in one file are both compared
#   P. a second label in one section fails, in any form (another form, a
#      blockquote, a mid-line mention) and after a malformed first label
#   Q. a heading at the rule's level or above ends its section, an indented
#      one and a # one included, and a deeper one, another rule's included,
#      leaves it open, so a nested rule's own label is a second in the outer
#      rule's section and a label on a deeper heading line is the rule's; a ##
#      rule heading is a rule and a ##### one is not; a
#      tab after a rule heading's hashes reads as level 4, so a #### heading
#      ends its section and a ##### one does not; a lone carriage return or a
#      U+001C makes no heading; and a fenced block is not special-cased: each
#      as the runtime's section_is_deprecated reads it
#   R. only a `removed in` that starts like a version is a target, and the
#      paragraph ends at a blank line (one of spaces included), a block
#      quote, a **Field:** line (a line merely opening with bold does not), a
#      heading, a list item (-, *, +, 1. or 1)) or a fence (``` or ~~~)
#   S. one target named twice says so
#   T. a rule file that is not UTF-8 is reported and not examined, its label
#      not compared and the other files' labels still compared; a parse that
#      cannot run fails: no python3 on PATH, a python3 that fails, and one that
#      prints nothing; and a module at the repository root does not shadow the
#      parse's standard library
#   U. a rule file in a subdirectory of framework/rules/, or one not
#      tracked, is not read
#   V. one label's outcome never hides another's: a due label after one
#      above version, in one file and in a later file; a malformed label
#      before or after a due one, in one file and across files; two
#      malformed labels in one file; two labels outside any section; a third
#      label in one section; and a label outside any section, or a second in
#      one, before a due one are each reported or compared
#
# Every case fails as well on a finding that none of its expects claims or that
# is printed twice, so a run that adds a wrong or a repeated finding beside the
# right one does not pass.
#
# Usage: scripts/tests/test-deprecation-removal.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
FAMILY="$REPO_ROOT/scripts/audit/deprecation-removal.sh"
LIB="$REPO_ROOT/scripts/audit/lib.sh"

failures=0
skipped=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }
skip() { printf '  SKIP  %s\n' "$1"; skipped=$((skipped + 1)); }

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

# run_family DIR [PATH] — run the fixture's copy, under PATH when given; sets
# out, err and code.
run_family() {
  settle
  set +e
  if [ -n "${2:-}" ]; then
    out="$(PATH="$2" "$BASH" "$1/scripts/audit/deprecation-removal.sh" 2> "$WORK/stderr")"
  else
    out="$("$BASH" "$1/scripts/audit/deprecation-removal.sh" 2> "$WORK/stderr")"
  fi
  code=$?
  set -e
  err="$(cat "$WORK/stderr")"
}

# unreadable FILE — make FILE unreadable; fails when this user still reads it
# (root), so the caller skips the case rather than passing it.
unreadable() {
  chmod 000 "$1"
  [ ! -r "$1" ]
}

# counted PATTERN — the last run's coverage line matches PATTERN.
counted() {
  if printf '%s' "$err" | grep -qE -- "$1"; then pass "coverage: $err"; else fail "coverage, want /$1/: $err"; fi
}

# rule LABEL [ID] — one rule file holding one rule whose label paragraph is LABEL.
rule() {
  printf '# Rules\n\n## BE-AUTH — Authentication\n\n### %s\n\n%s\n\n> Comparison MUST be constant-time.\n\n**Verification:** none.\n' "${2:-BE-AUTH-009}" "$1"
}

label() { printf '**DEPRECATED in %s:** moved to `SEC-AUTH-001`; removed in %s.' "$1" "$2"; }

# two_rules LABEL LABEL — one rule file holding BE-AUTH-009 and BE-AUTH-010,
# each with the label paragraph given, at lines 7 and 13.
two_rules() {
  printf '# Rules\n\n## BE-AUTH — Authentication\n\n### BE-AUTH-009\n\n%s\n\n> A MUST.\n\n### BE-AUTH-010\n\n%s\n\n> A MUST.\n' "$1" "$2"
}

# expect NAME CODE PATTERN — the last run exited CODE and its findings match
# PATTERN (an extended regex; empty means no finding at all).
expect() {
  [ -n "$case_name" ] || case_name="$1"
  patterns="$patterns$3
"
  if [ "$code" -ne "$2" ]; then fail "$1: exit $code, want $2: $out"; return; fi
  if [ -z "$3" ]; then
    if [ -n "$out" ]; then fail "$1: want no finding: $out"; else pass "$1"; fi
    return
  fi
  if printf '%s' "$out" | grep -qE -- "$3"; then pass "$1"; else fail "$1: no finding matched /$3/: $out"; fi
}

# settle — every finding the last run printed must match a pattern an expect
# claimed for that run, and none may be printed twice, so a run that adds a
# wrong or a repeated finding beside the right one fails. run_family calls it before each run, and the summary once at the end.
patterns=""
case_name=""
out=""
settle() {
  if [ -n "$case_name" ] && [ -n "$out" ]; then
    repeated="$(printf '%s\n' "$out" | sort | uniq -d)"
    if [ -n "$repeated" ]; then fail "$case_name: a finding printed more than once: $repeated"; fi
    while IFS= read -r finding; do
      [ -n "$finding" ] || continue
      matched=0
      while IFS= read -r p; do
        [ -n "$p" ] || continue
        if printf '%s' "$finding" | grep -qE -- "$p"; then matched=1; break; fi
      done << EOF
$patterns
EOF
      if [ "$matched" -eq 0 ]; then fail "$case_name: a finding no expect claims: $finding"; fi
    done << EOF
$out
EOF
  fi
  patterns=""
  case_name=""
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
counted '^deprecation-removal: 1 of 1 rule file[(]s[)] examined; 1 deprecation label[(]s[)] found, 1 compared with version 0.61.0$'
run_family "$(make_tree two-line-version "$(printf '0.61.0\n0.62.0')" "$(rule "$(label 0.60.0 0.62.0)")")"
expect "only the first line of version is compared" 0 ""
run_family "$(make_tree above-patch 0.62.0 "$(rule "$(label 0.60.0 0.62.1)")")"
expect "target 0.62.1, version 0.62.0" 0 ""

echo "C. a target equal to version fails, naming the rule and its citing files"
run_family "$(make_tree equal 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")"
expect "the finding at the label's line" 1 \
  'framework/rules/fixture-backend.md:7 [|] BE-AUTH-009 is deprecated in 0.60.0 and removed in 0.62.0, and version is 0.62.0'
expect "the citing spec named" 1 'specs/001-x/spec.md'
expect "each citing file counted" 1 'each of the 2 tracked file[(]s[)] citing it [(]framework/rules/fixture-backend.md specs/001-x/spec.md[)]'
d="$(make_tree word-cite 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")"
mkdir -p "$d/specs/002-y"
printf '# 002\n\nCites `BE-AUTH-0090`.\n' > "$d/specs/002-y/spec.md"
git -C "$d" add -A
run_family "$d"
expect "a longer ID is not a citation" 1 'each of the 2 tracked file[(]s[)] citing it'

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
run_family "$(make_tree wrapped-phrase 0.62.0 "$(rule "$(printf '**DEPRECATED in 0.60.0:** moved to `SEC-AUTH-001`; removed\nin 0.62.0.')")")"
expect "a wrap inside the phrase" 1 'removed in 0.62.0, and version is 0.62.0'
run_family "$(make_tree wrapped-version 0.62.0 "$(rule "$(printf '**DEPRECATED in 0.60.0:** moved to `SEC-AUTH-001`; removed in\n0.62.0.')")")"
expect "a wrap before the version" 1 'removed in 0.62.0, and version is 0.62.0'
run_family "$(make_tree three-lines 0.62.0 "$(rule "$(printf '**DEPRECATED in 0.60.0:** moved\nto `SEC-AUTH-001`;\nremoved in 0.62.0.')")")"
expect "a label of three lines" 1 'removed in 0.62.0, and version is 0.62.0'

echo "F. the target is matched in any case"
run_family "$(make_tree any-case 0.62.0 "$(rule '**DEPRECATED in 0.60.0:** moved to `SEC-AUTH-001`. Removed in 0.62.0.')")"
expect "Removed in 0.62.0" 1 'removed in 0.62.0, and version is 0.62.0'
run_family "$(make_tree all-caps 0.62.0 "$(rule '**DEPRECATED in 0.60.0:** moved to `SEC-AUTH-001`. REMOVED IN 0.62.0.')")"
expect "REMOVED IN 0.62.0" 1 'removed in 0.62.0, and version is 0.62.0'
run_family "$(make_tree mixed-case 0.62.0 "$(rule '**DEPRECATED in 0.60.0:** moved to `SEC-AUTH-001`; removed In 0.62.0.')")"
expect "removed In 0.62.0" 1 'removed in 0.62.0, and version is 0.62.0'

echo "G. a label naming no target, two targets, or a malformed one fails"
run_family "$(make_tree no-target 0.60.0 "$(rule '**DEPRECATED in 0.60.0:** moved to `SEC-AUTH-001`.')")"
expect "no target" 1 'names no removal target'
counted '1 deprecation label[(]s[)] found, 0 compared'
run_family "$(make_tree two-targets 0.59.0 "$(rule '**DEPRECATED in 0.60.0:** replaces BE-OLD-001 (removed in 0.58.0); removed in 0.62.0.')")"
expect "two targets" 1 'names more than one removal target \(0.58.0, 0.62.0\)'
run_family "$(make_tree short-target 0.60.0 "$(rule '**DEPRECATED in 0.60.0:** moved; removed in 0.62.')")"
expect "target 0.62" 1 'removal target 0.62 is not MAJOR.MINOR.PATCH'
run_family "$(make_tree long-target 0.62.0 "$(rule '**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.1.')")"
expect "target 0.62.0.1" 1 'removal target 0.62.0.1 is not MAJOR.MINOR.PATCH'
run_family "$(make_tree suffixed-target 0.62.0 "$(rule '**DEPRECATED in 0.60.0:** moved; removed in 0.62.0-rc.1.')")"
expect "target 0.62.0-rc.1" 1 'removal target 0.62.0-rc.1 is not MAJOR.MINOR.PATCH'
run_family "$(make_tree prefixed-target 0.61.0 "$(rule '**DEPRECATED in 0.60.0:** moved; removed in v0.62.0.')")"
expect "target v0.62.0" 1 'removal target v0.62.0 is not MAJOR.MINOR.PATCH'

echo "H. a deprecating version that is not MAJOR.MINOR.PATCH fails"
run_family "$(make_tree bad-deprecated 0.60.0 "$(rule '**DEPRECATED in v0.60:** moved; removed in 0.62.0.')")"
expect "DEPRECATED in v0.60" 1 'DEPRECATED in v0.60, which is not MAJOR.MINOR.PATCH'
run_family "$(make_tree suffixed-deprecated 0.62.0 "$(rule '**DEPRECATED in 0.60.0-rc.1:** moved; removed in 0.62.0.')")"
expect "DEPRECATED in 0.60.0-rc.1" 1 'DEPRECATED in 0.60.0-rc.1, which is not MAJOR.MINOR.PATCH'
run_family "$(make_tree prefixed-deprecated 0.61.0 "$(rule '**DEPRECATED in v0.60.0:** moved; removed in 0.62.0.')")"
expect "DEPRECATED in v0.60.0" 1 'DEPRECATED in v0.60.0, which is not MAJOR.MINOR.PATCH'
run_family "$(make_tree backticked-deprecated 0.62.0 "$(rule '**DEPRECATED in `0.60.0`:** moved; removed in 0.62.0.')")"
expect "a backticked deprecating version is read" 1 'BE-AUTH-009 is deprecated in 0.60.0 and removed in 0.62.0'

echo "I. a label outside any rule section fails"
run_family "$(make_tree no-section 0.60.0 "$(printf '# Rules\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "before any rule heading" 1 'outside any rule section'
counted '1 deprecation label[(]s[)] found, 0 compared'
run_family "$(make_tree no-section-other-form 0.60.0 "$(printf '# Rules\n\n**DEPRECATED:** moved; removed in 0.62.0.\n')")"
expect "one in another form" 1 'fixture-backend.md:3 [|] a deprecation label outside any rule section'
counted '1 deprecation label[(]s[)] found, 0 compared'
run_family "$(make_tree on-heading 0.60.0 "$(printf '# Rules\n\n### BE-XX-001 **DEPRECATED in 0.60.0:** removed in 0.62.0.\n\n> A MUST.\n')")"
expect "on the rule's own heading line" 1 'fixture-backend.md:3 [|] a deprecation label outside any rule section'
counted '1 deprecation label[(]s[)] found, 0 compared'
run_family "$(make_tree after-category 0.60.0 "$(printf '# Rules\n\n## BE-XX\n\n### BE-XX-001\n\n> A MUST.\n\n## BE-YY\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "under a ## heading that follows a rule" 1 'framework/rules/fixture-backend.md:11 [|] a deprecation label outside any rule section'
k=0
for outside in '> **DEPRECATED in 0.60.0:** moved; removed in 0.62.0.' '  **DEPRECATED in 0.60.0:** moved; removed in 0.62.0.' 'Rules are retired with a **DEPRECATED in X.Y.Z:** label.'; do
  k=$((k + 1))
  run_family "$(make_tree "outside-$k" 0.60.0 "$(printf '# Rules\n\n%s\n\n### BE-XX-001\n\n> A MUST.\n' "$outside")")"
  expect "outside any section, $outside" 1 'fixture-backend.md:3 [|] a deprecation label outside any rule section'
  counted '1 deprecation label[(]s[)] found, 0 compared'
done

echo "J. a label that does not open its line fails"
run_family "$(make_tree blockquote 0.60.0 "$(rule "> $(label 0.60.0 0.62.0)")")"
expect "a blockquoted label" 1 'does not open its line'
run_family "$(make_tree indented 0.60.0 "$(rule "  $(label 0.60.0 0.62.0)")")"
expect "an indented label" 1 "BE-AUTH-009's deprecation label does not open its line"
run_family "$(make_tree mid-line 0.60.0 "$(rule "Moved: $(label 0.60.0 0.62.0)")")"
expect "a mid-line label" 1 "BE-AUTH-009's deprecation label does not open its line"
run_family "$(make_tree other-form 0.60.0 "$(rule '**DEPRECATED:** moved; removed in 0.62.0.')")"
expect "**DEPRECATED:**" 1 'is not in the form [*][*]DEPRECATED in X[.]Y[.]Z:[*][*]'

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
if printf '%s' "$err" | grep -q 'no deprecation label, so nothing to compare with version 0.62.0'; then pass "the distinct coverage line"; else fail "stderr: $err"; fi

echo "M. a missing, unreadable or malformed version fails"
run_family "$(make_tree no-version '' "$(rule "$(label 0.60.0 0.62.0)")")"
expect "no version file" 1 '^deprecation-removal [|] version [|] missing'
counted 'nothing compared: version is missing'
run_family "$(make_tree short-version 0.62 "$(rule "$(label 0.60.0 0.62.0)")")"
expect "version 0.62" 1 "first line '0.62' is not MAJOR.MINOR.PATCH"
if printf '%s' "$err" | grep -q 'nothing compared'; then pass "a coverage line on the early exit"; else fail "stderr: $err"; fi
run_family "$(make_tree long-version 0.62.0.1 "$(rule "$(label 0.60.0 0.62.0)")")"
expect "version 0.62.0.1" 1 "first line '0.62.0.1' is not MAJOR.MINOR.PATCH"
run_family "$(make_tree prefixed-version v0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")"
expect "version v0.62.0" 1 "first line 'v0.62.0' is not MAJOR.MINOR.PATCH"
d="$(make_tree unreadable-version 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")"
if unreadable "$d/version"; then
  run_family "$d"
  expect "a version file that cannot be read" 1 '^deprecation-removal [|] version [|] not readable'
  counted 'nothing compared: version is not readable'
else
  skip "an unreadable version: this user reads a mode-000 file"
fi

echo "N. no tracked rule file, or one that cannot be read, fails"
run_family "$(make_tree no-rules 0.60.0 '')"
expect "no tracked rule file" 1 'no tracked rule file'
counted 'nothing compared: no tracked rule file'
d="$(make_tree not-a-work-tree 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")"
rm -rf "$d/.git"
run_family "$d"
expect "a directory that is not a work tree" 1 'git could not list the tracked rule files'
counted 'nothing compared: git could not list the tracked rule files'
no_git="$WORK/bin-no-git"
mkdir -p "$no_git"
for tool in head tr grep sed dirname; do
  ln -s "$(command -v "$tool")" "$no_git/$tool"
done
run_family "$(make_tree no-git 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")" "$no_git"
expect "no git on PATH" 1 'git could not list the tracked rule files'
counted 'nothing compared: git could not list the tracked rule files'
d="$(make_tree unreadable 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")" "$(rule 'Nothing is deprecated here.' SEC-AUTH-002)")"
rm "$d/framework/rules/fixture-backend.md"
run_family "$d"
expect "a tracked file deleted from the working tree" 1 'fixture-backend.md [|] tracked but missing or unreadable'
if printf '%s' "$err" | grep -q '1 of 2 rule file(s) examined'; then pass "counted as not examined: $err"; else fail "stderr: $err"; fi
d="$(make_tree unreadable-mode 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")" "$(rule 'Nothing is deprecated here.' SEC-AUTH-002)")"
if unreadable "$d/framework/rules/fixture-backend.md"; then
  run_family "$d"
  expect "a tracked file that cannot be read" 1 'fixture-backend.md [|] tracked but missing or unreadable'
  counted '1 of 2 rule file[(]s[)] examined'
else
  skip "an unreadable rule file: this user reads a mode-000 file"
fi
d="$(make_tree readable-label 0.62.0 "$(rule 'Nothing is deprecated here.')" "$(rule "$(label 0.60.0 0.62.0)" SEC-AUTH-002)")"
rm "$d/framework/rules/fixture-backend.md"
run_family "$d"
expect "the deleted file" 1 'fixture-backend.md [|] tracked but missing or unreadable'
expect "the readable file's label is compared" 1 'fixture-cross.md:7 [|] SEC-AUTH-002 is deprecated'
counted '1 of 2 rule file[(]s[)] examined; 1 deprecation label[(]s[)] found, 1 compared with version 0.62.0'
d="$(make_tree none-readable 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")"
rm "$d/framework/rules/fixture-backend.md"
run_family "$d"
expect "no tracked file can be read" 1 'fixture-backend.md [|] tracked but missing or unreadable'
counted 'nothing compared: none of 1 tracked rule file[(]s[)] could be read'

echo "O. two deprecated rules in one file are both compared"
run_family "$(make_tree two-in-one 0.62.0 "$(printf '# Rules\n\n## BE-AUTH — Authentication\n\n### BE-AUTH-009\n\n%s\n\n> A MUST.\n\n### BE-AUTH-010\n\n%s\n\n> A MUST.\n' "$(label 0.60.0 0.62.0)" "$(label 0.60.0 0.62.0)")")"
expect "the first rule" 1 'fixture-backend.md:7 [|] BE-AUTH-009 is deprecated'
expect "the second rule" 1 'fixture-backend.md:13 [|] BE-AUTH-010 is deprecated'
counted '1 of 1 rule file[(]s[)] examined; 2 deprecation label[(]s[)] found, 2 compared'

echo "P. a second label in one section fails"
run_family "$(make_tree second-label 0.60.0 "$(rule "$(printf '%s\n\n%s' "$(label 0.60.0 0.62.0)" "$(label 0.60.0 0.63.0)")")")"
expect "the second label" 1 "fixture-backend.md:9 [|] a second deprecation label in BE-AUTH-009's section, after the one at line 7"
k=0
for second in '**DEPRECATED:** moved; removed in 0.63.0.' '> **DEPRECATED in 0.60.0:** moved; removed in 0.63.0.' 'See the **DEPRECATED in 0.60.0:** label above.'; do
  k=$((k + 1))
  run_family "$(make_tree "second-other-$k" 0.60.0 "$(rule "$(printf '%s\n\n%s' "$(label 0.60.0 0.62.0)" "$second")")")"
  expect "a second label, $second" 1 "fixture-backend.md:9 [|] a second deprecation label in BE-AUTH-009's section, after the one at line 7"
done
run_family "$(make_tree second-after-malformed 0.60.0 "$(rule "$(printf '%s\n\n%s' '**DEPRECATED:** moved.' "$(label 0.60.0 0.62.0)")")")"
expect "a malformed first label" 1 "fixture-backend.md:7 [|] BE-AUTH-009's deprecation label is not in the form"
expect "and a second after it" 1 "fixture-backend.md:9 [|] a second deprecation label in BE-AUTH-009's section, after the one at line 7"

echo "Q. section bounds follow the runtime's section_is_deprecated"
run_family "$(make_tree same-level 0.60.0 "$(printf '# Rules\n\n### BE-XX-001\n\n> A MUST.\n\n### Notes\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "a ### heading ends a ### rule's section" 1 'fixture-backend.md:9 [|] a deprecation label outside any rule section'
run_family "$(make_tree deeper 0.62.0 "$(printf '# Rules\n\n### BE-XX-001\n\n> A MUST.\n\n#### Notes\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "a #### heading leaves it open" 1 'fixture-backend.md:9 [|] BE-XX-001 is deprecated in 0.60.0 and removed in 0.62.0'
run_family "$(make_tree nested 0.62.0 "$(printf '# Rules\n\n### BE-XX-001\n\n> A MUST.\n\n#### BE-XX-002\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "a nested rule's label is the outer rule's too" 1 'fixture-backend.md:9 [|] BE-XX-001 is deprecated'
expect "and the nested rule's" 1 'fixture-backend.md:9 [|] BE-XX-002 is deprecated'
counted '1 deprecation label[(]s[)] found, 1 compared'
run_family "$(make_tree nested-both 0.62.0 "$(printf '# Rules\n\n### BE-XX-001\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n\n#### BE-XX-002\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "the outer rule's own label" 1 'fixture-backend.md:5 [|] BE-XX-001 is deprecated'
expect "a nested rule's own label is a second in the outer rule's section" 1 "fixture-backend.md:9 [|] a second deprecation label in BE-XX-001's section, after the one at line 5"
expect "and the nested rule's label" 1 'fixture-backend.md:9 [|] BE-XX-002 is deprecated'
run_family "$(make_tree deeper-heading-label 0.62.0 "$(printf '# Rules\n\n### BE-XX-001\n\n> A MUST.\n\n#### Notes **DEPRECATED in 0.60.0:** removed in 0.62.0.\n')")"
expect "a label on a deeper heading line is the rule's" 1 "fixture-backend.md:7 [|] BE-XX-001's deprecation label does not open its line"
run_family "$(make_tree tab 0.62.0 "$(printf '# Rules\n\n###\tBE-XX-001\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n\n### BE-XX-002\n\n> A MUST.\n')")"
expect "a tab after the hashes" 1 'fixture-backend.md:5 [|] BE-XX-001 is deprecated in 0.60.0 and removed in 0.62.0'
run_family "$(make_tree tab-level 0.62.0 "$(printf '# Rules\n\n###\tBE-XX-001\n\n#### Notes\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "a tab heading is read as level 4, so a #### heading ends it" 1 'fixture-backend.md:7 [|] a deprecation label outside any rule section'
run_family "$(make_tree tab-below 0.62.0 "$(printf '# Rules\n\n###\tBE-XX-001\n\n##### Notes\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "and a ##### heading leaves it open" 1 'fixture-backend.md:7 [|] BE-XX-001 is deprecated'
run_family "$(make_tree level-2-rule 0.62.0 "$(printf '# Rules\n\n## BE-XX-001\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "a ## rule heading is a rule" 1 'fixture-backend.md:5 [|] BE-XX-001 is deprecated'
run_family "$(make_tree level-5-rule 0.62.0 "$(printf '# Rules\n\n##### BE-XX-001\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "a ##### one is not" 1 'fixture-backend.md:5 [|] a deprecation label outside any rule section'
run_family "$(make_tree indented-heading 0.62.0 "$(printf '# Rules\n\n### BE-XX-001\n\n> A MUST.\n\n  ### Notes\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "an indented heading ends a section" 1 'fixture-backend.md:9 [|] a deprecation label outside any rule section'
run_family "$(make_tree level-1-ends 0.62.0 "$(printf '# Rules\n\n### BE-XX-001\n\n> A MUST.\n\n# Next\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "and so does a # heading" 1 'fixture-backend.md:9 [|] a deprecation label outside any rule section'
run_family "$(make_tree lone-cr 0.62.0 "$(printf '# Rules\n\n### BE-XX-001\n\n> A MUST.\r### Notes\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "a lone carriage return does not split a line" 1 'fixture-backend.md:7 [|] BE-XX-001 is deprecated'
run_family "$(make_tree unit-separator 0.62.0 "$(printf '# Rules\n\n### BE-XX-001\n\n\034### Notes\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "U+001C is not whitespace before the hashes" 1 'fixture-backend.md:7 [|] BE-XX-001 is deprecated'
run_family "$(make_tree unit-separator-heading 0.62.0 "$(printf '# Rules\n\n###\034BE-XX-001\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "nor after them" 1 'fixture-backend.md:5 [|] a deprecation label outside any rule section'
run_family "$(make_tree fenced-label 0.62.0 "$(printf '# Rules\n\n### BE-XX-001\n\n```\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n```\n')")"
expect "a label in a fenced block is read" 1 'fixture-backend.md:6 [|] BE-XX-001 is deprecated'
run_family "$(make_tree fenced-heading 0.62.0 "$(printf '# Rules\n\n### BE-XX-001\n\n```\n### Notes\n```\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n')")"
expect "and so is a heading in one" 1 'fixture-backend.md:9 [|] a deprecation label outside any rule section'

echo "R. a target starts like a version, and the paragraph ends at a quote or a field"
run_family "$(make_tree favour 0.62.0 "$(rule '**DEPRECATED in 0.60.0:** removed in favour of `SEC-AUTH-001`; removed in 0.62.0.')")"
expect "removed in favour of" 1 'BE-AUTH-009 is deprecated in 0.60.0 and removed in 0.62.0'
run_family "$(make_tree attached-quote 0.62.0 "$(rule "$(printf '%s\n> Tokens MUST be removed in 24 hours.' "$(label 0.60.0 0.62.0)")")")"
expect "a Statement on the next line" 1 'BE-AUTH-009 is deprecated in 0.60.0 and removed in 0.62.0'
run_family "$(make_tree attached-field 0.62.0 "$(rule "$(printf '%s\n**Verification:** a key removed in 1.0.0 is reported.' "$(label 0.60.0 0.62.0)")")")"
expect "a field on the next line" 1 'BE-AUTH-009 is deprecated in 0.60.0 and removed in 0.62.0'
run_family "$(make_tree after-blank 0.62.0 "$(rule "$(printf '%s\n\nTokens are removed in 24 hours.' "$(label 0.60.0 0.62.0)")")")"
expect "a paragraph after a blank line" 1 'BE-AUTH-009 is deprecated in 0.60.0 and removed in 0.62.0'
run_family "$(make_tree after-whitespace-line 0.62.0 "$(rule "$(printf '%s\n   \nTokens are removed in 24 hours.' "$(label 0.60.0 0.62.0)")")")"
expect "a line of spaces is blank" 1 'BE-AUTH-009 is deprecated in 0.60.0 and removed in 0.62.0'
run_family "$(make_tree bold-continuation 0.62.0 "$(rule "$(printf '**DEPRECATED in 0.60.0:** moved;\n**SEC-AUTH-001** succeeds it; removed in 0.62.0.')")")"
expect "a line opening with bold that is not a field continues it" 1 'BE-AUTH-009 is deprecated in 0.60.0 and removed in 0.62.0'
run_family "$(make_tree attached-list 0.62.0 "$(rule "$(printf '**DEPRECATED in 0.60.0:** moved.\n- Tokens are removed in 9.0.0.')")")"
expect "a list item on the next line" 1 "BE-AUTH-009's label names no removal target"
run_family "$(make_tree attached-fence 0.62.0 "$(rule "$(printf '**DEPRECATED in 0.60.0:** moved.\n```\nremoved in 9.0.0\n```')")")"
expect "a fence on the next line" 1 "BE-AUTH-009's label names no removal target"
n=0
for marker in '~~~' '* item' '+ item' '1. item' '1) item'; do
  n=$((n + 1))
  run_family "$(make_tree "attached-block-$n" 0.62.0 "$(rule "$(printf '**DEPRECATED in 0.60.0:** moved.\n%s removed in 9.0.0' "$marker")")")"
  expect "a block opening with $marker" 1 "BE-AUTH-009's label names no removal target"
done
run_family "$(make_tree attached-heading 0.62.0 "$(rule "$(printf '**DEPRECATED in 0.60.0:** moved.\n#### Removed in 0.62.0')")")"
expect "a heading on the next line" 1 "BE-AUTH-009's label names no removal target"

echo "S. one target named twice says so"
run_family "$(make_tree repeated 0.60.0 "$(rule '**DEPRECATED in 0.60.0:** moved. Removed in 0.62.0; removed in 0.62.0.')")"
expect "0.62.0 twice" 1 "BE-AUTH-009's label names its removal target 0.62.0 more than once"

echo "T. a rule file that is not UTF-8 is not examined, and a parse that cannot run fails"
d="$(make_tree not-utf8 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")" "$(rule "$(label 0.60.0 0.62.0)" SEC-AUTH-002)")"
printf '# Rules\n\n### BE-XX-001\n\n**DEPRECATED in 0.60.0:** moved; removed in 0.62.0.\n\n\377\n' > "$d/framework/rules/fixture-backend.md"
run_family "$d"
expect "a rule file that is not UTF-8" 1 'fixture-backend.md [|] not UTF-8, so its deprecation labels were not examined'
expect "the other file's label is still compared" 1 'fixture-cross.md:7 [|] SEC-AUTH-002 is deprecated'
counted '1 of 2 rule file[(]s[)] examined; 1 deprecation label[(]s[)] found, 1 compared'
d="$(make_tree only-not-utf8 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")"
printf '# Rules\n\n### BE-XX-001\n\n\377\n' > "$d/framework/rules/fixture-backend.md"
run_family "$d"
expect "the only rule file is not UTF-8" 1 'fixture-backend.md [|] not UTF-8'
counted 'nothing compared: none of 1 tracked rule file[(]s[)] could be read'
no_python="$WORK/bin-no-python"
mkdir -p "$no_python"
for tool in git head tr grep sed dirname; do
  ln -s "$(command -v "$tool")" "$no_python/$tool"
done
run_family "$(make_tree no-python 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")" "$no_python"
expect "no python3 on PATH" 1 'python3 is not on PATH'
counted 'nothing compared: python3 is missing'
silent_python="$WORK/bin-silent-python"
mkdir -p "$silent_python"
cp -P "$no_python"/* "$silent_python/"
printf '#!/bin/sh\nexit 0\n' > "$silent_python/python3"
chmod +x "$silent_python/python3"
run_family "$(make_tree silent-python 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")" "$silent_python"
expect "a python3 that prints nothing" 1 'the label parse printed no counts'
counted 'nothing compared: the label parse printed no counts'
failing_python="$WORK/bin-failing-python"
mkdir -p "$failing_python"
cp -P "$no_python"/* "$failing_python/"
printf '#!/bin/sh\nexit 1\n' > "$failing_python/python3"
chmod +x "$failing_python/python3"
run_family "$(make_tree failing-python 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")" "$failing_python"
expect "a python3 that fails" 1 'the label parse exited non-zero'
counted 'nothing compared: the label parse failed'
d="$(make_tree shadowing-module 0.62.0 "$(rule "$(label 0.60.0 0.62.0)")")"
printf 'raise SystemExit(0)\n' > "$d/re.py"
git -C "$d" add -A
run_family "$d"
expect "a re.py at the repository root does not shadow the standard library" 1 'BE-AUTH-009 is deprecated in 0.60.0 and removed in 0.62.0'

echo "U. a rule file in a subdirectory is not read"
d="$(make_tree subdirectory 0.62.0 "$(rule 'Nothing is deprecated here.')")"
mkdir -p "$d/framework/rules/sub"
rule "$(label 0.60.0 0.62.0)" > "$d/framework/rules/sub/fixture-backend.md"
git -C "$d" add -A
run_family "$d"
expect "framework/rules/sub/" 0 ""
counted '1 of 1 rule file[(]s[)] examined; no deprecation label'
d="$(make_tree untracked 0.62.0 "$(rule 'Nothing is deprecated here.')")"
rule "$(label 0.60.0 0.62.0)" > "$d/framework/rules/untracked-backend.md"
run_family "$d"
expect "an untracked rule file" 0 ""
counted '1 of 1 rule file[(]s[)] examined; no deprecation label'

echo "V. one label's outcome never hides another's"
run_family "$(make_tree notdue-then-due 0.62.0 "$(two_rules "$(label 0.60.0 0.63.0)" "$(label 0.60.0 0.62.0)")")"
expect "a due label after one above version" 1 'fixture-backend.md:13 [|] BE-AUTH-010 is deprecated'
counted '2 deprecation label[(]s[)] found, 2 compared'
run_family "$(make_tree notdue-then-due-files 0.62.0 "$(rule "$(label 0.60.0 0.63.0)")" "$(rule "$(label 0.60.0 0.62.0)" SEC-AUTH-002)")"
expect "and in a later file" 1 'fixture-cross.md:7 [|] SEC-AUTH-002 is deprecated'
counted '2 of 2 rule file[(]s[)] examined; 2 deprecation label[(]s[)] found, 2 compared'
run_family "$(make_tree malformed-then-due 0.62.0 "$(two_rules '**DEPRECATED in 0.60.0:** moved.' "$(label 0.60.0 0.62.0)")")"
expect "a malformed label" 1 "fixture-backend.md:7 [|] BE-AUTH-009's label names no removal target"
expect "and a due one after it" 1 'fixture-backend.md:13 [|] BE-AUTH-010 is deprecated'
counted '2 deprecation label[(]s[)] found, 1 compared'
run_family "$(make_tree due-then-malformed 0.62.0 "$(two_rules "$(label 0.60.0 0.62.0)" '**DEPRECATED in 0.60.0:** moved.')")"
expect "a due label" 1 'fixture-backend.md:7 [|] BE-AUTH-009 is deprecated'
expect "and a malformed one after it" 1 "fixture-backend.md:13 [|] BE-AUTH-010's label names no removal target"
run_family "$(make_tree malformed-then-due-files 0.62.0 "$(rule '**DEPRECATED in 0.60.0:** moved.')" "$(rule "$(label 0.60.0 0.62.0)" SEC-AUTH-002)")"
expect "a malformed label in one file" 1 "fixture-backend.md:7 [|] BE-AUTH-009's label names no removal target"
expect "and a due one in the next" 1 'fixture-cross.md:7 [|] SEC-AUTH-002 is deprecated'
run_family "$(make_tree two-malformed 0.62.0 "$(two_rules '**DEPRECATED in 0.60.0:** moved.' '**DEPRECATED in 0.60.0:** moved; removed in 0.62.')")"
expect "two malformed labels, the first" 1 "fixture-backend.md:7 [|] BE-AUTH-009's label names no removal target"
expect "and the second" 1 "fixture-backend.md:13 [|] BE-AUTH-010's removal target 0.62 is not MAJOR.MINOR.PATCH"
run_family "$(make_tree two-outside 0.60.0 "$(printf '# Rules\n\n%s\n\n%s\n' "$(label 0.60.0 0.62.0)" "$(label 0.60.0 0.62.0)")")"
expect "two labels outside any section, the first" 1 'fixture-backend.md:3 [|] a deprecation label outside any rule section'
expect "and the second" 1 'fixture-backend.md:5 [|] a deprecation label outside any rule section'
counted '2 deprecation label[(]s[)] found, 0 compared'
run_family "$(make_tree third-label 0.60.0 "$(rule "$(printf '%s\n\n%s\n\n%s' "$(label 0.60.0 0.62.0)" "$(label 0.60.0 0.63.0)" "$(label 0.60.0 0.64.0)")")")"
expect "a second label in one section" 1 "fixture-backend.md:9 [|] a second deprecation label in BE-AUTH-009's section, after the one at line 7"
expect "and a third" 1 "fixture-backend.md:11 [|] a second deprecation label in BE-AUTH-009's section, after the one at line 7"
run_family "$(make_tree outside-then-due 0.62.0 "$(printf '# Rules\n\n%s\n\n### BE-AUTH-009\n\n%s\n' "$(label 0.60.0 0.63.0)" "$(label 0.60.0 0.62.0)")")"
expect "a label outside any section" 1 'fixture-backend.md:3 [|] a deprecation label outside any rule section'
expect "and a due one after it" 1 'fixture-backend.md:7 [|] BE-AUTH-009 is deprecated'
run_family "$(make_tree second-then-due 0.62.0 "$(two_rules "$(printf '%s\n\n%s' "$(label 0.60.0 0.63.0)" "$(label 0.60.0 0.63.0)")" "$(label 0.60.0 0.62.0)")")"
expect "a second label in one section" 1 "fixture-backend.md:9 [|] a second deprecation label in BE-AUTH-009's section, after the one at line 7"
expect "and a due one in the next" 1 'fixture-backend.md:15 [|] BE-AUTH-010 is deprecated'

settle

if [ "$failures" -ne 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
if [ "$skipped" -ne 0 ]; then
  echo "all cases passed, $skipped skipped"
else
  echo "all cases passed"
fi
