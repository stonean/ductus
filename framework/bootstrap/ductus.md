---
description: Adopt or update ductus in an existing project.
argument-hint: "[project] [--agents=key1,key2,...] [--add-agent] [--ref=latest|main|ductus-vX.Y.Z]"
parity:
  strict-files:
    - "{cli-config-dir}/commands/ductus.md"
    - "{cli-config-dir}/commands/{project}/specify.md"
    - "{cli-config-dir}/commands/{project}/clarify.md"
    - "AGENTS.md"
  semantic-fields:
    - completion-message
---

# ductus

Bootstrap `ductus` in an existing project. This command fetches templates from the `ductus` repo, scaffolds `ductus` files for one or more AI coding CLIs, resolves placeholders, and displays next steps.

The same `ductus.md` supports every agent the framework knows about. The set of supported agents lives in the **Agent Registry** below; per-agent values are looked up by registry key during scaffolding.

## Instructions

> **For agent runtimes**: backticked primitive names in this section (`fetch-archive`, `extract-archive`, `apply-manifest`, `merge-managed-block`, `enforce-manifest`) map to MCP tools the [ductus runtime](https://crates.io/crates/ductus) exposes under bare `<primitive>` names (e.g., `fetch-archive`). Hosts wrap them with a server-name prefix taken from the agent's MCP registration (Claude: `mcp__ductus__fetch-archive`; Auggie and Antigravity: `mcp:ductus:fetch-archive`; OpenCode: a `ductus*`-prefixed `<server>_<tool>` name; Pi: a `ductus__<name>`-prefixed extension-bridge tool). Match the **prefix** rather than a spelling — the separator is a host detail, and `framework/constitution.md` §runtime-host-integration is the canonical mapping this restates. When the server is registered for your session, **call the corresponding tool** for each step listed below — that is the deterministic path. When it is not registered, walk the markdown-only reference below (`tar -xzf`, `curl`, etc.) to produce the same result. The two paths share a contract; neither one wraps the other.

**Where a named section lives.** This file is the half of the procedure that runs before the framework archive exists. A **bolded section name** or `§name` below that has no heading in this file is in the archive half, `framework/bootstrap/ductus-procedure.md`, read from the extracted tree — §The archive half lists all nine and gives the path. Nothing in this file dispatches from there; the numbered walker below is the whole execution order.

**Procedural fidelity.** Execute the steps below as written. The only confirmation prompts to issue are those the procedure specifies: project inputs (§Inputs), agent-selection prompts on `--add-agent` / first-run (§Agent Selection), and the registry-driven migration prompts (§Pre-run Migrations — outer "apply N pending migrations" prompt plus any per-entry inner prompts the procedure files specify). Do not stop to warn about uncommitted edits to update-strategy files, custom slash commands that **Slash command cleanup** is about to remove, or "data loss" from the stale → write-and-abort path. The procedure already encodes safety: `.ductus/config.toml` `[pinned] files` is the opt-out, the stale path writes upstream and aborts cleanly (recoverable from git), and slash-command cleanup is unconditional for unpinned files. Extra prompts duplicate information the procedure already gives the user and stall routine runs.

1. The walker context carries the inputs the host has already gathered and validated: project (the destination project name), description (one-line project description), languages (comma-separated), agents (registry keys), framework-version (release tag), archive-url and sha256-url (computed from framework-version), staging-dir, substitutions-map, manifest-entries (the per-strategy list described in **Shared Files** and **Per-Agent Scaffolding**), pinned-list (from `.ductus/config.toml`'s `[pinned] files` block), gitignore-block (the `.claude/`, `.augment/`, `.agents/`, `.opencode/`, `.pi/`, `specs/.cache/`, etc. lines), host-block (the `project` value — the team-shared slash-command namespace — written to committed `.ductus/config.toml`, plus the per-contributor `cli-config-dir` written to the gitignored `.ductus/session.toml` since teammates may use different agents; the runtime reads all three at `ductus exec` time to resolve the installed command file — `{cli-config-dir}/commands/{project}/<name>.md` for `claude-style`, then `{cli-config-dir}/command/{project}/<name>.md` for `opencode`, then `{cli-config-dir}/prompts/{project}-<name>.md` for `pi` (§Derived values **Command/skill path**; the runtime tries all three, so any layout resolves)), enforce-directories (the slash-command directories whose top-level `*.md` files are pruned to the manifest), and the per-agent ductus-install entry with `keep-literals: ["project", "cli-config-dir"]`. The host runs the markdown-only reference below to collect inputs, derive registry values, validate `.ductus/config.toml`, and seed context; the runtime walks the procedure that follows.

2. Invoke `fetch-archive` (MCP: `fetch-archive`) to download the framework tarball. The primitive verifies the sha256 against a sidecar URL when one is supplied; without a sidecar (the framework archive's case on every source, since GitHub's auto-generated source tarballs ship without sidecars) it returns the computed digest and `verified: false`, leaving any out-of-band verification to the host. A sidecar mismatch halts the procedure with an `error` envelope so no partial state lands in the destination tree.

3. Invoke `extract-archive` (MCP: `extract-archive`) to expand the verified tarball into the staging directory. Path-traversal protection is applied per entry; symlinks are skipped. Otherwise, follow the markdown-only path's `tar -xzf` workflow.

4. Invoke `apply-manifest` (MCP: `apply-manifest`) with the host-built manifest entries and the pinned list. The primitive walks each entry, applies the per-entry strategy (update for framework-owned files, create for adopter-seedable files, skip-if-conflict for adopter-owned templates — the three strategy values defined in **Shared Files** below), short-circuits on the pinned list, returns aggregate counts the host surfaces in the completion message. This single call replaces the per-file update / create / skip loops the markdown-only reference describes below.

5. Invoke `merge-managed-block` (MCP: `merge-managed-block`) against `.gitignore` with `marker-style: "line-prefix"` and `marker: "ductus"` to install or update the framework-managed block (the `.claude/`, `.augment/`, `.agents/`, `.opencode/`, `.pi/`, `specs/.cache/`, etc. lines). First-run creates the file; subsequent runs update only the region between the `# ductus` preamble line and the next blank line, preserving the rest of the file byte-for-byte. Replaces the inline `grep` check the markdown-only reference describes for the `.gitignore` merge step.

6. Establish the team-shared host configuration. Invoke `merge-managed-block` (MCP: `merge-managed-block`) against the **active config file** (write policy: §Project Configuration) with `marker-style: "line-prefix"`, `marker: "ductus (host)"`, and a block carrying **only** the resolved `project` value (the team-shared slash-command namespace). First-run creates the file with just the managed block; subsequent runs update only the region between the `# ductus (host)` preamble line and the next blank line, preserving every other config section (`[pinned]`, `[migrations]`, `[review]`) byte-for-byte — and dropping any legacy `cli-config-dir` key a prior version wrote into the managed block. On the markdown-only path, the host writes the `[host]` block to the same active file with its file-writing tool. See §Project Configuration for the `[host]` schema.

7. Record the per-contributor agent identity. Invoke `write-session` (MCP: `write-session`) with `cli-config-dir` set to the agent's resolved config-dir and **no** target fields — a host-config write that stores the agent identity in the gitignored `.ductus/session.toml` (preserving any existing target), never in committed config, because teammates on one project may each use a different agent. The runtime reads `project` from `.ductus/config.toml` and `cli-config-dir` from the session file at `ductus exec` time to resolve the installed command file — `{cli-config-dir}/commands/{project}/<name>.md` for `claude-style`, then `{cli-config-dir}/command/{project}/<name>.md` for `opencode`, then `{cli-config-dir}/prompts/{project}-<name>.md` for `pi` (§Derived values **Command/skill path**; the runtime tries all three, so any layout resolves); absent all three, it falls back to `.claude` / repo directory basename — fine for the framework's own repo, broken for any adopter whose layout doesn't match the defaults. On the markdown-only path, the host writes the session-file `cli-config-dir` key with its file-writing tool.

8. Invoke `enforce-manifest` (MCP: `enforce-manifest`) once per directory in the host's enforce-directories list (typically the per-agent slash-command directory). The primitive removes files matching the glob-include arg (default `*.md`) whose relative path is neither in the expected list nor pinned. One call replaces the slash-command manifest enforcement loop the markdown-only reference describes. Adopter cleanup of historical conventions is owned by the **Pre-run Migrations** section (in the archive half) and the `framework/migrations.toml` registry it drives.

9. Invoke `apply-manifest` (MCP: `apply-manifest`) a second time with a single entry for the per-agent `ductus` self-install (the agent's **`ductus` install path** from §Derived values — `{cli-config-dir}/commands/ductus.md` for `claude-style`, `{cli-config-dir}/command/ductus.md` for `opencode`, `{cli-config-dir}/skills/ductus/SKILL.md` for `antigravity`, as §`ductus` self-installation specifies) and an **empty substitutions map** (`{}`). `ductus.md`'s body contains prose references to every placeholder name the bulk step substitutes — `{project}`, `{cli-config-dir}`, `{project-name}`, `{One-line project description.}` — describing what those placeholders mean in *other* files. None of them are values to substitute in `ductus.md` itself, so the self-install call passes no substitutions rather than relying on `keep-literals` to mask individual keys from the full map. The split from step 4 isolates the no-substitute concern from the bulk substitute step.

10. Render the completion message (host responsibility): list the agents configured, the next pipeline command — `specify`, written in **each agent's derived Invocation form** (§Derived values), not the `claude-style` colon form for everyone — the acquired runtime's store path, and any per-agent post-install reminders from the registry rows above.

## Agent Registry

The registry lists every supported agent. Per-agent paths and behaviors are derived from these rows — the rest of this file references registry values, not agent names.

| `key` | `name` | `config_dir` | `layout` | `settings_template` | `rules_file_note` |
| --- | --- | --- | --- | --- | --- |
| `claude` | Claude Code | `.claude` | `claude-style` | `{ "permissions": { "allow": ["Bash(curl *)", "Bash(ls *)", "Bash(tar *)", "Bash(mktemp *)", "Bash(git status *)", "Bash(git config *)", "Bash(git rev-parse *)", "Bash(git diff *)", "Bash(git ls-files *)", "Bash(chmod *)", "Bash(awk *)", "Bash(command -v *)", "Bash(mkdir *)", "Bash(shasum *)", "Bash(sha256sum *)", "Bash(certutil *)", "Bash(ln *)", "Bash(cp *)", "Bash(~/.ductus/bin/ductus *)", "Bash(.ductus/bin/ductus *)", "Read(/private/var/folders/**/T/ductus-*/**)", "Read(//private/var/folders/**/T/ductus-*/**)", "Read(/var/folders/**/T/ductus-*/**)", "Read(//var/folders/**/T/ductus-*/**)", "Read(/tmp/ductus-*/**)", "Read(//tmp/ductus-*/**)"], "deny": [] } }` | Claude Code reads `CLAUDE.md` natively. |
| `auggie` | Auggie | `.augment` | `claude-style` | `{ "toolPermissions": [ { "toolName": "launch-process", "shellInputRegex": "^curl ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^ls ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^tar ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^mktemp ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^git status ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^git config ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^git rev-parse ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^git diff ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^git ls-files ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^chmod ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^awk ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^command -v ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^mkdir ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^shasum ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^sha256sum ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^certutil ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^ln ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^cp ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^[^ ]*\\.ductus/bin/ductus ", "permission": { "type": "allow" } } ] }` | Auggie reads `CLAUDE.md` natively — no second rules file is needed. |
| `antigravity` | Antigravity | `.agents` | `antigravity` | `{ "permissions": { "allow": [ "command(curl)", "command(ls)", "command(tar)", "command(mktemp)", "command(git status)", "command(git config)", "command(git rev-parse)", "command(git diff)", "command(git ls-files)", "command(chmod)", "command(awk)", "command(which)", "command(mkdir)", "command(shasum)", "command(sha256sum)", "command(certutil)", "command(ln)", "command(cp)" ], "deny": [], "ask": [] } }` | Antigravity reads `AGENTS.md` natively — no second rules file is needed. |
| `opencode` | OpenCode | `.opencode` | `opencode` | `{ "$schema": "https://opencode.ai/config.json", "permission": { "bash": { "curl *": "allow", "ls *": "allow", "tar *": "allow", "mktemp *": "allow", "git status *": "allow", "git config *": "allow", "git rev-parse *": "allow", "git diff *": "allow", "git ls-files *": "allow", "chmod *": "allow", "awk *": "allow", "command -v *": "allow", "mkdir *": "allow", "shasum *": "allow", "sha256sum *": "allow", "certutil *": "allow", "ln *": "allow", "cp *": "allow", "*/.ductus/bin/ductus *": "allow" } } }` | OpenCode reads `AGENTS.md` natively — no second rules file is needed. |
| `pi` | Pi | `.pi` | `pi` | `{}` | Pi reads `AGENTS.md` natively — no second rules file is needed. (Pi has **no** permission-gating settings; the `settings_template` is empty by design, and §Permission Setup is a no-op for Pi — see the derived `pi` column below.) |

### Derived values

For each agent, these paths and behaviors are computed by convention from its row — they are **not** stored in the table. Values that are the same for every agent are layout-independent; the rest are selected by the row's `layout` field.

**Layout-independent (every agent):**

| Derived value | Formula |
| --- | --- |
| Configure source path | `framework/bootstrap/configure/{key}.md` |

**Layout-derived (selected by `layout`):**

| Derived value | `claude-style` | `antigravity` | `opencode` | `pi` |
| --- | --- | --- | --- | --- |
| Command/skill path | `{config_dir}/commands/{project}/<name>.md` | `{config_dir}/skills/{project}-<name>/SKILL.md` | `{config_dir}/command/{project}/<name>.md` | `{config_dir}/prompts/{project}-<name>.md` |
| Invocation | `/{project}:<name>` | `/{project}-<name>` | `/{project}/<name>` | `/{project}-<name>` |
| `ductus` install path | `{config_dir}/commands/ductus.md` | `{config_dir}/skills/ductus/SKILL.md` | `{config_dir}/command/ductus.md` | `{config_dir}/prompts/ductus.md` |
| Settings file | `{config_dir}/settings.local.json` | `{config_dir}/settings.json` | `opencode.json` (repo root; same file as MCP wiring) | `{config_dir}/settings.json` (no permission-gating surface — Permission Setup is a documented no-op) |
| Permission shape | `permissions.allow/deny` (Claude) / `toolPermissions[]` (Auggie) | `permissions.allow/deny/ask` (action grammar) | `permission` action map (`allow`/`ask`/`deny`) | none (Pi displays no host permission prompt; trust is the only gate) |
| Native rule-loading dir | — (rules read from shared `specs/rules/`) | `{config_dir}/rules/<name>.md` | — (rules read from shared `specs/rules/`) | — (rules read from shared `specs/rules/`) |
| Native rules file | `CLAUDE.md` | `AGENTS.md` | `AGENTS.md` | `AGENTS.md` |
| Slash-command cleanup glob | `*.md` in the commands dir | `{project}-*/` skill dirs in `skills/` | `*.md` in `command/{project}/` | `{project}-*.md` in `prompts/` (never the default `*.md` — the flat directory is shared with the adopter's own prompt templates) |

The session state file is `.ductus/session.toml` for every adopter — not a derived per-agent path (the path is uniform across agents). It's gitignored, and it additionally records the per-contributor `cli-config-dir` (see §Session state).

### MCP registration (per-agent)

MCP discovery is **not** layout-derived — it is a per-agent property. A host can share Claude's command/skill layout and native `CLAUDE.md` reading (Auggie does) yet register MCP servers somewhere entirely different. Each agent therefore declares its own MCP registration descriptor; the State-B auto-wire (§ductus runtime detection) and §MCP wiring branch on the `mechanism` column.

| `key` | MCP target | scope | mechanism | surfaced instruction (when `surface-instruction`) |
| --- | --- | --- | --- | --- |
| `claude` | `.mcp.json` (repo root) | `project-committed` | `write-file` | — |
| `auggie` | `~/.augment/settings.json` | `user-global` | `surface-instruction` | `auggie mcp add ductus --command ~/.ductus/bin/ductus --args "mcp"` |
| `antigravity` | `~/.gemini/config/mcp_config.json` | `home-level` | `surface-instruction` | edit `~/.gemini/config/mcp_config.json`, then `/mcp` reload |
| `opencode` | `opencode.json` (repo root) `mcp` block | `project-committed` | `write-file` | — |
| `pi` | `.pi/extensions/ductus.ts` (the bridge from the staging archive) | `project-local` (gitignored; re-scaffolded by every `/ductus`, repairable by `/{project}:configure`) | `write-file` | — |

- **`write-file`** — ductus writes `target` additively at State-B wire time (the additive merge in §MCP wiring). Only `project-committed` agents use it.
- **`surface-instruction`** — ductus writes **no** MCP file; State B surfaces the instruction in the **Closing restart** and the user runs it once per machine, then restarts. Required for `user-global` / `home-level` agents, whose MCP config lives outside the repo and which ductus must not silently mutate.
- **Antigravity** loads MCP servers only from home-level `~/.gemini/config/mcp_config.json`; project-local `.agents/mcp_config.json` is **ignored** (verified against the live `agy` CLI). There is no scriptable `agy mcp add`, so registration is a config-file edit plus a `/mcp` reload.
- **Pi** has **no built-in MCP client** (verified against the shipped pi install, spec 064 §Verified Pi Layout). Its `mechanism` is `write-file` because the State-B operation is identical — write a file additively at wiring time — but the file is a TypeScript **extension bridge** (`.pi/extensions/ductus.ts`) rather than an MCP config: the bridge spawns the runtime's own MCP server over stdio and re-registers every tool under `ductus__<name>` (spec 064 D3). The write is to the project-local, gitignored `.pi/` directory, so the scope is `project-local` rather than `project-committed`: each contributor's bridge is re-scaffolded by every `/ductus` run and repaired by `/{project}:configure`. The tools appear after the pi process **restarts** (project-local extensions load at session start, trust-gated) — the same restart cadence as the MCP hosts, surfaced in the **Closing restart**.

### Adding a new agent

A `claude-style` agent (markdown commands under `{config_dir}/commands/{project}/`, reads `CLAUDE.md`) is a one-row registry append plus an MCP registration entry plus two satellite files:

1. Append a row with the six fields (`layout: claude-style`).
2. Add a row to §MCP registration (per-agent). MCP discovery is per-agent, not layout-derived, so even a `claude-style` agent must declare its own `target` / `scope` / `mechanism` — it is **not** inherited from the layout.
3. Add `framework/bootstrap/configure/{key}.md` with the agent's full permission set in its native settings format.
4. Add a curl snippet for the new agent to the README's adoption section.

An agent on a **different layout** (a new value in the `layout` column) additionally needs its branch added to §Derived values and the layout-keyed steps in §Per-Agent Scaffolding and §Permission Setup — the work the `antigravity` and `opencode` layouts each introduced. (MCP registration is per-agent regardless of layout, covered by step 2 above.)

## Inputs

The project inputs are the **project name**, a one-line **description**, and the primary **language(s)**. From `$ARGUMENTS`, extract the project name now (a single non-flag word, if present) and recognize the flags below. **Do not prompt for any missing project input here** — interactive collection is deferred to **§Collect Project Inputs**, which runs *after* the **Pre-flight Phase**. Collecting them earlier means a pre-flight abort — a stale `ductus.md`, which is the one contributor that still stops the run — discards the user's freshly-typed answers and forces them to re-enter everything on the restart. Nothing before §Collect Project Inputs — the pre-flight checks, agent selection, permission seeding, and the Pre-flight Phase itself — needs the interactive inputs; they need only `$ARGUMENTS` and the project's on-disk layout.

Recognized flags in `$ARGUMENTS`:

- `--agents=key1,key2,...` — explicit list of agent keys to scaffold. Bypasses any prompt. Reject unknown keys.
- `--add-agent` — force the agent-selection prompt even when agents are already detected.
- `--ref=<value>` — the source every fetch back into the `ductus` repository names: `latest` (the latest release), `main`, or a release tag such as `ductus-v0.55.0`. Leaving it out uses the project's recorded source, else the latest release. Validated and resolved by **Source resolution** (§Pre-flight Phase); the full contract is [061 — Updates track the latest release tag](https://github.com/stonean/ductus/blob/main/specs/061-updates-track-the-latest-release-tag/spec.md).

Flags may appear in any order alongside the project name.

## Pre-flight Checks

Before any scaffolding, verify:

- The current directory **is** an existing git repository. If not, stop and report: "This is not a git repository. Run `git init` first."
- If the spec-root directory already exists, this is a re-run. The spec-root name is `[paths] specs-root` from `.ductus/config.toml` when that file is present, else `specs` (spec 040). Report: "Existing {spec-root}/ directory found — running in update mode." Proceed normally; `update` strategy files will be overwritten, `create` strategy files will be skipped, `skip` strategy files will be left alone.

## Agent Selection

Determine which agents to scaffold using the first matching rule:

1. **Explicit list (`--agents=`)** — parse the comma-separated keys. For each key, look up the registry row. If any key is not present in the registry, stop before any scaffolding and report: "Unknown agent key: `{key}`. Valid keys: {comma-separated registry keys}." Do not partially scaffold. If the list is non-empty and all keys are valid, scaffold exactly those agents — no prompt.

2. **Auto-detect (default — routine update path)** — when neither `--agents=` nor `--add-agent` is present, list registry entries whose `config_dir` exists in the project. If at least one is detected, scaffold those silently with no prompt. This is the path that runs on every routine `/ductus` re-run.

3. **Add-agent / first-run prompt** — triggered when `--add-agent` is present, OR when no agent dirs are detected (first run after the curl install). Iterate the registry in row order and ask one yes/no `AskUserQuestion` per agent. Pre-select "Yes" when:
   - the agent's `config_dir` exists in the project, OR
   - this is first run (no detected dirs) AND the agent's `config_dir` is the parent directory of the running `ductus.md` file (i.e., the agent the user just curled into).

   If the running command cannot infer its own install path, fall back to no pre-selection — the user picks explicitly. This is acceptable on first run because the user just installed the file and knows which agent they're in.

   If the user confirms with zero agents selected, reject with: "At least one agent must be selected." Do not partially scaffold.

The user must end up with at least one selected agent in every path. Removing an adopted agent's tree is not part of this command's scope — see **Re-Run Behavior**.

## Permission Setup

For each selected agent, before fetching any files:

1. Read the agent's settings file — `{config_dir}/settings.local.json` for `claude-style`, `{config_dir}/settings.json` for `antigravity`, or the **repo-root `opencode.json`** for `opencode` (the same file as OpenCode's MCP-wiring target — settings and MCP wiring share one file; create it if missing, with the agent's `settings_template` from the registry; for `opencode`, merge into the adopter's existing `opencode.jsonc` instead if that is where their config lives). **For `pi`, skip this step entirely** — the settings file `.pi/settings.json` has **no permission-gating surface** (verified, spec 064 §Verified Pi Layout), so there is nothing to read, merge, or write; see the Pi section of §Derived values.
2. Merge the agent's `settings_template` entries into the existing file additively: add any entries that are missing, do not deduplicate or reorder anything else, and do not overwrite entries the user or `/{project}:configure` previously added. For `claude-style` the entries live under `permissions.allow`/`permissions.deny` (Claude) or `toolPermissions` (Auggie); for `antigravity` they live under `permissions.allow`/`permissions.deny`/`permissions.ask`; for `opencode` they live under the `permission` action map (preserving `$schema` and every other top-level key).
3. Write the file if anything was added.

This prevents repeated permission prompts during the fetch and scaffolding phases. The full permission set is applied later by `/{project}:configure` (which writes the same per-layout settings file). The seed also covers every step of **Runtime acquisition** — `mkdir`, the platform checksum tool, `ln`/`cp` for the pointer, and execution of the store and pointer paths for the version probe — so a bootstrap that acquires the runtime prompts for nothing. The checksum entry is the one that matters most: leaving it unseeded adds no safety, it puts a dialog at the one gate that must never be waved through, and a prompt that appears on every bootstrap trains the reflex to approve it. The digest comparison is what protects the adopter, and it halts before anything is written.

### ductus runtime auto-wiring

`/ductus` **acquires** the ductus runtime and registers it as an MCP server — the **Pre-flight Phase → State B** path. The runtime is required (§runtime-boundary), so a missing binary is work to perform rather than a condition to report: `/ductus` downloads the pinned release for the host platform into a ductus-owned store, materializes a per-project pointer to it, and wires the MCP config to that pointer. `PATH` is not consulted at any point.

Wiring depends on the agent's MCP registration `mechanism` (§MCP registration): a `write-file` agent gets its MCP file — or, for Pi, its extension bridge — written; a `surface-instruction` agent gets a one-line registration command surfaced for the user to run (ductus never writes the user's home config) — see **ductus runtime detection → MCP wiring** for the per-mechanism rules. In the same pass, either way, `/ductus` adds the **ductus tool permissions** to the settings file so the next session calls the runtime without a per-tool prompt:

- **Claude** (`permissions.allow`): `mcp__ductus__*`
- **Antigravity** (`permissions.allow`): `mcp(ductus/*)`
- **Auggie** (`toolPermissions`): `{ "toolName": "mcp:ductus:*", "permission": { "type": "allow" } }` if Auggie's matcher honors the wildcard, otherwise the enumerated `mcp:ductus:<tool>` set `/{project}:configure` already installs.
- **OpenCode** (`permission`): `"ductus*": "allow"` (a single glob in the root `opencode.json` `permission` map).
- **Pi**: **no permission entries exist to add** — Pi displays no host permission prompt for tool calls, so this pass is a no-op; the trust reminder in the completion message is the whole of Pi's handling (see §Derived values and the Pi layout section).

The wildcard is the minimal bootstrap grant; the enumerated per-tool set stays owned by the generated block in `/{project}:configure`'s permission file and coexists harmlessly (exact-match dedup leaves both). Both the wiring write and this permission write are additive and idempotent and follow the same merge rules as the seed above — no existing entry is removed, reordered, or overwritten. There is **no new confirmation prompt**: the acquisition and the wiring are disclosed by the **Closing restart** message, which names the store path and every file written — consistent with the §Procedural-fidelity rule the silent seed writes already follow.

## Pre-flight Phase

Run a single pre-flight phase after the **Permission Setup** seed (so the ductus binary probe is pre-authorized) and before **Pre-run Migrations** and the full archive fetch. It opens with **Source resolution**, which settles the one ref every later fetch back into the `ductus` repository names; that step restarts nothing, and each of its failures halts the run before anything is written. The phase then owns two restart-requiring checks — **ductus runtime detection** and the **ductus.md self-update check** — that can each force the session to restart: ductus detection to load a newly-wired MCP server, the self-update check to load a fresh `ductus.md`. Neither pays the cost of the multi-hundred-KB archive; both run on a small fetch or no fetch, so a restart-triggering abort never leaves archive work on disk.

The phase runs both checks and sorts every restart-requiring write into one of **two** sets: the **pending-restart set**, which stops the run at once (a stale installed `ductus.md`), and the **deferred-restart set**, which does not (State B's acquisition and wiring) and is carried to the **Closing restart** at the end of the run. **Pre-flight abort** inspects both. If neither check needs a restart, the run proceeds to **Pre-run Migrations**. Running both checks before that inspection is what collapses the worst case — a stale `ductus.md` on an adopter who has never wired ductus — into one restart instead of two.

**Create `{tempdir}` first, before anything else in the phase.** Source resolution and both checks fetch into it, and so does the later **Archive fetch and extract**:

```text
mktemp -d -t ductus-XXXXXX
```

On macOS/Linux this lands under `$TMPDIR` or `/tmp`. Never reuse a directory from a prior run — a fresh fetch is the only way `/ductus` picks up upstream changes. It is created here rather than inside any one step because the steps run in order and the *first* of them needs it: **Source resolution** fetches the version pin into it. Creating it inside a later step left the earlier ones with nowhere to fetch to, which is what made greenfield acquisition halt before it could start.

### Source resolution

Settle the run's one source before anything fetches from it. **One run, one source**: the version pin, the self-update fetch, and the framework archive all name the ref this step resolves, and nothing re-resolves it later, so a release published partway through the run cannot split the run across two releases. The values it settles — `{raw-ref}`, `{archive-ref}`, `{source-label}`, `{pin}` — and the `{ref-floor}` constant are listed in **Derived paths** below. The steps run in order, and each halt below fires before anything is written — only the **Permission Setup** seed, which is additive, idempotent and identical for every source, precedes this step.

1. **Read the inputs.** Collect every `--ref` in `$ARGUMENTS` (§Inputs). Read the **active config file** (§Project Configuration) once, for its `[source] ref`, its `[migrations] last_applied`, and the `[runtime] path` that **Runtime acquisition** Branch 1 reads. A malformed config file aborts here under §Project Configuration's existing rule, since this is now the first step to read it.

   Read `DUCTUS_REPO` too, which every fetch below names as `${DUCTUS_REPO:-stonean/ductus}` (spec 065), with `awk 'BEGIN { print ENVIRON["DUCTUS_REPO"] }'`, a command every agent's **Permission Setup** seed already allows. When it is set, non-empty, and not `stonean/ductus`, print this before anything is fetched, so that a variable left set in a shell profile cannot silently change where the runtime binary comes from:

   > `Source repository: {value} (from DUCTUS_REPO), not the canonical stonean/ductus. Every fetch this run, the runtime binary included, comes from it.`

2. **Check the grammar.** An accepted value is exactly `latest`, `main`, or `ductus-v<MAJOR>.<MINOR>.<PATCH>` — digits only, no pre-release or build suffix, which is the form every release tag carries.
   - `--ref` given more than once halts, naming every value given. Picking one would decide silently between two stated intents.

     > Halt: `--ref was given {n} times ({values}) — give it once.`
   - A `--ref` value outside the grammar halts, naming it. A bare `--ref` and `--ref=` are empty values and halt the same way.

     > Halt: `invalid --ref "{value}" — accepted forms are latest, main, or ductus-v<MAJOR>.<MINOR>.<PATCH>.`
   - With no `--ref`, the recorded `[source] ref` governs, and a recorded value outside the grammar halts, naming the file it came from. With a `--ref`, the record is not consulted: the flag replaces it, which is also how a bad record is repaired without a hand-edit.

     > Halt: `invalid [source] ref "{value}" in {active config file} — accepted forms are latest, main, or ductus-v<MAJOR>.<MINOR>.<PATCH>. Fix or remove it, or re-run with --ref to replace it.`

   The source is the `--ref` value, else the recorded value, else `latest`, and its origin is `--ref`, `recorded`, or `default` accordingly. A recorded `latest` is never written by `/ductus`, since the record's absence already means it, but reads as the default when present.

3. **Resolve `latest`.** When the source is `latest`, read GitHub's latest release — never a draft or a prerelease — from the redirect, without following it:

   ```text
   curl -sSI https://github.com/${DUCTUS_REPO:-stonean/ductus}/releases/latest
   ```

   The tag is the last path segment of the `Location` header (`location` over HTTP/2) after `/releases/tag/`. A response carrying no `Location`, or one whose target has no `/releases/tag/` segment, halts; so does a tag outside the `ductus-v<MAJOR>.<MINOR>.<PATCH>` grammar. Neither falls back to `main`: a fallback would put the run on exactly the unreleased state the default exists to avoid, and would read identically to a successful resolution.

   > Halt: `could not resolve the latest release from https://github.com/${DUCTUS_REPO:-stonean/ductus}/releases/latest — {what came back: the status line and the Location value, or "no Location header"}. /ductus does not fall back to main; pass --ref=main or --ref=<tag> to choose a source explicitly.`

   The REST API's `releases/latest` means the same thing but is limited to 60 unauthenticated calls an hour, and the highest `ductus-v*` tag can name a tag whose release, and so whose runtime assets, does not exist yet. The redirect has neither problem (spec 061, Resolved Questions).

4. **Check the release floor.** A resolved tag — named, recorded, or the latest release — below `ductus-v{ref-floor}` halts, naming the tag and the floor. Compare the three version fields numerically. Every release below the floor ships a bootstrap that fetches from `main` whatever ref it is given, so once its bootstrap was placed the tag could not be honored.

   > Halt (a named or recorded tag): `{tag} is older than ductus-v{ref-floor}, the first release whose bootstrap honors --ref. Every earlier release's bootstrap fetches from main, so {tag} could not be honored. Name ductus-v{ref-floor} or later, or pass --ref=main.`

   The latest release reaches this floor only before the release carrying this bootstrap is published — a pushed tag takes some minutes to become a release, and until then `main`'s bootstrap is the only one that has this step.

   > Halt (the latest release): `the latest release, {tag}, is older than ductus-v{ref-floor}, the first release that carries this bootstrap. That release may still be publishing. Re-run once it exists, or pass --ref=main to proceed now.`

5. **Fetch the pin at the ref.** One SemVer line, no `v` prefix:

   ```text
   curl -fsSL https://raw.githubusercontent.com/${DUCTUS_REPO:-stonean/ductus}/{raw-ref}/version -o {tempdir}/version
   ```

   This fetch is also the existence check for a named or recorded tag, so a tag costs no extra request: every release at or above the floor carries a `version` file, so an HTTP 404 on a tag means the tag does not exist.

   > Halt (404 on a named or recorded tag): `tag {tag} does not exist — https://raw.githubusercontent.com/${DUCTUS_REPO:-stonean/ductus}/{tag}/version returned 404.`

   Any other failure, or a file that is **absent or unparseable**, halts naming the URL: guessing a version or falling through to another source silently installs a runtime the framework was never tested against.

   > Halt: `could not read the runtime version pin from https://raw.githubusercontent.com/${DUCTUS_REPO:-stonean/ductus}/{raw-ref}/version — /ductus cannot state which runtime this framework revision requires.`

   The file's one line is `{pin}` for the rest of the run. It is fetched here rather than read out of the framework archive because acquisition runs in pre-flight, long before **Archive fetch and extract**, and State B is the first-run state by definition, so the pin must never depend on the archive.

6. **Check the migration floor** — on a tag only, and only when `[migrations] last_applied` is set. Migrations run forward only (§Pre-run Migrations), so moving below one the project has applied would lay that release's pre-migration files over the migrated layout. `last_applied` holds a migration **id**, and the registry mapping ids to `introduced_in` ships in the archive, which pre-flight has not fetched, so read the tag's registry:

   ```text
   curl -fsSL https://raw.githubusercontent.com/${DUCTUS_REPO:-stonean/ductus}/{raw-ref}/framework/migrations.toml -o {tempdir}/migrations.toml.ref
   ```

   - **The id is an entry there**: the tag knows the migration, so it is not older than it. Pass.
   - **The id is absent**: fetch `main`'s registry the same way, into `{tempdir}/migrations.toml.main`. Entries are appended, and removed only when they sunset.
     - **An entry at `main`**: the migration was added after the tag. Halt.
     - **Absent at `main` too**: the migration has been retired, so the project's layout predates every live entry. Pass, as §Pre-run Migrations' stale-reference behavior treats a retired id.

   The tag's registry alone cannot tell a migration newer than the tag from a retired one, and those need opposite answers. The `main` read is a lookup that validates the tag, not a source fetch: nothing the run installs comes from it, so it does not break *one run, one source*. Either fetch failing halts naming its URL, as step 5's pin halt does.

   > Halt: `{tag} is older than migration {id} (introduced in {introduced_in}), which this project has already applied ([migrations] last_applied). Migrations run forward only, so {tag}'s files would be laid over the migrated layout. Name a release at or after {introduced_in}, or pass --ref=main.`

   The latest release reaches this floor when a project on `main` applied a migration no release carries yet; the same message covers it, and a later release clears it.

7. **Report the source.** Emit one line, before any later step can halt or abort, so every run past this point says which source it used:

   > `Source: {source-label}`

   The line is repeated in **Post-Scaffolding Output**.

### ductus runtime detection

Resolve whether the ductus runtime is live in this session and, when it is not, acquire and wire it so the next session runs the deterministic path. Detection resolves to one of **two** states — A (runtime live this session) and B (not live: acquire, wire, restart). There is no third "binary absent" state: the runtime is required, and a missing binary is work to perform.

#### Detection mechanism

- **Tool-inventory introspection (State A).** Inspect your own available-tool inventory for any `ductus`-namespaced MCP tool — `mcp__ductus__*` on Claude Code, `mcp:ductus:*` on Auggie and Antigravity, a `ductus*`-prefixed `<server>_<tool>` name on OpenCode, a `ductus__*`-prefixed extension-bridge tool on Pi (the `.pi/extensions/ductus.ts` bridge registers them) — counting deferred or lazily-loaded tool names as present (a host that lists tool names before exposing their schemas still has the runtime registered). Any match ⇒ **State A**. This needs no shell and no permission; you always know your own tools.
- **Store probe (State B).** Only when introspection finds no `ductus` tool. This is a **filesystem check for the ductus-owned store**, not a `PATH` lookup: test whether `{store-path}` exists and executes. `PATH` is not consulted — an adopter's `ductus` on `PATH` is ignored entirely, not consulted, not warned about, not removed. The probe is pre-authorized by the **Permission Setup** seed. A probe that cannot run classifies the run as State B, which acquires; acquisition is idempotent, so a false negative costs a version comparison, not a redundant download.

#### Namespace scope

**Only `ductus`-namespaced tools count — in either state, and for the whole run.** No MCP tool outside the `ductus` namespace may perform a step of this procedure, and none counts as evidence that the runtime is available. This governs tools that would otherwise stand in for the runtime — a retired `gvrn` server above all; it says nothing about unrelated servers an adopter has registered for their own purposes, which this procedure never calls either way. The introspection above already scopes *classification* this way; this scopes *execution* the same way, because a host reaching for a live look-alike is otherwise following the general preference for the deterministic path rather than disregarding an instruction.

The reason is not tidiness. A retired-namespace server is a *different runtime at a different version*, and its primitives resolve paths against the directory layout of the release that shipped them — a pre-`.ductus/` binary resolves `.govern/` and then the legacy root, neither of which a converged project has. Two adopter shapes reach it, and the rule binds for the whole run in both:

- **A migrating run (State B).** The adopter still registers the retired key *and* still has the retired binary, both by design: the rename declines to touch an installed binary, and the retired crate stays published rather than yanked. Its resolvers are wrong **by construction** here, because this run is what migrates the layout — so a write lands in the pre-migration location with a success result. The rule binds past the rename step that removes the key, since an MCP server is spawned at session start and is not torn down when its registration is deleted.
- **A converged project whose retired registration survives (State A).** For a `surface-instruction` agent the retired key lives in the user's home config, which `ductus-rename` warns about rather than rewriting — so it persists until the user acts on that warning. Every session after they also register `ductus` has both namespaces live, indefinitely, and the retired resolver falls through to a path the project no longer has.

#### Derived paths

| Name | Value |
| --- | --- |
| `{store-dir}` | `~/.ductus/bin/` |
| `{store-path}` | `~/.ductus/bin/ductus` (`ductus.exe` on Windows) |
| `{pointer-path}` | `.ductus/bin/ductus` (repo-relative; `ductus.exe` on Windows) |
| `{pin}` | the single SemVer line in `{tempdir}/version`, fetched by **Source resolution** step 5 |
| `{triple}` | the host target triple, from the table in **Runtime acquisition** |
| `{raw-ref}` | the path segment naming the resolved source on `raw.githubusercontent.com`: `main`, or the tag (`ductus-v0.55.0`) |
| `{archive-ref}` | the path segment naming the resolved source on `codeload.github.com`: `refs/heads/main`, or `refs/tags/{tag}` |
| `{source-label}` | the resolved source and its origin, as reported: `main (--ref)`, `ductus-v0.55.0 (recorded)`, `latest release ductus-v0.55.0 (default)` |
| `{ref-floor}` | `0.55.0` — the version of the first release whose bootstrap carries **Source resolution**; `install.sh` carries the same value, and `/ductus:audit` Family 14 asserts the two agree |

#### State A — runtime live this session

A `ductus`-namespaced tool is available to this session, so the runtime is live and the rest of the run takes the **deterministic primitive path**.

**Live is not current — version-check it against `{pin}` before trusting it.** Probe the resolved binary (the `[runtime] path` when the project configures one, else `{store-path}`) and read its reported version. This is the same probe **Runtime acquisition** step 2 performs, and the **Permission Setup** seed pre-authorizes it.

- **Reports `{pin}`** — proceed. ductus contributes nothing to **either** restart set, and detection emits no message. This is the routine path.
- **A project-supplied `[runtime] path` reports something else** — emit Branch 1's warning and continue. A project naming a path has stated deliberately which binary it wants.
- **Anything else** — the runtime is **live but stale**. Acquire `{pin}` per **Runtime acquisition** Branch 2, then run the rest of this session through `{pointer-path} <primitive>` rather than the MCP tools: the server was spawned at session start and is still the old binary, so its tool surface stays stale no matter what is now in the store. Add the acquisition to the **deferred-restart set** and carry the notice to the **Closing restart**, exactly as State B does.

A live-but-stale runtime fails in the direction hardest to attribute. It is missing primitives the framework has since come to depend on, and `/ductus` reports success because a tool *was* in the inventory — so the failure surfaces later, somewhere else, as someone else's problem. Observed 2026-08-19 in an adopter project: the store held `0.29.10` while the framework pinned `0.31.0`, so the pre-commit hook that same run refreshed called `derive-dependencies` and `derive-references`, which that binary does not carry, and the shell generators they replaced had already been deleted. Every commit in that project halted, and nothing in the `/ductus` run said the runtime was behind. Detection that stops at "a tool is in my inventory" answers the wrong question: what the run needs to know is whether the runtime it is about to depend on is the one this framework revision was tested against.

State A is a **binding execution contract, not a preference.** Detecting the runtime and then walking the prose `curl`/`tar`/`python3` path anyway is the exact failure 029 exists to prevent — it spends the markdown path's tokens despite a cheaper path being live, and it is what makes the State-B wire-and-restart pointless. For the rest of this run:

- **Every step that names a backticked primitive** — a bare name (`fetch-archive`, `extract-archive`, `apply-manifest`, `merge-managed-block`, `enforce-manifest`, `merge-permissions`, `run-generator`, …) that matches a `ductus` tool in your inventory — **MUST be performed by calling that MCP tool** (`mcp__ductus__<primitive>` on Claude, `mcp:ductus:<primitive>` on Auggie/antigravity, a `ductus*`-prefixed name on OpenCode; mapping per §Instructions).
- **The shell commands shown under those steps** (`curl`, `tar -xzf`, `python3`, `awk`, byte-compares, hand-authored scaffold loops) are the **markdown-only specification.** In State A they document the contract each tool fulfills, and a State-B run reaches the same primitives through the CLI at `{pointer-path}`, so they are **not instructions to execute** in either state. Do not run them. If you are about to run `curl`/`tar`/`python3` for a step that names a primitive, stop — that is the fallback path leaking into a State-A run; call the tool instead.
- **Steps with no backticked primitive run as shown in every state** — the per-language `.gitignore` `curl` against `github.com/github/gitignore`, `git config core.hooksPath`, `chmod`, the git repo / tracked-file checks, and the §Collect Project Inputs prompts have no tool equivalent.
- **If a primitive call errors** — e.g., a too-old wired `ductus` surfaces a parse error per `spec 022` §Versioning enforcement — fall back to **that step's** shell specification for that one step and continue; do not abandon the deterministic path for the whole run.

#### State B — runtime not live: acquire, wire, restart

No `ductus` tool is available to this session. In order:

1. **Resolve the binary** per **Runtime acquisition** below — either the project's own `[runtime]` path, or an acquisition into the store.
2. **Materialize the pointer** per **Pointer materialization** below.
3. Register the `ductus` server per the agent's MCP registration `mechanism` (§MCP registration; details in **MCP wiring**): for `write-file`, write the MCP file additively; for `surface-instruction`, write **no** MCP file — the registration command is surfaced in the **Closing restart** for the user to run once per machine.
4. Add the permission entries needed to call the `ductus` tools (see **Permission Setup**), so the next session calls them without a prompt. This write is the same for every agent regardless of `mechanism` — it targets the project-level settings file, not the MCP-server location.
5. Add the acquisition, the wiring, and the permission write to the **deferred-restart set** and contribute this notice to the **Closing restart** at the end of the run, naming the store path and every file written. **Closing restart** is a subsection of **Post-Scaffolding Output** in the archive half; State B reaches it, because State B does not abort — the set is carried forward and rendered there. State B does **not** stop here: the binary is now on disk and `Bash(.ductus/bin/ductus *)` was seeded in **Permission Setup** before the probe, so the run continues and invokes every remaining primitive as `{pointer-path} <primitive>` — the same deterministic code the MCP tools call, reached through the CLI surface spec 022 AC1 ships alongside them. The abort this used to raise bought nothing the CLI could not already give this run.

> **ductus runtime acquired.** The runtime was not registered as an MCP server for this project, so the rest of this run calls it through the CLI at `{pointer-path}` rather than as tools. Installed `{version}` to `{store-path}`.

This notice rides the **Closing restart** at the end of the run, not a pre-flight abort — State B does not stop here. It takes the form matching the selected agent's `mechanism`:

- **`write-file` agent** (e.g. Claude): "It has now been wired in so the next session runs through the runtime, which uses far fewer tokens. Files written: {comma-separated paths — the store, the pointer, the wiring file, and the settings file when permission entries were added}."
- **`surface-instruction` agent** (e.g. Auggie): "{Agent} registers MCP servers in your user-level config, which `/ductus` does not write. To enable the runtime, run this once, then start a fresh session: `{the agent's surfaced instruction from §MCP registration}`. Files written: {the store, the pointer, and the settings file when permission entries were added}."

State B issues **no separate consent prompt** — the writes are additive and idempotent, matching the silent **Permission Setup** writes; the **Closing restart**'s file list (and, for a `surface-instruction` agent, the one-line command) is the disclosure. There is no opt-out flag.

When acquisition **fails**, the run halts per **Runtime acquisition → Failure**; it does not proceed to steps 2–5 and does not silently continue on the markdown path.

#### Runtime acquisition

Performed in State B, before any wiring. Two branches.

##### Branch 1 — the project supplies its own binary

When `.ductus/config.toml` has a `[runtime]` `path` key, the project has taken responsibility for supplying the runtime. This is the supported route for building from source, for an air-gapped or firewalled checkout, and for a platform with no published asset.

1. Resolve `path` (relative to the repo root, or absolute).
2. **No download is attempted and nothing is written to the store.**
3. Execute it to read its version. Compare against `{pin}`: a mismatch emits one warning line and continues — a project naming a path has stated deliberately which binary it wants, and a development build is expected to run ahead of the last release.

   > `warning: [runtime] path {path} reports {found}, the framework pins {pin} — using the configured binary.`
4. A path that does not exist, or will not execute, **halts** the run naming the configured path. Never fall through to downloading: that would discard the project's stated choice without saying so.

   > Halt: `[runtime] path {path} does not exist` / `… will not execute`. Fix the path or remove the `[runtime]` key to let `/ductus` acquire the pinned release.
5. The pointer resolves to this path rather than to the store.

##### Branch 2 — acquire the pinned release

1. **Read the pin.** `{pin}` is the line **Source resolution** step 5 fetched into `{tempdir}/version` and validated; nothing is fetched here. Reading it from the framework archive is what this step once specified, and it halted every greenfield adoption: State B is the first-run state by definition, so the pin was never on disk when this step needed it. A one-line file keeps pre-flight's small-fetch-or-no-fetch property intact — it is the archive's multi-hundred-KB cost this phase avoids, not a `curl`.

   The pin and the framework tree arrive in separate fetches, and they agree because both name the ref **Source resolution** settled — and both honor `$DUCTUS_REPO` (spec 065): when unset or empty the canonical `stonean/ductus` is fetched byte-identically, and set to another owner/repo (e.g. `DUCTUS_REPO=myfork/ductus`) the whole adoption stays on one origin. A tag does not move between them. `main` can: a push landing between the two fetches is the sole divergence on that source, it is bounded by one run, and the next `/ductus` re-acquires against the newer pin — acquisition is idempotent and re-probes the store. That exposure is what `--ref=main` opts into.

2. **Probe the store for idempotency.** Execute `{store-path}` and read its reported version.
   - Reports `{pin}` ⇒ **already current**. Perform no download and leave the binary byte-unchanged. Continue to the pointer.
   - Reports a different version ⇒ re-acquire, overwriting the store.
   - **Will not execute, or reports nothing** ⇒ treat as *no usable runtime*, not as *version unknown*, and acquire. A truncated download, a wrong-architecture asset, or a missing system library all land here — which is why the probe executes the binary rather than reading a recorded marker.

3. **Derive the target triple** from the host platform and architecture:

   | Platform | Architecture | `{triple}` |
   | --- | --- | --- |
   | macOS | arm64 | `aarch64-apple-darwin` |
   | macOS | x86_64 | `x86_64-apple-darwin` |
   | Linux | x86_64 | `x86_64-unknown-linux-gnu` |
   | Linux | arm64 | `aarch64-unknown-linux-gnu` |
   | Windows | x86_64 | `x86_64-pc-windows-msvc` |

   A host matching no row halts naming the platform and the `[runtime]` key — supplying a binary is the escape hatch for an unpublished platform.

4. **Fetch the archive and its sidecar** from the release, into `{tempdir}`:

   ```text
   curl -fsSL "https://github.com/${DUCTUS_REPO:-stonean/ductus}/releases/download/ductus-v{pin}/ductus-{triple}.tar.gz" \
     -o {tempdir}/ductus-{triple}.tar.gz
   curl -fsSL "https://github.com/${DUCTUS_REPO:-stonean/ductus}/releases/download/ductus-v{pin}/ductus-{triple}.tar.gz.sha256" \
     -o {tempdir}/ductus-{triple}.tar.gz.sha256
   ```

5. **Verify the digest before installing anything.** Compute the archive's SHA-256 with the platform tool — `shasum -a 256` on macOS, `sha256sum` on Linux, `certutil -hashfile … SHA256` on Windows — and compare against the sidecar. This is stricter than the framework archive fetch, which tolerates a missing sidecar because GitHub's auto-generated source tarballs ship without one; the runtime's release assets always carry theirs, so a **missing sidecar is a failure here**, not a skip.

6. **Install into the store**, only after the digest matches:

   ```text
   mkdir -p {store-dir}
   tar -xzf {tempdir}/ductus-{triple}.tar.gz -C {tempdir}
   ```

   Write the extracted binary to `{store-path}` via **tempfile + rename** — the same atomic write every other ductus write uses — so a concurrent `/ductus` run in another project sees the old binary or the new one, never a partial file. Then `chmod +x {store-path}`.

7. **Re-probe** the store path and confirm it reports `{pin}`. A binary that installs but does not run is an acquisition failure, not a success.

##### Failure

A network failure, an unpublished asset for the host platform, a missing sidecar, or a checksum mismatch **aborts the run**. Nothing is written into the store or the pointer, and the run does **not** degrade to the markdown path — a requirement that quietly is not one would leave both execution paths alive, which is the cost the requirement exists to end.

The error names the exact store path and the release URL, so an adopter behind a firewall can place the binary by hand and re-run:

> Halt: `could not acquire the ductus runtime {pin} for {triple}: {reason}.`
> `Place the binary at {store-path} and re-run, or set [runtime] path in .ductus/config.toml to a binary you supply.`
> `Release: https://github.com/${DUCTUS_REPO:-stonean/ductus}/releases/tag/ductus-v{pin}`

**The home directory is unwritable, absent, or on a read-only mount** — some CI containers and locked-down images. Halt with the same shape, naming the store path and the `[runtime]` key, since supplying a binary from a writable location is exactly the escape hatch for this case.

#### Pointer materialization

The pointer exists for one reason: a committed MCP config must not name a machine-specific absolute path. `.mcp.json` and `opencode.json` are shared with the whole team, so `/Users/alice/.ductus/bin/ductus` in either breaks every other contributor and every CI checkout.

1. `mkdir -p .ductus/bin`
2. Attempt a **symlink** from `{pointer-path}` to the resolved binary (`ln -sf`).
3. If symlink creation fails, **fall back to a copy** (`cp`). Windows requires developer mode or elevation to create a symlink, and no supported platform may require elevated privileges — the copy is what keeps that true.
4. **Repair without ceremony.** A missing or dangling pointer is recreated, not reported. It is gitignored, so it never arrives with a clone: a dangling pointer is the expected state of any checkout nobody has bootstrapped on this machine yet, not an error.

The pointer is covered by the framework-managed `.gitignore` block's `/.ductus/bin/` entry, so `git status` reports nothing untracked after a bootstrap.

#### MCP wiring

How State B registers `ductus` depends on the agent's MCP registration `mechanism` (§MCP registration). For the `mcpServers`-shaped agents (Claude/Auggie/Antigravity) the server entry is a `mcpServers` map keyed by name; only the **location** (and whether ductus writes it) differs:

```json
{ "mcpServers": { "ductus": { "command": ".ductus/bin/ductus", "args": ["mcp"] } } }
```

OpenCode uses a different shape — an `mcp` key with a typed local-server entry — written into the committed root `opencode.json` (the OpenCode sub-case below):

```json
{ "mcp": { "ductus": { "type": "local", "command": [".ductus/bin/ductus", "mcp"], "enabled": true } } }
```

**`write-file` agents** (scope `project-committed` — Claude and OpenCode; scope `project-local` — Pi). ductus writes the agent's `target` from §MCP registration, using that agent's wiring shape — **Claude:** `.mcp.json` at the repo root, the `mcpServers` map, `{ "command": ".ductus/bin/ductus", "args": ["mcp"] }`; **OpenCode:** the committed root `opencode.json` (or the adopter's existing `opencode.jsonc`), the `mcp` map, `{ "type": "local", "command": [".ductus/bin/ductus", "mcp"], "enabled": true }`; **Pi:** a **verbatim copy** of `framework/bootstrap/pi/ductus-bridge.ts` from the staging archive to `.pi/extensions/ductus.ts` — the extension bridge that re-registers the runtime's MCP server's tools as `ductus__<name>` pi tools (spec 064 D3; the bridge is the whole of Pi's wiring, and it is the one `write-file` target whose `scope` is `project-local` rather than `project-committed` — because `.pi/` is gitignored — so the write is idempotent and re-scaffolded by every `/ductus`). For Claude and OpenCode, the write **updates the file in place — it never replaces or truncates it.** Apply the matching case (read `{servers-key}` as `mcpServers` for Claude, `mcp` for OpenCode; the Pi case is the verbatim copy above, independent of these):

- **Missing file** — create it containing only the `ductus` entry (for OpenCode, include `"$schema": "https://opencode.ai/config.json"`).
- **Has `{servers-key}`, no `ductus`** — add the `ductus` entry; preserve every other server and every other top-level key (including OpenCode's `$schema` and `permission`).
- **Already has a `ductus` entry** — no-op; leave the file byte-unchanged (idempotent re-run).
- **No `{servers-key}` key** — add the key with just the `ductus` entry; preserve all other top-level keys.
- **Not valid JSON** — do **not** touch the file. Skip wiring and warn the user to repair it. The runtime is still acquired and the pointer still materialized — only the registration is skipped — so the next run wires it once the file parses. A hand-maintained config is never clobbered.

There is no `ductus` runtime primitive for this merge: State B is the runtime-absent case by definition, so the write is always host-side.

**`surface-instruction` agents** (scope `user-global` / `home-level` — Auggie and Antigravity). The split follows the config's **scope**, not the agent: a `project-committed` target names the repo-relative pointer, a `user-global` / `home-level` target names the absolute store path. That is what removes the asymmetry these agents used to carry — their config holds a single `ductus` entry serving every project on the machine, so no project-specific path could ever be correct in it, and a store owned by no project can. The agent reads MCP servers from a file in the user's **home** directory, shared across all their projects, which ductus must **not** write. ductus writes no MCP file; instead the **Closing restart** surfaces the agent's registration instruction for the user to run once per machine, then restart:

- **Auggie** — `auggie mcp add ductus --command {store-path} --args "mcp"` (the documented, schema-stable subcommand; it writes `~/.augment/settings.json`). The **absolute store path**, not the pointer: this config is per-machine and shared across every project, so no project-relative path could be correct in it.
- **Antigravity** — add a `ductus` block to `~/.gemini/config/mcp_config.json` naming the **absolute store path** (`{"mcpServers": {"ductus": {"command": "{store-path}", "args": ["mcp"]}}}`), then reload via the in-prompt `/mcp` overlay (there is no scriptable `agy mcp add`; project-local `.agents/mcp_config.json` is ignored). Absolute for the same reason as Auggie: the file is per-machine and serves every project.

The permission write (State B step 4) still happens for these agents — it targets the project-level settings file the agent reads, independent of the home-level MCP-server location.

### Recording the source

A `--ref` is recorded in the project's committed configuration, so the choice outlives the run that made it. It is written here — after **ductus runtime detection**, because that is the first point at which `merge-managed-block` is reachable in both states (the MCP tool in State A, `{pointer-path} merge-managed-block` in State B), and before the **Self-update check**, because the self-update's stale path aborts pre-flight and the re-run that follows must read the choice back rather than resolve the default and replace the bootstrap again. Nothing is recorded before **Source resolution** has validated the ref, so a bad `--ref` never persists.

Invoke `merge-managed-block` against the **active config file** (§Project Configuration) with `marker-style: "line-prefix"` and `marker: "ductus (source)"`:

- **`--ref=main` or `--ref=<tag>`** — the block is two lines, `[source]` and `ref = "{value}"`, with the value exactly as given.
- **`--ref=latest`, with a `# ductus (source)` block present** — the block is `[source]` alone. A bare `[source]` table reads as the default, so the project returns to the latest release.
- **`--ref=latest` with no `[source]` table in the file, or no `--ref` at all** — write nothing. A recorded or default source is already what the file says.

The primitive appends a new block at the end of the file, and a trailing TOML table cannot capture another table's keys, so appending is safe. It also removes lines elsewhere in the file that repeat the block's own lines, which is right for `.gitignore` and wrong for a hand-written `[source]` table outside the block: its header would go and its `ref` key would be left under whichever table precedes it. So when a `--ref` is given and the file carries a `[source]` table that is not inside a `# ductus (source)` block, halt instead of writing — `--ref=latest` included, since writing nothing would leave that table governing the next run:

> Halt: `{active config file} has a [source] table outside its "# ductus (source)" managed block. Add that line directly above [source], or remove the table, then re-run /ductus --ref={value}.`

### Self-update check

Verify the running session's `ductus.md` instructions are current.

#### Small fetch

`{tempdir}` already exists — the **Pre-flight Phase** created it before anything else, and **Source resolution** has already fetched the version pin into it. Do not create a second one.

Issue exactly one `curl` against `raw.githubusercontent.com` for the upstream bootstrap file, at the ref **Source resolution** settled:

```text
curl -fsSL "https://raw.githubusercontent.com/${DUCTUS_REPO:-stonean/ductus}/{raw-ref}/framework/bootstrap/ductus.md" \
  -o {tempdir}/ductus.md.upstream
```

"Upstream" is the resolved source's copy, not `main`'s: a project on the latest release or a tag compares its installed bootstrap with that release's, so an update replaces it only with a released bootstrap.

If the fetch fails — non-zero `curl` exit, network error, or a 404 — abort the run with this error and do not continue:

> Failed to fetch the ductus.md self-update check ({reason}). Re-run after checking network connectivity, or report this if it persists.

#### Per-agent comparison

For each selected agent, compare the upstream `{tempdir}/ductus.md.upstream` against the agent's installed `ductus` file and assign one status. For `claude-style` the installed file is `{config_dir}/commands/ductus.md`, for `opencode` it is `{config_dir}/command/ductus.md`, and for `pi` it is `{config_dir}/prompts/ductus.md` — all three installed verbatim (frontmatter included), so the comparison is a direct byte-compare against `{tempdir}/ductus.md.upstream`. For `antigravity` the installed file is `{config_dir}/skills/ductus/SKILL.md`, which wraps **only the upstream body** in `name: ductus` frontmatter — the installer drops `ductus.md`'s own frontmatter when wrapping. So compare **bodies on both sides**: strip the leading frontmatter block (the first `---`-delimited region) from the installed `SKILL.md` **and** from `{tempdir}/ductus.md.upstream`, then byte-compare what remains. Stripping only the `SKILL.md` side leaves `ductus.md`'s frontmatter on the upstream side, which never matches — a false `stale` on every run. The statuses below are assigned from this body-vs-body (antigravity) or file-vs-file (`claude-style` / `opencode` / `pi`) comparison:

- **`no installed copy`** — the installed file does not exist (first run for this agent). Continue.
- **`current`** — the two files are byte-identical, **or** the installed file is byte-identical to upstream and listed in `.ductus/config.toml` `pinned.files` (the pin had nothing to suppress this run). Continue.
- **`stale`** — the two files differ and the installed file is **not** pinned. The running session is using instructions other than the resolved source's — usually older ones, or newer ones when the project has moved to an earlier release.
- **`pinned-divergent`** — the two files differ and the installed file **is** listed in `.ductus/config.toml` `pinned.files`. The pin intentionally suppresses the update; continue, and emit a single advisory line in the post-scaffolding output.

The check is scoped to **selected agents only** — agents whose `config_dir` exists in the project but are not in this run's selection are not diffed. An unselected stale agent will trip the check on its very next `/ductus` run targeting it.

#### Stale → defer to pre-flight abort

If any selected agent is recorded as `stale`:

1. For **each stale agent**, overwrite **the installed file the staleness comparison just read** — not the canonical filename — so the next session loads the up-to-date instructions. For `claude-style` that is `{config_dir}/commands/ductus.md` once the entry point has been renamed, but `{config_dir}/commands/govern.md` for an adopter still carrying the retired one; copy `{tempdir}/ductus.md.upstream` verbatim over whichever the comparison resolved. For `opencode`, the same rule against `{config_dir}/command/`; for `pi`, against `{config_dir}/prompts/`. Writing the canonical name instead would create a second command file beside the stale one, and `ductus-rename` step 6 then moves the stale file onto the canonical path, replacing the fresh copy with the one it just superseded. The canonical filename is owned solely by that migration. For `antigravity`, write `{config_dir}/skills/ductus/SKILL.md` as the transformed skill — `name: ductus` frontmatter followed by the upstream body — **not** the raw `ductus.md` (a raw copy is not a loadable skill). In both cases do not substitute placeholders in the body — `{project}` and `{cli-config-dir}` stay literal, per the `ductus` self-install rule.
2. Run the **Post-Write Integrity Check** (see below) on each freshly written file.
3. Do not write `ductus.md` for non-stale agents — their installed copies already match upstream.
4. Do not write `ductus.md` for `pinned-divergent` agents — the pin opts them out of automatic updates.
5. Add each stale agent's overwrite to the **pending-restart set** and contribute this notice to the combined **Pre-flight abort** — do **not** abort here:

> **The ductus command itself has updated from {source-label}.** Your installed copy differed from that source's and the running session is using the older instructions. The freshly fetched copy has been written to disk for stale agents.
>
> Stale agents updated: {comma-separated names}.

The shared "start a new session and re-run" line and the skip of every later section are owned by **Pre-flight abort**, so a stale `ductus.md` and a freshly-wired ductus surface in one abort and one restart rather than two.

#### Pinned-divergent → continue with advisory

If a selected agent is recorded as `pinned-divergent`, the run continues normally. After scaffolding, the **Post-Scaffolding Output** includes one advisory line per divergent agent (see **Post-Scaffolding Output → Pinned ductus.md advisory**). The advisory is silent on runs where every pinned agent is `current` (the pinned version happens to match upstream this run).

Pinning is an opt-out from automatic updates, not an opt-out from knowing the pin is currently active. When the pinned version actually drifts from upstream, the user usually wants to either review the upstream changes and unpin, or consciously confirm they are staying on the old version. Adopters who are deliberately and indefinitely on an old version see no recurring nag because the advisory only fires when divergence is real.

#### Current / no installed copy → continue

When all selected agents are `current` or `no installed copy`, the self-update check contributes nothing to the **pending-restart set**. The `{tempdir}` the **Pre-flight Phase** created is reused by the **Archive fetch and extract** step below — one `mktemp` for the whole run, no leaked extra temp directory. Whether the run proceeds is decided by **Pre-flight abort** once ductus detection has also run.

### Pre-flight abort

After both checks have run, inspect **both** sets. They are **not** equivalent — only the first stops the run — so they are inspected separately:

- **A stale `ductus.md` (self-update) — the pending-restart set** — abort **now**, before any further work. The run must not proceed on instructions it has just replaced: the installed copy that is executing cannot be trusted to describe the procedure that now exists on disk. Emit the stale-update notice — see **Self-update check → Stale → defer to pre-flight abort** — with the closing line **Start a new session and re-run `/{installed-command}` to pick up the changes**, where `{installed-command}` is the basename of the installed file step 1 just overwrote. Name the command the adopter can actually invoke, never the canonical one: an adopter still on the retired entry point has no `/ductus` command until `ductus-rename` step 6 creates it, and that migration cannot run until they successfully invoke the bootstrap again — so hardcoding `/ductus` here names nothing they have, and the chain never starts.
- **State B wiring only — the deferred-restart set** — do **not** abort. The binary is on disk, the CLI is permission-seeded, and every remaining step's primitives are reachable as `{pointer-path} <primitive>`, so stopping here would defer the entire run to another session for nothing. Continue to **Collect Project Inputs** and carry the wiring notice to the **Closing restart**.
- **Both empty** — no restart is needed at all. Proceed to **Collect Project Inputs**. (ductus detection resolved to State A, and the self-update check saw `current` / `no installed copy` / `pinned-divergent` for every selected agent.)

On the **stale `ductus.md`** branch, everything past the pre-flight phase is skipped. That set is — **Collect Project Inputs**, **Pre-run Migrations**, **Project Configuration**, the **Archive fetch and extract**, **Frontmatter Migration**, **Shared Files**, **Per-Agent Scaffolding**, **Security Audit**, and **Post-Scaffolding Output**. Five of those — **Pre-run Migrations**, **Frontmatter Migration**, **Security Audit**, **Post-Scaffolding Output**, and (with the rest of the tail) **Hook Installation** — are in the archive half and are skipped here for the stronger reason that this branch never fetches the archive that carries them. **This list names which sections are skipped, not the order they run in** — read as a sequence it would put **Pre-run Migrations** ahead of the **Archive fetch and extract**, which is impossible: that section's own step 1 reads `framework/migrations.toml` *from the fetched archive*, so it necessarily runs after extraction. The execution order is the numbered walker in §Instructions, which fetches at step 2 and extracts at step 3. On the **State B** branch none of it is: the run proceeds through all of it via the CLI and stops only at the **Closing restart**. The only writes performed are the additive **Permission Setup** entries, any recorded source (**Recording the source**), any per-stale-agent `ductus.md` overwrite, and any ductus wiring plus its permission entries. Because input collection now lives past this point, an aborted run never prompts the user for the project name, description, or languages — they are asked exactly once, in the session that proceeds to scaffold. The next `/ductus` run in a new session sees ductus live (or absent) and every selected agent `current` (or `no installed copy`), and proceeds normally without abort.

## Collect Project Inputs

The Pre-flight Phase has passed (nothing in the pending-restart set), so this run will proceed to scaffold. **Only now** — never before the Pre-flight Phase — resolve the project inputs, so an abort can never discard answers the user just typed.

The **active config file** (write policy: §Project Configuration) is the persistent home for these answers. Resolve each input from the first available source and **prompt only for what is still missing**:

1. **Project name** — from `$ARGUMENTS` (a single non-flag word, per §Inputs), else `[project] name` in the config file (else `[host] project` for configs predating the `[project]` table), else prompt. Used for `{project}` substitution and command directory naming.
2. **Project description** — from `[project] description` in the config file, else prompt. Used for AGENTS.md.
3. **Primary language(s)** — from `[project] languages` in the config file, else prompt. Used for .gitignore language patterns.
4. **Rule surfaces** — from `[rules] surfaces` in the config file, else prompt ("Which rule surfaces does this project need? backend / frontend / both"). Recorded as a list with members in `{backend, frontend}` ("both" records both). Selects which rule files `/ductus` installs (§Shared Files) and which `/ductus:review` enforces (`review.md` §Behavior step 5). When the recorded surfaces exclude a surface that `[project] languages` implies (e.g., a frontend language is listed but `surfaces` omits `frontend`), emit one advisory line and honor the explicit value. **Validate a present value before using it** (degenerate configs fail fast per `CFG-ENV-003`, never silently ignored): the **empty list** (`surfaces = []`) is valid and means cross-only — install only `*-cross.md`, no surface-suffixed files — and is distinct from the key being unset (which derives/installs all); an **unrecognized member** outside `{backend, frontend}` (a typo, or `"cross"` — cross-cutting files are not a selectable surface) halts with `/ductus: invalid [rules] surfaces member "<value>" — accepted members are "backend" and "frontend" (use [] for cross-only; -cross.md files always apply)`, and a list mixing valid and invalid members fails on the invalid one; a **non-list value** (a bare string) halts with `/ductus: [rules] surfaces must be a list of strings, got <type>`.
5. **Spec-root directory** — from `[paths] specs-root` in the config file, else prompt ("What should the spec-root directory be named?") **defaulting to `specs`**. Names the top-level directory that holds every ductus artifact (feature dirs, `inbox.md`, `rules/`, shared docs). The prompt lives **only** in `/ductus` — no other command asks for it — and when the key stays unset every command and the runtime default to `specs`, so an adopter who never sets it sees unchanged behavior (spec [040](https://github.com/stonean/ductus/blob/main/specs/040-configurable-specs-dir/spec.md)). **Validate a present or entered value before using it** (fail fast — a value that breaks path resolution is never silently accepted): a name that is empty or contains any character outside `[A-Za-z0-9_-]` (a path separator, `.`/`..`, or other punctuation) halts with `/ductus: invalid [paths] specs-root "<value>" — must be a single directory name using only letters, digits, '-', and '_'`. Two **non-blocking** notices after a valid value is chosen: when the chosen directory **already exists on disk and is not a ductus spec root** (no `inbox.md`, no numbered `NNN-*` subdirs), emit one line naming it and proceed — it may be a sibling framework's directory (e.g. RSpec's `spec/`), and the operator's choice is honored after the warning; when the configured `specs-root` is **absent on disk but a different ductus-shaped directory exists**, emit a one-line half-finished-rename notice rather than silently scaffolding a new empty tree.

On a routine re-run (update mode) the config file already carries all five, so this step prompts for nothing. On a first scaffold it prompts for whatever is missing, then **persists the three project inputs into the active config file's `[project]` table** (`name`, `description`, `languages`), **the rule surfaces into the `[rules]` table** (`surfaces`), **and the spec-root into the `[paths]` table** (`specs-root`; see §Project Configuration), preserving every other section, so the next run — and the session after any State B / stale-`ductus.md` restart — reads them back instead of re-asking. `host.project` continues to be written from `project.name` as the runtime's slash-command namespace.

When prompting (AskUserQuestion), every question **must** include an `options` array with 2–4 example choices (the user can always select "Other" for custom input):

- **Project name** — example options: the current directory name, `my-service`.
- **Project description** — example options: `A new microservice`, `CLI tool for X`.
- **Primary language(s)** — comma-separated list. Example options: `Go`, `Python`, `Node`, `Go, Python`.
- **Rule surfaces** — example options: `backend`, `frontend`, `both`.
- **Spec-root directory** — example options: `specs` (the default), `governance`, `design`.

Validate the project name: must be lowercase, alphanumeric, and hyphens only. If invalid, reject with: "Project name must be lowercase, alphanumeric, and hyphens only."

## Project Configuration

`.ductus/config.toml` is the project's configuration and persisted-decisions store. Readers fall back through the earlier locations while it is absent — `.govern/config.toml` (042-era) then the repo root `.govern.toml` (pre-042) — and the newest existing file wins when more than one is present (specs 042, 049). If the file exists, read it before processing the file manifest. The file is optional — if it does not exist, use default behavior for every key. If the file exists but is malformed (TOML parse error), abort the run with a clear error rather than silently proceeding.

**Write policy — the `/ductus` migration is the sole cutover (spec 042).** Every config write in this procedure (the `[host]` and `[source]` managed blocks, the `[project]`/`[rules]`/`[paths]` input persistence, `[migrations].last_applied`) and every session write targets the **active file**: the newest tier that exists — `.ductus/`, else `.govern/`, else the repo root — and the `.ductus/` file for a fresh project where none exists. No write outside the directory migrations ever creates `.ductus/config.toml` while an older config lingers — that partial file would win on read and strand the legacy file's other sections. The migration moves the whole file as one unit; the runtime's `config_path_for_write` / `session_path_for_write` resolvers are the canonical statement of this rule, and `write-session` applies it on every session write.

The file is a flat collection of top-level sections. There is no umbrella namespace; each section is keyed to the thing it governs. The sections that may appear in the config file:

```toml
# ductus (host)
[host]
# `project` only — the team-shared slash-command namespace. The per-contributor
# `cli-config-dir` lives in the gitignored `.ductus/session.toml` (teammates may
# use different agents), never here.
project = "my-service"

# ductus (source)
[source]
# Where every /ductus fetch back into the ductus repository comes from:
# "main", or a release tag such as "ductus-v0.55.0". Absent means the latest
# release. Written by /ductus from --ref; --ref=latest removes the key.
ref = "main"

[project]
# The inputs /ductus collects (§Collect Project Inputs), persisted so re-runs
# and post-restart sessions read them back instead of re-prompting. This table
# is the source of truth for the answers; host.project below is the derived
# slash-command namespace, written from project.name.
name = "my-service"
description = "A new microservice"
languages = ["Go", "Python"]

[rules]
# Which rule surfaces /ductus:review enforces and /ductus installs. A list with
# members in {"backend", "frontend"}; full-stack lists both. "cross" is not a
# member — cross-cutting (-cross.md) rule files always apply. The empty list
# ([]) is valid and means cross-only (only -cross.md), distinct from the key
# being unset. Unset means "derive": /ductus:review falls back to stack detection
# and /ductus installs every rule file (pre-033 behavior). An unrecognized
# member or a non-list value fails fast. Collected by /ductus (§Collect
# Project Inputs); read by /ductus:review (§Behavior step 5).
surfaces = ["backend"]

[paths]
# The top-level directory that holds every ductus artifact — feature dirs,
# inbox.md, rules/, and shared docs. Defaults to "specs" when unset, so an
# adopter who never sets it sees byte-for-byte unchanged behavior. Set it to
# rename the tree (e.g. to avoid colliding with RSpec's spec/). A single
# directory name using only letters, digits, '-', and '_'. Collected by
# /ductus (§Collect Project Inputs); resolved by every command and the runtime
# (spec 040). When unset, all of them default to "specs".
specs-root = "specs"

[pinned]
# Files listed here use 'skip' instead of 'update'.
# Use destination paths (after placeholder resolution).
files = [
  ".claude/commands/myapp/implement.md",
  ".ductus/constitution.md",
]

[migrations]
# Slug of the newest pre-run migration applied. Bootstrap runs only entries
# newer than this (see §Pre-run Migrations). Absent section means "no
# migrations applied" — bootstrap runs every active entry. Maintained by
# /ductus; do not edit by hand.
last_applied = "rule-files-relocate"

# Consumed by /ductus:clarify, /ductus:plan and /ductus:analyze (spec 063),
# through the check-artifact-size primitive. The size, in bytes, above which a
# spec artifact (spec.md, plan.md, tasks.md, data-model.md, each scenario) may
# not be read in one call by an agent's file reader; each command warns on
# every artifact over it and names the fixes. A whole number of bytes, at
# least 1; unset means 50000. Raise it for a host with a larger read cap, lower
# it for a smaller one or for token-dense artifacts. Any other value is
# reported and the default used. There is no value that switches the warning
# off. Listed here for schema reference; uncomment and edit to use.
#
# [artifacts]
# read-size-bytes = 50000

# Consumed by /ductus:review (not /ductus itself). Excludes rule files from
# /ductus:review's selection regardless of stack detection. The `reason` field
# is mandatory (trimmed length ≥ 16 Unicode codepoints) and is the audit
# trail for the override. Listed here for schema reference; uncomment and
# edit to use.
#
# [[review.disabled-rule-files]]
# file = "accessibility-frontend.md"
# reason = "Internal admin UI — WCAG AA enforcement deferred to Q3"
#
# [[review.disabled-rule-files]]
# file = "api-backend.md"
# reason = "Pre-OpenAPI; revisit after schema lands (PROJ-1234)"

# Shared constitutions (spec 055) — governance documents an organization owns,
# so a rule is stated once and every one of its projects receives it. Alias-keyed,
# and more than one may be registered. `repo` is identity and navigation only and
# is NEVER fetched; the local `path` is the only state read. Listed here for
# schema reference; uncomment and edit to use.
#
# [constitutions.acme]
# repo = "https://github.com/acme/governance"
# path = "../governance"
# description = "Acme engineering house rules"
```

`host.project` — the project's slash-command namespace, written by `/ductus` into a managed block (`# ductus (host)` line-prefix marker) in committed `.ductus/config.toml` on every run (idempotent — re-runs update rather than append). The per-contributor `cli-config-dir` (the agent's config-dir name) is **not** committed: teammates on one project may each use a different agent, so `/ductus` writes it to the gitignored `.ductus/session.toml` instead (§Instructions step 7). The runtime reads `project` from `.ductus/config.toml` and `cli-config-dir` from the session file at `ductus exec` time to resolve the installed command file — `{cli-config-dir}/commands/{project}/<name>.md` for `claude-style`, then `{cli-config-dir}/command/{project}/<name>.md` for `opencode`, then `{cli-config-dir}/prompts/{project}-<name>.md` for `pi` (§Derived values **Command/skill path**; the runtime tries all three, so any layout resolves); all three fall back to `.claude` / the repo directory basename when absent. Adopters whose layout matches the defaults (this repo, anyone on Claude Code with the conventional `.claude/commands/<project>/`) never observe the difference; Auggie / OpenCode adopters and anyone with a non-standard layout do.

`source.ref` — the source every fetch back into the `ductus` repository names (§Pre-flight Phase → **Source resolution**): `main`, or a `ductus-v<MAJOR>.<MINOR>.<PATCH>` release tag. Absent — or a bare `[source]` table — means the latest release, and that is the default. Written into a managed block (`# ductus (source)` line-prefix marker) by a run given `--ref=main` or a tag, rewritten without `ref` by `--ref=latest`, and never written by the installer, which reads and writes no project configuration (**Recording the source**). It is committed rather than per-contributor because the framework files `/ductus` scaffolds are committed, so the release a project is on is a fact the whole team shares. A value outside that grammar halts the run naming it. Change it with `--ref` rather than by hand; the full schema is declared in [`specs/061-updates-track-the-latest-release-tag/data-model.md`](https://github.com/stonean/ductus/blob/main/specs/061-updates-track-the-latest-release-tag/data-model.md).

`project.name`, `project.description`, and `project.languages` — the project inputs collected at §Collect Project Inputs (name; one-line description for AGENTS.md; primary languages for .gitignore patterns), written into the `[project]` table additively (preserving every other section) and read back on every subsequent run so the inputs are asked at most once. `[project]` is the source of truth for the answers; `host.project` is written from `project.name` as the runtime's slash-command namespace (the derived runtime view of the same value), so the two cannot diverge. Editing a `[project]` value re-runs the corresponding scaffold step with the new value on the next `/ductus` — the documented way to rename a project or change its languages. The table is host-side state (the host gathers inputs before the runtime walks per §Instructions step 1), so it is written on every adoption path without a runtime primitive.

`rules.surfaces` — the rule surfaces the project enforces and installs (§Collect Project Inputs, item 4). A list with members in `{backend, frontend}`; `-cross.md` rule files are unconditional and not selectable members. When unset, `/ductus` installs every rule file and `/ductus:review` derives the surface from stack detection (pre-033 behavior). When set, the **Shared Files** manifest pass installs only the rule files whose suffix matches a listed surface plus every `*-cross.md`, and `/ductus:review` enforces only those (`review.md` §Behavior step 5). The **empty list** (`surfaces = []`) is a valid set value meaning **cross-only** — only `*-cross.md` is installed/enforced — and is distinct from the key being unset (the empty list declares "no surface rules"; unset means "derive"). A **degenerate value** fails fast per `CFG-ENV-003` rather than being silently ignored: an unrecognized member outside `{backend, frontend}` (a typo, or `"cross"`) and a non-list value both halt the command that reads the setting (`/ductus` here, `/ductus:review` in `review.md` §Behavior step 5), naming the offending value or type. Editing `surfaces` takes effect on the next `/ductus`: newly-listed surfaces are installed, and rule files for a removed surface are **left in place** (not deleted — they are not in `enforce-directories`), they simply stop receiving updates.

`pinned.files` — any file listed that would normally use `update` strategy is treated as `skip` instead. Report pinned files in the post-scaffolding summary.

`migrations.last_applied` — slug of the newest pre-run migration applied to this project, written by `/ductus` after each successful migration in §Pre-run Migrations. Absent section means "no migrations applied"; bootstrap runs every active entry on the next run. Adopters should not edit this field by hand — the registry in `framework/migrations.toml` and the per-entry procedure files in `framework/migrations/{id}.md` are the authoritative sources.

`review.disabled-rule-files` — array-of-tables consumed by `/ductus:review` at rule-file selection time (see its §Inputs and §Behavior step 5). `/ductus` does not read this key; it is documented here so adopters see the full `.ductus/config.toml` schema in one place.

`constitutions.<alias>` — the shared-constitution registry (spec 055). Each `[constitutions.<alias>]` table names a `repo` (the canonical repository URL, recorded verbatim as identity and navigation only — **never fetched**) and a local `path` (the checkout, relative to the repo root or absolute; `..` is permitted because a sibling checkout is the normal case), plus an optional `description` saying what the source governs — no resolution behavior depends on it, and it is rendered wherever `ductus` names the source (`/{project}:target`'s loaded and skipped reports, a review's **Unexamined governance** section), so an alias is never the only thing an operator has to go on. The document read from a resolved checkout is `{path}/constitution.md`; nothing else in the checkout is read, and a registered checkout's own config is never read, so registration is not transitive. More than one entry may be registered, and every registered entry is loaded, in alias order. Where a registered constitution and the shipped one disagree, the constitution's §governance-precedence says which governs — and that nothing enforces it. The full schema is declared in [`specs/055-shared-constitution/data-model.md`](https://github.com/stonean/ductus/blob/main/specs/055-shared-constitution/data-model.md).

**Validating the registry.** `/ductus` validates each entry when it reads the config, and the two failure modes are deliberately different severities. A **malformed entry halts** the run per `CFG-ENV-003`, naming the offending alias and field: an alias that is not a bare TOML key (letters, digits, hyphens, underscores — no whitespace, dots, or quotes), a `repo` that is not URL-shaped (a scheme and a host — an scp-style git address such as `git@github.com:acme/gov.git` carries no scheme and is written `ssh://git@github.com/acme/gov.git` instead), or a `path` that is empty, whitespace counting as empty because it is the same operator mistake and an empty `path` resolves to the repository root, where the run would report the operator's own config error as a missing document in their checkout. A **`path` that does not resolve only warns** — `/ductus` emits a one-line notice naming the alias and continues, exactly as `/{project}:link` treats a missing service checkout ("a missing checkout is the valid `not-checked-out` state, surfaced at resolution time, not a config error"). The asymmetry is the point: a malformed entry is a mistake in the project's own committed config and is always wrong, while an unresolved checkout is a machine-local state that is correct for any contributor who has not cloned the governance repository yet, and blocking on it would make the pipeline a hard dependency on someone else's repo state. Registration itself is a hand-edit — there is no `/{project}:link`-style command for it, matching that command's own rule that edits to an existing entry stay hand-edits.

The full schema (allowed values, case-insensitive matching, empty-section behavior, future-section guidance) is declared in [`specs/019-config-decisions/data-model.md`](https://github.com/stonean/ductus/blob/main/specs/019-config-decisions/data-model.md).

## File Fetching

Files from the `ductus` repo are sourced from a single archive download, extracted into the temp directory established during the **Pre-flight Phase**, and resolved as local paths for the rest of the run. Per-language `.gitignore` patterns from `github.com/github/gitignore` are **not** part of this archive — they remain separate `curl` calls (see the **.gitignore** subsection of **Shared Files** below).

This section runs only after the **Pre-flight Phase** passes — that is, once **Pre-flight abort** has found the **pending-restart set** empty (no stale `ductus.md`). A State B run reaches it too: its wiring sits in the **deferred-restart set**, which does not stop the run. On a pre-flight abort, the archive is never fetched.

**State A reminder:** the archive fetch/extract and the manifest passes below are primitive-backed. In a State-A run (ductus live), call the `fetch-archive`, `extract-archive`, `apply-manifest`, and `enforce-manifest` tools; a State-B run reaches the same four through the CLI at `{pointer-path}`, because acquisition has already put the binary on disk. Either way the `curl`/`tar` blocks shown specify what those primitives do, not commands to execute (see **§Pre-flight Phase → State A — runtime live this session**). The per-language `.gitignore` `curl` is *not* primitive-backed and runs as shown in every state.

### Archive fetch and extract

Issue exactly one `curl` against GitHub's archive host, at the ref **Source resolution** settled, downloading into the temp directory established during the pre-flight phase:

```text
curl -fsSL "https://codeload.github.com/${DUCTUS_REPO:-stonean/ductus}/tar.gz/{archive-ref}" \
  -o {tempdir}/framework.tar.gz
```

This is the direct `codeload.github.com` endpoint — the target that `https://github.com/${DUCTUS_REPO:-stonean/ductus}/archive/{archive-ref}.tar.gz` 302-redirects to. Fetch it directly: the redirect form lands the command on a **new host mid-flight**, which some hosts (e.g. Antigravity) gate with a permission prompt even when a `curl` allow is pre-granted, because the grant matched the original host, not the redirect target. The direct URL has no redirect, so the bootstrap seed's `curl` pre-grant (`command(curl)` / `Bash(curl *)` / the Auggie `^curl` regex matcher) actually covers it.

After fetching:

1. Extract the archive into the existing temp directory: `tar -xzf {tempdir}/framework.tar.gz -C {tempdir}`.
2. Derive the framework root, `{framework-root}`: **the single top-level directory the extraction produced** — on the runtime path, `{tempdir}/` joined with the first path component every entry in `extract-archive`'s `files` shares (those paths are relative to the extraction's destination); on the markdown path, the one directory `tar` created in `{tempdir}`. Treat it as the local mirror of the `ductus` repo for the rest of the run; the framework files live at `{framework-root}/framework/...`. GitHub names that directory after the ref — `ductus-main/` for `main`, `ductus-ductus-v0.55.0/` for that tag — and the rule is GitHub's, so it is read from the extraction rather than predicted from the ref, where a restated rule would break silently the day GitHub changed it.

If the fetch or extraction fails — non-zero exit from `curl` or `tar`, or an extraction that produced zero or several top-level directories, so that no single `{framework-root}` exists — abort the run with this error and do not continue scaffolding:

> Failed to fetch or extract the `ductus` archive ({reason}). Re-run after checking network connectivity, or report this if it persists.

A missing archive means **every** manifest entry would be missing, so partial scaffolding is impossible — the abort is the correct behavior. The pre-flight phase has already completed by this point, so a stale `ductus.md` would have already triggered the pre-flight abort earlier; a freshly-wired ductus does not abort and reaches this section through the CLI.

### Per-file resolution

For each manifest entry below (in **Shared Files** and **Per-Agent Scaffolding**):

1. Compute the local source path: `{framework-root}/{source-path}`.
2. If the local source path does not exist — the file was renamed, removed upstream, or the manifest is out of sync — warn `Source not found in archive: {source-path}; skipping.` and continue with the remaining entries. This preserves the "do not abort on a single fetch error" guarantee at the per-entry level, even though the archive itself is fetched once.
3. Apply the entry's strategy (`update`, `create`, `skip`, `merge`, `pinned`) using the local file as the new content. For `update` strategy, compare the local file against the existing destination file; only overwrite and report as "updated" if the content differs. If the content is identical, report as "unchanged" (or omit from the summary). Same semantics as before — no network round-trip per file.
4. Apply placeholder substitution after reading the local source, before writing to the destination. Same rules as documented in **Placeholder Substitution** below, including the `ductus.md` self-install exception that keeps `{project}` and `{cli-config-dir}` literal.

### Cleanup

`/ductus` does not delete the temp directory. The path is logged in the post-scaffolding summary (and, on abort, in the error message) so the user can inspect it if needed. Both macOS (`/var/folders/.../T/`) and Linux (`/tmp` on systemd-tmpfiles distros) sweep their temp directories automatically; a few hundred KB of extracted files waiting for the next sweep is acceptable in exchange for not granting an `rm -rf` permission to the bootstrap.

The leftover directory is for inspection only — the next `/ductus` run creates its own fresh temp directory via `mktemp` and never reuses a prior extract.

## The archive half

The sections that follow the archive fetch do not live in this file. They ship
**in** the archive this run just extracted, at
`{framework-root}/framework/bootstrap/ductus-procedure.md` — the framework
root §Archive fetch and extract derived above and called the local mirror of
the `ductus` repo for the rest of the run. Read them from there when
the run reaches them: **Pre-run Migrations**, **Frontmatter Migration**,
**Security Audit (brownfield)**, **Hook Installation**, **What This Command Does
NOT Do**, **Edge Cases**, **Post-Scaffolding Output**, **Idempotency**, and
**Directory Creation**.

They are held there rather than here because none of them can run before the
archive exists, and this file is what an adopter curls, installs into every
agent, loads into context at every invocation, and byte-compares on every run.
That half needs no self-update check of its own: it is re-fetched on every run,
so it cannot be stale.

A run that halts at **Pre-flight abort** never reaches the fetch, so it never
needs that file — which is the same set of sections that branch already skips.

## Shared Files

These files are scaffolded **once per `/ductus` invocation**, regardless of how many agents are selected. They are unaffected by the agent registry.

**Invoking `apply-manifest` on the State B (CLI) path.** `entries`, `pinned`, and `substitutions` are arrays and maps of objects, so they are not clap flags — on the MCP and interpreter paths they arrive through the JSON context. From the CLI, write each to a temp file and pass its path: `{pointer-path} apply-manifest --source-root {staging} --target-root . --entries-json {file} --pinned-json {file} --substitutions-json {file}`. **Slash command cleanup**'s `enforce-manifest` takes `--expected-json` and `--pinned-json` the same way. An unreadable or malformed file is an **error**, never an empty default: an empty manifest is a legal manifest, so a silent fallback would copy nothing and report success — the adopter would end the run with none of the shared files and no indication why.

**Rule-file surface filter.** The `framework/rules/*.md → specs/rules/*.md` entries below are filtered by `[rules] surfaces` (§Project Configuration) before the manifest is applied: an entry is kept when its suffix matches a configured surface (`*-backend.md` for `backend`, `*-frontend.md` for `frontend`), and every `*-cross.md` entry is kept unconditionally. When `surfaces` is the **empty list** (`[]`), no surface-suffixed entry matches, so only the `*-cross.md` entries are kept (cross-only). When `surfaces` is unset, all rule files are kept (pre-033 behavior). (A degenerate `surfaces` value — an unrecognized member or a non-list — has already halted the run at §Collect Project Inputs item 4 before this filter runs.) Entries the filter omits are simply not applied — never pruned — so a rule file already on disk for a now-unconfigured surface is left in place (rule files are not in `enforce-directories`); it just stops receiving updates.

### `ductus`-owned shared files (strategy: update)

| Source Path | Destination Path |
| --- | --- |
| `framework/constitution.md` | `.ductus/constitution.md` |
| `framework/rules/accessibility-frontend.md` | `specs/rules/accessibility-frontend.md` |
| `framework/rules/api-backend.md` | `specs/rules/api-backend.md` |
| `framework/rules/concurrency-backend.md` | `specs/rules/concurrency-backend.md` |
| `framework/rules/configuration-cross.md` | `specs/rules/configuration-cross.md` |
| `framework/rules/observability-backend.md` | `specs/rules/observability-backend.md` |
| `framework/rules/performance-backend.md` | `specs/rules/performance-backend.md` |
| `framework/rules/performance-frontend.md` | `specs/rules/performance-frontend.md` |
| `framework/rules/quality-cross.md` | `specs/rules/quality-cross.md` |
| `framework/rules/reliability-backend.md` | `specs/rules/reliability-backend.md` |
| `framework/rules/security-backend.md` | `specs/rules/security-backend.md` |
| `framework/rules/security-frontend.md` | `specs/rules/security-frontend.md` |
| `framework/bootstrap/hooks/ductus-pre-commit` | `.githooks/ductus-pre-commit` |
| `.markdownlint-cli2.jsonc` | `.markdownlint-cli2.jsonc` |
| `framework/templates/spec/spec.md` | `specs/templates/spec.md` |
| `framework/templates/spec/plan.md` | `specs/templates/plan.md` |
| `framework/templates/spec/tasks.md` | `specs/templates/tasks.md` |
| `framework/templates/spec/data-model.md` | `specs/templates/data-model.md` |
| `framework/templates/spec/research.md` | `specs/templates/research.md` |
| `framework/templates/spec/scenario.md` | `specs/templates/scenario.md` |

### Project-specific shared files (strategy: create)

| Source Path | Destination Path |
| --- | --- |
| `framework/templates/project/system.md` | `specs/system.md` |
| `framework/templates/project/errors.md` | `specs/errors.md` |
| `framework/templates/project/events.md` | `specs/events.md` |
| `framework/templates/project/inbox.md` | `specs/inbox.md` |
| `framework/bootstrap/hooks/pre-commit` | `.githooks/pre-commit` |

### Shared files with conflict handling

**AGENTS.md** (strategy: skip) — if it exists, leave it alone. If not, fetch `framework/templates/project/agents.md` from the `ductus` repo and copy it as `AGENTS.md`, substituting `{project-name}` with the project name and `{One-line project description.}` with the project description.

**CLAUDE.md** (strategy: skip, `claude-style` only) — written only when at least one selected agent is `claude-style`. If it exists, leave it alone. Otherwise, when a `claude-style` agent is selected, fetch `framework/templates/project/claude-md.md` from the `ductus` repo and copy it as `CLAUDE.md`. `claude-style` agents read `CLAUDE.md` natively (see each row's `rules_file_note`); the `antigravity`, `opencode`, and `pi` layouts read `AGENTS.md` natively and do not need `CLAUDE.md`, so an **Antigravity-only**, **OpenCode-only**, or **Pi-only** adoption ships no `CLAUDE.md`. (`AGENTS.md` is still written for every adoption, as below.)

**.gitignore** (strategy: merge) — install or update a framework-managed block delimited by a `# ductus` line preamble, then dedup any adopter-area copies of canonical patterns. Mirrors the runtime `merge-managed-block` contract (line-prefix style, marker `ductus`):

1. Fetch `framework/templates/project/gitignore` from the `ductus` repo. This is the **canonical block** — including its blank-line-separated subsections.
2. If `.gitignore` does not exist, create it with `# ductus\n{canonical-block}\n`. Skip to step 5 for language patterns.
3. If `.gitignore` exists and contains a `# ductus` line preamble, replace the managed region (the `# ductus` line through the rest of the block — note the canonical block itself contains blank lines between subsections, so do not stop at the first interior blank) with `# ductus\n{canonical-block}\n`. If no `# ductus` line is present, append `# ductus\n{canonical-block}\n` after the existing content, separated by exactly one blank line.
4. **Dedup pass (canonical-block wins).** After the managed block is in place, scan the rest of the file (everything outside `# ductus` through the canonical block's end) and remove any non-blank, non-comment line that string-equals a non-blank, non-comment line inside the canonical block. Adopter-area blank lines and comment lines are preserved untouched even when they happen to share text with a canonical pattern. This collapses duplicates that an adopter (or another command) pasted above or below the marker; the canonical copy inside `# ductus` is the surviving one.
5. For each primary language provided by the user, fetch from `https://raw.githubusercontent.com/github/gitignore/main/{Language}.gitignore` and append below a `# {Language}` comment header. If the file is being re-merged on a subsequent run and a `# {Language}` section is already present, leave it alone — language sections, once written, are adopter territory.

**Shared constitution imports** (strategy: merge, layout-derived) — install or update a framework-managed block in each selected layout's **native rules file**, listing the shared constitutions registered in `[constitutions.*]` (§Project Configuration, spec 055).

1. Invoke `resolve-constitutions` to get the registered entries split into `loaded` and `skipped`.
2. Determine the target file from the Agent Registry's derived **Native rules file** value — `CLAUDE.md` for `claude-style`, `AGENTS.md` for the `antigravity` and `opencode` layouts. When both kinds of agent are selected, both files get a block. The file itself is strategy `skip` and is never overwritten; only the managed region is rewritten.
3. Invoke `merge-managed-block` against that file with `marker-style: "html-comment"` and `marker: "ductus:constitutions"`, supplying one line per entry in `loaded`, in the order returned:
   - **`claude-style`** → `@import {document-path}`, matching the `@import` lines the template already carries.
   - **`antigravity` / `opencode`** → `- See [{alias}]({document-path})`, a plain link. These layouts read `AGENTS.md` and have **no import directive**, so a link is the whole of what the file can express there.
4. When `loaded` is empty the block is written **empty** — present but carrying no lines — rather than omitted. An empty managed region and an absent one differ: the first says `/ductus` looked and found nothing registered, the second says nothing ran.
5. Report each `skipped` entry by alias and reason (`not-checked-out`, `no-constitution-document`). A skipped source is **not** written into the block: the block lists what an agent can actually read, and listing an unreadable path would put a broken import in an adopter's rules file.

**The block is navigation, not the loading mechanism.** `/{project}:target` step 4 is what puts a shared constitution's rules in effect, for every agent and every layout, by reading the documents `resolve-constitutions` resolves. The block exists so the documents are also reachable from the file a contributor opens — and so `claude-style` agents pick them up ambiently. An adopter whose rules file has no block still gets the rules, because the command loads them.

## Per-Agent Scaffolding

For each selected agent (in registry row order), run these steps with `{config_dir}` resolved to the agent's value and `{key}` to the agent's key.

The steps below describe the **`claude-style`** layout. For an agent whose registry `layout` is **`antigravity`**, apply **### Antigravity layout** below in place of **### Slash commands** and **### Slash command cleanup**. The `ductus` self-install, the **Pre-flight Phase**, the **Post-Write Integrity Check**, and **Placeholder Substitution** each carry their own `layout: antigravity` branch in their own sections.

For an agent whose `layout` is **`opencode`**, apply **### OpenCode layout** below in place of **### Slash commands** and **### Slash command cleanup**. OpenCode's installer is a **verbatim markdown file** (no skill wrapper), so the `ductus` self-install, **Self-update check**, **Post-Write Integrity Check**, and **Placeholder Substitution** follow the **`claude-style`** path — with the command directory `command/` (singular) and `{cli-config-dir}` resolving to `.opencode`.

For an agent whose `layout` is **`pi`**, apply **### Pi layout** below in place of **### Slash commands** and **### Slash command cleanup**. `{config_dir}` resolves to `.pi`; Pi discovers prompt templates under `{config_dir}/prompts/` — flat, non-recursive, the filename (minus `.md`) is the command name. The `ductus` self-install, **Self-update check**, **Post-Write Integrity Check**, and **Placeholder Substitution** each carry their own `layout: pi` branch in their own sections.

### Slash commands (strategy: update)

Fetch each command template and copy it into `{config_dir}/commands/{project}/`. In each copied file, replace `{project}` with the user-provided project name and `{cli-config-dir}` with `{config_dir}`.

| Source Path | Destination Path |
| --- | --- |
| `framework/commands/amend.md` | `{config_dir}/commands/{project}/amend.md` |
| `framework/commands/clarify.md` | `{config_dir}/commands/{project}/clarify.md` |
| `framework/commands/consolidate.md` | `{config_dir}/commands/{project}/consolidate.md` |
| `framework/commands/fold.md` | `{config_dir}/commands/{project}/fold.md` |
| `framework/commands/groom.md` | `{config_dir}/commands/{project}/groom.md` |
| `framework/commands/help.md` | `{config_dir}/commands/{project}/help.md` |
| `framework/commands/implement.md` | `{config_dir}/commands/{project}/implement.md` |
| `framework/commands/link.md` | `{config_dir}/commands/{project}/link.md` |
| `framework/commands/log.md` | `{config_dir}/commands/{project}/log.md` |
| `framework/commands/plan.md` | `{config_dir}/commands/{project}/plan.md` |
| `framework/commands/prune.md` | `{config_dir}/commands/{project}/prune.md` |
| `framework/commands/review.md` | `{config_dir}/commands/{project}/review.md` |
| `framework/commands/specify.md` | `{config_dir}/commands/{project}/specify.md` |
| `framework/commands/status.md` | `{config_dir}/commands/{project}/status.md` |
| `framework/commands/target.md` | `{config_dir}/commands/{project}/target.md` |
| `framework/commands/analyze.md` | `{config_dir}/commands/{project}/analyze.md` |
| `framework/bootstrap/configure/{key}.md` | `{config_dir}/commands/{project}/configure.md` |

The configure row uses the agent-specific source `framework/bootstrap/configure/{key}.md` and writes it as the canonical `configure.md` in the project's command directory.

### Slash command cleanup

After processing the slash command manifest above, list all `.md` files in `{config_dir}/commands/{project}/`. For each file that is **not** in the slash command manifest above and **not** listed in `.ductus/config.toml` `pinned.files`:

- Delete the file.
- Report it as "removed" in the post-scaffolding summary.

Files listed in `pinned.files` are never deleted — report them as "pinned (kept)" instead.

### Antigravity layout (`layout: antigravity`)

When the agent's registry `layout` is `antigravity`, the two subsections above (**Slash commands**, **Slash command cleanup**) are replaced by the skill-based equivalents below. `{config_dir}` resolves to `.agents`; Antigravity discovers dir-form skills under `{config_dir}/skills/`.

**Skills (strategy: update).** For each row in the slash-command manifest above — the sixteen `framework/commands/*.md` rows plus the `framework/bootstrap/configure/{key}.md` configure row — transform the source into a dir-form skill at `{config_dir}/skills/{project}-{name}/SKILL.md` (instead of copying to `{config_dir}/commands/{project}/{name}.md`):

1. Read the source markdown (frontmatter + body).
2. Write `{config_dir}/skills/{project}-{name}/SKILL.md` with frontmatter `name: {project}-{name}` and the `description:` carried from the source frontmatter, followed by the source body.
3. Substitute `{project}` and `{cli-config-dir}` in the body exactly as in the `claude-style` copy (`{cli-config-dir}` → `.agents`).

`{name}` is the command's base name (`specify`, `clarify`, …; the configure row's `{name}` is `configure`). The skills are invoked as `/{project}-{name}`.

**Rules (strategy: update).** Mirror **every** `*.md` file present in `specs/rules/` into `{config_dir}/rules/{name}.md`, so Antigravity loads them natively. The source is a **directory walk of what is on disk**, not the **Shared Files** manifest rows: the manifest ships `ductus`'s own rule files, but a project may author its own (constitution §rules Lifecycle), and those are discovered by the same walk `/{project}:review` and `/{project}:analyze` use. Mirroring only the manifest rows would leave a project's own rules enforced by the pipeline but absent from Antigravity's native loading — the one agent with a native rules dir would see a smaller rule set than every other agent. `ductus`-shipped files regenerate from `framework/rules/` on every `/ductus` run; project-authored files are mirrored as they stand. `specs/rules/` stays the pipeline-read location for every agent; `{config_dir}/rules/` is the Antigravity-native mirror. The `specs/rules/` write itself (in **Shared Files**) is layout-independent and unchanged.

**Rules mirror cleanup.** List the `*.md` files under `{config_dir}/rules/`. Delete any whose basename no longer exists in `specs/rules/` and is not listed in `.ductus/config.toml` `pinned.files`; report removals and pinned-keeps as for the skill cleanup below. Without this, a rule file deleted or renamed in `specs/rules/` keeps loading from the mirror — Antigravity would enforce a rule the pipeline has already dropped.

**Skill cleanup (replaces Slash command cleanup).** List the skill directories under `{config_dir}/skills/` whose name matches `{project}-*`. Delete any `{config_dir}/skills/{project}-{name}/` whose `{project}-{name}` is not produced by the skills manifest above and is not listed in `.ductus/config.toml` `pinned.files`; report removals and pinned-keeps as for the `claude-style` cleanup. Skill dirs outside the `{project}-*` namespace (and the `ductus` skill) are adopter/agent territory and are never touched.

### OpenCode layout (`layout: opencode`)

When the agent's registry `layout` is `opencode`, the two subsections above (**Slash commands**, **Slash command cleanup**) are replaced by the equivalents below. `{config_dir}` resolves to `.opencode`; OpenCode discovers markdown commands under `{config_dir}/command/` (singular), namespaced by subdirectory.

**Commands (strategy: update).** For each row in the slash-command manifest above — the sixteen `framework/commands/*.md` rows plus the `framework/bootstrap/configure/{key}.md` configure row — copy the source **verbatim** (frontmatter + body, no skill transform) to `{config_dir}/command/{project}/{name}.md` (instead of `{config_dir}/commands/{project}/{name}.md`). Substitute `{project}` and `{cli-config-dir}` (→ `.opencode`) in the body exactly as in the `claude-style` copy, and carry the `description` frontmatter as-is. `{name}` is the command's base name (the configure row's `{name}` is `configure`). The commands are invoked `/{project}/{name}` — OpenCode namespaces by subdirectory (verified: `command/ductus/specify.md` registers as command key `ductus/specify`).

**Command cleanup (replaces Slash command cleanup).** List the `.md` files under `{config_dir}/command/{project}/`. Delete any whose base name is not produced by the manifest above and is not listed in `.ductus/config.toml` `pinned.files`; report removals and pinned-keeps as for the `claude-style` cleanup. Files outside the `{project}/` subdirectory are adopter/agent territory and are never touched.

**Rules.** OpenCode reads `AGENTS.md` natively (via its `instructions` resolution) and the pipeline reads the shared `specs/rules/` directly — there is **no** native rules-dir mirror (unlike `antigravity`). Nothing extra to scaffold.

**MCP + permissions.** Both the `ductus` `mcp` block and the `permission` set live in the committed root `opencode.json` — seeded by §Permission Setup, wired by §ductus runtime detection (State-B `write-file`), and completed by `/{project}:configure`. See §Derived values and §MCP registration.

### Pi layout (`layout: pi`)

When the agent's registry `layout` is `pi`, the two subsections above (**Slash commands**, **Slash command cleanup**) are replaced by the equivalents below. `{config_dir}` resolves to `.pi`; Pi discovers prompt templates under `{config_dir}/prompts/` — flat, non-recursive, the filename (minus `.md`) is the command name. The flat directory is **shared** with the adopter's own prompt templates, so the namespace is the project name: files are named `{project}-{name}.md` and invoked `/{project}-{name}` (the same spelling as the antigravity layout, but a prompt template rather than a skill).

**Prompt templates (strategy: update).** For each row in the slash-command manifest above — the sixteen `framework/commands/*.md` rows plus the `framework/bootstrap/configure/{key}.md` configure row — copy the source **verbatim** (frontmatter + body, no skill transform) to `{config_dir}/prompts/{project}-{name}.md`. Substitute `{project}` and `{cli-config-dir}` (→ `.pi`) in the body exactly as in the `claude-style` copy, and carry the `description` and `argument-hint` frontmatter as-is; Pi's prompt-template loader honors those two keys and ignores all others (the `parity:` blocks pass through inert). The commands are invoked `/{project}-<name>` — Pi names by the flat filename, and the project-hyphen prefix is what keeps the shared directory collision-free across projects and against the adopter's own templates.

**Prompt-template cleanup (replaces Slash command cleanup).** List the `.md` files under `{config_dir}/prompts/` matching the glob `{project}-*.md` — **never** the default `*.md`: the flat directory is shared with the adopter's own prompt templates, and the default glob would delete them. Delete any matching file whose `{project}-{name}` is not produced by the manifest above and is not listed in `.ductus/config.toml` `pinned.files`; report removals and pinned-keeps as for the `claude-style` cleanup. Files outside the `{project}-*.md` namespace are adopter/agent territory and are never touched.

**Rules.** Pi reads `AGENTS.md` natively (loaded regardless of project trust) and the pipeline reads the shared `specs/rules/` directly — there is **no** native rules-dir mirror. Nothing extra to scaffold.

**Bridge (strategy: update — the one agent with a tool surface that is not MCP).** Copy `framework/bootstrap/pi/ductus-bridge.ts` **verbatim** (no `{project}` / `{cli-config-dir}` substitution — the bridge holds no placeholders, and the `keep-literals` masking used for the `ductus` file does not apply to it) to `{config_dir}/extensions/ductus.ts`. This is the `write-file` mechanism's target from §MCP registration: the extension wraps the runtime's own MCP server over stdio and re-registers every `ductus__<name>` tool. Pi has no MCP config file and no permission-gating settings file, so **nothing else is wired** for this agent — no `mcp` block, no permission merge (§Permission Setup is a no-op for Pi). The `.pi/` line in the framework-managed `.gitignore` block below covers the extension and the prompts (per-contributor scaffolding, regenerated by `/ductus`).

**Trust reminder.** Project-local `.pi/` resources load only after the project is trusted. A fresh checkout prompts on interactive start; non-interactive runs need `--approve` (or `defaultProjectTrust: "always"`) — the **completion message** carries this reminder whenever Pi is among the selected agents.

### Session state

The session state file lives at `.ductus/session.toml` (earlier locations — `.govern/session.toml`, then the pre-042 repo root `.govern.session.toml` — are read and written via the active-file rule until the directory migrations move it; specs 042, 049) — a single uniform path for every adopter, project-name-agnostic, gitignored, and **per-contributor**. It carries two things: the session target (feature, optional scenario, `set-at`), written on each `/{project}:target` (or its scenario sibling) invocation — this file is the shared default, which every target write sets, and a process with a session identity also keeps its own target under `.ductus/sessions/` (gitignored; §concurrent-features); and the contributor's `cli-config-dir`, written by `/ductus` at adoption (§Instructions step 7) — the one place an agent-specific value belongs, since teammates on one project may use different agents. Both are written by the runtime's `write-session` primitive (a target write preserves `cli-config-dir`; a host-config write preserves the target), or on the markdown-only path by the host's file-writing tool. Beyond this file, the only session state is those per-process targets; `cli-config-dir` is never per process.

### `ductus` self-installation (strategy: update)

Fetch `framework/bootstrap/ductus.md` and write it to the agent's `ductus` install path: `{config_dir}/commands/ductus.md` for `claude-style`, `{config_dir}/command/ductus.md` for `opencode`, `{config_dir}/skills/ductus/SKILL.md` for `antigravity`, or `{config_dir}/prompts/ductus.md` for `pi`. This is the same unified file the user is currently running, installed into every selected agent so the command is invokable from that agent on subsequent runs. For `antigravity`, wrap the body in `name: ductus` frontmatter (the dir-form skill); for `claude-style`, `opencode`, and `pi` the file is the verbatim `ductus.md`. The body keeps every placeholder literal (next paragraph).

In this file (and only this file), keep **every** placeholder literal — do **not** substitute anything. `{project}` and `{cli-config-dir}` must stay literal so `ductus` itself can read `$ARGUMENTS` and the per-agent config dir on each run; `{project-name}` and `{One-line project description.}` must stay literal because this file's prose *documents* those placeholders for the AGENTS.md template — substituting them would corrupt the documentation, not personalize a value.

After writing, run the **Post-Write Integrity Check** below.

## Placeholder Substitution

**The substitution map is keyed bare.** The keys below are written here as the placeholder *tokens they match in files* — `{project}`, `{cli-config-dir}` — because that is what an author scanning a template is looking for. The map `apply-manifest` takes is keyed by the token **without its braces**: `project`, `cli-config-dir`, `project-name`, `One-line project description.`. The primitive adds the braces itself to build what it searches for, so a braced key becomes `{{project}}` and matches nothing.

This is stated because the two forms genuinely disagree on the page and always did: this section writes placeholders braced, and the `keep-literals: ["project", "cli-config-dir"]` two sections above writes the same names bare — because one is naming tokens in files and the other is naming map keys. Nothing said which the map took. An adopter passed braced keys, every strategy ran, every file was written, and `apply-manifest` returned counts a correct run would return, while 370 literal placeholders shipped across the constitution, all 17 commands, 5 rule files and 2 templates — `/{project}:review` where the real command name belonged. The primitive now **rejects** a placeholder-shaped or empty key before it touches a file, and reports `substitutions-applied` (plus `entries-substituted` as its denominator) in the result, so an incomplete map is visible in the numbers even where an invalid one is impossible.

In every copied file (except each selected agent's installed `ductus` file — `{config_dir}/commands/ductus.md` for `claude-style`, `{config_dir}/command/ductus.md` for `opencode`, `{config_dir}/skills/ductus/SKILL.md` for `antigravity`, `{config_dir}/prompts/ductus.md` for `pi` — whose body keeps `{project}` and `{cli-config-dir}` as literal placeholders), replace:

- `{project}` with the user-provided project name (used in commands, README)
- `{project-name}` with the user-provided project name (used in AGENTS.md template)
- `{One-line project description.}` with the user-provided description
- `{cli-config-dir}` with the agent's `config_dir`

## Post-Write Integrity Check

After writing the agent's installed `ductus` file — whether via the **Pre-flight Phase** (stale-write path) or the **`ductus` self-installation** manifest step — verify it is well-formed. For `claude-style` (`{config_dir}/commands/ductus.md`), `opencode` (`{config_dir}/command/ductus.md`), and `pi` (`{config_dir}/prompts/ductus.md`), the file must start with a frontmatter block carrying a `description:` key, and the body after that frontmatter must start with `# ductus` — the installed copy is written verbatim, so the source's own frontmatter travels with it, and asserting against the first line alone fails on every correct write. For `antigravity` (`{config_dir}/skills/ductus/SKILL.md`), the file must start with a frontmatter block whose `name:` is `ductus`, and the body after that frontmatter must start with `# ductus`. If the check fails, the write was corrupted — report the error and re-read the source: `{tempdir}/ductus.md.upstream` for the self-update path, or `{framework-root}/framework/bootstrap/ductus.md` for the manifest path. Apply the check independently per agent.

## Re-Run Behavior

`/ductus` is idempotent and additive across agents:

- **Re-run with the same selection** — applies the manifest's `update` strategy to the agent's slash commands and refreshes shared files. `create`-strategy files are skipped if present. `skip`-strategy files are never overwritten.
- **Re-run adding a new agent** — scaffolds the new agent's tree from scratch alongside the existing one. The existing agent's command dir, settings, and session JSON are not touched.
- **Re-run removing an agent** — this command does not delete an agent's tree on its own. Removing an adopted agent is a manual `rm -rf {config_dir}` operation outside `/ductus`'s scope.
