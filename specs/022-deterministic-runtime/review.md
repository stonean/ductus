---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-05T19:30:09Z
reviewed-against: ed134eb9f6e5be4bc0429f5597ab6e221482c6c3
diff-base: 970b8322bf6a1e62110a0fd95804f97eba346e5c
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

A code-span exclusion for resolve-anchor, recorded as an edge case on the reference-kinds scenario. Zero MUST, zero SHOULD, zero low-confidence. Found by running the primitive against the constitution rather than by the review pass, which is the second time this cycle the subject under change surfaced its own defect. It reuses the shared inline_code_spans helper rather than adding a scanner, matching the splice-sharing decision recorded on the same scenario and made for the same reason.

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
