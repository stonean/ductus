---
spec: 008-security-rules
last-run: 2026-11-18T12:49:35Z
analyzed-against: 9cae484cf4b41ad6e9d3de3eb4d6fe675cc1e551
hard-fail: 0
blocking-findings: 0
advisory: 7
unexamined: 10
analyzed-digest:
  data-model.md: b3c50297287cbc973c443e03f8f40f62dd8921d274b613aabf22bc87cdbc7acc
  plan.md: 4c2eab134c7f28fe9e15e97a32ff2bb573047c6777eec26ef34d09c48caa6826
  research.md: ed21f392dba2461ab97f92269f73dcaa38aaf924e16ce309427a316a797da9d4
  review.md: b1cf15c4ca9c0e2f768511518244263dae8ecaba2f91eb22202b77b7452acdde
  scenarios/a-custom-credential-header-does-not-follow-a-redirect.md: 2c25f447f5731eb05192956205e08b600cc355376fd35492fc47573d2ebe0b2f
  scenarios/a-statement-carries-one-obligation-keyword.md: 25130f7eb66ca03216927ef7293bbb0a1e55d9e26697be629c143b176d687e11
  scenarios/an-operator-proxy-is-the-fetch-trust-boundary.md: db3d868a816c78dd41f62dd5bb08b44da41e0d694ea28097e8de53dcba753011
  scenarios/independent-review-findings-await-the-owner.md: 356a0ea3225449ddeff7e75966ade95c35af36f83e3187baf9ca05db8eed5c30
  scenarios/rules-are-as-short-as-their-required-content.md: f6afe0da29359d2c2ae5ef8f81930ddf7b82cb8fc02d53c7dcaaf237d1455705
  scenarios/single-host-appliance-rule-fit.md: 8f352bb3397cb2ccd81c841a72c74a19f6f125f8171485948969d337b1441a14
  scenarios/x-frame-options-carries-one-strength.md: 3f003bce779297c8d2af9d604d36f9d197534e28cfe7e835a2c4d5463ce5ee06
  spec.md: a69bcaaa7047e07a60a3a076895a6a48915011faca8d0475d74f9fbea193568f
  tasks.md: 5d55937b6f0233289d9242992e36919438719874906a8153b4f54e27ecc459e6
unexamined-by-reason:
  not-a-live-claim: 4
  root-absent: 1
  ships-to-adopter: 5
rule-digest:
  accessibility-frontend.md: 5c1e7a9b5f625f14e6a1879c145ee30e25c5f5aff04ae4c853ee589e21b1efbb
  api-backend.md: 0caddd70fd1a546d80a5e77d254a38c36d158fcad464ee2493491a23cde06ea5
  concurrency-backend.md: 15209bb99ee1c7a63cfcde08a73f6de5df857834776cea43fadb7f0f1040370c
  configuration-cross.md: 2a807ba8facdfd1c3738383e6d44a1f6f2c6f91b387eac0ca5ed90d75dc9e178
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 0522a169c04113521bad128dfef283cd7e910264bf49c4a5320f27ba625fcdee
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 7
  undispositioned: 0
decisions:
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
  - key: grounding — plan.md Coverage rationale states as fact that native deserialization (Java ObjectInputStream, Python Pickle, .NET BinaryFormatter) and type-instantiating parsers enable RCE, with no citation
    outcome: waived
    reason: "the section's lead names its source, a gap audit against OWASP Top 10 and OWASP API Security Top 10, and each bullet's rule carries its OWASP cheat-sheet Source: line in framework/rules/security-backend.md, so the claim is sourced; a per-bullet citation would restate the rule file"
    decided-at: 2026-11-18T12:49:35Z
    decided-by: rule
  - key: grounding — plan.md Coverage rationale states that default XML parser configurations frequently allow external entity expansion and that Jinja/Twig RCE on user input in templates, uncited
    outcome: waived
    reason: "the section's lead names its source, a gap audit against OWASP Top 10 and OWASP API Security Top 10, and each bullet's rule carries its OWASP cheat-sheet Source: line in framework/rules/security-backend.md, so the claim is sourced; a per-bullet citation would restate the rule file"
    decided-at: 2026-11-18T12:49:35Z
    decided-by: rule
  - key: grounding — plan.md Coverage rationale states that frameworks binding request bodies to model fields without an allowlist let users set admin flags, uncited
    outcome: waived
    reason: "the section's lead names its source, a gap audit against OWASP Top 10 and OWASP API Security Top 10, and each bullet's rule carries its OWASP cheat-sheet Source: line in framework/rules/security-backend.md, so the claim is sourced; a per-bullet citation would restate the rule file"
    decided-at: 2026-11-18T12:49:35Z
    decided-by: rule
---

# Analysis — 008-security-rules

## Summary

0 hard-fail, 0 blocking, 7 advisory; not blocking. 10 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 7 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-AUTHN-009; targeted for removal in 0.62.0 — `specs/008-security-rules/spec.md` — **waived**: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03). (decided by the rule)
- rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-AUTHN-013; targeted for removal in 0.62.0 — `specs/008-security-rules/spec.md` — **waived**: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03). (decided by the rule)
- rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-AUTHN-020; targeted for removal in 0.62.0 — `specs/008-security-rules/spec.md` — **waived**: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03). (decided by the rule)
- rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-DEPS-006; targeted for removal in 0.62.0 — `specs/008-security-rules/spec.md` — **waived**: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03). (decided by the rule)
- grounding — plan.md Coverage rationale states as fact that native deserialization (Java ObjectInputStream, Python Pickle, .NET BinaryFormatter) and type-instantiating parsers enable RCE, with no citation — `specs/008-security-rules/plan.md` — **waived**: the section's lead names its source, a gap audit against OWASP Top 10 and OWASP API Security Top 10, and each bullet's rule carries its OWASP cheat-sheet Source: line in framework/rules/security-backend.md, so the claim is sourced; a per-bullet citation would restate the rule file (decided by the rule)
- grounding — plan.md Coverage rationale states that default XML parser configurations frequently allow external entity expansion and that Jinja/Twig RCE on user input in templates, uncited — `specs/008-security-rules/plan.md` — **waived**: the section's lead names its source, a gap audit against OWASP Top 10 and OWASP API Security Top 10, and each bullet's rule carries its OWASP cheat-sheet Source: line in framework/rules/security-backend.md, so the claim is sourced; a per-bullet citation would restate the rule file (decided by the rule)
- grounding — plan.md Coverage rationale states that frameworks binding request bodies to model fields without an allowlist let users set admin flags, uncited — `specs/008-security-rules/plan.md` — **waived**: the section's lead names its source, a gap audit against OWASP Top 10 and OWASP API Security Top 10, and each bullet's rule carries its OWASP cheat-sheet Source: line in framework/rules/security-backend.md, so the claim is sourced; a per-bullet citation would restate the rule file (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 4
- root-absent: 1
- ships-to-adopter: 5

## Fixed in this run

*None.*
