---
spec: 030-cross-service-references
last-run: 2026-10-16T19:36:00Z
reviewed-against: ec72c234476d97f7057f68d83c6e92f9e88eb94b
diff-base: 6bbc7fb1d9ccd226e5ea77036d8c3471c0008aa9
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 3
scope: 54
skipped-passes: []
reviewed-digest:
  data-model.md: e1b1a7edfa47be545f6f6acfb2de765f32392a32c4b67d7ab58130d4ed3fe325
  scenarios/referenced-service-spec-root.md: eb2dd76dce8f7f1d319535d29235b12024fa43dee6de9bcf2d8e7c5a1dfe1fff
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 030-cross-service-references

## Summary

Scoped re-review of the body-path round's rewrite to current (26664d5d). examined: 3: data-model.md and the scenario referenced-service-spec-root.md in full, and spec.md as its window diff. The scenario's new pointer was checked against runtime/src/primitives/derive_references.rs, whose module doc carries the Root-aware matching heading it names. The five passes ran over the files counted as examined, against all eleven rule files and AGENTS.md, and found no finding and no observation. Not read and not counted: every other scope entry - the plan's Affected Files, unchanged in this window, and the files other specs' amendments and 067's tasks 10-11 changed after the base (including runtime/src/primitives/derive_dependencies.rs unless named above), which are the subject of their own reviews.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
