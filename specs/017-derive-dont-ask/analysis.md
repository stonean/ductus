---
spec: 017-derive-dont-ask
last-run: 2026-11-20T04:30:21Z
analyzed-against: 705e9c968275137154a33a61c25d67850ce19127
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 28
analyzed-digest:
  data-model.md: dadd28edc909a348e9b5042e89d03c3f60f1d31c8de3eeb1ee0ca6c9a835574d
  plan.md: 24faf607e3f02cf5003243fde0af27351d668847fab383eced6560793c14b72a
  review.md: 762cb27c3849964749c5d54bf69c5f09338fea315e5eb17aaf27fe28af3fd82b
  scenarios/a-commit-stages-only-its-own-generated-files.md: 8626b701ad07371d0907c6a1a3a1ad4ee682c4d0d1622b3f503a76ae2bc189dc
  scenarios/a-secret-at-rest-is-flagged-in-every-project-shape.md: 6c52e86d521006fe5655e4a4fd704efad69ad5d9a6eec3bf70701b7962cdecf9
  scenarios/a-throwaway-credential-is-not-a-secret.md: e2167e26e43b1d9f1d6d0a6fb47928594a4c9b41d25d3b55bea4988c1a1e8fa5
  scenarios/detect-dependency-cycles.md: 799b70962ea172c7d17b580e8b90ba0f9b19ed89139dac4353fb8afd8c2c91a8
  scenarios/generator-sync-claim-honesty.md: 5be65fb004623287403f9f61b9d13fe8a7b8079d623ff9cd70c85aebf6d0ee65
  scenarios/ranged-and-inventoried-config-values-from-any-source.md: 8a6671d9417b0183141f771c438c8a7d985b69b82b23e476e3bd2ec602a0e963
  scenarios/required-config-values-from-any-source.md: a42163d6d945a1387ee92b834ece04f4430c4adb2bbd996cc44a16644b4dc463
  scenarios/skip-prose-cross-references.md: 7b080b9a2cfad2635b704f7418143cae520a151601b6d2a66368b0811e4a3c43
  scenarios/the-adopter-ci-template-can-run.md: f294181444f3bd340d48df644da28f5181e8301531d6ec87276a8894a0961cf5
  scenarios/tracked-specs-not-worktree.md: 8ad8da4efcd159e76d359d978541f585f2e53d63876d218a0c27b15068b19ace
  spec.md: 737e0a9a576d6da8d729c54971c843669a74e72a9cc05d6a7805306d20c730ec
  tasks.md: 6cc8abbd89d8aa43c227cb44720a16c3bd3ac062b680f8614bbf324d9fb630c8
unexamined-by-reason:
  anchor-target-unjudgeable: 1
  never-existed: 1
  no-readable-state: 1
  not-a-live-claim: 20
  root-absent: 2
  ships-to-adopter: 3
rule-digest:
  accessibility-frontend.md: 39a0edea7e2cad9ed30c6e3d5c2edccddfecd8a6b77151df4c71c9b20a901cd4
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 050a5b8dd375fe4790f5d92753db27ae53ac730db1557cbee84b923b72499ca9
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: 511a68b25ceb148f8292cda2a829aaff03a38bc3453732e33a70bd5d6acf5df5
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 5fddb21530faa805cb00f23c929e9402548ed0d6a0ed3e8e9a910561b74a45a3
rule-verdicts:
  BE-DATA-001: {verdict: addressed, cite: "plan.md#CI safety net"}
  BE-DATA-011: {verdict: addressed, cite: "plan.md#CI safety net"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#CI safety net"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#CI safety net"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#CI safety net"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "scenarios/generator-sync-claim-honesty.md#Behavior"}
  QUAL-GROUND-001: {verdict: addressed, cite: "scenarios/generator-sync-claim-honesty.md#Behavior"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Pre-commit hook architecture"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#CI safety net"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 0
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

0 hard-fail, 0 blocking, 0 advisory; not blocking. 28 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

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

*None.*
