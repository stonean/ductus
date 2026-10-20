---
spec: 038-concurrency-backend-rules
last-run: 2026-10-20T00:52:03Z
analyzed-against: 709e043f00dc1f311d689fb8642f86b3bc0e5b7c
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 1
analyzed-digest:
  plan.md: cd84ff2856193cc78a01d3912ac043a07aab204251ba1d6896c6eb92699850ad
  review.md: 6887eec2b33fe8116aa6c635aa903c84fd80f86c9cf670f2d490a050fe943841
  spec.md: 283db126d7eb9142565855c61b43cc04df25f63c07462c024b77ba9ac5dfa749
  tasks.md: 86f9335290c68cb01110e06b398bf556aa0f6de90d05358f4228240a0aacf777
unexamined-by-reason:
  ships-to-adopter: 1
blocking: false
dispositions:
  fixed: 2
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 038-concurrency-backend-rules

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 1 unexamined target(s). Dispositions: 2 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

- ships-to-adopter: 1

## Fixed in this run

- anchor-resolution — spec.md:63 references §Trade-offs, a plan section, which resolves to no constitution marker — `specs/038-concurrency-backend-rules/spec.md` — **fixed**
- grounding — spec.md Motivation asserted in the present tense that the backend surface has zero concurrency coverage, which concurrency-backend.md falsified — `specs/038-concurrency-backend-rules/spec.md` — **fixed**
