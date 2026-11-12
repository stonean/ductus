---
title: "002-project-scaffolding — spec"
status: in-progress
dependencies: [000-slash-commands, 001-system-spec-templates]
tags: [templates, bootstrap]
next-criterion: 8
---

# 002 — Project Scaffolding Templates

Templates for the project-level files that every `ductus`-adopting project needs beyond the constitution, AGENTS.md, and spec templates.

## Problem

Bootstrapping a new project requires creating several files that follow `ductus` conventions but are not currently provided as templates: a project README with a feature status table, a `.gitignore` that excludes claude settings but preserves commands, a `CLAUDE.md` with import directives, and a session state file. Adopter projects created all of these independently.

## Behavior

`ductus` provides additional templates in the `framework/templates/project/` directory for project-level files.

### README.md template

**Retired** (spec 003 task 6, the owner's answer (A), 2026-11-12). The template reached no adopter once `/{project}:init`, which copied it, was retired: `/ductus` never writes `README.md`, and an adopter's README serves its own product's users (013). Its file is deleted, and AC1 and AC5 are annotated.

### .gitignore template

A minimal baseline `.gitignore` for `ductus`-adopting projects:

- Environment and secrets (`.env`, `.env.*`)
- Claude settings exclusion with commands exception (`.claude/*`, `!.claude/commands/`)
- IDE and OS files (`.vscode/`, `.idea/`, `.DS_Store`, etc.)

The template stays minimal — no language-specific patterns. Language-specific entries are populated during bootstrap (003) by fetching from github.com/github/gitignore based on the project's tech stack.

### CLAUDE.md template

A minimal file whose imports use Claude Code's `@path` form, the only one it resolves (Claude Code's memory documentation, *Import additional files*; measured in scenario `claude-md-template-imports-load`):

- Imports the constitution as `@.ductus/constitution.md`, which fits one read, and `AGENTS.md` as `@AGENTS.md` (scenario `claude-md-template-imports-load`)
- Carries the `ductus:constitutions` managed block, where `/ductus` imports each registered shared constitution

## Acceptance Criteria

- [x] AC1: framework/templates/project/project-readme.md exists with placeholder sections for project name, quick start, getting started (referencing the configure and status commands), documentation links, feature table, pipeline overview, and slash command reference **Retired (2026-11-12):** the template is deleted, since it reached no adopter after `/{project}:init` was retired (spec 003 task 6).
- [x] AC2: `framework/templates/project/gitignore` exists with minimal sections for secrets, claude settings, IDE files, and OS files — no language-specific patterns
- [x] AC3: `framework/templates/project/claude-md.md` exists, imports `.ductus/constitution.md` as `@.ductus/constitution.md` and `AGENTS.md` as `@AGENTS.md`, and holds no `@import` line
- [x] AC4: Each template uses `{project}` placeholder where the project name appears
- [x] AC5: The README template includes a feature table matching the format used by constitution's spec numbering convention **Retired (2026-11-12)** with the template (spec 003 task 6).
- [x] AC6: The .gitignore preserves `.claude/commands/` while excluding other `.claude/` contents
- [x] AC7: All markdown templates pass markdownlint

## Resolved Questions

- **README and setup command** — the README template must include a "Getting Started" section referencing `/{project}:configure` and `/{project}:status` as the onramp for new contributors.
- **Gitignore and language patterns** — the template stays minimal (secrets, claude settings, IDE, OS). Language-specific patterns are fetched from github.com/github/gitignore during bootstrap (spec 003) based on the project's tech stack.
- **Settings.local.json template** — no static template. The `/{project}:configure` command, named `setup` when this was decided, handles creation and configuration of `.claude/settings.local.json` as the single source of truth for permissions.

## References

Declared dependencies for this spec, surfaced here so the `derive-dependencies` runtime primitive sees them in the body.

- [000-slash-commands](../000-slash-commands/spec.md)
- [001-system-spec-templates](../001-system-spec-templates/spec.md)
