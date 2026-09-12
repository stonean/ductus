---
spec: 022-deterministic-runtime
reviewed-at: 2026-09-12T23:02:51Z
reviewed-against: 263a3644be0d26872006a55f063a61e430cba067
diff-base: 85f523cbb0eb58c271d2064f02bcf1e4a033d3f2
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Reviewed the seven-task change (022/110-113, with the 050 and 055 halves that land alongside) across all five dimensions, scoped with --since=85f523c because 022's default diff base predates the 0.28.0 cycle and resolves hundreds of files (AGENTS.md Gotchas). 37 files in the window, 64 in scope after the plan union. Three items were raised during the passes and all three were fixed and committed before this report was written, so none stands. Security: a cross-spec-impact entry was joined to the spec root unscreened, so a traversing entry read a spec.md outside the corpus and discharged the obligation from it — proved by deleting the screen and watching the gate pass, now screened through parse_feature_dir (f8ef4a4). Reuse: a third copy of the same whitespace-collapse chain, now one shared helper with three callers (f8ef4a4). Quality: data-model.md named a section of its own spec with the constitution sigil, unresolvable by construction (263a364). Nothing outstanding: 0 MUST, 0 SHOULD, 0 low-confidence.

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
