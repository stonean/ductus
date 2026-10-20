---
spec: 008-security-rules
last-run: 2026-10-20T15:53:03Z
analyzed-against: 17a86a76a5ff9e7266c4ed39957343e3f89ce8f3
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 9
analyzed-digest:
  data-model.md: 8194b8d2f1806419df0b01ce21a48232606090b043cb75b70a23ac891bb086e7
  plan.md: c12080c74d37b53f9401d034050911de181b5130406b28a79ba49a6a1677117f
  research.md: c66fcc8f01a105c459ee6838831919bc52a983632a21f3f89d499f1f47e34e6c
  review.md: e2b989700caaf7a8d55e086577508172ceab7a39658c50867eddbb3f40f89498
  scenarios/a-statement-carries-one-obligation-keyword.md: f24f63017a687d400cffdbee325302ee21158f0a9aa0a4d122f24f83a0bd3742
  scenarios/independent-review-findings-await-the-owner.md: 96446a0d9c76fe28a3fe2b1532ad0bb3fbf3912ed48e2cdfa13d129fa78e9a99
  scenarios/x-frame-options-carries-one-strength.md: 8cf5c99537d8f9cf81233184b4b13d76f7c3bc66a0ce96f0effb075572e53fc4
  spec.md: c491a85ccc6636fee832f6218ece507613e5e4825ac404c71836fa09bdbf572c
  tasks.md: 719d1a1ec7dd338b382d8df47f5a458f6c8378ccc65c6322ed3efa01bdb5e032
unexamined-by-reason:
  not-a-live-claim: 4
  ships-to-adopter: 5
blocking: false
dispositions:
  fixed: 2
  routed: 0
  discarded: 1
  undispositioned: 0
decisions:
  - key: grounding — 008's data model and spec signpost describe a rule's Verification as an instruction to /{project}:analyze, while quality-cross.md, which cites that data model for its schema, declares review-time Verifications
    outcome: discarded
    reason: the constitution's §rules defines the field the same way and its Citable criterion admits a step a reviewer can check, which quality-cross.md's review-time Verifications follow; rewording the field definition across the constitution and two data models is a framing change beyond this run
    decided-at: 2026-10-20T15:53:03Z
    decided-by: andy@stone.dev
---

# Analysis — 008-security-rules

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 9 unexamined target(s). Dispositions: 2 fixed, 0 routed, 1 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — 008's data model and spec signpost describe a rule's Verification as an instruction to /{project}:analyze, while quality-cross.md, which cites that data model for its schema, declares review-time Verifications — `specs/008-security-rules/data-model.md` — **discarded**: the constitution's §rules defines the field the same way and its Citable criterion admits a step a reviewer can check, which quality-cross.md's review-time Verifications follow; rewording the field definition across the constitution and two data models is a framing change beyond this run

## Unexamined targets

- not-a-live-claim: 4
- ships-to-adopter: 5

## Fixed in this run

- grounding — spec.md's Edge Cases quote the unknown- and deprecated-rule citation messages, which analyze.md had not stated since 022's parseable rewrite (11aad341) — `specs/008-security-rules/spec.md` — **fixed**
- grounding — plan.md placed the brownfield security audit in framework/bootstrap/ductus.md, which 056 moved to ductus-procedure.md — `specs/008-security-rules/plan.md` — **fixed**
