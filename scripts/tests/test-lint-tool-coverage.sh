#!/usr/bin/env bash
# Test surface for scripts/lint-tool-coverage.sh.
#
# The lint is the only check that a command naming a runtime tool carries the
# constitution's §runtime-host-integration pointer, so the evidence it works
# is that it FAILS on each condition it claims to catch, and that a subject it
# cannot examine never reads as one that passed (QUAL-CLAIM-001).
#
# Coverage:
#   A. the committed tree passes and says how much it examined (control)
#   B. a command naming a tool without the pointer fails, naming file and tool,
#      though another command carries it
#   C. the same command with the pointer passes, and the count says so
#   D. a command naming no tool needs no pointer
#   E. a name on a `# cli-only:` line counts as a tool
#   F. a manifest with no tool names fails rather than passing vacuously
#   G. a missing manifest fails
#   H. no command file at all fails rather than passing vacuously
#
# Usage: scripts/tests/test-lint-tool-coverage.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
LINT="$REPO_ROOT/scripts/lint-tool-coverage.sh"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# A throwaway tree holding a copy of the lint, which resolves its subject
# relative to its own location, with a manifest naming `read-spec` and the
# cli-only `run-generator`.
new_tree() {
  case_dir="$WORK/$1"
  mkdir -p "$case_dir/scripts" "$case_dir/framework/commands"
  cp "$LINT" "$case_dir/scripts/"
  printf '# header\nread-spec\n# cli-only: run-generator\n' > "$case_dir/framework/runtime-tools.txt"
}

# Run the case's lint, capturing its exit status and its combined output.
run_lint() {
  out="$(bash "$case_dir/scripts/lint-tool-coverage.sh" 2>&1)" && code=0 || code=$?
}

POINTER='> **For agent runtimes**: see §runtime-host-integration.'

echo "A. the committed tree"
case_dir="$REPO_ROOT"
run_lint
if [ "$code" -eq 0 ] && printf '%s' "$out" | grep -q 'command file(s) examined'; then
  pass "exit 0 with a count line"
else
  fail "A: exit $code: $out"
fi

echo "B. a tool named without the pointer, beside a command that carries it"
new_tree b
printf '# x\n\n1. Invoke `read-spec`.\n' > "$case_dir/framework/commands/x.md"
# A second command carrying the pointer: the rule is per file, so one file's
# pointer must not excuse another's.
printf '# y\n\n%s\n\n1. Invoke `read-spec`.\n' "$POINTER" > "$case_dir/framework/commands/y.md"
run_lint
if [ "$code" -eq 1 ] && printf '%s' "$out" | grep -q "framework/commands/x.md: names runtime tool 'read-spec'" \
  && ! printf '%s' "$out" | grep -q "framework/commands/y.md:"; then
  pass "exit 1 naming the file and the tool, and only that file"
else
  fail "B: exit $code: $out"
fi

echo "C. the same command with the pointer"
printf '# x\n\n%s\n\n1. Invoke `read-spec`.\n' "$POINTER" > "$case_dir/framework/commands/x.md"
run_lint
if [ "$code" -eq 0 ] && printf '%s' "$out" | grep -q '2 command file(s) examined, 2 naming a runtime tool, 2 of them carrying the pointer'; then
  pass "exit 0, counted"
else
  fail "C: exit $code: $out"
fi

echo "D. a command naming no tool"
new_tree d
printf '# help\n\nNo tools here.\n' > "$case_dir/framework/commands/help.md"
run_lint
if [ "$code" -eq 0 ] && printf '%s' "$out" | grep -q '1 command file(s) examined, 0 naming a runtime tool'; then
  pass "exit 0 with no pointer required"
else
  fail "D: exit $code: $out"
fi

echo "E. a cli-only name"
new_tree e
printf '# audit\n\n1. Invoke `run-generator`.\n' > "$case_dir/framework/commands/audit.md"
run_lint
if [ "$code" -eq 1 ] && printf '%s' "$out" | grep -q "names runtime tool 'run-generator'"; then
  pass "exit 1: the cli-only name is a tool"
else
  fail "E: exit $code: $out"
fi

echo "F. a manifest naming no tool"
new_tree f
printf '# only comments\n\n' > "$case_dir/framework/runtime-tools.txt"
printf '# x\n\n1. Invoke `read-spec`.\n' > "$case_dir/framework/commands/x.md"
run_lint
if [ "$code" -eq 2 ] && printf '%s' "$out" | grep -q 'nothing to examine'; then
  pass "exit 2"
else
  fail "F: exit $code: $out"
fi

echo "G. a missing manifest"
new_tree g
rm "$case_dir/framework/runtime-tools.txt"
run_lint
if [ "$code" -eq 2 ] && printf '%s' "$out" | grep -q 'Manifest not found'; then
  pass "exit 2"
else
  fail "G: exit $code: $out"
fi

echo "H. no command file"
new_tree h
run_lint
if [ "$code" -eq 2 ] && printf '%s' "$out" | grep -q 'No framework/commands/\*.md file found'; then
  pass "exit 2"
else
  fail "H: exit $code: $out"
fi

if [ "$failures" -gt 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "all cases passed"
