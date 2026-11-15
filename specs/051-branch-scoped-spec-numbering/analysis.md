---
spec: 051-branch-scoped-spec-numbering
last-run: 2026-11-15T15:41:10Z
analyzed-against: b43e2b9de4750902224b11dd6435f9605d08473b
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 9
analyzed-digest:
  data-model.md: 6dcf114a4cc86fd5d980afbd844a61959c18125e482da6cc5f58bb389d5d729f
  plan.md: b63de4b06a856773a3e0fdb14030b1a6f242e566e1843dc250dc4b0392f35886
  review.md: 07491611471d62ec036aedf68b2cc1307b628c2f2ebc2af32e32f550cf687b08
  scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md: 015b0ba736e27e91d0d86b37c65e6f7d0d947b5773cf6495313a8ff337b93a47
  scenarios/a-fold-target-is-held-to-the-name-grammar.md: 6bb12cc93d494ce3de1b50243e07fbcaf8c654713a3d6a72c71e97412bdb2f04
  scenarios/a-link-inside-the-target-stays-inside-it.md: a367c3a7c1e260f98b27eb73a87958625422a4e458b758399fd5fa526eb2e1a4
  scenarios/a-pending-fold-never-holds-done.md: dc5c2daa3fd78c7251edf856c21f5d36737e3725d91c808231e112f84502e8b5
  scenarios/fold-target-checked-before-the-rewrite.md: 40bfa2011fb9af497ae85dac6f23f2ad8719dddcc8c5f5654dda7560aade7f32
  scenarios/rewrites-preserve-line-endings.md: 1ab09161757ade0d1f4a3959f574e54d68dab19f275ec84b345b8d16a75ccd26
  scenarios/the-numbering-grammar-reaches-every-surface.md: d4f11334754c65e3cf73346d224aaf8d782f5385a266a2bf22d3339a1079089a
  scenarios/the-spec-root-is-read-before-the-first-write.md: cd12facf1f7af5a4d7a241a909fe34f69fd1bc02984e066e80bc55e7b3be9106
  spec.md: badad3a835a70e93a4c544bf989a11aba4ba27b1b2b5c473fc654bf41b0f48e2
  tasks.md: 02c46f466a17e76af8e6f8a64b400fd3127458e2d737ce02047ab657bfdf7d50
unexamined-by-reason:
  not-a-live-claim: 5
  root-absent: 2
  ships-to-adopter: 2
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
  BE-COMPAT-001: {verdict: addressed, cite: "data-model.md#Primitive argument and result shapes"}
  BE-COORD-002: {verdict: addressed, cite: "plan.md#Fold-back is a command over deterministic primitives"}
  BE-ERR-002: {verdict: addressed, cite: "spec.md#Edge cases and failure modes"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#Directory-name grammar"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Directory-name grammar"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#`create-feature` gains optional arguments rather than a sibling primitive"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#`rewrite-spec-links` (new)"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#`create-feature` gains optional arguments rather than a sibling primitive"}
  BE-REPR-001: {verdict: addressed, cite: "data-model.md#`invalidate-review` (new)"}
  BE-REPR-002: {verdict: addressed, cite: "data-model.md#Primitive argument and result shapes"}
  BE-SCHEMA-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#One directory-form parse, replacing the boolean predicate"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: plan-consistency — data-model.md lists append-task as extended (new `appended` result) and plan.md's Fold-back section relies on its dedup, but plan.md's Affected Files table has no row for append-task's source file
    outcome: waived
    reason: Affected Files is a planning aid the plan template calls non-exhaustive (framework/reference/analyze.md, Plan consistency), and data-model.md names append-task's extension, which plan.md's prose cites
    decided-at: 2026-11-15T15:41:10Z
    decided-by: rule
---

# Analysis — 051-branch-scoped-spec-numbering

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 9 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- plan-consistency — data-model.md lists append-task as extended (new `appended` result) and plan.md's Fold-back section relies on its dedup, but plan.md's Affected Files table has no row for append-task's source file — `specs/051-branch-scoped-spec-numbering/plan.md` — **waived**: Affected Files is a planning aid the plan template calls non-exhaustive (framework/reference/analyze.md, Plan consistency), and data-model.md names append-task's extension, which plan.md's prose cites (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 5
- root-absent: 2
- ships-to-adopter: 2

## Fixed in this run

*None.*
