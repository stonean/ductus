---
spec: 017-derive-dont-ask
last-run: 2026-11-14T13:41:18Z
analyzed-against: 545dde67ba90a5421ba9ad2ff52d3a51c7f27993
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 26
analyzed-digest:
  data-model.md: e479fd28e4ec72784c38ae901c289ded35029dc3045d3a9813672a7c5480379d
  plan.md: 7e2bc78dc657d176eec46311d6f8554c7cb3bbaf38132b46a43496a32a607605
  review.md: 097d7818bdee1185d83fe76a432243a4a34be078e2a985d945574c0a9298e9c5
  scenarios/a-commit-stages-only-its-own-generated-files.md: 8626b701ad07371d0907c6a1a3a1ad4ee682c4d0d1622b3f503a76ae2bc189dc
  scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md: 6c52e86d521006fe5655e4a4fd704efad69ad5d9a6eec3bf70701b7962cdecf9
  scenarios/detect-dependency-cycles.md: 799b70962ea172c7d17b580e8b90ba0f9b19ed89139dac4353fb8afd8c2c91a8
  scenarios/generator-sync-claim-honesty.md: 44623d33d0bc3e48b829d5f3417915673226ada11272da7a9dfc245462894c3e
  scenarios/ranged-and-inventoried-config-values-from-any-source.md: 8a6671d9417b0183141f771c438c8a7d985b69b82b23e476e3bd2ec602a0e963
  scenarios/required-config-values-from-any-source.md: a42163d6d945a1387ee92b834ece04f4430c4adb2bbd996cc44a16644b4dc463
  scenarios/skip-prose-cross-references.md: 7b080b9a2cfad2635b704f7418143cae520a151601b6d2a66368b0811e4a3c43
  scenarios/the-adopter-ci-template-can-run.md: f294181444f3bd340d48df644da28f5181e8301531d6ec87276a8894a0961cf5
  scenarios/tracked-specs-not-worktree.md: 8ad8da4efcd159e76d359d978541f585f2e53d63876d218a0c27b15068b19ace
  spec.md: ace25aa8ef38ac3229d584ef29a02f51027863f979dfd705f6ef921213f729ab
  tasks.md: d184f293048dbe248f95ec01c5849c7ecb90f0db8b06f79cced62e0f5df8c783
unexamined-by-reason:
  anchor-target-unjudgeable: 1
  never-existed: 1
  no-readable-state: 1
  not-a-live-claim: 18
  root-absent: 2
  ships-to-adopter: 3
rule-digest:
  accessibility-frontend.md: 5c1e7a9b5f625f14e6a1879c145ee30e25c5f5aff04ae4c853ee589e21b1efbb
  api-backend.md: 0caddd70fd1a546d80a5e77d254a38c36d158fcad464ee2493491a23cde06ea5
  concurrency-backend.md: 15209bb99ee1c7a63cfcde08a73f6de5df857834776cea43fadb7f0f1040370c
  configuration-cross.md: 2a807ba8facdfd1c3738383e6d44a1f6f2c6f91b387eac0ca5ed90d75dc9e178
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 0522a169c04113521bad128dfef283cd7e910264bf49c4a5320f27ba625fcdee
  security-backend.md: d89d6dad3ab51586ae4e68944b0c8e9ad4d5da721560f10590de3cdecc904d3f
  security-cross.md: dc98261801405fccaf5de9eb8f8ed9b020ebc78fcb5bfb124778a5fcc287950b
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-DATA-001: {verdict: addressed, cite: "plan.md#CI safety net"}
  BE-DATA-011: {verdict: addressed, cite: "plan.md#CI safety net"}
  BE-DEPS-007: {verdict: finding, tier: should, at: "plan.md#CI safety net", message: "The plan adds CI workflows (generators.yml, adopter-generators.yml) but commits to no automated vulnerability scan on dependency-set changes."}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#Marker-comment convention for generated content blocks"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Configuration rule ID format"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#Pre-commit hook architecture"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Pre-commit hook architecture"}
  BE-INPUT-008: {verdict: addressed, cite: "data-model.md#Frontmatter schema (after this spec)"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "scenarios/generator-sync-claim-honesty.md#Behavior"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#CI safety net"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#CI safety net"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#CI safety net"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#CI safety net"}
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 3
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-DEPS-006: plan.md introduces a CI workflow and a shipped CI template without committing to immutable pinning of their actions or to an update process for the pins"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-20T16:32:24Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — BE-DEPS-007: plan.md's CI configuration commits to no automated dependency scan"
    outcome: waived
    reason: 017 introduces no manifest or lockfile and its workflows check generator drift, not dependency changes; the repository's per-change scan is committed in 022's plan, cargo audit in runtime.yml
    decided-at: 2026-10-20T16:32:24Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — FE-DEPS-006: plan.md's CI configuration commits to no frontend dependency scan"
    outcome: waived
    reason: ductus has no frontend dependency set for a scan to cover; the trigger fires only on its unqualified reading of CI configuration
    decided-at: 2026-10-20T16:32:24Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — FE-BUNDLE-003: plan.md's CI configuration includes no automated bundle-budget check that fails on exceedance"
    outcome: waived
    reason: ductus ships no frontend bundle for a budget check to measure; the trigger fires only on its unqualified reading of CI configuration
    decided-at: 2026-10-28T20:14:13Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — FE-A11YTEST-003: plan.md's CI configuration includes no automated accessibility check on each change"
    outcome: waived
    reason: ductus ships no frontend interface for an accessibility check to exercise; the trigger fires only on its unqualified reading of CI configuration
    decided-at: 2026-10-28T20:14:13Z
    decided-by: andy@stone.dev
  - key: "scenario-consistency — ranged-and-inventoried-config-values-from-any-source: the Behavior section describes process ('each open question below is answered by the owner...') and states no requirement; the behavior lives only in Resolved Questions"
    outcome: waived
    reason: "advisory and no chore: the contract is carried by the scenario's Resolved Questions (CFG-ENV-005 widened to any source; CFG-ENV-002 widened to configuration-file keys, flags left to help output) and by data-model.md's Rule set, so no requirement is lost; rewriting a scenario's Behavior is a requirement edit, not a mechanical chore"
    decided-at: 2026-11-14T13:41:18Z
    decided-by: rule
  - key: rule-citation — Spec at specs/017-derive-dont-ask/spec.md references deprecated rule BE-DEPS-006; targeted for removal in 0.62.0
    outcome: waived
    reason: "advisory and no chore: 017's spec records SEC-DEPS-001 taking over the deprecated BE-DEPS-006 beside 077's obligation, a record of the move that keeps the old ID by design, as 008's analysis waived the same citation; its treatment once 0.62.0 removes the rule is 067's scenario a-record-cites-a-removed-rule"
    decided-at: 2026-11-14T13:41:18Z
    decided-by: rule
---

# Analysis — 017-derive-dont-ask

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 26 unexamined target(s). Dispositions: 3 fixed, 0 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: The plan adds CI workflows (generators.yml, adopter-generators.yml) but commits to no automated vulnerability scan on dependency-set changes. — `specs/017-derive-dont-ask/plan.md` — **waived**: 017 introduces no manifest or lockfile and its workflows check generator drift, not dependency changes; the repository's per-change scan is committed in 022's plan, cargo audit in runtime.yml
- scenario-consistency — ranged-and-inventoried-config-values-from-any-source: the Behavior section describes process ('each open question below is answered by the owner...') and states no requirement; the behavior lives only in Resolved Questions — `specs/017-derive-dont-ask/scenarios/ranged-and-inventoried-config-values-from-any-source.md` — **waived**: advisory and no chore: the contract is carried by the scenario's Resolved Questions (CFG-ENV-005 widened to any source; CFG-ENV-002 widened to configuration-file keys, flags left to help output) and by data-model.md's Rule set, so no requirement is lost; rewriting a scenario's Behavior is a requirement edit, not a mechanical chore (decided by the rule)
- rule-citation — Spec at specs/017-derive-dont-ask/spec.md references deprecated rule BE-DEPS-006; targeted for removal in 0.62.0 — `specs/017-derive-dont-ask/spec.md` — **waived**: advisory and no chore: 017's spec records SEC-DEPS-001 taking over the deprecated BE-DEPS-006 beside 077's obligation, a record of the move that keeps the old ID by design, as 008's analysis waived the same citation; its treatment once 0.62.0 removes the rule is 067's scenario a-record-cites-a-removed-rule (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1
- never-existed: 1
- no-readable-state: 1
- not-a-live-claim: 18
- root-absent: 2
- ships-to-adopter: 3

## Fixed in this run

- rule-assessment — BE-PAYLOAD-001: derive-dependencies/derive-references return lists (examined, untracked-skipped, unwritten, updated, cycles) bounded only by spec count, with no committed maximum size or lists-file bound in spec, plan or data model — `specs/017-derive-dont-ask/scenarios/generator-sync-claim-honesty.md` — **fixed**
- grounding — Q2 states the two constitutions were 'byte-identical at decision time' and the plan says their historical divergence 'was zero', with no command or method cited — `specs/017-derive-dont-ask/spec.md` — **fixed** (decided by the rule)
- grounding — Plan's rejected-alternative claim that 'git worktree add fails on an unborn HEAD' and a killed hook 'leaves a registered worktree in .git/worktrees' carries no command or citation — `specs/017-derive-dont-ask/plan.md` — **fixed** (decided by the rule)
