---
spec: 082-rule-assessments-converge-across-runs
last-run: 2026-11-12T21:09:11Z
analyzed-against: 9c9a7e4162274fdba52c547f5bf878a20fbf294a
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 1
analyzed-digest:
  data-model.md: f2342da1234ffcbc2c2f76156b21515970299eff57dc0ba3a2e29dccf32c29e8
  plan.md: 6e9d20a39c7e4c161d56a49606d810cee2c87e1a56bb8a61660e46b81d7cf11a
  review.md: 9856afd6f14e627c482233a0e9e6e8ef618c317f8c055aab8211687470bfe288
  spec.md: d7332be91a71e0bb86d2b82254e6c34ae169866a6e497a3309df5c19c4aee53c
  tasks.md: c3c61e9c4b9a74870902cfa60295d04d8001fa1156bacc81869fb2b65f6bfbe9
unexamined-by-reason:
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
  security-backend.md: d89d6dad3ab51586ae4e68944b0c8e9ad4d5da721560f10590de3cdecc904d3f
  security-cross.md: dc98261801405fccaf5de9eb8f8ed9b020ebc78fcb5bfb124778a5fcc287950b
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-CACHE-001: {verdict: addressed, cite: "plan.md#A stored verdict is current while four things match"}
  BE-CACHE-002: {verdict: addressed, cite: "plan.md#A stored verdict is current while four things match"}
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Primitive interface changes, classified under `BE-COMPAT-001`"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#`write-analysis` derives the rest"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#An `addressed` citation is checked by `write-analysis`"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#An `addressed` citation is checked by `write-analysis`"}
  BE-INPUT-008: {verdict: addressed, cite: "data-model.md#Record fields"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#A contradiction is derived from the prior record"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#`write-analysis` derives the rest"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "plan.md#`write-analysis` derives the rest"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Affected Files"}
blocking: false
dispositions:
  fixed: 8
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 082-rule-assessments-converge-across-runs

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 1 unexamined target(s). Dispositions: 8 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

- ships-to-adopter: 1

## Fixed in this run

- rule-assessment — BE-INPUT-004: the plan checked an addressed citation against the design subjects but committed to no containment of the host-supplied path (canonical components, no .., no symlink) — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md's Overview said a stored verdict is reused while the design subjects and the rule file are unchanged; the loaded rule-file set is a fourth input — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md's discover-rule-files decision listed two reason values and put a tier on every reusable entry; four values exist and only a finding carries a tier — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md said record_reused counts a reused finding by the rule's tier; it counts it in its stored tier — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md's tier-floor bullet ran the exec tier rule into the mixed-rule rationale, leaving a clause that refers to nothing — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md said a prior record that does not parse is overwritten; one whose frontmatter does not parse is refused, and only one whose rule fields do not is overwritten — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md's cross-spec section said 082's cross-spec-impact does not list 022; it does — `specs/082-rule-assessments-converge-across-runs/plan.md` — **fixed** (decided by the rule)
- grounding — spec.md's Resolved Question said the exec walker asks about every loaded rule; since this spec it skips a rule whose verdict is current — `specs/082-rule-assessments-converge-across-runs/spec.md` — **fixed** (decided by the rule)
