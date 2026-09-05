---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-05T18:25:27Z
reviewed-against: 46793ce23ee57a8cb348fe547d708c07a2d4d482
diff-base: b993a948184f61a76ccf0e945ce8feac07a529e4
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

resolve-anchor's three reference kinds. Zero MUST, zero SHOULD, zero low-confidence. The finding that shaped this change was raised by the existing test suite rather than by the review: the first draft excluded a line citing the markers file itself, which would have dropped the one kind of reference the primitive exists to check, and a pre-existing test caught it. Correcting it surfaced three further unresolved references the coarser rule had hidden — all three historical claims in done specs, correct as written and correctly still reported. The design decision worth recording is the refusal to widen the rule until the residue is empty: matching 'the bootstrap's' or 'spec 022' means guessing which document prose meant, and a rule that fires falsely on a correct reference is worse than the silence it replaces. 34 classified findings a maintainer can read beats 112 undifferentiated ones, and beats zero bought by guessing.

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
