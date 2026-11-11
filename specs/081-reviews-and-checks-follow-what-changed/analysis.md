---
spec: 081-reviews-and-checks-follow-what-changed
last-run: 2026-11-11T22:46:20Z
analyzed-against: 58878cfb5d8efcb7e0afd5294ff697f323461858
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 7
analyzed-digest:
  plan.md: 6c047eb6c3e2481d6709fe8c3c5d2bb4be04d04fbc9c94fd72553319ad61a00e
  review.md: cdbfc4d2d5317506d3319f785330cba6fb617c428c00bbcaf268b92b1fe64848
  spec.md: b942b1e4276fc343c225b694e7c6eb15ed5a6aeb8ce7e075cbc1855d3f42f21d
  tasks.md: cf529d49459789676be9e6d635065c2d4a617c9e82c20dcfde35382f4f26c96a
unexamined-by-reason:
  not-a-live-claim: 3
  root-absent: 4
blocking: false
dispositions:
  fixed: 10
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 081-reviews-and-checks-follow-what-changed

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 7 unexamined target(s). Dispositions: 10 fixed, 0 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Unexamined targets

- not-a-live-claim: 3
- root-absent: 4

## Fixed in this run

- rule-assessment — SEC-DEPS-001: plan.md's CI changes commit to no immutable pinning of their inputs or update process for the pins — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-SCHEMA-002: plan.md modifies three primitives' tool contracts and names no single source for their schemas — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-DEPS-007: plan.md's CI changes commit to no automated scan on dependency changes — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- body-path-existence — names `scripts/audit/review-freshness.sh`, which existed in this repository and no longer resolves — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says the constitution is 49,914 bytes, present tense and with no commit or method — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says a release lags main by 1,083 commits, with no method — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says the deleted Family 19 header records 42 of 48, present tense and with no commit — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says check-review-freshness has no other caller, with no method — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — spec.md says 6158d7df was 1,083 commits past ductus-v0.59.11, with no method — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md says 079's spec.md was 87,348 bytes, with no commit or method — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **fixed** (decided by the rule)
