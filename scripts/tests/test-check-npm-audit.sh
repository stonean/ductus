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
#   L. npm's own error object, with no vulnerabilities map, is exit 2
#   M. a listed advisory whose package changed fails
#   N. a listed advisory npm now reports below high fails (stale)
#   O. a vulnerability whose via is null is exit 2, never a traceback
#   P. beside a listed advisory, a gating package that reaches no gating
#      advisory is exit 2
#   Q. one advisory on two packages fails when either has a non-breaking fix
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

passed=0

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
    passed=$((passed + 1))
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
run "K a gating package with no gating advisory is exit 2" 2 orphan none "reaches no high or critical advisory"

printf '{"error": {"code": "ENOLOCK", "summary": "no lockfile"}}\n' > "$WORK/error.json"
run "L npm's error object is exit 2" 2 error listed "no \`vulnerabilities\` map"

cat > "$WORK/othername.allowed.json" <<'EOF'
{"allowed": [{"advisory": "GHSA-fc5q-39xc-vjpf", "package": "micromatch",
  "range": "<=4.0.7", "reason": "listed under the wrong package"}]}
EOF
run "M a listed advisory whose package changed fails" 1 high othername "judge the entry again"

report moderate moderate '<=4.0.7' "$BREAKING"
run "N a listed advisory now below high fails" 1 moderate listed "no longer high or critical"

cat > "$WORK/vianull.json" <<'EOF'
{"auditReportVersion": 2, "vulnerabilities": {
  "picomatch": {"name": "picomatch", "severity": "high", "via": null, "fixAvailable": false}
}, "metadata": {}}
EOF
run "O a null via is exit 2" 2 vianull listed "has no \`via\` list"

cat > "$WORK/ghost.json" <<EOF
{"auditReportVersion": 2, "vulnerabilities": {
  "picomatch": {"name": "picomatch", "severity": "high", "isDirect": false,
    "via": [{"source": 1, "name": "picomatch", "title": "ReDoS", "severity": "high",
      "url": "https://github.com/advisories/GHSA-fc5q-39xc-vjpf", "range": "<=4.0.7"}],
    "range": "*", "fixAvailable": $BREAKING},
  "other": {"name": "other", "severity": "critical", "isDirect": true,
    "via": ["ghost"], "range": "*", "fixAvailable": false}
}, "metadata": {}}
EOF
run "P a gating package reaching no advisory, beside a listed one, is exit 2" 2 ghost listed "reaches no high or critical advisory"

cat > "$WORK/twice.json" <<EOF
{"auditReportVersion": 2, "vulnerabilities": {
  "picomatch": {"name": "picomatch", "severity": "high", "isDirect": false,
    "via": [{"source": 1, "name": "picomatch", "title": "ReDoS", "severity": "high",
      "url": "https://github.com/advisories/GHSA-fc5q-39xc-vjpf", "range": "<=4.0.7"}],
    "range": "*", "fixAvailable": {"name": "x", "version": "1.0.1", "isSemVerMajor": false}},
  "picomatch-copy": {"name": "picomatch-copy", "severity": "high", "isDirect": false,
    "via": [{"source": 1, "name": "picomatch", "title": "ReDoS", "severity": "high",
      "url": "https://github.com/advisories/GHSA-fc5q-39xc-vjpf", "range": "<=4.0.7"}],
    "range": "*", "fixAvailable": $BREAKING}
}, "metadata": {}}
EOF
run "Q one advisory on two packages, one fixable, fails" 1 twice listed "apply it"

if [ "$failures" -ne 0 ]; then
  printf '%s case(s) failed\n' "$failures" >&2
  exit 1
fi
printf 'test-check-npm-audit: %s case(s) passed\n' "$passed"
