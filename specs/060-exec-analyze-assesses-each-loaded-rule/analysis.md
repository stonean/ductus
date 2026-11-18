---
spec: 060-exec-analyze-assesses-each-loaded-rule
last-run: 2026-11-18T18:47:45Z
analyzed-against: 76d5449ec6bd987b1e195b0479d971b3db4429ac
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 2
analyzed-digest:
  plan.md: 1b71bf0f0c4d5e4f831cdb12b8b55e3c68977c80d3d0115e331a3795a0093123
  review.md: 212fbcad0c20495a3725cdf152cb95c16a5b27a3e9a3b63d6c7edd35b20332a7
  scenarios/an-assessment-sees-the-loaded-rule-files.md: 7a1341466362f69527aecd8d09472a6d9ba5ec6d9db5e30a0e316814fae06007
  scenarios/an-assessment-sees-the-rule-it-asks-about.md: 03489f5510e4b9cf5f38194f6a0e8ab6ffcff7272359914c991129ba72a9b70f
  scenarios/rule-judging-runs-from-repo-scripts.md: 19aaa6a8d61cbcfb40306a9c46c2e264d1d61d35b1b322bf38686e7abc93cd96
  spec.md: 36c10fe0d8fed3f7c83441ef040a90daa8e0683e8918e358851f5d8d5058d6f5
  tasks.md: e0bba6cc86e3687c9b3bbfa635cecb5c15940a7880f95bb6c5df734c8f589d4b
unexamined-by-reason:
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
  BE-ERR-002: {verdict: addressed, cite: "spec.md#Edge Cases"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#One request per rule of the step's tier"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#One rule-section parser, sharing check-rule-ids' heading grammar"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#The walker loads the rule set once, on the first assessment step"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#One request per rule of the step's tier"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  CFG-CONST-004: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  CFG-ENV-004: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  CFG-ENV-005: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  CFG-ENV-007: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
blocking: false
dispositions:
  fixed: 6
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 060-exec-analyze-assesses-each-loaded-rule

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 2 unexamined target(s). Dispositions: 6 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

- ships-to-adopter: 2

## Fixed in this run

- grounding — Trade-offs, 'Round trips and payload': 'Each request puts `spec-content` before `rule`' is stated as fact about the request's field order with no citation to the type or symbol that fixes it (§Grounding (advisory)). — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-INPUT-003: The scripts shell out to the runtime and git with a feature argument and env settings, but the plan names no safe-invocation mechanism (argument vector, no shell string). — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — **fixed** (decided by the rule)
- rule-assessment — CFG-ENV-005: ANALYZE_TIMEOUT_SECONDS has a stated upper bound (MAX_TIMEOUT_SECONDS) but the plan names no startup range check whose error names the setting, offending value and violated bound. — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — **fixed** (decided by the rule)
- rule-assessment — CFG-ENV-007: Settings with a flag (--cell-bytes) or env var plus a default constant state no resolution order and no confirmation that it follows flag > environment > default. — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — **fixed** (decided by the rule)
- rule-assessment — CFG-ENV-008: The plan names no type for DUCTUS_BIN, ANALYZE_WORK or ANALYZE_TIMEOUT_SECONDS and commits to no startup parse that fails fast on a present but malformed value. — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-PAYLOAD-001: the plan stated no size bound for the per-rule assessSpecQuality request stream, which AGENTS.md says the rule reaches on the ductus exec protocol — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — **fixed** (decided by the rule)
