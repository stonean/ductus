---
spec: 023-govern-refinement
last-run: 2026-11-18T12:49:36Z
analyzed-against: 9cae484cf4b41ad6e9d3de3eb4d6fe675cc1e551
hard-fail: 0
blocking-findings: 0
advisory: 5
unexamined: 35
analyzed-digest:
  plan.md: 575b36a531cdbdb4e4a00d373a2a7b59dc6a20cf88a320ba4c6d6fcae3a92784
  review.md: cfbe5d046e077c070a9ae160c124c7a605cd43690815f0d1b623f75c5d6f8d93
  scenarios/configure-dedup-permissions.md: bf3690f13886de1650ad52685c636082dbb34e89028f6f81a674e6206d843a41
  scenarios/configure-inert-write-path-entries.md: b57cb8cfbe2da0696b5c8f82dcb1406602fdd8aadd55a491923d6ccf319c8cbb
  scenarios/configure-permission-pattern-safety.md: 0c1881bbf82925e4971fbcda5394b24219befcd4a8c6185602c94b5717fe8387
  scenarios/configure-retires-formerly-canonical-entries.md: 0edf0a2eeed43837628c59de7c539e2ce1119fd9ab088066baf5b46a8d2eccca
  scenarios/extend-existing-scenario-task.md: 70df3af27fc39ca140538dbe470a13112dc0ca007913b4f07bb2baf9dbb617e6
  scenarios/living-specs.md: 0331b227b72b54edbcb383f8f27c994505d62ab792c6a1d58272c210297d7feb
  spec.md: e6759bad462fc10559605437d49c9b179bbd3741fc6becb9ec75a3dda0ce10b3
  tasks.md: 63c0438b1a50935e4f831bfe5d863db6be0ca48d878c35e137c258ed454f7f5b
unexamined-by-reason:
  anchor-target-unjudgeable: 3
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
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-ERR-002: {verdict: addressed, cite: "spec.md#Acceptance Criteria"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-INPUT-008: {verdict: addressed, cite: "scenarios/configure-dedup-permissions.md#Edge Cases"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-SCHEMA-002: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#Phase sequencing — invariant first, runtime primitives second, command consolidation third"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 5
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
---

# Analysis — 023-govern-refinement

## Summary

0 hard-fail, 0 blocking, 5 advisory; not blocking. 35 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 5 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- body-path-existence — names `framework/templates/project/project-readme.md`, which existed in this repository and no longer resolves: "2. **Reference sweep** — replace `/{project}:validate`, `/ductus:validate`, and `validate.md` with their `analyze` counterparts in every file that named them: `framework/commands/help.md`, `framework/commands/review.md`, `framework/constitution.md`, `framework/commands/analyze.md` (the renamed `validate.md` itself), `framework/bootstrap/ductus.md`, `framework/templates/spec/spec.md`, `framework/templates/project/project-readme.md`, `scripts/gen-help-tables.sh`, `scripts/lint-frontmatter.sh`, `README.md`. The Claude-commands generator picks up the rename on its next run and prunes the old `.claude/commands/ductus/validate.md`." — `specs/023-govern-refinement/plan.md` — **waived**: the plan row records the path as it stood when 023 was delivered; the owner's 2026-11-12 decision in 98be4f19 (retire(003) task 6) keeps 023's plan rows naming the retired template as record, never gating (decided by the rule)
- body-path-existence — names `framework/templates/project/project-readme.md`, which existed in this repository and no longer resolves: "| `framework/templates/project/project-readme.md` | Modify | Drop `spec-and-plan` from the templates list. |" — `specs/023-govern-refinement/plan.md` — **waived**: the plan row records the path as it stood when 023 was delivered; the owner's 2026-11-12 decision in 98be4f19 (retire(003) task 6) keeps 023's plan rows naming the retired template as record, never gating (decided by the rule)
- grounding — plan.md Help-tables section asserts the generator's existing splice logic handles marker renames without code changes, uncited — `specs/023-govern-refinement/plan.md` — **waived**: a design-time claim about a script this spec's task 13 then changed and ran in sync; grounding it means reading the script, which this command does not do, so it is no chore (decided by the rule)
- grounding — spec.md AC12/AC34 state as fact that Claude Code matches file permissions against Edit rules only and warns on Write(path) at session start, with no primary-source citation — `specs/023-govern-refinement/spec.md` — **waived**: a claim about Claude Code's permission matching, an external host's behavior the spec relies on; grounding it needs the host's documentation fetched and read, so the fix is not mechanical, and the advisory holds nothing (decided by the rule)
- grounding — spec.md Section 6 asserts an allow entry for an unregistered MCP tool is a no-op, uncited — `specs/023-govern-refinement/spec.md` — **waived**: a claim about Claude Code's permission matching, an external host's behavior the spec relies on; grounding it needs the host's documentation fetched and read, so the fix is not mechanical, and the advisory holds nothing (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 3
- never-existed: 1
- not-a-live-claim: 23
- root-absent: 8

## Fixed in this run

*None.*
