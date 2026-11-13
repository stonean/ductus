---
spec: 073-decided-rule-departures-are-recorded
last-run: 2026-11-13T14:44:54Z
analyzed-against: fe977b6a5db26f86ca967d83054ee25f0c1b375f
hard-fail: 0
blocking-findings: 1
advisory: 0
unexamined: 2
analyzed-digest:
  data-model.md: 8434e82adf1b4b244412bd9e542cfaaee558fb0c08727e8d8d58dea0269470e1
  plan.md: e55851ef7096c391b0fd7023fbd5894def18cc00fcee93eee6bccfca6520b116
  review.md: f862705e9eef876b74ce23da9c411aeee593243eaaef56b48cdf1bcb4e8bdfe2
  spec.md: 126fc9ed63cbb53f9c2d7a989c7ee187ca4c36fdd40ddad08d6edbd800b2c48c
  tasks.md: eced89d157aa9d5211daa8f5c0916aadee36550de3425be4926af660540cd23b
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
rule-verdicts:
  BE-AUTHZ-007: {verdict: addressed, cite: "data-model.md#`waived` rule verdict"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#Departure entry"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Departure entry"}
  BE-INPUT-008: {verdict: addressed, cite: "data-model.md#Departure entry"}
  BE-PAYLOAD-001: {verdict: finding, tier: must, at: "plan.md#Review: `write-review` derives the waivers, `process-waivers` expires them", message: "write-review returns malformed-departures and superseded-deferrals with no committed size bound or lists-file form, where its analyze-stale-paths list is bounded through bounded_lists::place past the read size"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#One reader, one attribution rule, four consumers"}
blocking: true
dispositions:
  fixed: 0
  deferred: 1
  waived: 0
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-PAYLOAD-001: write-review returns malformed-departures and superseded-deferrals with no committed size bound or lists-file form, where its analyze-stale-paths list is bounded through bounded_lists::place past the read size"
    outcome: deferred
    target: specs/073-decided-rule-departures-are-recorded/tasks.md
    task: "13"
    gating: true
    decided-at: 2026-11-13T14:44:54Z
    decided-by: andy@stone.dev
---

# Analysis — 073-decided-rule-departures-are-recorded

## Summary

0 hard-fail, 1 blocking, 0 advisory; blocking. 2 unexamined target(s). Dispositions: 0 fixed, 1 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

- rule-assessment — BE-PAYLOAD-001: write-review returns malformed-departures and superseded-deferrals with no committed size bound or lists-file form, where its analyze-stale-paths list is bounded through bounded_lists::place past the read size — `specs/073-decided-rule-departures-are-recorded/plan.md` — **deferred** to `specs/073-decided-rule-departures-are-recorded/tasks.md` task 13

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- root-absent: 2

## Fixed in this run

*None.*
