---
spec: 017-derive-dont-ask
last-run: 2026-10-20T16:32:24Z
analyzed-against: 6855e5bf265493fb824ec91f804e03445d99e41c
hard-fail: 0
blocking-findings: 1
advisory: 2
unexamined: 20
analyzed-digest:
  data-model.md: 59aa84e8bc06cb7526e20597cbeb672066cfbe1e8fa9e70a0f3db6f97b4a756b
  plan.md: f92ac215def0ca22c22ceb5880b61492509f2c39ff0d780a7e7ca46d7538d333
  review.md: 78f4298cb2382bb386464ec77f67774deeb75453ea7140ee0d6ea6d8ea2d0156
  scenarios/detect-dependency-cycles.md: 799b70962ea172c7d17b580e8b90ba0f9b19ed89139dac4353fb8afd8c2c91a8
  scenarios/generator-sync-claim-honesty.md: 1bd156f5abd11ebb6c854241092e5c428cec4d3601f2a9fb4c4545c997aa1fc9
  scenarios/ranged-and-inventoried-config-values-from-any-source.md: 873eeab49a775f68b5283737ad21952b98e771c5409c018df418f573b89b2041
  scenarios/required-config-values-from-any-source.md: a42163d6d945a1387ee92b834ece04f4430c4adb2bbd996cc44a16644b4dc463
  scenarios/skip-prose-cross-references.md: 7b080b9a2cfad2635b704f7418143cae520a151601b6d2a66368b0811e4a3c43
  scenarios/tracked-specs-not-worktree.md: e3808ab1ae9a4d6b1b2103c85537c6236e99898b7569536eb9ac2bfcdde684bf
  spec.md: 0af969e32ce5386863260b4c5f909faa86f237191ad8d92faa23b2ecfb20c7cc
  tasks.md: fc3ab4ce2800f6cba866b4d0911c220435662738df9ecbfa7c05ea25aba118f2
unexamined-by-reason:
  never-existed: 1
  no-readable-state: 1
  not-a-live-claim: 16
  ships-to-adopter: 2
blocking: true
dispositions:
  fixed: 1
  routed: 1
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

0 hard-fail, 1 blocking, 2 advisory; blocking. 20 unexamined target(s). Dispositions: 1 fixed, 1 routed, 2 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

- rule-assessment — BE-DEPS-006: plan.md introduces a CI workflow and a shipped CI template without committing to immutable pinning of their actions or to an update process for the pins — `specs/017-derive-dont-ask/plan.md` — **routed** to `specs/022-deterministic-runtime/tasks.md`

## Advisory findings

- rule-assessment — BE-DEPS-007: plan.md's CI configuration commits to no automated dependency scan — `specs/017-derive-dont-ask/plan.md` — **discarded**: 017 introduces no manifest or lockfile and its workflows check generator drift, not dependency changes; the repository's per-change scan is committed in 022's plan, cargo audit in runtime.yml
- rule-assessment — FE-DEPS-006: plan.md's CI configuration commits to no frontend dependency scan — `specs/017-derive-dont-ask/plan.md` — **discarded**: ductus has no frontend dependency set for a scan to cover; the trigger fires only on its unqualified reading of CI configuration

## Unexamined targets

- never-existed: 1
- no-readable-state: 1
- not-a-live-claim: 16
- ships-to-adopter: 2

## Fixed in this run

- grounding — spec.md's Q6 note said two of the first version's examples no longer hold, while CFG-ENV-002's widening on 2026-10-20 made a third out of date — `specs/017-derive-dont-ask/spec.md` — **fixed**
