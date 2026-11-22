---
spec: 023-govern-refinement
last-run: 2026-11-22T04:26:44Z
analyzed-against: 1d03e0d75daa903429978deb0c33b4f13e85dc18
hard-fail: 0
blocking-findings: 0
advisory: 10
unexamined: 38
analyzed-digest:
  plan.md: 8f123a293033c7cba8a5b4fd09941c8bb03a6a9f0b3ff9530a72e6a94200d2b1
  review.md: 989907cba92d54c9384eed4be53ce4845e1ca270b941bde2f43f8e3242b1d1fc
  scenarios/configure-dedup-permissions.md: bf3690f13886de1650ad52685c636082dbb34e89028f6f81a674e6206d843a41
  scenarios/configure-inert-write-path-entries.md: 6548c68140f0b3363c38b9ae228b264aa85041aa0fe1537437b735bf8320af65
  scenarios/configure-permission-pattern-safety.md: 23794a3651ae508412681c6754aa282157e15322e972ca3a881a6879d7678738
  scenarios/configure-retires-formerly-canonical-entries.md: 0edf0a2eeed43837628c59de7c539e2ce1119fd9ab088066baf5b46a8d2eccca
  scenarios/extend-existing-scenario-task.md: 70df3af27fc39ca140538dbe470a13112dc0ca007913b4f07bb2baf9dbb617e6
  scenarios/living-specs.md: 0331b227b72b54edbcb383f8f27c994505d62ab792c6a1d58272c210297d7feb
  spec.md: 353b35e6d3963c84de1bf3f41a1d34b47589ca46d41275782441277b6f84be89
  tasks.md: 0bab5ed85e0faab20b50fae3a7f1516b75e51a643de548c76bdb4bd2db0450d3
unexamined-by-reason:
  anchor-target-unjudgeable: 5
  never-existed: 1
  not-a-live-claim: 24
  root-absent: 8
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
  "plan.md": e3b0c44298fc1c14
  "plan.md#023 — `ductus` Refinement Plan": ec8eb5471866c481
  "plan.md#Affected Files": c797605027e753da
  "plan.md#Considered and rejected": 3c3ec71fae86886a
  "plan.md#Constitution slash-command sweep": ca947fc8e2ceb47d
  "plan.md#Description tightening for `/analyze` and `/review`": 946fc7eaff34d6d3
  "plan.md#Help-tables generator update": 8f193ce730538f58
  "plan.md#Known limitations": 793451894b46d237
  "plan.md#Lightweight track sweep strategy": e42408fee8657d4d
  "plan.md#MCP allow-list generator — `scripts/gen-configure-mcp.sh`": a517078863824c33
  "plan.md#Migration check in `/ductus` bootstrap": fa143c975bfbd7ce
  "plan.md#Overview": aa200be8d33a2133
  "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third": 7dad1121907d367c
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Trade-offs": 6d48e49804e54ced
  "plan.md#`/amend` rewrite — classifier + scenario branch": 411ddec58b4baf2a
  "plan.md#`/capture` deletion strategy": b5efbc18d5c7bb95
  "plan.md#`/validate` → `/analyze` rename": fa1ee59d86938e4a
  "scenarios/configure-dedup-permissions.md": d3822901deaad9bc
  "scenarios/configure-dedup-permissions.md#Behavior": 0a40166e6fd0c551
  "scenarios/configure-dedup-permissions.md#Configure-dedup-permissions": 744f4366955b048f
  "scenarios/configure-dedup-permissions.md#Context": 3aacf9f702122bd6
  "scenarios/configure-dedup-permissions.md#Edge Cases": 24361ad213c5da1b
  "scenarios/configure-dedup-permissions.md#Open Questions": 9fc5979d590cdb97
  "scenarios/configure-dedup-permissions.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/configure-inert-write-path-entries.md": d3822901deaad9bc
  "scenarios/configure-inert-write-path-entries.md#Behavior": 0b51d9ffda142bd3
  "scenarios/configure-inert-write-path-entries.md#Configure-inert-write-path-entries": d586fccd16cd8db8
  "scenarios/configure-inert-write-path-entries.md#Context": 2544f3c15dcb8517
  "scenarios/configure-inert-write-path-entries.md#Edge Cases": 9ac11e145bce00d2
  "scenarios/configure-inert-write-path-entries.md#Open Questions": 9fc5979d590cdb97
  "scenarios/configure-inert-write-path-entries.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/configure-permission-pattern-safety.md": d3822901deaad9bc
  "scenarios/configure-permission-pattern-safety.md#Behavior": 87fef29e90f84b4c
  "scenarios/configure-permission-pattern-safety.md#Configure-permission-pattern-safety": 9bd68758b41ee377
  "scenarios/configure-permission-pattern-safety.md#Context": fca079f151d35cc1
  "scenarios/configure-permission-pattern-safety.md#Edge Cases": 085f5efbbc56bcbf
  "scenarios/configure-permission-pattern-safety.md#Open Questions": 9fc5979d590cdb97
  "scenarios/configure-permission-pattern-safety.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/configure-retires-formerly-canonical-entries.md": d3822901deaad9bc
  "scenarios/configure-retires-formerly-canonical-entries.md#Behavior": fcf625408d898fa9
  "scenarios/configure-retires-formerly-canonical-entries.md#Configure-retires-formerly-canonical-entries": cbfd08ae54af1acd
  "scenarios/configure-retires-formerly-canonical-entries.md#Context": 305f3d31c1bf8175
  "scenarios/configure-retires-formerly-canonical-entries.md#Edge Cases": 0f27899be0707d9e
  "scenarios/configure-retires-formerly-canonical-entries.md#Open Questions": 9fc5979d590cdb97
  "scenarios/configure-retires-formerly-canonical-entries.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/extend-existing-scenario-task.md": a0f44d9b66843b66
  "scenarios/extend-existing-scenario-task.md#Behavior": 0a20e4dec1c4fdee
  "scenarios/extend-existing-scenario-task.md#Context": 7b90ca097bf6f50d
  "scenarios/extend-existing-scenario-task.md#Edge Cases": c8a5c42c0eefcf10
  "scenarios/extend-existing-scenario-task.md#Extend-existing-scenario-task": 273409643164897c
  "scenarios/extend-existing-scenario-task.md#Open Questions": 20edda8881516e4e
  "scenarios/extend-existing-scenario-task.md#Resolved Questions": 282a2fe696fd61e2
  "scenarios/living-specs.md": f4036d4aa3763d17
  "scenarios/living-specs.md#Behavior": e41f1127612b1397
  "scenarios/living-specs.md#Context": 3e85949ddbd4bd21
  "scenarios/living-specs.md#Edge Cases": c3112ce8cb5fced3
  "scenarios/living-specs.md#Living-specs": 54c46e4cc2407099
  "scenarios/living-specs.md#Open Questions": 9fc5979d590cdb97
  "scenarios/living-specs.md#Resolved Questions": bfd9665ca114e31b
  "spec.md": a4367157bcd74091
  "spec.md#023 — `ductus` Refinement": d70bb4929db6b68a
  "spec.md#1. Lightweight track removed": 5da377d47d935b21
  "spec.md#2. `/capture` consolidated into `/specify`": 34572638b9db4def
  "spec.md#3. `/elaborate` consolidated into `/amend`": de3a8b85fd165328
  "spec.md#4. `/validate` renamed to `/analyze`": b398d794ee579044
  "spec.md#5. `/configure` allow entries for ductus-owned state files": 03da7cdfccb6ec54
  "spec.md#6. `/configure` allow statements for runtime MCP tools": 43aa6bc926739c8a
  "spec.md#Acceptance Criteria": 51d90b7288724e2d
  "spec.md#Motivation": 83871b63fe5bc1ce
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 04f7b686822cc9eb
  "spec.md#Scope of change": 4b29331134337f59
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-SCHEMA-002: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
blocking: false
dispositions:
  fixed: 8
  deferred: 0
  waived: 10
  undispositioned: 0
decisions:
  - key: grounding — plan.md states that scripts/gen-help-tables.sh's existing splice logic handles marker renames without code changes, citing no line of the script
    outcome: waived
    reason: a design-time claim about a script this spec's task 13 then changed and ran in sync; grounding it means reading the script, which this command does not do, so it is no chore
    decided-at: 2026-11-07T23:45:39Z
    decided-by: rule
  - key: "body-path-existence — names `framework/templates/project/project-readme.md`, which existed in this repository and no longer resolves: \"2. **Reference sweep** — replace `/{project}:validate`, `/ductus:validate`, and `validate.md` with their `analyze` counterparts in every file that named them: `framework/commands/help.md`, `framework/commands/review.md`, `framework/constitution.md`, `framework/commands/analyze.md` (the renamed `validate.md` itself), `framework/bootstrap/ductus.md`, `framework/templates/spec/spec.md`, `framework/templates/project/project-readme.md`, `scripts/gen-help-tables.sh`, `scripts/lint-frontmatter.sh`, `README.md`. The Claude-commands generator picks up the rename on its next run and prunes the old `.claude/commands/ductus/validate.md`.\""
    outcome: waived
    reason: the plan row records the path as it stood when 023 was delivered; the owner's 2026-11-12 decision in 98be4f19 (retire(003) task 6) keeps 023's plan rows naming the retired template as record, never gating
    decided-at: 2026-11-14T02:04:28Z
    decided-by: rule
  - key: "body-path-existence — names `framework/templates/project/project-readme.md`, which existed in this repository and no longer resolves: \"| `framework/templates/project/project-readme.md` | Modify | Drop `spec-and-plan` from the templates list (line 26). |\""
    outcome: waived
    reason: the plan row records the path as it stood when 023 was delivered; the owner's 2026-11-12 decision in 98be4f19 (retire(003) task 6) keeps 023's plan rows naming the retired template as record, never gating
    decided-at: 2026-11-14T02:04:28Z
    decided-by: rule
  - key: grounding — spec.md AC12/AC34 state as fact that Claude Code matches file permissions against Edit rules only and warns on Write(path) at session start, with no primary-source citation
    outcome: waived
    reason: a claim about Claude Code's permission matching, an external host's behavior the spec relies on; grounding it needs the host's documentation fetched and read, so the fix is not mechanical, and the advisory holds nothing
    decided-at: 2026-11-18T12:49:36Z
    decided-by: rule
  - key: grounding — spec.md Section 6 asserts an allow entry for an unregistered MCP tool is a no-op, uncited
    outcome: waived
    reason: a claim about Claude Code's permission matching, an external host's behavior the spec relies on; grounding it needs the host's documentation fetched and read, so the fix is not mechanical, and the advisory holds nothing
    decided-at: 2026-11-18T12:49:36Z
    decided-by: rule
  - key: grounding — Help-tables generator section states 'scripts/gen-help-tables.sh currently builds five tables' with no cited symbol or command.
    outcome: waived
    reason: a design-time count of the script this spec's task 13 then changed and ran in sync; grounding it means reading the script, which this command does not do, so it is no chore
    decided-at: 2026-11-22T04:26:44Z
    decided-by: rule
  - key: grounding — Resolved Questions (Impact on spec 022) states 'substitute-templates is shaped for bulk template->destination tree copy' and 'mark-task ... does not append' as facts about existing primitives, citing no primitive description or source.
    outcome: waived
    reason: claims about 022's primitives as they stood when the question was decided; grounding them means reading runtime source, which this command does not do, so it is no chore
    decided-at: 2026-11-22T04:26:44Z
    decided-by: rule
  - key: grounding — Section 5 states adopters are 'repeatedly prompted to confirm writes' to .ductus/session.toml and that the explicit entry 'disambiguate[s] from any agent-host heuristic that otherwise prompts', an external-host behavior claim with no cited source.
    outcome: waived
    reason: the observation that motivated section 5 comes from adopters' sessions, which this repository cannot open, and a host's prompting is documented outside it; no edit here can ground it, so it is no chore
    decided-at: 2026-11-22T04:26:44Z
    decided-by: rule
  - key: grounding — Motivation states 'ductus exposed 14 user-visible slash commands across 5 categories' with no command or source cited, though it is framed as when the spec was written.
    outcome: waived
    reason: a count of the command set as it stood when 023 was written, which 023 itself then changed; grounding it means counting at that commit, so it is no chore
    decided-at: 2026-11-22T04:26:44Z
    decided-by: rule
  - key: grounding — Scope of change section 4 asserts Spec Kit's name has become the emerging-standard term for this gate, citing no source (§Grounding).
    outcome: waived
    reason: a claim about another project's naming; grounding it needs GitHub Spec Kit's documentation fetched and read, so the fix is not mechanical, and the advisory holds nothing
    decided-at: 2026-11-22T04:26:44Z
    decided-by: rule
---

# Analysis — 023-govern-refinement

## Summary

0 hard-fail, 0 blocking, 10 advisory; not blocking. 38 unexamined target(s). Dispositions: 8 fixed, 0 deferred, 10 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Help-tables generator section states 'The script's existing splice logic handles marker renames without code changes' as fact about scripts/gen-help-tables.sh with no symbol or command cited and no assumption marker. — `specs/023-govern-refinement/plan.md` — **waived**: a design-time claim about a script this spec's task 13 then changed and ran in sync; grounding it means reading the script, which this command does not do, so it is no chore (decided by the rule)
- grounding — Help-tables generator section states 'scripts/gen-help-tables.sh currently builds five tables' with no cited symbol or command. — `specs/023-govern-refinement/plan.md` — **waived**: a design-time count of the script this spec's task 13 then changed and ran in sync; grounding it means reading the script, which this command does not do, so it is no chore (decided by the rule)
- grounding — Resolved Questions (Impact on spec 022) states 'substitute-templates is shaped for bulk template->destination tree copy' and 'mark-task ... does not append' as facts about existing primitives, citing no primitive description or source. — `specs/023-govern-refinement/spec.md` — **waived**: claims about 022's primitives as they stood when the question was decided; grounding them means reading runtime source, which this command does not do, so it is no chore (decided by the rule)
- grounding — Section 5 states adopters are 'repeatedly prompted to confirm writes' to .ductus/session.toml and that the explicit entry 'disambiguate[s] from any agent-host heuristic that otherwise prompts', an external-host behavior claim with no cited source. — `specs/023-govern-refinement/spec.md` — **waived**: the observation that motivated section 5 comes from adopters' sessions, which this repository cannot open, and a host's prompting is documented outside it; no edit here can ground it, so it is no chore (decided by the rule)
- grounding — AC12 states 'Claude Code matches file permissions against Edit rules only' and 'the host warns about it at session start', a claim about an external system with no cited source (AC34 and its scenario are named only for the removal). — `specs/023-govern-refinement/spec.md` — **waived**: a claim about Claude Code's permission matching, an external host's behavior the spec relies on; grounding it needs the host's documentation fetched and read, so the fix is not mechanical, and the advisory holds nothing (decided by the rule)
- grounding — Motivation states 'ductus exposed 14 user-visible slash commands across 5 categories' with no command or source cited, though it is framed as when the spec was written. — `specs/023-govern-refinement/spec.md` — **waived**: a count of the command set as it stood when 023 was written, which 023 itself then changed; grounding it means counting at that commit, so it is no chore (decided by the rule)
- grounding — Scope of change section 4 asserts Spec Kit's name has become the emerging-standard term for this gate, citing no source (§Grounding). — `specs/023-govern-refinement/spec.md` — **waived**: a claim about another project's naming; grounding it needs GitHub Spec Kit's documentation fetched and read, so the fix is not mechanical, and the advisory holds nothing (decided by the rule)
- grounding — Section 6 asserts an allow entry for an unregistered MCP tool is a no-op, uncited. — `specs/023-govern-refinement/spec.md` — **waived**: a claim about Claude Code's permission matching, an external host's behavior the spec relies on; grounding it needs the host's documentation fetched and read, so the fix is not mechanical, and the advisory holds nothing (decided by the rule)
- body-path-existence — names `framework/templates/project/project-readme.md`, which existed in this repository and no longer resolves: "2. **Reference sweep** — replace `/{project}:validate`, `/ductus:validate`, and `validate.md` with their `analyze` counterparts in every file that named them: `framework/commands/help.md`, `framework/commands/review.md`, `framework/constitution.md`, `framework/commands/analyze.md` (the renamed `validate.md` itself), `framework/bootstrap/ductus.md`, `framework/templates/spec/spec.md`, `framework/templates/project/project-readme.md`, `scripts/gen-help-tables.sh`, `scripts/lint-frontmatter.sh`, `README.md`. The Claude-commands generator picks up the rename on its next run and prunes the old `.claude/commands/ductus/validate.md`." — `specs/023-govern-refinement/plan.md` — **waived**: the plan row records the path as it stood when 023 was delivered; the owner's 2026-11-12 decision in 98be4f19 (retire(003) task 6) keeps 023's plan rows naming the retired template as record, never gating (decided by the rule)
- body-path-existence — names `framework/templates/project/project-readme.md`, which existed in this repository and no longer resolves: "| `framework/templates/project/project-readme.md` | Modify | Drop `spec-and-plan` from the templates list. |" — `specs/023-govern-refinement/plan.md` — **waived**: the plan row records the path as it stood when 023 was delivered; the owner's 2026-11-12 decision in 98be4f19 (retire(003) task 6) keeps 023's plan rows naming the retired template as record, never gating (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 5
- never-existed: 1
- not-a-live-claim: 24
- root-absent: 8

## Fixed in this run

- plan-integrity — Technical Decision 'Help-tables generator update' lists changes but states no rationale. — `specs/023-govern-refinement/plan.md` — **fixed** (decided by the rule)
- plan-integrity — Affected Files row for specs/README.md still says to add a 'Past Renames' note, while AC23 and AC21 state that section was deleted by the living-specs scenario and rename history lives in git log. — `specs/023-govern-refinement/plan.md` — **fixed** (decided by the rule)
- plan-integrity — Technical Decision '/validate -> /analyze rename' states it is a pure rename with no behavior change but carries no reason for the rename itself (the reason lives only in spec.md section 4). — `specs/023-govern-refinement/plan.md` — **fixed** (decided by the rule)
- line-number-citations — Constitution slash-command sweep's table cited framework/constitution.md line numbers in a Line column, which §grounding forbids in a plan; the column is dropped and the Section column locates each row. — `specs/023-govern-refinement/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-TXN-002: append-task is a read-modify-write of tasks.md that loses a task under two concurrent calls and the plan names no lost-update guard; it records the guard only as deferred to spec 022 task 35 (a deferral for step 19, not a guard). — `specs/023-govern-refinement/plan.md` — **fixed** (decided by the rule)
- grounding — Help-tables generator section states 'the table becomes a single-row table with `/amend` only' for the refine table, with no citation, while spec.md Resolved Questions (Help category collapse) records Refine now carries four rows from scripts/gen-help-tables.sh; the plan's present-tense claim is uncited and stale. — `specs/023-govern-refinement/plan.md` — **fixed** (decided by the rule)
- grounding — Migration check decision still says 027 replaced the check with the spec-and-plan-sunset entry in framework/migrations.toml, which 027's review has since archived (c061c1f0). — `specs/023-govern-refinement/plan.md` — **fixed** (decided by the rule)
- grounding — Constitution slash-command sweep asserts the constitution carries eight references to deleted verbs, at listed lines, with no command or search cited (§Grounding). — `specs/023-govern-refinement/plan.md` — **fixed** (decided by the rule)
