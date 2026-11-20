---
spec: 051-branch-scoped-spec-numbering
last-run: 2026-11-20T00:23:43Z
analyzed-against: 9d7d98b0a7e346493b4cf6cff2081f47c37ff26e
hard-fail: 0
blocking-findings: 0
advisory: 5
unexamined: 4
analyzed-digest:
  data-model.md: 8e8baa256ea782bfb50a0620c598997cec119266cb41a5f8a0cd233879d6427a
  plan.md: 39b46c96f7085bf0b09a1af476a9931440292e396627615b7abe2e786708288d
  review.md: e97337ad35519fa7ef3d5df9725f01f3a9540499059c4ba9dd5060981b940aaf
  scenarios/a-definition-and-a-title-follow-the-link-grammar.md: fef8585771df20e1845e743ac4f6ef3d14684cabc8dc9f0b4caf4e8d3284b2e0
  scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md: 015b0ba736e27e91d0d86b37c65e6f7d0d947b5773cf6495313a8ff337b93a47
  scenarios/a-fold-target-is-held-to-the-name-grammar.md: dd15a827e2e30c67606ed2b49c965773d69e0356283264ed639f97c5f4adc901
  scenarios/a-link-inside-the-target-stays-inside-it.md: a367c3a7c1e260f98b27eb73a87958625422a4e458b758399fd5fa526eb2e1a4
  scenarios/a-pending-fold-never-holds-done.md: dc5c2daa3fd78c7251edf856c21f5d36737e3725d91c808231e112f84502e8b5
  scenarios/fold-target-checked-before-the-rewrite.md: 40bfa2011fb9af497ae85dac6f23f2ad8719dddcc8c5f5654dda7560aade7f32
  scenarios/rewrites-preserve-line-endings.md: 4389080643595553497a9a10a89e7320d157623df1a825094c2bb1599ba84d9c
  scenarios/the-numbering-grammar-reaches-every-surface.md: d4f11334754c65e3cf73346d224aaf8d782f5385a266a2bf22d3339a1079089a
  scenarios/the-spec-root-is-read-before-the-first-write.md: ccf10584e584b2f0f83808e7e1413a30cd98c1b5691a4c464da9432b0fd53ff9
  spec.md: 72917d3588e93fe6fde475d006583aa3b6e3be0c2ab94278ca9308637252113b
  tasks.md: 3fb8b973dd22053aa742ca213befef317799e4567add6db42b74a24a1024be3d
unexamined-by-reason:
  root-absent: 3
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
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "data-model.md#The write guard"}
  BE-ERR-002: {verdict: addressed, cite: "data-model.md#The write guard"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#`create-feature` gains optional arguments rather than a sibling primitive"}
  BE-INPUT-002: {verdict: addressed, cite: "spec.md#Branch-scoped numbers"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#`create-feature` gains optional arguments rather than a sibling primitive"}
  BE-LOCK-001: {verdict: addressed, cite: "data-model.md#The write guard"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#`rewrite-spec-links` (new)"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#`create-feature` gains optional arguments rather than a sibling primitive"}
  BE-TXN-002: {verdict: addressed, cite: "data-model.md#The write guard"}
  BE-TXN-004: {verdict: addressed, cite: "data-model.md#The write guard"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "grounding — Branch-scoped numbers: 'Teams name branches for their tracker ... a Jira branch carries PROJ-1111, a GitLab work-item branch 1111-PROJ' states what external trackers produce as fact, with no source or assumption marker (the same claim recurs in Resolved Questions, 'How is the branch identifier supplied?')."
    outcome: waived
    reason: "an operator-stated premise recorded as a decision in the spec's Resolved Questions: no repository artifact can source a tracker's branch-name format, and the sanitizing rule holds for any input"
    decided-at: 2026-11-20T00:23:43Z
    decided-by: rule
  - key: "grounding — Branch-scoped numbers (and Resolved Questions, 'What grammar constrains the operator-supplied identifier?'): 'every Jira-style identifier is uppercase' is a claim about an external system, uncited and unhedged, and it is the stated reason sanitizing replaced rejecting."
    outcome: waived
    reason: "an operator-stated premise recorded as a decision in the spec's Resolved Questions: no repository artifact can source a tracker's branch-name format, and the sanitizing rule holds for any input"
    decided-at: 2026-11-20T00:23:43Z
    decided-by: rule
  - key: "grounding — Branch-scoped numbers: 'identifiers differing only in case name one namespace, which is what a case-insensitive filesystem would enforce regardless' asserts filesystem behavior as fact with no citation or assumption marker."
    outcome: waived
    reason: a sentence stating a consequence the spec's own sanitizing rule already guarantees (AC12 holds by lowercasing), so the filesystem remark changes no behavior
    decided-at: 2026-11-20T00:23:43Z
    decided-by: rule
  - key: "grounding — Technical Decisions, create-feature subsection: 'A gate step dispatches unconditionally on the exec path' describes the exec walker's existing behavior and names no file, symbol or command for it; the re-blessed specify-basic.jsonl is cited only for the consequence."
    outcome: waived
    reason: the sentence's evidence is the re-blessed specify-basic.jsonl golden it cites, and the walker behavior is spec 022's, which owns it
    decided-at: 2026-11-20T00:23:43Z
    decided-by: rule
  - key: grounding — plan.md cites `.githooks/pre-commit:68`, a line number, in the rewrite-spec-links bullet
    outcome: waived
    reason: the citation names the hook's derive calls by position; check-artifacts' own line-number check, which enforces the rule, does not flag it, and fixing it would stale all three judges' reads for a locator that still resolves
    decided-at: 2026-11-20T00:23:43Z
    decided-by: rule
---

# Analysis — 051-branch-scoped-spec-numbering

## Summary

0 hard-fail, 0 blocking, 5 advisory; not blocking. 4 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 5 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Branch-scoped numbers: 'Teams name branches for their tracker ... a Jira branch carries PROJ-1111, a GitLab work-item branch 1111-PROJ' states what external trackers produce as fact, with no source or assumption marker (the same claim recurs in Resolved Questions, 'How is the branch identifier supplied?'). — `specs/051-branch-scoped-spec-numbering/spec.md` — **waived**: an operator-stated premise recorded as a decision in the spec's Resolved Questions: no repository artifact can source a tracker's branch-name format, and the sanitizing rule holds for any input (decided by the rule)
- grounding — Branch-scoped numbers (and Resolved Questions, 'What grammar constrains the operator-supplied identifier?'): 'every Jira-style identifier is uppercase' is a claim about an external system, uncited and unhedged, and it is the stated reason sanitizing replaced rejecting. — `specs/051-branch-scoped-spec-numbering/spec.md` — **waived**: an operator-stated premise recorded as a decision in the spec's Resolved Questions: no repository artifact can source a tracker's branch-name format, and the sanitizing rule holds for any input (decided by the rule)
- grounding — Branch-scoped numbers: 'identifiers differing only in case name one namespace, which is what a case-insensitive filesystem would enforce regardless' asserts filesystem behavior as fact with no citation or assumption marker. — `specs/051-branch-scoped-spec-numbering/spec.md` — **waived**: a sentence stating a consequence the spec's own sanitizing rule already guarantees (AC12 holds by lowercasing), so the filesystem remark changes no behavior (decided by the rule)
- grounding — Technical Decisions, create-feature subsection: 'A gate step dispatches unconditionally on the exec path' describes the exec walker's existing behavior and names no file, symbol or command for it; the re-blessed specify-basic.jsonl is cited only for the consequence. — `specs/051-branch-scoped-spec-numbering/plan.md` — **waived**: the sentence's evidence is the re-blessed specify-basic.jsonl golden it cites, and the walker behavior is spec 022's, which owns it (decided by the rule)
- grounding — plan.md cites `.githooks/pre-commit:68`, a line number, in the rewrite-spec-links bullet — `specs/051-branch-scoped-spec-numbering/plan.md` — **waived**: the citation names the hook's derive calls by position; check-artifacts' own line-number check, which enforces the rule, does not flag it, and fixing it would stale all three judges' reads for a locator that still resolves (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- root-absent: 3
- ships-to-adopter: 1

## Fixed in this run

*None.*
