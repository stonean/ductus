---
spec: 045-decision-state-drift-detection
last-run: 2026-11-18T12:55:30Z
analyzed-against: 3c839d4298c8264c09da71b3dc843f88d6a92675
hard-fail: 0
blocking-findings: 0
advisory: 5
unexamined: 17
analyzed-digest:
  data-model.md: 23e99b2c0e83aa0f588287652db21683d1719fc0181c855bf119f143d29c30e4
  plan.md: 9ec6e42800855fe9005a4f4756195579f0f04faac221f7fa570d8601f3cc5ea3
  review.md: 81930205c4a39cb4d673aeeb283d6b60b62e42ca1adf83f88010240f8ecba2a6
  spec.md: dfbc0627b68adf226465576b8f6c265410d283fcbd1889f04cd3dfaa22573765
  tasks.md: 7c8c81b9a6d8c257003f09d2769834cae277f1bd9e1f8a6a8a10e1b52972027c
unexamined-by-reason:
  not-a-live-claim: 8
  root-absent: 5
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
rule-verdicts:
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#What the families read, and how much they return"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#What the families read, and how much they return"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#What the families read, and how much they return"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "plan.md#An unreadable target produces no finding, but is not silent"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "body-path-existence — names `../spec.md`, which existed in this repository and no longer resolves: \"For each inline `[text](target)` in a scanned artifact, resolve `target` relative to the *containing file's* directory and keep it only when the result lands inside the feature directory. This is what makes `../spec.md` from `scenarios/foo.md` a sibling while `../022-deterministic-runtime/spec.md` from `spec.md` is not — the distinction the spec's §Scope depends on, and the one the Motivation measurement relied on (21 cross-feature links in `## Motivation` sections, all out of scope).\""
    outcome: waived
    reason: "correct as written: `../spec.md` is the worked example of a link resolved from the hypothetical `scenarios/foo.md`, not a path the plan claims exists; the family resolves it from plan.md's directory instead"
    decided-at: 2026-11-11T17:22:42Z
    decided-by: rule
  - key: "body-path-existence — names `framework/workflows/registry.json`, which existed in this repository and no longer resolves: \"| `framework/workflows/registry.json` | yes | no → finding |\""
    outcome: waived
    reason: "correct as written: the row is the worked example's expected outcome, the path deleted by 531e3ea and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach"
    decided-at: 2026-11-11T17:22:42Z
    decided-by: rule
  - key: "body-path-existence — names `scripts/audit/registry-equivalence.sh`, which existed in this repository and no longer resolves: \"| `scripts/audit/registry-equivalence.sh` | yes | no → finding |\""
    outcome: waived
    reason: "correct as written: the row is the worked example's expected outcome, the script deleted by 3ff65445 and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach"
    decided-at: 2026-11-11T17:22:42Z
    decided-by: rule
  - key: grounding — spec.md states across all 47 specs Motivation sections contain zero same-feature links and 21 links to other specs as fact, with no query or command named
    outcome: waived
    reason: a measurement recorded when the spec was written, of the 47 specs then in the corpus, to motivate scoping the check to sibling links; re-deriving its method now would count a different corpus, so a citation is no mechanical fix
    decided-at: 2026-11-18T12:55:30Z
    decided-by: rule
  - key: grounding — spec.md states the first full-repo run's single finding across 47 specs was a false positive, with no command or recorded run cited
    outcome: waived
    reason: an account of the family's first run when it landed, a record of how the check behaved then rather than a claim about the tree today; the run's commit, not a citation, is its record
    decided-at: 2026-11-18T12:55:30Z
    decided-by: rule
---

# Analysis — 045-decision-state-drift-detection

## Summary

0 hard-fail, 0 blocking, 5 advisory; not blocking. 17 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 5 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- body-path-existence — names `../spec.md`, which existed in this repository and no longer resolves: "For each inline `[text](target)` in a scanned artifact, resolve `target` relative to the *containing file's* directory and keep it only when the result lands inside the feature directory. This is what makes `../spec.md` from `scenarios/foo.md` a sibling while `../022-deterministic-runtime/spec.md` from `spec.md` is not — the distinction the spec's §Scope depends on, and the one the Motivation measurement relied on (21 cross-feature links in `## Motivation` sections, all out of scope)." — `specs/045-decision-state-drift-detection/plan.md` — **waived**: correct as written: `../spec.md` is the worked example of a link resolved from the hypothetical `scenarios/foo.md`, not a path the plan claims exists; the family resolves it from plan.md's directory instead (decided by the rule)
- body-path-existence — names `framework/workflows/registry.json`, which existed in this repository and no longer resolves: "| `framework/workflows/registry.json` | yes | no → finding |" — `specs/045-decision-state-drift-detection/data-model.md` — **waived**: correct as written: the row is the worked example's expected outcome, the path deleted by 531e3ea and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach (decided by the rule)
- body-path-existence — names `scripts/audit/registry-equivalence.sh`, which existed in this repository and no longer resolves: "| `scripts/audit/registry-equivalence.sh` | yes | no → finding |" — `specs/045-decision-state-drift-detection/data-model.md` — **waived**: correct as written: the row is the worked example's expected outcome, the script deleted by 3ff65445 and no longer resolving; the deletion is stated on the example's lead-in line, which the per-line exemption does not reach (decided by the rule)
- grounding — spec.md states across all 47 specs Motivation sections contain zero same-feature links and 21 links to other specs as fact, with no query or command named — `specs/045-decision-state-drift-detection/spec.md` — **waived**: a measurement recorded when the spec was written, of the 47 specs then in the corpus, to motivate scoping the check to sibling links; re-deriving its method now would count a different corpus, so a citation is no mechanical fix (decided by the rule)
- grounding — spec.md states the first full-repo run's single finding across 47 specs was a false positive, with no command or recorded run cited — `specs/045-decision-state-drift-detection/spec.md` — **waived**: an account of the family's first run when it landed, a record of how the check behaved then rather than a claim about the tree today; the run's commit, not a citation, is its record (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 8
- root-absent: 5
- ships-to-adopter: 4

## Fixed in this run

*None.*
