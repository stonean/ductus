---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-17T22:10:16Z
reviewed-against: 40705898dfc286ec851c27950a238f97f9e6f0c8
diff-base: 979a3f015573599e2f7e78b209c8f79530a357d2
must-violations: 0
should-violations: 0
low-confidence: 1
captured-issues: 2
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Reviews the runtime work that reopened this spec: `check-orphaned-references` gaining the historical managed roots and the `matched-prefixes` field, released as `ductus-v0.29.9`. **0 MUST, 0 SHOULD outstanding, 1 low-confidence — not blocking.** Scope is 33 files this time rather than the 557 of the previous review, because the diff base moved to the commit that reopened the spec; the substantive delta is +107/-8 in `check_orphaned_references.rs`, +12 in `schema/primitives.rs`, and the `runtime-release.yml` gating fix. The defect was real and was reproduced before being fixed: `managed_roots` returned only *current* roots, while the orphan a migration chain leaves is a reference to the path *before* the move and therefore carries the *old* root — so the check was blind in exactly the dimension it existed to cover. The strongest evidence is not the argument but the artifact: this primitive's own test carried the comment *"`.govern/` is not a managed root, so the stale reference must be caught by naming a path under one that does not resolve"* and built its fixture around the gap. A test that routes around a blind spot documents it, and that test now asserts the retired-root reference is reported. The historical prefixes are `scripts/gen-` and `scripts/lib/` rather than `scripts/` on purpose, because the framework owned those and never the whole directory; a directory root would turn every unresolved adopter script into a finding, which is the noise the original scoping existed to prevent. That boundary is covered by a test asserting an adopter's own `scripts/build.sh` is not flagged. The `matched-prefixes` field is the scope-honesty half: `examined` already bounded the claim by *subject*, and a reference carrying no listed prefix is never reported under `skipped` because nothing recognizes it as a reference at all, so a clean result without the field asserts *no orphans* while meaning *no orphans among these prefixes*. Verified end-to-end on the case that motivated it rather than only in tests: the second real adopter bootstrap for 048's AC10 reported `AGENTS.md:118 names scripts/gen-spec-deps.sh, which moved to .ductus/scripts/gen-spec-deps.sh` — the precise orphan the previous run passed over and a human found by reading the file. Dogfooded against this repository in the same shape a caller sees: 0 findings, all four referrers examined, nothing skipped, and the six prefixes declared — so the widened roots introduce no self-inflicted noise, which was checked by reproducing the `preceded_by_path_char` guard before the change rather than after, since every `scripts/gen-` occurrence in this repo's own referrers sits inside a longer `.ductus/scripts/…` path. The one remaining entry is carried from the previous review and unchanged: `CFG-ENV-001` in `fetch_archive.rs` reads its env var per call where the rule requires a cached read, recorded low-confidence because the count is bounded, nothing in the crate mutates the environment in-process so a cached read would be identical, and re-reading per redirect hop is defensible as part of screening each hop independently. Both captured issues from this window closed inside it and are reconciled below rather than left reading as open. Whole-surface evidence at this HEAD: 956 lib tests plus 13 suites green including one corrected and three added; `clippy --release --all-targets -D warnings` exit 0; `fmt --check` clean; `scripts/audit/run-all.sh` exit 0 run after committing; and the release gate itself green on all five platforms with the acquisition legs verified against the published assets.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: CFG-ENV-001 — insecure-host allowlist is read from the environment per call rather than cached once

- **File**: `runtime/src/primitives/fetch_archive.rs:295-299`
- **Rule**: All environment variables MUST be read once at startup and the value cached; per-call reads from os.environ (or equivalent) are forbidden.
- **Finding**: Unchanged from the previous review and carried forward because the file remains in scope. `host_is_insecure_allowed` calls `std::env::var` on each invocation and `validate_fetch_url` calls it for the initial URL plus every redirect hop, so one fetch performs up to 1 + MAX_FETCH_REDIRECTS reads. Low-confidence because none of the rule's three stated harms land: the count is bounded rather than a hot path, nothing in the crate calls `set_var`/`remove_var` so a value cached at first use would be identical, and the default is documented at the constant. Re-reading per hop is also defensible on its own terms, since each redirect target is meant to receive a complete independent screen.
- **Auto-fixable**: yes
- **Suggested fix**: Wrap the parsed allowlist in a `std::sync::LazyLock<Option<Vec<String>>>`, or resolve it once in `run` and thread it into `validate_fetch_url`. Safe as written — nothing mutates the environment in-process, and the parity tests set the variable on a subprocess before it starts, so a startup read still observes it.

## Waived findings

*None.*

## Captured issues

- [x] bug: the crates.io publish did not depend on the SBOM job, so a release could ship its crate with no bill of materials — observed on `ductus-v0.29.9`, whose SBOM upload hit a transient GitHub 5xx while the publish proceeded regardless. **Closed in `4070589`**: `publish` now needs `sbom`, so the artifact and its SBOM ship together or neither does. Removed from `specs/inbox.md`, so it is no longer outstanding and does not count above.
- [x] decision: whether to keep `[migrations].applied` after the defect motivating it proved not to be one — `workflows-sunset` carries `sunset_after = "0.25.0"` and was correctly excluded as retired. **Closed in `4ceb799`**: the operator chose to revert, the watermark is restored, and 027's scenario and task were withdrawn rather than left recording a fix for a non-defect. Removed from `specs/inbox.md`, so it is no longer outstanding and does not count above.

## Observations

*None.*

## Skipped passes

*None.*
