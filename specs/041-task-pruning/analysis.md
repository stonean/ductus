---
spec: 041-task-pruning
last-run: 2026-11-16T02:36:14Z
analyzed-against: 8b207c9430c9d0f1b865a2f139f32a65a48f4de1
hard-fail: 0
blocking-findings: 0
advisory: 10
unexamined: 18
analyzed-digest:
  data-model.md: a0070f57393f898ae607087b2de92769bee86dc9a3071d022f29b77485ce7e32
  plan.md: fe23b8a2b9cbb891e590432c10e1dd0efa95a54e821ff0d3644b4acfeb73c2f7
  review.md: 8169919c77ca990e91d5ca2b02b88235f62419c728b3d5caa63c9209d324210f
  scenarios/a-plan-edit-does-not-reopen-a-done-spec.md: d212af7304352b7080514a22d93fa67f76e350b7989670730c16045c50dc2051
  scenarios/a-pruned-task-number-is-never-reissued.md: 7d97f1b790d2b12d3b1630cc78f24eac2aff167c9ab932b436964f0f50205d06
  scenarios/a-suffixed-task-number-is-a-task.md: 362cf9e7efe74c12c39d20cd82d2347148695310611c9895b5d7694dad611f17
  scenarios/plan-records-the-design-as-it-stands.md: b3bfd54321552fcc9e32f91dfa5b24a21a923685937f661a38e78b8ed67d5658
  scenarios/prune-all-walks-every-spec.md: 0bcf1f2b448c089721deab6e96dba269082c23a092c0b8aaca56ee90244aefab
  scenarios/prune-reduces-the-spec-directory.md: c485de78348784233c118c59c649bc38d6b33ba4b6d9a05e0a8daa9c656f57c0
  scenarios/prune-walks-the-project-files.md: 8cb1a20f84195213bb7ddac0789d75a3a557a3872fdc480cf868115d06125a3e
  spec.md: 300e7f9920931e4a84ed6d9a24a107b7deeaa19c12745eec675133a66fd1e7c4
  tasks.md: 6a0b960779585089dd03d85bec3fafab6387500318d5eefa1de9728e6b8b725d
unexamined-by-reason:
  never-existed: 1
  root-absent: 13
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
  BE-ERR-002: {verdict: addressed, cite: "data-model.md#Operational errors"}
  BE-ERR-004: {verdict: addressed, cite: "data-model.md#Operational errors"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Error taxonomy"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Error taxonomy"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Error taxonomy"}
  BE-INPUT-008: {verdict: addressed, cite: "plan.md#Error taxonomy"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#An apply refuses a file that changed since it read it"}
  BE-LOCK-002: {verdict: addressed, cite: "data-model.md#The write guard"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#Walking every spec"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#An apply refuses a file that changed since it read it"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#An apply refuses a file that changed since it read it"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "data-model.md#The write guard"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#An apply refuses a file that changed since it read it"}
  BE-TXN-004: {verdict: addressed, cite: "plan.md#An apply refuses a file that changed since it read it"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#One finding key serves the advisory and prune"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#The design record is the plan template's `##` headings"}
  CFG-CONST-003: {verdict: addressed, cite: "data-model.md#The write guard"}
  CFG-ENV-005: {verdict: addressed, cite: "data-model.md#Operational errors"}
  CFG-ENV-008: {verdict: addressed, cite: "data-model.md#Operational errors"}
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 10
  undispositioned: 0
decisions:
  - key: grounding — plan.md gives 842 sections and 161,841 bytes over 60 specs with no command
    outcome: waived
    reason: a measurement the sentence dates to when the walk was built, kept as the reason the walk lists lines; re-deriving it changes no decision
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says every back-edge, amend and plan/implement cycle appends tasks
    outcome: waived
    reason: motivation describing how task lists grow, not a claim the design rests on
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says ductus has no command to reclaim that space
    outcome: waived
    reason: motivation stating the gap this spec filled; /prune now exists by this spec
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says a reset target's commented example headings would parse as phantom tasks without comment-awareness
    outcome: waived
    reason: the reason for an alignment this spec delivered, held by AC13 and its tests
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md says an adopter tree holds only its own copy of the tasks template
    outcome: waived
    reason: the bounded divergence AC11 records; prune.md's markdown-only reference states it
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: "grounding — plan.md counts 7 fenced ## lines across 010, 026 and 027 and a code-font comment in 058, with no command"
    outcome: waived
    reason: a measurement this run re-derived (2026-10-01) naming the files it counts
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says the --all batch shape is the one /analyze --all already has
    outcome: waived
    reason: a comparison to a command in this repository, named by its invocation
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says a filled tasks.md has already stripped the guidance comment
    outcome: waived
    reason: rationale for a rejected alternative in Trade-offs
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md describes 022's data model as the registry of check families, at 105 scenarios and 1,529 lines
    outcome: waived
    reason: the rationale of a decision, dated by this run to when it was made
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says every other tasks primitive trusts checkbox state
    outcome: waived
    reason: a known limitation stated in Trade-offs, not a claim the design rests on
    decided-at: 2026-10-01T18:58:23Z
    decided-by: andy@stone.dev
---

# Analysis — 041-task-pruning

## Summary

0 hard-fail, 0 blocking, 10 advisory; not blocking. 18 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 10 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — plan.md gives 842 sections and 161,841 bytes over 60 specs with no command — `specs/041-task-pruning/plan.md` — **waived**: a measurement the sentence dates to when the walk was built, kept as the reason the walk lists lines; re-deriving it changes no decision
- grounding — spec.md says every back-edge, amend and plan/implement cycle appends tasks — `specs/041-task-pruning/spec.md` — **waived**: motivation describing how task lists grow, not a claim the design rests on
- grounding — spec.md says ductus has no command to reclaim that space — `specs/041-task-pruning/spec.md` — **waived**: motivation stating the gap this spec filled; /prune now exists by this spec
- grounding — spec.md says a reset target's commented example headings would parse as phantom tasks without comment-awareness — `specs/041-task-pruning/spec.md` — **waived**: the reason for an alignment this spec delivered, held by AC13 and its tests
- grounding — spec.md says an adopter tree holds only its own copy of the tasks template — `specs/041-task-pruning/spec.md` — **waived**: the bounded divergence AC11 records; prune.md's markdown-only reference states it
- grounding — plan.md counts 7 fenced ## lines across 010, 026 and 027 and a code-font comment in 058, with no command — `specs/041-task-pruning/plan.md` — **waived**: a measurement this run re-derived (2026-10-01) naming the files it counts
- grounding — plan.md says the --all batch shape is the one /analyze --all already has — `specs/041-task-pruning/plan.md` — **waived**: a comparison to a command in this repository, named by its invocation
- grounding — plan.md says a filled tasks.md has already stripped the guidance comment — `specs/041-task-pruning/plan.md` — **waived**: rationale for a rejected alternative in Trade-offs
- grounding — plan.md describes 022's data model as the registry of check families, at 105 scenarios and 1,529 lines — `specs/041-task-pruning/plan.md` — **waived**: the rationale of a decision, dated by this run to when it was made
- grounding — plan.md says every other tasks primitive trusts checkbox state — `specs/041-task-pruning/plan.md` — **waived**: a known limitation stated in Trade-offs, not a claim the design rests on

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- root-absent: 13
- ships-to-adopter: 4

## Fixed in this run

- decision-reasons — plan.md Trade-off 'The advisory rides prune-plan, not a check-artifacts family' rejects the family option chiefly because it would have reopened 022 for a full re-review of 105 scenarios and a 1,529-line data model — `specs/041-task-pruning/plan.md` — **fixed** (decided by the rule)
