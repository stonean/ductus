#!/usr/bin/env bash
# Test surface for scripts/audit/migration-coverage.sh checks 10a, 10d, 10e,
# 10f and 10b's archived half (Family 10).
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
#   I. an entry with no procedure_file is reported, and its target_paths
#      are still examined (10b and 10c's unit-separated rows)
#   J. an entry with no target_paths does not stop 10b for the entries
#      after it, or 10d (bash 3.2's empty array under `set -u`)
#   K. 10e: an entry at or past its sunset_after fails, naming it; one whose
#      sunset_after is still ahead passes, with a coverage line
#   L. 10e: no version file, or one that is no release, fails rather than
#      passing, and the second names the version file and its remedy rather
#      than an archival
#   M. 10b's archived half: an archived entry whose framework/ target path
#      still exists fails, one with no **Target paths:** line fails, and one
#      whose paths are gone passes, with a coverage line
#   N. 10b's archived half: a **Target paths:** line naming no backticked path
#      fails, and a CHANGELOG.md with no Archived migrations section fails
#      rather than reading as an empty archive
#   O. 10f: an id two entries share fails, naming it, though 10a, 10c and 10d
#      each pass it, with a coverage line
#   P. 10e: an entry whose sunset_after is no release fails with a remedy for
#      that value rather than an archival
#   Q. 10a: a framework/migrations/*.md no entry names fails, naming it, while
#      the file an entry names passes
#   R. 10a: an orphan whose path appears only in a TOML comment fails, since
#      10a reads the parsed procedure_file values, not the raw text
#   T. 10a: a registry that parses as TOML but whose entries the check cannot
#      read, after it printed one path, fails with the "parse did not finish"
#      finding rather than using the partial list
#   S. 10f: a registry whose id is no string, which the history and duplicate
#      comparisons cannot read, fails with the "produced no output" finding
#      rather than passing
#
# Every fixture holds a version file (0.59.0), since 10e reads it.
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
  printf '0.59.0\n' > "$dir/version"
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

echo "Running migration-coverage tests..."

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

# 10b and 10c read their rows with a unit separator, since a tab is IFS
# whitespace and `read` collapses consecutive tabs: an entry with no
# procedure_file would otherwise read its target_paths as the procedure_file,
# and 10b would never examine it. 10b also expands an entry's paths array,
# which is empty for an entry with no target_paths; bash 3.2 (macOS /bin/bash)
# fails an empty array under `set -u`, so J needs that shell to fail without
# the fix.
dir="$(make_repo i)"
printf '# There\n' > "$dir/framework/there.md"
cat > "$dir/framework/migrations.toml" << 'TOML'
[[migrations]]
id = "no-proc"
introduced_in = "0.60.0"
summary = "s"
target_paths = ["framework/there.md"]
TOML
commit "$dir"
run_check "$dir"
expect "I: an entry with no procedure_file is reported as missing it" 1 \
  "entry 'no-proc' references missing procedure_file: "
expect "I: that entry's target_paths are still examined" 1 \
  "entry 'no-proc' claims 'framework/there.md' was removed but it still exists"

dir="$(make_repo j)"
printf '# There\n' > "$dir/framework/there.md"
printf '# P1\n' > "$dir/framework/migrations/p1.md"
printf '# P2\n' > "$dir/framework/migrations/p2.md"
cat > "$dir/framework/migrations.toml" << 'TOML'
[[migrations]]
id = "no-targets"
introduced_in = "0.60.0"
summary = "s"
procedure_file = "framework/migrations/p1.md"

[[migrations]]
id = "later"
introduced_in = "0.60.0"
summary = "s"
procedure_file = "framework/migrations/p2.md"
target_paths = ["framework/there.md"]
TOML
commit "$dir"
run_check "$dir"
expect "J: an entry with no target_paths does not stop the entries after it" 1 \
  "entry 'later' claims 'framework/there.md' was removed but it still exists"
if printf '%s' "$err" | grep -q '10d examined: 2 entries'; then
  pass "J: 10d still runs after it"
else
  fail "J: 10d did not run: $err"
fi

# sunset_entry DIR ID VERSION SUNSET — an entry carrying a sunset_after.
sunset_entry() {
  printf '[[migrations]]\nid = "%s"\nintroduced_in = "%s"\nsunset_after = "%s"\nsummary = "s"\ntarget_paths = ["adopter/x"]\nprocedure_file = "framework/migrations/%s.md"\n\n' \
    "$2" "$3" "$4" "$2" >> "$1/framework/migrations.toml"
  printf '# %s\n' "$2" > "$1/framework/migrations/$2.md"
}

dir="$(make_repo k)"
sunset_entry "$dir" overdue 0.2.0 0.10.0
sunset_entry "$dir" at-version 0.50.0 0.59.0
sunset_entry "$dir" ahead 0.58.0 0.61.0
commit "$dir"
run_check "$dir"
expect "K: an entry past its sunset_after fails, naming it" 1 \
  "entry 'overdue' is past its sunset_after 0.10.0 \(version 0.59.0\)"
expect "K: an entry whose sunset_after equals the version fails" 1 \
  "entry 'at-version' is past its sunset_after 0.59.0"
if printf '%s' "$out" | grep -q "'ahead'"; then
  fail "K: an entry whose sunset_after is still ahead was reported: $out"
else
  pass "K: an entry whose sunset_after is still ahead passes"
fi
if printf '%s' "$err" | grep -q '10e examined: 3 entries with a sunset_after compared with version 0.59.0'; then
  pass "K: the coverage line states what 10e compared"
else
  fail "K: 10e coverage line missing or wrong: $err"
fi

dir="$(make_repo l)"
entry "$dir" alpha 0.60.0; commit "$dir"
rm -f "$dir/version"
run_check "$dir"
expect "L: no version file fails rather than passing" 1 "10e cannot run: no version file"
printf 'main\n' > "$dir/version"
run_check "$dir"
expect "L: a version that is no release fails" 1 "version holds 'main', not a MAJOR.MINOR.PATCH release"
expect "L: that finding names the version file and its remedy" 1 \
  "^migration-coverage \| version \| version holds 'main'[^|]*\| write the release"
if printf '%s' "$out" | grep -q 'archive it'; then
  fail "L: a malformed version was given the archival remedy: $out"
else
  pass "L: a malformed version is not given the archival remedy"
fi

dir="$(make_repo m)"
entry "$dir" alpha 0.60.0
mkdir -p "$dir/framework/workflows"
printf '# Still here\n' > "$dir/framework/workflows/kept.md"
cat > "$dir/CHANGELOG.md" << 'MD'
# CHANGELOG

## Archived migrations

### stale-one (introduced 0.2.0, sunset after 0.10.0)

**Summary:** s.

**Target paths:** `adopter/y`, `framework/workflows/kept.md`

### no-paths-line (introduced 0.2.0, sunset after 0.10.0)

**Summary:** s.

### clean-one (introduced 0.3.0, sunset after 0.10.0)

**Summary:** s.

**Target paths:** `framework/skills/`, `framework/gone/*.md`

## Later section

### not-archived (introduced 0.4.0, sunset after 0.10.0)
MD
commit "$dir"
run_check "$dir"
expect "M: an archived framework/ target path that still exists fails" 1 \
  "archived entry 'stale-one' claims 'framework/workflows/kept.md' was removed but it still exists"
expect "M: an archived entry with no Target paths line fails" 1 \
  "archived entry 'no-paths-line' has no \*\*Target paths:\*\* line"
if printf '%s' "$out" | grep -qE "'clean-one'|'not-archived'"; then
  fail "M: a clean archived entry, or a heading outside the section, was reported: $out"
else
  pass "M: an archived entry whose paths are gone passes, and only the section is read"
fi
if printf '%s' "$err" | grep -q '10b examined: 3 archived entries in CHANGELOG.md'; then
  pass "M: the coverage line counts the archived entries"
else
  fail "M: 10b archived coverage line missing or wrong: $err"
fi

dir="$(make_repo n)"
entry "$dir" alpha 0.60.0
cat > "$dir/CHANGELOG.md" << 'MD'
# CHANGELOG

## Archived migrations

### bare-paths (introduced 0.2.0, sunset after 0.10.0)

**Target paths:** framework/workflows/kept.md
MD
commit "$dir"
run_check "$dir"
expect "N: a Target paths line naming no backticked path fails" 1 \
  "archived entry 'bare-paths' has no \*\*Target paths:\*\* line naming a backticked path"
printf '# CHANGELOG\n\n## Retired migrations\n\n### moved (introduced 0.2.0, sunset after 0.10.0)\n\n**Target paths:** `framework/x/`\n' > "$dir/CHANGELOG.md"
run_check "$dir"
expect "N: a CHANGELOG.md with no Archived migrations section fails" 1 \
  "CHANGELOG.md holds no '## Archived migrations' section"

# 10f: Pre-run Migrations' guard aborts on an id two entries share, but the
# other checks read the registry by procedure file or by id and pass it: both
# entries name one existing procedure file (10a, 10c), and 10d keys entries by
# id, so the second replaces the first (spec 027 AC25).
dir="$(make_repo o)"
entry "$dir" alpha 0.60.0
printf '[[migrations]]\nid = "alpha"\nintroduced_in = "0.60.1"\nsummary = "s"\ntarget_paths = ["adopter/x"]\nprocedure_file = "framework/migrations/alpha.md"\n\n' \
  >> "$dir/framework/migrations.toml"
entry "$dir" beta 0.60.0
commit "$dir"
run_check "$dir"
expect "O: an id two entries share fails, naming it" 1 \
  "^migration-coverage \| framework/migrations.toml \| \[\[migrations\]\] id 'alpha' names 2 entries"
if printf '%s' "$out" | grep -q "'beta'"; then
  fail "O: an id one entry holds was reported: $out"
else
  pass "O: an id one entry holds passes"
fi
if printf '%s' "$err" | grep -q '10f examined: 3 entries, 2 distinct ids'; then
  pass "O: the coverage line counts entries and distinct ids"
else
  fail "O: 10f coverage line missing or wrong: $err"
fi

dir="$(make_repo p)"
sunset_entry "$dir" bad-sunset 0.58.0 soon
commit "$dir"
run_check "$dir"
expect "P: a sunset_after that is no release fails, naming the entry" 1 \
  "entry 'bad-sunset' has sunset_after 'soon', not a MAJOR.MINOR.PATCH release[^|]*\| set the entry's sunset_after"
if printf '%s' "$out" | grep -q 'archive it'; then
  fail "P: a malformed sunset_after was given the archival remedy: $out"
else
  pass "P: a malformed sunset_after is not given the archival remedy"
fi

dir="$(make_repo q)"
entry "$dir" alpha 0.60.0
printf '# orphan\n' > "$dir/framework/migrations/orphan.md"
commit "$dir"
run_check "$dir"
expect "Q: a procedure file no entry names fails, naming it" 1 \
  "^migration-coverage \| framework/migrations/orphan.md \| orphan procedure file"
if printf '%s' "$out" | grep -q 'alpha.md'; then
  fail "Q: the procedure file an entry names was reported: $out"
else
  pass "Q: the procedure file an entry names passes"
fi

dir="$(make_repo r)"
entry "$dir" alpha 0.60.0
printf '# procedure_file = "framework/migrations/orphan.md"\n' >> "$dir/framework/migrations.toml"
printf '# orphan\n' > "$dir/framework/migrations/orphan.md"
commit "$dir"
run_check "$dir"
expect "R: an orphan whose path appears only in a TOML comment fails" 1 \
  "^migration-coverage \| framework/migrations/orphan.md \| orphan procedure file"

dir="$(make_repo s)"
printf '[[migrations]]\nid = ["a"]\nintroduced_in = "0.60.0"\nsummary = "s"\ntarget_paths = ["adopter/x"]\nprocedure_file = "framework/migrations/a.md"\n' \
  > "$dir/framework/migrations.toml"
printf '# a\n' > "$dir/framework/migrations/a.md"
commit "$dir"
run_check "$dir"
expect "S: an id that is no string fails with 10f's produced-no-output finding" 1 \
  "10f produced no output: the duplicate-id comparison could not run"

dir="$(make_repo t)"
printf 'migrations = [{ id = "a", introduced_in = "0.60.0", procedure_file = "framework/migrations/a.md" }, 1]\n' \
  > "$dir/framework/migrations.toml"
printf '# a\n' > "$dir/framework/migrations/a.md"
printf '# orphan\n' > "$dir/framework/migrations/orphan.md"
commit "$dir"
run_check "$dir"
expect "T: a parse that prints, then raises, fails rather than using the partial list" 1 \
  "10a's parse did not finish: the registered procedure files could not be read"
if printf '%s' "$out" | grep -q 'orphan procedure file'; then
  fail "T: the partial list was used as complete: $out"
else
  pass "T: no orphan is reported from a partial list"
fi

if [ "$failures" -ne 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "All migration-coverage cases passed."
