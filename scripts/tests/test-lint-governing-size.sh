#!/usr/bin/env bash
# Test surface for scripts/lint-governing-size.sh.
#
# The lint is the exit that keeps the governing documents small, so the only
# evidence it works is that it FAILS on each condition it claims to catch, and
# that a file which cannot be measured never reads as one that passed.
#
# Coverage:
#   A. the committed governed files pass (control)
#   B. AGENTS.md one byte over fails, naming file, size, ceiling and fix
#   C. every row's file exactly at its ceiling passes
#   D. the constitution one byte over fails
#   E. AGENTS.md absent from the index fails rather than passing vacuously
#   F. B's file brought back under the ceiling passes
#   G. a file the index holds but whose blob is gone fails as unreadable
#   H. the glob rows equal the runtime's DEFAULT_READ_SIZE_BYTES, and each
#      governing ceiling is below it and a multiple of 1,000 (spec 089)
#   I. a ceiling that is not a whole number of bytes fails rather than passing
#   J. a command file no row names, one byte over, fails: a glob governs it
#   K. a reference file one byte over fails
#   L. a glob matching no file fails rather than passing over nothing
#   M. a reference file whose blob is gone fails as unreadable
#
# Usage: scripts/tests/test-lint-governing-size.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
LINT="$REPO_ROOT/scripts/lint-governing-size.sh"

# The ceilings are read from the lint, so lowering one there needs no edit here.
ceiling_of() {
  awk -F'|' -v row="$1" '{ sub(/^CEILINGS="/, "") } $1 == row { print $2; exit }' "$LINT"
}
AGENTS_CEILING="$(ceiling_of AGENTS.md)"
CONSTITUTION_CEILING="$(ceiling_of framework/constitution.md)"
COMMANDS_CEILING="$(ceiling_of 'framework/commands/*.md')"
REFERENCE_CEILING="$(ceiling_of 'framework/reference/*.md')"
if [ -z "$AGENTS_CEILING" ] || [ -z "$CONSTITUTION_CEILING" ] ||
  [ -z "$COMMANDS_CEILING" ] || [ -z "$REFERENCE_CEILING" ]; then
  echo "could not read the ceilings from $LINT" >&2
  exit 1
fi

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# A throwaway repository holding a copy of the lint. The lint resolves its
# subject relative to its own location, so each case gets its own tree and
# never touches this checkout's index. Each tree holds one small command and
# one small reference file, so no case meets the empty-glob failure unless it
# removes one.
new_tree() {
  case_dir="$WORK/$1"
  mkdir -p "$case_dir/scripts" "$case_dir/framework/commands" "$case_dir/framework/reference"
  cp "$LINT" "$case_dir/scripts/"
  git -C "$case_dir" init -q
  printf 'seed command\n' > "$case_dir/framework/commands/seed.md"
  printf 'seed reference\n' > "$case_dir/framework/reference/seed.md"
  git -C "$case_dir" add -- framework/commands/seed.md framework/reference/seed.md
}

# Write a file of exactly $3 bytes at $2 inside tree $1 and stage it.
sized() {
  head -c "$3" /dev/zero | tr '\0' 'a' > "$WORK/$1/$2"
  git -C "$WORK/$1" add -- "$2"
}

run_lint() {
  bash "$WORK/$1/scripts/lint-governing-size.sh" > "$WORK/$1.out" 2>&1
}

echo "Running lint-governing-size tests..."

# A. control — the files as this checkout's index holds them, every command
# and reference file among them.
new_tree a
git -C "$WORK/a" rm -q --cached -- framework/commands/seed.md framework/reference/seed.md
rm -f "$WORK/a/framework/commands/seed.md" "$WORK/a/framework/reference/seed.md"
copied=0
for governed in AGENTS.md framework/constitution.md \
  $(git -C "$REPO_ROOT" ls-files -- ':(glob)framework/commands/*.md' ':(glob)framework/reference/*.md'); do
  git -C "$REPO_ROOT" show ":$governed" > "$WORK/a/$governed"
  git -C "$WORK/a" add -- "$governed"
  copied=$((copied + 1))
done
if [ "$copied" -lt 4 ]; then
  fail "A: copied $copied governed files from the index, too few to be the real set"
elif run_lint a; then
  pass "A: the committed governed files pass ($copied copied)"
else
  fail "A: the committed files fail the lint: $(cat "$WORK/a.out")"
fi

# B. one byte over, and the message carries everything a contributor needs.
new_tree b
sized b AGENTS.md $((AGENTS_CEILING + 1))
sized b framework/constitution.md 1000
if run_lint b; then
  fail "B: AGENTS.md over its ceiling passed"
elif grep -q "^AGENTS.md: $((AGENTS_CEILING + 1)) bytes, over its $AGENTS_CEILING-byte ceiling — retire, merge, or move an entry" "$WORK/b.out"; then
  pass "B: AGENTS.md one byte over fails, naming size, ceiling and fix"
else
  fail "B: failed with the wrong message: $(cat "$WORK/b.out")"
fi

# C. the ceiling is a maximum, not a strict bound.
new_tree c
sized c AGENTS.md "$AGENTS_CEILING"
sized c framework/constitution.md "$CONSTITUTION_CEILING"
sized c framework/commands/seed.md "$COMMANDS_CEILING"
sized c framework/reference/seed.md "$REFERENCE_CEILING"
if run_lint c; then
  pass "C: every row's file exactly at its ceiling passes"
else
  fail "C: a file at its ceiling failed: $(cat "$WORK/c.out")"
fi

# D. the constitution's own ceiling.
new_tree d
sized d AGENTS.md 1000
sized d framework/constitution.md $((CONSTITUTION_CEILING + 1))
if run_lint d; then
  fail "D: the constitution over its ceiling passed"
elif grep -q "^framework/constitution.md: $((CONSTITUTION_CEILING + 1)) bytes, over its $CONSTITUTION_CEILING-byte ceiling — cut it, not the ceiling" "$WORK/d.out"; then
  pass "D: the constitution one byte over fails, naming size, ceiling and fix"
else
  fail "D: failed with the wrong message: $(cat "$WORK/d.out")"
fi

# E. a file the index does not hold cannot be measured, and is not a pass.
new_tree e
sized e framework/constitution.md 1000
if run_lint e; then
  fail "E: AGENTS.md absent from the index passed"
elif grep -q "^AGENTS.md: not in the index, so its size cannot be measured$" "$WORK/e.out"; then
  pass "E: AGENTS.md absent from the index fails with its own message"
else
  fail "E: failed with the wrong message: $(cat "$WORK/e.out")"
fi

# F. B's tree, brought back under: what was refused is accepted once it fits.
sized b AGENTS.md $((AGENTS_CEILING - 1))
if run_lint b; then
  pass "F: AGENTS.md back under its ceiling passes"
else
  fail "F: AGENTS.md under its ceiling failed: $(cat "$WORK/b.out")"
fi

# G. the index names the file but its blob is gone — unreadable, not clean.
new_tree g
sized g AGENTS.md 1000
sized g framework/constitution.md 1000
blob="$(git -C "$WORK/g" rev-parse :AGENTS.md)"
rm -f "$WORK/g/.git/objects/${blob:0:2}/${blob:2}"
if run_lint g; then
  fail "G: an unreadable AGENTS.md passed"
elif grep -q "^AGENTS.md: could not be read from the index$" "$WORK/g.out"; then
  pass "G: an unreadable AGENTS.md fails with its own message"
else
  fail "G: failed with the wrong message: $(cat "$WORK/g.out")"
fi

# H. the glob rows copy a runtime constant, and each governing ceiling sits
# below it at a whole thousand, as spec 089 sets them.
RUNTIME_SRC="$REPO_ROOT/runtime/src/primitives/check_artifact_size.rs"
runtime_default="$(grep -o 'DEFAULT_READ_SIZE_BYTES: u64 = [0-9_]*' "$RUNTIME_SRC" 2>/dev/null | head -n 1 | sed 's/.*= //; s/_//g')"
if [ -z "$runtime_default" ]; then
  fail "H: DEFAULT_READ_SIZE_BYTES not found in $RUNTIME_SRC — nothing to compare the ceilings with"
elif [ "$runtime_default" != "$COMMANDS_CEILING" ] || [ "$runtime_default" != "$REFERENCE_CEILING" ]; then
  fail "H: a glob row's ceiling ($COMMANDS_CEILING, $REFERENCE_CEILING) differs from DEFAULT_READ_SIZE_BYTES ($runtime_default)"
elif [ "$AGENTS_CEILING" -ge "$runtime_default" ] || [ "$CONSTITUTION_CEILING" -ge "$runtime_default" ]; then
  fail "H: a governing ceiling ($AGENTS_CEILING, $CONSTITUTION_CEILING) is not below DEFAULT_READ_SIZE_BYTES ($runtime_default)"
elif [ $((AGENTS_CEILING % 1000)) -ne 0 ] || [ $((CONSTITUTION_CEILING % 1000)) -ne 0 ]; then
  fail "H: a governing ceiling ($AGENTS_CEILING, $CONSTITUTION_CEILING) is not a multiple of 1,000"
else
  pass "H: the glob rows equal DEFAULT_READ_SIZE_BYTES ($runtime_default) and both governing ceilings sit below it at a whole thousand"
fi

# I. a ceiling typed as `50,000` makes `[ -gt ]` error to stderr, which an
# `if` reads as false — the over-ceiling file would pass silently.
new_tree i
sed "s/^CEILINGS=\"AGENTS.md|$AGENTS_CEILING|/CEILINGS=\"AGENTS.md|50,000|/" "$LINT" > "$WORK/i/scripts/lint-governing-size.sh"
sized i AGENTS.md $((AGENTS_CEILING + 10000))
sized i framework/constitution.md 1000
if run_lint i; then
  fail "I: a malformed ceiling passed an over-size AGENTS.md"
elif grep -q "^AGENTS.md: its ceiling '50,000' is not a whole number of bytes" "$WORK/i.out"; then
  pass "I: a malformed ceiling fails with its own message"
else
  fail "I: failed with the wrong message: $(cat "$WORK/i.out")"
fi

# J. a new command file is governed from its first commit, with no row added.
new_tree j
sized j AGENTS.md 1000
sized j framework/constitution.md 1000
sized j framework/commands/new-command.md $((COMMANDS_CEILING + 1))
if run_lint j; then
  fail "J: a new command file over the read size passed"
elif grep -q "^framework/commands/new-command.md: $((COMMANDS_CEILING + 1)) bytes, over its $COMMANDS_CEILING-byte ceiling — retire a copy" "$WORK/j.out"; then
  pass "J: a command file no row names, one byte over, fails naming size, ceiling and fix"
else
  fail "J: failed with the wrong message: $(cat "$WORK/j.out")"
fi

# K. the reference row's own ceiling.
new_tree k
sized k AGENTS.md 1000
sized k framework/constitution.md 1000
sized k framework/reference/seed.md $((REFERENCE_CEILING + 1))
if run_lint k; then
  fail "K: a reference file over the read size passed"
elif grep -q "^framework/reference/seed.md: $((REFERENCE_CEILING + 1)) bytes, over its $REFERENCE_CEILING-byte ceiling — retire a copy" "$WORK/k.out"; then
  pass "K: a reference file one byte over fails naming size, ceiling and fix"
else
  fail "K: failed with the wrong message: $(cat "$WORK/k.out")"
fi

# L. a glob with nothing to measure is not a pass.
new_tree l
sized l AGENTS.md 1000
sized l framework/constitution.md 1000
git -C "$WORK/l" rm -q --cached -- framework/reference/seed.md
if run_lint l; then
  fail "L: a glob matching no file passed"
elif grep -q "^framework/reference/\*.md: matches no file in the index, so nothing it governs was measured$" "$WORK/l.out"; then
  pass "L: a glob matching no file fails with its own message"
else
  fail "L: failed with the wrong message: $(cat "$WORK/l.out")"
fi

# M. a glob's file whose blob is gone — unreadable, not clean.
new_tree m
sized m AGENTS.md 1000
sized m framework/constitution.md 1000
blob="$(git -C "$WORK/m" rev-parse :framework/reference/seed.md)"
rm -f "$WORK/m/.git/objects/${blob:0:2}/${blob:2}"
if run_lint m; then
  fail "M: an unreadable reference file passed"
elif grep -q "^framework/reference/seed.md: could not be read from the index$" "$WORK/m.out"; then
  pass "M: an unreadable reference file fails with its own message"
else
  fail "M: failed with the wrong message: $(cat "$WORK/m.out")"
fi

if [ "$failures" -gt 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "All cases passed."
