---
title: "002-project-scaffolding — plan"
---

# 002 — Project Scaffolding Templates Plan

## Overview

Two template files in the `framework/templates/project/` directory, `gitignore` and `claude-md.md`, are project-level files for adopting projects. `/ductus` installs both (`framework/bootstrap/ductus.md`, Shared files with conflict handling): it merges the `.gitignore` template as a managed `# ductus` block and copies `CLAUDE.md` when one is absent.

## Technical Decisions

### Gitignore stays minimal

The `.gitignore` template contains only `ductus`-universal entries. No language-specific patterns. The file is named `gitignore` (no dot) in `framework/templates/project/` to avoid it being treated as an active gitignore by git. `/ductus` writes it into the project's `.gitignore` as the managed `# ductus` block, creating the file when absent.

### CLAUDE.md imports the constitution and AGENTS.md in the form Claude Code resolves

The template is deliberately minimal: two imports, `@.ductus/constitution.md` then `@AGENTS.md`, and the empty `ductus:constitutions` managed block. Where a learning goes is `AGENTS.md`'s, in the block `/{project}` writes there (`050-constitution`'s `memory-routing-lives-in-agents-md`). Projects may add more configuration later, but the starting point is always the same. Claude Code imports a file only as `@path`, so `@import AGENTS.md` names a file called `import` and loads nothing (Claude Code's memory documentation, *Import additional files*; the scenario's Context records the `claude -p` measurement); the template writes the `@path` form, and the bootstrap writes each shared constitution in the block the same way. The constitution fits one read and is imported first; `/{project}:target` skips a document an import already loaded and reads one that did not load.

`CLAUDE.md` is `skip`-strategy (its row in `framework/bootstrap/ductus.md`'s **Shared files with conflict handling**), so an existing adopter's copy changes only through the `claude-md-import-syntax` migration, which rewrites each `@import <path>` line outside code to `@<path>`, the `.ductus/constitution.md` line included; `constitution-import` (spec 050) adds `@.ductus/constitution.md` where nothing imports the constitution. `check-orphaned-references` reads an `@` that starts a token as the start of a reference, so a dangling `@path` import into a ductus-managed path is reported.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
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

### Considered: including `{description}` placeholder in templates

Rejected for these templates, which use only `{project}`. The `AGENTS.md` template takes the other path, substituting `{One-line project description.}` from the project inputs (`framework/bootstrap/ductus.md`, Shared files with conflict handling).

## Open Questions Resolved

All open questions were resolved during clarification. See spec.md Resolved Questions section.
