---
spec: 087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file
last-run: 2026-11-26T21:42:42Z
analyzed-against: a66c818a1a6a315810acc79da08c04ad51c71d9b
hard-fail: 0
blocking-findings: 0
advisory: 5
unexamined: 13
analyzed-digest:
  plan.md: 695553ed1e311e6d68116824a3363d6be3e71d6a0e6bc7242d42debf618bf7dc
  review.md: dddd63a252e8f4807b357c9086330c217817e9d4e4ee5174544d5d185973ae67
  spec.md: 794a0beae333287d7bfe46c472a5d0beffc1dbb2587159c973049c5ecd98a9c8
  tasks.md: 5b1eda3772672480e39ae737ff20e32e777662cfa209746b146a454c75dd8b17
unexamined-by-reason:
  never-existed: 9
  not-a-live-claim: 2
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
  "plan.md#087 — Parts moved out of a project file are read, measured and pruned as that file Plan": ff33e0a405a17b2a
  "plan.md#A shared constitution's parts (AC16, AC17)": 1ead629d03be9fbf
  "plan.md#Affected Files": efab8b73322f7c20
  "plan.md#Cross-spec impact": a088ef9ae4bac068
  "plan.md#Data Model": 4b77820597781599
  "plan.md#Every registry layout (AC18)": 6a6d32e203bd6f79
  "plan.md#No regression without parts (AC8)": 5d2d537d34945283
  "plan.md#One definition of the part set": 6482e6320970b27f
  "plan.md#Overview": dc31e5b81f878cd3
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Tests fail without their behavior (AC10)": 7c3d0269325cab34
  "plan.md#Text surfaces (AC9, AC13, AC19)": 55ed724db6e58d43
  "plan.md#The migration entry announces the move (AC15)": c8840b3468dd5ef0
  "plan.md#This repository's procedure file moves": 6c71c7964a67852d
  "plan.md#Trade-offs": 64454c6dffaef6fe
  "plan.md#`/analyze` records the integrity findings (AC6, AC12, AC20)": f75f919a48aed14a
  "plan.md#`/implement` and `/review` read by pointer (AC3)": f9e8489b2cdd1f71
  "plan.md#`/plan` and `/analyze` read every part before judging the plan (AC1, AC2)": 5cf7cd1b9563aa22
  "plan.md#`check-artifact-size` supplies the list (AC1, AC2, AC4–AC6, AC11, AC12, AC20)": 7ac55bff8ef60a81
  "plan.md#`prune-project` walks parts and moves a pointer's target (AC7, AC13, AC14)": 865af1d8b0329a0b
  "spec.md": 4cd89b5665061ba0
  "spec.md#087 — Parts moved out of a project file are read, measured and pruned as that file": 6c47fc9043c2ba10
  "spec.md#Acceptance Criteria": 3a63e8bf62d62618
  "spec.md#Applicable Rules": f02f5bcd6ecacf08
  "spec.md#Cost": d736f49e5ee9f4f8
  "spec.md#Edge Cases": 59ccca11d90dc733
  "spec.md#Motivation": f9c484596b075215
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Parts": 159c8cd1224469a4
  "spec.md#Resolved Questions": 09e4171e2119621b
  "spec.md#The parts are the file": 3fafa47ed76188e9
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#No regression without parts (AC8)"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#`prune-project` walks parts and moves a pointer's target (AC7, AC13, AC14)"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#`prune-project` walks parts and moves a pointer's target (AC7, AC13, AC14)"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#`prune-project` walks parts and moves a pointer's target (AC7, AC13, AC14)"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#`prune-project` walks parts and moves a pointer's target (AC7, AC13, AC14)"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#`check-artifact-size` supplies the list (AC1, AC2, AC4–AC6, AC11, AC12, AC20)"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#`prune-project` walks parts and moves a pointer's target (AC7, AC13, AC14)"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#`prune-project` walks parts and moves a pointer's target (AC7, AC13, AC14)"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#`prune-project` walks parts and moves a pointer's target (AC7, AC13, AC14)"}
  BE-TXN-004: {verdict: addressed, cite: "plan.md#`prune-project` walks parts and moves a pointer's target (AC7, AC13, AC14)"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#One definition of the part set"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "plan.md#One definition of the part set"}
  QUAL-DELEG-001: {verdict: addressed, cite: "plan.md#One definition of the part set"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Tests fail without their behavior (AC10)"}
rules-judged: 306
rules-carried: 0
blocking: false
dispositions:
  fixed: 18
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "criterion-path-existence — acceptance criterion names `.ductus/docs/system/`, which no longer resolves: \"AC11: A markdown file directly under each of `.ductus/docs/agents/`, `.ductus/docs/system/`, `.ductus/docs/events/` and `.ductus/docs/errors/` is measured as a part of its project file whether or not the index links it, and a file there that is not markdown is neither measured nor reported, except one that is a symbolic link leaving the project, which is reported as not looked at.\""
    outcome: waived
    reason: They name the directories any project's parts live in, not claims about this tree, and backticked full paths stay where a rename sweep greps for them (the operator's decision for 087's analysis, 2026-11-26).
    decided-at: 2026-11-26T21:42:42Z
    decided-by: rule
  - key: "criterion-path-existence — acceptance criterion names `.ductus/docs/events/`, which no longer resolves: \"AC11: A markdown file directly under each of `.ductus/docs/agents/`, `.ductus/docs/system/`, `.ductus/docs/events/` and `.ductus/docs/errors/` is measured as a part of its project file whether or not the index links it, and a file there that is not markdown is neither measured nor reported, except one that is a symbolic link leaving the project, which is reported as not looked at.\""
    outcome: waived
    reason: They name the directories any project's parts live in, not claims about this tree, and backticked full paths stay where a rename sweep greps for them (the operator's decision for 087's analysis, 2026-11-26).
    decided-at: 2026-11-26T21:42:42Z
    decided-by: rule
  - key: "criterion-path-existence — acceptance criterion names `.ductus/docs/errors/`, which no longer resolves: \"AC11: A markdown file directly under each of `.ductus/docs/agents/`, `.ductus/docs/system/`, `.ductus/docs/events/` and `.ductus/docs/errors/` is measured as a part of its project file whether or not the index links it, and a file there that is not markdown is neither measured nor reported, except one that is a symbolic link leaving the project, which is reported as not looked at.\""
    outcome: waived
    reason: They name the directories any project's parts live in, not claims about this tree, and backticked full paths stay where a rename sweep greps for them (the operator's decision for 087's analysis, 2026-11-26).
    decided-at: 2026-11-26T21:42:42Z
    decided-by: rule
  - key: "criterion-path-existence — acceptance criterion names `.ductus/docs/constitution/`, which no longer resolves: \"AC16: `/target`'s report of a loaded shared constitution gives each part under its checkout's `.ductus/docs/constitution/` with its bytes, names a fixture part one byte over the limit with its owning repository as the fix, reports the parts of one not checked out as not measured, and the run continues.\""
    outcome: waived
    reason: They name the directories any project's parts live in, not claims about this tree, and backticked full paths stay where a rename sweep greps for them (the operator's decision for 087's analysis, 2026-11-26).
    decided-at: 2026-11-26T21:42:42Z
    decided-by: rule
  - key: "body-path-existence — names `.ductus/scripts/`, which existed in this repository and no longer resolves: \"- **Where do parts live?** Under `.ductus/docs/{file}/`, one directory per project file: `.ductus/docs/system/`, `.ductus/docs/agents/` (the owner's proposal, recommended by the agent and chosen by the owner, 2026-11-24). It costs no setting, cannot collide with a project's own `docs/`, and is inside 081's artifact set, so a part edit owes artifact checks and no review. `/ductus` leaves it alone: of the 1,069 lines of `framework/bootstrap/ductus.md`, those that name both `.ductus` and a removal cover only the top-level `*.md` files of `.ductus/reference/` and the runtime pointer, and the shipped `.gitignore` template ignores `session.toml`, `sessions/`, `bin/` and `results/` under `.ductus/` and nothing else. Of the 13 files under `framework/migrations/` that mention `.ductus/`, the lines naming both it and a removal cover `.ductus/scripts/`, the legacy session file and the emptied `.govern/`, never `.ductus/docs/`; the files were searched by line, not read whole. What this gives up: a hidden directory is less visible to someone browsing the repository. Rejected: a `[paths]` key, which costs a setting and leaves a part under `docs/` as code unless 081's set reads the key, the configurable list 081 rejected; and a directory under the spec root, which leaves `AGENTS.md`, at the project root, without a home beside it.\""
    outcome: waived
    reason: The sentence names `.ductus/scripts/` as a legacy directory the migrations remove from an adopter's tree, which is its point; it is no claim that the path exists in this repository.
    decided-at: 2026-11-26T21:42:42Z
    decided-by: rule
---

# Analysis — 087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file

## Summary

0 hard-fail, 0 blocking, 5 advisory; not blocking. 13 unexamined target(s). Dispositions: 18 fixed, 0 deferred, 5 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- criterion-path-existence — acceptance criterion names `.ductus/docs/system/`, which no longer resolves: "AC11: A markdown file directly under each of `.ductus/docs/agents/`, `.ductus/docs/system/`, `.ductus/docs/events/` and `.ductus/docs/errors/` is measured as a part of its project file whether or not the index links it, and a file there that is not markdown is neither measured nor reported, except one that is a symbolic link leaving the project, which is reported as not looked at." — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md` — **waived**: They name the directories any project's parts live in, not claims about this tree, and backticked full paths stay where a rename sweep greps for them (the operator's decision for 087's analysis, 2026-11-26). (decided by the rule)
- criterion-path-existence — acceptance criterion names `.ductus/docs/events/`, which no longer resolves: "AC11: A markdown file directly under each of `.ductus/docs/agents/`, `.ductus/docs/system/`, `.ductus/docs/events/` and `.ductus/docs/errors/` is measured as a part of its project file whether or not the index links it, and a file there that is not markdown is neither measured nor reported, except one that is a symbolic link leaving the project, which is reported as not looked at." — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md` — **waived**: They name the directories any project's parts live in, not claims about this tree, and backticked full paths stay where a rename sweep greps for them (the operator's decision for 087's analysis, 2026-11-26). (decided by the rule)
- criterion-path-existence — acceptance criterion names `.ductus/docs/errors/`, which no longer resolves: "AC11: A markdown file directly under each of `.ductus/docs/agents/`, `.ductus/docs/system/`, `.ductus/docs/events/` and `.ductus/docs/errors/` is measured as a part of its project file whether or not the index links it, and a file there that is not markdown is neither measured nor reported, except one that is a symbolic link leaving the project, which is reported as not looked at." — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md` — **waived**: They name the directories any project's parts live in, not claims about this tree, and backticked full paths stay where a rename sweep greps for them (the operator's decision for 087's analysis, 2026-11-26). (decided by the rule)
- criterion-path-existence — acceptance criterion names `.ductus/docs/constitution/`, which no longer resolves: "AC16: `/target`'s report of a loaded shared constitution gives each part under its checkout's `.ductus/docs/constitution/` with its bytes, names a fixture part one byte over the limit with its owning repository as the fix, reports the parts of one not checked out as not measured, and the run continues." — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md` — **waived**: They name the directories any project's parts live in, not claims about this tree, and backticked full paths stay where a rename sweep greps for them (the operator's decision for 087's analysis, 2026-11-26). (decided by the rule)
- body-path-existence — names `.ductus/scripts/`, which existed in this repository and no longer resolves: "- **Where do parts live?** Under `.ductus/docs/{file}/`, one directory per project file: `.ductus/docs/system/`, `.ductus/docs/agents/` (the owner's proposal, recommended by the agent and chosen by the owner, 2026-11-24). It costs no setting, cannot collide with a project's own `docs/`, and is inside 081's artifact set, so a part edit owes artifact checks and no review. `/ductus` leaves it alone: of the 1,069 lines of `framework/bootstrap/ductus.md`, those that name both `.ductus` and a removal cover only the top-level `*.md` files of `.ductus/reference/` and the runtime pointer, and the shipped `.gitignore` template ignores `session.toml`, `sessions/`, `bin/` and `results/` under `.ductus/` and nothing else. Of the 13 files under `framework/migrations/` that mention `.ductus/`, the lines naming both it and a removal cover `.ductus/scripts/`, the legacy session file and the emptied `.govern/`, never `.ductus/docs/`; the files were searched by line, not read whole. What this gives up: a hidden directory is less visible to someone browsing the repository. Rejected: a `[paths]` key, which costs a setting and leaves a part under `docs/` as code unless 081's set reads the key, the configurable list 081 rejected; and a directory under the spec root, which leaves `AGENTS.md`, at the project root, without a home beside it." — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md` — **waived**: The sentence names `.ductus/scripts/` as a legacy directory the migrations remove from an adopter's tree, which is its point; it is no claim that the path exists in this repository. (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 9
- not-a-live-claim: 2
- ships-to-adopter: 2

## Fixed in this run

- grounding — Cost section: 'Descriptions, loaded when a Claude Code session fetches the tool and by Pi's bridge at load' states how two hosts load descriptions as fact, with no citation or assumption marker. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md` — **fixed** (decided by the rule)
- grounding — Resolved Questions, 'Which files may have parts?': 'bars sub-files for data-model.md, whose name the review's contract digest tracks' states how 063 and the review digest behave, citing neither a file-and-symbol nor a query. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md` — **fixed** (decided by the rule)
- grounding — Every registry layout: 'pi has no permission surface to seed' is asserted as fact about the pi layout with no citation or assumption marker. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed** (decided by the rule)
- grounding — The migration entry, Seed: 'as it seeds label-criteria and relocate-audit-records' states what the claude row's settings_template currently seeds, with no file-and-symbol citation. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed** (decided by the rule)
- decision-reasons — Resolved Questions, 'Does the staged size check gain a per-file waiver?': the option 'one as a scenario on 063' is rejected on the stated reason 'which reopens it' alone, a process cost with no result named. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md` — **fixed** (decided by the rule)
- grounding — Resolved Questions, 'Does the staged size check gain a per-file waiver?': '075 chose refusing growth and rejected an exit that depends on someone choosing to take it, and 063 declined an off switch as permanent configuration surface' states what two specs decided, citing neither a section nor a link. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md` — **fixed** (decided by the rule)
- grounding — Resolved Questions, 'Which files may have parts?': 'The framework constitution has no parts: it installs whole, with .ductus/reference/ as its on-demand content' states how the constitution installs, with no citation. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md` — **fixed** (decided by the rule)
- grounding — `/analyze` subsection: 'Step 15 already carries audit:ignore-promotion' states the current state of step 15 with no citation. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed** (decided by the rule)
- grounding — `/plan` subsection: 'the exec walker no-ops' host work cites only the heading 'Exec-path scope', with no file or symbol. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed** (decided by the rule)
- grounding — A shared constitution's parts, Unmeasured callers: 'The unmeasured function, which write-review, write-analysis and /ductus use, reads no part' states its callers with no file-and-symbol citation. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed** (decided by the rule)
- grounding — This repository's procedure file moves, AGENTS.md size: 'It measured 49,427 bytes, a figure that includes another session's uncommitted edits' records a measurement with no method or command. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed** (decided by the rule)
- grounding — Every registry layout: '.claude/commands/ductus/*.md for claude-style and .pi/prompts/ductus-*.md for pi, which the hook regenerates' states where each layout's generated copies land and who regenerates them, with no citation. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed** (decided by the rule)
- grounding — Trade-offs, 'Rejected: nesting parts inside each project-files record': 'the staged CLI reads that list inline' states how the staged CLI reads the list, with no citation. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed** (decided by the rule)
- grounding — No regression without parts (AC8), BE-COMPAT-001 paragraph: 'within major version zero' states the project's version with no citation to where the version is recorded. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed** (decided by the rule)
- grounding — No regression without parts (AC8), BE-COMPAT-001 paragraph: 'response enums not documented as open' states how the existing enums are documented, citing no file or data model. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed** (decided by the rule)
- grounding — No regression without parts (AC8), BE-COMPAT-001 paragraph: 'No field is removed, renamed or retyped' is a claim about the whole result surface of three primitives, with no test, diff or command cited. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-INPUT-004: The move's from (a link target taken from committed text) and to are screened by shape only (inside the project, a new *.md directly under a parts directory); no canonicalization and containment is committed for reading, rewriting and removing from or creating to, so a symlinked path could escape the project. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed**
- rule-assessment — BE-COMPAT-001: the plan classifies none of the contract changes 087 makes, two of them new values in response enums not documented as open (check-artifact-size's oversized[].kind and its fixes' kind), which the rule's Verification requires of a plan that modifies an existing contract. — `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md` — **fixed**
