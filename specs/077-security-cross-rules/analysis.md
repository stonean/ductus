---
spec: 077-security-cross-rules
last-run: 2026-11-08T06:37:34Z
analyzed-against: b138c78f9af009a49376a1fb604f723332ae7597
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 2
analyzed-digest:
  data-model.md: 7c47616784eef9e266659f9323cbfea1a710519df6d959dc8f056c2a026efce6
  plan.md: 03252f18bd14a9246ae292a764153ea0aca834f522b4defc2ae810d1beaa31e7
  review.md: 45af5d9fcc85be5465da9d8cd4a2d896e35672e9b579ec0339ac2f225f58c5f2
  spec.md: cd772d6af2c33bab59f7ccabff6506b6530a00d3e4fb1f99a542ee942f85e1a2
  tasks.md: 959b6c0f334e229097e4f3fd943b90ce3a9fd542e09b702238d5ac572dc46c2e
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

0 hard-fail, 0 blocking, 4 advisory; not blocking. 2 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 4 waived, 0 undispositioned.

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

2 target(s) unexamined; no breakdown recorded.

## Fixed in this run

*None.*
