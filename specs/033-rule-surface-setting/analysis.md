---
spec: 033-rule-surface-setting
last-run: 2026-11-14T13:34:21Z
analyzed-against: a1190623854a039277238e0ad3d5ba97f89fef23
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 0
analyzed-digest:
  plan.md: 021b90c8748caf6f299313ae56af54f82149b28752ba220efb74707291b9029b
  review.md: 198bb6e0864485b6628b60c176579f18e109b749eacdcb51660f16d5bc73f1df
  scenarios/an-out-of-surface-citation-resolves.md: f4c103aa35a1c6007a4b00507bdb1a45cbb0674ee7b43c41c8e2e3ab384456fd
  scenarios/degenerate-surfaces-config.md: 13f9a4e5153df9ca6f1de6af3b933b6b98224a947c1fd79cee7113c64130d456
  spec.md: 59daa2480f364c6b54499228218d74f1ab02760b3bdfa1ab6d254e79dd2d9aa9
  tasks.md: 9cb0fa395f55887ac124286372964b32385b0c874cd52ae8831410abcf9a7891
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
  BE-INPUT-001: {verdict: addressed, cite: "scenarios/degenerate-surfaces-config.md#Behavior"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Setting schema — `[rules] surfaces`"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#Setting schema — `[rules] surfaces`"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "scenarios/an-out-of-surface-citation-resolves.md#Behavior"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#Setting schema — `[rules] surfaces`"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Documentation"}
  CFG-ENV-007: {verdict: addressed, cite: "spec.md#Setting"}
  CFG-ENV-008: {verdict: addressed, cite: "scenarios/degenerate-surfaces-config.md#Behavior"}
  CFG-ENV-012: {verdict: addressed, cite: "plan.md#Setting schema — `[rules] surfaces`"}
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 033-rule-surface-setting

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 0 unexamined target(s). Dispositions: 4 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

*None — every target was examined.*

## Fixed in this run

- rule-assessment — CFG-ENV-012: The new `[rules]` section states no treatment of an unrecognized key; cite 019's CFG-ENV-012 decision (report by name, run continues) and register `rules` keys in `runtime/src/schema/config_keys.rs`. — `specs/033-rule-surface-setting/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-PAYLOAD-001: check-rule-ids now reports a list of not-installed citations bounded only by the spec's citations, with no committed maximum size or lists-file bound — `specs/033-rule-surface-setting/scenarios/an-out-of-surface-citation-resolves.md` — **fixed** (decided by the rule)
- grounding — Motivation states as fact that security-frontend.md carries the only CSRF/secure-cookie rules, with no citation, command or marker — `specs/033-rule-surface-setting/spec.md` — **fixed** (decided by the rule)
- grounding — Plan asserts that rule files are not in enforce-directories and only slash-command dirs are pruned, citing no file or symbol — `specs/033-rule-surface-setting/plan.md` — **fixed** (decided by the rule)
