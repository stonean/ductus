---
spec: 081-reviews-and-checks-follow-what-changed
last-run: 2026-11-13T17:45:31Z
analyzed-against: 61ad8aa57fd04690bb54c5e541f7282104102ed2
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 7
analyzed-digest:
  plan.md: ee827e9f7f5cca4ff8beaaf746f738fa85b2c6cb4f11eb7ee92d6df56bdc08a0
  review.md: 07ffb070b8c4f377273ea894bc748f85a6b57db89cec50c929adc3bd780f7391
  spec.md: 9c27ac95515a282631e7abb26ceed2c7ae64830e383b991ed427adb248f11b20
  tasks.md: 94e25ca06a794be5deb2d2dfcba00ac9b0999e043cf62fdfac183349f28e322b
unexamined-by-reason:
  not-a-live-claim: 3
  root-absent: 4
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
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#The change is breaking"}
  BE-DEPS-007: {verdict: addressed, cite: "plan.md#Check selection in this repository's CI"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Credit for a read"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Credit for a read"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#The gate"}
  BE-SCHEMA-002: {verdict: addressed, cite: "plan.md#The change is breaking"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#Check selection in this repository's CI"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 3
  undispositioned: 0
decisions:
  - key: grounding — Resolved Questions (which commit-subject-less review owes the code) state as fact that the pipeline prescribes no commit-subject convention, with no citation to the artifact that settles it.
    outcome: waived
    reason: "A claim of absence: neither the constitution nor any command source prescribes a commit-subject convention, so there is no one primary source to cite, and the Resolved Question records the owner's decision taken on that premise (owner, 2026-11-11). Choosing what to cite is a judgment, not a mechanical edit."
    decided-at: 2026-11-13T17:45:31Z
    decided-by: rule
  - key: grounding — Plan Attribution states as fact that fix rounds and diff-cross-spec read CommitClaims but not attribute_window, citing no file or command that shows it.
    outcome: waived
    reason: The claim names the symbols it rests on (`CommitClaims`, `attribute_window`, `diff-cross-spec`), the file-and-symbol form §grounding asks for less the path; this run's review read compute_review_scope.rs whole and its callers, and the plan records the design as it stands. Adding a path is a citation choice, not a mechanical edit.
    decided-at: 2026-11-13T17:45:31Z
    decided-by: rule
  - key: grounding — Resolved Question on review.md states as fact that the gate already orders a reopening's review ahead of its analysis, with no citation.
    outcome: waived
    reason: "The order is stated by check-review-gate's compiled description, the canonical source for a primitive's behavior (§drift-prevention): the review record's checks, then the analyze record's. The Resolved Question records a decision taken on it; choosing to add the pointer is a citation choice, not a mechanical edit."
    decided-at: 2026-11-13T17:45:31Z
    decided-by: rule
---

# Analysis — 081-reviews-and-checks-follow-what-changed

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 7 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Resolved Questions (which commit-subject-less review owes the code) state as fact that the pipeline prescribes no commit-subject convention, with no citation to the artifact that settles it. — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **waived**: A claim of absence: neither the constitution nor any command source prescribes a commit-subject convention, so there is no one primary source to cite, and the Resolved Question records the owner's decision taken on that premise (owner, 2026-11-11). Choosing what to cite is a judgment, not a mechanical edit. (decided by the rule)
- grounding — Plan Attribution states as fact that fix rounds and diff-cross-spec read CommitClaims but not attribute_window, citing no file or command that shows it. — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **waived**: The claim names the symbols it rests on (`CommitClaims`, `attribute_window`, `diff-cross-spec`), the file-and-symbol form §grounding asks for less the path; this run's review read compute_review_scope.rs whole and its callers, and the plan records the design as it stands. Adding a path is a citation choice, not a mechanical edit. (decided by the rule)
- grounding — Resolved Question on review.md states as fact that the gate already orders a reopening's review ahead of its analysis, with no citation. — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **waived**: The order is stated by check-review-gate's compiled description, the canonical source for a primitive's behavior (§drift-prevention): the review record's checks, then the analyze record's. The Resolved Question records a decision taken on it; choosing to add the pointer is a citation choice, not a mechanical edit. (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 3
- root-absent: 4

## Fixed in this run

*None.*
