---
spec: 002-project-scaffolding
last-run: 2026-11-13T15:00:04Z
analyzed-against: 50dbe1e74c6a2543f158a67a053e046838bf5e19
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 6
analyzed-digest:
  plan.md: 8e0ff29ccb99f25628c603ddb94a2e704d944951ba34ad7baedb215862fc9a01
  review.md: 867cddd8402c5b7890b9359d8441e81c91ced605b1e47e1945c21d17e3ef6443
  scenarios/claude-md-template-imports-load.md: 829f166c1f68a1b13a22e42139750d319dd4f4bdab91dec878bf28818808941a
  spec.md: 0246d451c7506de563e4b2fd41273fb567677b8ab6174cec1a64ae55e20fcae5
  tasks.md: 29a39e1a8ef8d870df9a57b8f4674177155b2d316c161914c96cf2a317eb9ec9
unexamined-by-reason:
  root-absent: 4
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
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 002-project-scaffolding

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 6 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

- root-absent: 4
- ships-to-adopter: 2

## Fixed in this run

- body-path-existence — plan.md names framework/templates/project/project-readme.md, which existed in this repository and no longer resolves, as created, and its fate as 003's pending task 6 — `specs/002-project-scaffolding/plan.md` — **fixed** (decided by the rule)
