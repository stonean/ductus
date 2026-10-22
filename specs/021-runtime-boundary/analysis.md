---
spec: 021-runtime-boundary
last-run: 2026-10-22T13:16:08Z
analyzed-against: 9052af26fa71410ec6f8d002959b48a481730b20
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 0
analyzed-digest:
  plan.md: 52f08bb8714724326222c6ff90455f2da0f613b8fe1e13e41cbf3c307b092bc6
  review.md: 50e7f9be3ecb83e0c250d5f4dcbf702b8294ef402cab8f0cfe76dcc8c4129894
  spec.md: f72d304ae17a05308ca027b51a89b5b90719336535abfd45c79795637bae8a06
  tasks.md: 662afd1f16e77ca2256ec3bcb25d7d074219c6f61fe362a56406a4375be68e05
blocking: false
dispositions:
  fixed: 4
  routed: 1
  discarded: 0
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-DEPS-007: plan.md's framework-checks.yml workflow commits to no automated vulnerability scan on dependency changes"
    outcome: routed
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-22T13:16:08Z
    decided-by: andy@stone.dev
---

# Analysis — 021-runtime-boundary

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 0 unexamined target(s). Dispositions: 4 fixed, 1 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: plan.md's framework-checks.yml workflow commits to no automated vulnerability scan on dependency changes — `specs/021-runtime-boundary/plan.md` — **routed** to `specs/022-deterministic-runtime/tasks.md`

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

- rule-assessment — BE-DEPS-006: plan.md's framework-checks.yml workflow commits to no immutable pinning of its inputs and no update process for the pins — `specs/021-runtime-boundary/plan.md` — **fixed**
- rule-assessment — BE-DEPS-004: plan.md's framework-checks.yml workflow names no signature or provenance verification for the markdownlint-cli2 it installs from npm or the crates its runtime build fetches — `specs/021-runtime-boundary/plan.md` — **fixed**
- rule-assessment — BE-TIMEOUT-001: plan.md's framework-checks.yml workflow names no timeout bounding its registry fetches — `specs/021-runtime-boundary/plan.md` — **fixed**
- rule-assessment — CFG-CONST-003: plan.md writes the tool-coverage lint's tunable 20-line window as a bare literal and commits to no named constant — `specs/021-runtime-boundary/plan.md` — **fixed**
