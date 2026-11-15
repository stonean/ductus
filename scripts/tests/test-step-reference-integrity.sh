#!/usr/bin/env bash
# Test surface for the file form of /audit Family 34
# (scripts/audit/step-reference-integrity.sh; spec 026 task 6, BE-PAYLOAD-001).
#
# `check-step-references` returns its five lists inline while the result fits
# `[artifacts] read-size-bytes` and past it in the file its `lists-file` names.
# The family renders the lists, so in the file form it must read them there,
# and a file it cannot read, or one lacking a list, must be a finding that
# says nothing was checked, never an empty result.
#
# Coverage:
#   A. inline form: a command file with an unresolved `step N` reference is
#      reported, and no list file is written
#   B. file form: the same tree with a read size too small for the result
#      reports the same finding, and the list file exists, so the file form
#      ran rather than the inline one
#   C. a result naming a list file that does not exist is a `(precondition)`
#      finding and reports no step reference as checked
#   D. a result naming a list file that lacks a list is a `(precondition)`
#      finding too
#
# Needs the release runtime (runtime/target/release/ductus), as
# framework-checks.yml builds it before this step.
#
# Usage: scripts/tests/test-step-reference-integrity.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
BIN="$REPO_ROOT/runtime/target/release/ductus"

if [ ! -x "$BIN" ]; then
  echo "no release runtime at $BIN — build it (cargo build --release in runtime/) before this test" >&2
  exit 1
fi

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# make_tree CASE — a fixture repository holding the family, its lib, the
# runtime as an adopter-owned pointer, and one command file whose Instructions
# refer to step 9, which it lacks. Prints the directory.
make_tree() {
  local dir="$WORK/$1/repo"
  mkdir -p "$dir/scripts/audit" "$dir/framework/commands" "$dir/.ductus/bin"
  cp "$REPO_ROOT/scripts/audit/lib.sh" "$REPO_ROOT/scripts/audit/step-reference-integrity.sh" \
    "$dir/scripts/audit/"
  ln -s "$BIN" "$dir/.ductus/bin/ductus"
  cat > "$dir/framework/commands/cmd.md" << 'MD'
# Cmd

## Instructions

1. One, see step 9.
2. Two.
3. Three.
MD
  printf '%s\n' "$dir"
}

# run_family DIR — run the fixture's copy of the family; sets out and code.
run_family() {
  code=0
  out="$("$BASH" "$1/scripts/audit/step-reference-integrity.sh" 2> /dev/null)" || code=$?
}

# stub DIR BODY — replace the fixture's runtime with a script printing BODY.
stub() {
  rm -f "$1/.ductus/bin/ductus"
  printf '#!/usr/bin/env bash\ncat << '"'"'JSON'"'"'\n%s\nJSON\n' "$2" > "$1/.ductus/bin/ductus"
  chmod +x "$1/.ductus/bin/ductus"
}

echo "A. inline form"
dir="$(make_tree inline)"
run_family "$dir"
inline_out="$out"
if [ "$code" = 1 ] && printf '%s' "$out" | grep -q 'cmd.md:5 references step 9'; then
  pass "an unresolved reference is reported"
else
  fail "inline form: exit $code, output: $out"
fi
if [ ! -e "$dir/.ductus/results" ]; then
  pass "no list file is written under the read size"
else
  fail "inline form wrote a list file"
fi

echo "B. file form"
dir="$(make_tree file)"
mkdir -p "$dir/.ductus"
printf '[artifacts]\nread-size-bytes = 1\n' > "$dir/.ductus/config.toml"
run_family "$dir"
if [ -n "$(find "$dir/.ductus/results/check-step-references" -name '*.json' 2> /dev/null)" ]; then
  pass "the result moved its lists to a file"
else
  fail "file form: no list file was written"
fi
if [ "$code" = 1 ] && [ "$out" = "$inline_out" ]; then
  pass "the file form reports what the inline form reports"
else
  fail "file form: exit $code, output: $out"
fi

echo "C. a missing list file"
dir="$(make_tree missing)"
stub "$dir" '{"form":"file","lists-file":".ductus/results/check-step-references/all-specs/absent.json","findings-count":1,"examined-count":1,"with-steps-count":1,"not-a-procedure-count":0,"skipped-count":0,"references-out-of-subject":0}'
run_family "$dir"
if [ "$code" = 1 ] && printf '%s' "$out" | grep -q '(precondition)' \
  && printf '%s' "$out" | grep -q 'so no step reference was checked'; then
  pass "an unreadable list file is a precondition finding"
else
  fail "missing list file: exit $code, output: $out"
fi

echo "D. a list file lacking a list"
dir="$(make_tree lacking)"
mkdir -p "$dir/.ductus/results"
printf '{"findings": [], "examined": ["x"], "with-steps": [], "not-a-procedure": []}\n' > "$dir/.ductus/results/partial.json"
stub "$dir" '{"form":"file","lists-file":".ductus/results/partial.json","findings-count":0,"examined-count":1,"with-steps-count":0,"not-a-procedure-count":0,"skipped-count":0,"references-out-of-subject":0}'
run_family "$dir"
if [ "$code" = 1 ] && printf '%s' "$out" | grep -q '(precondition)' \
  && printf '%s' "$out" | grep -q 'holds no skipped list'; then
  pass "a list file missing a list is a precondition finding"
else
  fail "list file lacking a list: exit $code, output: $out"
fi

if [ "$failures" -gt 0 ]; then
  echo "$failures failure(s)" >&2
  exit 1
fi
echo "all passed"
