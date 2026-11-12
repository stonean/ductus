---
title: "000-slash-commands — plan"
---

# 000 — Slash Command Templates Plan

## Overview

Generic slash command `.md` files, one per command, in `framework/commands/`. Each command is derived from the working implementation in a prior adopter project — a Go backend service that had built its own slash commands — but generalized: that project's own references are replaced with `{project}` placeholders, and its language-specific logic (Go code style, module patterns) is removed in favor of references to the constitution and AGENTS.md.

## Technical Decisions

### Directory location

Command sources live at `framework/commands/{command}.md`; the `configure` command is the exception, sourced per agent from `framework/bootstrap/configure/{agent}.md`. This is the template source — `/ductus` installs each command at the agent's command path (`.claude/commands/{project}/` for Claude Code; `framework/bootstrap/ductus.md` §Derived values lists every layout) and replaces the placeholders.

Rationale: Keeping them under `framework/` (not under `.claude/commands/`) avoids them being treated as active slash commands in the `ductus` repo itself. The repo's own installed copies in `.claude/commands/ductus/` are generated from these sources by `scripts/gen-claude-commands.sh`.

### Parameterization approach

Every command uses literal `{project}` as the project placeholder, and `{cli-config-dir}` for the agent's config directory (added by `012-multi-agent-govern`). They appear in:

- Command cross-references: `/{project}:clarify`, `/{project}:plan`
- Command directory references: `{cli-config-dir}/commands/{project}/`

No other placeholders are needed. The bootstrap command (spec 003) handles find-and-replace during project scaffolding.

### Deriving from the source project

Each command is based on the corresponding command in that project with these transformations:

- Replace the project's own name with `{project}` in all references
- Remove its project-specific file paths (`shared/`, `modules/`, `docker-compose.yml`)
- Remove Go-specific conventions (Querier interface, pgx patterns)
- Replace its project-specific template paths (`specs/templates/spec-template.md`) with generic `specs/templates/spec.md`
- Keep constitution references (pipeline gates, readiness check, spec lifecycle)
- Keep AGENTS.md references (conventions, boundaries) as generic pointers

### Analyze and markdownlint

`/{project}:analyze` lints every markdown file in the feature directory through `lint-markdown`, which runs `markdownlint-cli2`, and reports each violation as an advisory finding.

### A `done` stop says the request is not built

The message shape is one bullet in the constitution's §spec-lifecycle, which every command loads; `amend`, `clarify`, `plan`, `implement` and `target` point at it, never copy it, and `target`'s `done` row takes the qualified form. The bullet states the "Not built yet: {request}." line, said only when new work is asked for, and the qualified `done`, always; `/{project}:status`'s cell is unchanged, and the bullet leaves it unnamed since the constitution sits at its byte ceiling. `cross-spec-impact:` names `014-reclarify-backedge` only: its criteria quote `/clarify`'s `done` message and `/amend`'s re-open prompt. `023-govern-refinement`'s AC30 still holds (the `done` row points at `/amend`); only its annotation's quote of the old row wording is provenance of the day it shipped, so it takes no entry.

### `/amend`'s criterion route writes with host tools, not a primitive

No runtime primitive appends an acceptance criterion, and none is added. The route is a classifier signal plus four steps in `framework/commands/amend.md`: a normalized-whitespace duplicate check by the host (the check the question route's preview already makes), the host appending `- [ ] {text}` to `## Acceptance Criteria` (creating the section in template order when missing), `label-criteria` for the label, and `set-status` for the `done` back-edge. The label, the counter and the status are the primitives' writes; the append is one unlabelled list item, which a primitive would only restate. A `draft` spec is refused, since `/{project}:clarify` owns criteria there (owner, 2026-11-12).

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/commands/help.md` | Create | Pipeline overview, no file reads |
| `framework/commands/target.md` | Create | Set session target |
| `framework/commands/status.md` | Create | Dashboard of all specs |
| `framework/bootstrap/configure/claude.md` | Create | Configure permissions |
| `framework/commands/specify.md` | Create | Create new feature spec |
| `framework/commands/clarify.md` | Create | Resolve open questions, advance to clarified |
| `framework/commands/plan.md` | Create | Create plan and tasks, advance to planned |
| `framework/commands/implement.md` | Create | Execute tasks, advance to done |
| `framework/commands/analyze.md` | Create | Audit artifacts for consistency; project-level consistency reads the installed command set by its layout-derived rows (scenario `analyze-reads-the-layout-derived-command-set`), and its generated copies follow through `scripts/gen-claude-commands.sh` |

## Trade-offs

### Considered: a single-file command reference instead of one file per command

Rejected. Each command needs enough instruction detail that a single file would be unwieldy. Separate files also match how Claude Code discovers and lists slash commands — one file per command.

### Considered: using a different placeholder syntax (e.g., `{{project}}`, `$PROJECT`)

Rejected. `{project}` is simple and readable in markdown. The decision rested on a second premise as well — that curly braces are not used in the constitution or template content — and that one no longer holds: the brace form became the corpus-wide placeholder convention, so the constitution now carries 57 of them across nine names and `framework/templates/spec/spec.md` six. The decision stands on that consistency rather than on the scarcity it was argued from.

## Open Questions Resolved

All open questions were resolved during clarification. See spec.md Resolved Questions section.
