---
spec: 075-agents-md-holds-what-every-session-needs
last-run: 2026-11-15T16:57:17Z
analyzed-against: 21c538312a6aac030868045c22aea7be41726e04
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 8
analyzed-digest:
  plan.md: df9b40d5562e3045508788d0b2d08b3ab4085dd4494366eb0a821ec74982caa4
  review.md: 471c2d6b5f05cd8bbae6db2e85a367d3affb16092901bc8c21c980594850772d
  spec.md: efa5896f4ef1d80c91da1a69dfb82914cf73616263cfd7ff12b33ea73f922f28
  tasks.md: 966ecd354e133c3285fe2bf015268a916e8caa7ee1427a16a00a308a4af4ac0d
unexamined-by-reason:
  never-existed: 1
  ships-to-adopter: 7
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
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#`check-artifact-size` measures the project files (063)"}
  BE-INPUT-004: {verdict: finding, tier: must, at: "spec.md#`/prune` reduces the project files", message: "`/prune --project` writes a confirmed entry into a home (a code comment or test at a site included) and says only that nothing is written outside the project; no canonicalization and whole-component containment check, or pointer to an existing mechanism, is committed (041's plan and scenario name none)."}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#`check-artifact-size` measures the project files (063)"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#`check-artifact-size` measures the project files (063)"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#`check-artifact-size` measures the project files (063)"}
blocking: false
dispositions:
  fixed: 0
  deferred: 1
  waived: 3
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-INPUT-004: `/prune --project` writes a confirmed entry into a home (a code comment or test at a site included) and says only that nothing is written outside the project; no canonicalization and whole-component containment check, or pointer to an existing mechanism, is committed (041's plan and scenario name none)."
    outcome: deferred
    target: specs/041-task-pruning/tasks.md
    task: "25"
    reason: 075's plan says 041 owns prune-project's design; the owner deferred this finding to 041 task 25 (2026-11-15)
    decided-at: 2026-11-15T16:57:17Z
    decided-by: andy@stone.dev
  - key: grounding — Motivation states that Pi reads AGENTS.md natively with no citation, command or assumption marker
    outcome: waived
    reason: "The claim holds: framework/bootstrap/ductus.md's agent registry row for pi says Pi reads AGENTS.md natively; advisory, and no requirement rests on it."
    decided-at: 2026-11-15T16:57:17Z
    decided-by: rule
  - key: "grounding — Resolved Question on reaching homes states that Claude Code's path-scoped .claude/rules/ files and skills load on demand and that a paths: glob must name which code is architecture, uncited"
    outcome: waived
    reason: A claim about Claude Code's own loading, from the memory documentation the Motivation cites as read 2026-10-29; the chosen option stands on the memory-routing block alone, so the claim is not load-bearing.
    decided-at: 2026-11-15T16:57:17Z
    decided-by: rule
  - key: grounding — Motivation states that /plan read system.md on every run and /analyze and /review checked against all three project files, uncited
    outcome: waived
    reason: "The claim holds: framework/commands/plan.md's scope boundary reads specs/system.md, and the analyze and review references check against events.md and errors.md; it is historical motivation, advisory."
    decided-at: 2026-11-15T16:57:17Z
    decided-by: rule
---

# Analysis — 075-agents-md-holds-what-every-session-needs

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 8 unexamined target(s). Dispositions: 0 fixed, 1 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Motivation states that Pi reads AGENTS.md natively with no citation, command or assumption marker — `specs/075-agents-md-holds-what-every-session-needs/spec.md` — **waived**: The claim holds: framework/bootstrap/ductus.md's agent registry row for pi says Pi reads AGENTS.md natively; advisory, and no requirement rests on it. (decided by the rule)
- grounding — Resolved Question on reaching homes states that Claude Code's path-scoped .claude/rules/ files and skills load on demand and that a paths: glob must name which code is architecture, uncited — `specs/075-agents-md-holds-what-every-session-needs/spec.md` — **waived**: A claim about Claude Code's own loading, from the memory documentation the Motivation cites as read 2026-10-29; the chosen option stands on the memory-routing block alone, so the claim is not load-bearing. (decided by the rule)
- grounding — Motivation states that /plan read system.md on every run and /analyze and /review checked against all three project files, uncited — `specs/075-agents-md-holds-what-every-session-needs/spec.md` — **waived**: The claim holds: framework/commands/plan.md's scope boundary reads specs/system.md, and the analyze and review references check against events.md and errors.md; it is historical motivation, advisory. (decided by the rule)

## Deferred to other specs

- rule-assessment — BE-INPUT-004: `/prune --project` writes a confirmed entry into a home (a code comment or test at a site included) and says only that nothing is written outside the project; no canonicalization and whole-component containment check, or pointer to an existing mechanism, is committed (041's plan and scenario name none). — `specs/075-agents-md-holds-what-every-session-needs/spec.md` — **deferred** to `specs/041-task-pruning/tasks.md` task 25

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- ships-to-adopter: 7

## Fixed in this run

*None.*
