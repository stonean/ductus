---
spec: 071-command-reference-split
last-run: 2026-11-20T01:16:32Z
analyzed-against: 677574fb857d0452d243b34fb791e4699fd2529f
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 6
analyzed-digest:
  plan.md: 1e18f0a6572e979c01d8d680450c165e16e1f215da22c4db191f8be9b5b7ddf2
  review.md: c4ea8651074678095c549e7e59253da83bc61ccab373d91b159ea6f3efa38ca3
  scenarios/reference-sections-are-reached-by-a-step.md: 36e3ed15f3cc1a502d2b6fb284da192027b185b80692faab1522a48cf48ab0c9
  spec.md: 035215499bba2a9cd29c25a0f04fa097389a98c908210bf42f0bbffa17bd8bfa
  tasks.md: 42e9462ff7a62bb68002eb57a66fbb3638e7307500b3bf1655ef7b10af6201ad
unexamined-by-reason:
  anchor-target-unjudgeable: 1
  not-a-live-claim: 2
  root-absent: 2
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
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: grounding — spec.md:24–28 states that Claude Code compaction keeps only the first 5,000 tokens of an invoked skill, citing no documentation
    outcome: waived
    reason: motivating context the spec marks as an assumption, and no criterion rests on it (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:53:14Z
    decided-by: andy@stone.dev
---

# Analysis — 071-command-reference-split

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 6 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md Motivation states that Claude Code's documentation says compaction keeps only the first 5,000 tokens of an invoked skill, citing no documentation — `specs/071-command-reference-split/spec.md` — **waived**: motivating context the spec marks as an assumption, and no criterion rests on it (owner decision, 2026-10-26)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1
- not-a-live-claim: 2
- root-absent: 2
- ships-to-adopter: 1

## Fixed in this run

*None.*
