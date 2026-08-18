---
spec: 050-constitution
reviewed-at: 2026-08-17T21:40:00Z
reviewed-against: e318dbc
diff-base: 16d7a6123fc9f1f8650bf3f2848ddb811943192d
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 050-constitution

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

- convention: the constitution's §drift-prevention now carries the sweep-target rule, but nothing checks that the sweep's own target list stays current when a later spec relocates a directory — the failure the rule warns about is exactly what happened to .ductus/scripts/ between 042 and 049, and it stayed invisible because a grep over the old location returns clean. A check would need to compare the list against the paths the manifest actually ships. — `framework/constitution.md`

## Skipped passes

*None.*
