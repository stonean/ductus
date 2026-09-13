---
spec: 048-govern-acquired-runtime
reviewed-at: 2026-09-13T00:17:28Z
reviewed-against: 19745b4abfdc123f838523cde2f4b1affb9e68bf
diff-base: 3db3d0e9238f824995b87e3757d97363a76d2029
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 048-govern-acquired-runtime

## Summary

Full five-pass review against the 11 loaded rule files. Scope read in full: framework/bootstrap/ductus.md (the acquisition procedure above all), all four CI workflows (runtime-release, runtime-acquisition, framework-checks, generators), scripts/audit/version-agreement.sh and run-all.sh, framework/migrations.toml, both permission sets, the shipped adopter CI template, and this spec's own spec.md and plan.md. Acquisition verifies clean: the digest is checked before anything is written, a missing sidecar is a hard failure rather than a skip, the install is a tempfile+rename, and the binary is re-probed after — a truncated or wrong-architecture asset reads as no usable runtime rather than version unknown. The release job graph is correct and enforced: audit+build to acquire to sbom to publish (crates.io, irreversible) to release-assets to verify-published, with release-assets gated on publish, which is the ordering release-halves-publish-together exists to hold, and lint-release-ordering.sh asserts it on every PR. The migration registry resolves completely — every one of the twelve ids has its procedure file. One shape that looked like a defect is not: runtime-acquisition.yml's `[ "$target" = ... ] && binary="ductus.exe"` sits mid-script under set -euo pipefail, and the AND-list is fatal only as the last command of a block, which AGENTS.md states precisely and a direct test confirmed. Two defects were found, both fixed: plan.md named framework/migrations/runtime-path-rewrite.md as Create when it shipped under the registry id runtime-store-path, an actively misleading pointer rather than an ordinary planning-aid entry; and the spec claimed README "currently" documents a manual `sudo install -m 0755 ductus /usr/local/bin/ductus`, which README contradicts outright — it states the runtime is acquired, not installed, and registration moved to docs/runtime.md. Fixed in de5ccb4 and 1e9fa8c. 0 MUST, 0 SHOULD, 0 low-confidence outstanding.

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

## Unexamined governance

*None.*
