---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-05T16:30:04Z
reviewed-against: 233506a9f1608426b2e7ea5e1445c21250d17e89
diff-base: 96823ec01951e4e7f4883051f3a69dd7d1dea983
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

apply-manifest's substitution-key contract: a placeholder-shaped or empty key is now rejected before any filesystem operation, and the replacement count the walk had been computing and discarding is reported per entry and in aggregate. Zero MUST violations, zero SHOULD, zero low-confidence. The change is itself the repair of a QUAL-CLAIM-001 instance in this primitive — a result that reported created/updated/unchanged identically whether every placeholder had been substituted or none had — and the new fields are shaped to the same rule: the per-entry count is absent rather than zero when substitution never ran, so examined-and-matched-nothing and never-examined cannot arrive as the same value. Validation is exact rather than stylistic, refusing only keys incapable of matching a placeholder, which is what lets it be a hard error rather than the warning §design-principles would reject. Verified through the release binary rather than the MCP tools per AGENTS.md: braced keys exit 1 with zero files written, bare keys substitute and report their counts. Four new unit tests, including a reproduction of the adopter call that motivated it, asserting both the error and that the destination tree is untouched.

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
