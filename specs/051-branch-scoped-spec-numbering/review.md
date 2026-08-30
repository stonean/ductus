---
spec: 051-branch-scoped-spec-numbering
reviewed-at: 2026-08-30T20:33:11Z
reviewed-against: 08355dd173cb4357fb02dd0f2361f4bd95b1cfbc
diff-base: eb52e24a8bf9ce362430abe583d2ef63dd9bafc9
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 051-branch-scoped-spec-numbering

## Summary

Re-review, not a fresh audit. No code belonging to this spec changed since the previous review; what moved is this spec's own account of `retire-feature`, annotated by spec 052 after that spec gated the sequential-form refusal behind an explicit `allow-sequential` argument.

The annotation landed in `data-model.md`, which is one of the durable contracts the release gate diffs — so the previous review, recorded against `b985b9ff`, no longer described the artifacts it claimed to and Family 19 blocked the release on it. That block was correct and is the reason this run exists: the gate caught an edit to a `done` spec's contract that had been reasoned about and waved through as safe. It was safe for the spec's *behavior* and not for the *record*, which is the distinction the check exists to hold.

All five passes ran over the unchanged scope and produced no findings. The behavioral claim the annotation qualifies is still true for this spec's own caller: `/{project}:fold` does not pass the opt-in and has no argument that would, which spec 052 pinned in a test (`fold_never_reaches_the_sequential_opt_in`). The `fold-target` row is untouched and applies to both callers, so the anti-stranding guarantee this spec relies on is unchanged.

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
