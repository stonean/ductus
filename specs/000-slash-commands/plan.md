---
title: "000-slash-commands — plan"
---

# 000 — Slash Command Templates Plan

> **Note:** this plan was written against the original layout. Command sources now live in `framework/commands/`; several command names were later renamed (`about → help`, `setup → configure`, `validate → analyze`, `next` retired). See `spec.md` for the full rename history.

## Overview

Create ten generic slash command `.md` files in a `commands/` directory at the `ductus` root. Each command is derived from the working implementation in a prior adopter project — a Go backend service that had built its own slash commands — but generalized: that project's own references are replaced with `{project}` placeholders, and its language-specific logic (Go code style, module patterns) is removed in favor of references to the constitution and AGENTS.md.

## Technical Decisions

### Directory location

Commands live at `commands/{command}.md` in the `ductus` root. This is the template source — adopting projects copy these to `.claude/commands/{project}/` and replace `{project}` placeholders.

Rationale: Keeping them at the `ductus` root (not under `.claude/commands/`) avoids them being treated as active slash commands in the `ductus` repo itself. `ductus`'s own commands (like `/ductus:init`) live separately in `.claude/commands/ductus/`.

### Parameterization approach

Every command uses literal `{project}` as the placeholder. This appears in:

- Command cross-references: `/{project}:clarify`, `/{project}:plan`
- Session file path: `.claude/{project}-session.json`
- Command directory references: `.claude/commands/{project}/`

No other placeholders are needed. The bootstrap command (spec 003) handles find-and-replace during project scaffolding.

### Deriving from the source project

Each command is based on the corresponding command in that project with these transformations:

- Replace the project's own name with `{project}` in all references
- Remove its project-specific file paths (`shared/`, `modules/`, `docker-compose.yml`)
- Remove Go-specific conventions (Querier interface, pgx patterns)
- Replace its project-specific template paths (`specs/templates/spec-template.md`) with generic `specs/templates/spec.md`
- Keep constitution references (pipeline gates, readiness check, spec lifecycle)
- Keep AGENTS.md references (conventions, boundaries) as generic pointers

### Spec file detection

> Superseded by [023](../023-govern-refinement/spec.md), which removed the lightweight track and with it the two-filename fallback from every command source. The pattern below is what 000 planned and delivered.

Commands that operate on a feature's spec need to handle both `spec.md` and `spec-and-plan.md`. The pattern is:

1. Check for `spec.md` first
2. If not found, check for `spec-and-plan.md`
3. If neither exists, report "no spec found"

This applies to: target, clarify, plan, implement, validate, next, status.

### Validate and markdownlint

The validate command runs `npx markdownlint-cli2` on all `.md` files in the feature directory as its final check. This is reported as a PASS/FAIL alongside the other artifact checks.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `commands/about.md` | Create | Pipeline overview, no file reads |
| `commands/target.md` | Create | Set session target |
| `commands/status.md` | Create | Dashboard of all specs |
| `commands/setup.md` | Create | Configure permissions |
| `commands/specify.md` | Create | Create new feature spec |
| `commands/clarify.md` | Create | Resolve open questions, advance to clarified |
| `commands/plan.md` | Create | Create plan and tasks, advance to planned |
| `commands/implement.md` | Create | Execute tasks, advance to done |
| `commands/analyze.md` | Create | Audit artifacts for consistency |
| `commands/next.md` | Create | Auto-advance to next pipeline phase |
| `framework/commands/analyze.md` | Edit | Project-level consistency reads the installed command set by its layout-derived rows (scenario `analyze-reads-the-layout-derived-command-set`); its generated copies follow through `scripts/gen-claude-commands.sh` |

## Trade-offs

### Considered: single-file command reference instead of ten files

Rejected. Each command needs enough instruction detail that a single file would be unwieldy. Separate files also match how Claude Code discovers and lists slash commands — one file per command.

### Considered: using a different placeholder syntax (e.g., `{{project}}`, `$PROJECT`)

Rejected. `{project}` is simple and readable in markdown. The decision rested on a second premise as well — that curly braces are not used in the constitution or template content — and that one no longer holds: the brace form became the corpus-wide placeholder convention, so the constitution now carries 57 of them across nine names and `framework/templates/spec/spec.md` six. The decision stands on that consistency rather than on the scarcity it was argued from.

## Open Questions Resolved

All open questions were resolved during clarification. See spec.md Resolved Questions section.
