---
spec: 045-decision-state-drift-detection
reviewed-at: 2026-08-03T03:05:16Z
reviewed-against: d99df57ecd05936029a1d29d08706ff48904ae01
diff-base: 5103cd3a32fa53b07a9536200d609f3632e57a71
must-violations: 0
should-violations: 0
low-confidence: 1
captured-issues: 0
skipped-passes: []
---

# Review — 045-decision-state-drift-detection

## Summary

Re-run to clear the same captured issue 017 carried — `compute-review-scope` returning an unusable scope and a polluted captured-issues list. It is fixed under 022's `review-scope-parse-fidelity` scenario, not carried forward: a multi-table `## Affected Files` section no longer emits header rows as paths, a qualified first cell yields its backticked span, and both `compute-review-scope` and `diff-cross-spec` intersect their inbox additions against the shared comment-aware bullet grammar. The item is out of the inbox, so it is no longer a captured issue on either spec. 045's own surface is unchanged since the prior pass and re-reviewed: the two `check-artifacts` families, the `skipped` result field, the shared block splitter, the constitution amendment, and `analyze.md`'s documentation. What remains is the one low-confidence trade-off recorded across the previous two passes.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: BE-INPUT-004 — sibling resolution is lexical, so an in-tree symlink can point outside the feature dir

- **File**: `runtime/src/primitives/check_artifacts.rs:700-732`
- **Rule**: User-supplied values MUST NOT be used directly in filesystem paths. Filesystem operations MUST resolve the canonical path and verify it falls within the expected base directory before opening the file.
- **Finding**: resolve_sibling performs the containment half of the rule — `..` is consumed by PathBuf::pop and the result must starts_with(feature_dir), so a link like ../../../etc/passwd is rejected — but not the canonical half. A symlink committed inside the feature directory (scenarios/evil.md -> /etc/shadow) resolves lexically inside the base and is then opened. Recorded low-confidence on applicability rather than mechanism: the hrefs come from repo-committed markdown carrying the same trust as the source, the primitive runs locally against the operator's own checkout and never over a network boundary, and the opened file's content is discarded after a readability test, so nothing is disclosed. Canonicalization was rejected deliberately — it fails on a missing target and makes the result symlink-dependent, which would break the repeat-run determinism AC8 requires.
- **Auto-fixable**: no
- **Suggested fix**: If the symlink case is judged in scope, keep the lexical resolution for the determinism guarantee and add a std::fs::symlink_metadata check on the resolved target, treating a symlink as target-unparseable. Otherwise record the trust boundary explicitly in the scenario's Edge Cases so the omission stays a decision rather than an oversight.

## Waived findings

*None.*

## Captured issues

*None.*

## Skipped passes

*None.*
