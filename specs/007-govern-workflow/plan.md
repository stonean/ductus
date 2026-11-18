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

The ductus file contains manifest tables (**Shared Files**, **Per-Agent Scaffolding**) mapping source paths to destination paths with conflict strategies. This is readable, self-documenting, and easy to update. The host builds the manifest entries from the tables, and the runtime's `apply-manifest` applies them against the extracted framework archive.

### Gitignore language patterns fetched from GitHub

For each primary language, fetch from `https://raw.githubusercontent.com/github/gitignore/main/{Language}.gitignore` and append below a comment header. Failures are reported but don't abort.

This is the approach 003 decided for the `.gitignore` merge it owns, inherited rather than decided again here: the shipped template stays minimal, with no language-specific patterns, and each project gets its languages' patterns from GitHub's collection at bootstrap (`002-project-scaffolding` Resolved Questions, *Gitignore and language patterns*), never bundled into the framework archive (`015-tarball-fetch`). A failed fetch does not abort because AC6 holds every individual fetch to that, as the spec's File Fetching section does for an entry missing from the archive.

### Every fetch is bounded

No fetch this command makes waits unboundedly on GitHub (BE-TIMEOUT-001). The per-language `.gitignore` fetch above and the self-update check's fetch of the upstream `ductus.md` (scenario `ductus-self-update-precheck`) each run under `curl --connect-timeout {connect-timeout} --max-time {request-timeout}`, the two named values `framework/bootstrap/ductus.md` defines under **Derived paths**, 10 and 60 seconds (§Shared Files' **.gitignore** merge step 5, and the **Self-update check**'s **Small fetch**). The framework archive is `fetch-archive`'s, bounded by its `FETCH_TIMEOUT` and `FETCH_DEADLINE` constants (`runtime/src/primitives/fetch_archive.rs`). 003's plan states the same bound for the merge it owns.

### Paths built from the inputs are contained

The project name and the spec-root name become path components of what the run writes, `{cli-config-dir}/commands/{project}/` and the files under the spec root among them. The character rules the spec's Inputs section and `validate_specs_root` (`runtime/src/schema/paths.rs`) apply are not what contains them (BE-INPUT-004). The manifest writes are `apply-manifest`'s, `framework/bootstrap/ductus.md` Instructions step 4, and **Slash command cleanup**'s removals are `enforce-manifest`'s, step 8. Each refuses, before any filesystem operation, a path that is absolute, carries a `..` component, or passes through a symbolic link resolving outside its canonical root (`validate_contained` in `runtime/src/primitives/mod.rs`). A refusal stands: the run halts and never falls back to the step's shell specification, which carries none of those screens (`ductus.md` **State A — runtime live this session**; scenario `state-a-deterministic-path-forcing` of `029-bootstrap-runtime-autowire`). A step whose primitive cannot run halts the run too, naming the primitive, rather than fall back to that specification (029 task 17; the same section).

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/bootstrap/ductus.md` | Create | The ductus command, one file for every CLI; its self-update check runs in the pre-flight phase, ahead of the framework archive fetch (scenario `ductus-self-update-precheck`) |
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

The ductus command could do `.claude/` → another CLI's directory replacement at copy time without changing the source templates. Rejected because it's fragile — any new `.claude/` reference in a template would silently break adoption on the other CLI. The `{cli-config-dir}` placeholder makes the abstraction explicit and grep-able.

### Considered: separate command template directories per CLI

Maintaining one command template directory per CLI. Rejected — the commands are identical except for path prefixes: every layout installs the same `framework/commands/*.md` sources, substituting only `{project}` and `{cli-config-dir}` (`framework/bootstrap/ductus.md` §Per-Agent Scaffolding, the Pi layout's **Prompt templates**). Duplicating them would create drift. The placeholder approach keeps one source of truth.

## Open Questions Resolved

All open questions were resolved during clarification. See spec.md Open Questions section.
