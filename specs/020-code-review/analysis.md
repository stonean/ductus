---
spec: 020-code-review
last-run: 2026-11-19T22:37:23Z
analyzed-against: c5e8e1a2e54efd74276bce008882785783a14d74
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 3
analyzed-digest:
  data-model.md: 1625833bc1e1572da569f7c582ededc4ea9b59efe55b3f6d459efb59becd9632
  plan.md: edb79569e7bdbdf9bdad94675575a54bed8325a177abd711325c2c64587a9913
  review.md: 9e0e1c66021c475928344908fd3b7f132ba1ac9641d6429e61864919d7d9cee6
  scenarios/every-loaded-rule-file-has-a-pass.md: 4e31f3bd59ad2c2145b8cb24ceaf8561580ffab2ad3143de1087f65b5109a0f3
  scenarios/review-flag-parsing-is-specified.md: a19fe0f4f50c954363f7cca4d913a81d96374535d31a986792243be5263631c1
  scenarios/review-reports-a-defect-under-its-owner.md: ef91f01e38d96520853db62f5ee62b9b8893d41582f055f4f01a25d5a796b665
  scenarios/waiver-expiry.md: 0c35cbf8e26a8a6f321c8549b7209c61b37122713bf13270e5b3c972b788f4ed
  scenarios/waiver-file-lists.md: 8b4180d64e4e43ac6a66f57df7c65d6f1f00bb343ae284bb2d7214c88c824f53
  spec.md: 2da567697f013e8373c51ea3a740b226356581bc4085571370fe59c5eaf68081
  tasks.md: 186082a0e82e5cbc34e05f6d98ea4ed45f3385887557a75a016121c4ab7934f3
unexamined-by-reason:
  never-existed: 1
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
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-INPUT-001: {verdict: addressed, cite: "scenarios/review-flag-parsing-is-specified.md#Behavior"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Waiver record"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#Waiver-expiry warrants a scenario file"}
  CFG-ENV-001: {verdict: addressed, cite: "data-model.md#`.ductus/config.toml [review]` section"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#`.ductus/config.toml` follows the shared-database convention"}
  CFG-ENV-008: {verdict: addressed, cite: "data-model.md#`.ductus/config.toml [review]` section"}
  CFG-ENV-012: {verdict: addressed, cite: "data-model.md#`.ductus/config.toml [review]` section"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "grounding — Acceptance Criteria, AC15: \"Nothing reads `examined` now; it is read at the completion gate by a person\" is a descriptive claim about current system behavior (an absence claim) with no citation to a primary source and no assumption marker."
    outcome: waived
    reason: "A criterion's claims are judged against the tree by every review's criteria step, which records the evidence beside the verdict (020's review of 2026-11-19 judged AC15 met: no runtime code reads the review record's `examined`), so a citation inside the criterion would be a second, unchecked copy of that evidence (owner decision, 2026-11-19)."
    decided-at: 2026-11-19T02:44:45Z
    decided-by: andy@stone.dev
---

# Analysis — 020-code-review

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 3 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md: Acceptance Criteria AC15 states as fact that 'Nothing reads `examined` now; it is read at the completion gate by a person', a claim about current code behavior carrying no file-and-symbol, command or artifact citation and no assumption marker. — `specs/020-code-review/spec.md` — **waived**: A criterion's claims are judged against the tree by every review's criteria step, which records the evidence beside the verdict (020's review of 2026-11-19 judged AC15 met: no runtime code reads the review record's `examined`), so a citation inside the criterion would be a second, unchecked copy of that evidence (owner decision, 2026-11-19).

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- not-a-live-claim: 1
- root-absent: 1

## Fixed in this run

*None.*
