---
spec: 036-quality-cross-rules
last-run: 2026-11-08T18:47:48Z
analyzed-against: 6bde245550fccbd34d03b7a3fc41e6dbcca97388
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 2
analyzed-digest:
  data-model.md: 9eee84f8a7bb752114eeab946005594c51cb5abc12339d8bee4831678e5f1499
  plan.md: 8c5d16d3cf975fc920da66bdb5a41ed481b26d0dc7451f543bac357ce2a1bd0c
  review.md: 5daaf05521f456aed791d570565dc06924a6cfdb0b85db0d89ba56a6594352b2
  scenarios/a-bound-test-pins-the-stated-figure.md: c3a43e07760d50fc4acdfb62624cdea16b6fdfc3802722965ee02d729fb85204
  spec.md: 145275462f88a32403ef8ca1af44237fba9a3b63993d9c2f536c83ed0fef06ee
  tasks.md: 3dd4c8ec9210c74608b5adc6a6424485b119b696c00dcdce6cea857b5c5ff4f3
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 036-quality-cross-rules

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 2 unexamined target(s). Dispositions: 2 fixed, 0 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Unexamined targets

2 target(s) unexamined; no breakdown recorded.

## Fixed in this run

- grounding — spec.md's Motivation says BE-SCHEMA-002 carries a fail-loud rationale, which the cited rule no longer has since 008's rules-are-as-short-as-their-required-content cut — `specs/036-quality-cross-rules/spec.md` — **fixed** (decided by the rule)
- grounding — plan.md's Cross-spec impact says the canonical-sources table's rule-file-format row carries per-surface registrations, which the cited row no longer holds since 1405eacc — `specs/036-quality-cross-rules/plan.md` — **fixed** (decided by the rule)
