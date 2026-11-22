---
spec: 020-code-review
last-run: 2026-11-22T02:09:17Z
analyzed-against: 776c4bb26f19f84c1778ea64df63d0a57001e858
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 4
analyzed-digest:
  data-model.md: 1625833bc1e1572da569f7c582ededc4ea9b59efe55b3f6d459efb59becd9632
  plan.md: 9f318f3a76c929443074eef18fece4b30e1cca8097353d7aa2e9335321fbfe18
  review.md: 13c0b8761f9cee382ae575355bcf90146aeb4f50aae24a462cf99f8149e769c5
  scenarios/every-loaded-rule-file-has-a-pass.md: 1f684e5b4c4c3d6aa764a4744f27fda185952ca95ae7743de1190e042337b78b
  scenarios/review-flag-parsing-is-specified.md: a19fe0f4f50c954363f7cca4d913a81d96374535d31a986792243be5263631c1
  scenarios/review-reports-a-defect-under-its-owner.md: ef91f01e38d96520853db62f5ee62b9b8893d41582f055f4f01a25d5a796b665
  scenarios/waiver-expiry.md: 0c35cbf8e26a8a6f321c8549b7209c61b37122713bf13270e5b3c972b788f4ed
  scenarios/waiver-file-lists.md: 8b4180d64e4e43ac6a66f57df7c65d6f1f00bb343ae284bb2d7214c88c824f53
  spec.md: cc3e711087f7567cce88202dfdf9654b48e63f023b0a2347bb2d3d47bc26b681
  tasks.md: cf5205c36a760bbc9900ee69422b5887ee8a632d51d95030c35328d7ff63d7ea
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
  security-backend.md: 511a68b25ceb148f8292cda2a829aaff03a38bc3453732e33a70bd5d6acf5df5
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 5fddb21530faa805cb00f23c929e9402548ed0d6a0ed3e8e9a910561b74a45a3
section-digest:
  "data-model.md": 6d0edc78b49e423e
  "data-model.md#020 — `/ductus:review` Data Model": 05ddac7278ca1598
  "data-model.md#Body sections (in order)": 5517d95176fda68c
  "data-model.md#Cross-references": 7d5d4ed52c0e9e9d
  "data-model.md#Expiry rule": d754490fa32f76d0
  "data-model.md#Finding record": 047e7c34fed5d43c
  "data-model.md#Frontmatter": 71a9109b021b8406
  "data-model.md#Idempotency invariant": 5dfd2b1f55db3ec3
  "data-model.md#Waiver record": 6c1b780a72ead6a4
  "data-model.md#`.ductus/config.toml [review]` section": 8382d121440a2075
  "data-model.md#`review.md` artifact": 0b42aeda787bd798
  "plan.md": 6d0edc78b49e423e
  "plan.md#020 — `/ductus:review` Plan": 55d3c5e776a78643
  "plan.md#A finding needs a trigger the project can reach (AC18)": d3f828da6c5dda55
  "plan.md#A review leaves out a rule file its header gives to analysis (AC17)": b1eef63f5e35e766
  "plan.md#Affected Files": 5cd86de5c24a6273
  "plan.md#CI template stays minimal": 047fa7d247858711
  "plan.md#Considered and rejected": 602b7ae4ca3f08fa
  "plan.md#Frontmatter `review:` block ships in templates": 39698467844ca23b
  "plan.md#Idempotency invariant is a property of inputs, not state": ca0561d6b7de0ac1
  "plan.md#Known limitations": 847819a1af5ca5c6
  "plan.md#Markdown-instructed implementation, no new code": b66dc73d890315b3
  "plan.md#Overview": 2f93b8a850c39f4e
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
  "scenarios/waiver-expiry.md#Behavior": 261506da7212bd6f
  "scenarios/waiver-expiry.md#Context": 282c2356f7c2a15c
  "scenarios/waiver-expiry.md#Edge Cases": 9b4d16ecc437ae37
  "scenarios/waiver-expiry.md#Open Questions": 833b08cc1a85801f
  "scenarios/waiver-expiry.md#Resolved Questions": 6d07bd0230afd3d3
  "scenarios/waiver-expiry.md#Waiver expiry": 9b70b9059f43e7fe
  "scenarios/waiver-file-lists.md": 9aab3f2b95e73eb4
  "scenarios/waiver-file-lists.md#Behavior": 8f99db2eb1cc0321
  "scenarios/waiver-file-lists.md#Context": b59b6c514e97f6f5
  "scenarios/waiver-file-lists.md#Edge Cases": 40606c76e6bfa72d
  "scenarios/waiver-file-lists.md#Open Questions": 9fc5979d590cdb97
  "scenarios/waiver-file-lists.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/waiver-file-lists.md#Waiver-file-lists": aaf1e1299b50c29a
  "spec.md": c909e6b4f6a2e014
  "spec.md#020 — `/ductus:review` code review command with blocking gate": a867b9f5873b9bcb
  "spec.md#Acceptance Criteria": 111de66eb0f304be
  "spec.md#Affected files": 3da3125affec7ca8
  "spec.md#Blocking message": 18c9bc717d4ba4a3
  "spec.md#Embedded artifacts": 38fed4e29a5a94d2
  "spec.md#Frontmatter schema": bb8fd06a63e8c877
  "spec.md#Motivation": 9dbe9adea9d2bfce
  "spec.md#Non-goals": e5be029c645275f6
  "spec.md#Open questions": 4a8a652831ceeeb8
  "spec.md#Resolved questions": 5d31321e859c0947
  "spec.md#Summary": b16d9e57f8341894
  "spec.md#Tasks": 079030de60900b7b
rule-verdicts:
  BE-INPUT-001: {verdict: addressed, cite: "scenarios/waiver-file-lists.md#Edge Cases"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Waiver record"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#Waiver-expiry warrants a scenario file"}
  CFG-ENV-001: {verdict: addressed, cite: "data-model.md#`.ductus/config.toml [review]` section"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#`.ductus/config.toml` follows the shared-database convention"}
  CFG-ENV-008: {verdict: addressed, cite: "data-model.md#`.ductus/config.toml [review]` section"}
  CFG-ENV-012: {verdict: addressed, cite: "data-model.md#`.ductus/config.toml [review]` section"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "spec.md#Acceptance Criteria"}
  QUAL-GROUND-001: {verdict: addressed, cite: "plan.md#Rule files map to passes in one compiled table, and the reference states it in prose a test holds to it"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Rule files map to passes in one compiled table, and the reference states it in prose a test holds to it"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: "grounding — Acceptance Criteria, AC15: \"Nothing reads `examined` now; it is read at the completion gate by a person\" is a descriptive claim about current system behavior (an absence claim) with no citation to a primary source and no assumption marker."
    outcome: waived
    reason: "A criterion's claims are judged against the tree by every review's criteria step, which records the evidence beside the verdict (020's review of 2026-11-19 judged AC15 met: no runtime code reads the review record's `examined`), so a citation inside the criterion would be a second, unchecked copy of that evidence (owner decision, 2026-11-19)."
    decided-at: 2026-11-19T02:44:45Z
    decided-by: andy@stone.dev
  - key: grounding — Decision 'A finding needs a trigger the project can reach (AC18)' states that an adopter's review on 0.60.0 spent about four hours of fix rounds on unreachable cases, citing only 'the adopter's report', which is no reachable artifact, query or command.
    outcome: waived
    reason: "advisory and no chore: the figure comes from an adopter's report, recorded where the decision it motivated was made; this repository names no adopter and cannot open its tree (AGENTS.md), so no reachable artifact can be cited, and no behavior rests on it"
    decided-at: 2026-11-22T02:09:17Z
    decided-by: rule
---

# Analysis — 020-code-review

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 4 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — AC15 states as fact that 'Nothing reads `examined` now; it is read at the completion gate by a person' with no citation to a command, primitive or check that substantiates it. — `specs/020-code-review/spec.md` — **waived**: A criterion's claims are judged against the tree by every review's criteria step, which records the evidence beside the verdict (020's review of 2026-11-19 judged AC15 met: no runtime code reads the review record's `examined`), so a citation inside the criterion would be a second, unchecked copy of that evidence (owner decision, 2026-11-19).
- grounding — Decision 'A finding needs a trigger the project can reach (AC18)' states that an adopter's review on 0.60.0 spent about four hours of fix rounds on unreachable cases, citing only 'the adopter's report', which is no reachable artifact, query or command. — `specs/020-code-review/plan.md` — **waived**: advisory and no chore: the figure comes from an adopter's report, recorded where the decision it motivated was made; this repository names no adopter and cannot open its tree (AGENTS.md), so no reachable artifact can be cited, and no behavior rests on it (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- not-a-live-claim: 2
- root-absent: 1

## Fixed in this run

*None.*
