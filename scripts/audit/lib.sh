#!/usr/bin/env bash
# scripts/audit/lib.sh — shared boilerplate for the `/audit` check scripts.
#
# Sourced, never executed. Holds the three things every check script would
# otherwise re-declare: the repo root (plus the `cd` into it), the `drift`
# accumulator, and the `emit` function that renders the pipe-separated
# finding line specified in ./README.md §Contract. The finding shape is
# defined here once so a change to the contract is a one-file edit rather
# than a sweep across every family.
#
# Usage — the first three lines of a family script:
#
#     set -uo pipefail
#     . "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
#     audit_family cross-doc
#
# then `emit LOCATION MESSAGE SUGGESTED-FIX` per finding, and `exit "$drift"`
# at the end. Sourcing performs the `cd`, so a script stays directly
# invocable from any working directory — the per-family standalone contract
# in README.md is unchanged. The `|| exit 1` matters: without it a script
# that cannot find this file runs on with `emit` undefined, and a family
# that exits 0 by design (ssot-invariants) would report clean.
#
# Bash 3.2 compatible (macOS system bash): no associative arrays, no `local -n`.

# shellcheck disable=SC2034  # ROOT/drift are read by the sourcing script, not here

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT" || exit 1

# Set to 1 by `emit`. Every check script ends with `exit "$drift"`.
drift=0

# Leading column of each finding line. `audit_family` overwrites it; the
# placeholder default keeps `set -u` satisfied if a script emits before
# naming itself.
AUDIT_FAMILY="audit"

# audit_family NAME — set the family label for this script's findings.
audit_family() {
  AUDIT_FAMILY="$1"
}

# emit LOCATION MESSAGE SUGGESTED-FIX — one finding line, and mark drift.
emit() {
  echo "$AUDIT_FAMILY | $1 | $2 | $3"
  drift=1
}
