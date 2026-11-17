#!/usr/bin/env bash
# Test surface for scripts/check-npm-audit.py (spec 022 task 51).
#
# The checker is what holds framework-checks.yml and runtime-currency.yml to
# BE-DEPS-001 for the npm lint tooling, so the evidence it works is that it
# FAILS on each condition it claims to catch, and that a report it cannot read
# never reads as one that passed (QUAL-CLAIM-001). Each case feeds it an
# `npm audit --json` report shaped as npm 10 prints one.
#
# Coverage:
#   A. a listed high advisory, with only a breaking fix, passes
#   B. the same advisory unlisted fails, naming it
#   C. a listed one fails once npm reports a fix that is not breaking (true)
#   D. ... or a non-breaking fix object
#   E. a listed one fails once its affected range changes
#   F. an entry whose advisory is no longer reported fails (stale)
#   G. a low advisory needs no entry
#   H. an unlisted critical advisory fails
#   I. a report that is not JSON is exit 2, never a pass
#   J. an entry with no reason is exit 2
#   K. a gating package tracing to no gating advisory is exit 2
#
# CHECKER, when set, names the checker to run, so a mutant can be run against
# these cases.
#
# Usage: scripts/tests/test-check-npm-audit.sh

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
CHECKER="${CHECKER:-$REPO_ROOT/scripts/check-npm-audit.py}"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# report NAME SEVERITY RANGE FIX: a report with one advisory on picomatch,
# which micromatch inherits, its fixAvailable set to FIX (JSON).
report() {
  cat > "$WORK/$1.json" <<EOF
{"auditReportVersion": 2, "vulnerabilities": {
  "picomatch": {"name": "picomatch", "severity": "$2", "isDirect": false,
    "via": [{"source": 1, "name": "picomatch", "title": "ReDoS", "severity": "$2",
      "url": "https://github.com/advisories/GHSA-fc5q-39xc-vjpf", "range": "$3"}],
    "range": "*", "fixAvailable": $4},
  "micromatch": {"name": "micromatch", "severity": "$2", "isDirect": false,
    "via": ["picomatch"], "range": ">=4.0.0", "fixAvailable": $4}
}, "metadata": {}}
EOF
}

# allowed NAME [REASON]: a list naming the picomatch advisory at <=4.0.7.
allowed() {
  reason="${2-globs only the patterns this repository writes}"
  cat > "$WORK/$1.allowed.json" <<EOF
{"allowed": [{"advisory": "GHSA-fc5q-39xc-vjpf", "package": "picomatch",
  "range": "<=4.0.7", "reason": "$reason"}]}
EOF
}

printf '{"allowed": []}\n' > "$WORK/none.allowed.json"
BREAKING='{"name": "markdownlint-cli2", "version": "0.0.4", "isSemVerMajor": true}'

# run CASE EXPECTED REPORT ALLOWED [TEXT]: the checker exits EXPECTED and, when
# TEXT is given, prints it.
run() {
  out="$(python3 -I "$CHECKER" --report "$WORK/$3.json" --allowed "$WORK/$4.allowed.json" 2>&1)"
  got=$?
  if [ "$got" -ne "$2" ]; then
    fail "$1: exit $got, expected $2: $out"
  elif [ -n "${5-}" ] && ! printf '%s' "$out" | grep -qF -- "$5"; then
    fail "$1: output lacks '$5': $out"
  else
    pass "$1"
  fi
}

report high high '<=4.0.7' "$BREAKING"
allowed listed
run "A listed advisory with only a breaking fix passes" 0 high listed "1 listed"
run "B an unlisted high advisory fails" 1 high none "GHSA-fc5q-39xc-vjpf (high"

report fixtrue high '<=4.0.7' true
run "C a listed advisory with a fix fails" 1 fixtrue listed "apply it"
report fixminor high '<=4.0.7' '{"name": "micromatch", "version": "4.0.9", "isSemVerMajor": false}'
run "D a listed advisory with a non-breaking fix fails" 1 fixminor listed "apply it"

report moved high '<4.0.9' "$BREAKING"
run "E a listed advisory whose range changed fails" 1 moved listed "judge the entry again"

printf '{"auditReportVersion": 2, "vulnerabilities": {}, "metadata": {}}\n' > "$WORK/clean.json"
run "F a stale entry fails" 1 clean listed "no longer reported"

report low low '<=4.0.7' "$BREAKING"
run "G a low advisory needs no entry" 0 low none "0 high or critical"

report critical critical '<=4.0.7' "$BREAKING"
run "H an unlisted critical advisory fails" 1 critical none "(critical"

printf 'npm ERR! not json\n' > "$WORK/broken.json"
run "I an unreadable report is exit 2" 2 broken listed "nothing was judged"

allowed noreason ""
run "J an entry with no reason is exit 2" 2 high noreason "no \`reason\`"

cat > "$WORK/orphan.json" <<'EOF'
{"auditReportVersion": 2, "vulnerabilities": {
  "micromatch": {"name": "micromatch", "severity": "high", "isDirect": false,
    "via": ["picomatch"], "range": ">=4.0.0", "fixAvailable": false}
}, "metadata": {}}
EOF
run "K a gating package with no gating advisory is exit 2" 2 orphan none "no gating advisory"

if [ "$failures" -ne 0 ]; then
  printf '%s case(s) failed\n' "$failures" >&2
  exit 1
fi
printf 'test-check-npm-audit: 11 case(s) passed\n'
