---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-03T03:00:12Z
reviewed-against: b6884f2903f47dc6fed98647976c7efed4a98ab3
diff-base: 113a1bc0000000000000000000000000000000
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Second pass on 022, covering the `review-scope-parse-fidelity` scenario (task 77). Three parsing corrections: `parse_affected_files` now ends a table at a non-table line (its header state previously survived the first separator, so a multi-table section emitted every later header row as a path) and takes the backticked span from a qualified first cell; `compute-review-scope` and `diff-cross-spec` both intersect their inbox additions against the bullets the shared comment-aware `iter_bullets` finds in the post-image file. All three are pure parsing changes over content the primitives already read — no new I/O beyond one blob/file read of an inbox that was already in scope, no new external contract, no new unbounded input. The `iter_bullets` reuse is the reuse pass's preferred outcome rather than a finding: the grammar existed and was simply not used here. Failure modes were checked: an inbox absent, unreadable, or non-UTF-8 in the post-image yields an empty bullet set and therefore no captured issues, which is the documented "nothing can be proven" direction rather than a silent claim. Verified against the real 017 input that motivated it — plan-affected 43 entries with zero malformed, captured-issues one real bullet. No findings.

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

## Skipped passes

*None.*
