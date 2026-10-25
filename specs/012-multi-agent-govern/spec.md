---
title: "012-multi-agent-govern — spec"
status: in-progress
dependencies: [007-govern-workflow]
tags: [bootstrap, agent]
next-criterion: 20
---

# 012 — Multi-Agent Ductus

A single bootstrap command, `framework/bootstrap/ductus.md`, that adopts `ductus` for multiple AI coding CLIs in the same project, with the target agent(s) selected at run time rather than baked into the file. Re-runs are additive — a project initialized for one agent can later adopt another by re-running `/ductus` and selecting the new agent.

## Problem

[007-govern-workflow](../007-govern-workflow/spec.md) shipped a separate bootstrap file per CLI — one for Claude Code, one for Auggie. The target CLI was implicit, determined by which file the user installed. This worked for single-agent projects but created two failure modes when a project used more than one agent:

1. **Half-installed projects** — when only one variant had been run, the second CLI had no commands and no settings. Adding the second agent required the user to know about the curl bootstrap and pick the right URL.
2. **Format leakage between agents** — the two variants were 95% identical, and divergent details (settings file format, permission command source) drifted unless the parity rule in `CLAUDE.md` was followed strictly. Drift had already produced a `permissions` key (Claude format) inside `.augment/settings.local.json`, which Auggie cannot parse.

The two-file model also made the parity rule itself a recurring maintenance cost: any change to shared logic had to be applied in both files.

## Distribution Model

The deliverable is a single file, `framework/bootstrap/ductus.md`. It is self-contained and supports every agent the framework knows about. Each supported agent contributes a small set of per-agent values consumed by the same shared logic.

### Initial install

The user installs the bootstrap into one agent with `install.sh`, a release asset that takes the agent as an optional argument and defaults to Claude Code (`003-bootstrap-automation`). After that first install, adding another agent needs no second install: the unified command installs itself into every selected agent's command directory during scaffolding.

### Supported agents

The set of agents the unified command supports is intrinsic to the file. Its **Agent Registry** has one row per agent and is the live list; the data model states the registry's schema. Adding an agent within an existing layout means contributing per-agent values to the unified command, never creating a new top-level bootstrap file.

`028-antigravity-agent` gave each row a `layout`, so an agent whose tree differs structurally from Claude's — Antigravity's `.agents/` skills, for one — can join the registry. An agent introducing a new layout also branches the scaffolding, permission-setup and bootstrap sections.

## Agent Selection

When `/ductus` runs, it determines which agents to scaffold using this order:

1. **Explicit `$ARGUMENTS`** — if `$ARGUMENTS` contains `--agents=...`, the listed agents are scaffolded and no prompt is shown.
2. **Auto-detect (default — routine update)** — if any supported agent's config directory exists in the project (e.g., `.augment/`), those agents are scaffolded silently with no prompt. This is the path that runs on every routine `/ductus` re-run.
3. **Add-agent mode** — if `$ARGUMENTS` contains `--add-agent`, or no agent dirs are detected (first run), a prompt is shown with detected agents pre-selected. The user picks the additional agents to adopt.

The user must end up with at least one selected agent. In add-agent or first-run mode, selecting zero is rejected with a clear message. The auto-detect path always has at least one agent (the dir the running `ductus.md` lives in is detected).

Removing an adopted agent's tree is not part of `/ductus`'s scope — see the **Re-Run Behavior** section.

## Per-Agent Scaffolding

For each selected agent, the unified command performs the same steps with per-agent values substituted, at paths derived from the agent's `layout` (**Derived values** in `framework/bootstrap/ductus.md`):

- Resolve `{cli-config-dir}` to the agent's config directory (e.g., `.claude`, `.augment`).
- Scaffold the project's slash commands into the agent's command location — `{cli-config-dir}/commands/{project}/` for a `claude-style` agent.
- Scaffold the `configure` command from the agent's permission source, `framework/bootstrap/configure/{key}.md`.
- Merge the registry's bootstrap permission entries into the agent's settings file, in the agent's native format, so `/ductus`'s own steps run without prompts.
- Install a copy of the unified bootstrap at the agent's install path — `{cli-config-dir}/commands/ductus.md` for a `claude-style` agent — so the command is invokable from that agent on subsequent runs.

Session state is not per agent: every adopter keeps it at `.ductus/session.toml`.

Per-agent native conventions are preserved:

- **Claude Code** uses `permissions.allow`/`permissions.deny` arrays, and reads `CLAUDE.md`, which is written from `framework/templates/project/claude-md.md` if missing.
- **Auggie** uses a `toolPermissions` array with `toolName`/`shellInputRegex` entries, and reads `CLAUDE.md` natively — no second rules file is needed.

## Shared Files

These files are scaffolded once per `/ductus` run regardless of how many agents are selected. The canonical list is the **Shared Files** manifest in `framework/bootstrap/ductus.md`.

- The constitution, installed at `.ductus/constitution.md`
- The rule files under `specs/rules/`
- `.markdownlint-cli2.jsonc`
- `AGENTS.md` (skip if exists)
- `CLAUDE.md` (skip if exists; written only when a `claude-style` agent is selected)
- `.gitignore` (merge — the framework's patterns live in a managed block opened by a `# ductus` line, rewritten in place on each run)
- `specs/system.md`, `specs/errors.md`, `specs/events.md`, `specs/inbox.md` (create if missing)
- `specs/templates/*.md` (update strategy)
- The pre-commit hooks under `.githooks/`

Shared files are unaffected by which agents are selected on re-runs.

## Re-Run Behavior

`/ductus` is idempotent and additive across agents:

- **Re-run with the same selection** — applies the manifest's `update` strategy to the agent's slash commands and refreshes shared files. Project-specific files (strategy `create`) are skipped if present.
- **Re-run adding a new agent** — scaffolds the new agent's tree from scratch alongside the existing one. The existing agent's command dir and settings are not touched.
- **Re-run removing an agent** — the unified command does not delete an agent's tree on its own. If the user wants to remove an adopted agent, that is a manual cleanup operation outside this command's scope.

## Migration from 007's Multi-File Model

Projects scaffolded under 007's per-CLI model moved to the unified command in three steps, in April 2026:

1. **Replace the installed bootstrap by hand.** A per-CLI file could not update itself to the unified one, because its own fetch manifest named its per-CLI source, which the unification removed from the repo. Users fetched the unified file over the installed one with a one-time `curl`.
2. **Run `/ductus`** in a new session, so the unified file loaded. Detection pre-selected whichever agent's directory existed, and the user could add agents in the same run. Scaffolding overwrote the project's per-agent permission command with the agent's own source (the manifest's `update` strategy).
3. **Run the permission command** in each adopted agent. For Auggie projects affected by the pre-`9eb9a2f` bug, its migration block stripped the legacy `permissions` key from `.augment/settings.local.json` and wrote proper `toolPermissions`.

No compatibility alias was left at the per-CLI source paths. The manual step was the migration path, and an alias would have reintroduced the very file the unification removed.

The migration is spent: nothing has scaffolded the per-CLI model since, and the bootstrap is now installed with `install.sh` (`003-bootstrap-automation`).

## Pre-flight Checks

Before any scaffolding, the unified command verifies:

- The current directory **is** an existing git repository. If not, stop and report: "This is not a git repository. Run `git init` first."
- If the spec-root directory (`specs/` by default) already exists, this is a re-run. Report: "Existing {spec-root}/ directory found — running in update mode."

These are the checks 007 describes; the unified command runs them once per run, whatever agents are selected.

## Edge Cases

- **Unknown agent in `--agents=`** — if `$ARGUMENTS` lists an agent key not present in the registry, stop before scaffolding and report the unknown key with the list of valid keys. Do not partially scaffold.
- **All supported agents already adopted with `--add-agent`** — show the prompt with all agents pre-selected; if the user confirms with no additions, treat it as a routine update and continue silently. Do not error.
- **The agent's settings file already has entries beyond the bootstrap** — the unified command adds the registry's bootstrap entries that are missing and, since 022's task 18, removes the entries an earlier seed carried and the current one replaces, listed by exact text in `framework/bootstrap/ductus.md` §Permission Setup. It must not overwrite, deduplicate, or reorder entries added by `/{project}:configure` or by the user.
- **`ductus.md` content already matches the unified version** — when the manifest's `update` strategy compares fetched content to the installed file, identical content reports as "unchanged" and avoids a redundant write. Same rule applies to the per-project `configure.md` and other update-strategy files.
- **Pinned `ductus.md` in `.ductus/config.toml`** — pinned files are skipped, including the installed bootstrap itself, so a pinned copy never updates until the pin is removed. Each run still compares it with upstream and prints one advisory line while the two differ (007's `ductus-self-update-precheck` scenario).
- **A source file missing from the framework archive** — report it and continue with the remaining files. Do not abort the entire scaffolding pass.
- **First-run prompt with no detected dirs and only one supported agent** — the prompt still appears (the agent must be explicitly chosen), but the single agent is pre-selected. Confirming is one keystroke.

## Cross-Spec Impact

[007-govern-workflow](../007-govern-workflow/spec.md) is superseded on the points where this spec inverted its decisions:

- 007 specified "one markdown file per supported CLI" and "the target CLI is implicit." This spec replaced both with a single file and explicit runtime selection.
- 007's resolved question "CLI-specific command variants or path variable?" remains correct — the per-agent values are still substituted at ductus time. Only the file count changed.

The signpost note added to 007 is delivered by this spec. 007 keeps its `done` status — its work shipped, and this spec carries forward the new design.

## Acceptance Criteria

- [x] AC1: A single `framework/bootstrap/ductus.md` replaces the two per-CLI bootstrap files, one for Claude Code and one for Auggie — only one bootstrap source exists. Delivered in the repository's original top-level bootstrap directory; the later reorganization moved it to `framework/bootstrap/`. `framework/bootstrap/govern.md` is not a second source: it is a byte-identical copy at the retired name, kept so an adopter whose installed bootstrap predates the rename still fetches the current file (`049-rename-govern-to-ductus`), and `/ductus:audit` Family 21 holds the two byte-identical.
- [x] AC2: Running `/ductus` in a project with at least one detected agent dir (`.claude/` or `.augment/`) updates the detected agents silently — no agent-selection prompt is shown (routine-update path)
- [x] AC3: Running `/ductus --add-agent` shows the agent-selection prompt with detected agents pre-selected, allowing the user to add additional agents
- [x] AC4: Running `/ductus` for the first time in a project with no detected agent dirs (e.g., immediately after the curl install) shows the agent-selection prompt with the agent whose dir holds the running `ductus.md` pre-selected
- [x] AC5: An explicit `--agents=...` list in `$ARGUMENTS` selects exactly those agents and bypasses the prompt
- [x] AC6: Selecting zero agents in any prompt path (add-agent or first-run) is rejected with a clear message
- [x] AC7: For each selected agent, scaffolding produces: project commands at `{cli-config-dir}/commands/{project}/`, the agent's permission source written as `configure.md`, an empty session JSON, a settings file in the agent's native format, and a copy of `ductus.md` at `{cli-config-dir}/commands/ductus.md` — **delivered as written; the session leg no longer holds.** The per-agent `{cli-config-dir}/{project}-session.json` was replaced by a single `.ductus/session.toml`, uniform across agents and written by `write-session` rather than created empty at scaffold time; `042-consolidate-govern-per-project-files-under-govern-directory` moved it to its current path. The other four legs still hold, with destinations now derived per `layout` (see Per-Agent Scaffolding).
- [x] AC8: Claude Code's `settings.local.json` uses `permissions.allow`/`permissions.deny`; Auggie's uses a `toolPermissions` array — neither agent receives the other's format
- [x] AC9: Re-running `/ductus` and adding a previously unselected agent scaffolds that agent's tree alongside the existing one without modifying the existing agent's command dir, settings, or session JSON. The session-JSON leg went with AC7's: there is no per-agent session file to modify.
- [x] AC10: Re-running `/ductus` with the same selection updates the agent's slash commands per the manifest's `update` strategy and leaves `create`-strategy files alone
- [x] AC11: Shared files (`constitution.md`, `specs/templates/*`, `.markdownlint-cli2.jsonc`, `AGENTS.md`, `CLAUDE.md`, `.gitignore`, `specs/system.md`, `specs/errors.md`, `specs/events.md`, `specs/inbox.md`) are scaffolded once per run and untouched by which agents are selected. The claim still holds; the list is 012's delivery snapshot and is **not** the canonical set — later specs added the rule files and the two hooks. The canonical list is the **Shared Files** manifest in `framework/bootstrap/ductus.md`.
- [x] AC12: Adding support for a new agent requires only adding the agent's per-agent values (config dir, settings format, configure source) inside the unified command — no new top-level ductus file is created
- [x] AC13: Existing projects scaffolded under 007's multi-file model migrate to the unified `/ductus` via the documented one-time curl, after which subsequent re-runs handle updates without further manual intervention (see [Migration from 007's Multi-File Model](#migration-from-007s-multi-file-model))
- [x] AC14: The README documents the install path for each supported agent and states that subsequent agents do not require a second curl
- [x] AC15: 007's spec gains a signpost note pointing to 012, marking the multi-file design as superseded
- [x] AC16: The `CLAUDE.md` "Ductus File Parity" rule is removed — there is only one ductus file
- [x] AC17: An unknown agent key in `--agents=` is rejected with a clear error before any scaffolding occurs
- [x] AC18: `settings.local.json` entries added by `/{project}:configure` or the user are preserved across `/ductus` re-runs — only the registry's bootstrap entries are added if missing
- [x] AC19: All shipped markdown files pass `npx markdownlint-cli2`

## Open Questions

None — all resolved.

## Resolved Questions

- **Agent selection UI** — sequential per-agent yes/no questions via `AskUserQuestion`, one question per supported agent. Each pre-selects "Yes" when the agent's config dir is detected. Universally supported in both CLIs, scales to N agents, and avoids per-CLI rendering branches. An explicit `--agents=` list in `$ARGUMENTS` skips the prompt entirely.
- **Per-agent values: inline conditionals vs. fragments** — inline, structured as an "agent registry" table near the top of `ductus.md`. The rest of the file references registry values (config dir, settings format, permission source, rules-file note) rather than branching on agent name. Adding a new agent is a one-row addition to the registry. Fragments were rejected because they reintroduce multi-file maintenance, fetch-failure modes, and a bootstrap question — the exact burdens this spec exists to remove.
- **Configure source layout** — a per-agent directory, `framework/bootstrap/configure/{key}.md` (e.g., `framework/bootstrap/configure/claude.md`, `framework/bootstrap/configure/auggie.md`). The source path becomes derivable from the agent key, removing the asymmetric per-agent filenames of the two-file model; moving there was a pair of file moves, and the only consumer is the bootstrap itself. The agent registry doesn't need an explicit source column — the path is derived by convention.
- **Bootstrap install path** — keep one curl snippet per supported agent in the README, with a short framing line that the multi-agent flow is available on re-run. A canonical single-path install would tax every single-agent user (the common case) to simplify the rare multi-agent bootstrap; the framing line addresses discoverability without that tax. — **This decision was reversed, and the premise it rested on did not hold.** `003-bootstrap-automation` shipped `install.sh` on 2026-06-10, six weeks after 012 landed: one canonical snippet, `curl … | sh -s -- <agent>`, with the agent argument **optional** and defaulting to `claude`. The single-agent user is therefore not taxed — the common case is the shortest command — which is the cost this entry priced the option out on. The README keeps a per-agent table (argument, destination, MCP note), so the discoverability the framing line bought survives. Recorded rather than deleted: an adopted option still filed as rejected argues against the design that won.
- **Add-an-agent contract** — five fields per agent: `key`, `name`, `config_dir`, `settings_template`, `rules_file_note`; `028-antigravity-agent` later added a sixth, `layout`. Everything else per agent — its permission source path and its command and install paths — is derived from these. `settings_template` stores the **fully populated** bootstrap JSON in each agent's native syntax — Claude uses `Bash(...)` allow strings under `permissions.allow`, Auggie uses `launch-process` entries with `shellInputRegex` under `toolPermissions`. The registry seeds only the entries `/ductus` itself needs; the full permission set lives in `framework/bootstrap/configure/{key}.md` and is applied later via `/{project}:configure`. Adding an agent was three edits when this was decided — registry row, permission source, README curl snippet. The current list is **Adding a new agent** in `framework/bootstrap/ductus.md`, which adds an MCP registration row and an `install.sh` arm, and turns the README edit into an install-table row; an agent introducing a new `layout` costs more again.
- **Stale per-CLI ductus files on migration** — replacing the installed `ductus.md` is sufficient; no special detection or cleanup logic. The manifest's `update` strategy covers `ductus.md` and the per-project permission command, which is enough to overwrite all on-disk legacy artifacts. No version stamping or sentinel comments. Migration was a manual curl, then `/ductus`, then the permission command in each agent (see "Migration from 007's Multi-File Model" section). Pinning `ductus.md` in `.ductus/config.toml` was documented as a footgun — pinned files would not migrate, and that was acceptable.
- **First-run default agent selection** — dissolved by the revised agent-selection rules. Routine `/ductus` runs are silent (auto-detect path), so first-run is the only case that prompts. On first run, detection has nothing to lock onto except the agent whose dir holds the running `ductus.md` — that agent is pre-selected. No separate "no detection" branch is needed.
- **Routine update vs. agent selection — single command or two?** — single command, with default behavior driven by detection. `/ductus` with no flags performs a silent update of detected agents (the common case). `/ductus --add-agent` triggers the agent-selection prompt with detected agents pre-selected. `/ductus --agents=...` bypasses the prompt entirely. A separate `/ductus-add-agent` command was rejected because it adds a second entry point to discover and a second file to scaffold into every agent's commands dir, with no upside the flag doesn't already provide.
- **Deselecting an adopted agent** — stays manual. `/ductus` is purely additive/idempotent; it never deletes an adopted agent's tree. `--agents=` has scope-limiting effect (scaffolds only the listed agents), not removal effect — agents not in the list are left untouched. Removing an adopted agent is a manual `rm -rf {config_dir}` operation outside `/ductus`'s scope, justified by the destructive nature of the action and the rarity of the use case.
