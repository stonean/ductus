---
title: "002-project-scaffolding — plan"
---

# 002 — Project Scaffolding Templates Plan

## Overview

Create three new template files in the existing `framework/templates/project/` directory: `project-readme.md`, `gitignore`, and `claude-md.md`. These are project-level files that the init command (spec 003) copies into new projects. They use `{project}` placeholders that get replaced during bootstrap.

## Technical Decisions

### README template derived from an adopter project

The project README template is based on an adopter project's `README.md` structure, generalized:

- Project-specific content (Go, PostgreSQL, NATS, Docker commands) replaced with `{project}` placeholders and generic sections
- Feature table format preserved — it's the standard from the constitution's numbering convention
- Slash command references use `/{project}:*` pattern
- Getting Started section references `/{project}:configure` and `/{project}:status` as decided during clarification

### Gitignore stays minimal

The `.gitignore` template contains only `ductus`-universal entries. No language-specific patterns. The file is named `gitignore` (no dot) in `framework/templates/project/` to avoid it being treated as an active gitignore by git. The init command renames it to `.gitignore` during copy.

### CLAUDE.md imports the constitution and AGENTS.md in the form Claude Code resolves

The template is deliberately minimal: two imports, `@.ductus/constitution.md` then `@AGENTS.md`, the empty `ductus:constitutions` managed block and the Auto-Memory Routing section. Projects may add more configuration later, but the starting point is always the same. Claude Code imports a file only as `@path`, so `@import AGENTS.md` names a file called `import` and loads nothing (Claude Code's memory documentation, *Import additional files*; the scenario's Context records the `claude -p` measurement); the template writes the `@path` form, and the bootstrap writes each shared constitution in the block the same way. The constitution fits one read and is imported first; `/{project}:target` skips a document an import already loaded and reads one that did not load.

`CLAUDE.md` is `skip`-strategy, so an existing adopter's copy changes only through the `claude-md-import-syntax` migration, which rewrites each `@import <path>` line outside code to `@<path>`, the `.ductus/constitution.md` line included; `constitution-import` (spec 050) adds `@.ductus/constitution.md` where nothing imports the constitution. `check-orphaned-references` reads an `@` that starts a token as the start of a reference, so a dangling `@path` import into a ductus-managed path is reported.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/templates/project/project-readme.md` | Create | Project README with quick start, getting started, docs, feature table, pipeline, slash commands |
| `framework/templates/project/gitignore` | Create | Minimal .gitignore for secrets, claude settings, IDE, OS files |
| `framework/templates/project/claude-md.md` | Create | CLAUDE.md importing `@.ductus/constitution.md` and `@AGENTS.md`, with the shared-constitution managed block |
| `framework/migrations.toml` | Modify | `claude-md-import-syntax` registry entry |
| `framework/migrations/claude-md-import-syntax.md` | Create | Migration procedure rewriting an adopter's `@import` lines |
| `framework/bootstrap/ductus.md` | Modify | Shared constitution imports render `@{document-path}`; how the block loads |
| `framework/bootstrap/govern.md` | Modify | Byte-identical copy of `ductus.md` |
| `framework/bootstrap/ductus-procedure.md` | Modify | Step 8's dangling-import sentence |
| `framework/commands/target.md` | Modify | Step 4 loads a document only when it is not already in context |
| `framework/reference/analyze.md` | Modify | Orphaned-references dangling-import sentence |
| `docs/shared-constitution.md` | Modify | §How it loads |
| `runtime/src/primitives/check_orphaned_references.rs` | Modify | An `@` starting a token begins a reference |
| `scripts/mutation/claude-md-imports.py` | Create | Mutations proving the two `@path` tests |

## Trade-offs

### Considered: naming the README template `readme.md`

Rejected. Using `project-readme.md` avoids confusion — `readme.md` could be mistaken for the `ductus` project's own README. The `project-` prefix makes it clear this is a template for adopting projects.

### Considered: including `{description}` placeholder in templates

Rejected. Only `{project}` is used as a placeholder. The project description appears in specific locations (README header, AGENTS.md intro) and the init command fills those in directly from user input rather than using a second placeholder.

## Open Questions Resolved

All open questions were resolved during clarification. See spec.md Resolved Questions section.
