---
spec: 045-decision-state-drift-detection
reviewed-at: 2026-08-03T02:44:40Z
reviewed-against: 8891da925ff7b5f8d5c2892ffd1689bb8f8d4915
diff-base: 5103cd3a32fa53b07a9536200d609f3632e57a71
must-violations: 0
should-violations: 0
low-confidence: 1
captured-issues: 1
skipped-passes: []
---

# Review — 045-decision-state-drift-detection

## Summary

Third pass, re-run because the two prior ones (`c11bbb1`, `0857f07`) predate the live-claim exemption and the `mark-task` symmetry change. The first pass reported 0 MUST and 3 SHOULD, all fixed rather than shipped; the fix for the duplicated spec parse also removed the `.ok()` swallow a fourth, low-confidence finding named. Since then the `criterion-path-existence` family gained the live-claim exemption — thirteen closed phrases marking a criterion as a deletion, rename, adopter-scope, or hedge statement, so a path that fails to resolve confirms it rather than contradicting it. That change was itself prompted by a measurement error worth recording in a review: an earlier triage classified findings by path prefix without reading their criteria and reported 35 true positives at 69% precision, when the real figure was 5 of 28. The corrected measurement and the reasoning are in `data-model.md`. What remains is one low-confidence trade-off, unchanged from the second pass and re-verified against the current code.

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

- [ ] bug: `compute-review-scope` returns an unusable scope and a polluted captured-issues list — plan-affected is not parsed as a table, and captured-issues takes raw added lines rather than the shared comment-aware bullet grammar.

## Skipped passes

*None.*
