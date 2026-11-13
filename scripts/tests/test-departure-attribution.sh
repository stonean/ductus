#!/usr/bin/env bash
# Test surface for `check-departures`' attribution (spec 073, AC7).
#
# An entry's `decided-by` and `decided-at` are the author and author time of
# the commit that introduced it, read from git history. The unit tests in the
# runtime build their own repositories and run in runtime.yml's depth-1
# matrix; this script runs the built release binary against a repository it
# commits to here, in the job that has full history, and proves each state:
#
#   A. attributed: the introducing commit's author email, and its author time
#      as RFC 3339 UTC with Z (a non-UTC author offset is converted)
#   B. edited: a later edit of the entry keeps the introducing commit
#   C. uncommitted: an entry only in the working tree is `unattributed`, with
#      no `decided-at`
#   D. re-added: an entry removed and added back is introduced by the
#      re-adding commit
#   E. shallow: a `git clone --depth 1` of the fixture attributes nothing
#
# This script reads history, so it MUST run in a checkout with full depth: it
# fails, naming `fetch-depth: 0`, when its own checkout is shallow, because a
# check that cannot run must never look like one that passed. The fixture is
# built under a temp directory, never read from the checkout.
#
# Usage: scripts/tests/test-departure-attribution.sh
# Needs git and jq, and ./runtime/target/release/ductus (or `ductus` on PATH).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

if [ "$(git -C "$REPO_ROOT" rev-parse --is-shallow-repository)" != "false" ]; then
  echo "FAIL: this checkout is shallow, so the attribution test cannot run; check out with fetch-depth: 0" >&2
  exit 1
fi

if [ -x "$REPO_ROOT/runtime/target/release/ductus" ]; then
  RUNTIME="$REPO_ROOT/runtime/target/release/ductus"
elif command -v ductus > /dev/null; then
  RUNTIME="$(command -v ductus)"
else
  echo "FAIL: no ductus runtime: build runtime/target/release/ductus first" >&2
  exit 1
fi
command -v jq > /dev/null || { echo "FAIL: jq is required" >&2; exit 1; }

# Isolate git from the user's configuration and the runtime from any session.
unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE GIT_PREFIX GIT_COMMON_DIR \
  GIT_OBJECT_DIRECTORY GIT_ALTERNATE_OBJECT_DIRECTORIES \
  CLAUDE_CODE_SESSION_ID DUCTUS_SESSION
export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
FIXTURE="$WORK/fixture"

# commit_as EMAIL DATE MESSAGE — commit what is staged, with fixed identity and
# time, so every expectation is exact.
commit_as() {
  GIT_AUTHOR_NAME="Author" GIT_AUTHOR_EMAIL="$1" GIT_AUTHOR_DATE="$2" \
    GIT_COMMITTER_NAME="Committer" GIT_COMMITTER_EMAIL="committer@example.com" \
    GIT_COMMITTER_DATE="2030-01-01T00:00:00+0000" \
    git -C "$FIXTURE" commit -q -m "$3"
}

# write_spec REASON RULE... — spec.md holding one departure entry per rule,
# each with the reason REASON.
write_spec() {
  local reason="$1"
  shift
  {
    printf -- '---\nstatus: draft\ndependencies: []\n---\n\n# Spec\n\n## Applicable Rules\n\n'
    for rule in "$@"; do
      printf -- '- departs: `%s`\n  - reason: %s\n' "$rule" "$reason"
    done
    printf '\n## Next\n\ntext\n'
  } > "$FIXTURE/specs/001-x/spec.md"
}

mkdir -p "$FIXTURE/specs/001-x" "$FIXTURE/specs/rules"
git -C "$FIXTURE" init -q
git -C "$FIXTURE" config commit.gpgsign false
{
  printf '# Demo rules\n\n'
  for n in 001 002 003 004; do
    printf '### DM-MUST-%s\n\n> The service MUST do the thing.\n\n' "$n"
  done
} > "$FIXTURE/specs/rules/demo-cross.md"

# c1: DM-MUST-001 (attributed, then edited) and DM-MUST-003 enter.
write_spec "as first written" DM-MUST-001 DM-MUST-003
git -C "$FIXTURE" add specs
commit_as "first@example.com" "2026-02-03T10:00:00+0200" "introduce entries"
# c2: both reasons are edited by another author.
write_spec "as edited" DM-MUST-001 DM-MUST-003
git -C "$FIXTURE" add specs
commit_as "second@example.com" "2026-03-04T11:00:00+0000" "edit the entries"
# c3: DM-MUST-003 is removed.
write_spec "as edited" DM-MUST-001
git -C "$FIXTURE" add specs
commit_as "third@example.com" "2026-04-05T12:00:00+0000" "remove an entry"
# c4: DM-MUST-003 is added back by a fourth author.
write_spec "as edited" DM-MUST-001 DM-MUST-003
git -C "$FIXTURE" add specs
commit_as "fourth@example.com" "2026-05-06T13:30:45+0000" "re-add an entry"
# Not committed: DM-MUST-004 is only in the working tree.
write_spec "as edited" DM-MUST-001 DM-MUST-003 DM-MUST-004

# attribution DIR RULE FIELD — a field of RULE's entry in DIR's check-departures
# result, or the word `absent` when the entry has no such field.
attribution() {
  local out
  out="$(cd "$1" && "$RUNTIME" check-departures --feature 001-x)" \
    || { echo "check-departures failed in $1" >&2; return 1; }
  printf '%s' "$out" | jq -r --arg rule "$2" --arg field "$3" \
    '[.entries[] | select(.rule == $rule)] | if length == 1 then (.[0][$field] // "absent") else "no single entry" end'
}

# expect CASE DIR RULE FIELD WANT
expect() {
  local got
  got="$(attribution "$2" "$3" "$4")" || { fail "$1: the primitive did not run"; return; }
  if [ "$got" = "$5" ]; then
    pass "$1: $3 $4 is $5"
  else
    fail "$1: $3 $4 is '$got', want '$5'"
  fi
}

echo "check-departures attribution (spec 073, AC7)"

expect "A attributed" "$FIXTURE" DM-MUST-001 decided-by "first@example.com"
expect "A attributed" "$FIXTURE" DM-MUST-001 decided-at "2026-02-03T08:00:00Z"
expect "B edited" "$FIXTURE" DM-MUST-001 decided-by "first@example.com"
expect "C uncommitted" "$FIXTURE" DM-MUST-004 decided-by "unattributed"
expect "C uncommitted" "$FIXTURE" DM-MUST-004 decided-at "absent"
expect "D re-added" "$FIXTURE" DM-MUST-003 decided-by "fourth@example.com"
expect "D re-added" "$FIXTURE" DM-MUST-003 decided-at "2026-05-06T13:30:45Z"

SHALLOW="$WORK/shallow"
git clone -q --depth 1 "file://$FIXTURE" "$SHALLOW"
if [ "$(git -C "$SHALLOW" rev-parse --is-shallow-repository)" = "true" ]; then
  expect "E shallow" "$SHALLOW" DM-MUST-001 decided-by "unattributed"
  expect "E shallow" "$SHALLOW" DM-MUST-001 decided-at "absent"
  expect "E shallow" "$SHALLOW" DM-MUST-003 decided-by "unattributed"
else
  fail "E shallow: the depth-1 clone is not shallow, so the case proves nothing"
fi

if [ "$failures" -gt 0 ]; then
  echo "$failures failure(s)" >&2
  exit 1
fi
echo "all attribution cases passed"
