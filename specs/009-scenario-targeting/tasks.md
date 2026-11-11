---
title: "009-scenario-targeting — tasks"
---

# 009 — Scenario Targeting Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Update scenario template

- [x] Add `## Open Questions` and `## Resolved Questions` sections to `framework/templates/spec/scenario.md`

Done when: `framework/templates/spec/scenario.md` includes Open Questions and Resolved Questions sections after Edge Cases.

## 2. Update target command

- [x] Add no-argument mode: display current target (feature + scenario if set), inform user how to change focus
- [x] Add `{feature}/{scenario-slug}` parsing
- [x] Add scenario validation: feature exists → `scenarios/` directory exists → slug file exists, with distinct error messages for each failure
- [x] Add scenario fields (`scenario`, `scenario-path`) to session file write
- [x] Add scenario detail to target display (scenario name, section, context summary)
- [x] Update `framework/commands/target.md`
- [x] Re-derive `.claude/commands/ductus/target.md`

Done when: target command handles no-args display, feature-only targeting (clears scenario), and feature/scenario targeting with validation and error messages.

## 3. Update scenario command

- [x] After creating the scenario file, write session file with `scenario` and `scenario-path` fields (no confirmation prompt)
- [x] Update `framework/commands/amend.md`
- [x] Re-derive `.claude/commands/ductus/amend.md`

Done when: creating a scenario automatically sets it as the session target.

## 4. Update clarify command

- [x] Add scenario-targeted behavior: if session has scenario, resolve open questions in the scenario file instead of spec
- [x] Enumerate scenario-specific edge cases and verify behavior section
- [x] When no scenario targeted, existing behavior unchanged — do not surface scenario-level questions
- [x] Update `framework/commands/clarify.md`
- [x] Re-derive `.claude/commands/ductus/clarify.md`

Done when: clarify operates on the scenario file when a scenario is targeted, and on the spec when not.

## 5. Update status command

- [x] Add scenario-level display when a scenario is targeted: open questions, section, context summary
- [x] Update `framework/commands/status.md`
- [x] Re-derive `.claude/commands/ductus/status.md`

Done when: status shows scenario detail when a scenario is targeted.

## 6. Update implement command

- [x] Add scenario context loading: when a scenario is targeted, include the scenario file as primary context for implementation
- [x] Update `framework/commands/implement.md`
- [x] Re-derive `.claude/commands/ductus/implement.md`

Done when: implement scopes context to the targeted scenario when one is set.

## 7. Verify question command

- [x] Verify `framework/commands/amend.md` already handles scenario targeting via Target File Detection section
- [x] If any gaps exist, update the command
- [x] Verify `.claude/commands/ductus/amend.md` matches

Done when: question command correctly appends to scenario Open Questions when a scenario is targeted and spec Open Questions when not.

## 8. Verify feature-only commands

- [x] Verify specify, plan, and analyze commands ignore the scenario field
- [x] No changes expected — confirm and document

Done when: feature-only commands operate at the feature level regardless of scenario targeting.

## 9. Final lint and verification

- [x] Run `npx markdownlint-cli2` on all new and modified files
- [x] Verify all acceptance criteria from the spec are addressed by the tasks above

Done when: all files pass lint and every acceptance criterion maps to a completed task.

## 10. Restore target's change-focus hint

- [x] `framework/commands/target.md` step 1's no-argument branch tells the user they can run `/{project}:target {feature}` or `/{project}:target {feature}/{scenario-slug}` to change focus, whether or not a target is set (AC14). 022's task 13 (`886b34b0`) dropped the hint when it flattened the command's steps; the owner chose to restore it (2026-11-11)
- [x] Regenerate the dogfooded copies with `scripts/gen-claude-commands.sh` and confirm `--check` reports no drift

- **Done when**: a no-argument `/ductus:target` names both forms of the command after its display, and `scripts/gen-claude-commands.sh --check` reports no drift
