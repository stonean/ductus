---
spec: 041-task-pruning
last-run: 2026-11-20T00:49:10Z
analyzed-against: e33cc634f862f60287ee26f7d574db5a45643acd
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 19
analyzed-digest:
  data-model.md: 8da08307a7f78ceeea3992437bbb46b31f0a399ee4150449387a1bbb4da60b77
  plan.md: 8b9b1b18ed8d9cda16c07571609c726c2f0222e5ddeb2b7a78efba2a10a989fd
  review.md: b30237d707acba72422520881afe6c6747490a31259bb28c153dda15f442f13b
  scenarios/a-plan-edit-does-not-reopen-a-done-spec.md: d212af7304352b7080514a22d93fa67f76e350b7989670730c16045c50dc2051
  scenarios/a-pruned-task-number-is-never-reissued.md: 7d97f1b790d2b12d3b1630cc78f24eac2aff167c9ab932b436964f0f50205d06
  scenarios/a-suffixed-task-number-is-a-task.md: 362cf9e7efe74c12c39d20cd82d2347148695310611c9895b5d7694dad611f17
  scenarios/an-open-fenced-block-or-comment-is-not-examined.md: 4f9252380ba21bb4bd80e3f0ca308a75cc3fe001b7f688ecae7d1a84f08a00c5
  scenarios/plan-records-the-design-as-it-stands.md: b3bfd54321552fcc9e32f91dfa5b24a21a923685937f661a38e78b8ed67d5658
  scenarios/prune-all-walks-every-spec.md: 0bcf1f2b448c089721deab6e96dba269082c23a092c0b8aaca56ee90244aefab
  scenarios/prune-reduces-the-spec-directory.md: c485de78348784233c118c59c649bc38d6b33ba4b6d9a05e0a8daa9c656f57c0
  scenarios/prune-walks-the-project-files.md: 8cb1a20f84195213bb7ddac0789d75a3a557a3872fdc480cf868115d06125a3e
  spec.md: cb7a621e271e2349ba2bc2ad39e066f340028e2ca4082d8c621eebc052c6ee53
  tasks.md: 9f007b9e8ba0fb47390ef2ab2ffb132cd841a6dd4315bbae3b51111223bec621
unexamined-by-reason:
  anchor-target-unjudgeable: 1
  never-existed: 1
  root-absent: 14
  ships-to-adopter: 3
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
  BE-ERR-002: {verdict: addressed, cite: "plan.md#Error taxonomy"}
  BE-ERR-004: {verdict: addressed, cite: "plan.md#Error taxonomy"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Error taxonomy"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Error taxonomy"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#`prune-project` — the four project files"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#Error taxonomy"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#An apply refuses a file that changed since it read it"}
  BE-LOCK-002: {verdict: addressed, cite: "data-model.md#The write guard"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#`--all` repeats a one-spec reduction per spec"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#An apply refuses a file that changed since it read it"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#An apply refuses a file that changed since it read it"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "data-model.md#The write guard"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#An apply refuses a file that changed since it read it"}
  BE-TXN-004: {verdict: addressed, cite: "plan.md#An apply refuses a file that changed since it read it"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#The design record is the plan template's `##` headings"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: grounding — plan.md gives 842 sections and 161,841 bytes over 60 specs with no command
    outcome: waived
    reason: a measurement the sentence dates to when the walk was built, kept as the reason the walk lists lines; re-deriving it changes no decision
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: "grounding — plan.md counts 7 fenced ## lines across 010, 026 and 027 and a code-font comment in 058, with no command"
    outcome: waived
    reason: a measurement this run re-derived (2026-10-01) naming the files it counts
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
---

# Analysis — 041-task-pruning

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 19 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — plan.md gives 842 sections and 161,841 bytes over 60 specs with no command — `specs/041-task-pruning/plan.md` — **waived**: a measurement the sentence dates to when the walk was built, kept as the reason the walk lists lines; re-deriving it changes no decision
- grounding — plan.md counts 7 fenced ## lines across 010, 026 and 027 and a code-font comment in 058, with no command — `specs/041-task-pruning/plan.md` — **waived**: a measurement this run re-derived (2026-10-01) naming the files it counts

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1
- never-existed: 1
- root-absent: 14
- ships-to-adopter: 3

## Fixed in this run

*None.*
