---
spec: 047-analyze-findings-durability
last-run: 2026-11-11T23:02:26Z
analyzed-against: 9e33f4b70f6f228cc96a42f1163d88ece524f5b4
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 4
analyzed-digest:
  plan.md: 592f5e53543c982a820734e3409480499e2522d3540746030d0dfab02d5e1af6
  review.md: d7006ec755b88fb38a11862d96ad8c897a6019e9f72837a49090fcbb35c3a270
  scenarios/analyze-record-freshness.md: 0b7e45cdb3b02bf29796dbb88f370ca8572c20dfa87bf7373a5f90ba6162c627
  scenarios/analyze-run-durability.md: 57c6da00840c4165108adc30e20c52f7161229d0faa32fec876f904446a16f23
  spec.md: ae7e8a6ec58f312ac167e6f6ee5180fbd104fb2e61c43c8186afd2c0f3ee09ed
  tasks.md: 2707ab1b5159c614bdce108e197f5d58428f85b2837220da6b4ad869a6b9f101
unexamined-by-reason:
  no-readable-state: 1
  ships-to-adopter: 3
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 047-analyze-findings-durability

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 4 unexamined target(s). Dispositions: 2 fixed, 0 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Unexamined targets

- no-readable-state: 1
- ships-to-adopter: 3

## Fixed in this run

- rule-assessment — BE-COMPAT-001: 047's scenarios add check-review-gate's not-analyzed, analyze-findings and analyze-stale block reasons, and no 047 artifact classifies the change as additive or breaking — `specs/047-analyze-findings-durability/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-PAYLOAD-001: analyze-record-freshness has the gate name the changed paths, and no 047 artifact commits to that list's bound — `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md` — **fixed** (decided by the rule)
