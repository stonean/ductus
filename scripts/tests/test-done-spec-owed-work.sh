#!/usr/bin/env bash
# Test surface for a `done` spec owing work and for code no review is going to
# read (spec 084, AC9's CI half and AC15): /audit Family 27
# (scripts/audit/done-spec-criteria.sh) and the adopter CI template's two steps
# (framework/templates/ci/adopter-generators.yml), "Done specs owe no work"
# and "No code is left unreviewed". Step (h) runs Family 27 over this
# repository, where no `done` spec owes work, and nothing here runs the
# template, so a check that stopped failing would pass there.
#
# Coverage, each condition present and absent:
#   A. Family 27 passes on a `done` spec owing nothing
#   B. Family 27 reports a `done` spec's unticked criterion
#   C. Family 27 reports a `done` spec's unchecked box in tasks.md
#   D. both template steps pass on a `done` spec owing nothing, no code after
#      the project record's seed
#   E. the owed-work step fails on an unchecked box, naming it
#   F. the owed-work step fails on an unticked criterion, naming it
#   G. the unreviewed-code step fails on code no spec claims under "block"
#   H. the same passes under "warn", with a warning naming the path
#
# The template's steps run through scripts/run-workflow-steps.py from a copy
# in each fixture, so the fixture is the repository it runs in, with this
# tree's release build first on PATH. The copied template leaves out its
# install step, which downloads the pinned release; that release predates the
# two primitives, and its pin moves in the release commit that ships them.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BIN="$REPO_ROOT/runtime/target/release/ductus"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

failures=0
pass() { echo "  PASS  $1"; }
fail() { echo "  FAIL  $1"; failures=$((failures + 1)); }

if [ ! -x "$BIN" ]; then
  echo "test-done-spec-owed-work: no release build at $BIN — build runtime/ first; a test that cannot run is not a pass" >&2
  exit 1
fi
mkdir -p "$WORK/bin"
ln -s "$BIN" "$WORK/bin/ductus"

g() { GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1 git -c user.email=t@example.invalid -c user.name=t "$@"; }

# spec_tree NAME CRITERION BOX — a repository holding one `done` spec whose
# criterion reads `- [CRITERION] AC1: …` and whose one task box reads
# `- [BOX] …`, with Family 27 and the runtime pointer in place, committed.
spec_tree() {
  local dir="$WORK/$1"
  mkdir -p "$dir/scripts/audit" "$dir/.ductus/bin" "$dir/specs/001-a"
  cp "$REPO_ROOT/scripts/audit/lib.sh" "$REPO_ROOT/scripts/audit/done-spec-criteria.sh" "$dir/scripts/audit/"
  ln -s "$BIN" "$dir/.ductus/bin/ductus"
  printf '.ductus/bin/\n' > "$dir/.gitignore"
  printf '[paths]\nspecs-root = "specs"\n' > "$dir/.ductus/config.toml"
  printf -- '---\nstatus: done\ndependencies: []\n---\n\n# 001 — A\n\n## Acceptance Criteria\n\n- [%s] AC1: a holds\n\n## Open Questions\n\n*None.*\n' \
    "$2" > "$dir/specs/001-a/spec.md"
  printf '# Tasks\n\n## 1. Build a\n\n- [%s] write a\n' "$3" > "$dir/specs/001-a/tasks.md"
  g -C "$dir" init -q
  g -C "$dir" add -A
  g -C "$dir" commit -qm "001 as done"
  echo "$dir"
}

# family DIR — run DIR's copy of Family 27; sets out and code.
family() {
  set +e
  out="$(cd "$1" && bash scripts/audit/done-spec-criteria.sh 2> /dev/null)"
  code=$?
  set -e
}

# expect NAME CODE PATTERN — the last run exited CODE, and its output matches
# PATTERN, or is empty when PATTERN is.
expect() {
  if [ "$code" -ne "$2" ]; then fail "$1: exit $code, want $2: $out"; return; fi
  if [ -z "$3" ]; then
    if [ -n "$out" ]; then fail "$1: want no finding: $out"; else pass "$1"; fi
  elif printf '%s' "$out" | grep -qE -- "$3"; then pass "$1"
  else fail "$1: nothing matched /$3/: $out"; fi
}

echo "Running done-spec owed-work tests..."

family "$(spec_tree a x x)"
expect "A: Family 27 passes on a done spec owing nothing" 0 ""
family "$(spec_tree b ' ' x)"
expect "B: Family 27 reports an unticked criterion" 1 'specs/001-a/spec\.md \| spec is status: done but criterion AC1 is unchecked'
family "$(spec_tree c x ' ')"
expect "C: Family 27 reports an unchecked box" 1 'specs/001-a/tasks\.md \| spec is status: done but tasks\.md holds an unchecked box: - \[ \] write a'

# template_tree NAME CRITERION BOX — spec_tree's repository with the project
# record seeded and committed, the template as its workflow and the runner.
template_tree() {
  local dir
  dir="$(spec_tree "$1" "$2" "$3")"
  mkdir -p "$dir/.github/workflows"
  cp "$REPO_ROOT/scripts/run-workflow-steps.py" "$dir/scripts/"
  python3 -I - "$REPO_ROOT/framework/templates/ci/adopter-generators.yml" \
    "$dir/.github/workflows/ductus-generators.yml" <<'PY'
import re, sys
text = open(sys.argv[1], encoding="utf-8").read()
# Drop the install step: from its name to the next step.
text, n = re.subn(r"\n      - name: Install the ductus runtime\n.*?(?=\n      - name: )", "", text, flags=re.S)
assert n == 1, "the template's install step was not found"
open(sys.argv[2], "w", encoding="utf-8").write(text)
PY
  (cd "$dir" && "$BIN" write-review --seed > /dev/null)
  g -C "$dir" add -A
  g -C "$dir" commit -qm "seed the project record"
  echo "$dir"
}

# run_template DIR — run DIR's workflow steps; sets out to the runner's report
# and logs to its log directory.
run_template() {
  logs="$WORK/logs-$(basename "$1")"
  set +e
  out="$(cd "$1" && PATH="$WORK/bin:$PATH" python3 -I scripts/run-workflow-steps.py --log-dir "$logs" \
    .github/workflows/ductus-generators.yml 2>&1)"
  set -e
}

# step NAME VERDICT STEP — the last run_template recorded STEP as VERDICT.
step() {
  if printf '%s\n' "$out" | grep -qE "^$2 ductus-generators\.yml :: $3( |$)"; then pass "$1"
  else fail "$1: want $2 for '$3': $out"; fi
}

# logged NAME PATTERN — some step log of the last run_template matches PATTERN.
logged() {
  if grep -qE -- "$2" "$logs"/*.log 2> /dev/null; then pass "$1"; else fail "$1: no log matched /$2/"; fi
}

dir="$(template_tree d x x)"
run_template "$dir"
step "D: the owed-work step passes on a done spec owing nothing" PASS "Done specs owe no work"
step "D: the unreviewed-code step passes with nothing to review" PASS "No code is left unreviewed"

run_template "$(template_tree e x ' ')"
step "E: the owed-work step fails on an unchecked box" FAIL "Done specs owe no work"
logged "E: it names the box" '::error::done spec 001-a: tasks\.md holds an unchecked box'

run_template "$(template_tree f ' ' x)"
step "F: the owed-work step fails on an unticked criterion" FAIL "Done specs owe no work"
logged "F: it names the criterion" '::error::done spec 001-a: criterion AC1 is unticked'

mkdir -p "$dir/src"
echo 'fn a() {}' > "$dir/src/a.rs"
g -C "$dir" add -A
g -C "$dir" commit -qm "code naming no spec"
run_template "$dir"
step "G: the unreviewed-code step fails on unreviewed code under block" FAIL "No code is left unreviewed"
printf '[paths]\nspecs-root = "specs"\n\n[review]\nunreviewed-push = "warn"\n' > "$dir/.ductus/config.toml"
run_template "$dir"
step "H: the same passes under warn" PASS "No code is left unreviewed"
logged "H: with a warning naming the path" '^::warning::no committed review covers src/a\.rs$'

if [ "$failures" -gt 0 ]; then
  echo "$failures done-spec owed-work case(s) failed." >&2
  exit 1
fi
echo "All done-spec owed-work cases passed."
