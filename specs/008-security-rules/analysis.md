---
spec: 008-security-rules
last-run: 2026-11-20T04:35:50Z
analyzed-against: 5e5e33c30d4b02a1d068376c4a65645535204427
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 10
analyzed-digest:
  data-model.md: b3c50297287cbc973c443e03f8f40f62dd8921d274b613aabf22bc87cdbc7acc
  plan.md: e663d4ef533a5b0f15b37918a22b6a5ddffee3de093468d0285b374ebd030d16
  research.md: ed21f392dba2461ab97f92269f73dcaa38aaf924e16ce309427a316a797da9d4
  review.md: 6097a6939e1a99702cc42c525412af7d9d649416167fe09a62d180e2b6b4544a
  scenarios/a-custom-credential-header-does-not-follow-a-redirect.md: 2c25f447f5731eb05192956205e08b600cc355376fd35492fc47573d2ebe0b2f
  scenarios/a-statement-carries-one-obligation-keyword.md: 25130f7eb66ca03216927ef7293bbb0a1e55d9e26697be629c143b176d687e11
  scenarios/an-operator-proxy-is-the-fetch-trust-boundary.md: db3d868a816c78dd41f62dd5bb08b44da41e0d694ea28097e8de53dcba753011
  scenarios/independent-review-findings-await-the-owner.md: 356a0ea3225449ddeff7e75966ade95c35af36f83e3187baf9ca05db8eed5c30
  scenarios/rules-are-as-short-as-their-required-content.md: f6afe0da29359d2c2ae5ef8f81930ddf7b82cb8fc02d53c7dcaaf237d1455705
  scenarios/security-rules-flag-what-their-sources-support.md: c4160242eee5209251f179cb2e7a225baadabca63294a9e52d802669c21e0892
  scenarios/single-host-appliance-rule-fit.md: 8f352bb3397cb2ccd81c841a72c74a19f6f125f8171485948969d337b1441a14
  scenarios/x-frame-options-carries-one-strength.md: 3f003bce779297c8d2af9d604d36f9d197534e28cfe7e835a2c4d5463ce5ee06
  spec.md: 79ad4993823b33abf5e627aa5b6aaaaa008f9c1e1b846209055958ca10df3ae6
  tasks.md: 0d1112f459467211f1c8766662a6dc4a71fb74ba30f3bc4e8a82242781afbc70
unexamined-by-reason:
  not-a-live-claim: 4
  root-absent: 1
  ships-to-adopter: 5
rule-digest:
  accessibility-frontend.md: 39a0edea7e2cad9ed30c6e3d5c2edccddfecd8a6b77151df4c71c9b20a901cd4
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 75143f9bd0dec8d62b3a8ed592f9a8c29a4192d863330fb7a92424029878fa50
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: 511a68b25ceb148f8292cda2a829aaff03a38bc3453732e33a70bd5d6acf5df5
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 5fddb21530faa805cb00f23c929e9402548ed0d6a0ed3e8e9a910561b74a45a3
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
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
---

# Analysis — 008-security-rules

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 10 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-AUTHN-009; targeted for removal in 0.62.0 — `specs/008-security-rules/spec.md` — **waived**: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03). (decided by the rule)
- rule-citation — Spec at specs/008-security-rules/spec.md references deprecated rule BE-AUTHN-013; targeted for removal in 0.62.0 — `specs/008-security-rules/spec.md` — **waived**: 008's spec records the move of these rules to security-cross.md, in 077's signpost and its category descriptions, which keep the old IDs by design; their treatment once 0.62.0 removes the rules is deferred to 067's scenario a-record-cites-a-removed-rule, as 077's analysis records (owner, 2026-11-03). (decided by the rule)

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
