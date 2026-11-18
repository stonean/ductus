---
spec: 012-multi-agent-govern
last-run: 2026-11-18T21:16:56Z
analyzed-against: 340afb65f47fd3d2be7bae27c5fe64dd836dcb3a
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 10
analyzed-digest:
  data-model.md: 3811330ca711759e48b2e1658edced9b1c93bb1a85d7ea641d371ff33e3a64b0
  plan.md: 7ec8bb1a73a117a8eec5dcf8371074d8037107ea274edcab6bb70764a4df1975
  review.md: 9a99fdba9412c08066d01612b38ace19731400a77c0373002fd8ee3d687dadb5
  scenarios/a-dropped-hosts-leftover-tree-is-named.md: b90dd4b9460182627500c70781228121fc32704ea3e4501e01c42b751726e735
  scenarios/settings-template-bash-allowlist.md: 6fb0d4346bb688e9b02a14ca19f6e9f515ab5d927483d5e4bc505f0c551349ac
  spec.md: 99b8e8c973ce53a600b734a78eca75b1a41ef24a1dbf92d4f776ddea47186943
  tasks.md: 072a1b23b5175893b590871e7e697d4465796258104c333be8e9d6e58b1f88f0
unexamined-by-reason:
  not-a-live-claim: 5
  root-absent: 1
  ships-to-adopter: 4
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
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 012-multi-agent-govern

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 10 unexamined target(s). Dispositions: 4 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

- not-a-live-claim: 5
- root-absent: 1
- ships-to-adopter: 4

## Fixed in this run

- grounding — Per-Agent Scaffolding said Pi has no permission-gating settings and reads AGENTS.md natively, with no citation (§Grounding (advisory)). — `specs/012-multi-agent-govern/spec.md` — **fixed** (decided by the rule)
- grounding — The Migration section said a per-CLI file could not self-update because its fetch manifest named a removed source, with no citation (§Grounding (advisory)). — `specs/012-multi-agent-govern/spec.md` — **fixed** (decided by the rule)
- grounding — The 007 signpost section said 007 already carries a signpost from 011, with no citation (§Grounding (advisory)). — `specs/012-multi-agent-govern/plan.md` — **fixed** (decided by the rule)
- plan-consistency — The 'Per-agent scaffolding loop' Technical Decision listed steps with no rationale (§Spec integrity, rationale). — `specs/012-multi-agent-govern/plan.md` — **fixed** (decided by the rule)
