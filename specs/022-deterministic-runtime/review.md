---
spec: 022-deterministic-runtime
scenario: review-gate-unexaminable-contracts
reviewed-at: 2026-08-27T16:14:53Z
reviewed-against: 1913abbf57595f1faba26801934fad0a75cf4b83
diff-base: a4df343d01d56901d79bb82476ec69c9e47a0126
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Reviewed at `1913abb`. 0 MUST, 0 SHOULD.

The previous run's `QUAL-CLAIM-001` SHOULD is **resolved, not waived**. `unexaminable_contracts_guidance` returned `None` both when every durable contract was committed and when git could not be queried, so a caller read a bare pass as assurance in a state where nothing had been examined — the same conflation the function exists to remove one level up. It now reports that the working tree could not be inspected, distinctly from finding nothing dirty, and `a_working_tree_that_cannot_be_inspected_is_not_reported_as_clean` covers it. Resolving rather than accepting was the right call precisely because the finding was against this scenario's own contract: an exception here would have been the scenario contradicting itself.

Fixing it surfaced a second instance in the existing tests. `passes_when_lint_clean_and_review_current` asserted `guidance.is_none()` against a bare tempdir with no git repository — an assertion that encoded the old conflation, since the gate cannot inspect a non-repo and silence there could never have meant "examined and clean". It now asserts the honest outcome, with the genuinely-clean path covered separately by `a_clean_tree_emits_no_unexaminable_guidance`, which commits first. A test that had to change to accommodate the fix is worth naming: it was asserting the defect.

Release state is now correct. The prior commit changed `runtime/` without a version bump, so it sat on `main` reachable by no adopter — `AGENTS.md` §Workflow makes the bump and the `ductus-v<version>` tag part of the completion gate rather than a separate chore, and `ductus-v0.31.0` was already tagged. All three sites now read 0.32.0 (repo-root `version`, `runtime/Cargo.toml`, a matching `CHANGELOG.md` section); Family 20 and `lint-release-ordering.sh` both pass. The parity goldens needed no re-bless — the version is a placeholder the harness substitutes — and the release binary was rebuilt first, per the same entry.

Security, reuse, efficiency, and simplicity are unchanged from the prior run and re-checked: no new surface, `is_durable_contract` still shared with `stale_review_block` rather than restated, one pathspec-bounded status walk, and no new abstraction — the fail-distinctly change is a closure returning the same `Option<String>` the field already carried.

Verification: 1020 lib tests, parity 11/11, mcp 26/26, every other target green; `cargo fmt --check` and `clippy --release --all-targets --locked -- -D warnings` clean; the six repo lint scripts clean; `shellcheck -S warning` over the tracked shell set clean; `markdownlint-cli2 '**/*.md'` clean; the 29-family audit green. The audit is re-run after this commit and before tagging, per the entry that exists because a green pre-commit audit is not evidence for the families that read history.

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
