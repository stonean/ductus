---
spec: 055-shared-constitution
last-run: 2026-11-14T03:33:14Z
analyzed-against: 8e180287a383eac4653f6c6c411423738fbca206
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 5
analyzed-digest:
  data-model.md: a8134b0d6cb8dff8f7cab2925bdf0843b17dfc794a5631f8f6c02ae9e7380c99
  plan.md: 374c95e85d4a90daa330dfaed032558acc3cb0880feb5882c1f79e0430118de2
  review.md: dd89d9c52b3209e8308cdc946d3f29d6a10bf5ceeb25e62a4dd55539984e14ca
  scenarios/a-registered-source-is-named-with-its-description.md: f04ffbef1f2349e9f23d8446c20490833b592ae5edb7dad11ed94bbe9c2166f3
  scenarios/target-reports-each-constitutions-size.md: 7661b1fccbf76a02af4467a2a40ebf845cfc302b6104f84b831ff25238b6bd3c
  spec.md: e7288e652122d01eea5a2ed305db894af3f965ff013c0264ba4ea46988f492eb
  tasks.md: a4825fc3536b56c16bd882c0e5c621045648bfffb1ba0dc764e0cef7a17ab7fa
unexamined-by-reason:
  root-absent: 3
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
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#Config schema"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Config schema"}
  BE-INPUT-004: {verdict: waived, reason: "a registry `path` names a shared-constitution checkout outside the project by design, so no base directory exists to contain it; it is the operator's own configuration, and only the fixed `constitution.md` is joined to it", decided-by: "andy@stone.dev", decided-at: "2026-11-14T03:29:58Z"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#Primitive result shape"}
  CFG-ENV-001: {verdict: addressed, cite: "data-model.md#Config schema"}
  CFG-ENV-002: {verdict: addressed, cite: "data-model.md#Config schema"}
  CFG-ENV-003: {verdict: addressed, cite: "data-model.md#Config schema"}
  CFG-ENV-008: {verdict: addressed, cite: "data-model.md#Config schema"}
  CFG-ENV-009: {verdict: addressed, cite: "data-model.md#Config schema"}
  CFG-ENV-012: {verdict: finding, tier: should, at: "data-model.md#Config schema", message: "the loader's treatment of an unrecognized key in [constitutions.<alias>] is unstated"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "rule-assessment — CFG-ENV-012: the loader's treatment of an unrecognized key in [constitutions.<alias>] is unstated"
    outcome: waived
    reason: "Unrecognized-key handling in .ductus/config.toml is project-wide runtime behaviour, not restated per section: the notice names a key inside a [constitutions.<alias>] table and the file, verified on a fixture (unknown config key: [constitutions.acme] descripton)."
    decided-at: 2026-11-10T00:59:00Z
    decided-by: rule
---

# Analysis — 055-shared-constitution

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 5 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — CFG-ENV-012: the loader's treatment of an unrecognized key in [constitutions.<alias>] is unstated — `specs/055-shared-constitution/data-model.md` — **waived**: Unrecognized-key handling in .ductus/config.toml is project-wide runtime behaviour, not restated per section: the notice names a key inside a [constitutions.<alias>] table and the file, verified on a fixture (unknown config key: [constitutions.acme] descripton).

## Deferred to other specs

*None.*

## Waived rules

- BE-INPUT-004 (MUST) — a registry `path` names a shared-constitution checkout outside the project by design, so no base directory exists to contain it; it is the operator's own configuration, and only the fixed `constitution.md` is joined to it — decided by `andy@stone.dev` at `2026-11-14T03:29:58Z`

## Contradictions

*None.*

## Unexamined targets

- root-absent: 3
- ships-to-adopter: 2

## Fixed in this run

*None.*
