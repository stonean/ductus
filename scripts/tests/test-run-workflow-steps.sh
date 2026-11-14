#!/usr/bin/env bash
# Test surface for scripts/run-workflow-steps.py (spec 079, task 38).
#
# The runner stands in for a workflow run, so the only evidence it works is
# that each state it reports comes from the step itself: a failing step reads
# FAIL and fails the run, a step it cannot run reads SKIP and is never run, and
# a workflow it found no step in fails the run rather than passing over
# nothing.
#
# Coverage:
#   A. a passing step reads PASS, with each workflow's counts, and exits 0
#   B. a failing step reads FAIL beside a passing one, and the run exits 1
#   C. a step carrying an `if:` reads SKIP by name and is not run
#   D. a workflow with no `run:` step fails the run, naming it
#   E. a step whose script holds a `${{ }}` expression reads SKIP and is not run
#   F. a step runs from its job's `working-directory`, and a step's own wins
#   G. a workflow file that cannot be read exits 2
#
# Usage: scripts/tests/test-run-workflow-steps.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RUNNER="$REPO_ROOT/scripts/run-workflow-steps.py"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# Run the runner over the fixture workflows named, capturing output and exit.
run() {
  name=$1
  shift
  rc=0
  python3 -I "$RUNNER" --log-dir "$WORK/logs-$name" "$@" > "$WORK/$name.out" 2>&1 || rc=$?
}

has() { grep -qF -- "$2" "$WORK/$1.out"; }

cat > "$WORK/passing.yml" <<EOF
on: push
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: A step that passes
        run: |
          touch "$WORK/passing-ran"
EOF

cat > "$WORK/failing.yml" <<EOF
on: push
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - name: A step that passes
        run: true
      - name: A step that fails
        run: |
          echo "the failure's own output"
          exit 3
EOF

cat > "$WORK/gated.yml" <<EOF
on: push
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - name: A gated step
        if: steps.cache.outputs.cache-hit == 'true'
        run: touch "$WORK/gated-ran"
      - name: An ungated step
        run: true
EOF

cat > "$WORK/empty.yml" <<'EOF'
on: push
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
EOF

cat > "$WORK/expression.yml" <<EOF
on: push
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - name: A step reading a matrix value
        run: |
          touch "$WORK/expression-ran"
          echo "\${{ matrix.os }}"
      - name: An ungated step
        run: true
EOF

mkdir -p "$WORK/tree"
cat > "$WORK/workdir.yml" <<EOF
on: push
jobs:
  check:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: scripts
    steps:
      - name: From the job's directory
        run: pwd > "$WORK/job-dir"
      - name: From the step's own
        working-directory: runtime
        run: pwd > "$WORK/step-dir"
EOF

# A
run a "$WORK/passing.yml"
if [ "$rc" -eq 0 ] && has a "PASS passing.yml :: A step that passes" \
  && has a "passing.yml: 1 run step(s) found, 1 run, 0 skipped" && [ -e "$WORK/passing-ran" ]; then
  pass "A. a passing step reads PASS with its workflow's counts, exit 0"
else
  fail "A. passing step (exit $rc): $(cat "$WORK/a.out")"
fi

# B
run b "$WORK/failing.yml"
log="$WORK/logs-b/failing.yml-02.log"
if [ "$rc" -eq 1 ] && has b "PASS failing.yml :: A step that passes" \
  && has b "FAIL failing.yml :: A step that fails — exit 3" \
  && grep -qF "the failure's own output" "$log" 2>/dev/null; then
  pass "B. a failing step reads FAIL from its own status, and the run exits 1"
else
  fail "B. failing step (exit $rc): $(cat "$WORK/b.out")"
fi

# C
run c "$WORK/gated.yml"
if [ "$rc" -eq 0 ] && has c "SKIP gated.yml :: A gated step — if: steps.cache.outputs.cache-hit == 'true'" \
  && [ ! -e "$WORK/gated-ran" ] && has c "gated.yml: 2 run step(s) found, 1 run, 1 skipped"; then
  pass "C. an if: step reads SKIP by name and is not run"
else
  fail "C. gated step (exit $rc): $(cat "$WORK/c.out")"
fi

# D
run d "$WORK/passing.yml" "$WORK/empty.yml"
if [ "$rc" -eq 1 ] && has d "empty.yml: no run step found, so nothing was checked" \
  && has d "PASS passing.yml :: A step that passes"; then
  pass "D. a workflow with no run step fails the run"
else
  fail "D. empty workflow (exit $rc): $(cat "$WORK/d.out")"
fi

# E
run e "$WORK/expression.yml"
if [ "$rc" -eq 0 ] && has e "SKIP expression.yml :: A step reading a matrix value" \
  && [ ! -e "$WORK/expression-ran" ]; then
  pass "E. a step holding a \${{ }} expression reads SKIP and is not run"
else
  fail "E. expression step (exit $rc): $(cat "$WORK/e.out")"
fi

# F
run f "$WORK/workdir.yml"
if [ "$rc" -eq 0 ] && [ "$(cat "$WORK/job-dir" 2>/dev/null)" = "$REPO_ROOT/scripts" ] \
  && [ "$(cat "$WORK/step-dir" 2>/dev/null)" = "$REPO_ROOT/runtime" ]; then
  pass "F. a step runs from its job's working-directory, and its own wins"
else
  fail "F. working directory (exit $rc): job $(cat "$WORK/job-dir" 2>/dev/null), step $(cat "$WORK/step-dir" 2>/dev/null)"
fi

# G
run g "$WORK/absent.yml"
if [ "$rc" -eq 2 ] && has g "cannot read"; then
  pass "G. an unreadable workflow exits 2"
else
  fail "G. unreadable workflow (exit $rc): $(cat "$WORK/g.out")"
fi

if [ "$failures" -gt 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "all cases passed"
