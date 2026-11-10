---
spec: 055-shared-constitution
last-run: 2026-11-10T19:07:34Z
analyzed-against: 3602c737fda2c3fec362f3c25a3145a44c95c862
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 0
analyzed-digest:
  data-model.md: 15cf5405bd3d5f0dd859356aa83fb22d893d4c709b5a5e01895234a3f916fa40
  plan.md: 4b9c4e6d86f03a433ea5f40dd15ec7ba1df1c9dc9cc6e89ba2237ab252090276
  review.md: dd89d9c52b3209e8308cdc946d3f29d6a10bf5ceeb25e62a4dd55539984e14ca
  scenarios/a-registered-source-is-named-with-its-description.md: f04ffbef1f2349e9f23d8446c20490833b592ae5edb7dad11ed94bbe9c2166f3
  scenarios/target-reports-each-constitutions-size.md: 7661b1fccbf76a02af4467a2a40ebf845cfc302b6104f84b831ff25238b6bd3c
  spec.md: 68e0d756e4f584f845de34c5b7e50844189251146675227083f6475bedb59148
  tasks.md: 1aa60e99b162dfb9d282333e85b9aa53c9bd3b4483c29c317cd5fe78516bdf9f
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "rule-assessment — CFG-ENV-012: the loader's treatment of an unrecognized key in [constitutions.<alias>] is unstated"
    outcome: waived
    reason: "Unrecognized-key handling in .ductus/config.toml is project-wide runtime behaviour, not restated per section: the notice names a key inside a [constitutions.<alias>] table and the file, verified on a fixture (unknown config key: [constitutions.acme] descripton)."
    decided-at: 2026-11-10T00:59:00Z
    decided-by: rule
---

# Analysis — 055-shared-constitution

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 0 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — CFG-ENV-012: the loader's treatment of an unrecognized key in [constitutions.<alias>] is unstated — `specs/055-shared-constitution/data-model.md` — **waived**: Unrecognized-key handling in .ductus/config.toml is project-wide runtime behaviour, not restated per section: the notice names a key inside a [constitutions.<alias>] table and the file, verified on a fixture (unknown config key: [constitutions.acme] descripton). (decided by the rule)

## Deferred to other specs

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

*None.*
