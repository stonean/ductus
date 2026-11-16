---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-16T17:59:10Z
analyzed-against: 80d0388c716ffe800b901915335bc10a1a7eb9c0
hard-fail: 0
blocking-findings: 0
advisory: 21
unexamined: 4
analyzed-digest:
  data-model.md: 1413afb3cec6f554723b56bcc6e190fc2140fe7ab52407e09e6f27984a75ba7f
  plan.md: d9e54e4df9faad19a1c35985727d8d372cfba5471916a3122bddb407f31e4d58
  research.md: 037fbe82665461d9f3e784ae20cf6daca658dfbeeafe38e7bff6db73b33d5b04
  review.md: 5d1790b2f5a934c6da0b44a79c05f1f55763159231cfc33a4ffc2f94af0e6d49
  spec.md: 2ba05b487bea7d5e4e1be36c1beb5ef3e09723f4653449fb568b4d88ccfc271b
  tasks.md: fce4651dd9262f9233e1cc8565d22fab75b9a006c7918ceca2412fd7d94438bd
unexamined-by-reason:
  anchor-target-unjudgeable: 1
  ships-to-adopter: 3
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
  BE-DEPS-007: {verdict: finding, tier: should, at: "plan.md#Affected Files", message: "The plan changes CI configuration (.github/workflows/framework-checks.yml step v) but commits to no automated vulnerability scan on a manifest or lockfile change."}
  BE-INPUT-001: {verdict: addressed, cite: "spec.md#Applicable Rules"}
  BE-INPUT-002: {verdict: addressed, cite: "spec.md#Applicable Rules"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#The workflow-step runner"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#The workflow-step runner"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#`measure-reopenings` (new primitive)"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#The workflow-step runner"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#The writers this spec changes are guarded where they are owned"}
  CFG-CONST-001: {verdict: addressed, cite: "data-model.md#Constants"}
  CFG-CONST-002: {verdict: addressed, cite: "data-model.md#Constants"}
  CFG-CONST-003: {verdict: addressed, cite: "data-model.md#Constants"}
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 19
  undispositioned: 2
decisions:
  - key: grounding — spec.md says 12 arguments of four primitives have no CLI flag and 15 command pointers offered the CLI, with no method
    outcome: waived
    reason: Neither bbff7a53's message nor research.md records how they were counted, re-counting reads the CLI schema, which is outside this command's scope, and the decision rests on the fixture at 46d0dd6e the next sentence cites.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — spec.md says three --help texts were one line of 52, 63 and 61 bytes, with no method
    outcome: waived
    reason: Measuring needs the binary's output at 46d0dd6e's parent, outside this command's scope, and the figures only illustrate a one-line description since replaced.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — spec.md says a rejected option retires 2,857 lines, with no method
    outcome: waived
    reason: 5f81add7, which wrote it, records no method, and the decision rests on the MUSTs the re-reads found.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — spec.md says a rejected wording saves 254 bytes against the chosen one's 38, with no method
    outcome: waived
    reason: The choice rests on each option naming what it gives up, not on the bytes.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — spec.md says 35 commits name a spec without touching its directory and 30 the reverse, with no method
    outcome: waived
    reason: b9077c1a, which wrote it, gives no method, re-deriving the counts is a new measurement, and the rule needs only that both kinds of commit exist.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — spec.md says renaming 078 meant a sweep of 13 files and 46 lines, with no method
    outcome: waived
    reason: Neither b13364b2 nor 50a63f60 records the method, the sweep was never made, and the decision rests on its three stated reasons.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — plan.md says 3 of 70 analysis records held a plan-record decision, with no method
    outcome: waived
    reason: 6aef0472, which wrote it, gives no method; re-counted there, a grep for plan-record matches 3 of 70 records and a count of its keys 2, and the decision rests on how rare such a decision is either way.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: artifact-size — `specs/079-review-and-analysis-work-converges/spec.md` is 87355 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: A trim of this size is no chore, and splitting this spec would part the criteria AC1 and AC20 measure together; the size blocks nothing.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — spec.md says the owner waived 14 observations the recommendations had proposed deferring, with no source
    outcome: waived
    reason: The 14 come from that session's recommendations, which no record keeps; 078's review at d62e72f3 holds 19 waivers a person made, a count that does not separate them, and the decision rests on a deferral into the reviewed spec guaranteeing another cycle, not on the count.
    decided-at: 2026-11-11T23:07:18Z
    decided-by: rule
  - key: grounding — spec.md says a pattern search found 21 boxes in 18 task lists, with no pattern or commit
    outcome: waived
    reason: The pattern was not recorded and the boxes it found were rewritten as prose, so re-deriving is a new measurement at a commit nobody recorded, and the decision rests on such boxes existing, which task 26's three further finds confirm.
    decided-at: 2026-11-11T23:07:18Z
    decided-by: rule
  - key: "rule-assessment — BE-DEPS-007: The plan changes CI configuration (.github/workflows/framework-checks.yml step v) but commits to no automated vulnerability scan on a manifest or lockfile change."
    outcome: waived
    reason: The step runs a repository script and alters no dependency set; runtime.yml's cargo audit job already scans every change to the runtime's manifest and lockfile, as 081's plan records, and 073's, 017's and 072's analyses waived this rule on the same kind of CI addition.
    decided-at: 2026-11-16T17:59:10Z
    decided-by: rule
  - key: grounding — spec.md Edge Cases (A stored finding the run did not read) says all 1,075 stored keys at f2e8c70a name a file, research.md, with no method
    outcome: waived
    reason: "Advisory and no chore: the commit is named but the query over stored keys is not"
    decided-at: 2026-11-16T17:59:10Z
    decided-by: rule
  - key: grounding — spec.md Resolved Questions (How does a person overrule a waiver the rule made?) says 6 rule decisions are stored, all in this spec's record, from one review run, with no command or query
    outcome: waived
    reason: "Advisory and no chore: no command or query is given for counting decisions stamped decided-by: rule"
    decided-at: 2026-11-16T17:59:10Z
    decided-by: rule
  - key: grounding — spec.md Resolved Questions (What does the measurement count as a person's decision?) says session transcripts hold 75 disposition prompts from 2026-10-13 to 2026-11-04 and none since, with no method for finding or counting them
    outcome: waived
    reason: "Advisory and no chore: the transcripts are not in the repository, and the search that found the prompts is not recorded"
    decided-at: 2026-11-16T17:59:10Z
    decided-by: rule
  - key: grounding — spec.md Resolved Questions (What is a review's scope...) says diff-cross-spec takes the median over the 37 open specs from 688 sibling paths to 51, with no method (the later 'Measured at d62e72f3' sentence covers the 11,979 and 2,688 figures, not this one)
    outcome: waived
    reason: "Advisory and no chore: the 688 and 51 are read by running diff-cross-spec on each open spec, which the entry does not say"
    decided-at: 2026-11-16T17:59:10Z
    decided-by: rule
  - key: grounding — spec.md Resolved Questions (Is the re-run after a stopped review over the bound?) says over all history 3 commits touch a budget stop and none a non-empty skipped-passes, with no method
    outcome: waived
    reason: "Advisory and no chore: the git query behind 'touch a budget stop' is not stated"
    decided-at: 2026-11-16T17:59:10Z
    decided-by: rule
  - key: grounding — spec.md Resolved Questions (Is the re-run after a cut-off review over the bound?) says one commit in all of history, 6aef0472, has carried the fix-rounds-open mark, with no method
    outcome: waived
    reason: "Advisory and no chore: the commit is named, but the search that shows it is the only one is not"
    decided-at: 2026-11-16T17:59:10Z
    decided-by: rule
  - key: decision-reasons — spec.md Resolved Questions (Does a record's commit leave code committed after the run unread?) rejects adding what lies between to the next window because a review preceding its commit would owe a second review for code it read, a review run as the stated reason
    outcome: waived
    reason: "Advisory and no chore: the entry gives 'a review ... owe a second review' as the reason for rejecting the option, with its line estimate beside it; the result the option would deliver (reading the in-between code) and the one the choice gives up are stated only as the gap's no-occurrence figures, so the entry would restate both to clear this"
    decided-at: 2026-11-16T17:59:10Z
    decided-by: rule
  - key: decision-reasons — spec.md Resolved Questions (What should reopen a done spec?) rejects keeping today's rule because it spends 47 analyses where they found almost nothing about the edit, a run count as the reason, though the findings-per-run figures give the option's result
    outcome: waived
    reason: "Borderline: the reason pairs run count with what those runs found (7 findings in 3 runs, none about the correction), which is a result measure beside the cost, so the decision is kept; flagged for re-examination only."
    decided-at: 2026-11-16T17:59:10Z
    decided-by: rule
---

# Analysis — 079-review-and-analysis-work-converges

## Summary

0 hard-fail, 0 blocking, 21 advisory; not blocking. 4 unexamined target(s). Dispositions: 2 fixed, 0 deferred, 19 waived, 2 undispositioned. An undispositioned advisory finding does not hold `done`; a hard-fail or blocking one holds it until the re-check no longer produces it.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: The plan changes CI configuration (.github/workflows/framework-checks.yml step v) but commits to no automated vulnerability scan on a manifest or lockfile change. — `specs/079-review-and-analysis-work-converges/plan.md` — **waived**: The step runs a repository script and alters no dependency set; runtime.yml's cargo audit job already scans every change to the runtime's manifest and lockfile, as 081's plan records, and 073's, 017's and 072's analyses waived this rule on the same kind of CI addition. (decided by the rule)
- grounding — spec.md says 12 arguments of four primitives have no CLI flag and 15 command pointers offered the CLI, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Neither bbff7a53's message nor research.md records how they were counted, re-counting reads the CLI schema, which is outside this command's scope, and the decision rests on the fixture at 46d0dd6e the next sentence cites.
- grounding — spec.md says three --help texts were one line of 52, 63 and 61 bytes, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Measuring needs the binary's output at 46d0dd6e's parent, outside this command's scope, and the figures only illustrate a one-line description since replaced.
- grounding — spec.md says a rejected option retires 2,857 lines, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: 5f81add7, which wrote it, records no method, and the decision rests on the MUSTs the re-reads found.
- grounding — spec.md says a rejected wording saves 254 bytes against the chosen one's 38, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: The choice rests on each option naming what it gives up, not on the bytes.
- grounding — spec.md says 35 commits name a spec without touching its directory and 30 the reverse, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: b9077c1a, which wrote it, gives no method, re-deriving the counts is a new measurement, and the rule needs only that both kinds of commit exist.
- grounding — spec.md says renaming 078 meant a sweep of 13 files and 46 lines, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Neither b13364b2 nor 50a63f60 records the method, the sweep was never made, and the decision rests on its three stated reasons.
- grounding — plan.md says 3 of 70 analysis records held a plan-record decision, with no method — `specs/079-review-and-analysis-work-converges/plan.md` — **waived**: 6aef0472, which wrote it, gives no method; re-counted there, a grep for plan-record matches 3 of 70 records and a count of its keys 2, and the decision rests on how rare such a decision is either way.
- grounding — spec.md says the owner waived 14 observations the recommendations had proposed deferring, with no source — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: The 14 come from that session's recommendations, which no record keeps; 078's review at d62e72f3 holds 19 waivers a person made, a count that does not separate them, and the decision rests on a deferral into the reviewed spec guaranteeing another cycle, not on the count.
- grounding — spec.md says a pattern search found 21 boxes in 18 task lists, with no pattern or commit — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: The pattern was not recorded and the boxes it found were rewritten as prose, so re-deriving is a new measurement at a commit nobody recorded, and the decision rests on such boxes existing, which task 26's three further finds confirm.
- grounding — spec.md Edge Cases (A stored finding the run did not read) says all 1,075 stored keys at f2e8c70a name a file, research.md, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Advisory and no chore: the commit is named but the query over stored keys is not (decided by the rule)
- grounding — spec.md Resolved Questions (How does a person overrule a waiver the rule made?) says 6 rule decisions are stored, all in this spec's record, from one review run, with no command or query — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Advisory and no chore: no command or query is given for counting decisions stamped decided-by: rule (decided by the rule)
- grounding — spec.md Resolved Questions (What does the measurement count as a person's decision?) says session transcripts hold 75 disposition prompts from 2026-10-13 to 2026-11-04 and none since, with no method for finding or counting them — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Advisory and no chore: the transcripts are not in the repository, and the search that found the prompts is not recorded (decided by the rule)
- grounding — spec.md Resolved Questions (What is a review's scope...) says diff-cross-spec takes the median over the 37 open specs from 688 sibling paths to 51, with no method (the later 'Measured at d62e72f3' sentence covers the 11,979 and 2,688 figures, not this one) — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Advisory and no chore: the 688 and 51 are read by running diff-cross-spec on each open spec, which the entry does not say (decided by the rule)
- grounding — spec.md Resolved Questions (Is the re-run after a stopped review over the bound?) says over all history 3 commits touch a budget stop and none a non-empty skipped-passes, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Advisory and no chore: the git query behind 'touch a budget stop' is not stated (decided by the rule)
- grounding — spec.md Resolved Questions (Is the re-run after a cut-off review over the bound?) says one commit in all of history, 6aef0472, has carried the fix-rounds-open mark, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Advisory and no chore: the commit is named, but the search that shows it is the only one is not (decided by the rule)
- grounding — spec.md says 68 of 70 records had no loop fields at 5f81add7, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **undispositioned**
- grounding — plan.md says a commit-claims estimate over the 126 reopenings found 12 owed a review only for an unclaimed commit, holding 3 of 261 runs, with no method — `specs/079-review-and-analysis-work-converges/plan.md` — **undispositioned**
- decision-reasons — spec.md Resolved Questions (Does a record's commit leave code committed after the run unread?) rejects adding what lies between to the next window because a review preceding its commit would owe a second review for code it read, a review run as the stated reason — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Advisory and no chore: the entry gives 'a review ... owe a second review' as the reason for rejecting the option, with its line estimate beside it; the result the option would deliver (reading the in-between code) and the one the choice gives up are stated only as the gap's no-occurrence figures, so the entry would restate both to clear this (decided by the rule)
- artifact-size — `specs/079-review-and-analysis-work-converges/spec.md` is 91364 bytes, 2 read pages at the 50000-byte read size — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: A trim of this size is no chore, and splitting this spec would part the criteria AC1 and AC20 measure together; the size blocks nothing.
- decision-reasons — spec.md Resolved Questions (What should reopen a done spec?) rejects keeping today's rule because it spends 47 analyses where they found almost nothing about the edit, a run count as the reason, though the findings-per-run figures give the option's result — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Borderline: the reason pairs run count with what those runs found (7 findings in 3 runs, none about the correction), which is a result measure beside the cost, so the decision is kept; flagged for re-examination only. (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1
- ships-to-adopter: 3

## Fixed in this run

- rule-assessment — BE-TXN-002: write-review, write-analysis and prune-plan read a project file and rewrite it, and neither plan.md nor data-model.md names a lost-update guard (compare-and-set helper, lock held across read and write, or departure entry) — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed**
- rule-assessment — BE-INPUT-004: The runner turns a workflow's job or step `working-directory:` (text committed to the repository) into the directory a step runs in, and the plan commits to no canonicalization or containment check against the repository root. — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed**
