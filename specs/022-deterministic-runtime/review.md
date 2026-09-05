---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-05T17:34:13Z
reviewed-against: 2402ded3bed649b627c484541efac5436ac9dacd
diff-base: 1d281af0
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

The runtime half of 047's analyze-record requirement: the write-analysis primitive, two new check-review-gate reasons, and the ninth check-artifacts family. Zero MUST, zero SHOULD, zero low-confidence. Two design choices carry the weight and are recorded in the scenario rather than left implicit: blocking is derived inside the primitive rather than accepted as an argument, so no call can record a clean gate over a dirty run — the same discipline write-review applies to its own blocking field, and the reason a field a caller can contradict is a field that will eventually be contradicted; and splice_review_block was generalized to splice_top_level_block with both callers routed through it, because two copies of the frontmatter region logic would agree until one met a shape the other had not, where the failure mode is a corrupted spec.md rather than a wrong answer. The parity goldens moved as expected and the pre-commit hook caught both before they landed.

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
