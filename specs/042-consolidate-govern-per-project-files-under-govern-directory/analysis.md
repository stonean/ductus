---
spec: 042-consolidate-govern-per-project-files-under-govern-directory
last-run: 2026-11-18T12:57:56Z
analyzed-against: a88aac5e5b9dc1bf0161154696d204eb9bf05f26
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 9
analyzed-digest:
  plan.md: dcb2cfff7eaf848eeae2730c88c879104b0897390b2d15b5f343693964bc9555
  review.md: 7a490e00904ef3e6730c8a809c2d652bea77ced06da4f4acf72af4defdc5b530
  scenarios/provenance-tags-name-resolved-config-path.md: 403822bfa3dad6559c093b5e29b8cd616cd33cdbf585629e7f38ef224ffa9942
  scenarios/runtime-doc-strings-name-active-paths.md: 5799c0f7c23e431c48a83c8d2a4696beded3861629e767b5fc1fa8e67e06c527
  spec.md: d375df53277c18cbbae4cee0b137b1db76f6d3b87191b22c63bcf254b4c16c94
  tasks.md: 2703eaec29b27b5aa69da2dff36c2e797c2fbe7cdcf1eb3bd9f76e7d2a269e89
unexamined-by-reason:
  not-a-live-claim: 7
  root-absent: 2
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
  fixed: 1
  deferred: 0
  waived: 1
  undispositioned: 2
decisions:
  - key: grounding — spec.md Edge cases states ductus's own repo carries the session-ignore line outside the managed block and adopters carry it inside, with no citation in the spec
    outcome: waived
    reason: an edge case describing the state the migration superseded, the /.govern.session.toml ignore line, which neither file carries now; it records what the migration had to handle, and plan.md cites .gitignore for it
    decided-at: 2026-11-18T12:57:56Z
    decided-by: rule
---

# Analysis — 042-consolidate-govern-per-project-files-under-govern-directory

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 9 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 1 waived, 2 undispositioned. An undispositioned advisory finding does not hold `done`; a hard-fail or blocking one holds it until the re-check no longer produces it.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md Edge cases states ductus's own repo carries the session-ignore line outside the managed block and adopters carry it inside, with no citation in the spec — `specs/042-consolidate-govern-per-project-files-under-govern-directory/spec.md` — **waived**: an edge case describing the state the migration superseded, the /.govern.session.toml ignore line, which neither file carries now; it records what the migration had to handle, and plan.md cites .gitignore for it (decided by the rule)
- grounding — spec.md Motivation says the generators were installed via the Shared Files manifest with update strategy, with no citation or assumption marker — `specs/042-consolidate-govern-per-project-files-under-govern-directory/spec.md` — **undispositioned**
- grounding — spec.md Resolved Questions states ~27 maintainer-only scripts with no citation — `specs/042-consolidate-govern-per-project-files-under-govern-directory/spec.md` — **undispositioned**

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 7
- root-absent: 2

## Fixed in this run

- grounding — spec.md migration section states /{project}:configure denies git mv, with no citation to the configure source or permission list — `specs/042-consolidate-govern-per-project-files-under-govern-directory/spec.md` — **fixed** (decided by the rule)
