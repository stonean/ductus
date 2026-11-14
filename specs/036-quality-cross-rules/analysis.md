---
spec: 036-quality-cross-rules
last-run: 2026-11-14T03:20:34Z
analyzed-against: aebce3f506c01b340636c01222b76f06cbfea5fc
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 2
analyzed-digest:
  data-model.md: 9eee84f8a7bb752114eeab946005594c51cb5abc12339d8bee4831678e5f1499
  plan.md: 81f093ac4bdb941b7ad7c4a2139054edbcaf92f141e11a1dc9277ed158a35902
  review.md: 5daaf05521f456aed791d570565dc06924a6cfdb0b85db0d89ba56a6594352b2
  scenarios/a-bound-test-pins-the-stated-figure.md: 76496e655ec680babdb0a411e89961b392546ad8bdee7a6e93547feedba33557
  spec.md: 145275462f88a32403ef8ca1af44237fba9a3b63993d9c2f536c83ed0fef06ee
  tasks.md: d864743960789fdb3c1411b22b22175a30d943b26b7c7d9127652ad4c5547f06
unexamined-by-reason:
  root-absent: 2
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
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: grounding — the Motivating incident, an adopter's passthrough stub, is stated as fact with no citation or marker
    outcome: waived
    reason: the incident is an adopter's, whose tree is out of reach and whose name this repository never records (AGENTS.md), so no citation can exist; the spec already labels it an adopter incident, and no criterion depends on it
    decided-at: 2026-11-14T03:20:34Z
    decided-by: rule
  - key: scenario-consistency — Scenario lists four Edge Cases, above the three that constitution section scenario-promotion says warrants its own spec
    outcome: waived
    reason: the scenario adds one clause to QUAL-TEST-001 and its Resolved Questions reject a rule of its own; promoting it would mint a spec for a single Verification condition
    decided-at: 2026-11-14T03:20:34Z
    decided-by: rule
---

# Analysis — 036-quality-cross-rules

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 2 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — the Motivating incident, an adopter's passthrough stub, is stated as fact with no citation or marker — `specs/036-quality-cross-rules/spec.md` — **waived**: the incident is an adopter's, whose tree is out of reach and whose name this repository never records (AGENTS.md), so no citation can exist; the spec already labels it an adopter incident, and no criterion depends on it (decided by the rule)
- scenario-consistency — Scenario lists four Edge Cases, above the three that constitution section scenario-promotion says warrants its own spec — `specs/036-quality-cross-rules/scenarios/a-bound-test-pins-the-stated-figure.md` — **waived**: the scenario adds one clause to QUAL-TEST-001 and its Resolved Questions reject a rule of its own; promoting it would mint a spec for a single Verification condition (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- root-absent: 2

## Fixed in this run

- plan-consistency — Technical Decision 'New surface registration — scripts/lint-rule-ids.sh' states the edit but carries no reason inside the decision itself — `specs/036-quality-cross-rules/plan.md` — **fixed**
