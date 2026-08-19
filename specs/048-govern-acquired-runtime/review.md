---
spec: 048-govern-acquired-runtime
scenario: release-halves-publish-together
reviewed-at: 2026-08-19T15:06:42Z
reviewed-against: 5bf20ffd1cd7dc6aabfc17d783f290ffe69e5d51
diff-base: 02c386bb2db6cefd013cfba629b1aa2fa9b066b5
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 048-govern-acquired-runtime

## Summary

Clean at `5bf20ff` — 0 MUST, 0 SHOULD, 0 low-confidence, across all five passes. One observation, captured to the inbox.

Scope is task 15's release-ordering work: `.github/workflows/runtime-release.yml`, `.github/workflows/runtime-acquisition.yml`, `.github/workflows/framework-checks.yml`, `scripts/lint-release-ordering.sh`, and `scripts/tests/test-lint-release-ordering.sh`.

Two defects were found during this review and fixed in `5bf20ff` before it was written, so they are recorded here rather than as open findings. **`retention-days: 1` on the staged assets** was correct when `release-assets` consumed them minutes after the build; it is now the last job in the chain, and the documented recovery from a failed publish — fix the cause, re-run — took a token rotation on `ductus-v0.30.0`. A next-day re-run would have failed on expired artifacts instead of on the real cause. Retention now outlives one pipeline run. **The lint mis-parsed two YAML shapes**: it matched job keys anywhere, so the two-space child `push:` under `on:` read as a job and captured everything below it, and it read the block-sequence `needs:` form as an empty list — a false alarm on a correctly-ordered workflow, which is the failure mode that trains a reader to ignore a lint. Job keys are now scoped to the top-level `jobs:` mapping and both `needs:` spellings parse to the same list, each covered by a test.

`QUAL-CLAIM-001` was the rule most at risk here and is satisfied deliberately: the lint distinguishes *examined and found nothing* from *could not examine* at every exit — an absent workflow, a parse that yielded no jobs, and a renamed `publish` or `release-assets` job each fail loudly rather than passing vacuously, and cases C, D, and F pin that behavior. `BE-DEPS-003` holds: the SBOM is still generated during the build from the resolved `Cargo.lock` graph and still ships attached to the release, staged through a workflow artifact rather than uploaded in its own job. That staging is load-bearing, not incidental — uploading to a release creates the release object when absent, which is how `ductus-v0.28.0`'s first attempt left a tag carrying an SBOM and no binaries.

The reordering does not weaken the publish gates. `publish` still names every gate that guarded it before, and asset completeness is now enforced further upstream: each `acquire` leg downloads its own target's staged artifact by exact name, so a target that produced nothing fails before anything irreversible happens. What the change gives up is the gating job's fetch over the wire from the release URL — impossible by construction, since the release no longer exists at that point. That narrowing is the observation below rather than a line in this Summary, which the next review run would regenerate away.

Workflow and scripts only — no `runtime/` change, so no three-way version bump and no `ductus-v*` tag.

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

- other: the constitution's §runtime-boundary acquisition invariant requires a CI job that fetches **the published asset** on every supported platform. After the release-halves reordering, the automatically-triggered job (`acquire` in runtime-release.yml) fetches the staged build artifacts instead — the release does not exist yet by construction — and the only job that fetches over the wire from the release URL is `runtime-acquisition.yml`, which is workflow_dispatch-only. The bytes are identical, so the invariant's substance holds, but its automatic enforcement narrowed. Either reword the sentence to match, or add a post-release verification job to runtime-release.yml. — `framework/constitution.md`

## Skipped passes

*None.*
