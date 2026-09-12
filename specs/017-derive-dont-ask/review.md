---
spec: 017-derive-dont-ask
reviewed-at: 2026-09-12T23:43:13Z
reviewed-against: de5ccb4ffa26d341f570a45526102b2aec169a21
diff-base: 3db3d0e9238f824995b87e3757d97363a76d2029
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 017-derive-dont-ask

## Summary

Reviewed the §State at hand-off retirement across all five dimensions. The change under review is a prose deletion from the spec body — 44 lines of hand-off narration whose claims had gone stale — with no source, test, or contract touched, and no dependency edge moved (the removed prose cited sibling slugs in backticks rather than links). The durable content it carried survives where a reader meets it: AC25 and AC26 state the requirements, and 022's review-observations-write-through and specify-routes-before-scaffolding carry the implementations. 0 MUST, 0 SHOULD, 0 low-confidence.

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

## Unexamined governance

*None.*
