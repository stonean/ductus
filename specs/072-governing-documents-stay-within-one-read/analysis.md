---
spec: 072-governing-documents-stay-within-one-read
last-run: 2026-10-28T18:53:01Z
analyzed-against: d1853444f01e2623b1b4bc7b92629fed3dde2825
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 4
analyzed-digest:
  plan.md: c638b765a744e7eefdd3f5a12d634adebd55897e5fb989ba52ace2edc39d5b27
  review.md: 1012b1ce8132b3021f6d93802f36d537409aa81a04876eee634e4ad11749ac04
  spec.md: 7216860f604d0f2491ecd631ada98ea86173197ca137190c5a403db7e69863c6
  tasks.md: 8de966c99a1186616700cbb3de162464d8915591204d5640f2fdd24a7c70bb8e
unexamined-by-reason:
  root-absent: 4
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 4
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-DEPS-007: plan §Wiring changes CI configuration and commits to no automated vulnerability scan on dependency changes"
    outcome: discarded
    reason: 072 adds or changes no dependency manifest or lockfile; step (o) runs two repo-local bash scripts and installs nothing; cargo audit runs in runtime.yml and 022's plan commits the per-change scan, the reason 017's analysis recorded for the same finding
    decided-at: 2026-10-26T21:55:35Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — FE-DEPS-006: plan §Wiring changes CI configuration and commits to no frontend dependency scan"
    outcome: discarded
    reason: ductus has no frontend dependency set for a scan to cover; the trigger fires only on its unqualified reading of CI configuration
    decided-at: 2026-10-28T18:53:01Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — FE-BUNDLE-003: plan §Wiring changes CI configuration and includes no automated bundle-budget check"
    outcome: discarded
    reason: ductus ships no frontend bundle for a budget check to measure; the trigger fires only on its unqualified reading of CI configuration
    decided-at: 2026-10-28T18:53:01Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — FE-A11YTEST-003: plan §Wiring changes CI configuration and includes no automated accessibility check"
    outcome: discarded
    reason: ductus ships no frontend interface for an accessibility check to exercise; the trigger fires only on its unqualified reading of CI configuration
    decided-at: 2026-10-28T18:53:01Z
    decided-by: andy@stone.dev
---

# Analysis — 072-governing-documents-stay-within-one-read

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 4 unexamined target(s). Dispositions: 0 fixed, 0 routed, 4 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: plan §Wiring changes CI configuration and commits to no automated vulnerability scan on dependency changes — `specs/072-governing-documents-stay-within-one-read/plan.md` — **discarded**: 072 adds or changes no dependency manifest or lockfile; step (o) runs two repo-local bash scripts and installs nothing; cargo audit runs in runtime.yml and 022's plan commits the per-change scan, the reason 017's analysis recorded for the same finding
- rule-assessment — FE-DEPS-006: plan §Wiring changes CI configuration and commits to no frontend dependency scan — `specs/072-governing-documents-stay-within-one-read/plan.md` — **discarded**: ductus has no frontend dependency set for a scan to cover; the trigger fires only on its unqualified reading of CI configuration
- rule-assessment — FE-BUNDLE-003: plan §Wiring changes CI configuration and includes no automated bundle-budget check — `specs/072-governing-documents-stay-within-one-read/plan.md` — **discarded**: ductus ships no frontend bundle for a budget check to measure; the trigger fires only on its unqualified reading of CI configuration
- rule-assessment — FE-A11YTEST-003: plan §Wiring changes CI configuration and includes no automated accessibility check — `specs/072-governing-documents-stay-within-one-read/plan.md` — **discarded**: ductus ships no frontend interface for an accessibility check to exercise; the trigger fires only on its unqualified reading of CI configuration

## Unexamined targets

- root-absent: 4

## Fixed in this run

*None.*
