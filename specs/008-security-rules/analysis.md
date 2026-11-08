---
spec: 008-security-rules
last-run: 2026-11-08T15:16:51Z
analyzed-against: 4c25aeb677285e78910c2b8e5877c29750484cd5
hard-fail: 0
blocking-findings: 0
advisory: 6
unexamined: 2
analyzed-digest:
  data-model.md: b3c50297287cbc973c443e03f8f40f62dd8921d274b613aabf22bc87cdbc7acc
  plan.md: db5b34b0d805cdc616776a484ff7e734fc6a067ab2fe24bbd3ad07e775ceee51
  research.md: ed21f392dba2461ab97f92269f73dcaa38aaf924e16ce309427a316a797da9d4
  review.md: b1cf15c4ca9c0e2f768511518244263dae8ecaba2f91eb22202b77b7452acdde
  scenarios/a-custom-credential-header-does-not-follow-a-redirect.md: 2c25f447f5731eb05192956205e08b600cc355376fd35492fc47573d2ebe0b2f
  scenarios/a-statement-carries-one-obligation-keyword.md: 3a916668e2ebec5d37147cdfa4ca8715f6dda8158b509977b3769cf702f8fc69
  scenarios/an-operator-proxy-is-the-fetch-trust-boundary.md: db3d868a816c78dd41f62dd5bb08b44da41e0d694ea28097e8de53dcba753011
  scenarios/independent-review-findings-await-the-owner.md: 356a0ea3225449ddeff7e75966ade95c35af36f83e3187baf9ca05db8eed5c30
  scenarios/rules-are-as-short-as-their-required-content.md: f6afe0da29359d2c2ae5ef8f81930ddf7b82cb8fc02d53c7dcaaf237d1455705
  scenarios/single-host-appliance-rule-fit.md: 8f352bb3397cb2ccd81c841a72c74a19f6f125f8171485948969d337b1441a14
  scenarios/x-frame-options-carries-one-strength.md: 3f003bce779297c8d2af9d604d36f9d197534e28cfe7e835a2c4d5463ce5ee06
  spec.md: 27f754c6cdacb0173dbdc0c116c64fd5d87a5bac9d65556f56053182f540dd12
  tasks.md: 5d3b60904accbdc9a726b5c7099c001184f32847b0b22a05c71d5ef064bca99e
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 6
  undispositioned: 0
decisions:
  - key: grounding — 008's data model and spec signpost describe a rule's Verification as an instruction to /{project}:analyze, while quality-cross.md, which cites that data model for its schema, declares review-time Verifications
    outcome: waived
    reason: the constitution's §rules defines the field the same way and its Citable criterion admits a step a reviewer can check, which quality-cross.md's review-time Verifications follow; rewording the field definition across the constitution and two data models is a framing change beyond this run
    decided-at: 2026-10-20T15:53:03Z
    decided-by: andy@stone.dev
  - key: rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-DEPS-006; targeted for removal in 0.62.0
    outcome: waived
    reason: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03).
    decided-at: 2026-11-08T15:16:51Z
    decided-by: rule
  - key: rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-AUTHN-009; targeted for removal in 0.62.0
    outcome: waived
    reason: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03).
    decided-at: 2026-11-08T15:16:51Z
    decided-by: rule
  - key: rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-AUTHN-013; targeted for removal in 0.62.0
    outcome: waived
    reason: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03).
    decided-at: 2026-11-08T15:16:51Z
    decided-by: rule
  - key: rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-AUTHN-020; targeted for removal in 0.62.0
    outcome: waived
    reason: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03).
    decided-at: 2026-11-08T15:16:51Z
    decided-by: rule
  - key: artifact-size — `specs/008-security-rules/tasks.md` is 279938 bytes, 6 read pages at the 50000-byte read size
    outcome: waived
    reason: "Not mechanical now: its spent sections hold the decision records 079's task 12 and 077's records cite by task number, so pruning them is /ductus:prune's call once 008 is done; a reader takes the file by sections, as this run's review did."
    decided-at: 2026-11-08T15:16:51Z
    decided-by: rule
---

# Analysis — 008-security-rules

## Summary

0 hard-fail, 0 blocking, 6 advisory; not blocking. 2 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 6 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — 008's data model and spec signpost describe a rule's Verification as an instruction to /{project}:analyze, while quality-cross.md, which cites that data model for its schema, declares review-time Verifications — `specs/008-security-rules/data-model.md` — **waived**: the constitution's §rules defines the field the same way and its Citable criterion admits a step a reviewer can check, which quality-cross.md's review-time Verifications follow; rewording the field definition across the constitution and two data models is a framing change beyond this run
- rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-DEPS-006; targeted for removal in 0.62.0 — `specs/008-security-rules/spec.md` — **waived**: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03). (decided by the rule)
- rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-AUTHN-009; targeted for removal in 0.62.0 — `specs/008-security-rules/spec.md` — **waived**: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03). (decided by the rule)
- rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-AUTHN-013; targeted for removal in 0.62.0 — `specs/008-security-rules/spec.md` — **waived**: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03). (decided by the rule)
- rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-AUTHN-020; targeted for removal in 0.62.0 — `specs/008-security-rules/spec.md` — **waived**: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03). (decided by the rule)
- artifact-size — `specs/008-security-rules/tasks.md` is 279938 bytes, 6 read pages at the 50000-byte read size — `specs/008-security-rules/tasks.md` — **waived**: Not mechanical now: its spent sections hold the decision records 079's task 12 and 077's records cite by task number, so pruning them is /ductus:prune's call once 008 is done; a reader takes the file by sections, as this run's review did. (decided by the rule)

## Deferred to other specs

*None.*

## Unexamined targets

2 target(s) unexamined; no breakdown recorded.

## Fixed in this run

- grounding — plan.md's starter-set section says BE-DEPS carries the immutable pinning of build inputs a lockfile does not cover, which security-cross.md's SEC-DEPS-001 has carried since spec 077 — `specs/008-security-rules/plan.md` — **fixed** (decided by the rule)
