---
spec: 023-govern-refinement
last-run: 2026-11-14T02:04:28Z
analyzed-against: 040eb2d84c339ee0a19525344877fc53744b2b64
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 32
analyzed-digest:
  plan.md: f837a221917b54fa71bd383b93ffe59141304dc4c14b36ac5f32d0179a6bb124
  review.md: cfbe5d046e077c070a9ae160c124c7a605cd43690815f0d1b623f75c5d6f8d93
  scenarios/configure-dedup-permissions.md: bf3690f13886de1650ad52685c636082dbb34e89028f6f81a674e6206d843a41
  scenarios/configure-inert-write-path-entries.md: b57cb8cfbe2da0696b5c8f82dcb1406602fdd8aadd55a491923d6ccf319c8cbb
  scenarios/configure-permission-pattern-safety.md: d394e4f64f6fe145459d5568850479a2f1f3ba1a48e0bf5546844870163aa79f
  scenarios/configure-retires-formerly-canonical-entries.md: 0edf0a2eeed43837628c59de7c539e2ce1119fd9ab088066baf5b46a8d2eccca
  scenarios/extend-existing-scenario-task.md: 1a0b674ce341509402b4a598e791bbed510e2f758be5e646786683340927155a
  scenarios/living-specs.md: 6410062310a49c5f895b61a518bc482372ca10a790dcefb92f84cfb4ddc97c9e
  spec.md: e6759bad462fc10559605437d49c9b179bbd3741fc6becb9ec75a3dda0ce10b3
  tasks.md: 54e3bb0b88eb76d1c36b68d03b36a94a6ea4f6ed1dd6619a03ae51ae34c15990
unexamined-by-reason:
  never-existed: 1
  not-a-live-claim: 23
  root-absent: 8
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
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: grounding — plan.md states that scripts/gen-help-tables.sh's existing splice logic handles marker renames without code changes, citing no line of the script
    outcome: waived
    reason: a design-time claim about a script this spec's task 13 then changed and ran in sync; grounding it means reading the script, which this command does not do, so it is no chore
    decided-at: 2026-11-07T23:45:39Z
    decided-by: rule
  - key: grounding — spec.md states that GitHub Spec Kit assigns /analyze the artifact-consistency role and that its name has become the emerging-standard term, citing no source
    outcome: waived
    reason: a claim about an outside project's naming, given as the rename's motivation; grounding it means fetching that project, which this command does not do, and the rename stands on its own
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
---

# Analysis — 023-govern-refinement

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 32 unexamined target(s). Dispositions: 3 fixed, 0 deferred, 4 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — plan.md states that scripts/gen-help-tables.sh's existing splice logic handles marker renames without code changes, citing no line of the script — `specs/023-govern-refinement/plan.md` — **waived**: a design-time claim about a script this spec's task 13 then changed and ran in sync; grounding it means reading the script, which this command does not do, so it is no chore
- grounding — spec.md states that GitHub Spec Kit assigns /analyze the artifact-consistency role and that its name has become the emerging-standard term, citing no source — `specs/023-govern-refinement/spec.md` — **waived**: a claim about an outside project's naming, given as the rename's motivation; grounding it means fetching that project, which this command does not do, and the rename stands on its own
- body-path-existence — names `framework/templates/project/project-readme.md`, which existed in this repository and no longer resolves: "2. **Reference sweep** — replace `/{project}:validate`, `/ductus:validate`, and `validate.md` with their `analyze` counterparts in every file that named them: `framework/commands/help.md`, `framework/commands/review.md`, `framework/constitution.md`, `framework/commands/analyze.md` (the renamed `validate.md` itself), `framework/bootstrap/ductus.md`, `framework/templates/spec/spec.md`, `framework/templates/project/project-readme.md`, `scripts/gen-help-tables.sh`, `scripts/lint-frontmatter.sh`, `README.md`. The Claude-commands generator picks up the rename on its next run and prunes the old `.claude/commands/ductus/validate.md`." — `specs/023-govern-refinement/plan.md` — **waived**: the plan row records the path as it stood when 023 was delivered; the owner's 2026-11-12 decision in 98be4f19 (retire(003) task 6) keeps 023's plan rows naming the retired template as record, never gating (decided by the rule)
- body-path-existence — names `framework/templates/project/project-readme.md`, which existed in this repository and no longer resolves: "| `framework/templates/project/project-readme.md` | Modify | Drop `spec-and-plan` from the templates list (line 26). |" — `specs/023-govern-refinement/plan.md` — **waived**: the plan row records the path as it stood when 023 was delivered; the owner's 2026-11-12 decision in 98be4f19 (retire(003) task 6) keeps 023's plan rows naming the retired template as record, never gating (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- not-a-live-claim: 23
- root-absent: 8

## Fixed in this run

- plan-consistency — Two Technical Decisions state what is done and carry no rationale: 'Lightweight track sweep strategy' and 'Constitution slash-command sweep'. — `specs/023-govern-refinement/plan.md` — **fixed**
- plan-consistency — Technical Decision 'Migration check in /ductus bootstrap' describes a prose-only step in framework/bootstrap/ductus.md as the design; spec section 1 and AC11 record that 027 replaced it with the spec-and-plan-sunset migration entry, inert since 0.10.0. — `specs/023-govern-refinement/plan.md` — **fixed** (decided by the rule)
- task-consistency — tasks.md intro still describes Phase A (primitives, MCP generator) and Phase B ordering, but the file holds only Phase C task 24. — `specs/023-govern-refinement/tasks.md` — **fixed** (decided by the rule)
