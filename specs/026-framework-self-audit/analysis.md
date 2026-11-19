---
spec: 026-framework-self-audit
last-run: 2026-11-19T13:52:43Z
analyzed-against: 1c92e0a9b9f61e8d2ae46af1cfe42b6eb25f8f6f
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 43
analyzed-digest:
  plan.md: 24f26aa3009899d20702b630f90c2b5489d2507c397c7312d197f78af71e3297
  review.md: 390822fead1440a5ff6fb4d8ce15dd9364e06e094ae54abe0ad4b5d092fa452b
  scenarios/audit-ci-hard-gate.md: a7bad7a167532019112d79a746696ad32d963171598d2283072af0e9f3234be7
  scenarios/audit-script-refactors.md: 4d4a93fd968898006fa4780beb8088e48616ed1ceae54574ac38a3a02ad9583b
  scenarios/families-follow-the-command-reference-split.md: 12e2262fecdcf34d018c78f9dfe65fcebc769ba9eabfca3859c172163da4b073
  scenarios/family-10-migration-coverage.md: a535ac8c1e4dce26084c03066fc88978f0668f05e15f3600d87dbb6bf85ecf9a
  scenarios/family-17-contract-binding.md: 1ab3dfbc5c53ca5ab082b587a299b1951c8dd3c3e29148db5ed3a5df50903faf
  scenarios/family-18-marker-list-parity.md: 296f55313954ec984b5260945b92639d15b8458f707f90e94e0bd1bbc0f052c0
  scenarios/family-22-adopter-shell-behavior.md: 995c7c45e1918732b886e1174e5b627161bdd3c42ea824a4cbce7b04b70e02ff
  scenarios/family-23-sweep-target-manifest-parity.md: fabc89386b8c36156f41c09b5ae28e5f0cd3130c08abc284fa90dcd37507ebb1
  scenarios/family-24-rename-sweep-residue.md: 3b70ff8b57ef5bbd117ce0bd29752dd15a481d07e28a3fc3505671a2e6507cea
  scenarios/family-25-unbalanced-inline-markup.md: bddc8f9f37f21e3404f24bf77f732d2adc591faa76038983b3b44a41e640e1c6
  scenarios/family-26-broken-relative-links.md: 9e03cfe250dbffe6128229b1bb064c8b574b61ead93ed1ce5fd2fe12ec8e615d
  scenarios/family-27-done-spec-unchecked-criteria.md: c7c66e810ae27435c40bb372d7295b101f480e897f91a648db3d0902cacf57b1
  scenarios/family-28-audit-family-registry-parity.md: bf80f1d64718deb87a7dec79c69e9248e8c1ab4a41e65c638488fdd9fcd5e529
  scenarios/family-34-step-reference-integrity.md: 8598c2bd4072742f934254d0ea46094ec1388a6d0206defa6a123c0553e29015
  scenarios/family-35-manifest-destination-links.md: 28297923f8f5f4ec6833696028c4076e247cfadb3d945b0a2446acc18237cc11
  scenarios/family-36-self-url-resolution.md: e4c14882f66a90cee309b539ffe60ecfeef564eb7ad024ab39a537314c0e051f
  scenarios/family-39-shipped-anchor-resolution.md: c25191adf85b75351517d55c89a55a24a5d083e325f894f680c04ace54401e8f
  scenarios/family-40-deprecated-rules-are-removed-on-time.md: 689441c4d67ea3b73b147f0bf319db6ef7a50034ea46218ef67799dee0f5732e
  scenarios/host-namespace-parity.md: e8a67c95e2e596cf9a231bbfa26545c4beee96af1a043fd83608899e16087790
  scenarios/link-check-consolidation.md: edaa15335c49241c5565a0b4063a3f59a5233b5b4a0c6362cbe9f4356b82812f
  scenarios/readme-command-parity.md: 899fdc8ebb306541496892772ec4540b4be968aed55ee366c87185cdcc5304b6
  spec.md: 8004fa3202742f54227d651b14c0a663f9b9b6beab9423244f3ed7874c7353a3
  tasks.md: e2c71bb4ef7aadd29dfbf42eb4bbce549aea67fea36d6f6ab44e6e16e5efc460
unexamined-by-reason:
  never-existed: 4
  not-a-live-claim: 24
  root-absent: 10
  ships-to-adopter: 5
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
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Every family contains the paths it did not write as a literal"}
  SEC-DEPS-001: {verdict: addressed, cite: "plan.md#CI integration"}
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "grounding — AC22: the 031 and 041 incident (\"both carried `should-violations: 1` in `spec.md` while their own `review.md` recorded `0`, for weeks\", and the 031 root cause of a hand-moved waiver) is stated as fact about past artifacts with no commit, query or artifact citation (§Grounding)."
    outcome: waived
    reason: An acceptance criterion's text is not edited during an analysis, since the edit changes the criterion's digest and owes a new review, and the review's criteria step judges each criterion against the tree, as the owner decided for 020's AC15. AC22 is annotated superseded by 057, so the 031 and 041 incident it recounts is the historical record of why the retired check existed.
    decided-at: 2026-11-19T13:52:43Z
    decided-by: rule
---

# Analysis — 026-framework-self-audit

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 43 unexamined target(s). Dispositions: 3 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — AC22: the 031 and 041 incident ("both carried `should-violations: 1` in `spec.md` while their own `review.md` recorded `0`, for weeks", and the 031 root cause of a hand-moved waiver) is stated as fact about past artifacts with no commit, query or artifact citation (§Grounding). — `specs/026-framework-self-audit/spec.md` — **waived**: An acceptance criterion's text is not edited during an analysis, since the edit changes the criterion's digest and owes a new review, and the review's criteria step judges each criterion against the tree, as the owner decided for 020's AC15. AC22 is annotated superseded by 057, so the 031 and 041 incident it recounts is the historical record of why the retired check existed. (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 4
- not-a-live-claim: 24
- root-absent: 10
- ships-to-adopter: 5

## Fixed in this run

- grounding — Technical Decisions, Check zero (and Known limitation: new generators require manual update in two places): states as fact that Family 5 (template-validate alignment) does not catch a generator or lint added to one of the two lists and not the other because both files enumerate the same list textually, citing no script or symbol and carrying no assumption marker — `specs/026-framework-self-audit/plan.md` — **fixed** (decided by the rule)
- grounding — Every family contains the paths it did not write as a literal: names `bounded_lists::place` and `validate_contained` in a claim about current code without their files. — `specs/026-framework-self-audit/plan.md` — **fixed** (decided by the rule)
- grounding — Technical Decisions, Check zero: "`scripts/audit/check-zero.sh` builds the runtime before running them" is stated as fact about the existing script and cites no file-and-symbol, command or assumption marker (§Grounding). — `specs/026-framework-self-audit/plan.md` — **fixed** (decided by the rule)
