---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-03T02:44:40Z
reviewed-against: 8891da925ff7b5f8d5c2892ffd1689bb8f8d4915
diff-base: 0e76a5401e35b6b1bd0b39dcecb1e8fbf0a1b45e
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Reopened by the scenario back-edge for five scenarios: `block-element-scanner`, `check-artifacts-skipped-targets`, `link-adjacent-drift-family`, `criterion-path-existence-family`, and `mark-task-untick-symmetry`. The runtime surface in this window is `split_blocks` / `strip_inline_comments` / `inline_code_spans` in `primitives/mod.rs`, two new `check-artifacts` families plus the `skipped` result field, and the symmetric done-when reconciliation in `mark_task.rs`. An earlier pass over the same code (against `0857f07`, recorded on 045) reported 0 MUST and 3 SHOULD; all three were fixed rather than shipped — an unrecorded unreadable-artifact skip, a per-(block, link) re-read of the same scenario file, and a duplicated spec parse. This pass covers the deltas since: the live-claim exemption and the `mark-task` symmetry change. Both are pure logic over already-read in-memory content with no new I/O, no new external contract, and no new unbounded input; loop termination was checked on each new scanner (`strip_inline_comments`, `link_hrefs`, `contains_outside_code` all advance their cursor unconditionally), as was byte-boundary safety on every slice (all needles and delimiters are ASCII, and `to_ascii_lowercase` preserves byte length so offsets into the lowered copy still index the original's spans). The one open trade-off — lexical rather than canonical sibling resolution — is recorded on 045, where the requirement lives, rather than duplicated here. No findings.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] bug: `compute-review-scope` returns an unusable scope and a polluted captured-issues list — plan-affected is not parsed as a table, and captured-issues takes raw added lines rather than the shared comment-aware bullet grammar.

## Skipped passes

*None.*
