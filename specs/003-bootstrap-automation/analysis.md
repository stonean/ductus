---
spec: 003-bootstrap-automation
last-run: 2026-11-18T21:17:21Z
analyzed-against: 025ff27b8247194eb251303764cf6929cb1ff66b
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 8
analyzed-digest:
  plan.md: 21263f4a01c1c43cf00456c89cb9577a98d6f57545e278a06c1aeb0c98de658d
  review.md: f4400fcd021b85ef4376ac9626bfc14e6ceb3ce432a3d6594430dc0408245d71
  scenarios/a-language-with-no-template-of-its-name.md: 3c0832c95ee24b41c8a7d2ee5c93a1c0721e523d93959c59fbbefbdaadae11eb
  scenarios/curl-sh-installer.md: 750384f78ec1ab55419f5274f027f06757469fff6836f6be8ad9021c8f8c707c
  spec.md: d1cd40cdb78144289eeed555a1dac9ab493fabb6fd1b052c104cc3d7294d9c54
  tasks.md: d2c5429f7b20b6171b9cfb894ecc07e0701ff2a426e69aba789b0a0f9856d8ae
unexamined-by-reason:
  not-a-live-claim: 2
  root-absent: 2
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
  BE-DATA-001: {verdict: addressed, cite: "plan.md#The per-language `.gitignore` fetch is bounded"}
  BE-INPUT-002: {verdict: addressed, cite: "scenarios/a-language-with-no-template-of-its-name.md#Edge Cases"}
  BE-INPUT-003: {verdict: addressed, cite: "scenarios/a-language-with-no-template-of-its-name.md#Edge Cases"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#The per-language `.gitignore` fetch is bounded"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#The per-language `.gitignore` fetch is bounded"}
blocking: false
dispositions:
  fixed: 7
  deferred: 0
  waived: 0
  undispositioned: 0
section-digest:
  "plan.md": 989cfcf0e576f1a0
  "plan.md#003 — Bootstrap Automation Plan": 9ffe15065db09f37
  "plan.md#Affected Files": 4a83783a662d47fe
  "plan.md#Considered: a single `/ductus:work` command instead of ten standard commands": 9fd455552604d57b
  "plan.md#Considered: generating standard commands dynamically from templates": 3a951804d0aedff9
  "plan.md#Considered: skipping the configure command for this repo": e283272ef2cdc77b
  "plan.md#Open Questions Resolved": 06f01e106515972f
  "plan.md#Overview": 7ed9b624e688a712
  "plan.md#Session file path": 8bc97427b3605f5d
  "plan.md#Standard commands are literal copies with placeholder replacement": c14b0e7e2ae93d8d
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The per-language `.gitignore` fetch is bounded": c83c8e41a8d593f0
  "plan.md#Trade-offs": 6d48e49804e54ced
  "scenarios/a-language-with-no-template-of-its-name.md": 78f9168eaf27502c
  "scenarios/a-language-with-no-template-of-its-name.md#A-language-with-no-template-of-its-name": e91e4013f2736add
  "scenarios/a-language-with-no-template-of-its-name.md#Behavior": 32f950a3ab396f9f
  "scenarios/a-language-with-no-template-of-its-name.md#Context": 92814468a67f6ab1
  "scenarios/a-language-with-no-template-of-its-name.md#Edge Cases": b91d4a4bf9857a97
  "scenarios/a-language-with-no-template-of-its-name.md#Open Questions": 20edda8881516e4e
  "scenarios/a-language-with-no-template-of-its-name.md#Resolved Questions": 5129626a9c67a350
  "scenarios/curl-sh-installer.md": 50d3794a575f214e
  "scenarios/curl-sh-installer.md#Behavior": b1b269d26dca20d1
  "scenarios/curl-sh-installer.md#Context": b8720edf3e88642e
  "scenarios/curl-sh-installer.md#Edge Cases": 163a0af1a4b3b15a
  "scenarios/curl-sh-installer.md#One-line installer (curl | sh)": bae08673560c030a
  "scenarios/curl-sh-installer.md#Open Questions": 545eadd502d01a7c
  "scenarios/curl-sh-installer.md#Resolved Questions": e267b4b49a06485a
  "spec.md": f20d50202826771a
  "spec.md#/ductus:init": 0be4071270bcd3eb
  "spec.md#003 — Bootstrap Automation": 87da8f334729d41b
  "spec.md#Acceptance Criteria": 22162e6761ba81f1
  "spec.md#Behavior": 29ab98cdbf8ff7c1
  "spec.md#Inputs": 2e1bd83b429790b5
  "spec.md#Pre-flight Check": 96aaeba34680aa3a
  "spec.md#Problem": 4403a6978bc023ae
  "spec.md#References": 0f29184bdab1595d
  "spec.md#Resolved Questions": 3d291d716dfc0b0c
  "spec.md#Scaffolding Steps": 3b2c4677b390483d
  "spec.md#Standard pipeline commands": c29b9a20f69d3f91
  "spec.md#What it does NOT do": 17e27eb41568d1de
---

# Analysis — 003-bootstrap-automation

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 8 unexamined target(s). Dispositions: 7 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

- not-a-live-claim: 2
- root-absent: 2
- ships-to-adopter: 4

## Fixed in this run

- grounding — Problem said the README describes a 7-step manual bootstrap, uncited and in the present tense (§Grounding (advisory)). — `specs/003-bootstrap-automation/spec.md` — **fixed** (decided by the rule)
- grounding — Problem said ductus itself has no slash commands, contradicted by the spec's own Note (§Grounding (advisory)). — `specs/003-bootstrap-automation/spec.md` — **fixed** (decided by the rule)
- grounding — The retirement note said init had no Shared Files manifest row, naming nothing that shows the absence (§Grounding (advisory)). — `specs/003-bootstrap-automation/spec.md` — **fixed** (decided by the rule)
- grounding — The retirement note said /{project}:review's tech-stack alignment check reads the Tech Stack table, with no file cited (§Grounding (advisory)). — `specs/003-bootstrap-automation/spec.md` — **fixed** (decided by the rule)
- grounding — The reversal Trade-off said the pre-commit hook runs the generator and check-zero fails on a stale copy, citing neither (§Grounding (advisory)). — `specs/003-bootstrap-automation/plan.md` — **fixed** (decided by the rule)
- grounding — The reversal Trade-off said the command set grew from ten to sixteen, with no count method (§Grounding (advisory)). — `specs/003-bootstrap-automation/plan.md` — **fixed** (decided by the rule)
- plan-consistency — The 'Session file path' Technical Decision was a bare statement with no rationale (§Spec integrity, rationale). — `specs/003-bootstrap-automation/plan.md` — **fixed** (decided by the rule)
