---
spec: 047-analyze-findings-durability
reviewed-at: 2026-09-05T17:34:03Z
reviewed-against: 2402ded3bed649b627c484541efac5436ac9dacd
diff-base: 1d281af040ae1c7217bedcfceab93a13fb07c2cf
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 047-analyze-findings-durability

## Summary

The durable analyze record, its gate, and the bounded exemption. Zero MUST, zero SHOULD, zero low-confidence. One finding was raised against this work and is fixed in the reviewed tree rather than carried: Family 37 reported a clean backlog when its own scan never ran — QUAL-CLAIM-001, the third instance of that shape found this session by reviewing new code against the rule set it enforces, and the second found inside a family written to enforce it. It is fixed in 2402ded and was proven failing first. The design decision worth recording is the refusal to backfill: a criterion label is derivable from the artifact, so 013's backfill computed something already true, while an analyze record asserts that a run happened, which nothing on disk substantiates. Grandfathering plus a monotonic counted baseline is the honest form of that, and Family 37 is the price of the exemption rather than a convenience around it.

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
