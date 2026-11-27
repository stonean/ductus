---
spec: 060-exec-analyze-assesses-each-loaded-rule
last-run: 2026-11-27T18:40:45Z
analyzed-against: 1ea9cae88ff90a6697ae7f384ab3deab32ad8c25
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 2
analyzed-digest:
  plan.md: 64b592eca46a034438acc2a23e804a0d596a02633550ed9a9a3c1db02f34d60d
  review.md: 043ccdcfd884cf62b3fdaf6a12cef214d257d608072ab57d8fb17afd9724f44f
  scenarios/an-assessment-sees-the-loaded-rule-files.md: 7a1341466362f69527aecd8d09472a6d9ba5ec6d9db5e30a0e316814fae06007
  scenarios/an-assessment-sees-the-rule-it-asks-about.md: 03489f5510e4b9cf5f38194f6a0e8ab6ffcff7272359914c991129ba72a9b70f
  scenarios/rule-judging-runs-from-repo-scripts.md: 3c3d8b962193afb84c80fddaf0d8e6144baa956d4f039fdb27e5dcca6593c08b
  spec.md: 69b39c7b6c05777307564e1a104509d9481ecb365ba620c4da43189e3799cd53
  tasks.md: bba8b81966bb671a89dface2de0fe811b70835dca16cd1e3df28bd89e7d2f943
unexamined-by-reason:
  ships-to-adopter: 2
rule-digest:
  accessibility-frontend.md: 20fb956e35bb749f061090adf631329bafe3e37662a4e47b0076b697784bbf50
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 050a5b8dd375fe4790f5d92753db27ae53ac730db1557cbee84b923b72499ca9
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: c9d90cf6bfa6e4f5492e6d1a49e073fbee467ba08c7788bf6c7224ae8bd5d411
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 9a200916d11a5597a6860d1538840bf7d9aa573bddce953a0bef604379d1451b
section-digest:
  "plan.md": e3b0c44298fc1c14
  "plan.md#022 is reopened, corrected, and re-reviewed": 6a9113f8a4314186
  "plan.md#060 — Exec analyze assesses each loaded rule Plan": 700ffe8a0753820b
  "plan.md#Affected Files": 59e9fb05a1e06508
  "plan.md#Documentation of the per-rule walk": c18402ec1508d2c8
  "plan.md#One request per rule of the step's tier": e14bd3e896fe51fa
  "plan.md#One rule-section parser, sharing check-rule-ids' heading grammar": 8dd30fb2d5019d9f
  "plan.md#Overview": 967b758609c8b336
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The analyze-basic fixture's rule gains a keyword": 92ef6b31cff3da05
  "plan.md#The tally takes the tier from the rule it asked about": 46168e9250f2c73b
  "plan.md#The unexamined vocabulary: one new reason, one redefined": 522447290e80f751
  "plan.md#The walker loads the rule set once, on the first assessment step": 3d99acfcb0579984
  "plan.md#This repository's host runs its rule judging from `scripts/analyze/`": da14220e2798b731
  "plan.md#Trade-offs": 35c11c59b30a3025
  "scenarios/an-assessment-sees-the-loaded-rule-files.md": b2cdecf1e3d2f467
  "scenarios/an-assessment-sees-the-loaded-rule-files.md#An-assessment-sees-the-loaded-rule-files": 496fc3f18870abe3
  "scenarios/an-assessment-sees-the-loaded-rule-files.md#Behavior": ca4461648bd83f4a
  "scenarios/an-assessment-sees-the-loaded-rule-files.md#Context": 4acedbd5beb7aba9
  "scenarios/an-assessment-sees-the-loaded-rule-files.md#Edge Cases": 2a5a73b9333344e7
  "scenarios/an-assessment-sees-the-loaded-rule-files.md#Open Questions": 9fc5979d590cdb97
  "scenarios/an-assessment-sees-the-loaded-rule-files.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/an-assessment-sees-the-rule-it-asks-about.md": 5473ad62d52d0e72
  "scenarios/an-assessment-sees-the-rule-it-asks-about.md#An-assessment-sees-the-rule-it-asks-about": 15d37e917fce8992
  "scenarios/an-assessment-sees-the-rule-it-asks-about.md#Behavior": 38e91d959278582c
  "scenarios/an-assessment-sees-the-rule-it-asks-about.md#Context": 12060fc03858b3c1
  "scenarios/an-assessment-sees-the-rule-it-asks-about.md#Edge Cases": 2627fe727b7cbda5
  "scenarios/an-assessment-sees-the-rule-it-asks-about.md#Open Questions": 9fc5979d590cdb97
  "scenarios/an-assessment-sees-the-rule-it-asks-about.md#Resolved Questions": 2f0609454bdda985
  "scenarios/rule-judging-runs-from-repo-scripts.md": 5473ad62d52d0e72
  "scenarios/rule-judging-runs-from-repo-scripts.md#Behavior": 7b686a88fa321a33
  "scenarios/rule-judging-runs-from-repo-scripts.md#Context": 7ed89d7d49155d20
  "scenarios/rule-judging-runs-from-repo-scripts.md#Edge Cases": da72e88ddc69c437
  "scenarios/rule-judging-runs-from-repo-scripts.md#Open Questions": 9fc5979d590cdb97
  "scenarios/rule-judging-runs-from-repo-scripts.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/rule-judging-runs-from-repo-scripts.md#Rule-judging-runs-from-repo-scripts": 613171fd65103777
  "spec.md": 95f3d24dca52a1a3
  "spec.md#060 — Exec analyze assesses each loaded rule": 992038454d49574d
  "spec.md#Acceptance Criteria": 183a46f12a8c8957
  "spec.md#Cost": 4c250b5741c89460
  "spec.md#Edge Cases": f8230bc0b5d3a84d
  "spec.md#Motivation": 37aef5db83e15c09
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 97a92600804204ea
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "scenarios/an-assessment-sees-the-loaded-rule-files.md#Edge Cases"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#One request per rule of the step's tier"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
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
  QUAL-CLAIM-001: {verdict: addressed, cite: "plan.md#The unexamined vocabulary: one new reason, one redefined"}
  QUAL-DELEG-001: {verdict: addressed, cite: "plan.md#One rule-section parser, sharing check-rule-ids' heading grammar"}
  QUAL-GROUND-001: {verdict: addressed, cite: "plan.md#This repository's host runs its rule judging from `scripts/analyze/`"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#The analyze-basic fixture's rule gains a keyword"}
rules-judged: 306
rules-carried: 0
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

- grounding — §Staging in the system temp directory states as fact that the CLI reads a `--*-json` file only from the system temp directory, citing no source file or symbol and carrying no assumption marker (§Grounding). — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — **fixed** (decided by the rule)
- grounding — Motivation states that this repository's 11 rule files held 192 rule sections with a Verification paragraph, dated 2026-09-26 but naming no command or query, so the count is not re-derivable (§Grounding). — `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` — **fixed** (decided by the rule)
- grounding — §Staging in the system temp directory states that a primitive the scripts call writes its own gitignored results under `.ductus/results/` as it does for any caller, citing no primitive description or source (§Grounding). — `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — **fixed** (decided by the rule)
- grounding — §Cost states 'The 643 bytes of that reference section and the 266 of the constitution's paragraph on scenarios' with no method; the section's stated measures (git ls-tree, wc -c of rendered briefs) do not cover them (§Grounding). — `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` — **fixed** (decided by the rule)
- grounding — §Resolved Questions (one request per rule or per step) states 'median tracked spec.md 17.5 KB over 58 specs' with a commit but no command or query (§Grounding). — `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` — **fixed** (decided by the rule)
- grounding — §Resolved Questions (tier and trigger answers) enumerate 'three data models' that would carry a structured field, naming none, so the enumeration cannot be re-derived (§Grounding). — `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md` — **fixed** (decided by the rule)
