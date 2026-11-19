---
spec: 081-reviews-and-checks-follow-what-changed
last-run: 2026-11-19T13:05:36Z
analyzed-against: 8792ca12000d52a9ab662bf84671bc4b202c29b9
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 8
analyzed-digest:
  plan.md: d18908f5f4b8b1e0f0cb2915ad381e2036ed29d7d5668ac2f9e5decfc2de5b34
  review.md: 4b29eba29ed0855dc13de2bdbc81b1d95e0896f7a855e33ed2c64b962c69ee34
  spec.md: 43a3c37a0cba9075e79dbb6b9cc84e3c15be891f63bf33fafa73ec70623cf984
  tasks.md: d992fa4122c9c50a4cb49cf2328dea0158f491b3da2b122e6c19a175088f2b70
unexamined-by-reason:
  not-a-live-claim: 3
  root-absent: 5
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
  BE-DEPS-007: {verdict: addressed, cite: "plan.md#Check selection in this repository's CI"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Credit for a read"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#The gate"}
  BE-SCHEMA-002: {verdict: addressed, cite: "plan.md#The change is breaking"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "spec.md#Applicable Rules"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#Check selection in this repository's CI"}
blocking: false
dispositions:
  fixed: 15
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: decision-reasons — Resolved Question 'Which paths are the project's artifacts?' rejects the spec root alone because it 'made editing the agent files owe a build', a cost of work started with no stated result the option would have delivered or the chosen one gives up (Decision reasons).
    outcome: waived
    reason: "Not the cost the rule targets: 081's subject is which checks a change owes, so the build an agent-file edit would owe under the spec-root-only set is the result this decision sets (AC6), not pipeline work the option would start. The entry states what the chosen set gives up (a project's own docs/ still runs code CI). Round 3's reader, over the same text, judged it a result reason. Rewording it is a decision, not a mechanical edit."
    decided-at: 2026-11-19T13:05:36Z
    decided-by: rule
---

# Analysis — 081-reviews-and-checks-follow-what-changed

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 8 unexamined target(s). Dispositions: 15 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- decision-reasons — Resolved Question 'Which paths are the project's artifacts?' rejects the spec root alone because it 'made editing the agent files owe a build', a cost of work started with no stated result the option would have delivered or the chosen one gives up (Decision reasons). — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **waived**: Not the cost the rule targets: 081's subject is which checks a change owes, so the build an agent-file edit would owe under the spec-root-only set is the result this decision sets (AC6), not pipeline work the option would start. The entry states what the chosen set gives up (a project's own docs/ still runs code CI). Round 3's reader, over the same text, judged it a result reason. Rewording it is a decision, not a mechanical edit. (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 3
- root-absent: 5

## Fixed in this run

- grounding — Resolved Questions, 'Which review owes the code of a commit whose subject names no spec?': 'the pipeline prescribes no commit-subject convention' is stated as fact about the existing pipeline with no citation to a command or artifact (the plan's Attribution section repeats it). — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **fixed** (decided by the rule)
- grounding — Resolved Questions, 'Which paths are the project's artifacts?': '.ductus/reference/ is generated from framework/reference/' is stated as fact about the existing system with no citation to the generator or installer. — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **fixed** (decided by the rule)
- grounding — Trade-offs, 'Repointing Family 19': 'Family 19 was built after gvrn-v0.26.1 and gvrn-v0.26.2 shipped with 022 scenarios added after its review' is a historical claim about the existing system with no cited commit, tag listing or artifact. — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — Attribution: 'Fix rounds and diff-cross-spec read CommitClaims but not attribute_window, so they are unchanged' is a claim about current code behavior that names symbols but carries no citation or marker showing it was checked. — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — Resolved Question 'Does a review that rewrites review.md still stale the analysis?' states that the gate already orders a reopening's review ahead of its analysis, with no citation. — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **fixed** (decided by the rule)
- grounding — Technical Decisions name symbols in claims about current code without their file: ProjectRepository::to_project, contracts_against_digest, review_freshness, attribute_window, split_changes, check-review-gate's module and ReviewGateBlock. — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — What makes a review owed names record_is_held, a closure 33688944 removed; the owed rule is owes_a_review in compute_review_scope.rs. — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — What makes a review owed says reviewed-against keeps its one reader, the rename exemption, while the credit for a read in compute_review_scope.rs reads it too. — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — Check selection in this repository's CI and Affected Files say framework-checks.yml and the release audit check out at depth 1, while both check out full history again (100d78c2, c016f6f5). — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — Attribution names first_named, which is no longer a symbol. — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — Overview says the workflows reuse a cached runtime binary, while they cache runtime/target, as the trade-off on caching only the binary says. — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — Resolved Question 'When a commit several specs claim changes code, who owes its review?' gives its 10-and-4 counts with no method. — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **fixed** (decided by the rule)
- grounding — Resolved Question 'Which review owes the code of a commit whose subject names no spec?' gives its 58-commit measurement with no method. — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **fixed** (decided by the rule)
- grounding — Check selection in this repository's CI gives its depth-1 measurement at debcc8cf with no method. — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — Check selection in this repository's CI: the claim that a checkout's fresh mtimes read to cargo as a change, so every file must be dated back before the restored build, describes cargo's external behavior and carries no citation, command or assumption marker (Grounding). — `specs/081-reviews-and-checks-follow-what-changed/plan.md` — **fixed** (decided by the rule)
