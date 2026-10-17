---
title: "009-scenario-targeting — plan"
---

# 009 — Scenario Targeting Plan

## Overview

Extend the pipeline so individual scenarios can be targeted alongside features. This involves modifying the session file format, updating the target command to support scenario syntax and no-argument display, adding an Open Questions section to the scenario template, and updating the scenario-aware commands (`amend`'s question route, `clarify`, `status`, `implement`) plus `amend`'s scenario route, which creates scenarios. All artifacts are markdown command files and a template — no application code, no persistence.

## Technical Decisions

### Session file gains optional `scenario` and `scenario-path` fields

The session file, `.ductus/session.toml`, is extended with two optional fields. When present, scenario-aware commands use `scenario-path` to locate the primary artifact. When absent, behavior is unchanged. The `scenario` field holds the slug; `scenario-path` holds the full relative path for direct file access without path construction.

### Target command supports three invocation modes

- No arguments: display current target (feature + scenario if set) and inform user how to change focus
- Feature only: existing behavior, clears any scenario field
- Feature/scenario-slug: sets both feature and scenario, validates the scenario file exists

Validation order for `{feature}/{scenario-slug}`: check feature exists → check `scenarios/` directory exists → check slug matches a file. Each failure has a distinct error message per the spec.

### Scenario template extended with Open Questions section

The `framework/templates/spec/scenario.md` file gets `## Open Questions` and `## Resolved Questions` sections appended — the same pattern specs use. Existing scenario files are not retroactively modified — they gain the sections when questions are added via the question command.

### Scenario-aware commands check session for scenario field

Each scenario-aware command (`amend`'s question route, `clarify`, `status`, `implement`) reads the session file and checks for the `scenario`/`scenario-path` fields. If present, the command operates on the scenario file instead of the spec. The branching is at the "target file detection" step — the rest of each command's logic operates on whichever file was selected.

### Clarify operates on scenario Open Questions when scenario-targeted

When a scenario is targeted, clarify resolves questions in the scenario file and enumerates scenario-specific edge cases. It does not touch the spec. When no scenario is targeted, existing behavior is unchanged and scenario-level questions are not surfaced.

### `amend` sets the target after creating a scenario, without confirmation

After creating a scenario file, `amend`'s scenario route writes the session target with the new scenario. No confirmation prompt — the user explicitly asked to create it.

### Feature-only commands ignore the scenario field

Specify, plan, and analyze read the session file for the feature but disregard the scenario field entirely. No changes needed to these commands beyond documenting the behavior (which the spec already covers).

### Command file parity maintained by the generator

Every command change is made in `framework/commands/` (templates with `{project}` and `{cli-config-dir}` placeholders). `scripts/gen-claude-commands.sh` regenerates the dogfooded copies in `.claude/commands/ductus/` (with `ductus` and `.claude` substituted), and the pre-commit hook runs it. The bootstrap's manifest already includes these command files, so adopting projects get the updates on their next `/ductus` run.

### The bootstrap needs no structural change

The scenario template's Open Questions section is already in the bootstrap's manifest (`framework/templates/spec/scenario.md` → `specs/templates/scenario.md`). No new files need to be added to the manifest, and the bootstrap itself needs no structural change — only the command files it installs are updated.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/templates/spec/scenario.md` | Modify | Add `## Open Questions` section |
| `framework/commands/target.md` | Modify | Add no-argument display, scenario targeting syntax, validation, error messages |
| `framework/commands/amend.md` | Modify | Set session target after scenario creation (no confirmation) |
| `framework/commands/amend.md` | Verify | Already scenario-aware — verify target file detection covers scenario path |
| `framework/commands/clarify.md` | Modify | Add scenario-targeted behavior (resolve scenario open questions, skip spec questions) |
| `framework/commands/status.md` | Modify | Add scenario-level detail display when scenario is targeted |
| `framework/commands/implement.md` | Modify | Add scenario context loading when scenario is targeted |
| `.claude/commands/ductus/target.md` | Modify | Re-derive from updated `framework/commands/target.md` |
| `.claude/commands/ductus/amend.md` | Modify | Re-derive from updated `framework/commands/amend.md` |
| `.claude/commands/ductus/amend.md` | Verify | Re-derive from updated `framework/commands/amend.md` if changed |
| `.claude/commands/ductus/clarify.md` | Modify | Re-derive from updated `framework/commands/clarify.md` |
| `.claude/commands/ductus/status.md` | Modify | Re-derive from updated `framework/commands/status.md` |
| `.claude/commands/ductus/implement.md` | Modify | Re-derive from updated `framework/commands/implement.md` |

## Trade-offs

### Considered: storing scenario status in the session file

Rejected. Scenarios do not have their own status field (per 006-bug-workflow). Resolution is determined by whether open questions remain. Adding status to the session would create a parallel tracking mechanism that conflicts with the established convention.

### Considered: making all commands scenario-aware

Rejected. Specify, plan, and analyze are inherently feature-level operations. Making them scenario-aware would add complexity without benefit — there is nothing to specify, plan, or analyze at the scenario level that isn't already covered by the feature-level operation.

### Considered: requiring target confirmation when scenario is set

Rejected per user decision during clarification. Over-prompting for confirmations is annoying. `amend` creates the scenario the user asked for and targets it — the next step is obvious.

### Considered: surfacing scenario questions during feature-level clarify

Rejected per user decision during clarification. Spec-level and scenario-level questions are independent concerns. Mixing them would blur scope boundaries and add noise.

## Open Questions Resolved

All open questions were resolved during clarification. See spec.md — Resolved Questions section.
