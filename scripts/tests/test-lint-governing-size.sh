#!/usr/bin/env bash
# Test surface for scripts/lint-governing-size.sh.
#
# The lint is the exit that keeps the governing documents small, so the only
# evidence it works is that it FAILS on each condition it claims to catch, and
# that a file which cannot be measured never reads as one that passed.
#
# Coverage:
#   A. the committed AGENTS.md and constitution pass (control)
#   B. AGENTS.md one byte over fails, naming file, size, ceiling and fix
#   C. AGENTS.md exactly at its ceiling passes
#   D. the constitution one byte over fails
#   E. AGENTS.md absent from the index fails rather than passing vacuously
#   F. B's file brought back under the ceiling passes
#   G. a file the index holds but whose blob is gone fails as unreadable
#   H. both ceilings equal the runtime's DEFAULT_READ_SIZE_BYTES
#   I. a ceiling that is not a whole number of bytes fails rather than passing
#
# Usage: scripts/tests/test-lint-governing-size.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
LINT="$REPO_ROOT/scripts/lint-governing-size.sh"

# The ceilings are read from the lint, so lowering one there needs no edit here.
ceiling_of() { grep -o "$1|[0-9]*" "$LINT" | head -n 1 | cut -d'|' -f2; }
AGENTS_CEILING="$(ceiling_of AGENTS.md)"
CONSTITUTION_CEILING="$(ceiling_of framework/constitution.md)"
if [ -z "$AGENTS_CEILING" ] || [ -z "$CONSTITUTION_CEILING" ]; then
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
# never touches this checkout's index.
new_tree() {
  case_dir="$WORK/$1"
  mkdir -p "$case_dir/scripts" "$case_dir/framework"
  cp "$LINT" "$case_dir/scripts/"
  git -C "$case_dir" init -q
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

# A. control — the files as this checkout's index holds them.
new_tree a
git -C "$REPO_ROOT" show :AGENTS.md > "$WORK/a/AGENTS.md"
git -C "$REPO_ROOT" show :framework/constitution.md > "$WORK/a/framework/constitution.md"
git -C "$WORK/a" add -- AGENTS.md framework/constitution.md
if run_lint a; then
  pass "A: the committed AGENTS.md and constitution pass"
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
if run_lint c; then
  pass "C: both files exactly at their ceilings pass"
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

# H. each ceiling is a copy of a runtime constant, so both are held to it here.
RUNTIME_SRC="$REPO_ROOT/runtime/src/primitives/check_artifact_size.rs"
runtime_default="$(grep -o 'DEFAULT_READ_SIZE_BYTES: u64 = [0-9_]*' "$RUNTIME_SRC" 2>/dev/null | head -n 1 | sed 's/.*= //; s/_//g')"
if [ -z "$runtime_default" ]; then
  fail "H: DEFAULT_READ_SIZE_BYTES not found in $RUNTIME_SRC — nothing to compare the ceiling with"
elif [ "$runtime_default" != "$AGENTS_CEILING" ]; then
  fail "H: the AGENTS.md ceiling ($AGENTS_CEILING) differs from DEFAULT_READ_SIZE_BYTES ($runtime_default)"
elif [ "$runtime_default" != "$CONSTITUTION_CEILING" ]; then
  fail "H: the constitution's ceiling ($CONSTITUTION_CEILING) differs from DEFAULT_READ_SIZE_BYTES ($runtime_default)"
else
  pass "H: both ceilings equal the runtime's DEFAULT_READ_SIZE_BYTES ($runtime_default)"
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

if [ "$failures" -gt 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "All cases passed."
