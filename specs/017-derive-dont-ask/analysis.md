---
spec: 017-derive-dont-ask
last-run: 2026-11-19T14:13:24Z
analyzed-against: 52957e7fec67f9b7690f54e4b3e720ddc6cf4ac8
hard-fail: 0
blocking-findings: 0
advisory: 6
unexamined: 28
analyzed-digest:
  data-model.md: 6221b688b8c41291ac1c053202731955b40775d672e65258e91ab1525609322e
  plan.md: 2b4a879c5607673d9263f5884deeaf69f14ca7101b00f53ebbd4eba7280236fb
  review.md: 437895bd22bd8b3a9d3f6883bffb8bb99c8c5460a2be33d6963804f1c04f29ce
  scenarios/a-commit-stages-only-its-own-generated-files.md: 8626b701ad07371d0907c6a1a3a1ad4ee682c4d0d1622b3f503a76ae2bc189dc
  scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md: 6c52e86d521006fe5655e4a4fd704efad69ad5d9a6eec3bf70701b7962cdecf9
  scenarios/detect-dependency-cycles.md: 799b70962ea172c7d17b580e8b90ba0f9b19ed89139dac4353fb8afd8c2c91a8
  scenarios/generator-sync-claim-honesty.md: 5be65fb004623287403f9f61b9d13fe8a7b8079d623ff9cd70c85aebf6d0ee65
  scenarios/ranged-and-inventoried-config-values-from-any-source.md: 8a6671d9417b0183141f771c438c8a7d985b69b82b23e476e3bd2ec602a0e963
  scenarios/required-config-values-from-any-source.md: a42163d6d945a1387ee92b834ece04f4430c4adb2bbd996cc44a16644b4dc463
  scenarios/skip-prose-cross-references.md: 7b080b9a2cfad2635b704f7418143cae520a151601b6d2a66368b0811e4a3c43
  scenarios/the-adopter-ci-template-can-run.md: f294181444f3bd340d48df644da28f5181e8301531d6ec87276a8894a0961cf5
  scenarios/tracked-specs-not-worktree.md: 8ad8da4efcd159e76d359d978541f585f2e53d63876d218a0c27b15068b19ace
  spec.md: 7975a416dfb7695dc8b18d01c8d381f0445c5f7c32e4f9be019e6e88770b95d8
  tasks.md: 1ac915cf15fb9e883bf00c2b6fc6e4da2648ec0bcbcd06d88b57fbb66fe7e48e
unexamined-by-reason:
  anchor-target-unjudgeable: 1
  never-existed: 1
  no-readable-state: 1
  not-a-live-claim: 20
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
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-DATA-001: {verdict: addressed, cite: "plan.md#CI safety net"}
  BE-DATA-011: {verdict: addressed, cite: "plan.md#CI safety net"}
  BE-DEPS-007: {verdict: finding, tier: should, at: "plan.md#CI safety net", message: "plan.md's CI configuration commits to no automated dependency scan"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#CI safety net"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#CI safety net"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#CI safety net"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "scenarios/generator-sync-claim-honesty.md#Behavior"}
  QUAL-GROUND-001: {verdict: addressed, cite: "scenarios/generator-sync-claim-honesty.md#Outcome of that assessment (2026-08-16)"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Pre-commit hook architecture"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#CI safety net"}
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 6
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
  - key: "grounding — Resolved Questions, Q4: states that prose references to other specs are common and would flood the proposal with false positives, with nothing cited (waived in round 1 as decision reasoning; still unsourced)."
    outcome: waived
    reason: "A Resolved Question's premise, recorded with the decision it was taken on: the reasoning for treating bare slug mentions as commentary, not a claim about the system as it stands. Re-deriving how common such mentions are is a measurement, not a mechanical edit."
    decided-at: 2026-11-19T14:13:24Z
    decided-by: rule
  - key: "grounding — Resolved Questions, Q4: states that 'always write proper links' is a convention authors already follow, with no source cited (waived in round 1; still unsourced)."
    outcome: waived
    reason: "A Resolved Question's premise, recorded with the decision it was taken on: the reasoning for scanning inline links only, not a claim about the system as it stands."
    decided-at: 2026-11-19T14:13:24Z
    decided-by: rule
  - key: "grounding — Resolved Questions, Q3: states the first-commit diff window is equivalent to the planned-to-in-progress transition for clean pipelines, with no command or artifact cited (waived in round 1; still unsourced)."
    outcome: waived
    reason: "A Resolved Question's reasoning, which carries its argument inline: pre-implement work touches only the spec directory, which the filter excludes. The window it describes is `derive-boundary`'s, whose compiled description states it."
    decided-at: 2026-11-19T14:13:24Z
    decided-by: rule
  - key: "grounding — Trade-offs, 'Symmetric hooks': states drift between commits is a real failure mode, with no observed instance or artifact cited (waived in round 1; still unsourced)."
    outcome: waived
    reason: A trade-off's rationale naming its failure scenario (a teammate reads stale dependencies from an out-of-date branch without running a ductus command); it is the reason for the decision, not a claim about the system as it stands.
    decided-at: 2026-11-19T14:13:24Z
    decided-by: rule
---

# Analysis — 017-derive-dont-ask

## Summary

0 hard-fail, 0 blocking, 6 advisory; not blocking. 28 unexamined target(s). Dispositions: 2 fixed, 0 deferred, 6 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Resolved Questions, Q4: states that prose references to other specs are common and would flood the proposal with false positives, with nothing cited (waived in round 1 as decision reasoning; still unsourced). — `specs/017-derive-dont-ask/spec.md` — **waived**: A Resolved Question's premise, recorded with the decision it was taken on: the reasoning for treating bare slug mentions as commentary, not a claim about the system as it stands. Re-deriving how common such mentions are is a measurement, not a mechanical edit. (decided by the rule)
- grounding — Resolved Questions, Q4: states that 'always write proper links' is a convention authors already follow, with no source cited (waived in round 1; still unsourced). — `specs/017-derive-dont-ask/spec.md` — **waived**: A Resolved Question's premise, recorded with the decision it was taken on: the reasoning for scanning inline links only, not a claim about the system as it stands. (decided by the rule)
- grounding — Resolved Questions, Q3: states the first-commit diff window is equivalent to the planned-to-in-progress transition for clean pipelines, with no command or artifact cited (waived in round 1; still unsourced). — `specs/017-derive-dont-ask/spec.md` — **waived**: A Resolved Question's reasoning, which carries its argument inline: pre-implement work touches only the spec directory, which the filter excludes. The window it describes is `derive-boundary`'s, whose compiled description states it. (decided by the rule)
- grounding — Trade-offs, 'Symmetric hooks': states drift between commits is a real failure mode, with no observed instance or artifact cited (waived in round 1; still unsourced). — `specs/017-derive-dont-ask/plan.md` — **waived**: A trade-off's rationale naming its failure scenario (a teammate reads stale dependencies from an out-of-date branch without running a ductus command); it is the reason for the decision, not a claim about the system as it stands. (decided by the rule)
- scenario-consistency — ranged-and-inventoried-config-values-from-any-source: the Behavior section describes process ('each open question below is answered by the owner...') and states no requirement; the behavior lives only in Resolved Questions — `specs/017-derive-dont-ask/scenarios/ranged-and-inventoried-config-values-from-any-source.md` — **waived**: advisory and no chore: the contract is carried by the scenario's Resolved Questions (CFG-ENV-005 widened to any source; CFG-ENV-002 widened to configuration-file keys, flags left to help output) and by data-model.md's Rule set, so no requirement is lost; rewriting a scenario's Behavior is a requirement edit, not a mechanical chore (decided by the rule)
- rule-assessment — BE-DEPS-007: plan.md's CI configuration commits to no automated dependency scan — `specs/017-derive-dont-ask/plan.md` — **waived**: 017 introduces no manifest or lockfile and its workflows check generator drift, not dependency changes; the repository's per-change scan is committed in 022's plan, cargo audit in runtime.yml

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1
- never-existed: 1
- no-readable-state: 1
- not-a-live-claim: 20
- root-absent: 2
- ships-to-adopter: 3

## Fixed in this run

- grounding — Generators and Hooks, ductus repo's pre-commit hook: states that running generators unconditionally costs 'a fraction of a second per commit' with no measurement cited (the plan's Trade-offs repeats it as 'sub-second'). — `specs/017-derive-dont-ask/spec.md` — **fixed** (decided by the rule)
- grounding — Trade-offs, 'Adopter hook installs only when no existing hook system is detected': states adopter projects already use husky/lefthook/pre-commit-py as fact, with no source cited. — `specs/017-derive-dont-ask/plan.md` — **fixed** (decided by the rule)
