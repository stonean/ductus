---
title: "007-govern-workflow — plan"
---

# 007 — Ductus Command Plan

## Overview

The `/ductus` command, `framework/bootstrap/ductus.md`, is a self-contained prompt that instructs the AI agent to fetch templates from GitHub, scaffold `ductus` files, resolve placeholders, and display next steps. One file serves every supported CLI. The command templates in `framework/commands/` carry a `{cli-config-dir}` placeholder so a single template set serves all CLIs.

## Technical Decisions

### One ductus file for every CLI

`framework/bootstrap/ductus.md` carries an Agent Registry, one row per supported CLI, and selects the agent at runtime (`012-multi-agent-govern`). The CLI-specific values (config directory, command path, settings file) are registry columns, not separate files.

Adding a new CLI means adding a registry row. The framework core (constitution, templates, command templates) does not change.

### Command templates use `{cli-config-dir}` placeholder

The command templates in `framework/commands/` reference `.claude/` paths for session state and settings. To support multiple CLIs, these references change to `{cli-config-dir}` — a placeholder resolved during adoption alongside `{project}`.

For this repository's own commands in `.claude/commands/ductus/`, the placeholder is already resolved to `.claude`. Adopting projects get it resolved to whichever CLI they chose.

Affected references in command templates:

- `.claude/settings.local.json` → `{cli-config-dir}/settings.local.json`
- `.claude/commands/{project}/` → `{cli-config-dir}/commands/{project}/`

### The ductus file lives in `framework/bootstrap/`, not in templates/

The ductus file is the distribution entry point: `install.sh` places it as the agent's `/ductus` command. It is not a template copied into projects; it is a bootstrap command, kept beside the other installer sources in `framework/bootstrap/`.

### File manifest is a markdown table in the ductus file

The ductus file contains manifest tables (**Shared Files**, **Per-Agent Scaffolding**) mapping source paths to destination paths with conflict strategies. This is readable, self-documenting, and easy to update. The agent reads the tables and executes the fetches.

### Gitignore language patterns fetched from GitHub

For each primary language, fetch from `https://raw.githubusercontent.com/github/gitignore/main/{Language}.gitignore` and append below a comment header. Failures are reported but don't abort.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/bootstrap/ductus.md` | Create | The ductus command, one file for every CLI; its self-update pre-check runs File Fetching before Frontmatter Migration (scenario `ductus-self-update-precheck`) |
| `framework/commands/help.md` | Modify | Replace `.claude/` with `{cli-config-dir}/` |
| `framework/commands/specify.md` | Modify | Replace `.claude/` with `{cli-config-dir}/` |
| `framework/commands/clarify.md` | Modify | Replace `.claude/` with `{cli-config-dir}/` |
| `framework/commands/plan.md` | Modify | Replace `.claude/` with `{cli-config-dir}/` |
| `framework/commands/implement.md` | Modify | Replace `.claude/` with `{cli-config-dir}/` |
| `framework/commands/status.md` | Modify | Replace `.claude/` with `{cli-config-dir}/` |
| `framework/commands/target.md` | Modify | Replace `.claude/` with `{cli-config-dir}/` |
| `framework/bootstrap/configure/claude.md` | Modify | Replace `.claude/` with `{cli-config-dir}/` |
| `framework/commands/analyze.md` | Modify | Replace `.claude/` with `{cli-config-dir}/` |
| `framework/commands/groom.md` | Modify | Replace `.claude/` with `{cli-config-dir}/` |
| `.claude/commands/ductus/*.md` | Modify | Re-derive from updated templates with `{cli-config-dir}` → `.claude` |
| `specs/007-govern-workflow/spec.md` | Modify | Update status to `done` |

## Trade-offs

### Considered: single ductus file with CLI prompt

A single `ductus.md` that asks "Which CLI are you using?" at runtime. Rejected because the user already chose their CLI by installing the file into a specific directory. A prompt adds friction and the file can't know which directory it was placed in.

**Adopted, not rejected — [012-multi-agent-govern](../012-multi-agent-govern/spec.md) reversed this.** The shipped installer is exactly the option this section turned down: one `framework/bootstrap/ductus.md` carrying an Agent Registry, selecting the agent at runtime via a first-run prompt (and `--add-agent` afterwards). The premise did not survive the change it was reasoning about: once there is a single file at a single URL, the user no longer "already chose their CLI by installing the file into a specific directory", so the argument against the prompt dissolved with the two-file model that supported it. Recorded here rather than deleted because it is the decision 007 actually made; left un-swept would read as a live objection to the design that won — the drift `AGENTS.md` §Gotchas records as invisible to every check, since the option's name never changed and the prose carries none of the tokens an identifier sweep greps for.

### Considered: keeping `.claude/` hardcoded in command templates, substituting only in ductus

The ductus command could do `.claude/` → `.augment/` replacement at copy time without changing the source templates. Rejected because it's fragile — any new `.claude/` reference in a template would silently break Auggie adoption. The `{cli-config-dir}` placeholder makes the abstraction explicit and grep-able.

### Considered: separate command template directories per CLI

Maintaining one command template directory for Claude Code and a second for Auggie. Rejected — the commands are identical except for path prefixes. Duplicating them would create drift. The placeholder approach keeps one source of truth.

## Open Questions Resolved

All open questions were resolved during clarification. See spec.md Open Questions section.
