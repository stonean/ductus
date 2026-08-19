#!/usr/bin/env bash
# Test surface for scripts/lint-release-ordering.sh.
#
# The lint guards a property that fails silently — the crates.io publish must be
# attempted before the GitHub release is created — so the only evidence it works
# is that it FAILS on each regression it claims to catch. A guard that cannot
# fail is the same false green AGENTS.md's first Design Principle records, and
# is what this file exists to rule out.
#
# Coverage:
#   A. the committed workflow passes (control)
#   B. release-assets reverted to the pre-048 `needs` fails
#   C. release-assets renamed fails — a lint pointed at nothing asserts nothing
#   D. publish renamed fails, for the same reason
#   E. the release-upload action in another job fails
#   F. an absent workflow fails rather than passing vacuously
#
# Usage: scripts/tests/test-lint-release-ordering.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
LINT="$REPO_ROOT/scripts/lint-release-ordering.sh"
WORKFLOW="$REPO_ROOT/.github/workflows/runtime-release.yml"

failures=0
pass() { printf '  PASS  %s\n' "$1"; }
fail() { printf '  FAIL  %s\n' "$1" >&2; failures=$((failures + 1)); }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# Run the lint against a copy of the workflow with one sed edit applied. The
# lint resolves its subject relative to its own location, so each case gets a
# throwaway tree rather than a mutated checkout.
probe() {
  case_dir="$WORK/$1"
  mkdir -p "$case_dir/.github/workflows" "$case_dir/scripts"
  cp "$LINT" "$case_dir/scripts/"
  if [ -n "$2" ]; then
    sed "$2" "$WORKFLOW" > "$case_dir/.github/workflows/runtime-release.yml"
  fi
  bash "$case_dir/scripts/lint-release-ordering.sh" > "$case_dir/out" 2>&1
}

echo "Running lint-release-ordering tests..."

# A. control — the committed workflow is correctly ordered.
if probe a-control 's|^$|&|'; then
  pass "A: the committed workflow passes"
else
  fail "A: the committed workflow fails the lint: $(cat "$WORK/a-control/out")"
fi

# B. the actual regression: the ordering this spec reversed, put back.
if probe b-needs 's|^    needs: publish$|    needs: [audit, build]|'; then
  fail "B: release-assets not needing publish was accepted"
elif grep -q "does not need 'publish'" "$WORK/b-needs/out"; then
  pass "B: release-assets not needing publish is rejected"
else
  fail "B: rejected for the wrong reason: $(cat "$WORK/b-needs/out")"
fi

# C/D. a renamed job must break the lint loudly rather than silently vacate it.
if probe c-release-renamed 's|^  release-assets:$|  release-upload:|'; then
  fail "C: a renamed release job was accepted"
elif grep -q "no 'release-assets' job" "$WORK/c-release-renamed/out"; then
  pass "C: a renamed release job is rejected"
else
  fail "C: rejected for the wrong reason: $(cat "$WORK/c-release-renamed/out")"
fi

if probe d-publish-renamed 's|^  publish:$|  crate-publish:|'; then
  fail "D: a renamed publish job was accepted"
elif grep -q "no 'publish' job" "$WORK/d-publish-renamed/out"; then
  pass "D: a renamed publish job is rejected"
else
  fail "D: rejected for the wrong reason: $(cat "$WORK/d-publish-renamed/out")"
fi

# E. uploading to a release creates the release object, so the action in any
#    other job can bring the release into being ahead of its gates.
if probe e-stray-upload 's|^          name: release-asset-sbom$|          uses: softprops/action-gh-release@v3|'; then
  fail "E: the release-upload action in the sbom job was accepted"
elif grep -q "uses softprops/action-gh-release" "$WORK/e-stray-upload/out"; then
  pass "E: the release-upload action outside release-assets is rejected"
else
  fail "E: rejected for the wrong reason: $(cat "$WORK/e-stray-upload/out")"
fi

# F. no subject is not the same as a clean subject.
if probe f-absent ""; then
  fail "F: an absent workflow passed vacuously"
elif grep -q "not found" "$WORK/f-absent/out"; then
  pass "F: an absent workflow is rejected"
else
  fail "F: rejected for the wrong reason: $(cat "$WORK/f-absent/out")"
fi

echo
if [ "$failures" -gt 0 ]; then
  echo "$failures test(s) failed" >&2
  exit 1
fi
echo "All lint-release-ordering tests passed."
