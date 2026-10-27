---
spec: 017-derive-dont-ask
last-run: 2026-10-27T06:49:21Z
analyzed-against: 98a2eae0e909d19a50b246d127006f8440535e75
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 21
analyzed-digest:
  data-model.md: 59aa84e8bc06cb7526e20597cbeb672066cfbe1e8fa9e70a0f3db6f97b4a756b
  plan.md: 29defd4a365b5dc78ea9676b803688822be263c252c850770ff121c71471d64d
  review.md: b88e8ffaf117f07514ca81fbb974deb016bc9297e7c0aecb5a128de1132ad93d
  scenarios/detect-dependency-cycles.md: 799b70962ea172c7d17b580e8b90ba0f9b19ed89139dac4353fb8afd8c2c91a8
  scenarios/generator-sync-claim-honesty.md: 1bd156f5abd11ebb6c854241092e5c428cec4d3601f2a9fb4c4545c997aa1fc9
  scenarios/ranged-and-inventoried-config-values-from-any-source.md: 873eeab49a775f68b5283737ad21952b98e771c5409c018df418f573b89b2041
  scenarios/required-config-values-from-any-source.md: a42163d6d945a1387ee92b834ece04f4430c4adb2bbd996cc44a16644b4dc463
  scenarios/skip-prose-cross-references.md: 7b080b9a2cfad2635b704f7418143cae520a151601b6d2a66368b0811e4a3c43
  scenarios/the-adopter-ci-template-can-run.md: 2d197440840c8229dc3dda3f93df9ebd73f2af5b491b9991db2c070b1f5b6c4c
  scenarios/tracked-specs-not-worktree.md: e3808ab1ae9a4d6b1b2103c85537c6236e99898b7569536eb9ac2bfcdde684bf
  spec.md: bdf719e807beb28e86ac10946686ec55e6a2e5e03a1d395ec3f25034183668bd
  tasks.md: e29de105359213c63a4ec1805bc904282714f5b443cb74d447939e32944d4d64
unexamined-by-reason:
  never-existed: 1
  no-readable-state: 1
  not-a-live-claim: 16
  root-absent: 1
  ships-to-adopter: 2
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 2
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-DEPS-006: plan.md introduces a CI workflow and a shipped CI template without committing to immutable pinning of their actions or to an update process for the pins"
    outcome: routed
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-20T16:32:24Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — BE-DEPS-007: plan.md's CI configuration commits to no automated dependency scan"
    outcome: discarded
    reason: 017 introduces no manifest or lockfile and its workflows check generator drift, not dependency changes; the repository's per-change scan is committed in 022's plan, cargo audit in runtime.yml
    decided-at: 2026-10-20T16:32:24Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — FE-DEPS-006: plan.md's CI configuration commits to no frontend dependency scan"
    outcome: discarded
    reason: ductus has no frontend dependency set for a scan to cover; the trigger fires only on its unqualified reading of CI configuration
    decided-at: 2026-10-20T16:32:24Z
    decided-by: andy@stone.dev
---

# Analysis — 017-derive-dont-ask

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 21 unexamined target(s). Dispositions: 0 fixed, 0 routed, 2 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: plan.md's CI configuration commits to no automated dependency scan — `specs/017-derive-dont-ask/plan.md` — **discarded**: 017 introduces no manifest or lockfile and its workflows check generator drift, not dependency changes; the repository's per-change scan is committed in 022's plan, cargo audit in runtime.yml
- rule-assessment — FE-DEPS-006: plan.md's CI configuration commits to no frontend dependency scan — `specs/017-derive-dont-ask/plan.md` — **discarded**: ductus has no frontend dependency set for a scan to cover; the trigger fires only on its unqualified reading of CI configuration

## Unexamined targets

- never-existed: 1
- no-readable-state: 1
- not-a-live-claim: 16
- root-absent: 1
- ships-to-adopter: 2

## Fixed in this run

*None.*
