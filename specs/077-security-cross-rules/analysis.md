---
spec: 077-security-cross-rules
last-run: 2026-11-04T03:46:07Z
analyzed-against: 65ec2d7f70c6ac37f7ea71b5486d20108575c435
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 0
analyzed-digest:
  data-model.md: cbf317f3d8635097191d82c1b4a9cc35aedd9275dd026eaeb68a9685f241be0d
  plan.md: fdfe23e5ed5c775bda496f1a7e0bdf73bd7cd2f9e0cf251e1de59f269a344d28
  review.md: 849d702700c49b296c7cf7a9c9be27381ebe3437cb4524a461f40b3c4aea45a3
  spec.md: 59fb87a32818e61288be2ed52283c29196d0d1e4f0d410c2fd5fb9b77ffc32f6
  tasks.md: f1e4152fb2b0f0c34e7b307c2a7c334d52354bff7f8e0a1c9a491c04b89075e9
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: rule-citation — Spec at specs/077-security-cross-rules/spec.md references deprecated rule BE-DEPS-006; targeted for removal in 0.62.0
    outcome: waived
    reason: 077's spec is the record of the deprecation it makes, which keeps the old ID; its treatment once 0.62.0 removes the rule is deferred to 067's scenario a-record-cites-a-removed-rule (owner, 2026-11-03, option a).
    decided-at: 2026-11-04T03:46:07Z
    decided-by: andy@stone.dev
  - key: rule-citation — Spec at specs/077-security-cross-rules/spec.md references deprecated rule BE-AUTHN-009; targeted for removal in 0.62.0
    outcome: waived
    reason: 077's spec is the record of the deprecation it makes, which keeps the old ID; its treatment once 0.62.0 removes the rule is deferred to 067's scenario a-record-cites-a-removed-rule (owner, 2026-11-03, option a).
    decided-at: 2026-11-04T03:46:07Z
    decided-by: andy@stone.dev
  - key: rule-citation — Spec at specs/077-security-cross-rules/spec.md references deprecated rule BE-AUTHN-013; targeted for removal in 0.62.0
    outcome: waived
    reason: 077's spec is the record of the deprecation it makes, which keeps the old ID; its treatment once 0.62.0 removes the rule is deferred to 067's scenario a-record-cites-a-removed-rule (owner, 2026-11-03, option a).
    decided-at: 2026-11-04T03:46:07Z
    decided-by: andy@stone.dev
  - key: rule-citation — Spec at specs/077-security-cross-rules/spec.md references deprecated rule BE-AUTHN-020; targeted for removal in 0.62.0
    outcome: waived
    reason: 077's spec is the record of the deprecation it makes, which keeps the old ID; its treatment once 0.62.0 removes the rule is deferred to 067's scenario a-record-cites-a-removed-rule (owner, 2026-11-03, option a).
    decided-at: 2026-11-04T03:46:07Z
    decided-by: andy@stone.dev
---

# Analysis — 077-security-cross-rules

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 0 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 4 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-citation — Spec at specs/077-security-cross-rules/spec.md references deprecated rule BE-DEPS-006; targeted for removal in 0.62.0 — `specs/077-security-cross-rules/spec.md` — **waived**: 077's spec is the record of the deprecation it makes, which keeps the old ID; its treatment once 0.62.0 removes the rule is deferred to 067's scenario a-record-cites-a-removed-rule (owner, 2026-11-03, option a).
- rule-citation — Spec at specs/077-security-cross-rules/spec.md references deprecated rule BE-AUTHN-009; targeted for removal in 0.62.0 — `specs/077-security-cross-rules/spec.md` — **waived**: 077's spec is the record of the deprecation it makes, which keeps the old ID; its treatment once 0.62.0 removes the rule is deferred to 067's scenario a-record-cites-a-removed-rule (owner, 2026-11-03, option a).
- rule-citation — Spec at specs/077-security-cross-rules/spec.md references deprecated rule BE-AUTHN-013; targeted for removal in 0.62.0 — `specs/077-security-cross-rules/spec.md` — **waived**: 077's spec is the record of the deprecation it makes, which keeps the old ID; its treatment once 0.62.0 removes the rule is deferred to 067's scenario a-record-cites-a-removed-rule (owner, 2026-11-03, option a).
- rule-citation — Spec at specs/077-security-cross-rules/spec.md references deprecated rule BE-AUTHN-020; targeted for removal in 0.62.0 — `specs/077-security-cross-rules/spec.md` — **waived**: 077's spec is the record of the deprecation it makes, which keeps the old ID; its treatment once 0.62.0 removes the rule is deferred to 067's scenario a-record-cites-a-removed-rule (owner, 2026-11-03, option a).

## Deferred to other specs

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

*None.*
