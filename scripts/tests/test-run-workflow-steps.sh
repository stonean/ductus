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
#   B+. the run goes on past a FAIL, and a step runs under -e and pipefail
#   H. env: reaches a step from the workflow, job and step levels (later
#      winning), and an expression in any of them reads SKIP
#   I. a step is bound by its own timeout-minutes, else its job's, the whole
#      process group is killed, and a malformed bound exits 2
#   K. a working directory outside the repository exits 2 before any step
#      runs: a relative escape, a job's absolute path, a sibling whose name
#      has the root's as a prefix, and a link under the root that points out
#      of it
#
# Usage: scripts/tests/test-run-workflow-steps.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
# Physical, as the runner resolves a working directory's symbolic links: a
# checkout reached through one (macOS's /var under $TMPDIR) would fail F.
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd -P)"
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
      - name: A step after the failure
        run: true
      - name: A multi-line step whose first command fails
        run: |
          false
          true
      - name: A pipeline whose first command fails
        run: false | true
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
  && has b "PASS failing.yml :: A step after the failure" \
  && has b "FAIL failing.yml :: A multi-line step whose first command fails" \
  && has b "FAIL failing.yml :: A pipeline whose first command fails" \
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

# H
cat > "$WORK/env.yml" <<EOF
on: push
env:
  WF_VAL: from-workflow
  OVERRIDDEN: workflow
jobs:
  check:
    runs-on: ubuntu-latest
    env:
      JOB_VAL: from-job
      OVERRIDDEN: job
    steps:
      - name: Literal values at every level
        env:
          STEP_VAL: "from-step"
        run: |
          printf '%s|%s|%s|%s' "\$WF_VAL" "\$JOB_VAL" "\$STEP_VAL" "\$OVERRIDDEN" > "$WORK/env-got"
      - name: Step expression
        env:
          SECRET: \${{ secrets.TOKEN }}
        run: touch "$WORK/env-step-expr-ran"
      - name: Ungated step
        run: true
  second:
    runs-on: ubuntu-latest
    env:
      JOB_SECRET: \${{ secrets.TOKEN }}
    steps:
      - name: Job expression
        run: touch "$WORK/env-job-expr-ran"
EOF
run h "$WORK/env.yml"
if [ "$rc" -eq 0 ] && [ "$(cat "$WORK/env-got" 2>/dev/null)" = "from-workflow|from-job|from-step|job" ] \
  && has h "SKIP env.yml :: Step expression — its env: holds a \${{ }} expression" \
  && has h "SKIP env.yml :: Job expression — its env: holds a \${{ }} expression" \
  && [ ! -e "$WORK/env-step-expr-ran" ] && [ ! -e "$WORK/env-job-expr-ran" ]; then
  pass "H. env: reaches a step at each level, and an expression in it reads SKIP"
else
  fail "H. env (exit $rc, got $(cat "$WORK/env-got" 2>/dev/null)): $(cat "$WORK/h.out")"
fi

# I
cat > "$WORK/timeout.yml" <<EOF
on: push
jobs:
  stepbound:
    runs-on: ubuntu-latest
    timeout-minutes: 60
    steps:
      - name: Step with its own bound
        timeout-minutes: 0.02
        run: |
          sleep 300 &
          echo \$! > "$WORK/grandchild-pid"
          sleep 30
  jobbound:
    runs-on: ubuntu-latest
    timeout-minutes: 0.02
    steps:
      - name: Step under the job's bound
        run: sleep 30
  quick:
    runs-on: ubuntu-latest
    timeout-minutes: 5
    steps:
      - name: Step inside its bound
        run: true
EOF
start=$SECONDS
run i "$WORK/timeout.yml"
elapsed=$((SECONDS - start))
gpid=$(cat "$WORK/grandchild-pid" 2>/dev/null || echo "")
gone=no
if [ -n "$gpid" ]; then
  # Give the killed group a moment to be reaped.
  for _ in 1 2 3 4 5 6 7 8 9 10; do
    kill -0 "$gpid" 2>/dev/null || { gone=yes; break; }
    sleep 0.3
  done
fi
if [ "$rc" -eq 1 ] && [ "$elapsed" -lt 25 ] && [ "$gone" = yes ] \
  && has i "FAIL timeout.yml :: Step with its own bound — timed out after 0.02 minutes" \
  && has i "FAIL timeout.yml :: Step under the job's bound — timed out after 0.02 minutes" \
  && has i "PASS timeout.yml :: Step inside its bound" \
  && has i "summary: PASS 1, FAIL 2, SKIP 0"; then
  pass "I. a step is killed, group and all, at its own or its job's bound"
else
  if [ -n "$gpid" ] && kill -0 "$gpid" 2>/dev/null; then
    kill "$gpid"
  fi
  fail "I. timeout (exit $rc, ${elapsed}s, grandchild gone: $gone): $(cat "$WORK/i.out")"
fi

cat > "$WORK/badtimeout.yml" <<EOF
on: push
jobs:
  check:
    runs-on: ubuntu-latest
    timeout-minutes: soon
    steps:
      - name: A step
        run: touch "$WORK/badtimeout-ran"
EOF
run j "$WORK/badtimeout.yml"
if [ "$rc" -eq 2 ] && has j "badtimeout.yml" && has j "'soon'" && [ ! -e "$WORK/badtimeout-ran" ]; then
  pass "I. a malformed timeout-minutes exits 2, naming the workflow and value"
else
  fail "I. malformed timeout (exit $rc): $(cat "$WORK/j.out")"
fi

# K
cat > "$WORK/escape.yml" <<EOF
on: push
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - name: A step inside the repository
        run: touch "$WORK/escape-inside-ran"
      - name: A step above the repository
        working-directory: ..
        run: touch "$WORK/escape-ran"
EOF
run k1 "$WORK/escape.yml"
k1=$rc
cat > "$WORK/escape-abs.yml" <<EOF
on: push
jobs:
  check:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: $WORK
    steps:
      - name: A step in a job outside the repository
        run: touch "$WORK/escape-abs-ran"
EOF
run k2 "$WORK/escape-abs.yml"
k2=$rc
sibling="../$(basename "$REPO_ROOT")2"
cat > "$WORK/escape-sibling.yml" <<EOF
on: push
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - name: A step in a sibling sharing the root's name
        working-directory: $sibling
        run: touch "$WORK/escape-sibling-ran"
EOF
run k3 "$WORK/escape-sibling.yml"
k3=$rc
# A link under the root that points out of it: a lexical check passes it, so
# a copy of the runner runs under a fake root holding one (the runner takes
# its root from its own location).
fake="$WORK/fakeroot"
mkdir -p "$fake/scripts" "$WORK/link-target"
cp "$RUNNER" "$fake/scripts/run-workflow-steps.py"
ln -s "$WORK/link-target" "$fake/linkdir"
cat > "$WORK/escape-link.yml" <<EOF
on: push
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - name: A step in a directory that is a link out of the repository
        working-directory: linkdir
        run: touch "$WORK/escape-link-ran"
EOF
k4=0
python3 -I "$fake/scripts/run-workflow-steps.py" --log-dir "$WORK/logs-k4" "$WORK/escape-link.yml" > "$WORK/k4.out" 2>&1 || k4=$?
if [ "$k1" -eq 2 ] && has k1 "working-directory '..' resolves outside the repository" \
  && [ "$k2" -eq 2 ] && has k2 "resolves outside the repository" \
  && [ "$k3" -eq 2 ] && has k3 "working-directory '$sibling' resolves outside the repository" \
  && [ ! -e "$WORK/escape-inside-ran" ] && [ ! -e "$WORK/escape-ran" ] \
  && [ ! -e "$WORK/escape-abs-ran" ] && [ ! -e "$WORK/escape-sibling-ran" ] \
  && [ "$k4" -eq 2 ] && has k4 "working-directory 'linkdir' resolves outside the repository" \
  && [ ! -e "$WORK/escape-link-ran" ]; then
  pass "K. a working directory outside the repository exits 2 before any step runs"
else
  fail "K. containment (exits $k1, $k2, $k3, $k4): $(cat "$WORK/k1.out" "$WORK/k2.out" "$WORK/k3.out" "$WORK/k4.out")"
fi

if [ "$failures" -gt 0 ]; then
  echo "$failures case(s) failed" >&2
  exit 1
fi
echo "all cases passed"
