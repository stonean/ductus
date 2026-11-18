---
spec: 056-bootstrap-archive-boundary-split
last-run: 2026-11-18T22:06:52Z
analyzed-against: cbffcddb4f01cd8261d079e02e4ce438d37b21e7
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 4
analyzed-digest:
  plan.md: c54fa27c119362c5acf5f2ee5cb5c4a50a6711058c34b6328ea662146b4171e8
  review.md: 31cc47b3b8c1e675e651cd5a834a07fdd84fe1c54b6f2b04f3443e6de287eac8
  scenarios/the-walker-places-every-archive-half-section.md: 317f5c421d25c4ee588bde505b22fbbf9629fef0880857e9e5b73f3f066de7d5
  spec.md: 7fcedd7562c75465743a221d9b2de780f6d390a05c6b4e3506b5ded2e5e42f1c
  tasks.md: 21ff3d40a7b56981911fa8aeb4a8fe36a8acfe96b5887154e993c74cc3911034
unexamined-by-reason:
  not-a-live-claim: 3
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
  fixed: 8
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 056-bootstrap-archive-boundary-split

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 4 unexamined target(s). Dispositions: 8 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

- not-a-live-claim: 3
- ships-to-adopter: 1

## Fixed in this run

- grounding — Rationale disposition said specs/inbox.md puts the rationale share at 17–25% of the file, in the present tense, though ff301e1b retired that inbox item (§Grounding (advisory)). — `specs/056-bootstrap-archive-boundary-split/spec.md` — **fixed** (decided by the rule)
- grounding — The archive-half frontmatter Resolved Question said runtime/legacy-prose-commands.txt's header today asserts every entry is a framework/commands/*.md file, though the header names framework/bootstrap/ too (§Grounding (advisory)). — `specs/056-bootstrap-archive-boundary-split/spec.md` — **fixed** (decided by the rule)
- grounding — Affected Files said plan.md records the post-split measurement (AC11), which 063e50ea's prune moved to the commit record (§Grounding (advisory)). — `specs/056-bootstrap-archive-boundary-split/plan.md` — **fixed** (decided by the rule)
- grounding — The 'allowlist header is corrected in the same change' decision said the header asserts a framework/commands/*.md-only claim, though it has named framework/bootstrap/ too since that change (§Grounding (advisory)). — `specs/056-bootstrap-archive-boundary-split/plan.md` — **fixed** (decided by the rule)
- grounding — The allowlist decision said the file is in framework-checks.yml's path filter, though 44c0babe removed that workflow's path filter (§Grounding (advisory)). — `specs/056-bootstrap-archive-boundary-split/plan.md` — **fixed** (decided by the rule)
- grounding — The plan's 26.3% known limitation stated the section walk's predicted 38,616-byte cut as the achieved saving; the split measured 36,536 bytes, 24.9% (§Grounding (advisory)). — `specs/056-bootstrap-archive-boundary-split/plan.md` — **fixed** (decided by the rule)
- grounding — Measured stated the section walk's 26.3% / 38,616 bytes as what an adopter's file drops by; the split measured 36,536 bytes, 24.9% (§Grounding (advisory)). — `specs/056-bootstrap-archive-boundary-split/spec.md` — **fixed** (decided by the rule)
- grounding — The 'Why the cut is 26%' heading named the section walk's 26.3% as the cut, though the installed half's net drop was 24.9% (§Grounding (advisory)). — `specs/056-bootstrap-archive-boundary-split/spec.md` — **fixed** (decided by the rule)
