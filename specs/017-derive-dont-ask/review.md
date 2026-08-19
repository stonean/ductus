---
spec: 017-derive-dont-ask
reviewed-at: 2026-08-19T01:24:46Z
reviewed-against: 4bfaaeec9e5af5d89d92811828412ca950d63cec
diff-base: e2779e5
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 017-derive-dont-ask

## Summary

Clean — 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations.

Scope: two link repairs in `scenarios/skip-prose-cross-references.md`, a scenario whose subject is precisely which cross-references produce a dependency edge — so its own links being wrong was worth correcting carefully rather than mechanically.

The two differ in kind, and the fix respects that. `024-rule-loader` is a real spec cited as a dependency-link example: the depth was corrected to `../../024-…` so it resolves. `018-scheduled-jobs` is an invented name illustrating a *navigational* link and names no spec that has ever existed — no depth correction could make it resolve, so it is now a backticked name rather than a link. That is also the form the scenario itself recommends for a citation that must not become an edge, which makes the example consistent with the rule it demonstrates.

The repair changes no claim, so it is mechanical under §spec-lifecycle and the spec stays `done`. Re-reviewed because Family 19 correctly flagged the change since `090ab025`.

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
