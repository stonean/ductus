---
spec: 073-decided-rule-departures-are-recorded
last-run: 2026-11-13T13:14:57Z
analyzed-against: ab7224f2be5b12d80a8f37cb90195aefef4f8d84
hard-fail: 0
blocking-findings: 1
advisory: 0
unexamined: 2
analyzed-digest:
  data-model.md: 25f122304411ed5b272c4b8aaef2d71ea19858cec8fd63e70f8d026d40840ee9
  plan.md: 698b777166b3c54ad65c8cd4bda22fbba38f00b9dd67d5de2d9b09cffe77e1a5
  review.md: e13456341090a2578d57db3887c066e7dd491103ddddded6884f93d420a416c9
  spec.md: 126fc9ed63cbb53f9c2d7a989c7ee187ca4c36fdd40ddad08d6edbd800b2c48c
  tasks.md: 5c53d95bd0f834969cad443568521e7b24abf2d5fece9cc1ee6af3a47c38e29e
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
  BE-PAYLOAD-001: {verdict: finding, tier: must, at: "plan.md#`check-departures` is a new primitive, owned by this spec", message: "check-departures returns its entries, findings and unexamined lists with no committed size bound or lists-file form"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#One reader, one attribution rule, four consumers"}
blocking: true
dispositions:
  fixed: 4
  deferred: 1
  waived: 0
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-PAYLOAD-001: check-departures returns its entries, findings and unexamined lists with no committed size bound or lists-file form, where every other list-returning primitive is bounded through bounded_lists::place past the read size"
    outcome: deferred
    target: specs/073-decided-rule-departures-are-recorded/tasks.md
    task: "12"
    gating: true
    decided-at: 2026-11-13T13:14:57Z
    decided-by: andy@stone.dev
---

# Analysis — 073-decided-rule-departures-are-recorded

## Summary

0 hard-fail, 1 blocking, 0 advisory; blocking. 2 unexamined target(s). Dispositions: 4 fixed, 1 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

- rule-assessment — BE-PAYLOAD-001: check-departures returns its entries, findings and unexamined lists with no committed size bound or lists-file form, where every other list-returning primitive is bounded through bounded_lists::place past the read size — `specs/073-decided-rule-departures-are-recorded/plan.md` — **deferred** to `specs/073-decided-rule-departures-are-recorded/tasks.md` task 12

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- root-absent: 2

## Fixed in this run

- grounding — plan.md Finding tiers names check_rule_ids' predicate cites_a_rule_id, renamed is_a_rule_id and matched against the whole token by the review's round 1 — `specs/073-decided-rule-departures-are-recorded/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says step 22 reports waived rules before the passed rules, a list step 22 no longer names — `specs/073-decided-rule-departures-are-recorded/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says the constitution's rule-verdicts row needs no edit, where the review's round 1 named the three verdicts there (49,969 bytes) — `specs/073-decided-rule-departures-are-recorded/plan.md` — **fixed** (decided by the rule)
- grounding — data-model.md says the host supplies the bare waived verdict, where under ductus exec the walker records it from an ordinary answer — `specs/073-decided-rule-departures-are-recorded/data-model.md` — **fixed** (decided by the rule)
