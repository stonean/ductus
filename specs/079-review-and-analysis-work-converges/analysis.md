---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-20T00:39:18Z
analyzed-against: c66d13d502a07ed86b003ba4909a3587ee5d1791
hard-fail: 0
blocking-findings: 0
advisory: 5
unexamined: 3
analyzed-digest:
  data-model.md: 06298383706eaad589c60381deb6ec991aa75d1ee934006eeb8ff1d792290580
  plan.md: 70f90f080fcb0f8c72e1dae8d9e9e7b5ecf58ef913e6085fa33fb7d425fe2638
  research.md: d992eacf8fdd3a7568c8f20415bb7150eb7bb2581d9164fc55f23d60aa7b5952
  review.md: aa2cf09c438fb23f2a01f34366455790213a3bd9159dc21fa54d9cce28917158
  scenarios/the-excluded-section-lists-paths-only.md: 1763a8c3a9dd478a6f39d2813a73d4804c21dc8e9581c401af3fdaf35e6ea8b1
  spec.md: 019cec0a588becf44f550d3faa819134361b359b03760bd184f6e13930fa7ec0
  tasks.md: d7640c1757c590bf6492db53d28d7e89fbe553dbf5883bf920af96ea6cd31fae
unexamined-by-reason:
  anchor-target-unjudgeable: 1
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
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#The change is breaking"}
  BE-INPUT-001: {verdict: addressed, cite: "spec.md#Applicable Rules"}
  BE-INPUT-002: {verdict: addressed, cite: "spec.md#Applicable Rules"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#The workflow-step runner"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#The workflow-step runner"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#The scope"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#The writers this spec changes are guarded where they are owned"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#The workflow-step runner"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#The writers this spec changes are guarded where they are owned"}
  CFG-CONST-001: {verdict: addressed, cite: "data-model.md#Constants"}
  CFG-CONST-002: {verdict: addressed, cite: "data-model.md#Constants"}
  CFG-CONST-003: {verdict: addressed, cite: "data-model.md#Constants"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "spec.md#Edge Cases"}
  QUAL-DELEG-001: {verdict: addressed, cite: "spec.md#Resolved Questions"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#The measurement"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: decision-reasons — spec.md Resolved Questions (Does a record's commit leave code committed after the run unread?) rejects adding what lies between to the next window because a review preceding its commit would owe a second review for code it read, a review run as the stated reason
    outcome: waived
    reason: "Advisory and no chore: the entry gives 'a review ... owe a second review' as the reason for rejecting the option, with its line estimate beside it; the result the option would deliver (reading the in-between code) and the one the choice gives up are stated only as the gap's no-occurrence figures, so the entry would restate both to clear this"
    decided-at: 2026-11-16T17:59:10Z
    decided-by: rule
  - key: decision-reasons — spec.md Resolved Questions (Does the gate enforce a review that is owed?) rejects an option for 53 more reviews, 33 of them reading only another spec's text
    outcome: waived
    reason: "the run counts are this spec's result under design, review and analysis work converging within its bound: the answer weighs misses against AC1 and over-bound closings, measured over 126 closings, not the reopen an option would start"
    decided-at: 2026-11-18T13:12:40Z
    decided-by: rule
  - key: artifact-size — `specs/079-review-and-analysis-work-converges/spec.md` is 96541 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: A trim of this size is no chore, and splitting this spec would part the criteria AC1 and AC20 measure together; the size blocks nothing.
    decided-at: 2026-11-20T00:39:18Z
    decided-by: rule
  - key: "grounding — Trade-offs, the entry on a person's lasting keep of a plan section: '3 of 70 analysis records held any plan-record decision' gives a count with no query or method cited (Grounding, advisory)."
    outcome: waived
    reason: The count is the owner's 2026-11-05 measurement over the 70 analysis records then, which today's corpus no longer reproduces, and the decision it supports rests on analyze.md step 19 having no waiver path, cited beside it; re-deriving it changes nothing the plan decides.
    decided-at: 2026-11-20T00:39:18Z
    decided-by: rule
  - key: "grounding — Technical Decisions, the section on writers this spec changes: 'the rest only read, or write a lists file under .ductus/results/ past the read size' enumerates what every other changed primitive writes with no source cited (Grounding, advisory)."
    outcome: waived
    reason: The sentence states the other changed primitives' write sets, each in its compiled description (`ductus <name> --help`), which the plan's writers section points at; the enumeration adds no requirement, and a citation per primitive is no chore.
    decided-at: 2026-11-20T00:39:18Z
    decided-by: rule
---

# Analysis — 079-review-and-analysis-work-converges

## Summary

0 hard-fail, 0 blocking, 5 advisory; not blocking. 3 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 5 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- artifact-size — `specs/079-review-and-analysis-work-converges/spec.md` is 96541 bytes, 2 read pages at the 50000-byte read size — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: A trim of this size is no chore, and splitting this spec would part the criteria AC1 and AC20 measure together; the size blocks nothing. (decided by the rule)
- grounding — Trade-offs, the entry on a person's lasting keep of a plan section: '3 of 70 analysis records held any plan-record decision' gives a count with no query or method cited (Grounding, advisory). — `specs/079-review-and-analysis-work-converges/plan.md` — **waived**: The count is the owner's 2026-11-05 measurement over the 70 analysis records then, which today's corpus no longer reproduces, and the decision it supports rests on analyze.md step 19 having no waiver path, cited beside it; re-deriving it changes nothing the plan decides. (decided by the rule)
- grounding — Technical Decisions, the section on writers this spec changes: 'the rest only read, or write a lists file under .ductus/results/ past the read size' enumerates what every other changed primitive writes with no source cited (Grounding, advisory). — `specs/079-review-and-analysis-work-converges/plan.md` — **waived**: The sentence states the other changed primitives' write sets, each in its compiled description (`ductus <name> --help`), which the plan's writers section points at; the enumeration adds no requirement, and a citation per primitive is no chore. (decided by the rule)
- decision-reasons — Resolved Questions, 'Does a record's commit leave code committed after the run unread?': the rejected 'adding what lies between to the next window' is dismissed because it would have a review preceding its commit owe a second review, the review run being the stated reason; the result given up is not named. — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Advisory and no chore: the entry gives 'a review ... owe a second review' as the reason for rejecting the option, with its line estimate beside it; the result the option would deliver (reading the in-between code) and the one the choice gives up are stated only as the gap's no-occurrence figures, so the entry would restate both to clear this (decided by the rule)
- decision-reasons — Resolved Questions, 'Does the gate enforce a review that is owed?': 'leaving the Edge Case's literal test' is rejected for '53 more reviews, 33 of them reading only another spec's text', a review-run count being the reason; the result that test would deliver is not stated. — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: the run counts are this spec's result under design, review and analysis work converging within its bound: the answer weighs misses against AC1 and over-bound closings, measured over 126 closings, not the reopen an option would start (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1
- ships-to-adopter: 2

## Fixed in this run

*None.*
