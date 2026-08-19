---
spec: 022-deterministic-runtime
scenario: orphan-check-adopter-authored-references
reviewed-at: 2026-08-19T00:56:38Z
reviewed-against: 74924ca6bacb3652443bd4be0ea2f647c2df2b86
diff-base: 8c2b67aca4bbfe07fdd633e0fa32b0de5846e3b9
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Clean — 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations.

Scope: the `orphan-check-adopter-authored-references` scenario and its implementation. The diff base is the documented `--since` override (`8c2b67a`, the commit before 022 was reopened) rather than the recorded `in-progress` base, which predates the 0.28.0 cycle and resolves several hundred files — see AGENTS.md §Gotchas. The override narrows the scope to this cycle's work only; nothing about the recorded base is stale.

Reviewed: `runtime/src/primitives/check_orphaned_references.rs` (referrer set now resolved rather than const), `framework/commands/analyze.md`, `specs/022-deterministic-runtime/data-model.md`, `runtime/CHANGELOG.md`, and the version sites.

Security: no new I/O surface. The added referrer is read-only, path-joined under the repo root, and passes through the existing `stays_in_repo` guard, so a traversal in an adopter-authored reference is still not treated as a managed path.

Reuse: the change threads the already-loaded `Paths` layout through `managed_roots` and `referrers` rather than adding a second config read — the one defect this review found, fixed in `74924ca` before recording.

Claim discipline (`QUAL-CLAIM-001`): an absent `system.md` remains neither a finding nor a skip, and `examined` is what distinguishes "nothing there" from "examined and clean". Verified by a dedicated test rather than by inspection.

Verification: two of the three new tests were proven red against the unchanged primitive before being trusted green; the third is a guard that correctly holds either way. Full suite 1042 passing, clippy clean, `cargo fmt` clean, 25 audit families green, markdownlint clean over 432 files.

Not delivered: `runtime/` changes reach no adopter until `ductus-v0.29.11` is tagged and pushed. The version is bumped across all three sites and the tag is outstanding.

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
