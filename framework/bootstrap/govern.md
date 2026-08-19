---
description: Adopt or update ductus in an existing project.
argument-hint: "[project] [--agents=key1,key2,...] [--add-agent]"
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

> **For agent runtimes**: backticked primitive names in this section (`fetch-archive`, `extract-archive`, `apply-manifest`, `merge-managed-block`, `enforce-manifest`) map to MCP tools the [ductus runtime](https://crates.io/crates/ductus) exposes under bare `<primitive>` names (e.g., `fetch-archive`). Hosts wrap them with a server-name prefix taken from the agent's MCP registration (Claude: `mcp__ductus__fetch-archive`; Auggie: `mcp:ductus:fetch-archive`). When the server is registered for your session, **call the corresponding tool** for each step listed below — that is the deterministic path. When it is not registered, walk the markdown-only reference below (`tar -xzf`, `curl`, etc.) to produce the same result. The two paths share a contract; neither one wraps the other.

**Procedural fidelity.** Execute the steps below as written. The only confirmation prompts to issue are those the procedure specifies: project inputs (§Inputs), agent-selection prompts on `--add-agent` / first-run (§Agent Selection), and the registry-driven migration prompts (§Pre-run Migrations — outer "apply N pending migrations" prompt plus any per-entry inner prompts the procedure files specify). Do not stop to warn about uncommitted edits to update-strategy files, custom slash commands that **Slash command cleanup** is about to remove, or "data loss" from the stale → write-and-abort path. The procedure already encodes safety: `.ductus/config.toml` `[pinned] files` is the opt-out, the stale path writes upstream and aborts cleanly (recoverable from git), and slash-command cleanup is unconditional for unpinned files. Extra prompts duplicate information the procedure already gives the user and stall routine runs.

1. The walker context carries the inputs the host has already gathered and validated: project (the destination project name), description (one-line project description), languages (comma-separated), agents (registry keys), framework-version (release tag), archive-url and sha256-url (computed from framework-version), staging-dir, substitutions-map, manifest-entries (the per-strategy list described in **Shared Files** and **Per-Agent Scaffolding**), pinned-list (from `.ductus/config.toml`'s `[pinned] files` block), gitignore-block (the `.claude/`, `.augment/`, `.agents/`, `.opencode/`, `specs/.cache/`, etc. lines), host-block (the `project` value — the team-shared slash-command namespace — written to committed `.ductus/config.toml`, plus the per-contributor `cli-config-dir` written to the gitignored `.ductus/session.toml` since teammates may use different agents; the runtime reads both at `ductus exec` time to resolve `{cli-config-dir}/commands/{project}/<name>.md`), enforce-directories (the slash-command directories whose top-level `*.md` files are pruned to the manifest), and the per-agent ductus-install entry with `keep-literals: ["project", "cli-config-dir"]`. The host runs the markdown-only reference below to collect inputs, derive registry values, validate `.ductus/config.toml`, and seed context; the runtime walks the procedure that follows.

2. Invoke `fetch-archive` (MCP: `fetch-archive`) to download the framework tarball. The primitive verifies the sha256 against a sidecar URL when one is supplied; without a sidecar (the live-on-main case, since GitHub's auto-generated source tarballs ship without sidecars) it returns the computed digest and `verified: false`, leaving any out-of-band verification to the host. A sidecar mismatch halts the procedure with an `error` envelope so no partial state lands in the destination tree.

3. Invoke `extract-archive` (MCP: `extract-archive`) to expand the verified tarball into the staging directory. Path-traversal protection is applied per entry; symlinks are skipped. Otherwise, follow the markdown-only path's `tar -xzf` workflow.

4. Invoke `apply-manifest` (MCP: `apply-manifest`) with the host-built manifest entries and the pinned list. The primitive walks each entry, applies the per-entry strategy (update for framework-owned files, create for adopter-seedable files, skip-if-conflict for adopter-owned templates — the three strategy values defined in **Shared Files** below), short-circuits on the pinned list, returns aggregate counts the host surfaces in the completion message. This single call replaces the per-file update / create / skip loops the markdown-only reference describes below.

5. Invoke `merge-managed-block` (MCP: `merge-managed-block`) against `.gitignore` with `marker-style: "line-prefix"` and `marker: "ductus"` to install or update the framework-managed block (the `.claude/`, `.augment/`, `.agents/`, `.opencode/`, `specs/.cache/`, etc. lines). First-run creates the file; subsequent runs update only the region between the `# ductus` preamble line and the next blank line, preserving the rest of the file byte-for-byte. Replaces the inline `grep` check the markdown-only reference describes for the `.gitignore` merge step.

6. Establish the team-shared host configuration. Invoke `merge-managed-block` (MCP: `merge-managed-block`) against the **active config file** (write policy: §Project Configuration) with `marker-style: "line-prefix"`, `marker: "ductus (host)"`, and a block carrying **only** the resolved `project` value (the team-shared slash-command namespace). First-run creates the file with just the managed block; subsequent runs update only the region between the `# ductus (host)` preamble line and the next blank line, preserving every other config section (`[pinned]`, `[migrations]`, `[review]`) byte-for-byte — and dropping any legacy `cli-config-dir` key a prior version wrote into the managed block. On the markdown-only path, the host writes the `[host]` block to the same active file with its file-writing tool. See §Project Configuration for the `[host]` schema.

7. Record the per-contributor agent identity. Invoke `write-session` (MCP: `write-session`) with `cli-config-dir` set to the agent's resolved config-dir and **no** target fields — a host-config write that stores the agent identity in the gitignored `.ductus/session.toml` (preserving any existing target), never in committed config, because teammates on one project may each use a different agent. The runtime reads `project` from `.ductus/config.toml` and `cli-config-dir` from the session file at `ductus exec` time to resolve `{cli-config-dir}/commands/{project}/<name>.md`; absent either, it falls back to `.claude` / repo directory basename — fine for the framework's own repo, broken for any adopter whose layout doesn't match the defaults. On the markdown-only path, the host writes the session-file `cli-config-dir` key with its file-writing tool.

8. Invoke `enforce-manifest` (MCP: `enforce-manifest`) once per directory in the host's enforce-directories list (typically the per-agent slash-command directory). The primitive removes files matching the glob-include arg (default `*.md`) whose relative path is neither in the expected list nor pinned. One call replaces the slash-command manifest enforcement loop the markdown-only reference describes. Adopter cleanup of historical conventions is owned by the **Pre-run Migrations** section above and the `framework/migrations.toml` registry it drives.

9. Invoke `apply-manifest` (MCP: `apply-manifest`) a second time with a single entry for the per-agent `ductus` self-install (the `{cli-config-dir}/commands/ductus.md` path) and an **empty substitutions map** (`{}`). `ductus.md`'s body contains prose references to every placeholder name the bulk step substitutes — `{project}`, `{cli-config-dir}`, `{project-name}`, `{One-line project description.}` — describing what those placeholders mean in *other* files. None of them are values to substitute in `ductus.md` itself, so the self-install call passes no substitutions rather than relying on `keep-literals` to mask individual keys from the full map. The split from step 4 isolates the no-substitute concern from the bulk substitute step.

10. Render the completion message (host responsibility): list the agents configured, the next pipeline command (`/{project}:specify`), the acquired runtime's store path, and any per-agent post-install reminders from the registry rows above.

## Agent Registry

The registry lists every supported agent. Per-agent paths and behaviors are derived from these rows — the rest of this file references registry values, not agent names.

| `key` | `name` | `config_dir` | `layout` | `settings_template` | `rules_file_note` |
| --- | --- | --- | --- | --- | --- |
| `claude` | Claude Code | `.claude` | `claude-style` | `{ "permissions": { "allow": ["Bash(curl *)", "Bash(ls *)", "Bash(tar *)", "Bash(mktemp *)", "Bash(git status *)", "Bash(git config *)", "Bash(git rev-parse *)", "Bash(git diff *)", "Bash(git ls-files *)", "Bash(chmod *)", "Bash(awk *)", "Bash(command -v *)", "Bash(mkdir *)", "Bash(shasum *)", "Bash(sha256sum *)", "Bash(certutil *)", "Bash(ln *)", "Bash(cp *)", "Bash(~/.ductus/bin/ductus *)", "Bash(.ductus/bin/ductus *)", "Read(/private/var/folders/**/T/ductus-*/**)", "Read(//private/var/folders/**/T/ductus-*/**)", "Read(/var/folders/**/T/ductus-*/**)", "Read(//var/folders/**/T/ductus-*/**)", "Read(/tmp/ductus-*/**)", "Read(//tmp/ductus-*/**)"], "deny": [] } }` | Claude Code reads `CLAUDE.md` natively. |
| `auggie` | Auggie | `.augment` | `claude-style` | `{ "toolPermissions": [ { "toolName": "launch-process", "shellInputRegex": "^curl ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^ls ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^tar ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^mktemp ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^git status ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^git config ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^git rev-parse ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^git diff ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^git ls-files ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^chmod ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^awk ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^command -v ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^mkdir ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^shasum ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^sha256sum ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^certutil ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^ln ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^cp ", "permission": { "type": "allow" } }, { "toolName": "launch-process", "shellInputRegex": "^[^ ]*\\.ductus/bin/ductus ", "permission": { "type": "allow" } } ] }` | Auggie reads `CLAUDE.md` natively — no second rules file is needed. |
| `antigravity` | Antigravity | `.agents` | `antigravity` | `{ "permissions": { "allow": [ "command(curl)", "command(ls)", "command(tar)", "command(mktemp)", "command(git status)", "command(git config)", "command(git rev-parse)", "command(git diff)", "command(git ls-files)", "command(chmod)", "command(awk)", "command(which)", "command(mkdir)", "command(shasum)", "command(sha256sum)", "command(certutil)", "command(ln)", "command(cp)" ], "deny": [], "ask": [] } }` | Antigravity reads `AGENTS.md` natively — no second rules file is needed. |
| `opencode` | OpenCode | `.opencode` | `opencode` | `{ "$schema": "https://opencode.ai/config.json", "permission": { "bash": { "curl *": "allow", "ls *": "allow", "tar *": "allow", "mktemp *": "allow", "git status *": "allow", "git config *": "allow", "git rev-parse *": "allow", "git diff *": "allow", "git ls-files *": "allow", "chmod *": "allow", "awk *": "allow", "command -v *": "allow", "mkdir *": "allow", "shasum *": "allow", "sha256sum *": "allow", "certutil *": "allow", "ln *": "allow", "cp *": "allow", "*/.ductus/bin/ductus *": "allow" } } }` | OpenCode reads `AGENTS.md` natively — no second rules file is needed. |

### Derived values

For each agent, these paths and behaviors are computed by convention from its row — they are **not** stored in the table. Values that are the same for every agent are layout-independent; the rest are selected by the row's `layout` field.

**Layout-independent (every agent):**

| Derived value | Formula |
| --- | --- |
| Configure source path | `framework/bootstrap/configure/{key}.md` |

**Layout-derived (selected by `layout`):**

| Derived value | `claude-style` | `antigravity` | `opencode` |
| --- | --- | --- | --- |
| Command/skill path | `{config_dir}/commands/{project}/<name>.md` | `{config_dir}/skills/{project}-<name>/SKILL.md` | `{config_dir}/command/{project}/<name>.md` |
| Invocation | `/{project}:<name>` | `/{project}-<name>` | `/{project}/<name>` |
| `ductus` install path | `{config_dir}/commands/ductus.md` | `{config_dir}/skills/ductus/SKILL.md` | `{config_dir}/command/ductus.md` |
| Settings file | `{config_dir}/settings.local.json` | `{config_dir}/settings.json` | `opencode.json` (repo root; same file as MCP wiring) |
| Permission shape | `permissions.allow/deny` (Claude) / `toolPermissions[]` (Auggie) | `permissions.allow/deny/ask` (action grammar) | `permission` action map (`allow`/`ask`/`deny`) |
| Native rule-loading dir | — (rules read from shared `specs/rules/`) | `{config_dir}/rules/<name>.md` | — (rules read from shared `specs/rules/`) |
| Native rules file | `CLAUDE.md` | `AGENTS.md` | `AGENTS.md` |
| Slash-command cleanup glob | `*.md` in the commands dir | `{project}-*/` skill dirs in `skills/` | `*.md` in `command/{project}/` |

The session state file is `.ductus/session.toml` for every adopter — not a derived per-agent path (the path is uniform across agents). It's gitignored, and it additionally records the per-contributor `cli-config-dir` (see §Session state).

### MCP registration (per-agent)

MCP discovery is **not** layout-derived — it is a per-agent property. A host can share Claude's command/skill layout and native `CLAUDE.md` reading (Auggie does) yet register MCP servers somewhere entirely different. Each agent therefore declares its own MCP registration descriptor; the State-B auto-wire (§ductus runtime detection) and §MCP wiring branch on the `mechanism` column.

| `key` | MCP target | scope | mechanism | surfaced instruction (when `surface-instruction`) |
| --- | --- | --- | --- | --- |
| `claude` | `.mcp.json` (repo root) | `project-committed` | `write-file` | — |
| `auggie` | `~/.augment/settings.json` | `user-global` | `surface-instruction` | `auggie mcp add ductus --command ~/.ductus/bin/ductus --args "mcp"` |
| `antigravity` | `~/.gemini/config/mcp_config.json` | `home-level` | `surface-instruction` | edit `~/.gemini/config/mcp_config.json`, then `/mcp` reload |
| `opencode` | `opencode.json` (repo root) `mcp` block | `project-committed` | `write-file` | — |

- **`write-file`** — ductus writes `target` additively at State-B wire time (the additive merge in §MCP wiring). Only `project-committed` agents use it.
- **`surface-instruction`** — ductus writes **no** MCP file; State B surfaces the instruction in the Pre-flight abort and the user runs it once per machine, then restarts. Required for `user-global` / `home-level` agents, whose MCP config lives outside the repo and which ductus must not silently mutate.
- **Antigravity** loads MCP servers only from home-level `~/.gemini/config/mcp_config.json`; project-local `.agents/mcp_config.json` is **ignored** (verified against the live `agy` CLI). There is no scriptable `agy mcp add`, so registration is a config-file edit plus a `/mcp` reload.

### Adding a new agent

A `claude-style` agent (markdown commands under `{config_dir}/commands/{project}/`, reads `CLAUDE.md`) is a one-row registry append plus an MCP registration entry plus two satellite files:

1. Append a row with the six fields (`layout: claude-style`).
2. Add a row to §MCP registration (per-agent). MCP discovery is per-agent, not layout-derived, so even a `claude-style` agent must declare its own `target` / `scope` / `mechanism` — it is **not** inherited from the layout.
3. Add `framework/bootstrap/configure/{key}.md` with the agent's full permission set in its native settings format.
4. Add a curl snippet for the new agent to the README's adoption section.

An agent on a **different layout** (a new value in the `layout` column) additionally needs its branch added to §Derived values and the layout-keyed steps in §Per-Agent Scaffolding and §Permission Setup — the work the `antigravity` and `opencode` layouts each introduced. (MCP registration is per-agent regardless of layout, covered by step 2 above.)

## Inputs

The project inputs are the **project name**, a one-line **description**, and the primary **language(s)**. From `$ARGUMENTS`, extract the project name now (a single non-flag word, if present) and recognize the flags below. **Do not prompt for any missing project input here** — interactive collection is deferred to **§Collect Project Inputs**, which runs *after* the **Pre-flight Phase**. Collecting them earlier means a pre-flight abort (a stale `ductus.md` or a freshly-wired ductus) discards the user's freshly-typed answers and forces them to re-enter everything on the restart. Nothing before §Collect Project Inputs — the pre-flight checks, agent selection, permission seeding, and the Pre-flight Phase itself — needs the interactive inputs; they need only `$ARGUMENTS` and the project's on-disk layout.

Recognized flags in `$ARGUMENTS`:

- `--agents=key1,key2,...` — explicit list of agent keys to scaffold. Bypasses any prompt. Reject unknown keys.
- `--add-agent` — force the agent-selection prompt even when agents are already detected.

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

1. Read the agent's settings file — `{config_dir}/settings.local.json` for `claude-style`, `{config_dir}/settings.json` for `antigravity`, or the **repo-root `opencode.json`** for `opencode` (the same file as OpenCode's MCP-wiring target — settings and MCP wiring share one file; create it if missing, with the agent's `settings_template` from the registry; for `opencode`, merge into the adopter's existing `opencode.jsonc` instead if that is where their config lives).
2. Merge the agent's `settings_template` entries into the existing file additively: add any entries that are missing, do not deduplicate or reorder anything else, and do not overwrite entries the user or `/{project}:configure` previously added. For `claude-style` the entries live under `permissions.allow`/`permissions.deny` (Claude) or `toolPermissions` (Auggie); for `antigravity` they live under `permissions.allow`/`permissions.deny`/`permissions.ask`; for `opencode` they live under the `permission` action map (preserving `$schema` and every other top-level key).
3. Write the file if anything was added.

This prevents repeated permission prompts during the fetch and scaffolding phases. The full permission set is applied later by `/{project}:configure` (which writes the same per-layout settings file). The seed also covers every step of **Runtime acquisition** — `mkdir`, the platform checksum tool, `ln`/`cp` for the pointer, and execution of the store and pointer paths for the version probe — so a bootstrap that acquires the runtime prompts for nothing. The checksum entry is the one that matters most: leaving it unseeded adds no safety, it puts a dialog at the one gate that must never be waved through, and a prompt that appears on every bootstrap trains the reflex to approve it. The digest comparison is what protects the adopter, and it halts before anything is written.

### ductus runtime auto-wiring

`/ductus` **acquires** the ductus runtime and registers it as an MCP server — the **Pre-flight Phase → State B** path. The runtime is required ([§runtime-boundary](../constitution.md#runtime-boundary)), so a missing binary is work to perform rather than a condition to report: `/ductus` downloads the pinned release for the host platform into a ductus-owned store, materializes a per-project pointer to it, and wires the MCP config to that pointer. `PATH` is not consulted at any point.

Wiring depends on the agent's MCP registration `mechanism` (§MCP registration): a `write-file` agent gets its MCP file written; a `surface-instruction` agent gets a one-line registration command surfaced for the user to run (ductus never writes the user's home config) — see **ductus runtime detection → MCP wiring** for the per-mechanism rules. In the same pass, either way, `/ductus` adds the **ductus tool permissions** to the settings file so the next session calls the runtime without a per-tool prompt:

- **Claude** (`permissions.allow`): `mcp__ductus__*`
- **Antigravity** (`permissions.allow`): `mcp(ductus/*)`
- **Auggie** (`toolPermissions`): `{ "toolName": "mcp:ductus:*", "permission": { "type": "allow" } }` if Auggie's matcher honors the wildcard, otherwise the enumerated `mcp:ductus:<tool>` set `/{project}:configure` already installs.
- **OpenCode** (`permission`): `"ductus*": "allow"` (a single glob in the root `opencode.json` `permission` map).

The wildcard is the minimal bootstrap grant; the enumerated per-tool set stays owned by the generated block in `/{project}:configure`'s permission file and coexists harmlessly (exact-match dedup leaves both). Both the wiring write and this permission write are additive and idempotent and follow the same merge rules as the seed above — no existing entry is removed, reordered, or overwritten. There is **no new confirmation prompt**: the acquisition and the wiring are disclosed by the **Pre-flight abort** message, which names the store path and every file written — consistent with the §Procedural-fidelity rule the silent seed writes already follow.

## Pre-flight Phase

Run a single pre-flight phase after the **Permission Setup** seed (so the ductus binary probe is pre-authorized) and before **Pre-run Migrations** and the full archive fetch. The phase owns two restart-requiring checks — **ductus runtime detection** and the **ductus.md self-update check** — that can each force the session to restart: ductus detection to load a newly-wired MCP server, the self-update check to load a fresh `ductus.md`. Neither pays the cost of the multi-hundred-KB archive; both run on a small fetch or no fetch, so a restart-triggering abort never leaves archive work on disk.

The phase runs both checks, accumulates every restart-requiring write into a **pending-restart set**, and at the end emits a **single combined abort** if that set is non-empty (see **Pre-flight abort**). If neither check needs a restart, the run proceeds to **Pre-run Migrations**. Running both checks before the single abort is what collapses the worst case — a stale `ductus.md` on an adopter who has never wired ductus — into one restart instead of two.

**Create `{tempdir}` first, before either check.** Both checks fetch into it, and so does the later **Archive fetch and extract**:

```text
mktemp -d -t ductus-XXXXXX
```

On macOS/Linux this lands under `$TMPDIR` or `/tmp`. Never reuse a directory from a prior run — a fresh fetch is the only way `/ductus` picks up upstream changes. It is created here rather than inside either check because the checks run in order and the *first* of them needs it: **ductus runtime detection** fetches the version pin into it. Creating it inside the second check left the first with nowhere to fetch to, which is what made greenfield acquisition halt before it could start.

### ductus runtime detection

Resolve whether the ductus runtime is live in this session and, when it is not, acquire and wire it so the next session runs the deterministic path. Detection resolves to one of **two** states — A (runtime live this session) and B (not live: acquire, wire, restart). There is no third "binary absent" state: the runtime is required, and a missing binary is work to perform.

#### Detection mechanism

- **Tool-inventory introspection (State A).** Inspect your own available-tool inventory for any `ductus`-namespaced MCP tool — `mcp__ductus__*` on Claude Code, `mcp:ductus:*` on Auggie and Antigravity — counting deferred or lazily-loaded tool names as present (a host that lists tool names before exposing their schemas still has the runtime registered). Any match ⇒ **State A**. This needs no shell and no permission; you always know your own tools.
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
| `{pin}` | the single SemVer line in `{tempdir}/version`, fetched by **Runtime acquisition** step 1 |
| `{triple}` | the host target triple, from the table in **Runtime acquisition** |

#### State A — runtime live this session

A `ductus`-namespaced tool is available to this session, so the runtime is live and the rest of the run takes the **deterministic primitive path**.

**Live is not current — version-check it against `{pin}` before trusting it.** Probe the resolved binary (the `[runtime] path` when the project configures one, else `{store-path}`) and read its reported version. This is the same probe **Runtime acquisition** step 2 performs, and the **Permission Setup** seed pre-authorizes it.

- **Reports `{pin}`** — proceed. ductus contributes nothing to the **pending-restart set**, and detection emits no message. This is the routine path.
- **A project-supplied `[runtime] path` reports something else** — emit Branch 1's warning and continue. A project naming a path has stated deliberately which binary it wants.
- **Anything else** — the runtime is **live but stale**. Acquire `{pin}` per **Runtime acquisition** Branch 2, then run the rest of this session through `{pointer-path} <primitive>` rather than the MCP tools: the server was spawned at session start and is still the old binary, so its tool surface stays stale no matter what is now in the store. Add the acquisition to the **deferred-restart set** and carry the notice to the **Closing restart**, exactly as State B does.

A live-but-stale runtime fails in the direction hardest to attribute. It is missing primitives the framework has since come to depend on, and `/ductus` reports success because a tool *was* in the inventory — so the failure surfaces later, somewhere else, as someone else's problem. Observed 2026-08-19 in an adopter project: the store held `0.29.10` while the framework pinned `0.31.0`, so the pre-commit hook that same run refreshed called `derive-dependencies` and `derive-references`, which that binary does not carry, and the shell generators they replaced had already been deleted. Every commit in that project halted, and nothing in the `/ductus` run said the runtime was behind. Detection that stops at "a tool is in my inventory" answers the wrong question: what the run needs to know is whether the runtime it is about to depend on is the one this framework revision was tested against.

State A is a **binding execution contract, not a preference.** Detecting the runtime and then walking the prose `curl`/`tar`/`python3` path anyway is the exact failure 029 exists to prevent — it spends the markdown path's tokens despite a cheaper path being live, and it is what makes the State-B wire-and-restart pointless. For the rest of this run:

- **Every step that names a backticked primitive** — a bare name (`fetch-archive`, `extract-archive`, `apply-manifest`, `merge-managed-block`, `enforce-manifest`, `merge-permissions`, `run-generator`, …) that matches a `ductus` tool in your inventory — **MUST be performed by calling that MCP tool** (`mcp__ductus__<primitive>` on Claude, `mcp:ductus:<primitive>` on Auggie/antigravity; mapping per §Instructions).
- **The shell commands shown under those steps** (`curl`, `tar -xzf`, `python3`, `awk`, byte-compares, hand-authored scaffold loops) are the **State-B/C fallback specification.** In State A they document the contract each tool fulfills; they are **not instructions to execute.** Do not run them. If you are about to run `curl`/`tar`/`python3` for a step that names a primitive, stop — that is the fallback path leaking into a State-A run; call the tool instead.
- **Steps with no backticked primitive run as shown in every state** — the per-language `.gitignore` `curl` against `github.com/github/gitignore`, `git config core.hooksPath`, `chmod`, the git repo / tracked-file checks, and the §Collect Project Inputs prompts have no tool equivalent.
- **If a primitive call errors** — e.g., a too-old wired `ductus` surfaces a parse error per `spec 022` §Versioning enforcement — fall back to **that step's** shell specification for that one step and continue; do not abandon the deterministic path for the whole run.

#### State B — runtime not live: acquire, wire, restart

No `ductus` tool is available to this session. In order:

1. **Resolve the binary** per **Runtime acquisition** below — either the project's own `[runtime]` path, or an acquisition into the store.
2. **Materialize the pointer** per **Pointer materialization** below.
3. Register the `ductus` server per the agent's MCP registration `mechanism` (§MCP registration; details in **MCP wiring**): for `write-file`, write the MCP file additively; for `surface-instruction`, write **no** MCP file — the registration command is surfaced in the abort for the user to run once per machine.
4. Add the permission entries needed to call the `ductus` tools (see **Permission Setup**), so the next session calls them without a prompt. This write is the same for every agent regardless of `mechanism` — it targets the project-level settings file, not the MCP-server location.
5. Add the acquisition, the wiring, and the permission write to the **deferred-restart set** and contribute this notice to the **Closing restart** at the end of the run, naming the store path and every file written. State B does **not** stop here: the binary is now on disk and `Bash(.ductus/bin/ductus *)` was seeded in **Permission Setup** before the probe, so the run continues and invokes every remaining primitive as `{pointer-path} <primitive>` — the same deterministic code the MCP tools call, reached through the CLI surface spec 022 AC1 ships alongside them. The abort this used to raise bought nothing the CLI could not already give this run.

> **ductus runtime acquired.** The runtime was not registered for this project, so `/ductus` could not use the deterministic path this run. Installed `{version}` to `{store-path}`.

The abort takes the form matching the selected agent's `mechanism`:

- **`write-file` agent** (e.g. Claude): "It has now been wired in so the next session runs through the runtime, which uses far fewer tokens. Files written: {comma-separated paths — the store, the pointer, the wiring file, and the settings file when permission entries were added}."
- **`surface-instruction` agent** (e.g. Auggie): "{Agent} registers MCP servers in your user-level config, which `/ductus` does not write. To enable the runtime, run this once, then start a fresh session: `{the agent's surfaced instruction from §MCP registration}`. Files written: {the store, the pointer, and the settings file when permission entries were added}."

State B issues **no separate consent prompt** — the writes are additive and idempotent, matching the silent **Permission Setup** writes; the abort's file list (and, for a `surface-instruction` agent, the one-line command) is the disclosure. There is no opt-out flag.

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

1. **Fetch and read the pin.** One SemVer line, no `v` prefix, fetched into the `{tempdir}` the **Pre-flight Phase** created:

   ```text
   curl -fsSL https://raw.githubusercontent.com/stonean/ductus/main/version -o {tempdir}/version
   ```

   It is fetched here rather than read out of the framework archive because acquisition runs in **pre-flight**, and the archive is not fetched until **Archive fetch and extract**, hundreds of lines later. Reading it from the archive is what this step used to specify, and it halted every greenfield adoption: State B is the first-run state by definition, so the pin was never on disk when this step needed it. A one-line file keeps pre-flight's small-fetch-or-no-fetch property intact — it is the archive's multi-hundred-KB cost this phase avoids, not a `curl`.

   The pin and the framework tree now arrive in two fetches rather than one, so they agree only because both name `main`. A push landing between them is the sole divergence, it is bounded by one run, and the next `/ductus` re-acquires against the newer pin — acquisition is idempotent and re-probes the store. If the fetch fails, or the file is **absent or unparseable**, halt naming it: guessing a version or falling through to "latest" silently installs a runtime the framework was never tested against.

   > Halt: `could not read the runtime version pin from https://raw.githubusercontent.com/stonean/ductus/main/version — /ductus cannot state which runtime this framework revision requires.`

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
   curl -fsSL https://github.com/stonean/ductus/releases/download/ductus-v{pin}/ductus-{triple}.tar.gz \
     -o {tempdir}/ductus-{triple}.tar.gz
   curl -fsSL https://github.com/stonean/ductus/releases/download/ductus-v{pin}/ductus-{triple}.tar.gz.sha256 \
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
> `Release: https://github.com/stonean/ductus/releases/tag/ductus-v{pin}`

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

**`write-file` agents** (scope `project-committed` — Claude and OpenCode). ductus writes the agent's `target` MCP file from §MCP registration, using that agent's server-entry shape — **Claude:** `.mcp.json` at the repo root, the `mcpServers` map, `{ "command": ".ductus/bin/ductus", "args": ["mcp"] }`; **OpenCode:** the committed root `opencode.json` (or the adopter's existing `opencode.jsonc`), the `mcp` map, `{ "type": "local", "command": [".ductus/bin/ductus", "mcp"], "enabled": true }`. The write **updates the file in place — it never replaces or truncates it.** Apply the matching case (read `{servers-key}` as `mcpServers` for Claude, `mcp` for OpenCode):

- **Missing file** — create it containing only the `ductus` entry (for OpenCode, include `"$schema": "https://opencode.ai/config.json"`).
- **Has `{servers-key}`, no `ductus`** — add the `ductus` entry; preserve every other server and every other top-level key (including OpenCode's `$schema` and `permission`).
- **Already has a `ductus` entry** — no-op; leave the file byte-unchanged (idempotent re-run).
- **No `{servers-key}` key** — add the key with just the `ductus` entry; preserve all other top-level keys.
- **Not valid JSON** — do **not** touch the file. Skip wiring and warn the user to repair it. The runtime is still acquired and the pointer still materialized — only the registration is skipped — so the next run wires it once the file parses. A hand-maintained config is never clobbered.

There is no `ductus` runtime primitive for this merge: State B is the runtime-absent case by definition, so the write is always host-side.

**`surface-instruction` agents** (scope `user-global` / `home-level` — Auggie and Antigravity). The split follows the config's **scope**, not the agent: a `project-committed` target names the repo-relative pointer, a `user-global` / `home-level` target names the absolute store path. That is what removes the asymmetry these agents used to carry — their config holds a single `ductus` entry serving every project on the machine, so no project-specific path could ever be correct in it, and a store owned by no project can. The agent reads MCP servers from a file in the user's **home** directory, shared across all their projects, which ductus must **not** write. ductus writes no MCP file; instead the **Pre-flight abort** surfaces the agent's registration instruction for the user to run once per machine, then restart:

- **Auggie** — `auggie mcp add ductus --command {store-path} --args "mcp"` (the documented, schema-stable subcommand; it writes `~/.augment/settings.json`). The **absolute store path**, not the pointer: this config is per-machine and shared across every project, so no project-relative path could be correct in it.
- **Antigravity** — add a `ductus` block to `~/.gemini/config/mcp_config.json` naming the **absolute store path** (`{"mcpServers": {"ductus": {"command": "{store-path}", "args": ["mcp"]}}}`), then reload via the in-prompt `/mcp` overlay (there is no scriptable `agy mcp add`; project-local `.agents/mcp_config.json` is ignored). Absolute for the same reason as Auggie: the file is per-machine and serves every project.

The permission write (State B step 2) still happens for these agents — it targets the project-level settings file the agent reads, independent of the home-level MCP-server location.

### Self-update check

Verify the running session's `ductus.md` instructions are current.

#### Small fetch

`{tempdir}` already exists — the **Pre-flight Phase** created it before either check, and **ductus runtime detection** has already fetched the version pin into it. Do not create a second one.

Issue exactly one `curl` against `raw.githubusercontent.com` for the upstream bootstrap file:

```text
curl -fsSL https://raw.githubusercontent.com/stonean/ductus/main/framework/bootstrap/ductus.md \
  -o {tempdir}/ductus.md.upstream
```

If the fetch fails — non-zero `curl` exit, network error, or a 404 — abort the run with this error and do not continue:

> Failed to fetch the ductus.md self-update check ({reason}). Re-run after checking network connectivity, or report this if it persists.

#### Per-agent comparison

For each selected agent, compare the upstream `{tempdir}/ductus.md.upstream` against the agent's installed `ductus` file and assign one status. For `claude-style` the installed file is `{config_dir}/commands/ductus.md` and for `opencode` it is `{config_dir}/command/ductus.md` — both installed verbatim (frontmatter included), so the comparison is a direct byte-compare against `{tempdir}/ductus.md.upstream`. For `antigravity` the installed file is `{config_dir}/skills/ductus/SKILL.md`, which wraps **only the upstream body** in `name: ductus` frontmatter — the installer drops `ductus.md`'s own frontmatter when wrapping. So compare **bodies on both sides**: strip the leading frontmatter block (the first `---`-delimited region) from the installed `SKILL.md` **and** from `{tempdir}/ductus.md.upstream`, then byte-compare what remains. Stripping only the `SKILL.md` side leaves `ductus.md`'s frontmatter on the upstream side, which never matches — a false `stale` on every run. The statuses below are assigned from this body-vs-body (antigravity) or file-vs-file (`claude-style` / `opencode`) comparison:

- **`no installed copy`** — the installed file does not exist (first run for this agent). Continue.
- **`current`** — the two files are byte-identical, **or** the installed file is byte-identical to upstream and listed in `.ductus/config.toml` `pinned.files` (the pin had nothing to suppress this run). Continue.
- **`stale`** — the two files differ and the installed file is **not** pinned. The running session is using older instructions than what is current upstream.
- **`pinned-divergent`** — the two files differ and the installed file **is** listed in `.ductus/config.toml` `pinned.files`. The pin intentionally suppresses the update; continue, and emit a single advisory line in the post-scaffolding output.

The check is scoped to **selected agents only** — agents whose `config_dir` exists in the project but are not in this run's selection are not diffed. An unselected stale agent will trip the check on its very next `/ductus` run targeting it.

#### Stale → defer to pre-flight abort

If any selected agent is recorded as `stale`:

1. For **each stale agent**, overwrite **the installed file the staleness comparison just read** — not the canonical filename — so the next session loads the up-to-date instructions. For `claude-style` that is `{config_dir}/commands/ductus.md` once the entry point has been renamed, but `{config_dir}/commands/govern.md` for an adopter still carrying the retired one; copy `{tempdir}/ductus.md.upstream` verbatim over whichever the comparison resolved. For `opencode`, the same rule against `{config_dir}/command/`. Writing the canonical name instead would create a second command file beside the stale one, and `ductus-rename` step 6 then moves the stale file onto the canonical path, replacing the fresh copy with the one it just superseded. The canonical filename is owned solely by that migration. For `antigravity`, write `{config_dir}/skills/ductus/SKILL.md` as the transformed skill — `name: ductus` frontmatter followed by the upstream body — **not** the raw `ductus.md` (a raw copy is not a loadable skill). In both cases do not substitute placeholders in the body — `{project}` and `{cli-config-dir}` stay literal, per the `ductus` self-install rule.
2. Run the **Post-Write Integrity Check** (see below) on each freshly written file.
3. Do not write `ductus.md` for non-stale agents — their installed copies already match upstream.
4. Do not write `ductus.md` for `pinned-divergent` agents — the pin opts them out of automatic updates.
5. Add each stale agent's overwrite to the **pending-restart set** and contribute this notice to the combined **Pre-flight abort** — do **not** abort here:

> **The ductus command itself has updated.** Your installed copy was behind upstream and the running session is using the older instructions. The freshly fetched copy has been written to disk for stale agents.
>
> Stale agents updated: {comma-separated names}.

The shared "start a new session and re-run" line and the skip of every later section are owned by **Pre-flight abort**, so a stale `ductus.md` and a freshly-wired ductus surface in one abort and one restart rather than two.

#### Pinned-divergent → continue with advisory

If a selected agent is recorded as `pinned-divergent`, the run continues normally. After scaffolding, the **Post-Scaffolding Output** includes one advisory line per divergent agent (see **Post-Scaffolding Output → Pinned ductus.md advisory**). The advisory is silent on runs where every pinned agent is `current` (the pinned version happens to match upstream this run).

Pinning is an opt-out from automatic updates, not an opt-out from knowing the pin is currently active. When the pinned version actually drifts from upstream, the user usually wants to either review the upstream changes and unpin, or consciously confirm they are staying on the old version. Adopters who are deliberately and indefinitely on an old version see no recurring nag because the advisory only fires when divergence is real.

#### Current / no installed copy → continue

When all selected agents are `current` or `no installed copy`, the self-update check contributes nothing to the **pending-restart set**. The `{tempdir}` the **Pre-flight Phase** created is reused by the **Archive fetch and extract** step below — one `mktemp` for the whole run, no leaked extra temp directory. Whether the run proceeds is decided by **Pre-flight abort** once ductus detection has also run.

### Pre-flight abort

After both checks have run, inspect the **pending-restart set**:

The set has two contributors and they are **not** equivalent, so they are inspected separately:

- **A stale `ductus.md` (self-update)** — abort **now**, before any further work. The run must not proceed on instructions it has just replaced: the installed copy that is executing cannot be trusted to describe the procedure that now exists on disk. Emit the stale-update notice — see **Self-update check → Stale → defer to pre-flight abort** — with the closing line **Start a new session and re-run `/{installed-command}` to pick up the changes**, where `{installed-command}` is the basename of the installed file step 1 just overwrote. Name the command the adopter can actually invoke, never the canonical one: an adopter still on the retired entry point has no `/ductus` command until `ductus-rename` step 6 creates it, and that migration cannot run until they successfully invoke the bootstrap again — so hardcoding `/ductus` here names nothing they have, and the chain never starts.
- **State B wiring only** — do **not** abort. The binary is on disk, the CLI is permission-seeded, and every remaining step's primitives are reachable as `{pointer-path} <primitive>`, so stopping here would defer the entire run to another session for nothing. Continue to **Collect Project Inputs** and carry the wiring notice to the **Closing restart**.
- **Empty** — no restart is needed at all. Proceed to **Collect Project Inputs**. (ductus detection resolved to State A, and the self-update check saw `current` / `no installed copy` / `pinned-divergent` for every selected agent.)

On the **stale `ductus.md`** branch, everything past the pre-flight phase — **Collect Project Inputs**, **Pre-run Migrations**, **Project Configuration**, the **Archive fetch and extract**, **Frontmatter Migration**, **Shared Files**, **Per-Agent Scaffolding**, **Security Audit**, and **Post-Scaffolding Output** — is skipped. On the **State B** branch none of it is: the run proceeds through all of it via the CLI and stops only at the **Closing restart**. The only writes performed are the additive **Permission Setup** entries, any per-stale-agent `ductus.md` overwrite, and any ductus wiring plus its permission entries. Because input collection now lives past this point, an aborted run never prompts the user for the project name, description, or languages — they are asked exactly once, in the session that proceeds to scaffold. The next `/ductus` run in a new session sees ductus live (or absent) and every selected agent `current` (or `no installed copy`), and proceeds normally without abort.

## Collect Project Inputs

The Pre-flight Phase has passed (nothing in the pending-restart set), so this run will proceed to scaffold. **Only now** — never before the Pre-flight Phase — resolve the project inputs, so an abort can never discard answers the user just typed.

The **active config file** (write policy: §Project Configuration) is the persistent home for these answers. Resolve each input from the first available source and **prompt only for what is still missing**:

1. **Project name** — from `$ARGUMENTS` (a single non-flag word, per §Inputs), else `[project] name` in the config file (else `[host] project` for configs predating the `[project]` table), else prompt. Used for `{project}` substitution and command directory naming.
2. **Project description** — from `[project] description` in the config file, else prompt. Used for AGENTS.md.
3. **Primary language(s)** — from `[project] languages` in the config file, else prompt. Used for .gitignore language patterns.
4. **Rule surfaces** — from `[rules] surfaces` in the config file, else prompt ("Which rule surfaces does this project need? backend / frontend / both"). Recorded as a list with members in `{backend, frontend}` ("both" records both). Selects which rule files `/ductus` installs (§Shared Files) and which `/ductus:review` enforces (`review.md` §Behavior step 5). When the recorded surfaces exclude a surface that `[project] languages` implies (e.g., a frontend language is listed but `surfaces` omits `frontend`), emit one advisory line and honor the explicit value. **Validate a present value before using it** (degenerate configs fail fast per `CFG-ENV-003`, never silently ignored): the **empty list** (`surfaces = []`) is valid and means cross-only — install only `*-cross.md`, no surface-suffixed files — and is distinct from the key being unset (which derives/installs all); an **unrecognized member** outside `{backend, frontend}` (a typo, or `"cross"` — cross-cutting files are not a selectable surface) halts with `/ductus: invalid [rules] surfaces member "<value>" — accepted members are "backend" and "frontend" (use [] for cross-only; -cross.md files always apply)`, and a list mixing valid and invalid members fails on the invalid one; a **non-list value** (a bare string) halts with `/ductus: [rules] surfaces must be a list of strings, got <type>`.
5. **Spec-root directory** — from `[paths] specs-root` in the config file, else prompt ("What should the spec-root directory be named?") **defaulting to `specs`**. Names the top-level directory that holds every ductus artifact (feature dirs, `inbox.md`, `rules/`, shared docs). The prompt lives **only** in `/ductus` — no other command asks for it — and when the key stays unset every command and the runtime default to `specs`, so an adopter who never sets it sees unchanged behavior (spec [040](../../specs/040-configurable-specs-dir/spec.md)). **Validate a present or entered value before using it** (fail fast — a value that breaks path resolution is never silently accepted): a name that is empty or contains any character outside `[A-Za-z0-9_-]` (a path separator, `.`/`..`, or other punctuation) halts with `/ductus: invalid [paths] specs-root "<value>" — must be a single directory name using only letters, digits, '-', and '_'`. Two **non-blocking** notices after a valid value is chosen: when the chosen directory **already exists on disk and is not a ductus spec root** (no `inbox.md`, no numbered `NNN-*` subdirs), emit one line naming it and proceed — it may be a sibling framework's directory (e.g. RSpec's `spec/`), and the operator's choice is honored after the warning; when the configured `specs-root` is **absent on disk but a different ductus-shaped directory exists**, emit a one-line half-finished-rename notice rather than silently scaffolding a new empty tree.

On a routine re-run (update mode) the config file already carries all five, so this step prompts for nothing. On a first scaffold it prompts for whatever is missing, then **persists the three project inputs into the active config file's `[project]` table** (`name`, `description`, `languages`), **the rule surfaces into the `[rules]` table** (`surfaces`), **and the spec-root into the `[paths]` table** (`specs-root`; see §Project Configuration), preserving every other section, so the next run — and the session after any State B / stale-`ductus.md` restart — reads them back instead of re-asking. `host.project` continues to be written from `project.name` as the runtime's slash-command namespace.

When prompting (AskUserQuestion), every question **must** include an `options` array with 2–4 example choices (the user can always select "Other" for custom input):

- **Project name** — example options: the current directory name, `my-service`.
- **Project description** — example options: `A new microservice`, `CLI tool for X`.
- **Primary language(s)** — comma-separated list. Example options: `Go`, `Python`, `Node`, `Go, Python`.
- **Spec-root directory** — example options: `specs` (the default), `governance`, `design`.

Validate the project name: must be lowercase, alphanumeric, and hyphens only. If invalid, reject with: "Project name must be lowercase, alphanumeric, and hyphens only."

## Pre-run Migrations

Adopter-side cleanup for conventions that have been removed or renamed since the adopter's last `/ductus` run. Driven by a machine-readable registry at `framework/migrations.toml` (one `[[migrations]]` entry per active removal); per-entry procedure bodies live at `framework/migrations/{id}.md`. Spec [027 — Bootstrap Migration Registry](../../specs/027-bootstrap-migration-registry/spec.md) defines the contract.

### Procedure

1. Read `framework/migrations.toml` from the fetched archive. If the file is missing or malformed (TOML parse error), abort with `Failed to read framework/migrations.toml; cannot run pre-run migrations.` and do not continue.
2. Resolve the **active config file** once for this run (write policy: §Project Configuration) and read its `[migrations].last_applied` (treat an absent `[migrations]` section as null). Every marker write-back in step 7 targets this same resolved file, so the read and the write-backs agree even though the config file is itself a migration target; the one step that changes the resolution mid-run is the `govern-dir-consolidate` procedure, whose config move makes `.ductus/config.toml` the active file for every later step (its procedure file says so).
3. Filter the registry to entries where both:

   - `introduced_in` is greater than `last_applied`'s `introduced_in` (SemVer comparison, lex tie-break on `id`); when `last_applied` is null, every entry qualifies.
   - Either `sunset_after` is absent, or the current ductus release version is less than `sunset_after` (SemVer comparison).

4. If the filtered list is empty, emit nothing and proceed to the next bootstrap section.
5. Otherwise, prompt once with text of the form:

   ```text
   N framework migrations are pending since your last /ductus run:
     - {id} (introduced {introduced_in})
     ...
   Apply now? (Y/n)
   ```

6. On decline, emit `warning: N migrations skipped; pipeline commands may fail on legacy artifacts until applied. Re-run /ductus to apply.` and proceed without filesystem changes.
7. On confirm, for each filtered entry in order:

   1. Read `framework/migrations/{id}.md` from the fetched archive.
   2. Execute its `## Procedure` steps. The procedure file owns idempotency (step 1 of every procedure exits silently when the target artifact is absent), per-file user prompts (when applicable), and the post-scaffolding summary line.
   3. After the procedure completes successfully, update the active config file's `[migrations].last_applied = "{id}"` atomically (tempfile + rename, matching the rest of the config file's write semantics) — the file resolved at step 2, or `.ductus/config.toml` once the `govern-dir-consolidate` procedure has moved it. The update happens **per entry**, not at end of batch — an aborted batch resumes from the next-pending entry on the following `/ductus` run.
   4. If a procedure aborts (rare — only via explicit user "stop everything" path inside the procedure file), halt the loop. The retained `last_applied` value points at the last-completed entry; the next run resumes.

8. After the loop, invoke `check-orphaned-references` (MCP: `check-orphaned-references`) once and report each finding. Each migration is authored against the layout as it stood at its own `introduced_in` and is correct there; nothing validates the **composition**, and an adopter far enough behind runs several in one batch, so the composition is what they actually execute. A later entry moving a path an earlier entry wrote into an **adopter-owned** file — `create` strategy, so the manifest never overwrites it, and unpinned, so the pinned-invoker warning never fires — leaves a reference pointing at nothing, and nothing errors: a dangling `@import` yields a constitution that is simply not loaded, and a hook calling a moved generator fails at commit time, far from the run that broke it. Report `Orphaned reference: {referrer}:{line} names {target}, which does not exist; most likely orphaned by migration {id}` — the `{id}` clause comes from the finding's `migration`, available here because the registry is in the fetched archive (the result's `attribution` reads `registry`). Findings in `skipped` are referrers that could not be read; surface them as unexamined rather than folding them into a clean count. **The batch does not halt**: the migrations that applied are correct and re-running is safe, and the adopter may have hand-edited the reference, so this reports and never repairs. Run it on **every** batch, not only multi-entry ones — a single migration can orphan a reference just as a chain can, and scoping this to chains would make it run least often in the case it was written for. When the batch applied nothing, there is nothing to verify and nothing is emitted — not a clean bill of health for files it never examined. The same primitive is `/{project}:analyze`'s durable adopter-facing surface, where it runs without the registry; see [027 — Bootstrap Migration Registry](../../specs/027-bootstrap-migration-registry/spec.md)'s `migration-chain-reference-integrity`.

### Stale-reference behavior

If the active config file's `[migrations].last_applied` references an `id` that no longer exists in the active registry (the entry was sunsetted since the adopter's last run), treat the field as "before the oldest active entry" and run every active entry. Emit one warning: `last_applied was "{retired_id}" which has been retired; see CHANGELOG.md for its recipe.` Adopters far enough behind to hit a sunsetted entry apply it manually from `CHANGELOG.md`.

### Duplicate-id and reference-integrity guard

If `framework/migrations.toml` contains two entries with the same `id`, or if any entry's `procedure_file` references a path that doesn't exist in the fetched archive, abort the loop before applying anything with a clear error. `/audit`'s Family 10 (`scripts/audit/migration-coverage.sh`) catches these at maintainer time; this guard is the runtime safety net.

## Project Configuration

`.ductus/config.toml` is the project's configuration and persisted-decisions store. Readers fall back through the earlier locations while it is absent — `.govern/config.toml` (042-era) then the repo root `.govern.toml` (pre-042) — and the newest existing file wins when more than one is present (specs 042, 049). If the file exists, read it before processing the file manifest. The file is optional — if it does not exist, use default behavior for every key. If the file exists but is malformed (TOML parse error), abort the run with a clear error rather than silently proceeding.

**Write policy — the `/ductus` migration is the sole cutover (spec 042).** Every config write in this procedure (the `[host]` managed block, the `[project]`/`[rules]`/`[paths]` input persistence, `[migrations].last_applied`) and every session write targets the **active file**: the newest tier that exists — `.ductus/`, else `.govern/`, else the repo root — and the `.ductus/` file for a fresh project where none exists. No write outside the directory migrations ever creates `.ductus/config.toml` while an older config lingers — that partial file would win on read and strand the legacy file's other sections. The migration moves the whole file as one unit; the runtime's `config_path_for_write` / `session_path_for_write` resolvers are the canonical statement of this rule, and `write-session` applies it on every session write.

The file is a flat collection of top-level sections. There is no umbrella namespace; each section is keyed to the thing it governs. The sections that may appear in the config file:

```toml
# ductus (host)
[host]
# `project` only — the team-shared slash-command namespace. The per-contributor
# `cli-config-dir` lives in the gitignored `.ductus/session.toml` (teammates may
# use different agents), never here.
project = "gov"

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
```

`host.project` — the project's slash-command namespace, written by `/ductus` into a managed block (`# ductus (host)` line-prefix marker) in committed `.ductus/config.toml` on every run (idempotent — re-runs update rather than append). The per-contributor `cli-config-dir` (the agent's config-dir name) is **not** committed: teammates on one project may each use a different agent, so `/ductus` writes it to the gitignored `.ductus/session.toml` instead (§Instructions step 7). The runtime reads `project` from `.ductus/config.toml` and `cli-config-dir` from the session file at `ductus exec` time to resolve `{cli-config-dir}/commands/{project}/<name>.md`; both fall back to `.claude` / the repo directory basename when absent. Adopters whose layout matches the defaults (this repo, anyone on Claude Code with the conventional `.claude/commands/<project>/`) never observe the difference; Auggie / OpenCode adopters and anyone with a non-standard layout do.

`project.name`, `project.description`, and `project.languages` — the project inputs collected at §Collect Project Inputs (name; one-line description for AGENTS.md; primary languages for .gitignore patterns), written into the `[project]` table additively (preserving every other section) and read back on every subsequent run so the inputs are asked at most once. `[project]` is the source of truth for the answers; `host.project` is written from `project.name` as the runtime's slash-command namespace (the derived runtime view of the same value), so the two cannot diverge. Editing a `[project]` value re-runs the corresponding scaffold step with the new value on the next `/ductus` — the documented way to rename a project or change its languages. The table is host-side state (the host gathers inputs before the runtime walks per §Instructions step 1), so it is written on every adoption path without a runtime primitive.

`rules.surfaces` — the rule surfaces the project enforces and installs (§Collect Project Inputs, item 4). A list with members in `{backend, frontend}`; `-cross.md` rule files are unconditional and not selectable members. When unset, `/ductus` installs every rule file and `/ductus:review` derives the surface from stack detection (pre-033 behavior). When set, the **Shared Files** manifest pass installs only the rule files whose suffix matches a listed surface plus every `*-cross.md`, and `/ductus:review` enforces only those (`review.md` §Behavior step 5). The **empty list** (`surfaces = []`) is a valid set value meaning **cross-only** — only `*-cross.md` is installed/enforced — and is distinct from the key being unset (the empty list declares "no surface rules"; unset means "derive"). A **degenerate value** fails fast per `CFG-ENV-003` rather than being silently ignored: an unrecognized member outside `{backend, frontend}` (a typo, or `"cross"`) and a non-list value both halt the command that reads the setting (`/ductus` here, `/ductus:review` in `review.md` §Behavior step 5), naming the offending value or type. Editing `surfaces` takes effect on the next `/ductus`: newly-listed surfaces are installed, and rule files for a removed surface are **left in place** (not deleted — they are not in `enforce-directories`), they simply stop receiving updates.

`pinned.files` — any file listed that would normally use `update` strategy is treated as `skip` instead. Report pinned files in the post-scaffolding summary.

`migrations.last_applied` — slug of the newest pre-run migration applied to this project, written by `/ductus` after each successful migration in §Pre-run Migrations. Absent section means "no migrations applied"; bootstrap runs every active entry on the next run. Adopters should not edit this field by hand — the registry in `framework/migrations.toml` and the per-entry procedure files in `framework/migrations/{id}.md` are the authoritative sources.

`review.disabled-rule-files` — array-of-tables consumed by `/ductus:review` at rule-file selection time (see [`framework/commands/review.md`](../commands/review.md) §Inputs and §Behavior step 5). `/ductus` does not read this key; it is documented here so adopters see the full `.ductus/config.toml` schema in one place.

The full schema (allowed values, case-insensitive matching, empty-section behavior, future-section guidance) is declared in [`specs/019-config-decisions/data-model.md`](../../specs/019-config-decisions/data-model.md).

## File Fetching

Files from the `ductus` repo are sourced from a single archive download, extracted into the temp directory established during the **Pre-flight Phase**, and resolved as local paths for the rest of the run. Per-language `.gitignore` patterns from `github.com/github/gitignore` are **not** part of this archive — they remain separate `curl` calls (see the **.gitignore** subsection of **Shared Files** below).

This section runs only after the **Pre-flight Phase** passes (no pending restart — no stale `ductus.md` and no freshly-wired ductus). On a pre-flight abort, the archive is never fetched.

**State A reminder:** the archive fetch/extract and the manifest passes below are primitive-backed. In a State-A run (ductus live), call the `fetch-archive`, `extract-archive`, `apply-manifest`, and `enforce-manifest` tools — the `curl`/`tar` blocks shown are their State-B/C fallback spec, not commands to execute (see **§Pre-flight Phase → State A — runtime live this session**). The per-language `.gitignore` `curl` is *not* primitive-backed and runs as shown in every state.

### Archive fetch and extract

Issue exactly one `curl` against GitHub's archive host, downloading into the temp directory established during the pre-flight phase:

```text
curl -fsSL https://codeload.github.com/stonean/ductus/tar.gz/refs/heads/main \
  -o {tempdir}/main.tar.gz
```

This is the direct `codeload.github.com` endpoint — the target that `https://github.com/stonean/ductus/archive/refs/heads/main.tar.gz` 302-redirects to. Fetch it directly: the redirect form lands the command on a **new host mid-flight**, which some hosts (e.g. Antigravity) gate with a permission prompt even when a `curl` allow is pre-granted, because the grant matched the original host, not the redirect target. The direct URL has no redirect, so the bootstrap seed's `curl` pre-grant (`command(curl)` / `Bash(curl *)` / the Auggie `^curl` regex matcher) actually covers it. The archive's top-level directory is `ductus-main/`; the framework files live at `ductus-main/framework/...` after extraction.

After fetching:

1. Extract the archive into the existing temp directory: `tar -xzf {tempdir}/main.tar.gz -C {tempdir}`.
2. Compute the framework root: `{tempdir}/ductus-main/`. Treat this as the local mirror of the `ductus` repo for the rest of the run.

If the fetch or extraction fails — non-zero exit from `curl` or `tar`, or a missing `ductus-main/` directory after extract — abort the run with this error and do not continue scaffolding:

> Failed to fetch or extract the `ductus` archive ({reason}). Re-run after checking network connectivity, or report this if it persists.

A missing archive means **every** manifest entry would be missing, so partial scaffolding is impossible — the abort is the correct behavior. The pre-flight phase has already completed by this point, so a stale `ductus.md` or a freshly-wired ductus would have already triggered the pre-flight abort earlier.

### Per-file resolution

For each manifest entry below (in **Shared Files** and **Per-Agent Scaffolding**):

1. Compute the local source path: `{tempdir}/ductus-main/{source-path}`.
2. If the local source path does not exist — the file was renamed, removed upstream, or the manifest is out of sync — warn `Source not found in archive: {source-path}; skipping.` and continue with the remaining entries. This preserves the "do not abort on a single fetch error" guarantee at the per-entry level, even though the archive itself is fetched once.
3. Apply the entry's strategy (`update`, `create`, `skip`, `merge`, `pinned`) using the local file as the new content. For `update` strategy, compare the local file against the existing destination file; only overwrite and report as "updated" if the content differs. If the content is identical, report as "unchanged" (or omit from the summary). Same semantics as before — no network round-trip per file.
4. Apply placeholder substitution after reading the local source, before writing to the destination. Same rules as documented in **Placeholder Substitution** below, including the `ductus.md` self-install exception that keeps `{project}` and `{cli-config-dir}` literal.

### Cleanup

`/ductus` does not delete the temp directory. The path is logged in the post-scaffolding summary (and, on abort, in the error message) so the user can inspect it if needed. Both macOS (`/var/folders/.../T/`) and Linux (`/tmp` on systemd-tmpfiles distros) sweep their temp directories automatically; a few hundred KB of extracted files waiting for the next sweep is acceptable in exchange for not granting an `rm -rf` permission to the bootstrap.

The leftover directory is for inspection only — the next `/ductus` run creates its own fresh temp directory via `mktemp` and never reuses a prior extract.

## Frontmatter Migration

If `specs/` does not exist (first run), skip this section — there is nothing to migrate.

Bring existing spec and scenario files into the YAML frontmatter format declared in `framework/constitution.md` §text-first-artifacts. Migration is idempotent: re-running on an already-migrated project produces no further metadata changes.

This section runs **after the Pre-flight Phase** so that a stale-ductus abort cannot leave migration changes from old rules on the working tree. The new ductus's migration logic — which may differ — is the only logic that ever writes migration changes.

### Precheck

Run `git status --porcelain -- specs/` (project-relative). If the output is non-empty, refuse with:

> Migration requires a clean working tree under `specs/`. Commit or stash your changes, then re-run.

Exit before any modifications. Unrelated in-flight work outside `specs/` does not block migration.

### Walk

For each file matching one of:

- `specs/**/spec.md`
- `specs/**/scenarios/*.md`

Determine whether the file needs migration:

- Read the first non-blank line of the file. If it is `---`, the file already has frontmatter — skip with reason "already frontmatter."
- Otherwise, scan the first few lines after the heading for bold-prefix metadata patterns (`**Status:**`, `**Dependencies:**`, `**spec-ref:**`). If at least one is found, the file needs migration.
- If no bold-prefix lines are present and no frontmatter exists, skip with reason "no metadata to migrate."

Skip files that appear in `.ductus/config.toml` `pinned.files` with reason "pinned." The adopter is responsible for migrating pinned files manually.

### Convert

For each file that needs migration:

**Spec files** (`spec.md`):

- Extract `**Status:** {value}` and `**Dependencies:** {value}` from the body.
- For dependencies, parse the comma-separated slug list. The literal value `none` becomes an empty list (`[]`).
- Preserve any additional bold-prefix fields the project may have added (e.g., `**Track:** lightweight` becomes `track: lightweight` under the open-schema rule).
- Construct the YAML frontmatter block:

  ```yaml
  ---
  status: {value}
  dependencies: [{slug, slug, ...}]
  tags: []
  ---
  ```

- Remove the bold-prefix lines from the body.
- Insert the frontmatter block at the very top of the file, with one blank line separating it from the heading.

**Scenario files** (`scenarios/{slug}.md`):

- Extract `**spec-ref:** {value}` from the body.
- Construct the YAML frontmatter block:

  ```yaml
  ---
  spec-ref: "{value}"
  tags: []
  ---
  ```

  Quote the `spec-ref` value because it conventionally contains an em-dash and spaces.

- Remove the bold-prefix line from the body.
- Insert the frontmatter block at the very top of the file, with one blank line separating it from the heading.

### Edge cases

- **Partially migrated file** (frontmatter present and bold-prefix lines also present in body): the precheck above treats this as "already frontmatter" and skips. The user may run a manual cleanup pass; the migration does not attempt mixed-state recovery.
- **Malformed bold-prefix metadata** (e.g., missing `**Status:**` line, typo in field name, unparseable value): log a warning to the summary as `skipped (malformed metadata): {file path}` with a brief reason. The user repairs manually before re-running.
- **Bold-prefix metadata with custom fields**: preserved as additional frontmatter fields under the open-schema rule.

### Summary

Print a per-file summary at the end of the migration step:

- `migrated: {file path}` for converted files
- `skipped (already frontmatter): {file path}` for files that were already in the new format
- `skipped (pinned): {file path}` for files listed in `.ductus/config.toml`
- `skipped (no metadata to migrate): {file path}` for files without recognizable metadata
- `skipped (malformed metadata): {file path} — {reason}` for files that could not be parsed

The user reviews the result via `git diff` and commits or aborts via `git restore`. No backup directory is created — git is the recovery mechanism.

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

**CLAUDE.md** (strategy: skip, `claude-style` only) — written only when at least one selected agent is `claude-style`. If it exists, leave it alone. Otherwise, when a `claude-style` agent is selected, fetch `framework/templates/project/claude-md.md` from the `ductus` repo and copy it as `CLAUDE.md`. `claude-style` agents read `CLAUDE.md` natively (see each row's `rules_file_note`); the `antigravity` and `opencode` layouts read `AGENTS.md` natively and do not need `CLAUDE.md`, so an **Antigravity-only** or **OpenCode-only** adoption ships no `CLAUDE.md`. (`AGENTS.md` is still written for every adoption, as below.)

**.gitignore** (strategy: merge) — install or update a framework-managed block delimited by a `# ductus` line preamble, then dedup any adopter-area copies of canonical patterns. Mirrors the runtime `merge-managed-block` contract (line-prefix style, marker `ductus`):

1. Fetch `framework/templates/project/gitignore` from the `ductus` repo. This is the **canonical block** — including its blank-line-separated subsections.
2. If `.gitignore` does not exist, create it with `# ductus\n{canonical-block}\n`. Skip to step 5 for language patterns.
3. If `.gitignore` exists and contains a `# ductus` line preamble, replace the managed region (the `# ductus` line through the rest of the block — note the canonical block itself contains blank lines between subsections, so do not stop at the first interior blank) with `# ductus\n{canonical-block}\n`. If no `# ductus` line is present, append `# ductus\n{canonical-block}\n` after the existing content, separated by exactly one blank line.
4. **Dedup pass (canonical-block wins).** After the managed block is in place, scan the rest of the file (everything outside `# ductus` through the canonical block's end) and remove any non-blank, non-comment line that string-equals a non-blank, non-comment line inside the canonical block. Adopter-area blank lines and comment lines are preserved untouched even when they happen to share text with a canonical pattern. This collapses duplicates that an adopter (or another command) pasted above or below the marker; the canonical copy inside `# ductus` is the surviving one.
5. For each primary language provided by the user, fetch from `https://raw.githubusercontent.com/github/gitignore/main/{Language}.gitignore` and append below a `# {Language}` comment header. If the file is being re-merged on a subsequent run and a `# {Language}` section is already present, leave it alone — language sections, once written, are adopter territory.

## Security Audit (brownfield)

Run a one-time security audit when the project newly receives a security rule file alongside existing feature specs. This is the brownfield-adoption hook described in `specs/008-security-rules/spec.md` — it routes findings through `specs/inbox.md` so the adopter can triage them via `/{project}:groom` at their own pace, rather than having every legacy spec immediately fail validate.

### Trigger

Run the audit only when **both** conditions hold after the **Shared Files** manifest pass has completed:

1. At least one of `specs/rules/security-backend.md` or `specs/rules/security-frontend.md` was **newly created** by the manifest pass (the destination file did not exist before this run). A file that was merely updated or unchanged does not trigger the audit.
2. The project contains at least one feature spec directory under `specs/` matching the `NNN-*` pattern (zero-padded, three-digit prefix followed by a hyphen and a slug).

If either condition fails, skip this section silently — no output, no finding, no inbox entry. This covers the two routine cases:

- **Greenfield adoption** — no `specs/NNN-*/` directories exist, so the audit has nothing to scan against.
- **Routine re-run** — the rule files were created on a prior run; the manifest pass reports them as "updated" or "unchanged" rather than "created".

### Loading rule files

For each rule file that passed the trigger:

1. Read the file from its destination path (`specs/rules/security-backend.md` or `specs/rules/security-frontend.md`).
2. Apply the same integrity checks `/{project}:analyze` uses for the security-rule check section: well-formed level-3 headings of the form `### {ID}`, the four required fields (Statement, Rationale, Verification, Source), an ID matching `{FE|BE}-{CATEGORY}-{NNN}`, and no duplicate IDs within the file.
3. If a file fails any integrity check, report `Security audit: {path} failed to load — {reason}; skipping audit for this file.` and continue with the other rule file (if applicable). Do not abort the surrounding `ductus` run.

This mirrors validate's posture — partial or guessed-at parsing produces unreliable findings, so an unloadable file is treated as absent for audit purposes.

### Per-rule check

For each rule that loaded successfully:

1. Identify the artifacts in scope: `specs/NNN-*/spec.md`, `specs/NNN-*/plan.md`, and any `specs/NNN-*/scenarios/*.md`.
2. Read the rule's **Verification** field. The field describes the trigger — what makes the rule applicable to a given artifact — and the commitment the artifact must include when triggered.
3. For each artifact whose content fires the rule's trigger but does not include the required commitment, produce one finding.

Rules whose Verification trigger does not fire for any artifact produce no finding (the contextual-application property — silently inert when no spec exercises the rule's surface).

### Writing findings to the inbox

Each finding is one line appended to `specs/inbox.md`:

```text
- [ ] {Rule ID}: {affected artifact path} does not address — {one-line summary}
```

The `{one-line summary}` describes the gap concretely (e.g., `does not name a memory-hard password hashing algorithm`, `does not specify an output encoding strategy`). Prefixing each line with the rule ID makes related findings group naturally during `/{project}:groom` and gives the adopter a stable handle for cross-referencing.

### Deduplication

Before appending each finding, scan the existing `specs/inbox.md` (if it exists) for any line beginning with `- [ ] {Rule ID}: {affected artifact path}` — the prefix up to the first em-dash. If a matching line is already present, skip the new finding. This makes the audit safe to re-trigger after a user deletes and re-installs a rule file.

Findings the user has already groomed (lines that have been removed or rewritten) are not re-emitted — once the adopter has triaged a finding, `ductus` does not resurrect it.

### Audit summary

Track the count of newly appended findings (post-deduplication). The total is reported by **Post-Scaffolding Output**; when the count is zero, the audit-summary line is omitted entirely.

## Per-Agent Scaffolding

For each selected agent (in registry row order), run these steps with `{config_dir}` resolved to the agent's value and `{key}` to the agent's key.

The steps below describe the **`claude-style`** layout. For an agent whose registry `layout` is **`antigravity`**, apply **### Antigravity layout** below in place of **### Slash commands** and **### Slash command cleanup**. The `ductus` self-install, the **Pre-flight Phase**, the **Post-Write Integrity Check**, and **Placeholder Substitution** each carry their own `layout: antigravity` branch in their own sections.

For an agent whose `layout` is **`opencode`**, apply **### OpenCode layout** below in place of **### Slash commands** and **### Slash command cleanup**. OpenCode's installer is a **verbatim markdown file** (no skill wrapper), so the `ductus` self-install, **Self-update check**, **Post-Write Integrity Check**, and **Placeholder Substitution** follow the **`claude-style`** path — with the command directory `command/` (singular) and `{cli-config-dir}` resolving to `.opencode`.

### Slash commands (strategy: update)

Fetch each command template and copy it into `{config_dir}/commands/{project}/`. In each copied file, replace `{project}` with the user-provided project name and `{cli-config-dir}` with `{config_dir}`.

| Source Path | Destination Path |
| --- | --- |
| `framework/commands/amend.md` | `{config_dir}/commands/{project}/amend.md` |
| `framework/commands/clarify.md` | `{config_dir}/commands/{project}/clarify.md` |
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

**Skills (strategy: update).** For each row in the slash-command manifest above — the fourteen `framework/commands/*.md` rows plus the `framework/bootstrap/configure/{key}.md` configure row — transform the source into a dir-form skill at `{config_dir}/skills/{project}-{name}/SKILL.md` (instead of copying to `{config_dir}/commands/{project}/{name}.md`):

1. Read the source markdown (frontmatter + body).
2. Write `{config_dir}/skills/{project}-{name}/SKILL.md` with frontmatter `name: {project}-{name}` and the `description:` carried from the source frontmatter, followed by the source body.
3. Substitute `{project}` and `{cli-config-dir}` in the body exactly as in the `claude-style` copy (`{cli-config-dir}` → `.agents`).

`{name}` is the command's base name (`specify`, `clarify`, …; the configure row's `{name}` is `configure`). The skills are invoked as `/{project}-{name}`.

**Rules (strategy: update).** Mirror **every** `*.md` file present in `specs/rules/` into `{config_dir}/rules/{name}.md`, so Antigravity loads them natively. The source is a **directory walk of what is on disk**, not the **Shared Files** manifest rows: the manifest ships `ductus`'s own rule files, but a project may author its own (constitution §rules Lifecycle), and those are discovered by the same walk `/{project}:review` and `/{project}:analyze` use. Mirroring only the manifest rows would leave a project's own rules enforced by the pipeline but absent from Antigravity's native loading — the one agent with a native rules dir would see a smaller rule set than every other agent. `ductus`-shipped files regenerate from `framework/rules/` on every `/ductus` run; project-authored files are mirrored as they stand. `specs/rules/` stays the pipeline-read location for every agent; `{config_dir}/rules/` is the Antigravity-native mirror. The `specs/rules/` write itself (in **Shared Files**) is layout-independent and unchanged.

**Rules mirror cleanup.** List the `*.md` files under `{config_dir}/rules/`. Delete any whose basename no longer exists in `specs/rules/` and is not listed in `.ductus/config.toml` `pinned.files`; report removals and pinned-keeps as for the skill cleanup below. Without this, a rule file deleted or renamed in `specs/rules/` keeps loading from the mirror — Antigravity would enforce a rule the pipeline has already dropped.

**Skill cleanup (replaces Slash command cleanup).** List the skill directories under `{config_dir}/skills/` whose name matches `{project}-*`. Delete any `{config_dir}/skills/{project}-{name}/` whose `{project}-{name}` is not produced by the skills manifest above and is not listed in `.ductus/config.toml` `pinned.files`; report removals and pinned-keeps as for the `claude-style` cleanup. Skill dirs outside the `{project}-*` namespace (and the `ductus` skill) are adopter/agent territory and are never touched.

### OpenCode layout (`layout: opencode`)

When the agent's registry `layout` is `opencode`, the two subsections above (**Slash commands**, **Slash command cleanup**) are replaced by the equivalents below. `{config_dir}` resolves to `.opencode`; OpenCode discovers markdown commands under `{config_dir}/command/` (singular), namespaced by subdirectory.

**Commands (strategy: update).** For each row in the slash-command manifest above — the fourteen `framework/commands/*.md` rows plus the `framework/bootstrap/configure/{key}.md` configure row — copy the source **verbatim** (frontmatter + body, no skill transform) to `{config_dir}/command/{project}/{name}.md` (instead of `{config_dir}/commands/{project}/{name}.md`). Substitute `{project}` and `{cli-config-dir}` (→ `.opencode`) in the body exactly as in the `claude-style` copy, and carry the `description` frontmatter as-is. `{name}` is the command's base name (the configure row's `{name}` is `configure`). The commands are invoked `/{project}/{name}` — OpenCode namespaces by subdirectory (verified: `command/ductus/specify.md` registers as command key `gov/specify`).

**Command cleanup (replaces Slash command cleanup).** List the `.md` files under `{config_dir}/command/{project}/`. Delete any whose base name is not produced by the manifest above and is not listed in `.ductus/config.toml` `pinned.files`; report removals and pinned-keeps as for the `claude-style` cleanup. Files outside the `{project}/` subdirectory are adopter/agent territory and are never touched.

**Rules.** OpenCode reads `AGENTS.md` natively (via its `instructions` resolution) and the pipeline reads the shared `specs/rules/` directly — there is **no** native rules-dir mirror (unlike `antigravity`). Nothing extra to scaffold.

**MCP + permissions.** Both the `ductus` `mcp` block and the `permission` set live in the committed root `opencode.json` — seeded by §Permission Setup, wired by §ductus runtime detection (State-B `write-file`), and completed by `/{project}:configure`. See §Derived values and §MCP registration.

### Session state

The session state file lives at `.ductus/session.toml` (earlier locations — `.govern/session.toml`, then the pre-042 repo root `.govern.session.toml` — are read and written via the active-file rule until the directory migrations move it; specs 042, 049) — a single uniform path for every adopter, project-name-agnostic, gitignored, and **per-contributor**. It carries two things: the session target (feature, optional scenario, `set-at`), written on each `/{project}:target` (or its scenario sibling) invocation; and the contributor's `cli-config-dir`, written by `/ductus` at adoption (§Instructions step 7) — the one place an agent-specific value belongs, since teammates on one project may use different agents. Both are written by the runtime's `write-session` primitive (a target write preserves `cli-config-dir`; a host-config write preserves the target), or on the markdown-only path by the host's file-writing tool. There is no per-agent session state beyond this one file.

### `ductus` self-installation (strategy: update)

Fetch `framework/bootstrap/ductus.md` and write it to the agent's `ductus` install path: `{config_dir}/commands/ductus.md` for `claude-style`, `{config_dir}/command/ductus.md` for `opencode`, or `{config_dir}/skills/ductus/SKILL.md` for `antigravity`. This is the same unified file the user is currently running, installed into every selected agent so the command is invokable from that agent on subsequent runs. For `antigravity`, wrap the body in `name: ductus` frontmatter (the dir-form skill); for `claude-style` and `opencode` the file is the verbatim `ductus.md`. The body keeps every placeholder literal (next paragraph).

In this file (and only this file), keep **every** placeholder literal — do **not** substitute anything. `{project}` and `{cli-config-dir}` must stay literal so `ductus` itself can read `$ARGUMENTS` and the per-agent config dir on each run; `{project-name}` and `{One-line project description.}` must stay literal because this file's prose *documents* those placeholders for the AGENTS.md template — substituting them would corrupt the documentation, not personalize a value.

After writing, run the **Post-Write Integrity Check** below.

## Hook Installation

After **Per-Agent Scaffolding** completes, manage the project's git pre-commit hook so generated artifacts (currently spec `dependencies:` and `references:` frontmatter, future generators if added) stay in sync on every commit.

Two files participate, with different ownership models:

- **`.githooks/ductus-pre-commit`** is ductus-owned. Placed by the **Shared Files** manifest with `update` strategy; carries the `# managed-by: ductus` sentinel on line 2; rewritten on every `/ductus` run unless pinned in `.ductus/config.toml`. Holds the derivation orchestration (currently `ductus derive-dependencies --write --staged` and `ductus derive-references --write --staged` plus output staging). Both run with `--staged` so a commit only rewrites the specs it touches, never unrelated ones. An unreachable runtime **halts** the commit rather than skipping the pass: these primitives produce derived frontmatter the commit captures, so a silent skip would land values they had already superseded. That is safe to make blocking because the hook cannot outrun the binary — `/ductus` wires `core.hooksPath` and acquires the runtime in the same run, and `core.hooksPath` is local git config a clone never carries, so a fresh clone has no hook until `/ductus` has run. `git commit --no-verify` is the deliberate bypass. It additionally runs `ductus label-criteria` once per staged spec — the acceptance-criterion labelling pass (spec 013), the backstop for a criterion typed by hand in an editor. That step keeps a *swallowed* failure, and the reason is blast radius rather than optionality: a missing `AC{n}` label is caught by `/ductus:analyze` and assigned on the next pass, so nothing wrong is committed, while a stale derived index is committed wrong data.
- **`.githooks/pre-commit`** is adopter-owned. Placed by the manifest with `create` strategy on first install; never overwritten thereafter. Initial content invokes `./.githooks/ductus-pre-commit`; adopters add their own pre-commit checks above or below that invocation.

This section's job is to wire git up to actually run the outer hook (`git config core.hooksPath .githooks`) without clobbering whatever hook system the project already uses.

Detection runs in this order — first match wins:

1. **`core.hooksPath` already points at `.githooks`** — already wired up. The manifest passes have already written `.githooks/ductus-pre-commit` (`update`) and, on first run, `.githooks/pre-commit` (`create`). Run `chmod +x .githooks/pre-commit .githooks/ductus-pre-commit` to ensure both files are executable. Report `pre-commit hook already wired up`.
2. **`core.hooksPath` points at any other path** — the project uses a custom hooks dir. Skip wiring; report a warning with the manual integration snippet below.
3. **A third-party hook system is detected** — any of `.husky/`, `.pre-commit-config.yaml`, `lefthook.yml`, or `lefthook-local.yml` exists. Skip wiring; report a warning with the manual integration snippet below.
4. **No conflicts** — run `git config core.hooksPath .githooks` and `chmod +x .githooks/pre-commit .githooks/ductus-pre-commit`. Report `pre-commit hook installed`.

The detection ladder no longer treats `.githooks/pre-commit` itself as a ductus-managed file — under the new model the outer file is adopter-owned, so its presence is not a signal that ductus installed it. Migration of pre-existing ductus-installed hooks (from spec-017 adopters) is handled by the **Migration from spec-017 hook** subsection below, which runs before the detection ladder.

The two frontmatter derivations are **runtime primitives**, not shipped scripts: `ductus derive-dependencies` and `ductus derive-references` (spec 022, `adopter-generator-promotion`). They arrive with the runtime `/ductus` acquires, so there is nothing to scaffold, refresh, or pin — a generator fix reaches adopters through the version bump that ships the binary. The pre-existing `.ductus/scripts/` entries were removed by the `generator-primitives` migration.

### Migration from spec-017 hook

Adopters who installed the pre-commit hook under spec 017 have a single ductus-managed file at `.githooks/pre-commit` carrying the `# managed-by: ductus` sentinel on line 2. The new layout splits that file into a ductus-owned inner script and an adopter-owned outer stub at the same path. Migration runs **before** the detection ladder above and **before** the manifest passes for the two hook files, so the manifest's `update`/`create` strategies see the post-rename layout.

Trigger:

- `.githooks/pre-commit` exists, AND
- the file's line 2 is exactly `# managed-by: ductus`, AND
- `.githooks/ductus-pre-commit` does **not** exist.

When all three hold, perform the rename:

1. Determine whether the file is tracked: `git ls-files --error-unmatch .githooks/pre-commit` (exit code 0 = tracked).
2. If tracked: `git mv .githooks/pre-commit .githooks/ductus-pre-commit`. If untracked: `mv .githooks/pre-commit .githooks/ductus-pre-commit`.
3. Continue with the detection ladder and the manifest passes. The renamed inner file is byte-identical to upstream for unmodified adopters, so the `update` strategy on `.githooks/ductus-pre-commit` is a no-op; the `create` strategy on `.githooks/pre-commit` writes the new outer stub since the path is now empty.
4. Append to the post-scaffolding summary: `migrated pre-commit hook: .githooks/pre-commit → .githooks/ductus-pre-commit; created adopter-owned .githooks/pre-commit stub`.

Recovery branches:

- **Pre-existing `.githooks/ductus-pre-commit` blocks the rename.** If the inner-file destination already exists when the trigger fires, abort the rename without renaming anything. Report `migration skipped: .githooks/ductus-pre-commit already exists; resolve manually` and continue with the detection ladder and manifest passes. The `update` strategy overwrites the pre-existing inner with the shipped contents; the existing `.githooks/pre-commit` (still carrying the sentinel) is left in place but is no longer detected as ductus-managed by the new ladder, so it is treated as adopter-owned going forward. The adopter resolves the duplicate manually.
- **`git mv` fails (permissions, repo locked, file in use).** Report `migration failed: could not rename .githooks/pre-commit; resolve manually` and continue with the detection ladder and manifest passes. The `update` strategy installs `.githooks/ductus-pre-commit` from scratch (destination doesn't exist); the `create` strategy sees `.githooks/pre-commit` still in place and skips. The adopter ends up with both files (legacy sentinel'd outer still functional, new ductus-owned inner idle) and completes the migration manually by editing the outer to call `./.githooks/ductus-pre-commit`.

If any of the trigger conditions does not hold, skip the migration silently — the detection ladder handles the case.

### Manual integration snippet (for skip cases)

When detection skips installation (cases 2 and 3 above), report this message to the user:

> The `ductus` pre-commit hook was not wired up because your project already uses an existing hook system. To get automatic spec-deps regeneration on every commit, add this line to your existing pre-commit chain:
>
> ```bash
> ./.githooks/ductus-pre-commit
> ```
>
> The shipped hook script is idempotent and safe to call from another hook runner.

### Pinning

Both hook files are subject to `.ductus/config.toml` `pinned.files`, but the meaning differs by ownership:

- **`.githooks/ductus-pre-commit`** is the only file pinning is meaningful for. A pinned inner file uses `skip` strategy instead of `update` — `/ductus` does not overwrite it across releases. Useful when an adopter has customized ductus's generator orchestration and does not want it reset.
- **`.githooks/pre-commit`** is `create`-strategy and never overwritten after first run regardless of pinning. Listing it in `pinned.files` is harmless but has no effect.

The Hook Installation section above still runs and may set `core.hooksPath` regardless of pinning.

## Placeholder Substitution

In every copied file (except each selected agent's installed `ductus` file — `{config_dir}/commands/ductus.md` for `claude-style`, `{config_dir}/command/ductus.md` for `opencode`, `{config_dir}/skills/ductus/SKILL.md` for `antigravity` — whose body keeps `{project}` and `{cli-config-dir}` as literal placeholders), replace:

- `{project}` with the user-provided project name (used in commands, README)
- `{project-name}` with the user-provided project name (used in AGENTS.md template)
- `{One-line project description.}` with the user-provided description
- `{cli-config-dir}` with the agent's `config_dir`

## Post-Write Integrity Check

After writing the agent's installed `ductus` file — whether via the **Pre-flight Phase** (stale-write path) or the **`ductus` self-installation** manifest step — verify it is well-formed. For `claude-style` (`{config_dir}/commands/ductus.md`) and `opencode` (`{config_dir}/command/ductus.md`), the file must start with a frontmatter block carrying a `description:` key, and the body after that frontmatter must start with `# ductus` — the installed copy is written verbatim, so the source's own frontmatter travels with it, and asserting against the first line alone fails on every correct write. For `antigravity` (`{config_dir}/skills/ductus/SKILL.md`), the file must start with a frontmatter block whose `name:` is `ductus`, and the body after that frontmatter must start with `# ductus`. If the check fails, the write was corrupted — report the error and re-read the source: `{tempdir}/ductus.md.upstream` for the self-update path, or `{tempdir}/ductus-main/framework/bootstrap/ductus.md` for the manifest path. Apply the check independently per agent.

## Re-Run Behavior

`/ductus` is idempotent and additive across agents:

- **Re-run with the same selection** — applies the manifest's `update` strategy to the agent's slash commands and refreshes shared files. `create`-strategy files are skipped if present. `skip`-strategy files are never overwritten.
- **Re-run adding a new agent** — scaffolds the new agent's tree from scratch alongside the existing one. The existing agent's command dir, settings, and session JSON are not touched.
- **Re-run removing an agent** — this command does not delete an agent's tree on its own. Removing an adopted agent is a manual `rm -rf {config_dir}` operation outside `/ductus`'s scope.

## What This Command Does NOT Do

- Modify `README.md` — the project's README is its own
- Create feature specs — the user does that via `/{project}:specify`
- Fill in AGENTS.md content — that requires project-specific knowledge
- Fill in system.md content — that requires architectural decisions
- Make git commits — the user decides when to commit
- Run `/{project}:configure` — that happens after adoption, interactively
- Delete an agent's adopted tree — manual cleanup

## Edge Cases

- **Unknown agent key in `--agents=`** — stop before scaffolding; report the unknown key with the list of valid keys.
- **All supported agents already adopted with `--add-agent`** — show the prompt with all agents pre-selected; if the user confirms with no additions, treat it as a routine update and continue silently.
- **`settings.local.json` already has entries beyond the bootstrap** — only add the curl/ls bootstrap entries if missing. Do not overwrite, deduplicate, or reorder entries added by `/{project}:configure` or by the user.
- **`ductus.md` content already matches the version on disk** — when the manifest's `update` strategy compares fetched content to the installed file, identical content reports as "unchanged" and avoids a redundant write. Same rule applies to per-project `configure.md` and other update-strategy files.
- **Pinned `ductus.md` in `.ductus/config.toml`** — the manifest's `update` strategy still skips the file (no overwrite), and the **Pre-flight Phase**'s self-update check never writes pinned files even on the stale-detect path. The check byte-compares anyway: matching upstream → recorded as `current`, no output; divergent from upstream → recorded as `pinned-divergent`, the run continues, and a single advisory line is printed in the post-scaffolding output. A pinned `ductus.md` will not pick up upstream changes until the pin is removed, but the user is told once when the pin is currently suppressing real divergence.
- **Self-update check sees a stale `ductus` in an unselected adopted agent** — the check is scoped to selected agents only. The unselected agent's stale copy is not diffed, not written, and does not trigger the abort; it will be detected the next time the user runs `/ductus` against it.
- **Self-update small fetch fails** — clean abort with the error message defined in **Pre-flight Phase → Self-update check → Small fetch**. No `ductus.md` writes occur, and the archive fetch is skipped. The user re-runs after the transient failure clears.
- **Archive fetch or extract fails** — clean abort with the error message defined in **File Fetching → Archive fetch and extract**. The pre-flight phase has already passed by this point, so no additional `ductus.md` or ductus-wiring writes are pending; the user re-runs after the transient failure clears.
- **A required source file is absent from the extracted archive** — warn `Source not found in archive: {source-path}; skipping.` and continue with the remaining manifest entries. Preserves the per-entry "do not abort on a single fetch error" guarantee at the entry level even though the archive itself is fetched once.
- **First-run prompt with no detected dirs and only one supported agent** — the prompt still appears (the agent must be explicitly chosen), but the single agent is pre-selected. Confirming is one keystroke.
- **Running `ductus.md` cannot infer its own install path** — fall back to no pre-selection in the first-run prompt. The user picks explicitly.
- **ductus runtime not live (State B)** — the **Pre-flight Phase** acquires the pinned release into the store, materializes the pointer, and registers the runtime per the agent's `mechanism` (writes the MCP config for a `write-file` agent, or surfaces the registration command for a `surface-instruction` agent), then joins the pending-restart set.
- **ductus wiring file is malformed JSON** — the wiring write does not touch the file. `/ductus` skips wiring and warns the user to repair it; the runtime is still acquired and the pointer still materialized, so the next run wires it once the file parses. A hand-maintained config is never clobbered.
- **ductus store probe cannot run or is denied** — the run is classified as State B and acquisition proceeds. Acquisition is idempotent, so a false negative costs a version comparison rather than a redundant download. Detection never hard-fails on a host without shell.
- **Stale `ductus.md` on an adopter who has never wired ductus** — both pre-flight checks contribute writes (a fresh `ductus.md` and the ductus wiring), but the **Pre-flight abort** emits one combined message and the user restarts once, not twice.

## Post-Scaffolding Output

After scaffolding, display:

- Summary of files created, updated, unchanged, skipped, pinned, merged, and removed — grouped by agent for per-agent files, with shared files in their own group
- For each scaffolded agent, the agent's `rules_file_note` from the registry
- Hook installation status — one line: `pre-commit hook installed`, `pre-commit hook already wired up`, or `pre-commit hook skipped — existing {husky|lefthook|pre-commit-py|core.hooksPath} detected; see manual integration snippet above`. When the spec-017 → spec-018 migration ran, append the migration summary line described in §Hook Installation > Migration from spec-017 hook (or the relevant recovery-branch warning if the rename was skipped or failed).
- Any fetch failures encountered
- Pinned `ductus.md` advisory (if applicable — see below)
- Security audit summary (if applicable — see below)
- ductus runtime tip (failed acquisition only — see below)
- Next steps (varies by mode):

### Closing restart

When the **deferred-restart set** is non-empty — State B wired `ductus` this run — append this after the summary, as the last thing the run says:

> **`ductus` was wired in this run and the work above is complete.** Every primitive ran through the CLI at `{pointer-path}`. Start a new session so the MCP server loads and the pipeline commands call the same primitives as tools.

The restart is for the **tool surface**, not for unfinished work — that distinction is the whole point of moving this here, and the message says which so the operator does not re-run `/ductus` expecting it to do more. One restart, not two: migrations, the archive fetch, Shared Files, and every scaffolding step already happened. An adopter carrying a `ductus.md` that predates the Pre-flight Phase still pays the separate self-update hop, because a copy that cannot execute this phase cannot be talked into it.

Two variants:

- **The wiring was skipped** because the agent's MCP file is not valid JSON: acquisition still happened and the pointer exists, so the work above still completed by CLI. Say the registration was skipped and name the file, rather than promising a tool surface that will not appear.
- **A `surface-instruction` agent** (Auggie, Antigravity) never gets an MCP file written by `/ductus` at all: keep surfacing the one-line registration command here, after the work rather than instead of it.

Nothing is emitted when the set is empty — a State A run has no restart to announce, and a line saying so would be noise on every run.

### ductus runtime tip

When acquisition was attempted and **failed** (the run halts, so this line accompanies the halt rather than a completed scaffold), append one line after the file summary:

> Tip: acquisition failed, so this run halted before the deterministic path was available — there is no markdown-only mode to degrade into. Recover either by placing the pinned binary into the store by hand (the halt message above names the store path and the release URL) or by setting `[runtime] path` in `.ductus/config.toml` to a binary you supply, then re-run `/ductus`. Your `PATH` is not consulted. See [The runtime](https://github.com/stonean/ductus#the-runtime) in the ductus README.

Omit the tip in **State A** (the runtime is already live) and **State B** (the run aborted in pre-flight before this output). State B's file disclosure rides the **Pre-flight abort** message, not this output.

### Pinned `ductus.md` advisory

If the **Pre-flight Phase** recorded any selected agent as `pinned-divergent` (the installed `ductus` file (`{config_dir}/commands/ductus.md`, or `{config_dir}/skills/ductus/SKILL.md` for `antigravity`) is listed in `.ductus/config.toml` `pinned.files` and differs from upstream), append one advisory line per divergent agent after the file summary and before next steps:

> {agent}: ductus.md pinned, upstream has changed.

The advisory is omitted when no agent is `pinned-divergent` — adopters whose pinned version still matches upstream see nothing; adopters with no pin see nothing. The check's `stale` path aborts before this output is ever produced, so the advisory is only ever about pinned files.

### Security audit summary

If the **Security Audit (brownfield)** section ran and appended one or more new findings to `specs/inbox.md`, append this single line to the file summary:

> {N} security audit items added to `specs/inbox.md`. Run `/{project}:groom` to triage.

Where `{N}` is the count of newly appended findings (after deduplication). Omit this line when:

- The audit did not run (trigger conditions did not fire — greenfield run, or routine re-run with rule files already present), OR
- The audit ran but every finding was already in the inbox (`N == 0`), OR
- The audit ran but produced no findings (no rule's Verification trigger fired against any existing artifact).

This summary complements `/{project}:groom`, which is the user's path to working through the inbox at their own pace.

### First run (no existing `specs/` directory)

---

**ductus adopted successfully.**

Adopted agents: {comma-separated `name` of selected agents}.

Next steps:

1. Run `/{project}:configure` in each adopted agent to apply the full permission set.
2. Fill in `AGENTS.md` — tech stack, project structure, code style, testing conventions, gotchas.
3. Fill in `specs/system.md` — architecture, request lifecycle, shared infrastructure.
4. Use `/{project}:log` to record any known issues or bugs into `specs/inbox.md`.
5. Run `/{project}:groom` to walk the inbox and route each item to its proper spec or scenario.
6. Create your first feature spec: `/{project}:specify {feature description}`.
7. The deterministic runtime is already acquired and wired by this run — you do not install it. See [The runtime](https://github.com/stonean/ductus#the-runtime) in the ductus README for how it is pinned and upgraded.

To adopt an additional agent later, re-run `/ductus --add-agent`.

Tip: `specs/` is plain markdown and works in any PKM tool (Obsidian, Logseq, Foam) or as a published site (Quartz, MkDocs). Pick whichever fits your workflow, or none.

---

### Update mode (existing `specs/` directory detected)

---

**ductus updated successfully.**

Updated agents: {comma-separated `name` of selected agents}.

Review changes to updated files and commit when ready. To adopt an additional agent, re-run `/ductus --add-agent`.

Tip: `specs/` is plain markdown and works in any PKM tool (Obsidian, Logseq, Foam) or as a published site (Quartz, MkDocs). `/ductus` keeps the deterministic runtime current on every run — see [The runtime](https://github.com/stonean/ductus#the-runtime) in the ductus README.

---

## Idempotency

This command is safe to run again. Files with `update` strategy are always overwritten with the latest `ductus` version — unless pinned in `.ductus/config.toml`, in which case they are skipped. Files with `create` strategy skip existing files. The `.gitignore` merge checks for the `# ductus` marker before appending. `skip` strategy files are never overwritten.

Re-runs are additive across agents — adopting a new agent leaves existing agents' files untouched.

## Directory Creation

Create intermediate directories as needed (e.g., `specs/`, `specs/templates/`, and — by layout — `{config_dir}/commands/{project}/` for `claude-style`, `{config_dir}/command/{project}/` for `opencode`, or `{config_dir}/skills/` and `{config_dir}/rules/` for `antigravity`).

Throughout this command, every `specs/…` destination — the §Shared Files manifest rows (`specs/system.md`, `specs/inbox.md`, `specs/rules/…`, …), the directories created here, and the spec-root named in the §Post-Scaffolding Output — is written under the configured `[paths] specs-root` (default `specs`, resolved in §Collect Project Inputs). The literal `specs/` paths in the manifest tables and prose are the documented default; substitute the configured name when the operator has set one. This keeps the manifest readable while honoring the override (spec 040).
