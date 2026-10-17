---
status: in-progress
dependencies: [012-multi-agent-govern, 022-deterministic-runtime]
next-criterion: 12
---

# 028 — Antigravity Agent Support

Generalize the agent registry from "two agents that share one layout" to "N
agents across differing per-agent layouts and host conventions," then use that
generalization to add Google's **Antigravity CLI** (`agy`) as the third
supported agent. Claude Code and Auggie adopt ductus by **file-scaffolding into
a per-project config dir** (`.claude/`, `.augment/`): markdown slash commands
under `{config_dir}/commands/{project}/`, a native rules file (`CLAUDE.md`), and
permissions in `{config_dir}/settings.local.json`. Antigravity uses the **same
file-scaffold model** but a **different layout**: workspace skills under
`.agents/skills/`, native rule files under `.agents/rules/`, and permissions in
`.agents/settings.json` with an `allow`/`deny`/`ask` schema. Closing that gap is
the work — once the registry models per-agent **command/skill location**,
**settings file + permission format**, **rules location**, and **native rules
file**, the *next* agent on an existing layout becomes a true one-row append,
which is the promise [012-multi-agent-govern](../012-multi-agent-govern/spec.md)
made but only delivered for `.claude`-style agents.

How an agent registers the ductus runtime as an MCP server is **not** part of
its layout. `031-agent-mcp-wiring` made it a per-agent descriptor (`target` /
`scope` / `mechanism`), because agents sharing a layout register MCP servers in
different places: Claude Code in the project's committed `.mcp.json`, Auggie in
user-global `~/.augment/settings.json`, and Antigravity in home-level
`~/.gemini/config/mcp_config.json`. Later specs added further layouts on the
same generalization — `032-opencode-agent` added `opencode` and
`064-pi-host-support` added `pi` — each a branch alongside the two this spec
defines.

> **Why Antigravity, not Gemini CLI.** This spec originally targeted Gemini CLI.
> Google is shutting down Gemini CLI on **2026-06-18** (no grace period for
> Pro/Ultra/free tiers) and migrating users to Antigravity CLI. Building support
> for a CLI that retires within days is wasted effort, so the concrete target is
> Antigravity.
>
> **Source provenance.** Antigravity's official docs are client-rendered and
> could not be fetched directly. The model below was established from (a) doc
> excerpts supplied by the maintainer, (b) **direct probing of the installed
> CLI** (`agy 1.0.7`, macOS: `agy plugin validate/install/list/uninstall`,
> `agy -p` live sessions), and (c) cross-checks between the two. One claim — that
> Antigravity reads `.claude/` — came from asking the `agy` *agent* about its own
> config; it was **empirically refuted** (a live `agy` session loaded a
> workspace skill from `.agents/skills/<name>/SKILL.md` and reported its exact
> path, while identical probes under `.claude/commands/` and `.claude/skills/`
> did not load). A second, from the docs of the day — that Antigravity loads MCP
> servers from a project-local `.agents/mcp_config.json` — was refuted by
> `031-agent-mcp-wiring`, which measured 0 server spawns there against a
> positive control that did spawn. Self-reports and unprobed docs are treated
> as unverified; observed behavior governs.

## Problem

[012-multi-agent-govern](../012-multi-agent-govern/spec.md) had collapsed the
per-CLI bootstrap variants into one unified file driven by an **Agent
Registry** (defined in `framework/bootstrap/ductus.md`). Its closing
contract — *"Adding a new agent is a one-row addition to the registry"* plus a
`configure/{key}.md` file and a README curl snippet, with *"No other changes are
required"* — held only because Claude and Auggie shared layout assumptions baked
into the unified procedure:

1. **Commands** lived at `{config_dir}/commands/{project}/<name>.md` and resolved
   to `/{project}:<name>`.
2. **The native rules file was `CLAUDE.md`**, read by both agents.
3. **Permissions** were per-tool allowlists in
   `{config_dir}/settings.local.json`, populated by `/{project}:configure`.

**Antigravity keeps the file-scaffold model but diverges on every one of those
specifics.** Its invocable units are **skills** (`.agents/skills/<name>/SKILL.md`,
frontmatter `name` + `description`, invoked `/<name>`), not
`commands/{project}/`. It reads **`AGENTS.md`** natively, not `CLAUDE.md`. And
its permissions live in **`.agents/settings.json`** under a `permissions` object
with **`allow`/`deny`/`ask`** arrays whose entries use an action grammar
(`command(...)`, `read_file(...)`, `mcp(server/tool)`, `unsandboxed(...)`, …) —
a *third* format alongside Claude's `permissions.allow/deny` and Auggie's
`toolPermissions[]`. Appending an Antigravity row to the registry as it stood
would have scaffolded commands into a directory Antigravity never scans. This
spec was therefore a **framework change** — generalizing the registry's layout
and host-convention assumptions — not a row append.

### The structural divergences

| Dimension | Claude / Auggie | Antigravity CLI |
| --- | --- | --- |
| Adoption | file-scaffold into config dir | file-scaffold into config dir (same model) |
| Config dir | `.claude/` / `.augment/` | `.agents/` |
| Invocable unit | markdown command, `commands/{project}/<name>.md` | **skill**, `.agents/skills/<name>/SKILL.md` (dir form; `name`+`description` frontmatter) |
| Invocation | `/{project}:<name>` (colon namespace) | `/<name>` (flat; ductus prefixes, e.g. `/{project}-<name>`) |
| Native rules file | `CLAUDE.md` | `AGENTS.md` (already shipped) |
| Domain rule files | filesystem `specs/rules/` | `.agents/rules/<name>.md` |
| Permissions | `settings.local.json` (`permissions.allow/deny` or `toolPermissions[]`) | `.agents/settings.json` (`permissions.allow/deny/ask`, action grammar) |
| MCP registration (per agent, not per layout) | Claude: project `.mcp.json`, written by ductus; Auggie: user-global `~/.augment/settings.json`, a command ductus surfaces | home-level `~/.gemini/config/mcp_config.json`, an edit ductus surfaces |

(Antigravity facts verified against `agy 1.0.7` and maintainer-supplied docs;
see Resolved Questions for the per-fact provenance, and `031-agent-mcp-wiring`
for the MCP row.)

## Verified Antigravity Layout

ductus adopts Antigravity by scaffolding into the project-local **`.agents/`**
directory (the workspace scope — verified: a live `agy` session loaded a
workspace skill at `<repo>/.agents/skills/<name>/SKILL.md`):

| ductus artifact | Antigravity destination | Notes |
| --- | --- | --- |
| Pipeline commands (`framework/commands/*.md`) | `.agents/skills/{project}-<name>/SKILL.md` | dir-form skill; body = the command procedure (gates preserved as in-body prompts); `name` + `description` frontmatter; invoked `/{project}-<name>` |
| `ductus` installer | `.agents/skills/ductus/SKILL.md` | placed by `install.sh` for bootstrap |
| Domain rule files | `.agents/rules/<name>.md` | native Antigravity rule loading |
| ductus runtime | home-level `~/.gemini/config/mcp_config.json` | Antigravity ignores a project-local `.agents/mcp_config.json`, so ductus writes no MCP file — the user edits the home-level config once and reloads with `/mcp` |
| Permissions | `.agents/settings.json` | `permissions.allow/deny/ask`; ductus covered by a single `mcp(ductus/*)` |
| Project context | `AGENTS.md` (already shipped) | read natively; no `CLAUDE.md`, no new context file |

A **global plugin** form also exists (`agy plugin install` →
`~/.gemini/config/plugins/<name>/` with `plugin.json` + `skills/` + `rules/` +
`mcp_config.json`), but it is per-user/global and not how a per-project tool
adopts. It is recorded as an **optional, deferred** distribution channel (see
Resolved Questions / marketplace), not ductus's adoption path.

## Registry Generalization

The Agent Registry stops assuming the `.claude`-style layout. These become
per-layout registry-derived values rather than hard-coded constants:

- **Command/skill location & invocation** — `{config_dir}/commands/{project}/`
  with `/{project}:<name>` vs `.agents/skills/<name>/SKILL.md` with `/<name>`.
- **Settings file & permission format** — `settings.local.json`
  (`permissions.allow/deny` / `toolPermissions[]`) vs `.agents/settings.json`
  (`permissions.allow/deny/ask`, action grammar).
- **Rules location** — filesystem `specs/rules/` vs `.agents/rules/`.
- **Native rules file** — `CLAUDE.md` vs `AGENTS.md`.

MCP registration is per agent, not per layout (the descriptor `031-agent-mcp-wiring`
introduced). A future `.claude`-style agent that reads `CLAUDE.md` remains a
registry-row append plus its own files — the generalization must not tax the
common case. **Derived values** in `framework/bootstrap/ductus.md` is the live
table of per-layout values, including the layouts added after this spec.

## Per-Agent Adoption for Antigravity

For Antigravity, `/ductus` scaffolds the project-local `.agents/` tree:

- **Skills.** Transform each `framework/commands/*.md` into
  `.agents/skills/{project}-<name>/SKILL.md`: keep the body (the procedure, with
  its approval-gate prompts) unchanged, carry the `description` frontmatter, add
  the `name` field. Placeholder substitution (`{project}`, `{cli-config-dir}`)
  still applies. The body's `$ARGUMENTS` references are carried as written;
  Antigravity's argument-passing equivalent was never pinned.
- **Rules.** Mirror every rule file in `specs/rules/` — the framework's and any
  the project authors — to `.agents/rules/<name>.md`, and remove a mirrored file
  whose source is gone, so Antigravity loads the same rule set the pipeline
  enforces.
- **MCP.** ductus writes no MCP file for Antigravity. Registration is
  `surface-instruction`: the user adds a `ductus` block naming the absolute store
  path to home-level `~/.gemini/config/mcp_config.json` once per machine, then
  reloads with `/mcp`.
- **Permissions (`configure`).** `framework/bootstrap/configure/antigravity.md`
  writes `.agents/settings.json` `permissions` in Antigravity's action grammar:
  `mcp(ductus/*)` for the runtime, `command(...)` allows/denies for shell, with
  workspace files auto-allowed by default. `gen-configure-mcp.sh` emits the
  Antigravity MCP block (a single `mcp(ductus/*)` line).
- **Context.** `AGENTS.md` (already shipped) is read natively — nothing new.

## Bootstrap

Because Antigravity skills are markdown and discovered from `.agents/`, the
file-scaffold bootstrap works unchanged — only the destination differs.
`install.sh` with the `antigravity` (or `agy`) argument places the `ductus` skill
at `.agents/skills/ductus/SKILL.md`, after which the user runs `/ductus`, which
scaffolds the rest. The ductus self-install rule (keep every placeholder literal)
carries over. No plugin build, no `agy plugin install`, no chicken-and-egg.

## Update Story

`/ductus` re-runs re-scaffold the `.agents/` files (from the resolved source), exactly like
the other file-scaffold agents — no install/registration step. Pinning via
`.ductus/config.toml` and the manifest `update`/`create`/`skip` strategies apply
unchanged.

## Out of Scope

- **Gemini CLI** — dropped given its 2026-06-18 shutdown.
- **Global plugin + marketplace** (`agy plugin install ductus@…`) — an optional,
  separate distribution channel, deferred to future work.
- **Skill-vs-slash-command form refinement** — probing showed dir-form
  `skills/<name>/SKILL.md` loads as an agent-context skill while a flat
  `.agents/skills/<name>.md` did not load in `agy -p` (print mode reported "no
  custom slash commands"), hinting at a skill (agent-context) vs slash-command
  (TUI-only) distinction. ductus targets the verified dir-form skill.
- Agents beyond Antigravity (Cursor, Copilot, …).
- Removing an adopted agent (unchanged from 012 — manual).

## Acceptance Criteria

- [x] AC1: The Agent Registry expresses per-agent **command/skill location**,
      **invocation**, **MCP-wiring file**, **settings file + permission format**,
      **rules location**, and **native rules file** as derived values;
      Claude/Auggie keep their `.claude`/`.augment` layout, Antigravity is
      `config_dir = .agents` with the skill/`mcp_config.json`/`settings.json`/
      `rules` layout, and the unified procedure branches on registry values, not
      the agent name. **The MCP leg is superseded**: `031-agent-mcp-wiring` took
      the MCP-wiring file out of the layout-derived values and made it a
      per-agent descriptor. The other legs hold.
- [x] AC2: A `.claude`-style file-scaffold agent that reads `CLAUDE.md` can still be
      added by a single registry-row append + `configure/{key}.md` + README
      snippet (verified by a documented "add a hypothetical agent" checklist).
      **The checklist has since grown**: the current one, **Adding a new agent**
      in `framework/bootstrap/ductus.md`, also asks for an MCP registration row
      (since 031) and an `install.sh` arm, and the README edit is an install-table
      row. It is still no layout change.
- [x] AC3: Scaffolding Antigravity writes each pipeline command as
      `.agents/skills/{project}-<name>/SKILL.md` (dir-form skill, body = the
      command procedure with gates intact, `name` + `description` frontmatter),
      invocable as `/{project}-<name>`
- [x] AC4: ductus's domain rule files scaffold to `.agents/rules/<name>.md`
- [x] AC5: ductus is wired via `.agents/mcp_config.json` (local stdio server, additive)
      **and** `.agents/settings.json` allows `mcp(ductus/*)`. **Half superseded.**
      The permission half still holds exactly — `framework/bootstrap/configure/antigravity.md`
      writes `mcp(ductus/*)` into `.agents/settings.json`, emitted by
      `scripts/gen-configure-mcp.sh`. The wiring half was **reversed** by
      `031-agent-mcp-wiring`: Antigravity loads MCP servers only from home-level
      `~/.gemini/config/mcp_config.json`, and project-local `.agents/mcp_config.json`
      is ignored — measured against the live `agy` CLI at 0 server spawns and 0 MCP
      log references, against a positive control that did spawn. ductus therefore
      writes **no** MCP file for Antigravity at all; its registration `mechanism` is
      `surface-instruction`, so the user performs a one-time config edit plus a `/mcp`
      reload. The criterion is left as delivered rather than rewritten, because the
      wiring it describes is what this spec shipped in June
- [x] AC6: A `framework/bootstrap/configure/{key}.md` for Antigravity writes
      `.agents/settings.json` `permissions` in Antigravity's action grammar;
      `gen-configure-mcp.sh` emits the Antigravity MCP block
- [x] AC7: Adopting Antigravity ships **no `CLAUDE.md`** and no new context file;
      `AGENTS.md` (already shipped) carries the context
- [x] AC8: Each agent receives only its own permission format — Antigravity never
      receives Claude's `permissions`/`Bash(...)` or Auggie's `toolPermissions`,
      and vice versa (the 012 format-leakage failure mode does not recur)
- [x] AC9: Auto-detection recognizes an existing `.agents/` adoption and re-scaffolds
      on routine re-runs, consistent with the 012 detect path
- [x] AC10: The README documents the Antigravity bootstrap (curl the `ductus` skill
      into `.agents/skills/ductus/SKILL.md`, then run `/ductus`). **The install
      step changed**: since `003-bootstrap-automation`, `install.sh` with the
      `antigravity` argument places the skill at that path, and the README's
      per-agent install table documents it; running `/ductus` afterwards holds
- [x] AC11: All shipped markdown passes `npx markdownlint-cli2`; the emitted
      `.agents/mcp_config.json` and `.agents/settings.json` are valid JSON.
      The markdown half holds. Of the JSON half only `.agents/settings.json`
      remains a file ductus emits — `.agents/mcp_config.json` is no longer
      written at all, per AC5's supersession above

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Unit + mapping (was "command→skill namespacing").** ductus targets
  Antigravity's documented surface — **skills** (`.agents/skills/<name>/SKILL.md`)
  for pipeline steps, **`rules/`** for ductus's rule files, **`mcp_config.json`**
  for ductus — not a `commands/` directory. The `commands/*.md → skills` conversion
  observed via `agy plugin validate`/`install` is an undocumented
  import-compatibility shim (for `agy plugin import claude|gemini`); ductus does
  not rely on it. Skill names are **flat** — no `/{project}:<name>` colon
  namespace — so ductus prefixes names (`/{project}-<name>`). ductus's rule files
  map 1:1 onto `rules/`. The `mcp_config.json` leg was reversed by
  `031-agent-mcp-wiring`: ductus writes no MCP file for Antigravity.
- **Skill invocation / UX (and the `.claude/` refutation).** Skills are
  user-invoked: `.agents/skills/<name>/SKILL.md` → `/<name>` (maintainer-confirmed
  for `.agents/skills/format-tests.md` → `/format-tests`). ductus's gated,
  user-driven pipeline therefore **ports directly** — the user types the slash
  command and the approval gates remain in-body prompts. A live `agy` session
  loaded a workspace skill from `<repo>/.agents/skills/probe-ws/SKILL.md`
  (reported with its exact path), while identical probes under `.claude/commands/`
  and `.claude/skills/` did **not** load — empirically refuting the agent's
  self-report that Antigravity uses `.claude/`. The **dir form**
  `<name>/SKILL.md` is what loaded; a flat `.agents/skills/<name>.md` did not load
  in print mode. ductus ships the dir form; the `$ARGUMENTS` equivalent was never
  pinned.
- **Install model + update flow (was "install: copy vs reference") — scoped to
  the optional plugin path.** `agy plugin install <dir>` snapshot-copies into
  `~/.gemini/config/plugins/<name>/` and is idempotent (re-install overwrites;
  verified VERSION_ONE → VERSION_TWO). This applies only to the deferred global
  plugin distribution; **ductus's adoption is file-scaffold into `.agents/`**, so
  its update flow is re-scaffolding files, not re-installing.
- **Rules file + context (was "CLAUDE.md readership").** Confirmed: Antigravity
  reads `AGENTS.md` natively. ductus ships **no `CLAUDE.md`** and no new context
  file; `AGENTS.md` (already shipped) carries the context; rule files map to the
  `.agents/rules/` dir. The constitution lands at `.ductus/constitution.md`, as
  it does for every agent.
- **Adoption model + detection (was "detection key").** Antigravity has a
  per-workspace scope — `.agents/skills/`, `.agents/settings.json`
  (workspace-skill loading empirically verified). ductus adopts by
  **file-scaffolding into `.agents/`** per project, exactly as for
  Claude/Auggie — not by building/installing a global plugin. Registry row:
  `config_dir = .agents`. **Detection** keys on `.agents/` existing in the
  project. **Update** is re-scaffolding (from the resolved source). The
  workspace scope was also believed to cover `.agents/mcp_config.json`, until
  `031-agent-mcp-wiring` measured it ignored.
- **Permissions / `configure` (corrected from "N/A").** Antigravity **does** have
  a permissions file — `.agents/settings.json` (workspace; global form
  `~/.gemini/antigravity-cli/settings.json`), **not** `.claude/settings.local.json`
  (refuted with the `.claude/` test). Schema: `{ "permissions": { "allow": [],
  "deny": [], "ask": [] } }`, structurally close to Claude's `permissions.allow/
  deny` plus an `ask` tier, with an action grammar (`command(...)`,
  `read_file(...)`, `mcp(server/tool)`, `unsandboxed(...)`, `*`). ductus therefore
  **gains a `configure` step** for Antigravity (a third permission format): a new
  `framework/bootstrap/configure/{key}.md` writes the allow/deny/ask set, with
  ductus covered by a single `mcp(ductus/*)` and shell allows/denies as
  `command(...)`; workspace files are auto-allowed so `read_file`/`write_file`
  are largely unneeded.
- **MCP wiring + merge ownership.** **Reversed by `031-agent-mcp-wiring` — this
  resolution no longer describes shipped behavior.** ductus spans only **one**
  `.agents/` file: `settings.json`, carrying the `mcp(ductus/*)` permission. The
  server definition is not ductus's to write — Antigravity reads MCP servers from
  home-level `~/.gemini/config/mcp_config.json`, which lives outside the repo, so
  ductus surfaces a one-time instruction instead of mutating a per-machine file.
  The original resolution, kept for the record of what was decided and why:
  ductus spans two `.agents/` files —
  `mcp_config.json` (server definition) and `settings.json` (`mcp(ductus/*)`
  permission) — mirroring Claude's `.mcp.json` + settings split. Both installs are
  additive (preserve adopter entries). `mcp_config.json` is a JSON-object merge;
  `settings.json` `permissions` is close enough to Claude's shape that
  `merge-permissions`
  ([022-deterministic-runtime](../022-deterministic-runtime/spec.md)) is a
  candidate to extend to a third format. Both are host-owned for now; whether a
  runtime primitive grows to own them is a plan-phase call (parallel to the
  existing Auggie `merge-permissions` gap).
- **Marketplace / global-plugin distribution — deferred.** Given the workspace
  `.agents/` adoption model, publishing a global plugin
  (`agy plugin install ductus@marketplace`) is an optional, separate distribution
  channel, out of scope for this spec. Revisit as future work if there is demand
  for a one-command global install.
