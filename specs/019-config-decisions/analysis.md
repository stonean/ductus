---
spec: 019-config-decisions
last-run: 2026-11-14T02:18:25Z
analyzed-against: 172d315670d87621fab6375ae9d39e285db03394
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 2
analyzed-digest:
  data-model.md: 4987aa08476438494a0c8351e96db5c6cd577e5359cf9baaf945f48fe63726aa
  plan.md: 38e09b797fc24498f818db73768404b9b4a663b7d49708058b1f5d8502ad75f2
  review.md: bc875f21810d4dbde73a13852232782c8e485ce2a2c94aa5214e8097c9f5128e
  scenarios/an-unknown-config-key-is-named.md: eb41ca8d0c1f6064da2407524a5e8e23a9b043a4a19dd067dd8cf0af6e0f8c34
  spec.md: 8f3531736641e0a4e4ca8c786bc4144622a1bf5fb967e054644a9fda7bf94d1c
  tasks.md: dfd167de04cc4e95dd769c9ff89f3c3e312af25ba59f56a9f7de09c50ceb1466
unexamined-by-reason:
  not-a-live-claim: 1
  root-absent: 1
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
  BE-DEPS-007: {verdict: finding, tier: should, at: "plan.md#Affected Files", message: "The plan modifies .github/workflows/runtime.yml (CI configuration) and commits to no automated vulnerability scan on dependency-set changes."}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Validation is per key, at the point of use"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Unknown keys are reported by name, never dropped"}
  BE-INPUT-008: {verdict: addressed, cite: "data-model.md#File location and lifecycle"}
  BE-INPUT-011: {verdict: addressed, cite: "plan.md#Unknown keys are reported by name, never dropped"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#Unknown keys are reported by name, never dropped"}
  BE-LOG-002: {verdict: addressed, cite: "plan.md#Unknown keys are reported by name, never dropped"}
  BE-PAYLOAD-001: {verdict: finding, tier: must, at: "plan.md#Unknown keys are reported by name, never dropped", message: "dashboard's new config.unknown-keys list and the unknown-key notices added to discover-rule-files and check-artifact-size are lists bounded only by the config file's key count; the plan commits to no maximum or read-size-bytes/lists-file bound for them"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#Unknown keys are reported by name, never dropped"}
  BE-RACE-002: {verdict: addressed, cite: "plan.md#Unknown keys are reported by name, never dropped"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#Unknown keys are reported by name, never dropped"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#Unknown keys are reported by name, never dropped"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Unknown keys are reported by name, never dropped"}
  CFG-ENV-012: {verdict: addressed, cite: "plan.md#Unknown keys are reported by name, never dropped"}
blocking: false
dispositions:
  fixed: 1
  deferred: 1
  waived: 4
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-DEPS-007: The plan modifies .github/workflows/runtime.yml (CI configuration) and commits to no automated vulnerability scan on dependency-set changes."
    outcome: waived
    reason: "advisory and no chore: 019's plan changes runtime.yml only to add framework/bootstrap/ductus.md to its trigger paths, so no dependency set changes; cargo audit runs in runtime.yml on every runtime/** change (as 024's analysis waived the same finding)"
    decided-at: 2026-11-14T02:18:25Z
    decided-by: rule
  - key: "rule-assessment — BE-PAYLOAD-001: dashboard's new config.unknown-keys list and the unknown-key notices added to discover-rule-files and check-artifact-size are lists bounded only by the config file's key count; the plan commits to no maximum or read-size-bytes/lists-file bound for them"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    task: "32"
    reason: "the owner deferred it to 022's task 32 on 2026-11-14: dashboard and discover-rule-files are 022's primitives (019's plan, The runtime half is owned by 022 and 063), and check-artifact-size's notices were bounded by 063's task 17"
    decided-at: 2026-11-14T02:18:25Z
    decided-by: andy@stone.dev
  - key: decision-reasons — Plan Trade-off 'Over MCP, stderr reaches the server log' rejects a notice list on every reader because it 'would have reopened 063, 062, 055, 030 and 067'; the reopens are the stated reason.
    outcome: waived
    reason: "advisory and no chore: the owner's 2026-10-20 decision (the scenario's Resolved Questions) chose where unknown keys are reported, and the trade-off also states the result the chosen option gives up; rewording its reason would re-decide it, which is the owner's"
    decided-at: 2026-11-14T02:18:25Z
    decided-by: rule
  - key: grounding — Spec intro states 'config.toml is currently a single-purpose pin file' as present fact with no citation; it is a pre-delivery description now superseded.
    outcome: waived
    reason: "advisory and no chore: the intro states the problem as it stood before this spec, under the spec's own note marking it historical; the fix is a rewrite of the motivation, not a citation"
    decided-at: 2026-11-14T02:18:25Z
    decided-by: rule
  - key: grounding — Resolved Question 'Validation rules' asserts the framework's existing posture for config.toml (no commit hook, no validate rule, summary-only enforcement) without a citation.
    outcome: waived
    reason: "advisory and no chore: a Resolved Question's account of the posture when it was decided; data-model.md's Schema validation section states the current posture with its sources"
    decided-at: 2026-11-14T02:18:25Z
    decided-by: rule
---

# Analysis — 019-config-decisions

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 2 unexamined target(s). Dispositions: 1 fixed, 1 deferred, 4 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: The plan modifies .github/workflows/runtime.yml (CI configuration) and commits to no automated vulnerability scan on dependency-set changes. — `specs/019-config-decisions/plan.md` — **waived**: advisory and no chore: 019's plan changes runtime.yml only to add framework/bootstrap/ductus.md to its trigger paths, so no dependency set changes; cargo audit runs in runtime.yml on every runtime/** change (as 024's analysis waived the same finding) (decided by the rule)
- decision-reasons — Plan Trade-off 'Over MCP, stderr reaches the server log' rejects a notice list on every reader because it 'would have reopened 063, 062, 055, 030 and 067'; the reopens are the stated reason. — `specs/019-config-decisions/plan.md` — **waived**: advisory and no chore: the owner's 2026-10-20 decision (the scenario's Resolved Questions) chose where unknown keys are reported, and the trade-off also states the result the chosen option gives up; rewording its reason would re-decide it, which is the owner's (decided by the rule)
- grounding — Spec intro states 'config.toml is currently a single-purpose pin file' as present fact with no citation; it is a pre-delivery description now superseded. — `specs/019-config-decisions/spec.md` — **waived**: advisory and no chore: the intro states the problem as it stood before this spec, under the spec's own note marking it historical; the fix is a rewrite of the motivation, not a citation (decided by the rule)
- grounding — Resolved Question 'Validation rules' asserts the framework's existing posture for config.toml (no commit hook, no validate rule, summary-only enforcement) without a citation. — `specs/019-config-decisions/spec.md` — **waived**: advisory and no chore: a Resolved Question's account of the posture when it was decided; data-model.md's Schema validation section states the current posture with its sources (decided by the rule)

## Deferred to other specs

- rule-assessment — BE-PAYLOAD-001: dashboard's new config.unknown-keys list and the unknown-key notices added to discover-rule-files and check-artifact-size are lists bounded only by the config file's key count; the plan commits to no maximum or read-size-bytes/lists-file bound for them — `specs/019-config-decisions/plan.md` — **deferred** to `specs/022-deterministic-runtime/tasks.md` task 32

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 1
- root-absent: 1

## Fixed in this run

- plan-consistency — Technical Decision 'Writes leave the rest of the file intact' states the rule but no rationale (no because/so-that clause). — `specs/019-config-decisions/plan.md` — **fixed** (decided by the rule)
