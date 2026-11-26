#!/usr/bin/env bash
# Test surface for scripts/stale-literals.sh, the literal search of a
# landing's sweep (.ductus/docs/agents/developing-ductus.md §Landing a runtime change, step 3).
#
# The search is only worth running if two things hold:
#   - it lists what a diff left behind;
#   - it never reads "nothing found" when it did not search.
# Each fixture is a small git repository with an AGENTS.md naming its
# live-artifact set, so the real tree's content never decides a case.
#
# Coverage:
#   A. a literal the diff removes that another target still holds is listed
#      with that hit, and the run exits 0
#   B. a literal held only outside the live-artifact set is not listed
#   C. a literal an edited line keeps on both sides is not examined
#   D. a removed literal no longer held anywhere is examined but not listed
#   E. a comma-grouped figure is a literal, a trailing comma included; a
#      four-digit year and a sha's digits are not (the examined count is exact)
#   F. it cannot run, and exits 2, with no BASE, with a BASE that does not
#      resolve, and with an AGENTS.md that names no live-artifact set
#
# Usage: scripts/tests/test-stale-literals.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SUBJECT="$(cd "$SCRIPT_DIR/.." && pwd)/stale-literals.sh"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

gitc() { git -c user.email=t@example.invalid -c user.name=t "$@"; }

# A repository whose AGENTS.md names docs/ and specs/NNN-*/ as its live set,
# with a parenthesized aside the reader must leave out, committed as the base.
# The backticks in these printf formats are literal characters.
make_repo() {
  local d="$1"
  mkdir -p "$d/docs" "$d/specs/001-alpha" "$d/notes"
  # shellcheck disable=SC2016
  printf 'The set is <!-- audit:sweep-targets:begin --> `docs/` (including `docs/x/`), and `specs/NNN-*/` <!-- audit:sweep-targets:end -->.\n' > "$d/AGENTS.md"
  # shellcheck disable=SC2016
  printf 'The tool takes `old-flag` and weighs 137,600, as measured.\nIt keeps `kept-name` on this line.\nMeasured 2026-11-16 at 4a1b2c3d4e.\n' > "$d/docs/guide.md"
  # shellcheck disable=SC2016
  printf 'This page mentions `outside-only`.\n' > "$d/docs/extra.md"
  # shellcheck disable=SC2016
  printf 'The spec still says `old-flag`.\n' > "$d/specs/001-alpha/spec.md"
  # shellcheck disable=SC2016
  printf 'A note outside the set says `outside-only`.\n' > "$d/notes/n.md"
  (cd "$d" && git init -q && gitc add -- AGENTS.md docs specs notes && gitc commit -q -m base)
}

run() {
  local d="$1"
  shift
  set +e
  out="$(cd "$d" && bash "$SUBJECT" "$@" 2>&1)"
  code=$?
  set -e
}

# --- A-E: one uncommitted edit that removes and keeps several literals -------
R="$WORK/r1"
make_repo "$R"
# Removed: `old-flag` (still in specs/), 137,600 (held nowhere after), and the
# year-and-sha line; kept on both sides of an edited line: `kept-name`;
# removed with docs/extra.md: `outside-only`, still held only in notes/.
# shellcheck disable=SC2016
printf 'The tool takes `new-flag` and weighs 159,969 bytes.\nIt still keeps `kept-name` on this line.\n' > "$R/docs/guide.md"
rm "$R/docs/extra.md"
run "$R" HEAD

if [ "$code" -eq 0 ] && printf '%s\n' "$out" | grep -qF '== old-flag: 1 hit(s)' \
  && printf '%s\n' "$out" | grep -qF 'specs/001-alpha/spec.md:1:'; then
  pass "A: a removed literal another target still holds is listed with its hit"
else
  fail "A: old-flag not listed with its specs/001-alpha hit (exit $code): $out"
fi
if printf '%s\n' "$out" | grep -qF '== outside-only'; then
  fail "B: outside-only, held only in notes/, was listed: $out"
else
  pass "B: a literal held only outside the live-artifact set is not listed"
fi
if printf '%s\n' "$out" | grep -qF '== kept-name'; then
  fail "C: kept-name, kept on both sides of an edited line, was listed: $out"
else
  pass "C: a literal an edited line keeps on both sides is not examined"
fi
if printf '%s\n' "$out" | grep -qF '== 137,600'; then
  fail "D: 137,600, held nowhere after the edit, was listed: $out"
else
  pass "D: a removed literal held nowhere is not listed"
fi
if printf '%s\n' "$out" | grep -qF 'examined 3 literal(s) removed since HEAD over 2 target(s); 1 still found'; then
  pass "E: exactly old-flag, 137,600 and outside-only are examined; the year and the sha's digits are not literals"
else
  fail "E: expected 3 removed literals over 2 targets, 1 still found: $out"
fi

# --- F: it cannot run -------------------------------------------------------
run "$R"
if [ "$code" -eq 2 ]; then
  pass "F: no BASE exits 2"
else
  fail "F: no BASE exited $code: $out"
fi
run "$R" no-such-ref
if [ "$code" -eq 2 ] && printf '%s\n' "$out" | grep -qF 'does not resolve'; then
  pass "F: a BASE that does not resolve exits 2"
else
  fail "F: an unresolved BASE exited $code: $out"
fi
N="$WORK/r2"
make_repo "$N"
printf '# Agents\n\nNo marked set here.\n' > "$N/AGENTS.md"
run "$N" HEAD
if [ "$code" -eq 2 ] && printf '%s\n' "$out" | grep -qF 'names no live-artifact set'; then
  pass "F: an AGENTS.md naming no live-artifact set exits 2, never an empty pass"
else
  fail "F: a markerless AGENTS.md exited $code: $out"
fi

if [ "$failures" -ne 0 ]; then
  printf '%s case(s) failed\n' "$failures" >&2
  exit 1
fi
printf 'All stale-literals cases passed.\n'
