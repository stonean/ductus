---
title: "011-brownfield-process — tasks"
---

# 011 — Brownfield Process Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Rename triage to inbox

Rename files and update all references from `triage` to `inbox` across the framework. This is a mechanical rename done first so subsequent tasks work with the final naming.

- [x] Rename `commands/triage.md` → `commands/inbox.md`
- [x] Rename `.claude/commands/ductus/triage.md` → `.claude/commands/ductus/inbox.md`
- [x] Rename `templates/triage.md` → `templates/inbox.md`
- [x] Update content in `commands/inbox.md` (heading, references, prose)
- [x] Update content in `.claude/commands/ductus/inbox.md` (heading, references, prose)
- [x] Update content in `templates/inbox.md` (heading, content)
- [x] Update `ductus/ductus.md` file manifest, command manifest, and post-scaffolding output
- [x] Update `ductus/ductus-auggie.md` file manifest, command manifest, and post-scaffolding output
- [x] Update `constitution.md` section heading, marker, and content
- [x] Update `sdd-context.md` references
- [x] Update `README.md` references
- [x] Update `commands/about.md` references
- [x] Update `.claude/commands/ductus/about.md` references
- [x] Update `AGENTS.md` references
- [x] Add signpost to `specs/006-bug-workflow/spec.md` noting the rename
- [x] Run `npx markdownlint-cli2` on all modified files

**Done when:** no file in the repository contains `triage` except in 006's historical spec content and the signpost note, and 011's own spec references.

## 2. Create capture command

Create the `/capture` command in both platform-agnostic and Claude Code forms.

- [x] Create `commands/capture.md` with freeform input flow, skeleton spec creation, session target update, and post-capture options
- [x] Create `.claude/commands/ductus/capture.md` as Claude Code instance with `/ductus:` prefix and `.claude` paths
- [x] Verify command file parity between the two files
- [x] Run `npx markdownlint-cli2` on both files

**Done when:** both capture command files exist, pass lint, and follow the same structure as other commands.

## 3. Update ductus file manifests and add migration

Add the capture command to the ductus file manifests and add a triage → inbox migration step.

- [x] Add `commands/capture.md` to `ductus/ductus.md` slash command manifest with `update` strategy
- [x] Add `commands/capture.md` to `ductus/ductus-auggie.md` slash command manifest with `update` strategy
- [x] Add triage → inbox migration to `ductus/ductus.md`: rename `specs/inbox.md` to `specs/inbox.md` if needed, merge if both exist, delete old triage command
- [x] Add triage → inbox migration to `ductus/ductus-auggie.md`: same migration with Auggie paths
- [x] Migration is reported in post-scaffolding summary
- [x] Add signpost to `specs/007-govern-workflow/spec.md` noting the ductus command changes by this spec
- [x] Run `npx markdownlint-cli2` on both ductus files and 007 spec

**Done when:** both ductus files include the capture command in their manifests and perform the triage → inbox migration for previously adopted projects.

## 4. Document brownfield process in constitution

Add the brownfield process, scenario promotion, and cross-spec impact patterns to `constitution.md`.

- [x] Add brownfield process section under the existing brownfield inbox section — documents capture → incremental growth → promotion lifecycle
- [x] Add scenario promotion subsection under the existing scenarios section — documents indicators and the promotion pattern
- [x] Add cross-spec impact as a pipeline boundary — documents that changes land where they belong with signpost references
- [x] Run `npx markdownlint-cli2` on `constitution.md`

**Done when:** constitution documents all three patterns and passes lint.

## 5. Update sdd-context and README

Update documentation to reflect the brownfield process.

- [x] Add capture command to `sdd-context.md` slash commands table
- [x] Add brownfield process section to `sdd-context.md` (capture, incremental growth, scenario promotion)
- [x] Add cross-spec impact to `sdd-context.md`
- [x] Update `README.md` slash commands table — add `/capture`, rename `/triage` to `/inbox`
- [x] Update `README.md` brownfield section to reference the process
- [x] Run `npx markdownlint-cli2` on both files

**Done when:** both files reflect the brownfield process, capture command, inbox rename, and pass lint.
