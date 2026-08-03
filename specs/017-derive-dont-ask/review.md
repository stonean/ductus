---
spec: 017-derive-dont-ask
reviewed-at: 2026-08-03T03:05:16Z
reviewed-against: d99df57ecd05936029a1d29d08706ff48904ae01
diff-base: 096dbc0cf65a2322c91bfa895a825ea60c5a23f8
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 017-derive-dont-ask

## Summary

Re-run to clear the captured issue the prior pass recorded. That item — `compute-review-scope` returning an unusable scope and a polluted captured-issues list — has been fixed under 022's `review-scope-parse-fidelity` scenario rather than carried: a multi-table `## Affected Files` section no longer emits header rows as paths, a qualified first cell yields its backticked span, and both `compute-review-scope` and `diff-cross-spec` intersect their inbox additions against the shared comment-aware bullet grammar. The item is removed from the inbox, so it is no longer a captured issue here. 017's own change in this window is unchanged and re-reviewed: the honest no-change reporting in `gen-spec-deps.sh` and `gen-cross-service-refs.sh`, the shared `untracked_specs()` / `report_no_changes()` helpers, and the drifted-but-unwritten counter that a review of the fix itself surfaced. The `list_specs` git-ls-files exclusion remains byte-for-byte unchanged. No findings.

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
