---
spec: 073-decided-rule-departures-are-recorded
last-run: 2026-11-13T20:03:20Z
analyzed-against: 7667bd370b03e719c18afb9425e5cec4d8a7c080
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 2
analyzed-digest:
  data-model.md: cc819aa341154309c3bf65ffb9ab3541b4bc938a8b70a51ff3baebe85e2b076f
  plan.md: c6c51e942fb759db29360cbb61fa12bab8d61c9ddde4c98782ab0db27b531ee3
  review.md: 4e859d79dae26e42931a05a0e60cd603ae83406f9bcf7e950b48877ca6bf2a52
  spec.md: 126fc9ed63cbb53f9c2d7a989c7ee187ca4c36fdd40ddad08d6edbd800b2c48c
  tasks.md: db6f8922786b66843bd50a06318ae26a15116d32b1dd91a80077f1d7080972e0
unexamined-by-reason:
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
  security-backend.md: d89d6dad3ab51586ae4e68944b0c8e9ad4d5da721560f10590de3cdecc904d3f
  security-cross.md: dc98261801405fccaf5de9eb8f8ed9b020ebc78fcb5bfb124778a5fcc287950b
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-AUTHZ-007: {verdict: addressed, cite: "data-model.md#Departure entry"}
  BE-DEPS-007: {verdict: finding, tier: should, at: "plan.md#The test that reads history runs in the job that has it", message: "the plan adds a framework-checks.yml step, CI configuration, and does not commit to an automated scan on dependency changes"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#Departure entry"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Departure entry"}
  BE-INPUT-008: {verdict: addressed, cite: "data-model.md#Departure entry"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#Review: `write-review` derives the waivers, `process-waivers` expires them"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#One reader, one attribution rule, four consumers"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 3
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-DEPS-007: the plan adds a framework-checks.yml step, CI configuration, and does not commit to an automated scan on dependency changes"
    outcome: waived
    reason: The step runs a repository script and alters no dependency set; runtime.yml's cargo audit job already scans every change to the runtime's manifest and lockfile, as 081's plan records. 017's and 072's analyses waived this rule on the same kind of CI addition.
    decided-at: 2026-11-13T18:42:18Z
    decided-by: rule
  - key: "grounding — plan.md Technical Decisions ('One reader, one attribution rule'): states how CommonMark renders lazy continuation, setext underlines and ordered-item openers, with no citation to the CommonMark specification"
    outcome: waived
    reason: The sentences define the entry parser's own contract (what it joins and where a child ends), which is prescriptive; CommonMark is named as the model it follows, and task 9 pins each choice with a test and mutant, so the behavior does not rest on the recollection.
    decided-at: 2026-11-13T18:42:18Z
    decided-by: rule
  - key: "grounding — spec.md Motivation: the adopter's two MUST departures (compatibility and idempotency) and its shared constitution's three advance decisions are stated as fact with no citation or assumption marker"
    outcome: waived
    reason: "The source is an adopter report from a tree nobody here can open (AGENTS.md: an adopter's tree is out of reach, and no project is named); the requirement stands on the first Motivation paragraph's dated, cited reading of the code, not on this account."
    decided-at: 2026-11-13T18:42:18Z
    decided-by: rule
---

# Analysis — 073-decided-rule-departures-are-recorded

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 2 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: the plan adds a framework-checks.yml step, CI configuration, and does not commit to an automated scan on dependency changes — `specs/073-decided-rule-departures-are-recorded/plan.md` — **waived**: The step runs a repository script and alters no dependency set; runtime.yml's cargo audit job already scans every change to the runtime's manifest and lockfile, as 081's plan records. 017's and 072's analyses waived this rule on the same kind of CI addition. (decided by the rule)
- grounding — plan.md 'One reader, one attribution rule': states how CommonMark renders a lazy continuation line, a setext underline (`---`) and an indented opener as fact, with no cited source (no spec link, command or named standard section) — `specs/073-decided-rule-departures-are-recorded/plan.md` — **waived**: The sentences define the entry parser's own contract (what it joins and where a child ends), which is prescriptive; CommonMark is named as the model it follows, and task 9 pins each choice with a test and mutant, so the behavior does not rest on the recollection. (decided by the rule)
- grounding — spec.md Motivation, second paragraph: states an adopter's spec listed two MUST rules as departures and recorded blocking-findings 0, and that its shared constitution decided three MUST departures in advance, with no citation or assumption marker (the first paragraph carries 'verified 2026-10-26 at HEAD c0952038') — `specs/073-decided-rule-departures-are-recorded/spec.md` — **waived**: The source is an adopter report from a tree nobody here can open (AGENTS.md: an adopter's tree is out of reach, and no project is named); the requirement stands on the first Motivation paragraph's dated, cited reading of the code, not on this account. (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- root-absent: 2

## Fixed in this run

*None.*
