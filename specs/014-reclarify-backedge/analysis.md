---
spec: 014-reclarify-backedge
last-run: 2026-11-14T01:09:19Z
analyzed-against: 25df4c5d76c4b59ab2c31225022a54023d8c0686
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 3
analyzed-digest:
  plan.md: 6dca1a1a605d3c22ea418b21386838c26d1d144137a921021207ca626e9bfce6
  review.md: e4ee5f45ee053eb1a7d1c9d414e03e85dff2c11783d5b69ea5636049241abf5f
  scenarios/reopen-after-informal-edits.md: 214c03a040136eb98002a3ff844bd2aafe4b1bc51badc60a8eac594dcf79acff
  spec.md: 28a1dbb682269c13aedbcba093071088cfecf31201ff17cfbb6d32d0ffccc0ef
  tasks.md: 023701294c3b8c06e7a1891048e49514715c69c806552bd60f0a4cc49b5221c6
unexamined-by-reason:
  not-a-live-claim: 1
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
  fixed: 0
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: grounding — Edge case 'amend on a clarified spec that is a dependency of an in-progress spec' states as fact that /clarify and /plan both check dependencies are at clarified or later, with no file-and-symbol citation, command or assumption marker
    outcome: waived
    reason: background to a prescriptive impact-display note, not a contract this spec depends on; citing it needs a read of the clarify and plan command sources, which is no mechanical chore
    decided-at: 2026-11-14T01:09:19Z
    decided-by: rule
---

# Analysis — 014-reclarify-backedge

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 3 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Edge case 'amend on a clarified spec that is a dependency of an in-progress spec' states as fact that /clarify and /plan both check dependencies are at clarified or later, with no file-and-symbol citation, command or assumption marker — `specs/014-reclarify-backedge/spec.md` — **waived**: background to a prescriptive impact-display note, not a contract this spec depends on; citing it needs a read of the clarify and plan command sources, which is no mechanical chore (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 1
- ships-to-adopter: 2

## Fixed in this run

*None.*
