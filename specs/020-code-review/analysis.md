---
spec: 020-code-review
last-run: 2026-11-15T17:43:45Z
analyzed-against: 1f658b89f5cfd4f79be6a65e0a6005c21e373065
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 3
analyzed-digest:
  data-model.md: a0ee875255692f13e34ea20dd9c584189028c2912e7888bc58f7b62557d0efeb
  plan.md: 436808aa63c62c4e75fae30ac7a332e3574663108c0859fb8e369ca8b10be877
  review.md: 829fdd861ef36fcbdfa06afd3a6ccf11662cc7c9afefd2a66e030cd8a992fc29
  scenarios/every-loaded-rule-file-has-a-pass.md: 4e31f3bd59ad2c2145b8cb24ceaf8561580ffab2ad3143de1087f65b5109a0f3
  scenarios/review-flag-parsing-is-specified.md: a19fe0f4f50c954363f7cca4d913a81d96374535d31a986792243be5263631c1
  scenarios/review-reports-a-defect-under-its-owner.md: ef91f01e38d96520853db62f5ee62b9b8893d41582f055f4f01a25d5a796b665
  scenarios/waiver-expiry.md: 0c35cbf8e26a8a6f321c8549b7209c61b37122713bf13270e5b3c972b788f4ed
  scenarios/waiver-file-lists.md: 8b4180d64e4e43ac6a66f57df7c65d6f1f00bb343ae284bb2d7214c88c824f53
  spec.md: d62a519c40518cfb40140211ed1b80a7ae592d0ba6e217df15db5620326a1d2b
  tasks.md: 186082a0e82e5cbc34e05f6d98ea4ed45f3385887557a75a016121c4ab7934f3
unexamined-by-reason:
  never-existed: 1
  not-a-live-claim: 1
  root-absent: 1
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
  BE-DEPS-007: {verdict: finding, tier: should, at: "plan.md#CI template stays minimal", message: "The plan adds a step to framework/templates/ci/adopter-generators.yml (CI configuration) and commits to no automated vulnerability scan on dependency-set changes."}
  BE-PAYLOAD-001: {verdict: finding, tier: must, at: "plan.md#Rule files map to passes in one compiled table, and the reference states it in prose a test holds to it", message: "performReview request lists rule-files and loaded-rule-files are bounded by no stated maximum or lists-file path (only process-waivers' lists are bounded in plan.md)"}
  CFG-ENV-001: {verdict: addressed, cite: "data-model.md#`.ductus/config.toml [review]` section"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#`.ductus/config.toml` follows the shared-database convention"}
  CFG-ENV-012: {verdict: finding, tier: should, at: "data-model.md#`.ductus/config.toml [review]` section", message: "The new [review] section is declared open-schema but the design does not state how its loader treats an unrecognized key (fail startup or warn by name)."}
blocking: false
dispositions:
  fixed: 0
  deferred: 1
  waived: 3
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-DEPS-007: The plan adds a step to framework/templates/ci/adopter-generators.yml (CI configuration) and commits to no automated vulnerability scan on dependency-set changes."
    outcome: waived
    reason: "advisory and no chore: 020's CI work adds the audit-record gate step to framework/templates/ci/adopter-generators.yml, which runs the already-installed ductus runtime and installs nothing, so no dependency set changes; cargo audit runs in runtime.yml on every runtime/** change (as 026's analysis waived the same finding)"
    decided-at: 2026-11-15T17:43:45Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — CFG-ENV-012: The new [review] section is declared open-schema but the design does not state how its loader treats an unrecognized key (fail startup or warn by name)."
    outcome: waived
    reason: "advisory and no chore: the loader behavior the rule asks for exists where it is owned. runtime/src/schema/config_keys.rs registers [review]'s keys, and every reader of the section, discover-rule-files among them, warns by name on each key outside it ('unknown config key: [review] … — this version of ductus does not read it'), per spec 019's scenario an-unknown-config-key-is-named and 022's config-readers-name-unknown-keys; a warning rather than a failure is the rule's own multi-version case, since teammates' ductus versions read one committed file"
    decided-at: 2026-11-15T17:43:45Z
    decided-by: andy@stone.dev
  - key: "rule-assessment — BE-PAYLOAD-001: performReview request lists rule-files and loaded-rule-files are bounded by no stated maximum or lists-file path (only process-waivers' lists are bounded in plan.md)"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    task: "36"
    reason: the exec extension request shapes are 022's protocol; the owner deferred this finding to 022 task 36, widened to name performReview's request (2026-11-15)
    decided-at: 2026-11-15T17:43:45Z
    decided-by: andy@stone.dev
  - key: grounding — Resolved Question 'MUST waivers and second sign-off' states as fact that 'Ductus has no runtime that could verify a second value belongs to a different person' with no citation or assumption marker; the runtime (spec 022) now exists and the claim is not substantiated
    outcome: waived
    reason: "advisory and no chore: what the decision turns on still holds. A second frontmatter value is self-asserted, and nothing the runtime (spec 022) reads, frontmatter or git's configured user.email for decided-by, attests that it names a different person, so the requirement would still be performative; the real second review stays at merge"
    decided-at: 2026-11-15T17:43:45Z
    decided-by: andy@stone.dev
---

# Analysis — 020-code-review

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 3 unexamined target(s). Dispositions: 0 fixed, 1 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-DEPS-007: The plan adds a step to framework/templates/ci/adopter-generators.yml (CI configuration) and commits to no automated vulnerability scan on dependency-set changes. — `specs/020-code-review/plan.md` — **waived**: advisory and no chore: 020's CI work adds the audit-record gate step to framework/templates/ci/adopter-generators.yml, which runs the already-installed ductus runtime and installs nothing, so no dependency set changes; cargo audit runs in runtime.yml on every runtime/** change (as 026's analysis waived the same finding)
- rule-assessment — CFG-ENV-012: The new [review] section is declared open-schema but the design does not state how its loader treats an unrecognized key (fail startup or warn by name). — `specs/020-code-review/data-model.md` — **waived**: advisory and no chore: the loader behavior the rule asks for exists where it is owned. runtime/src/schema/config_keys.rs registers [review]'s keys, and every reader of the section, discover-rule-files among them, warns by name on each key outside it ('unknown config key: [review] … — this version of ductus does not read it'), per spec 019's scenario an-unknown-config-key-is-named and 022's config-readers-name-unknown-keys; a warning rather than a failure is the rule's own multi-version case, since teammates' ductus versions read one committed file
- grounding — Resolved Question 'MUST waivers and second sign-off' states as fact that 'Ductus has no runtime that could verify a second value belongs to a different person' with no citation or assumption marker; the runtime (spec 022) now exists and the claim is not substantiated — `specs/020-code-review/spec.md` — **waived**: advisory and no chore: what the decision turns on still holds. A second frontmatter value is self-asserted, and nothing the runtime (spec 022) reads, frontmatter or git's configured user.email for decided-by, attests that it names a different person, so the requirement would still be performative; the real second review stays at merge

## Deferred to other specs

- rule-assessment — BE-PAYLOAD-001: performReview request lists rule-files and loaded-rule-files are bounded by no stated maximum or lists-file path (only process-waivers' lists are bounded in plan.md) — `specs/020-code-review/plan.md` — **deferred** to `specs/022-deterministic-runtime/tasks.md` task 36

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- not-a-live-claim: 1
- root-absent: 1

## Fixed in this run

*None.*
