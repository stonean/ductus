---
spec: 047-analyze-findings-durability
reviewed-at: 2026-09-07T16:22:10Z
reviewed-against: 054c888c6b6a4d4fd480e07a6abaaa50c41142aa
diff-base: 6c40644dc0a6240090403bd71867bf0b7a79e4f6
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 047-analyze-findings-durability

## Summary

0 MUST violation(s), 0 SHOULD violation(s), 0 low-confidence finding(s). blocking: no.

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

- perf: `compute-review-scope` reports `captured-issues: []` while items sit uncommitted in `specs/inbox.md` — it diffs the inbox across `diff-base..HEAD`, so issues captured this session are invisible to the report that exists to surface them. Same committed-tree horizon 047 just removed from the analyze record, in a third place. — `runtime/src/primitives/compute_review_scope.rs`
- convention: the take(3) / "(+N more)" path-list rendering is triplicated in check_review_gate.rs (`unexaminable_contracts_guidance`, `stale_review_block`, `stale_analyze_block`). Maps to no loaded rule — CFG-CONST-001 governs constants shared across modules, not duplicated rendering within one. — `runtime/src/primitives/check_review_gate.rs:348`

## Skipped passes

*None.*
