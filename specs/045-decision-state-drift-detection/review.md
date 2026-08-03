---
spec: 045-decision-state-drift-detection
reviewed-at: 2026-08-03T01:50:52Z
reviewed-against: 0857f07d6af648759126208401ef856ffccf3fb7
diff-base: 5103cd3a32fa53b07a9536200d609f3632e57a71
must-violations: 0
should-violations: 0
low-confidence: 1
captured-issues: 1
skipped-passes: []
---

# Review — 045-decision-state-drift-detection

## Summary

Second pass, against `0857f07`. The first run (`c11bbb1`) reported 0 MUST, 3 SHOULD, 2 low-confidence; all three SHOULD findings were fixed rather than shipped, and the fix for the double-parse also removed the `.ok()` swallow the first low-confidence finding named. What remains is one low-confidence observation about the deliberate lexical-vs-canonical path resolution. Posture: no blocking violations, no advisory violations, one recorded trade-off. The code under review is a read-only, locally-invoked markdown analyzer with no network, persistence, authentication, or concurrency surface, so the api / observability / concurrency / reliability rule sets load but have no applicable subject.

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

- [ ] **35 stale acceptance-criterion paths across 18 `done` specs, surfaced by 045's first full-repo run (2026-08-02).** The `criterion-path-existence` family flagged 51 paths named in `done` specs' `## Acceptance Criteria` that no longer resolve; 35 are confirmed true positives and need the `done → in-progress` back-edge to correct. Routing options and the 16 known false positives are recorded in the item.

## Skipped passes

*None.*
