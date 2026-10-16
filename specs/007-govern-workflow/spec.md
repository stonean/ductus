---
title: "007-govern-workflow — spec"
status: in-progress
dependencies: [003-bootstrap-automation]
tags: [bootstrap, commands]
next-criterion: 15
---

# 007 — Ductus Workflow

> **Note:** This spec was renamed from `007-adopt-workflow` to `007-govern-workflow` by [011-brownfield-process](../011-brownfield-process/spec.md). The ductus command also gains a triage → inbox migration step and `/capture` in the command manifest via 011.
>
> **Superseded in part by [012-multi-agent-govern](../012-multi-agent-govern/spec.md).** The two-file distribution model described below (`ductus/ductus.md` for Claude Code, `ductus/ductus-auggie.md` for Auggie) is replaced by a single unified `ductus/ductus.md` with an agent registry. The `{cli-config-dir}` placeholder approach remains correct; 012 changed only the file count and the runtime agent-selection mechanism, and how the command fetches is the signpost below. 007's status stays `done` because its work shipped; 012 carries forward the new design.
>
> **Note:** path references below (`ductus/ductus.md`, `templates/system.md`, `templates/spec.md`, etc.) reflect the original layout. The repository was later reorganized so the ductus installer lives at `framework/bootstrap/ductus.md`, spec templates at `framework/templates/spec/`, and project-scaffolding templates at `framework/templates/project/`. Adopting projects' destination paths did not change.
>
> **Note:** the `.gitignore` merge *mechanism* described below is superseded, while its requirement is not. This spec describes a `# Governance` comment header checked with an inline grep before appending. The marker was renamed twice — `# Governance` → `# govern` by the `gitignore-marker-rename` migration, then to `# ductus` — and the mechanism itself became the `merge-managed-block` primitive (line-prefix style, marker `ductus`), which rewrites a delimited managed region rather than appending below a header. AC5's idempotency requirement still holds and is still what the installer guarantees; only the marker string and the means of achieving it changed. `# Governance` survives in this spec, in the migration procedure, and nowhere else that states current behaviour.
>
> **Signpost:** where the command fetches from changed twice after this spec shipped, so the `main` URLs below — the install lines and the **Source URL pattern** — record the original design rather than live behaviour. [015](../015-tarball-fetch/spec.md) replaced the per-file fetches with one framework-archive download, and [061 — Updates track the latest release tag](../061-updates-track-the-latest-release-tag/spec.md) made every fetch back into the `ductus` repository — the bootstrap self-update, the runtime version pin, and that archive — name one source resolved per run: the latest release by default, or `main` or a named release tag through `--ref`. The installer is itself a release asset, fetched from `releases/latest/download/install.sh`.

A self-contained slash command file that bootstraps `ductus` in existing (brownfield) projects. Users fetch a single `.md` file into their CLI's command directory and run it — no clone of the `ductus` repo required. The command instructs the AI agent to fetch templates from GitHub, write them into the correct locations, perform placeholder substitution, handle conflicts with existing files, and display brownfield-specific next steps.

The command supports multiple AI coding CLIs. Each CLI gets native directory paths and configuration formats — no backward-compatibility shims.

## Distribution Model

The deliverable is one markdown file per supported CLI, hosted in the `ductus` repo. Each file is self-contained and tailored to its target CLI's conventions.

### Supported CLIs

| CLI | Command file | Install location | Invoke |
| ----- | ------------- | --------------- | -------- |
| Claude Code (default) | `ductus/ductus.md` | `.claude/commands/ductus.md` | `/ductus {project-name}` |
| Auggie | `ductus/ductus-auggie.md` | `.augment/commands/ductus.md` | `/ductus {project-name}` |

Install with one command:

```text
# Claude Code
curl -fsSL https://raw.githubusercontent.com/stonean/ductus/main/ductus/ductus.md \
  > .claude/commands/ductus.md

# Auggie
curl -fsSL https://raw.githubusercontent.com/stonean/ductus/main/ductus/ductus-auggie.md \
  > .augment/commands/ductus.md
```

No runtime, no dependencies, no build step — the "program" is a prompt. Each variant contains the same scaffolding logic but targets its CLI's native paths and configuration formats.

## Inputs

The command collects from `$ARGUMENTS` or prompts interactively:

1. **Project name** — lowercase, alphanumeric, hyphens allowed. Used for `{project}` placeholder substitution and command directory naming.
2. **Project description** — one-line description for AGENTS.md.
3. **Primary language(s)** — comma-separated list for .gitignore language patterns.

The target CLI is implicit — determined by which `ductus.md` variant the user installed. The command file itself knows its target and uses the correct paths throughout.

## Pre-flight Checks

Before scaffolding, verify:

- The current directory **is** an existing git repository.
- A `specs/` directory does **not** already exist (`ductus` not yet adopted). If it does, stop and report: "This project already has a specs/ directory. If you want to re-run adoption, remove it first."

## File Fetching

The command contains a manifest of files to fetch from the `ductus` repo. Each entry specifies:

- Source path (relative to the `ductus` repo root)
- Destination path (relative to project root)
- Conflict strategy: `skip` (don't overwrite), `merge` (append), or `create` (must not exist)

Source URL pattern:

```text
https://raw.githubusercontent.com/stonean/ductus/main/{source-path}
```

If a fetch fails, report the failure and continue with remaining files. The command must not abort on a single fetch error.

## Scaffolding Behavior

### CLI-agnostic files (strategy: create)

These files are identical regardless of target CLI:

- `constitution.md` — copied as-is from the repo root
- `.markdownlint-cli2.jsonc` — copied as-is from the repo root
- `specs/system.md` — from `templates/system.md`
- `specs/errors.md` — from `templates/errors.md`
- `specs/events.md` — from `templates/events.md`
- `specs/inbox.md` — from `templates/inbox.md`
- `specs/templates/spec.md` — from `templates/spec.md`
- `specs/templates/plan.md` — from `templates/plan.md`
- `specs/templates/tasks.md` — from `templates/tasks.md`
- `specs/templates/data-model.md` — from `templates/data-model.md`
- `specs/templates/research.md` — from `templates/research.md`
- `specs/templates/scenario.md` — from `templates/scenario.md`

### CLI-specific files (strategy: create)

These files use native paths and formats for the target CLI:

| File | Claude Code | Auggie |
| ------ | ------------ | -------- |
| Slash commands | `.claude/commands/{project}/*.md` | `.augment/commands/{project}/*.md` |
| Rules file | `CLAUDE.md` (from `templates/claude-md.md`) | `CLAUDE.md` (Auggie reads it natively) |

Slash command templates use a `{cli-config-dir}` placeholder for CLI-specific paths. The ductus command resolves this placeholder to the target CLI's native directory (`.claude` or `.augment`) during copy. Session state is not in this table — post-0.10.0 it lives at the repo-root `.ductus/session.toml` and is host-agnostic.

### Files with conflict handling

- **AGENTS.md** (strategy: skip) — if it exists, leave it alone. If not, copy from the framework and substitute project name and description.
- **CLAUDE.md** (strategy: skip) — if it exists, leave it alone. If not, copy from `templates/claude-md.md`. Used by both Claude Code and Auggie.
- **.gitignore** (strategy: merge) — if it exists, append the framework-managed patterns and language-specific patterns below existing content, separated by a `# Governance` comment header. If not, create from `templates/gitignore` plus language patterns.

### Placeholder substitution

In every copied file, replace:

- `{project}` and `{project-name}` with the user-provided project name
- `{One-line project description.}` with the user-provided description

### What the command does NOT do

- Modify `README.md` — the project's README is its own; `ductus` doesn't touch it
- Create feature specs — the user does that via `/{project}:specify`
- Fill in AGENTS.md content — that requires project-specific knowledge
- Fill in system.md content — that requires architectural decisions
- Make git commits — the user decides when to commit
- Run `/{project}:setup` — that happens after adoption, interactively

## Post-Scaffolding Output

After scaffolding, display:

- Summary of files created, skipped, and merged
- Any fetch failures encountered
- Brownfield-specific next steps (command names use the project's slash-command prefix):
  1. Run `/{project}:setup` to configure permissions
  2. Fill in `AGENTS.md` — tech stack, project structure, code style, testing conventions, gotchas
  3. Fill in `specs/system.md` — architecture, request lifecycle, shared infrastructure
  4. Populate `specs/inbox.md` with known issues and bugs
  5. Run `/{project}:inbox` to migrate items to specs and scenarios
  6. Create your first feature spec: `/{project}:specify {feature description}`

## Self-Maintenance

The command file remains in the CLI's command directory after execution. It is idempotent — running it again skips already-created files and only attempts files that are missing. The user may delete it after governing if desired.

## Edge Cases

- **No network access** — all fetches fail. The command reports all failures and produces no files. It does not create a partial scaffold from local-only content.
- **Partial previous run** — some files exist from a prior incomplete run. Idempotency handles this: `create` strategy skips existing files, `merge` checks for the `# Governance` marker before appending.
- **`.gitignore` merge dedup** — the command checks for the `# Governance` comment header before appending. If the marker exists, it skips the merge to avoid duplicating patterns.
- **Empty `$ARGUMENTS`** — no project name provided. The command prompts interactively for all required inputs.
- **Invalid project name** — uppercase, spaces, or special characters. The command rejects with a clear error: "Project name must be lowercase, alphanumeric, and hyphens only."
- **Command directory doesn't exist** — the CLI's command directory (e.g., `.claude/commands/`) may not exist yet. The command creates intermediate directories as needed.

## Acceptance Criteria

- [x] AC1: One `ductus.md` variant exists per supported CLI in the `ductus/` directory. Delivered as written against the per-CLI bootstrap files of the time. **Superseded twice over:** `012-multi-agent-govern` replaced the per-CLI variants with a single registry-driven installer, so there is no longer one file per CLI; and the repository was later reorganized, so the `ductus/` directory this criterion names does not exist — the installer is `framework/bootstrap/ductus.md`. Annotated rather than rewritten because the claim was superseded, not renamed: restating it as the single-file model would credit 007 with 012's work. `009`'s AC18 carries the same annotation for the same supersession
- [x] AC2: Running `curl` followed by `/ductus {name}` in an existing git repo produces a complete `ductus` scaffold
- [x] AC3: Each CLI variant scaffolds into its native directory paths (`.claude/` for Claude Code, `.augment/` for Auggie). **The requirement holds; the enumeration is superseded.** Native-path scaffolding is still exactly what the installer does, and `{cli-config-dir}` is still how it does it. But there are no longer "CLI variants" (012, per AC1), and the parenthetical names two agents where the registry in `framework/bootstrap/ductus.md` now carries four — `claude` → `.claude`, `auggie` → `.augment`, `antigravity` → `.agents`, `opencode` → `.opencode`, across three layout values. Left as written because the two it names are still correct and the criterion asserts no exclusivity; the registry is the live enumeration
- [x] AC4: Existing files (.gitignore, AGENTS.md, CLAUDE.md) are not overwritten
- [x] AC5: `.gitignore` merge is idempotent — running twice does not duplicate the managed patterns
- [x] AC6: Fetch failures for individual files do not abort the entire process
- [x] AC7: All generated files pass `npx markdownlint-cli2`
- [x] AC8: Slash commands are installed in the CLI's native command directory with `{project}` and `{cli-config-dir}` placeholders resolved
- [x] AC9: `specs/inbox.md` is created as the brownfield entry point
- [x] AC10: The command is idempotent — safe to run again without duplicating content
- [x] AC11: Post-scaffolding output displays brownfield-specific next steps
- [x] AC12: Invalid project names are rejected with a clear error message
- [x] AC13: Intermediate directories are created as needed
- [x] AC14: Adding a new CLI requires only a new ductus variant file — no changes to the framework core. Delivered as written, and the *intent* survives: adding an agent still leaves the constitution, templates and command sources untouched. **The mechanism is superseded.** 012 replaced the variant file with a row in the Agent Registry, so adding an agent means a registry row plus a walk of the §Derived values **Layout-derived** table — command path, invocation, install path, settings file, permission shape, native rule-loading dir, native rules file, cleanup glob. That is more than "only a new file", and `AGENTS.md` §Workflow records that nothing audits per-layout behaviour parity, so the walk is contributor discipline rather than a gate

## Open Questions

None — all resolved.

## Resolved Questions

- ~~Should the command also fetch `sdd-context.md`?~~ No — framework-internal only.
- ~~Should there be a `--dry-run` mode?~~ No — the command is idempotent with create/skip/merge strategies, making dry-run unnecessary.
- ~~How should the file manifest be maintained?~~ Hardcoded in each ductus variant. Updated when the framework templates change.
- ~~CLI-specific command variants or path variable?~~ Path variable (`{cli-config-dir}`) resolved at ductus time. One set of command templates, N ductus variants do the substitution.
- ~~How should `/ductus:setup` work for Auggie?~~ Skipped for now — Auggie permissions are global. Deferred to future considerations in `specs/spec.md`.

## References

Declared dependencies for this spec, surfaced here so the `derive-dependencies` runtime primitive sees them in the body.

- [003-bootstrap-automation](../003-bootstrap-automation/spec.md)
