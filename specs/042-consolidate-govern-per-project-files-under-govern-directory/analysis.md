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
section-digest:
  "plan.md": e3b0c44298fc1c14
  "plan.md#042 — Consolidate ductus per-project files under a `.ductus/` directory Plan": cbfbe073a53b5494
  "plan.md#Affected Files": f6e37b93d79ea0b7
  "plan.md#Config/session doc + canonical-source updates": 085f1d9b1075df97
  "plan.md#Dogfooding + fixtures": c42ffdb31530f1cc
  "plan.md#No data model": a5991beec5f33a33
  "plan.md#Overview": 2dac51fa29b5aa0a
  "plan.md#Session constant + compile-time guards move to the new path": e51612cbef27c598
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Test strategy": ada0d4c6e5e512ab
  "plan.md#The `govern-dir-consolidate` migration": 7ae9f79b1e6f4b9d
  "plan.md#Trade-offs": 648821edfb811bf0
  "plan.md#Two shared runtime path resolvers with legacy fallback": 26b8bcd08763ebd7
  "plan.md#Write policy — the `/ductus` migration is the sole cutover": e336245a933edade
  "plan.md#gitignore supersession": ad8b284b94ee10c6
  "scenarios/provenance-tags-name-resolved-config-path.md": c04ca6daded21b4e
  "scenarios/provenance-tags-name-resolved-config-path.md#Behavior": 27f13b56ac27f0b2
  "scenarios/provenance-tags-name-resolved-config-path.md#Context": 206612af0ac9700c
  "scenarios/provenance-tags-name-resolved-config-path.md#Edge Cases": df0b2b0a1d9a7585
  "scenarios/provenance-tags-name-resolved-config-path.md#Open Questions": 9fc5979d590cdb97
  "scenarios/provenance-tags-name-resolved-config-path.md#Provenance-tags-name-resolved-config-path": 57c6c794c2e8f83f
  "scenarios/provenance-tags-name-resolved-config-path.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/runtime-doc-strings-name-active-paths.md": cc1084e3b46a5f04
  "scenarios/runtime-doc-strings-name-active-paths.md#Behavior": 7aadf23f7ecb89f3
  "scenarios/runtime-doc-strings-name-active-paths.md#Context": ce6ad11665ebc0e6
  "scenarios/runtime-doc-strings-name-active-paths.md#Edge Cases": c7311d72d0f8e7c5
  "scenarios/runtime-doc-strings-name-active-paths.md#Open Questions": 9fc5979d590cdb97
  "scenarios/runtime-doc-strings-name-active-paths.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/runtime-doc-strings-name-active-paths.md#Runtime-doc-strings-name-active-paths": 3aa6475939057bd4
  "spec.md": 394078f337af7175
  "spec.md#042 — Consolidate ductus per-project files under a `.ductus/` directory": 277071ca98bf0008
  "spec.md#Acceptance Criteria": 851ad968cd88cf84
  "spec.md#Config file: `.ductus/config.toml`": 5d62799069a37b85
  "spec.md#Documentation and canonical sources": a6bbdc8ac3dbedc7
  "spec.md#Edge cases and transition hazards": 3a18add6823aaec9
  "spec.md#Motivation": 575220c7a4bfef22
  "spec.md#Open Questions": 77ef24ad40ad6b03
  "spec.md#Resolved Questions": 3eb75a118ff1dd56
  "spec.md#Scripts (retired)": 935c40627926af83
  "spec.md#Session file: `.ductus/session.toml`": c9e08187583230d8
  "spec.md#Target layout": 395a5a3ead9e6e85
  "spec.md#Transition and fallback": f955aadd741d96bf
  "spec.md#`/ductus` migration: reorganize a project on the old layout": 8ce618287284436e
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
