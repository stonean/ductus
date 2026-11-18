---
spec: 051-branch-scoped-spec-numbering
last-run: 2026-11-18T13:07:02Z
analyzed-against: d793782d76f6450f20e84d63c4a25e9770a9a4e8
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 9
analyzed-digest:
  data-model.md: 8e8baa256ea782bfb50a0620c598997cec119266cb41a5f8a0cd233879d6427a
  plan.md: 849424ef3fde9e1f190bcbb4dfdcbf1f40e13a1c43d5e9108d17bef9375871d6
  review.md: e4ca986c808415ccb75db85adea8d9d5d76b602207620d45783fded7127467f9
  scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md: 015b0ba736e27e91d0d86b37c65e6f7d0d947b5773cf6495313a8ff337b93a47
  scenarios/a-fold-target-is-held-to-the-name-grammar.md: dd15a827e2e30c67606ed2b49c965773d69e0356283264ed639f97c5f4adc901
  scenarios/a-link-inside-the-target-stays-inside-it.md: a367c3a7c1e260f98b27eb73a87958625422a4e458b758399fd5fa526eb2e1a4
  scenarios/a-pending-fold-never-holds-done.md: dc5c2daa3fd78c7251edf856c21f5d36737e3725d91c808231e112f84502e8b5
  scenarios/fold-target-checked-before-the-rewrite.md: 40bfa2011fb9af497ae85dac6f23f2ad8719dddcc8c5f5654dda7560aade7f32
  scenarios/rewrites-preserve-line-endings.md: 0b0c750d0094f3fa6d9bff4fc8bac7bdaaa458416af6d90acc554a8fe468d3c3
  scenarios/the-numbering-grammar-reaches-every-surface.md: d4f11334754c65e3cf73346d224aaf8d782f5385a266a2bf22d3339a1079089a
  scenarios/the-spec-root-is-read-before-the-first-write.md: ccf10584e584b2f0f83808e7e1413a30cd98c1b5691a4c464da9432b0fd53ff9
  spec.md: 7dccae30969eb8d2d97856fa8f75ef392cbfb971e218a144f0a362372f3774cd
  tasks.md: afe8c159c24abac97736ee9c9ee994934cf67fd604b179587960d9d1b069d2e1
unexamined-by-reason:
  not-a-live-claim: 5
  root-absent: 3
  ships-to-adopter: 1
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
  BE-COMPAT-001: {verdict: addressed, cite: "data-model.md#The write guard"}
  BE-ERR-002: {verdict: addressed, cite: "spec.md#Edge cases and failure modes"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#Directory-name grammar"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Directory-name grammar"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Technical Decisions"}
  BE-LOCK-001: {verdict: addressed, cite: "data-model.md#The write guard"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#`rewrite-spec-links` (new)"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#`create-feature` gains optional arguments rather than a sibling primitive"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#Fold-back is a command over deterministic primitives"}
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 0
  undispositioned: 2
---

# Analysis — 051-branch-scoped-spec-numbering

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 9 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 0 waived, 2 undispositioned. An undispositioned advisory finding does not hold `done`; a hard-fail or blocking one holds it until the re-check no longer produces it.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md Mode selection states the framework has no committed-yet-branch-local state and session.toml is the one per-branch surface, with no citation at that passage — `specs/051-branch-scoped-spec-numbering/spec.md` — **undispositioned**
- grounding — spec.md Edge cases states the directory is claimed atomically as fact about create-feature, with no file or symbol cite in spec.md (cited only in plan.md) — `specs/051-branch-scoped-spec-numbering/spec.md` — **undispositioned**

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 5
- root-absent: 3
- ships-to-adopter: 1

## Fixed in this run

- grounding — plan.md Known limitations claims specs/system.md does not exist in this repository (ls, 2026-10-28), but specs/system.md exists now; the cited claim is stale — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed** (decided by the rule)
