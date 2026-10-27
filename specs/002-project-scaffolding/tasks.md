---
title: "002-project-scaffolding — tasks"
---

# 002 — Project Scaffolding Templates Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Create framework/templates/project/project-readme.md

- [x] Create the template with sections: project name/description, Quick Start, Getting Started (referencing setup and status commands), Documentation, Feature Specs table, Development Pipeline, Slash Commands table, Working on Existing Specs
- [x] Use `{project}` placeholder in all command references and project name locations
- [x] Include an empty feature table with correct column headers (Spec, Status, Dependencies, Description)
- [x] Include the pipeline diagram with `/{project}:*` command references

Done when: `framework/templates/project/project-readme.md` exists with all sections, uses `{project}` placeholders, includes getting started with configure/status references, passes markdownlint.

## 2. Create framework/templates/project/gitignore

- [x] Create the template with sections: secrets (`.env`, `.env.*`), Claude settings (`.claude/*`, `!.claude/commands/`), IDE files (`.vscode/`, `.idea/`, `*.swp`, `*.swo`, `*~`), OS files (`.DS_Store`, `Thumbs.db`)
- [x] Name the file `gitignore` (no dot) to avoid being treated as active gitignore

Done when: `framework/templates/project/gitignore` exists with minimal entries, preserves `.claude/commands/`, no language-specific patterns.

## 3. Create framework/templates/project/claude-md.md

- [x] Create the template with `@import constitution.md` and `@import AGENTS.md`

Done when: `framework/templates/project/claude-md.md` exists with both import directives, passes markdownlint.

## 4. Final review and lint

- [x] Run `npx markdownlint-cli2` on markdown templates (`project-readme.md`, `claude-md.md`)
- [x] Verify `{project}` placeholder is used consistently across all templates
- [x] Compare against an adopter project's README, .gitignore, and CLAUDE.md to ensure no important sections are missing
- [x] Update spec status to `done`

Done when: all templates pass lint (where applicable), placeholders are consistent, and no major gaps compared to the adopter project.

## 5. The CLAUDE.md template's imports load

- [ ] Implement the behavior described in `scenarios/claude-md-template-imports-load.md`

The constitution stays over the 50,000-byte read size (53,813 bytes; `050-constitution` task 35 left the target as its scenario's open question), so this task follows the scenario as written: the template imports `@AGENTS.md` and drops the `@import .ductus/constitution.md` line, and the migration rewrites each `@import <path>` line outside code to `@<path>` and removes the line importing `.ductus/constitution.md`; give it `introduced_in = "0.60.0"`, the next release, which is the owner's to cut. The surfaces to correct are `framework/bootstrap/ductus.md` and its twin `govern.md` (the `claude-style` shared-constitution rendering near line 911, and the "navigation, not the loading mechanism" paragraph after it), `framework/commands/target.md` (step 4 and its Scope Boundaries line: load a document only when it is not already in context, an imported one counting as loaded), `docs/shared-constitution.md` §How it loads, the "dangling `@import`" sentences in `framework/reference/analyze.md` and `framework/bootstrap/ductus-procedure.md` step 8, and this spec's §CLAUDE.md template, AC3 and plan Affected Files. `check-orphaned-references` does **not** recognise an `@path` import today: `managed_paths_in` in `runtime/src/primitives/check_orphaned_references.rs` skips any path preceded by `@`. Allow an `@` that starts a token (line start or after whitespace), and add a test in each direction (an `@.ductus/missing.md` line is reported; `x@.ductus/y` is not). Auggie cannot be run here, so record its `@path` support as unverified against the scenario's fallback edge case. A migration change needs a real adopter bootstrap per `AGENTS.md`, after the push.

- **Done when**: the scenario's described behavior is correctly implemented and tested
