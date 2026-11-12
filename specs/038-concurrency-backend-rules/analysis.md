---
spec: 038-concurrency-backend-rules
last-run: 2026-11-12T00:32:34Z
analyzed-against: ab2ad0853b359dfe50598581bc9302daf093ef2a
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 1
analyzed-digest:
  plan.md: c1b586d4834516f588d58398694110e5cd241934f0324c320c742d2e76a42d54
  review.md: ecc107cdd9a5b8397557d1266acd9f89ab24a815aca13a7ba95fecbe2d395a18
  scenarios/lock-the-parent-before-rewriting-its-children.md: 81701adaf3813bbd84be93d692c6f65e309c58822c3229991be923461aef601b
  spec.md: 313f15d6f67726a9f1b992f8e3c9917e278fcd154f58d780200ae09713f8ea85
  tasks.md: 08215aadb71d3f50a37367b73656ffc3720c9256b8f6565733f250b16fd14ac7
unexamined-by-reason:
  ships-to-adopter: 1
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 038-concurrency-backend-rules

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 1 unexamined target(s). Dispositions: 2 fixed, 0 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Unexamined targets

- ships-to-adopter: 1

## Fixed in this run

- grounding — plan.md §Technical Decisions, "Categories disjoint": "verified: RACE/LOCK/TXN/COORD appear in none" names no command, query or artifact — `specs/038-concurrency-backend-rules/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md §Overview: "The §Shared Files note is already count-free (036)" cites only a spec number for a claim about framework/bootstrap/ductus.md — `specs/038-concurrency-backend-rules/plan.md` — **fixed** (decided by the rule)
