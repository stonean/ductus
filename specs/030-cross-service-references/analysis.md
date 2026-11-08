---
spec: 030-cross-service-references
last-run: 2026-11-08T00:58:26Z
analyzed-against: 8339e0f67c811f424936f698031bf4f0f6887bf2
hard-fail: 0
blocking-findings: 3
advisory: 0
unexamined: 2
analyzed-digest:
  data-model.md: 5b2a061d3c39858d28b00d1732787fcea4e6af2ed5336e959789edcdd362836e
  plan.md: 217ecf6b97a91ddcc09707457b4ae00e2a43083f19620a896879b40e57a693e6
  review.md: 78754f929556ee6e43e0a67461e0daf977694083c24f3cabc74c09cedbf6d580
  scenarios/referenced-service-spec-root.md: c76e9244f75881073a432932610036a6f425830df413a593c1e50c65c1b7dd13
  spec.md: 12978c34e5b5c0109abc86e374b4bfa0e8b02abd7ecd1c5d8b3d6b0b6f676a05
  tasks.md: 180b35dbcce3f60ac9c48c3427b527792d143b570f0539b9f7c892b7f6e61090
blocking: true
dispositions:
  fixed: 1
  deferred: 3
  waived: 0
  undispositioned: 0
decisions:
  - key: "rule-assessment — CFG-ENV-003: the data model makes `[services.<alias>]` `repo` and `path` required, and no artifact commits to rejecting an absent or empty one with an error naming the key and the file"
    outcome: deferred
    target: specs/030-cross-service-references/tasks.md
    task: "15"
    gating: true
    decided-at: 2026-11-08T00:58:26Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — CFG-ENV-008: the data model names the `[services]` values' types, and no artifact commits to a fail-fast parse of a malformed config"
    outcome: deferred
    target: specs/030-cross-service-references/tasks.md
    task: "15"
    gating: true
    decided-at: 2026-11-08T00:58:26Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — BE-PAYLOAD-001: resolve-references returns its references and duplicate-repos lists inline, and no artifact commits to a maximum response size or pagination"
    outcome: deferred
    target: specs/030-cross-service-references/tasks.md
    task: "15"
    gating: true
    decided-at: 2026-11-08T00:58:26Z
    decided-by: andy@stone.dev
---

# Analysis — 030-cross-service-references

## Summary

0 hard-fail, 3 blocking, 0 advisory; blocking. 2 unexamined target(s). Dispositions: 1 fixed, 3 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

- rule-assessment — CFG-ENV-003: the data model makes `[services.<alias>]` `repo` and `path` required, and no artifact commits to rejecting an absent or empty one with an error naming the key and the file — `specs/030-cross-service-references/data-model.md` — **deferred** to `specs/030-cross-service-references/tasks.md` task 15
- rule-assessment — CFG-ENV-008: the data model names the `[services]` values' types, and no artifact commits to a fail-fast parse of a malformed config — `specs/030-cross-service-references/data-model.md` — **deferred** to `specs/030-cross-service-references/tasks.md` task 15
- rule-assessment — BE-PAYLOAD-001: resolve-references returns its references and duplicate-repos lists inline, and no artifact commits to a maximum response size or pagination — `specs/030-cross-service-references/data-model.md` — **deferred** to `specs/030-cross-service-references/tasks.md` task 15

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Unexamined targets

2 target(s) unexamined; no breakdown recorded.

## Fixed in this run

- rule-assessment — CFG-ENV-012: the plan introduces the `[services]` section without stating how its loader treats an unrecognized key — `specs/030-cross-service-references/plan.md` — **fixed** (decided by the rule)
