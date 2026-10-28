---
spec: 021-runtime-boundary
last-run: 2026-10-28T13:21:06Z
analyzed-against: 47e6823337874c604176cec2d730a332b3ae8923
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 1
analyzed-digest:
  plan.md: 70ef5c11c308b11b0b41d07775685dde6018979594d1dc23388284ea3f6d3fc6
  review.md: 36c851aa71f7812fc2081ffcab07a59c81a42f3522d64542816c8111fc709616
  spec.md: 7492b468640a3005852e47a14df7c0635004bd4ba2b66cf18229b46294f6d25a
  tasks.md: 662afd1f16e77ca2256ec3bcb25d7d074219c6f61fe362a56406a4375be68e05
unexamined-by-reason:
  ships-to-adopter: 1
blocking: false
dispositions:
  fixed: 2
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

0 hard-fail, 0 blocking, 1 advisory; not blocking. 1 unexamined target(s). Dispositions: 2 fixed, 1 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: plan.md's framework-checks.yml workflow commits to no automated vulnerability scan on dependency changes — `specs/021-runtime-boundary/plan.md` — **routed** to `specs/022-deterministic-runtime/tasks.md`

## Unexamined targets

- ships-to-adopter: 1

## Fixed in this run

- grounding — plan.md says AGENTS.md references a root constitution.md as a sync target, with no source, and AGENTS.md has no such reference — `specs/021-runtime-boundary/plan.md` — **fixed**
- grounding — plan.md describes lint-tool-coverage.sh's two exemptions with no source, and one of them, the Markdown-only reference section skip, no longer exists — `specs/021-runtime-boundary/plan.md` — **fixed**
