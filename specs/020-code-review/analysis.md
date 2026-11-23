---
spec: 020-code-review
last-run: 2026-11-23T00:05:09Z
analyzed-against: 7028efa8ea7114362dcbb6407468124ca5844952
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 4
analyzed-digest:
  data-model.md: 2b2765b2d9c821ff57a660e6616eeb317e27f271b133e106b9cb2854a618150c
  plan.md: 30a7e6749b1241d3eac88a02131b633e1e95a63a5d81d7e0117dd86a6f491008
  review.md: 7dfa0bc2ef544caf6d39e2d93763180f9fbdf1203bc16a9c257040bf11957763
  scenarios/every-loaded-rule-file-has-a-pass.md: 1f684e5b4c4c3d6aa764a4744f27fda185952ca95ae7743de1190e042337b78b
  scenarios/review-flag-parsing-is-specified.md: a19fe0f4f50c954363f7cca4d913a81d96374535d31a986792243be5263631c1
  scenarios/review-reports-a-defect-under-its-owner.md: ef91f01e38d96520853db62f5ee62b9b8893d41582f055f4f01a25d5a796b665
  scenarios/waiver-expiry.md: 626f92a6e7ee0063e17e4a8135fc21e72a555e6caa0b731401acd29212337b42
  scenarios/waiver-file-lists.md: 3a76e64b910f1a1e0be02c6b603909803a959b70c71b1c50ff5ed12c2420684c
  spec.md: 8995181277d8441c20d988395230a8033463a054b8b1b8d7cc1f0187fe43a457
  tasks.md: 9e63882a08184259eb22671367a909a335c61f76b8398d2953332af8c32ab584
unexamined-by-reason:
  never-existed: 1
  not-a-live-claim: 2
  root-absent: 1
rule-digest:
  accessibility-frontend.md: 4d7f5ac09d457c99774f5fc341222a464d2b461cb0467cd58a13dc31c928359b
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 050a5b8dd375fe4790f5d92753db27ae53ac730db1557cbee84b923b72499ca9
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: f892282383d7377ded2e5fd1ee3e30421d0d6ad3f9bb1fd354ff0d9d114e807a
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 5fddb21530faa805cb00f23c929e9402548ed0d6a0ed3e8e9a910561b74a45a3
section-digest:
  "data-model.md": 6d0edc78b49e423e
  "data-model.md#020 — `/ductus:review` Data Model": 05ddac7278ca1598
  "data-model.md#Body sections (in order)": 5517d95176fda68c
  "data-model.md#Cross-references": 7d5d4ed52c0e9e9d
  "data-model.md#Expiry rule": 2c466b5487bb2ed1
  "data-model.md#Finding record": ba352a2cc63de14d
  "data-model.md#Frontmatter": cf856f9bfbda4cd9
  "data-model.md#Idempotency invariant": 5dfd2b1f55db3ec3
  "data-model.md#Waiver record": 9d7ba2aa028f84ff
  "data-model.md#`.ductus/config.toml [review]` section": 8382d121440a2075
  "data-model.md#`review.md` artifact": 0b42aeda787bd798
  "plan.md": 6d0edc78b49e423e
  "plan.md#020 — `/ductus:review` Plan": 55d3c5e776a78643
  "plan.md#A finding needs a trigger the project can reach (AC18)": d3f828da6c5dda55
  "plan.md#A review leaves out a rule file its header gives to analysis (AC17)": b882235347e5cd37
  "plan.md#A stored disposition covers only the findings at its anchor (AC19)": 386e5584d913182c
  "plan.md#Affected Files": d1126fe2e3015428
  "plan.md#CI template stays minimal": 047fa7d247858711
  "plan.md#Considered and rejected": 602b7ae4ca3f08fa
  "plan.md#Frontmatter `review:` block ships in templates": 39698467844ca23b
  "plan.md#Idempotency invariant is a property of inputs, not state": ca0561d6b7de0ac1
  "plan.md#Known limitations": 847819a1af5ca5c6
  "plan.md#Markdown-instructed passes, runtime-backed steps": e15c58794fbf7a13
  "plan.md#Overview": 9820eaf9072e979c
  "plan.md#Rule files map to passes in one compiled table, and the reference states it in prose a test holds to it": 637c39364c07cb04
  "plan.md#Tech-stack alignment is an agent judgment, not a parser": 2ccdadbcf5c12a7a
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Three-mechanism gate composes": 8f372d2885b2af1e
  "plan.md#Trade-offs": 6d48e49804e54ced
  "plan.md#Waiver-expiry warrants a scenario file": 1deab654b53ccd49
  "plan.md#`.ductus/config.toml` follows the shared-database convention": ad2dba81bdf29dcc
  "scenarios/every-loaded-rule-file-has-a-pass.md": 5473ad62d52d0e72
  "scenarios/every-loaded-rule-file-has-a-pass.md#Behavior": 0018d2e9b84af09d
  "scenarios/every-loaded-rule-file-has-a-pass.md#Context": 13cfff61d35e455b
  "scenarios/every-loaded-rule-file-has-a-pass.md#Edge Cases": be73ae3fd7638fe0
  "scenarios/every-loaded-rule-file-has-a-pass.md#Every-loaded-rule-file-has-a-pass": 0824a9630e665873
  "scenarios/every-loaded-rule-file-has-a-pass.md#Open Questions": 20edda8881516e4e
  "scenarios/every-loaded-rule-file-has-a-pass.md#Resolved Questions": 83e294af39fab43c
  "scenarios/review-flag-parsing-is-specified.md": 82aa59a00be3eada
  "scenarios/review-flag-parsing-is-specified.md#Behavior": 393a93e81915f4ff
  "scenarios/review-flag-parsing-is-specified.md#Context": 60a7a444f5e0e0db
  "scenarios/review-flag-parsing-is-specified.md#Edge Cases": deed301377702b1d
  "scenarios/review-flag-parsing-is-specified.md#Open Questions": 9fc5979d590cdb97
  "scenarios/review-flag-parsing-is-specified.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/review-flag-parsing-is-specified.md#Review-flag-parsing-is-specified": 761cb277269a43b0
  "scenarios/review-reports-a-defect-under-its-owner.md": 5473ad62d52d0e72
  "scenarios/review-reports-a-defect-under-its-owner.md#Behavior": f6c2b9e694785cf8
  "scenarios/review-reports-a-defect-under-its-owner.md#Context": cf6a0ea19dd8c192
  "scenarios/review-reports-a-defect-under-its-owner.md#Edge Cases": 5db4245ab6021b2d
  "scenarios/review-reports-a-defect-under-its-owner.md#Open Questions": 9fc5979d590cdb97
  "scenarios/review-reports-a-defect-under-its-owner.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/review-reports-a-defect-under-its-owner.md#Review-reports-a-defect-under-its-owner": b39ee6493626bbcd
  "scenarios/waiver-expiry.md": 9aab3f2b95e73eb4
  "scenarios/waiver-expiry.md#Behavior": 6e9baac9bcce1522
  "scenarios/waiver-expiry.md#Context": 1de38e95779075d0
  "scenarios/waiver-expiry.md#Edge Cases": 8b9f09198a9003c6
  "scenarios/waiver-expiry.md#Open Questions": 35737a7fd1b7cc55
  "scenarios/waiver-expiry.md#Resolved Questions": be6b892010f3abcb
  "scenarios/waiver-expiry.md#Waiver expiry": 9b70b9059f43e7fe
  "scenarios/waiver-file-lists.md": 9aab3f2b95e73eb4
  "scenarios/waiver-file-lists.md#Behavior": 7c32827e84b18bbe
  "scenarios/waiver-file-lists.md#Context": b59b6c514e97f6f5
  "scenarios/waiver-file-lists.md#Edge Cases": 7a3a5bb29a95dbb1
  "scenarios/waiver-file-lists.md#Open Questions": 9fc5979d590cdb97
  "scenarios/waiver-file-lists.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/waiver-file-lists.md#Waiver-file-lists": aaf1e1299b50c29a
  "spec.md": 49c43f6371a81033
  "spec.md#020 — `/ductus:review` code review command with blocking gate": a867b9f5873b9bcb
  "spec.md#Acceptance Criteria": de9afefde55b9537
  "spec.md#Affected files": 3da3125affec7ca8
  "spec.md#Blocking message": 18c9bc717d4ba4a3
  "spec.md#Embedded artifacts": 38fed4e29a5a94d2
  "spec.md#Frontmatter schema": ab3bd07a0cfab7f0
  "spec.md#Motivation": 9dbe9adea9d2bfce
  "spec.md#Non-goals": e5be029c645275f6
  "spec.md#Open questions": 4a8a652831ceeeb8
  "spec.md#Resolved questions": 5d31321e859c0947
  "spec.md#Summary": b16d9e57f8341894
  "spec.md#Tasks": 079030de60900b7b
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#A stored disposition covers only the findings at its anchor (AC19)"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#A stored disposition covers only the findings at its anchor (AC19)"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#A stored disposition covers only the findings at its anchor (AC19)"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#Waiver-expiry warrants a scenario file"}
  CFG-ENV-001: {verdict: addressed, cite: "data-model.md#`.ductus/config.toml [review]` section"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#`.ductus/config.toml` follows the shared-database convention"}
  CFG-ENV-008: {verdict: addressed, cite: "data-model.md#`.ductus/config.toml [review]` section"}
  CFG-ENV-012: {verdict: addressed, cite: "data-model.md#`.ductus/config.toml [review]` section"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "plan.md#A review leaves out a rule file its header gives to analysis (AC17)"}
  QUAL-DELEG-001: {verdict: addressed, cite: "plan.md#A stored disposition covers only the findings at its anchor (AC19)"}
  QUAL-GROUND-001: {verdict: addressed, cite: "plan.md#A stored disposition covers only the findings at its anchor (AC19)"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Rule files map to passes in one compiled table, and the reference states it in prose a test holds to it"}
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 3
  undispositioned: 0
decisions:
  - key: grounding — Decision 'A finding needs a trigger the project can reach (AC18)' states that an adopter's review on 0.60.0 spent about four hours of fix rounds on unreachable cases, citing only 'the adopter's report', which is no reachable artifact, query or command.
    outcome: waived
    reason: "advisory and no chore: the figure comes from an adopter's report, recorded where the decision it motivated was made; this repository names no adopter and cannot open its tree (AGENTS.md), so no reachable artifact can be cited, and no behavior rests on it"
    decided-at: 2026-11-22T02:09:17Z
    decided-by: rule
  - key: "grounding — Acceptance Criteria AC15: 'Nothing reads `examined` now; it is read at the completion gate by a person' is an uncited absence claim about current system behavior."
    outcome: waived
    reason: "advisory and no chore: the absence holds, since WriteReviewArgs::examined's doc comment (runtime/src/schema/primitives.rs) says no check reads it and no reader of the recorded field exists outside write_review; a citation inside the ticked AC15 is a criterion edit that reopens 020 for no change in what it requires"
    decided-at: 2026-11-23T00:05:09Z
    decided-by: rule
  - key: "decision-reasons — Technical Decisions, AC18: the rejected 'bar on severity alone' is rejected because a recorded NOTE 'still costs a round to dismiss', a fix-round (review-run) cost as the stated reason; borderline, since the false-positive result it would leave is implied but not stated."
    outcome: waived
    reason: "advisory and low confidence: the rejected severity bar's result, an unreachable case that does not block, is delivered by the chosen bar too, so rejecting it gives up no result; the stated reason is what the option would still leave, a recorded NOTE dismissed in a round"
    decided-at: 2026-11-23T00:05:09Z
    decided-by: rule
---

# Analysis — 020-code-review

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 4 unexamined target(s). Dispositions: 2 fixed, 0 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Technical Decisions, AC18 'A finding needs a trigger the project can reach': the adopter's review on 0.60.0 'spend about four hours of fix rounds' is an incident claim attributed to an unlinked 'adopter's report', with no artifact or command substantiating the duration or cases. — `specs/020-code-review/plan.md` — **waived**: advisory and no chore: the figure comes from an adopter's report, recorded where the decision it motivated was made; this repository names no adopter and cannot open its tree (AGENTS.md), so no reachable artifact can be cited, and no behavior rests on it (decided by the rule)
- grounding — Acceptance Criteria AC15: 'Nothing reads `examined` now; it is read at the completion gate by a person' is an uncited absence claim about current system behavior. — `specs/020-code-review/spec.md` — **waived**: advisory and no chore: the absence holds, since WriteReviewArgs::examined's doc comment (runtime/src/schema/primitives.rs) says no check reads it and no reader of the recorded field exists outside write_review; a citation inside the ticked AC15 is a criterion edit that reopens 020 for no change in what it requires (decided by the rule)
- decision-reasons — Technical Decisions, AC18: the rejected 'bar on severity alone' is rejected because a recorded NOTE 'still costs a round to dismiss', a fix-round (review-run) cost as the stated reason; borderline, since the false-positive result it would leave is implied but not stated. — `specs/020-code-review/plan.md` — **waived**: advisory and low confidence: the rejected severity bar's result, an unreachable case that does not block, is delivered by the chosen bar too, so rejecting it gives up no result; the stated reason is what the option would still leave, a recorded NOTE dismissed in a round (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- not-a-live-claim: 2
- root-absent: 1

## Fixed in this run

- grounding — Technical Decisions, 'Markdown-instructed implementation, no new code': 'This matches every other /ductus:* command' is stated as fact about existing commands with no citation or assumption marker. — `specs/020-code-review/plan.md` — **fixed** (decided by the rule)
- grounding — Overview: 'no new code, no new runtime' describes the delivered feature as fact, with no source, and the plan's own later sections (AC17-AC19, Affected Files) name runtime/ source edits. — `specs/020-code-review/plan.md` — **fixed** (decided by the rule)
