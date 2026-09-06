---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-06T14:19:42Z
reviewed-against: 0405f6f7699961618892800211dc0cc9840260c6
diff-base: 683a1e03c463c62ea644a4466acc5873eba0d1a4
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

The unexamined breakdown. Zero MUST, zero SHOULD, zero low-confidence. The finding this change answers came from a question rather than a pass: a bare unexamined total conflates an exclusion by construction with a target that could not be read, which is the conflation the field exists to prevent, one level down — and the reason set was closed and documented the whole time, so the first implementation discarded information it already had. Same shape as apply-manifest's discarded substitution count at the start of this session. The total is now summed from the breakdown so the two cannot disagree, matching the discipline that derives blocking rather than accepting it. Also records that criterion-path-existence scopes to done specs, so a record written mid-back-edge describes a smaller subject — measured on this spec itself.

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
