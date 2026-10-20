---
spec: 019-config-decisions
last-run: 2026-10-20T13:07:37Z
analyzed-against: 9d4d8c7685d1f1cc3fef5305456500521cbe6c96
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 2
analyzed-digest:
  data-model.md: d8bdc02dfbb5b9f2b299ece5704f596de6a3ab268008fa7860b9a7deff6cd480
  plan.md: 056676c551ef406927b69be8d34f60532e483c5dec87f152c276a46c8d084435
  review.md: 3fab79562d42da6fe98fe12ab7e7c9ee574e3b57b5c669b963b11bc1a88ac819
  scenarios/an-unknown-config-key-is-named.md: eb41ca8d0c1f6064da2407524a5e8e23a9b043a4a19dd067dd8cf0af6e0f8c34
  spec.md: a14af857c2be725b8f4ffd70a0515de3e9e514d91f8730a08da6dfeeabe786ad
  tasks.md: 8b99dc33b0dbdc10fd2d33f57fc39ba3eb8284463d2debb4a54b1b2bdd7362f5
unexamined-by-reason:
  not-a-live-claim: 1
  root-absent: 1
blocking: false
dispositions:
  fixed: 2
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 019-config-decisions

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 2 unexamined target(s). Dispositions: 2 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

- not-a-live-claim: 1
- root-absent: 1

## Fixed in this run

- rule-assessment — BE-INPUT-011: the stderr warning names a registered service checkout's config path, which comes from committed config, but only the key was committed to escaping — `specs/019-config-decisions/plan.md` — **fixed**
- grounding — plan.md says Paths::load runs on almost every primitive call, citing nothing — `specs/019-config-decisions/plan.md` — **fixed**
