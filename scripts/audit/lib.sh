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

# ductus_bin — echo the path to a usable runtime binary, or nothing.
#
# The three tiers in preference order: the adopter-owned pointer, this repo's
# release build, then `PATH`. Declared here because four families now need it
# and a fourth copy is how the tiers drift — the same reasoning that put the
# directory-membership rule in one predicate (spec 051).
#
# Returns empty rather than failing: whether an unreachable runtime is a
# precondition finding or a silent skip is the caller's decision, not this
# helper's, and every family so far treats it as a finding.
ductus_bin() {
  if [ -x .ductus/bin/ductus ]; then
    echo ".ductus/bin/ductus"
  elif [ -x runtime/target/release/ductus ]; then
    echo "runtime/target/release/ductus"
  elif command -v ductus > /dev/null 2>&1; then
    command -v ductus
  fi
}

# spec_corpus BIN — echo `slug<TAB>status` for every feature directory the
# runtime recognizes, one per line.
#
# The corpus comes from `dashboard`, so a family never carries its own copy
# of the directory-membership rule and never hand-rolls a frontmatter
# `status:` scan. Both were real defects: a `[0-9][0-9][0-9]-*` glob skips
# the 1000th spec entirely, and a hand-rolled `\s*`-based scalar read walked
# past an empty value onto the next line and produced a confidently-wrong
# finding (AGENTS.md, Design Principles).
#
# **Exit status, not emptiness, is the failure signal.** A project with no
# specs at all is a legitimately empty corpus, not a broken run — the python
# below exits non-zero only when the JSON does not parse, which is what an
# unusable `dashboard` produces. A caller testing `-z` instead would report a
# precondition failure at every fresh adopter, inventing a finding out of an
# empty repository.
spec_corpus() {
  "$1" dashboard 2> /dev/null | python3 -c '
import json, sys
try:
    data = json.load(sys.stdin)
except ValueError:
    sys.exit(1)
for spec in data.get("specs", []):
    print("%s\t%s" % (spec.get("slug", ""), spec.get("status", "")))
'
}
