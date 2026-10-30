---
title: "006-bug-workflow — plan"
---

# 006 — Bug Workflow Plan

## Overview

Add scenario support, a bug decision tree, and a brownfield inbox to the framework. This involves creating two new templates, two new slash commands (`/ductus:amend` and `/ductus:groom`), updating four existing commands and their templates, and updating the constitution and README. All artifacts are markdown files — no application code, no persistence.

## Technical Decisions

### Scenarios live alongside their parent spec

Scenario files are placed in `specs/{NNN-feature}/scenarios/{slug}.md`. This co-locates scenarios with the spec they elaborate, making them discoverable without a separate index. The `scenarios/` subdirectory keeps the feature directory uncluttered.

### Scenario template uses plain sections, not Given/When/Then

The spec explicitly states Given/When/Then is not required. The template uses a `section` frontmatter field, Context, Behavior, and Edge Cases sections. This matches the framework's preference for plain language over formal syntax.

### `/ductus:amend` creates both the scenario file and a task entry

When a scenario is created, `/ductus:amend` also appends a task to the parent spec's `tasks.md`. If `tasks.md` does not exist, it creates one. This ensures every scenario has a corresponding implementation task that carries completion status.

### `/ductus:groom` operates on a flat `specs/inbox.md` file

The inbox is a capture queue — a flat markdown list, not a directory structure. Each item is walked through the decision tree and migrated to the appropriate spec or scenario. Items are removed from `specs/inbox.md` as they are resolved. When `specs/inbox.md` is empty, the command reports the inbox is clean. The file is kept to preserve git history.

### Command templates updated in both `framework/commands/` and `.claude/commands/ductus/`

Template changes go into `framework/commands/` (the source of truth for adopting projects). The dogfooded copies in `.claude/commands/ductus/` are then re-derived from the template with `{project}` replaced by `ductus`, by `scripts/gen-claude-commands.sh`. This maintains the dogfooding principle from spec 003.

### Constitution updates are additive

New sections are added to the constitution for bug handling and scenario lifecycle. The existing spec phase file structure is extended to include `scenarios/`. No existing sections are removed or restructured.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/templates/spec/scenario.md` | Create | Scenario document starter with a `section` frontmatter field, Context, Behavior, Edge Cases |
| `framework/templates/project/inbox.md` | Create | Inbox heading and introduction, pointing at §brownfield-inbox for the rules that migrate its items |
| `framework/templates/spec/spec.md` | Modify | Add reference to scenarios directory convention |
| `framework/constitution.md` | Modify | Add bug handling section with decision tree, scenario lifecycle, scenario directory convention |
| `framework/commands/amend.md` | Create | `/amend` command template for creating scenario files |
| `framework/commands/groom.md` | Create | `/groom` command template for reviewing and migrating inbox items |
| `framework/commands/help.md` | Modify | Add `/amend` and `/groom` to command tables, add scenario concepts |
| `framework/commands/status.md` | Modify | Add scenario counts per spec to dashboard |
| `framework/commands/analyze.md` | Modify | Add scenario-linked task completeness check |
| `.claude/commands/ductus/amend.md` | Create | Dogfooded copy with `ductus` replacing `{project}` |
| `.claude/commands/ductus/groom.md` | Create | Dogfooded copy with `ductus` replacing `{project}` |
| `.claude/commands/ductus/help.md` | Modify | Re-derive from updated template |
| `.claude/commands/ductus/status.md` | Modify | Re-derive from updated template |
| `.claude/commands/ductus/analyze.md` | Modify | Re-derive from updated template |
| `README.md` | Modify | Document bug workflow and scenario conventions |

## Trade-offs

### Considered: scenario files with their own status field

Rejected. Scenarios are permanent requirement documents. Adding a status field would create a parallel lifecycle that conflicts with the spec lifecycle. The task in `tasks.md` carries the completion status instead.

### Considered: bug files as a standard artifact

Rejected for the default case. The spec explicitly states bug files are rarely needed. Scenarios are the primary artifact. Bug files are documented as an exception for complex root causes, reproduction context, or deferred workarounds.

### Considered: the inbox as a directory of individual files

Rejected. A flat markdown file is simpler for a temporary inbox. Individual files add filesystem overhead for items that should be migrated quickly.

### Considered: updating only `framework/commands/` templates and not `.claude/commands/ductus/`

Rejected. This repo must dogfood its own commands. Per spec 003, the generated copies are re-derived from templates after any template change.

## Open Questions Resolved

All open questions were resolved during clarification. See spec.md — the Open Questions section confirms all resolved.
