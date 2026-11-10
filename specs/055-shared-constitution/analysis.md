---
spec: 055-shared-constitution
last-run: 2026-11-10T00:59:00Z
analyzed-against: e8f1762716e0eb88ad4d0e49dca8f0ff7d8088a9
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 2
analyzed-digest:
  data-model.md: f9318bb945d78c3c9b13a3bc1333c02742584a6a96789638725abc17f915d748
  plan.md: 9af17b40126d6b4549d8e44aa67792efecbd2562eecdf5c56561587fad471218
  review.md: f1b6c8c7750533cddf3a6f9b231f0b3c18548c01c5b6a7288958ff3a95c68fff
  scenarios/a-registered-source-is-named-with-its-description.md: f04ffbef1f2349e9f23d8446c20490833b592ae5edb7dad11ed94bbe9c2166f3
  scenarios/target-reports-each-constitutions-size.md: 7661b1fccbf76a02af4467a2a40ebf845cfc302b6104f84b831ff25238b6bd3c
  spec.md: 77700017a2c80fa59d85452f2f4277fb88bac2babdc6011393fca3f102813f9f
  tasks.md: d2eb90623c7f6071bcfd96dd5cdb9ca7eb0b229ac96b7c1616888aa4ee2746cc
blocking: false
dispositions:
  fixed: 9
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

0 hard-fail, 0 blocking, 1 advisory; not blocking. 2 unexamined target(s). Dispositions: 9 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — CFG-ENV-012: the loader's treatment of an unrecognized key in [constitutions.<alias>] is unstated — `specs/055-shared-constitution/data-model.md` — **waived**: Unrecognized-key handling in .ductus/config.toml is project-wide runtime behaviour, not restated per section: the notice names a key inside a [constitutions.<alias>] table and the file, verified on a fixture (unknown config key: [constitutions.acme] descripton). (decided by the rule)

## Deferred to other specs

*None.*

## Unexamined targets

2 target(s) unexamined; no breakdown recorded.

## Fixed in this run

- rule-assessment — CFG-ENV-003: no artifact committed a malformed registry entry's error to naming the alias, the field and .ductus/config.toml — `specs/055-shared-constitution/data-model.md` — **fixed**
- rule-assessment — BE-PAYLOAD-001: resolve-constitutions' result stated no bound, its description only whitespace-collapsed — `specs/055-shared-constitution/data-model.md` — **fixed**
- rule-assessment — BE-COMPAT-001: no line classified the reopening's change to resolve-constitutions' arguments and result — `specs/055-shared-constitution/data-model.md` — **fixed**
- grounding — the plan quoted a §grounding sentence the constitution no longer holds — `specs/055-shared-constitution/plan.md` — **fixed** (decided by the rule)
- grounding — the plan said the precedence statement lands in the config-schema documentation, which the Resolved Question reversed — `specs/055-shared-constitution/plan.md` — **fixed** (decided by the rule)
- grounding — the plan's examined/skipped precedent claim cited nothing — `specs/055-shared-constitution/plan.md` — **fixed** (decided by the rule)
- grounding — rule-file registration by placement cited nothing — `specs/055-shared-constitution/spec.md` — **fixed** (decided by the rule)
- grounding — the [pinned] files behaviour in the Motivation cited nothing — `specs/055-shared-constitution/spec.md` — **fixed** (decided by the rule)
- grounding — the duplicate-path edge case's /ductus:link precedent cited nothing — `specs/055-shared-constitution/spec.md` — **fixed** (decided by the rule)
