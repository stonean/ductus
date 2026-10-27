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

## 6. Disposition out-of-spec finding: 017 AC16 names a root CLAUDE.md import that is gone

- [x] Route the correction to `050-constitution` task 35, whose open question decides whether this repository's `CLAUDE.md` imports the constitution and so what AC16 should say

- **Done when**: `017-derive-dont-ask` AC16 ("root `CLAUDE.md` imports it via `@import framework/constitution.md`") is false since `072-governing-documents-stay-within-one-read` made this repository's `CLAUDE.md` read `@AGENTS.md` alone, and the correction is recorded as prose in 050 task 35 beside the import it depends on, so the reopen it needs is spent once, when the owner answers that question.

## 7. Disposition out-of-spec finding: 022's constitution-excerpts scenario rests on the template's constitution import

- [x] Discard it with its reason: `022-deterministic-runtime`'s scenarios belong to a paused session the owner excluded from this work on 2026-10-26, so the premise is reported to the owner instead of edited

- **Done when**: `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md` says every adopter's `CLAUDE.md` carries `@import .ductus/constitution.md`, so the host already holds the constitution. That import never loaded, and the template no longer writes it. The discard and its reason are recorded here, and the premise is named in this session's report to the owner.
