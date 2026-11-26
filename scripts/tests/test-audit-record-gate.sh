#!/usr/bin/env bash
# Test surface for the adopter CI template's step "Audit-record gate on done
# specs" (framework/templates/ci/adopter-generators.yml; spec 085, AC1, AC4
# and AC5). The step runs `ductus check-audit-records`, turns each finding
# line into an annotation on its file from the repository root, and exits with
# the check's status. Nothing else here runs the template, so a step that
# stopped failing, or annotated the wrong file, would pass everywhere else.
#
# Coverage, from a project at the repository's root and from one in a
# subdirectory, whose job sets its working directory:
#   A. the step runs, never skipped, and passes on a clean corpus, reporting
#      what it examined and each exempt count with its mark
#   B. each finding fails the step, annotated on its file with the project's
#      path from the repository root: a spec directory linked out of the
#      project, a spec.md whose status cannot be read, a record that does not
#      load, a null last-run, a record with no blocking field, blocking: true,
#      a malformed, unreadable or outward-linked baseline, and an exempt set
#      over its mark
#   C. a corpus with no spec fails the step, saying nothing was examined
#   D. a spec root the check cannot list fails the step with an error of its
#      own, since the check names no file
#   E. a `%` in a message is escaped, so GitHub does not decode it
#
# The template's steps run through scripts/run-workflow-steps.py from a copy
# in each fixture, so the fixture is the repository it runs in, with this
# tree's release build first on PATH. The copy leaves out the install step,
# which downloads the pinned release; that release predates the primitive,
# and its pin moves in the release that ships it.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BIN="$REPO_ROOT/runtime/target/release/ductus"
TEMPLATE="$REPO_ROOT/framework/templates/ci/adopter-generators.yml"
STEP="Audit-record gate on done specs"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

failures=0
pass() { echo "  PASS  $1"; }
fail() { echo "  FAIL  $1"; failures=$((failures + 1)); }

if [ ! -x "$BIN" ]; then
  echo "test-audit-record-gate: no release build at $BIN — build runtime/ first; a test that cannot run is not a pass" >&2
  exit 1
fi
mkdir -p "$WORK/bin"
ln -s "$BIN" "$WORK/bin/ductus"
printf 'review-exempt: 5\n' > "$WORK/outside-baseline"
mkdir -p "$WORK/outside-spec"
printf -- '---\nstatus: done\ndependencies: []\n---\n\n# Outside\n' > "$WORK/outside-spec/spec.md"

g() { GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1 git -c user.email=t@example.invalid -c user.name=t "$@"; }

review() {
  printf -- '---\nspec: 001-a\nlast-run: 2026-11-25T00:00:00Z\nmust-violations: 0\nshould-violations: 0\nlow-confidence: 0\nblocking: false\n---\n\n# Review\n'
}

analysis() {
  printf -- '---\nspec: 001-a\nlast-run: 2026-11-25T00:00:00Z\nhard-fail: 0\nblocking-findings: 0\nadvisory: 0\nunexamined: 0\nblocking: false\n---\n\n# Analysis\n'
}

# fixture NAME LAYOUT — a repository holding the template as its workflow,
# its install step dropped and, for LAYOUT `sub`, the job's working directory
# set to `proj`, the runner, and a project at the root or in `proj/` with one
# `done` spec carrying both records, clean. Sets repo and proj; commits
# nothing, so a case edits the project first.
fixture() {
  repo="$WORK/$1"
  if [ "$2" = sub ]; then proj="$repo/proj"; else proj="$repo"; fi
  mkdir -p "$repo/.github/workflows" "$repo/scripts" "$proj/.ductus" "$proj/specs/001-a"
  cp "$REPO_ROOT/scripts/run-workflow-steps.py" "$repo/scripts/"
  python3 -I - "$TEMPLATE" "$repo/.github/workflows/ductus-generators.yml" "$2" <<'PY'
import re, sys
text = open(sys.argv[1], encoding="utf-8").read()
# Drop the install step: from its name to the next step.
text, n = re.subn(r"\n      - name: Install the ductus runtime\n.*?(?=\n      - name: )", "", text, flags=re.S)
assert n == 1, "the template's install step was not found"
if sys.argv[3] == "sub":
    commented = "    # defaults:\n    #   run:\n    #     working-directory: path/to/project\n"
    assert text.count(commented) == 1, "the template's working-directory comment was not found"
    text = text.replace(commented, "    defaults:\n      run:\n        working-directory: proj\n")
open(sys.argv[2], "w", encoding="utf-8").write(text)
PY
  printf '[paths]\nspecs-root = "specs"\n' > "$proj/.ductus/config.toml"
  printf -- '---\nstatus: done\ndependencies: []\n---\n\n# 001 — A\n' > "$proj/specs/001-a/spec.md"
  review > "$proj/specs/001-a/review.md"
  analysis > "$proj/specs/001-a/analysis.md"
  g -C "$repo" init -q
}

# run_gate — commit the fixture and run its workflow's steps; sets out to the
# runner's report and logs to its log directory.
run_gate() {
  g -C "$repo" add -A
  g -C "$repo" commit -qm "fixture"
  logs="$WORK/logs-$(basename "$repo")"
  set +e
  out="$(cd "$repo" && PATH="$WORK/bin:$PATH" python3 -I scripts/run-workflow-steps.py --log-dir "$logs" \
    .github/workflows/ductus-generators.yml 2>&1)"
  set -e
}

# verdict NAME VERDICT — the last run recorded the gate step as VERDICT, so it
# ran: a skipped step is neither.
verdict() {
  if printf '%s\n' "$out" | grep -qE "^SKIP ductus-generators\.yml :: $STEP( |$)"; then
    fail "$1: the step was skipped: $out"
  elif printf '%s\n' "$out" | grep -qE "^$2 ductus-generators\.yml :: $STEP( |$)"; then
    pass "$1"
  else
    fail "$1: want $2 for '$STEP': $out"
  fi
}

# logged NAME TEXT — some step log of the last run holds the line TEXT exactly.
logged() {
  if grep -qxF -- "$2" "$logs"/*.log 2> /dev/null; then pass "$1"
  else fail "$1: no log line is: $2"; fi
}

# annotated NAME FILE — the last run annotated FILE, from the repository root,
# as an audit-record finding.
annotated() {
  if grep -qE -- "^::error file=$(printf '%s' "$2" | sed 's/[.[\*^$]/\\&/g')::Audit-record gate: " "$logs"/*.log 2> /dev/null; then
    pass "$1"
  else
    fail "$1: no annotation on $2"
  fi
}

echo "Running audit-record gate tests..."

for layout in root sub; do
  if [ "$layout" = sub ]; then prefix="proj/"; else prefix=""; fi

  fixture "a-$layout" "$layout"
  run_gate
  verdict "A ($layout): the step passes on a clean corpus" PASS
  logged "A ($layout): it reports what it examined" \
    "check-audit-records: examined 1 spec(s) under 'specs'; 1 at done; 0 skipped"
  logged "A ($layout): and each exempt count with its mark" "check-audit-records:   review-exempt: 0 (baseline 0)"
  logged "A ($layout): both of them" "check-audit-records:   analyze-exempt: 0 (baseline 0)"

  # case NAME FILE EDIT — a clean fixture, EDIT run in its project directory,
  # fails the step with an annotation on FILE.
  case_of() {
    fixture "$1-$layout" "$layout"
    (cd "$proj" && eval "$3")
    run_gate
    verdict "B ($layout): $1 fails the step" FAIL
    annotated "B ($layout): $1 is annotated on $prefix$2" "$prefix$2"
  }

  case_of outside-project specs/002-out "ln -s '$WORK/outside-spec' specs/002-out"
  case_of spec-unreadable specs/002-b/spec.md "mkdir -p specs/002-b && printf '# no frontmatter\n' > specs/002-b/spec.md"
  case_of record-unreadable specs/001-a/review.md "printf '# no frontmatter\n' > specs/001-a/review.md"
  case_of no-last-run specs/001-a/review.md "review | sed 's/^last-run: .*/last-run: null/' > specs/001-a/review.md"
  case_of no-blocking specs/001-a/analysis.md "analysis | grep -v '^blocking:' > specs/001-a/analysis.md"
  case_of blocking specs/001-a/analysis.md "analysis | sed 's/^blocking: false/blocking: true/' > specs/001-a/analysis.md"
  case_of baseline-malformed .ductus/audit-record-baseline.txt "printf 'review-exempt: 50%%\n' > .ductus/audit-record-baseline.txt"
  if grep -qF -- "review-exempt is '50%25', not a count" "$logs"/*.log 2> /dev/null; then
    pass "E ($layout): a % in a message is escaped"
  else
    fail "E ($layout): a % in a message is escaped: no escaped value in $logs"
  fi
  case_of baseline-unreadable .ductus/audit-record-baseline.txt "printf '\377\376' > .ductus/audit-record-baseline.txt"
  case_of baseline-outside-project .ductus/audit-record-baseline.txt "ln -s '$WORK/outside-baseline' .ductus/audit-record-baseline.txt"
  case_of exempt-over-mark specs/001-a/spec.md "rm specs/001-a/analysis.md"

  fixture "c-$layout" "$layout"
  rm -r "$proj/specs/001-a"
  printf '# nothing\n' > "$proj/specs/README.md"
  run_gate
  verdict "C ($layout): a corpus with no spec fails the step" FAIL
  logged "C ($layout): saying nothing was examined" \
    "::error::Audit-record gate: nothing examined: no spec under 'specs' was read, which is not the same as passing"

  fixture "d-$layout" "$layout"
  rm -r "$proj/specs"
  run_gate
  verdict "D ($layout): a spec root that cannot be listed fails the step" FAIL
  logged "D ($layout): with an error of its own" \
    "::error::Audit-record gate: ductus check-audit-records failed (exit 1) — see its error above"
done

if [ "$failures" -gt 0 ]; then
  echo "$failures audit-record gate case(s) failed." >&2
  exit 1
fi
echo "All audit-record gate cases passed."
