---
spec: 048-govern-acquired-runtime
scenario: release-halves-publish-together
reviewed-at: 2026-08-19T15:14:21Z
reviewed-against: c650cb8809769eea9f24cd91f9caa8a2045e0fd9
diff-base: 02c386bb2db6cefd013cfba629b1aa2fa9b066b5
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 048-govern-acquired-runtime

## Summary

Clean at `c650cb8` — 0 MUST, 0 SHOULD, 0 low-confidence, across all five passes. The observation the prior run recorded is resolved, not carried: `c650cb8` restored the invariant rather than reworded it, and the inbox bullet is removed.

Scope is task 15's release-ordering work: `.github/workflows/runtime-release.yml`, `.github/workflows/runtime-acquisition.yml`, `.github/workflows/framework-checks.yml`, `scripts/lint-release-ordering.sh`, and `scripts/tests/test-lint-release-ordering.sh`.

**The ordering.** `publish` now runs before the GitHub release is created and `release-assets` depends on it, so the chain is `audit, build → acquire → sbom → publish → release-assets → verify-published`. crates.io is the irreversible half — a version can be yanked but never unpublished — so it is attempted first, and a failure leaves the recoverable half undone instead. Every gate that guarded `publish` before still guards it, and asset completeness moved further upstream: each `acquire` leg downloads its own target's staged artifact by exact name, so a target that produced nothing fails before anything irreversible happens.

**Two jobs had to move off the published release**, because both assumed it existed. `acquire` now reads staged artifacts. `sbom` now stages a `release-asset-*` artifact instead of attaching itself to the release — load-bearing, not cosmetic: uploading to a release creates the release object when absent, which is how `ductus-v0.28.0`'s first attempt left a tag carrying an SBOM and no binaries. `BE-DEPS-003` holds throughout; the SBOM is still generated during the build from the resolved `Cargo.lock` graph and still ships attached to the release.

**Three defects were found by review and fixed before this report.** `retention-days: 1` on the staged assets was sized for a `release-assets` that ran minutes after the build; it is now the last consumer in a longer chain whose documented recovery is a re-run, so a next-day re-run would have failed on expired artifacts rather than on the real cause. The lint mis-parsed two YAML shapes — job keys matched anywhere, so `on:`'s two-space child `push:` read as a job, and the block-sequence `needs:` form read as an empty list, a false alarm on a correct workflow. And the reordering had quietly narrowed the constitution's §runtime-boundary acquisition invariant to a dispatch-only workflow; `verify-published` restores it, calling `runtime-acquisition.yml` through `workflow_call` after the release exists so the published asset is fetched over the wire on every tag. Reuse rather than a second copy keeps the automatic and hand-dispatched paths from drifting.

`QUAL-CLAIM-001` was the rule most at risk and is satisfied deliberately at every exit of the new lint: an absent workflow, a parse yielding no jobs, a renamed `publish` or `release-assets`, and a `verify-published` moved ahead of the release each fail loudly rather than passing vacuously. Ten test cases pin that, and the two negative cases for the post-release check are the ones that matter most — a check that runs too early would fetch a URL that is not there yet and pass while proving nothing.

Workflow and scripts only — no `runtime/` change, so no three-way version bump and no `ductus-v*` tag. The new ordering takes effect on the next tag pushed after this lands.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*
