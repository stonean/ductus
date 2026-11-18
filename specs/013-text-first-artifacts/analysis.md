---
spec: 013-text-first-artifacts
last-run: 2026-11-18T21:00:07Z
analyzed-against: 43081c27d3619e53a7077149cd6e109f9f14f933
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 6
analyzed-digest:
  data-model.md: 351907a6128e8e325f6685f7bfedeed26efff4b46b3f76918a8bbca74e180dd8
  plan.md: 2c8f94c699f4cc0d2c61b462fc945c7b8d2fefff266c6e0cf1af5e7f8e833fc3
  review.md: d0bd15a2a3e186ed0ebddd6602710669e05a9bd6d129fa8a4eeb099b95ba667d
  scenarios/criterion-identifiers.md: d80a2e4ae75327ba6f941289e36f7268a1b6e5caa0e521fb6cc4979694a74e8c
  scenarios/past-tense-motivation-convention.md: 04b9583c6f2f4d56efa0cd7b544afd9811bd7925121ede083fbed9b6f2cdc1d4
  scenarios/the-precheck-ignores-what-the-same-run-wrote.md: 9b6bd04f749b64f5216d5dbde37787ba6681851147348ed3a6d0d9aa346336db
  spec.md: 2dc67e8e9c2b1da27e110e61346b33ed6ea6f9cd7d54f4500a9a14d72eeffb93
  tasks.md: cdfe9f4d86ae7966503b3229e70d29e5726443c5e7ce53c68bbe61d7d5bb4293
unexamined-by-reason:
  not-a-live-claim: 1
  root-absent: 1
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
  BE-INPUT-002: {verdict: addressed, cite: "spec.md#Frontmatter Schema"}
blocking: false
dispositions:
  fixed: 4
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 013-text-first-artifacts

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 6 unexamined target(s). Dispositions: 4 fixed, 0 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 1
- root-absent: 1
- ships-to-adopter: 4

## Fixed in this run

- grounding — §Problem said spec metadata 'today' is bold-prefix text parsed by every slash command, which the spec's own signpost and AC10 contradict (§Grounding (advisory)). — `specs/013-text-first-artifacts/spec.md` — **fixed** (decided by the rule)
- grounding — §Edge Cases 'Spec created manually outside /ductus:specify' described the template's `tags: []` placeholder and clarify's missing-tags finding, both of which 017 retired (§Grounding (advisory)). — `specs/013-text-first-artifacts/spec.md` — **fixed** (decided by the rule)
- grounding — Technical Decisions, Frontmatter format: 'Quartz, Obsidian, Logseq, MkDocs, Hugo, Jekyll, Eleventy all expect it' was stated with no source or assumption label (§Grounding (advisory)). — `specs/013-text-first-artifacts/plan.md` — **fixed** (decided by the rule)
- grounding — Trade-offs, Known limitations: 'git review tools handle this fine' was an uncited claim about external tools (§Grounding (advisory)). — `specs/013-text-first-artifacts/plan.md` — **fixed** (decided by the rule)
