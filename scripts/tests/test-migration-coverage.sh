#!/usr/bin/env bash
# Test surface for scripts/audit/migration-coverage.sh check 10d (Family 10).
#
# 10d holds the registry's ordering key, (introduced_in, id), against the order
# entries were added: Pre-run Migrations runs only entries after an adopter's
# last_applied, so an entry added after another but sorting before it never
# runs for an adopter whose last_applied names the other (spec 027 task 15).
# The check reads git history, so the evidence it works is that it FAILS on the
# pre-re-id order, and that a checkout whose history it cannot read is a
# finding rather than a pass.
#
# Coverage:
#   A. entries added in order pass, with a coverage line (control)
#   B. a later-added entry that sorts before an earlier one at the same
#      introduced_in fails, naming both (the pre-re-id order of
#      legacy-constitutions-region)
#   C. the same entry re-id'd to sort after passes (the fixed registry)
#   D. a later-added entry with a lower introduced_in fails
#   E. two entries added in one commit are exempt from each other
#   F. the decided historical exception, criterion-label-backfill added after
#      ductus-rename and runtime-store-path, passes; the same shape under other
#      ids fails
#   G. an entry not yet committed counts as newest and is compared
#   H. a shallow clone fails rather than passing
#
# The fixture repositories hold their own copy of the check and its lib, so the
# check reads the fixture's history. The job that runs this needs the git
# binary only; the fixture history is built here, not read from the checkout.
#
# Usage: scripts/tests/test-migration-coverage.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# make_repo CASE — an empty fixture repository holding the check and its lib.
# Prints the directory.
make_repo() {
  local dir="$WORK/$1"
  mkdir -p "$dir/scripts/audit" "$dir/framework/migrations"
  cp "$REPO_ROOT/scripts/audit/migration-coverage.sh" "$REPO_ROOT/scripts/audit/lib.sh" "$dir/scripts/audit/"
  git -C "$dir" init -q
  echo "$dir"
}

# entry DIR ID VERSION — append a [[migrations]] entry and its procedure file.
entry() {
  printf '[[migrations]]\nid = "%s"\nintroduced_in = "%s"\nsummary = "s"\ntarget_paths = ["adopter/x"]\nprocedure_file = "framework/migrations/%s.md"\n\n' \
    "$2" "$3" "$2" >> "$1/framework/migrations.toml"
  printf '# %s\n' "$2" > "$1/framework/migrations/$2.md"
}

# commit DIR — commit everything in the fixture.
commit() {
  git -C "$1" add -A
  git -C "$1" -c user.name=t -c user.email=t@t -c commit.gpgsign=false commit -q -m "c"
}

# run_check DIR — run the fixture's copy; sets out (findings), err (coverage), code.
run_check() {
  set +e
  out="$("$BASH" "$1/scripts/audit/migration-coverage.sh" 2> "$WORK/err")"
  code=$?
  set -e
  err="$(cat "$WORK/err")"
}

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

echo "Running migration-coverage 10d tests..."

dir="$(make_repo a)"
entry "$dir" alpha 0.60.0; commit "$dir"
entry "$dir" beta 0.60.0; commit "$dir"
run_check "$dir"
expect "A: entries added in sort order pass" 0 ""
if printf '%s' "$err" | grep -q '10d examined: 2 entries, 1 earlier-added pairs compared, 2 commits read'; then
  pass "A: the coverage line states what was examined"
else
  fail "A: coverage line missing or wrong: $err"
fi

dir="$(make_repo b)"
entry "$dir" memory-routing 0.60.0; commit "$dir"
entry "$dir" legacy-constitutions-region 0.60.0; commit "$dir"
run_check "$dir"
expect "B: a later entry sorting before an earlier one fails, naming both" 1 \
  "'legacy-constitutions-region' .* was added after 'memory-routing' .* but sorts before it"

dir="$(make_repo c)"
entry "$dir" memory-routing 0.60.0; commit "$dir"
entry "$dir" remove-legacy-constitutions-region 0.60.0; commit "$dir"
run_check "$dir"
expect "C: the re-id'd entry sorts after and passes" 0 ""

dir="$(make_repo d)"
entry "$dir" alpha 0.60.0; commit "$dir"
entry "$dir" omega 0.59.0; commit "$dir"
run_check "$dir"
expect "D: a later entry with a lower introduced_in fails" 1 "'omega' .* was added after 'alpha'"

dir="$(make_repo e)"
entry "$dir" beta 0.60.0
entry "$dir" alpha 0.60.0
commit "$dir"
run_check "$dir"
expect "E: two entries added in one commit are exempt from each other" 0 ""

dir="$(make_repo f)"
entry "$dir" ductus-rename 0.28.0
entry "$dir" runtime-store-path 0.28.0
commit "$dir"
entry "$dir" criterion-label-backfill 0.28.0; commit "$dir"
run_check "$dir"
expect "F: the decided historical exception passes" 0 ""

dir="$(make_repo f2)"
entry "$dir" ductus-renamed 0.28.0
entry "$dir" runtime-store-paths 0.28.0
commit "$dir"
entry "$dir" criterion-label-backfills 0.28.0; commit "$dir"
run_check "$dir"
expect "F: the same shape under other ids fails" 1 "'criterion-label-backfills' .* was added after"

dir="$(make_repo g)"
entry "$dir" beta 0.60.0; commit "$dir"
entry "$dir" alpha 0.60.0
run_check "$dir"
expect "G: an uncommitted entry counts as newest and is compared" 1 "'alpha' .* was added after 'beta'"

dir="$(make_repo h)"
entry "$dir" alpha 0.60.0; commit "$dir"
entry "$dir" beta 0.60.0; commit "$dir"
git clone -q --depth 1 "file://$dir" "$WORK/h-shallow"
run_check "$WORK/h-shallow"
expect "H: a shallow clone fails rather than passing" 1 "10d cannot run: the checkout is shallow"

if [ "$failures" -ne 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "All migration-coverage 10d cases passed."
