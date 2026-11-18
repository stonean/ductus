---
spec: 007-govern-workflow
last-run: 2026-11-18T21:53:05Z
analyzed-against: 0fbd9952d0c0e98286f7fdf931069ed08301c716
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 7
analyzed-digest:
  plan.md: 327c15f9ddc6eaeb265384fb6ebbb912805cc24ee6a23105468c175e70cdb84e
  review.md: c3d2824f685ba9ff921d7d5318385a774b4019168308c2e05d248c108a51323c
  scenarios/a-first-adoption-with-existing-specs-is-not-an-update.md: d99d7193f73d0109c94c7afd0aa5b36dc3177d6660b4f032542b9b3aa4d289c9
  scenarios/ductus-self-update-precheck.md: dc1def4b407b5791d61473d00aa941aca25c3ac0e695237ada00cc7eeb14500c
  spec.md: 9ec1b02511fd0ef02b74cae216b6a76ee1811abdd05eae112362680f1310fb81
  tasks.md: 608d8a75e4bc35ddf386b0a03728c6723139d33b94cac7ded61f004e9fea12f6
unexamined-by-reason:
  root-absent: 1
  ships-to-adopter: 6
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
  BE-BULK-002: {verdict: addressed, cite: "spec.md#File Fetching"}
  BE-DATA-001: {verdict: addressed, cite: "spec.md#Distribution Model"}
  BE-INPUT-002: {verdict: addressed, cite: "spec.md#Inputs"}
  BE-INPUT-004: {verdict: finding, tier: must, at: "plan.md#Paths built from the inputs are contained", message: "The plan now commits to canonicalization and containment through apply-manifest and enforce-manifest, but states that a step whose primitive cannot run falls back to a shell specification carrying none of those screens, so user-named paths are written uncontained on that path."}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#Every fetch is bounded"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#Every fetch is bounded"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#Every fetch is bounded"}
  CFG-ENV-007: {verdict: addressed, cite: "spec.md#Inputs"}
  CFG-ENV-008: {verdict: addressed, cite: "spec.md#Edge Cases"}
blocking: false
dispositions:
  fixed: 4
  deferred: 1
  waived: 0
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-INPUT-004: The plan now commits to canonicalization and containment through apply-manifest and enforce-manifest, but states that a step whose primitive cannot run falls back to a shell specification carrying none of those screens, so user-named paths are written uncontained on that path."
    outcome: deferred
    target: specs/029-bootstrap-runtime-autowire/tasks.md
    task: "17"
    reason: 029 owns the per-step fallback a step whose primitive cannot run takes (scenario state-a-deterministic-path-forcing, AC20); its task 17 keeps each screened primitive's path screens on that route (owner, 2026-11-18)
    decided-at: 2026-11-18T21:53:05Z
    decided-by: andy@stone.dev
---

# Analysis — 007-govern-workflow

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 7 unexamined target(s). Dispositions: 4 fixed, 1 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Deferred to other specs

- rule-assessment — BE-INPUT-004: The plan now commits to canonicalization and containment through apply-manifest and enforce-manifest, but states that a step whose primitive cannot run falls back to a shell specification carrying none of those screens, so user-named paths are written uncontained on that path. — `specs/007-govern-workflow/plan.md` — **deferred** to `specs/029-bootstrap-runtime-autowire/tasks.md` task 17

## Contradictions

*None.*

## Unexamined targets

- root-absent: 1
- ships-to-adopter: 6

## Fixed in this run

- plan-consistency — The 'Gitignore language patterns fetched from GitHub' Technical Decision said what is done but gave no reason for the choice (§Spec integrity, rationale). — `specs/007-govern-workflow/plan.md` — **fixed** (decided by the rule)
- grounding — The 'separate command template directories per CLI' Trade-off said the commands are identical except for path prefixes, with no citation or assumption marker (§Grounding (advisory)). — `specs/007-govern-workflow/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-TIMEOUT-001: The per-language gitignore fetch (plan) and the bootstrap self-update fetch (scenario ductus-self-update-precheck) are outbound network calls that name no bounding timeout. — `specs/007-govern-workflow/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-INPUT-004: User-named spec root and project name (and the config-file specs-root and name) become written paths, but the spec commits only to a character allowlist, never to canonicalization and a whole-component containment check. — `specs/007-govern-workflow/spec.md` — **fixed** (decided by the rule)
