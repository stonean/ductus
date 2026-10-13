#!/usr/bin/env bash
# scripts/audit/run-all.sh — `/audit` aggregator.
#
# Runs the check-zero precondition pass followed by the family check scripts
# registered below. Aggregates findings to stdout under per-family headers
# and exits 1 when any family (or check-zero) produced findings.
# Family numbers are stable identifiers: Family 3 (registry equivalence)
# was retired with the workflows feature (spec 043), leaving a gap.
#
# This script IS the implementation of `/audit`. The framework/commands/
# audit.md slash-command file is documentation that invokes this
# orchestrator via the runtime's `run-generator` primitive (single call,
# no per-step args needed — sidesteps the runtime parser's lack of
# per-step argument binding for procedural commands).

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1

# The aggregator renders per-family headers rather than finding lines, so it
# uses `drift` from the lib directly instead of `emit`.
run_check() {
  local label="$1" script="$2"
  if [ ! -x "$script" ]; then
    echo "$label | $script | check script missing or not executable | chmod +x $script"
    drift=1
    return
  fi
  local findings
  # Findings go to stdout and are rendered under a per-family header when the
  # family fails. A family's **coverage notices go to stderr** and are passed
  # straight through in both cases, rather than being captured and discarded on
  # success as they were until 0.49.8.
  #
  # That capture made every coverage line invisible on exactly the runs they
  # exist for. Family 19 has closed each run with a line naming what it
  # examined since it shipped, Families 30 and 35 report their counts the same
  # way, and none of it ever reached an aggregated run — so a clean `run-all.sh`
  # printed nothing at all, which is indistinguishable from a run that never
  # executed. That is `QUAL-CLAIM-001` turned on this aggregator: the exit code
  # said "no findings" and nothing said what had been looked at. Surfaced by
  # Family 38, whose whole contract is a standing notice and which was silent
  # on every clean run until this was fixed.
  #
  # The exit-code contract is unchanged, so CI still gates on it without
  # parsing anything (spec 026 AC12).
  if findings="$("$script")"; then
    :
  else
    echo "=== $label ==="
    echo "$findings"
    echo
    drift=1
  fi
}

run_check "check-zero (precondition)" "scripts/audit/check-zero.sh"
if [ "$drift" -eq 1 ]; then
  echo "(family checks skipped — check-zero failed; resolve the precondition findings and re-run /audit)"
  exit 1
fi

run_check "Family 1 — cross-doc claim consistency" "scripts/audit/cross-doc-consistency.sh"
run_check "Family 2 — manifest parity" "scripts/audit/manifest-parity.sh"
run_check "Family 4 — placeholder roundtrip" "scripts/audit/placeholder-roundtrip.sh"
run_check "Family 5 — template alignment" "scripts/audit/template-alignment.sh"
run_check "Family 6 — SSOT invariants" "scripts/audit/ssot-invariants.sh"
run_check "Family 7 — sibling-spec coupling" "scripts/audit/sibling-coupling.sh"
run_check "Family 8 — introducing-spec body drift" "scripts/audit/introducing-drift.sh"
run_check "Family 9 — primitive-promotion candidates" "scripts/audit/primitive-promotion-candidates.sh"
run_check "Family 10 — migration coverage" "scripts/audit/migration-coverage.sh"
run_check "Family 11 — consolidation-pair drift" "scripts/audit/consolidation-pair.sh"
run_check "Family 12 — fixture session-file shape" "scripts/audit/fixture-session-shape.sh"
run_check "Family 13 — runtime hardcoded paths" "scripts/audit/runtime-hardcoded-paths.sh"
run_check "Family 14 — installer/registry parity" "scripts/audit/installer-registry-parity.sh"
run_check "Family 15 — runtime probe parity" "scripts/audit/runtime-probe-parity.sh"
run_check "Family 16 — installer/command parity" "scripts/audit/installer-command-parity.sh"
run_check "Family 17 — host namespace parity" "scripts/audit/host-namespace-parity.sh"
run_check "Family 18 — marker-list parity" "scripts/audit/marker-list-parity.sh"
run_check "Family 19 — review freshness" "scripts/audit/review-freshness.sh"
run_check "Family 20 — version agreement" "scripts/audit/version-agreement.sh"
run_check "Family 21 — transitional bootstrap parity" "scripts/audit/transitional-bootstrap-parity.sh"
run_check "Family 22 — adopter shell behavior" "scripts/audit/adopter-shell-behavior.sh"
run_check "Family 23 — sweep-target manifest parity" "scripts/audit/sweep-target-manifest-parity.sh"
run_check "Family 24 — rename-sweep residue" "scripts/audit/rename-sweep-residue.sh"
run_check "Family 25 — unbalanced inline markup" "scripts/audit/unbalanced-inline-markup.sh"
run_check "Family 26 — broken relative links" "scripts/audit/broken-relative-links.sh"
run_check "Family 27 — done-spec unchecked criteria" "scripts/audit/done-spec-criteria.sh"
run_check "Family 28 — audit family registry parity" "scripts/audit/audit-family-parity.sh"
run_check "Family 29 — permission wildcard position" "scripts/audit/permission-wildcard-position.sh"
run_check "Family 30 — command flag/hint parity" "scripts/audit/command-flag-hint-parity.sh"
run_check "Family 32 — permission entry shape" "scripts/audit/permission-entry-shape.sh"
run_check "Family 33 — README command parity" "scripts/audit/readme-command-parity.sh"
run_check "Family 34 — step reference integrity" "scripts/audit/step-reference-integrity.sh"
run_check "Family 35 — manifest destination links" "scripts/audit/manifest-destination-links.sh"
run_check "Family 36 — self-URL resolution" "scripts/audit/self-url-resolution.sh"
run_check "Family 37 — analyze-record backlog" "scripts/audit/analyze-record-backlog.sh"
run_check "Family 38 — promotion coverage" "scripts/audit/promotion-coverage.sh"
run_check "Family 39 — shipped anchor resolution" "scripts/audit/shipped-anchor-resolution.sh"

exit "$drift"
