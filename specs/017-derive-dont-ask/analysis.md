---
spec: 017-derive-dont-ask
last-run: 2026-10-28T20:22:39Z
analyzed-against: c1cf51ffa655635ef1b56b977023762f596c1aaa
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 23
analyzed-digest:
  data-model.md: 1564ce26c444116e8378804cdfb3182671007492a51e8ad0291576cbc14bf05d
  plan.md: 6b757eec906e3de87b12f6e3b6f64add08a42f077fb4fc703842332b8fc115e1
  review.md: 4074f7a92d1c14b037bf50bbbe76e7295000125b6ed31626d33873e55e315546
  scenarios/detect-dependency-cycles.md: 799b70962ea172c7d17b580e8b90ba0f9b19ed89139dac4353fb8afd8c2c91a8
  scenarios/generator-sync-claim-honesty.md: 1bd156f5abd11ebb6c854241092e5c428cec4d3601f2a9fb4c4545c997aa1fc9
  scenarios/ranged-and-inventoried-config-values-from-any-source.md: 873eeab49a775f68b5283737ad21952b98e771c5409c018df418f573b89b2041
  scenarios/required-config-values-from-any-source.md: a42163d6d945a1387ee92b834ece04f4430c4adb2bbd996cc44a16644b4dc463
  scenarios/skip-prose-cross-references.md: 7b080b9a2cfad2635b704f7418143cae520a151601b6d2a66368b0811e4a3c43
  scenarios/the-adopter-ci-template-can-run.md: 2d197440840c8229dc3dda3f93df9ebd73f2af5b491b9991db2c070b1f5b6c4c
  scenarios/tracked-specs-not-worktree.md: e3808ab1ae9a4d6b1b2103c85537c6236e99898b7569536eb9ac2bfcdde684bf
  spec.md: 975c7e5369de666b94098a83b8512dd3615795290427ef810d4ca466e525727d
  tasks.md: e5a5b10f24e9ae7f5a7a7446db8332cee34e69d71e56da8ae763f570fbea5a84
unexamined-by-reason:
  never-existed: 1
  no-readable-state: 1
  not-a-live-claim: 17
  root-absent: 1
  ships-to-adopter: 3
blocking: false
dispositions:
  fixed: 4
  routed: 0
  discarded: 4
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
  - key: "rule-assessment — FE-BUNDLE-003: plan.md's CI configuration includes no automated bundle-budget check that fails on exceedance"
    outcome: discarded
    reason: ductus ships no frontend bundle for a budget check to measure; the trigger fires only on its unqualified reading of CI configuration
    decided-at: 2026-10-28T20:14:13Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — FE-A11YTEST-003: plan.md's CI configuration includes no automated accessibility check on each change"
    outcome: discarded
    reason: ductus ships no frontend interface for an accessibility check to exercise; the trigger fires only on its unqualified reading of CI configuration
    decided-at: 2026-10-28T20:14:13Z
    decided-by: andy@stone.dev
---

# Analysis — 017-derive-dont-ask

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 23 unexamined target(s). Dispositions: 4 fixed, 0 routed, 4 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: plan.md's CI configuration commits to no automated dependency scan — `specs/017-derive-dont-ask/plan.md` — **discarded**: 017 introduces no manifest or lockfile and its workflows check generator drift, not dependency changes; the repository's per-change scan is committed in 022's plan, cargo audit in runtime.yml
- rule-assessment — FE-DEPS-006: plan.md's CI configuration commits to no frontend dependency scan — `specs/017-derive-dont-ask/plan.md` — **discarded**: ductus has no frontend dependency set for a scan to cover; the trigger fires only on its unqualified reading of CI configuration
- rule-assessment — FE-BUNDLE-003: plan.md's CI configuration includes no automated bundle-budget check that fails on exceedance — `specs/017-derive-dont-ask/plan.md` — **discarded**: ductus ships no frontend bundle for a budget check to measure; the trigger fires only on its unqualified reading of CI configuration
- rule-assessment — FE-A11YTEST-003: plan.md's CI configuration includes no automated accessibility check on each change — `specs/017-derive-dont-ask/plan.md` — **discarded**: ductus ships no frontend interface for an accessibility check to exercise; the trigger fires only on its unqualified reading of CI configuration

## Unexamined targets

- never-existed: 1
- no-readable-state: 1
- not-a-live-claim: 17
- root-absent: 1
- ships-to-adopter: 3

## Fixed in this run

- grounding — spec.md says no spec beyond this one is in flight, a point-in-time claim with no source — `specs/017-derive-dont-ask/spec.md` — **fixed**
- grounding — spec.md says its own frontmatter carries title and tags, which it does not — `specs/017-derive-dont-ask/spec.md` — **fixed**
- grounding — spec.md says /ductus still installs the constitution at the adopter root, with no source, though 044 moved it — `specs/017-derive-dont-ask/spec.md` — **fixed**
- grounding — plan.md says adopters continue to receive constitution.md at their root, with no source, though 044 moved it — `specs/017-derive-dont-ask/plan.md` — **fixed**
