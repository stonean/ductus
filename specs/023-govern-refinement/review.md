---
spec: 023-govern-refinement
reviewed-at: 2026-08-19T01:07:52Z
reviewed-against: 9c06b2dfd5f16618c50fd3a0186caf534a517778
diff-base: 38b97b7413e04a6bf5e7f6dd712a7c60e7862f95
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 023-govern-refinement

## Summary

Clean — 0 MUST, 0 SHOULD, 0 low-confidence, 0 observations.

Scope: one edit, the supersession annotation on AC32. The criterion asserted that `.github/workflows/markdown-only-pipeline.yml` passes with the runtime absent from `PATH`; `048-govern-acquired-runtime` made the runtime required and acquired, retiring that invariant, and renamed the workflow to `framework-checks.yml`.

The criterion stays ticked because it *was* delivered — the removal belongs to the later spec, per §spec-requirements — and the annotation names the superseding spec by name rather than by link, so citing a remover does not harvest a dependency edge. The half of the criterion that still holds (the runtime CI workflow continues to pass) is called out as still holding rather than being blanket-superseded.

No code, no behavior, no other artifact touched.

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
