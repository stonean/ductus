---
spec: 022-deterministic-runtime
scenario: derive-unparseable-frontmatter-is-reported
reviewed-at: 2026-08-19T14:05:51Z
reviewed-against: 66c2cf8c007087410ae5f807bdcc6146c62101b6
diff-base: 9c06b2dfd5f16618c50fd3a0186caf534a517778
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Reviewed the derive-unparseable-frontmatter-is-reported scenario against 66c2cf8, the commit containing it — the ordering AGENTS.md prescribes and which the previous cycle got wrong. No MUST or SHOULD violations; the spec is not blocked, and the QUAL-CLAIM-001 finding the prior review carried is resolved rather than carried forward: both derive results now name the specs they could not examine, so an empty `updated` asserts examined-and-clean only alongside an empty `unparseable`. The reuse pass confirms the detector went into the shared scanner beside `is_frontmatter_fence` rather than into either splice — a second copy of that test is precisely the drift extracting the fence predicate was meant to end, and the two primitives reach the gap by different routes (one loses its insertion anchor, the other never locates its key) which would have made divergent copies easy. The quality pass checked the boundary the rule turns on: a file with no frontmatter at all is deliberately not unparseable, since there is no block to close and reporting every plain markdown file would bury the signal, and that exemption is covered by a unit test. The security and efficiency passes have no subject — the change is one early-return per spec on a condition already computed by reading the file. Two mutations verified the tests can fail: a detector that always returns false restores the silent skip and fails the golden suite; one that drops the opening-fence guard reports every file and fails the unit test.

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
