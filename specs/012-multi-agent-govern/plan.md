---
title: "012-multi-agent-govern — plan"
---

# 012 — Multi-Agent Ductus Plan

## Overview

Replace the two per-CLI ductus files (one for Claude Code, one for Auggie) with a single `framework/bootstrap/ductus.md` that carries an agent registry. The registry is a small markdown table near the top of the file listing each supported agent and the per-agent values needed to scaffold it (`key`, `name`, `config_dir`, `layout`, `settings_template`, `rules_file_note` — `layout` added by `028-antigravity-agent`). All other sections (pre-flight, manifest, scaffolding loop, post-scaffolding output) reference the registry rather than branching on agent name.

`/ductus` resolves which agents to scaffold using a fixed precedence: `--agents=` flag, then auto-detect (silent routine update), then a sequential `AskUserQuestion` prompt (one yes/no per agent). The scaffolding loop runs once per selected agent, writing the agent's commands, settings, configure command (from `framework/bootstrap/configure/{key}.md`), and a copy of `ductus.md` into the agent's command directory.

The configure sources are per-agent files, `framework/bootstrap/configure/{key}.md`, so each source path is derivable from the registry key.

The `CLAUDE.md` "Ductus File Parity" rule is deleted (only one ductus file exists), and `specs/007-govern-workflow/spec.md` gains a signpost note pointing to this spec.

## Technical Decisions

### Agent registry as a markdown table inside `ductus.md`

The registry lives near the top of `framework/bootstrap/ductus.md` as a markdown table — readable from the file itself, no external lookup, easy to extend. The agent reads the table and iterates the rows during scaffolding. This is the same pattern 007 uses for its file manifest.

Schema (six fields per row; `028-antigravity-agent` added `layout`):

| Field | Description |
| --- | --- |
| `key` | Lowercase identifier used in flags (`--agents=claude`), file paths (`framework/bootstrap/configure/{key}.md`), and prompt text |
| `name` | Human-readable label shown in prompts and post-scaffolding summary |
| `config_dir` | Project-relative config directory (e.g., `.claude`, `.augment`) |
| `layout` | The agent's directory and command shape, selecting its layout-derived paths |
| `settings_template` | Bootstrap JSON for the agent's settings file in its native format — only the entries `/ductus` itself needs, not the full permission set |
| `rules_file_note` | Short note about the agent's relationship to its native rules file, shown in the post-scaffolding summary |

Derived values (not stored — computed by convention):

- Configure source path: `framework/bootstrap/configure/{key}.md`
- Project commands directory: `{config_dir}/commands/{project}/` for a `claude-style` agent
- Ductus install path: `{config_dir}/commands/ductus.md` for a `claude-style` agent

Every other layout derives its own paths, listed in **Derived values** in `framework/bootstrap/ductus.md`.

Rejected alternatives:

- **Inline conditionals on agent name** — would scatter `if claude / else auggie` branches throughout the file. Rejected because adding a new agent would touch every branch. The registry centralizes per-agent values into a single addressable structure.
- **External fragments fetched per agent** — would reintroduce multi-file fetch failure modes and a bootstrap question. The whole point of unification is to avoid those.

### Configure source layout: `framework/bootstrap/configure/{key}.md`

Each agent's configure source lives at `framework/bootstrap/configure/{key}.md`, derivable from the registry key, so the registry doesn't need an explicit source field. Each uses `{cli-config-dir}` for paths and contains agent-native permission entries.

This repository's own `.claude/commands/ductus/configure.md` is generated from `framework/bootstrap/configure/claude.md` by `scripts/gen-claude-commands.sh`.

### Agent selection precedence

`/ductus` determines selection in this order, stopping at the first match:

1. **Explicit list** — `$ARGUMENTS` contains `--agents=claude,auggie`. Use exactly those keys, no prompt. Reject unknown keys before any scaffolding.
2. **Auto-detect (no `--add-agent`)** — list registry entries whose `config_dir` exists in the project. If at least one is found, scaffold those silently (routine-update path). This is the common case on re-runs.
3. **Add-agent prompt** — triggered when `$ARGUMENTS` contains `--add-agent`, OR when no detected dirs exist (first run). Iterate the registry and ask one yes/no `AskUserQuestion` per agent. Pre-select "Yes" when the agent's `config_dir` is detected, OR when this is first run AND the agent's key matches the dir holding the running `ductus.md` (detected via the running command's install path).

Rationale: routine `/ductus` re-runs (the common case) must be silent — adding a prompt every run would tax users for the rare multi-agent bootstrap. Sequential per-agent yes/no questions are universally supported in both CLIs and scale to N agents without per-CLI rendering branches.

If the user selects zero agents in any prompt path, reject with: "At least one agent must be selected." Do not partially scaffold.

### First-run "current agent" detection

On first run with no detected `config_dir` (no `.claude/`, no `.augment/`), the prompt still appears so the user explicitly chooses. To pre-select the right agent, the running `ductus.md` infers its own location: it is invoked from `{config_dir}/commands/ductus.md`, so the agent whose `config_dir` matches the running file's parent path is the one to pre-select.

If the agent cannot infer its own path (CLI doesn't expose it), the prompt iterates with no pre-selection — the user picks explicitly. This is acceptable on first run because the user just curled the file and knows which agent they're in.

### Per-agent scaffolding loop

For each selected agent, run these steps with registry values substituted:

1. Resolve `{cli-config-dir}` to the agent's `config_dir`.
2. Process the slash-command manifest into `{config_dir}/commands/{project}/`. Substitute `{project}` and `{cli-config-dir}` in each copied file.
3. Fetch `framework/bootstrap/configure/{key}.md` from the `ductus` repo and write it as `{config_dir}/commands/{project}/configure.md` (strategy: `update`).
4. Read or create `{config_dir}/settings.local.json` and merge in entries from the agent's `settings_template` if missing. Do not reorder, deduplicate, or overwrite entries beyond the bootstrap set.
5. Fetch `framework/bootstrap/ductus.md` and write it to `{config_dir}/commands/ductus.md` (strategy: `update`). Run the post-write integrity check: the file must open with a frontmatter block carrying `description:`, and the body after it must start with `# ductus`. If not, report the error and re-read the source.

Steps 2 and 5 name the `claude-style` paths; other layouts write to their own.

Shared files (the constitution, the rule files, `.markdownlint-cli2.jsonc`, `AGENTS.md`, `CLAUDE.md`, `.gitignore`, `specs/system.md`, `specs/errors.md`, `specs/events.md`, `specs/inbox.md`, `specs/templates/*`, and the pre-commit hooks) are scaffolded **once per run** outside the loop, regardless of how many agents are selected.

### `settings_template` content

The registry's `settings_template` is the **fully populated** bootstrap JSON in each agent's native syntax, holding only the bootstrap entries: enough for every command `/ductus` itself runs — fetching and extracting the framework, the git queries, acquiring the runtime — to run without permission prompts. Claude Code takes `Bash(...)` strings under `permissions.allow`; Auggie takes `launch-process` entries with a `shellInputRegex` under `toolPermissions`. The registry rows in `framework/bootstrap/ductus.md` carry the exact entries. The full permission set lives in `framework/bootstrap/configure/{key}.md` and is applied later via `/{project}:configure`.

Merge rules: read existing file, append any missing entries from `settings_template`, write back. Preserve all entries the user or `/{project}:configure` has already added — only the bootstrap set is touched.

### Migration: 007's per-CLI files → unified file

This migration ran in April 2026 and is spent. No backward-compatibility alias was left at the per-CLI source paths. Migration was three manual steps, recorded in the spec's "Migration from 007's Multi-File Model" section:

1. Curl the unified `ductus.md` into the agent's command dir, overwriting the old per-CLI file.
2. Run `/ductus` in a new session — detection pre-selects the agent whose dir exists.
3. Run `/{project}:configure`.

A backward-compat alias was rejected because it would reintroduce the file the unification is meant to remove.

### "Ductus File Parity" rule removed; "Command File Parity" rule kept

The CLAUDE.md "Ductus File Parity" rule disappears — there is only one ductus file. The "Command File Parity" rule (between `framework/commands/` and `.claude/commands/ductus/`) is unrelated and stays in place.

### 007's signpost note

`specs/007-govern-workflow/spec.md` already has a signpost from 011 at the top. Add a second signpost paragraph below it pointing to 012, marking the multi-file design as superseded. 007 keeps its `done` status — its work shipped, and 012 carries forward the new design.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/bootstrap/ductus.md` | Modify | Replace with unified content: agent registry table, selection logic, per-agent scaffolding loop |
| Auggie's per-CLI bootstrap file | Delete | Superseded by unified `ductus.md` |
| `framework/bootstrap/configure/claude.md` | Create | Claude Code's configure source |
| `framework/bootstrap/configure/auggie.md` | Create | Auggie's configure source |
| `commands/setup.md` | Delete | Replaced by `framework/bootstrap/configure/claude.md` |
| `commands/setup-auggie.md` | Delete | Replaced by `framework/bootstrap/configure/auggie.md` |
| `.claude/commands/ductus/configure.md` | Modify | Generated from `framework/bootstrap/configure/claude.md` (Command File Parity) |
| `README.md` | Modify | Add framing line under each curl snippet noting the multi-agent flow is available on re-run |
| `CLAUDE.md` | Modify | Remove the "Ductus File Parity" section |
| `specs/007-govern-workflow/spec.md` | Modify | Add signpost paragraph pointing to 012 |
| `specs/README.md` | Modify | Refresh the "Auggie permissions setup" resolved-note to reference `framework/bootstrap/configure/auggie.md` and the unified `ductus.md` |
| `specs/012-multi-agent-govern/data-model.md` | Create | Document the agent registry schema |
| `specs/012-multi-agent-govern/spec.md` | Modify | Status to `done` after acceptance criteria are verified (final task) |

## Trade-offs

### Considered: keep two ductus files, just deduplicate via shared fragments

Each per-CLI file would `curl` shared fragments at ductus time. Rejected — fragments reintroduce a fetch-failure mode (the bootstrap pulls more files), make the system harder to reason about offline, and re-create the parity burden the spec is trying to eliminate. The registry approach keeps one file with the per-agent surface area boxed into a single table.

### Considered: separate `/ductus-add-agent` command

A second slash command for the multi-agent path. Rejected because it doubles the discovery surface (two commands to remember, two files to scaffold into every project), with no upside the `--add-agent` flag doesn't already provide.

### Considered: detection-only auto-add of new agents

If a new agent's `config_dir` is detected on a routine re-run, scaffold it silently. Rejected — adopting a new agent should be an explicit user decision, not a side effect of a directory existing. The `--add-agent` flag is the explicit path.

### Considered: removal of an adopted agent via `/ductus`

Add a `--remove=auggie` flag that deletes `.augment/` and its scaffolding. Rejected — destructive, rarely needed, and the user can `rm -rf .augment/` themselves. `/ductus` stays purely additive/idempotent.

### Known limitation: pinned `ductus.md` blocks migration

A project that listed its installed bootstrap in `.ductus/config.toml` `pinned.files` did not migrate to the unified file via `/ductus` — the pin overrides the `update` strategy. It was documented as a footgun, not solved programmatically; removing the pin is a one-line edit. With the migration spent, a pin now just holds the installed bootstrap at its pinned version, and each run prints an advisory while it differs from upstream.

### Known limitation: agent self-path inference

The "first-run pre-select the agent whose dir holds the running `ductus.md`" logic depends on the running CLI exposing the running command's install path to the prompt. If a future CLI doesn't, the prompt falls back to no pre-selection. The user types one extra keystroke; nothing breaks.

## Open Questions Resolved

All eight open questions are resolved in the spec's "Resolved Questions" section. No new questions surfaced during planning.

- **Agent selection UI** — sequential per-agent `AskUserQuestion` yes/no, with `--agents=` bypass.
- **Per-agent values: inline conditionals vs. fragments** — registry table inside `ductus.md`.
- **Configure source layout** — `framework/bootstrap/configure/{key}.md`, derived from registry key.
- **Bootstrap install path** — keep one curl per supported agent in the README, with a framing line about the multi-agent flow on re-run. Reversed when `003-bootstrap-automation` shipped `install.sh`; the spec's resolved question records why.
- **Add-an-agent contract** — five fields per agent (`key`, `name`, `config_dir`, `settings_template`, `rules_file_note`), with `layout` added later by `028-antigravity-agent`; other paths derived by convention.
- **Stale per-CLI ductus files on migration** — manifest's `update` strategy overwrites the installed `ductus.md`; no special detection.
- **First-run default agent selection** — pre-select the agent whose `config_dir` holds the running `ductus.md`.
- **Routine update vs. agent selection — single command or two?** — single `/ductus`, with `--add-agent` triggering the prompt and `--agents=` bypassing it.
- **Deselecting an adopted agent** — manual `rm -rf {config_dir}` outside `/ductus`'s scope.
