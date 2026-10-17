# 028 — Antigravity Agent Support Plan

Implements [028 — Antigravity Agent Support](spec.md).

## Overview

Add Antigravity as a third file-scaffold agent by **generalizing the agent
registry into layout profiles** rather than widening every row. Claude and
Auggie share a `claude-style` profile (commands under
`{config_dir}/commands/{project}/`, `CLAUDE.md`, `settings.local.json`);
Antigravity gets an `antigravity` profile (dir-form skills under
`.agents/skills/`, `AGENTS.md`, `.agents/settings.json`). MCP registration is not
part of a profile: since `031-agent-mcp-wiring` it is a per-agent descriptor
(Technical Decision 4). `framework/bootstrap/ductus.md`'s derived values and
`framework/bootstrap/ductus.md` §Per-Agent Scaffolding branch on the profile; everything else (the unified
procedure, manifests, session state) is unchanged. The work is almost entirely
in the markdown bootstrap + one generator + one new configure source; no
mandatory runtime (Rust) change ships in this spec (see Technical Decision 6).

## Technical Decisions

### 1. Layout profiles, not per-field registry columns

The registry gains a per-agent **`layout`** discriminator, with two values from
this spec — `claude-style` (Claude, Auggie) and `antigravity` — and two added
later, `opencode` and `pi`. The derived-value formulas in
ductus.md §Derived values and the branch points in §Per-Agent Scaffolding select
behavior by `layout`. Claude and Auggie are unchanged (`layout: claude-style`;
they already differ only by `config_dir`). Antigravity is one new row.

Rejected: adding five columns (command location, MCP file, settings format,
rules location, rules file) to every row — it bloats the two existing rows with
identical values and scatters the divergence. A profile keeps a new
`.claude`-style agent a true one-row append (`layout: claude-style` + a
`config_dir`) — preserving 012's promise for the common case — while isolating
Antigravity's divergence in one named profile.

Antigravity registry row:

| field | value |
| --- | --- |
| `key` | `antigravity` |
| `name` | `Antigravity` |
| `config_dir` | `.agents` |
| `layout` | `antigravity` |
| `settings_template` | `{ "permissions": { "allow": [ "command(curl)", … ], "deny": [], "ask": [] } }` — the bootstrap-only seed; the row in `framework/bootstrap/ductus.md` §Agent Registry carries the full set |
| `rules_file_note` | `Antigravity reads AGENTS.md natively — no second rules file.` |

### 2. Skill transform (per-agent scaffolding for `antigravity`)

For `layout: antigravity`, `framework/bootstrap/ductus.md` §Per-Agent Scaffolding does not copy command files
verbatim. Each `framework/commands/<name>.md` becomes
`.agents/skills/{project}-<name>/SKILL.md`:

- Frontmatter: set `name: {project}-<name>`, carry the source `description`.
- Body: the command procedure, with `{project}` / `{cli-config-dir}` substituted
  (`{cli-config-dir}` resolves to `.agents`).
- The `ductus` installer → `.agents/skills/ductus/SKILL.md` (placeholders kept
  literal, per the existing self-install rule).
- Domain rule files → `.agents/rules/<name>.md`.
- Slash-command cleanup prunes stale `.agents/skills/{project}-*/` skill dirs not
  in the manifest (the `claude-style` cleanup prunes `*.md`; the enforced glob is
  profile-derived).

Skill names are flat and project-prefixed (`/{project}-<name>`), since
Antigravity has no colon namespace.

### 3. Configure — third permission format

New `framework/bootstrap/configure/antigravity.md` writes `.agents/settings.json`
`permissions.allow/deny/ask` in Antigravity's action grammar. Canonical-set
mapping:

- ductus runtime → a single `mcp(ductus/*)`, where Claude's configure lists one
  `mcp__ductus__*` line per tool.
- shell allows → `command(git add)`, `command(curl)`,
  `command(npx markdownlint-cli2)`, …
- denies → `command(rm -rf)`, `command(git push --force)`, …
- `read_file`/`write_file` largely omitted — workspace files are auto-allowed.

`gen-configure-mcp.sh` gains a **third splice target** emitting the Antigravity
MCP block (one `mcp(ductus/*)` line) into `antigravity.md`, preserving the
pre-commit invariant that runtime-tool permissions stay in sync across all
agents.

### 4. MCP wiring — one file, plus a surfaced instruction

ductus for Antigravity = the `mcp(ductus/*)` allow in `.agents/settings.json`
(additive — any permissions the adopter already has are preserved) **plus** a
one-time instruction to register the server in home-level
`~/.gemini/config/mcp_config.json`, then reload with `/mcp`. MCP registration
branches per agent, not per layout: ductus.md §MCP registration (per-agent) gives
each agent a `target`, `scope` and `mechanism`, and Antigravity's is
`surface-instruction`, because it ignores a project-local `.agents/mcp_config.json`
(`031-agent-mcp-wiring` measured 0 server spawns there).

### 5. gitignore

Add `.agents/` to the framework-managed `.gitignore` block (the
`merge-managed-block` content in ductus.md §Shared Files and
`framework/templates/project/gitignore`), parallel to the existing `.claude/`
line — Antigravity's scaffolded tree is adopter-local, gitignored like the
others.

### 6. Runtime `exec` command resolution — scoped out (markdown-only path ships)

ductus `exec` resolves a command file at `{cli-config-dir}/commands/{project}/
<name>.md` via [022](../022-deterministic-runtime/spec.md)'s `Host`. For
Antigravity the procedure lives at `.agents/skills/{project}-<name>/SKILL.md`, so
`ductus exec` would not find it without extending `Host` to the skill layout — a
Rust change.

This spec **scopes that out**: (a) Antigravity adoption works on the
markdown-only path immediately — the agent reads `SKILL.md` directly; (b) ductus's
MCP tools (`read-spec`, `mark-task`, `lint-markdown`, …) operate on `specs/`
independent of agent layout, so the runtime's value-add is available regardless.
`Host` still does not resolve the skill path: 022's `opencode-command-resolution`
scenario, which widened the resolver to the `opencode` and `pi` paths, names the
Antigravity layout as out of scope. Rationale: don't gate Antigravity adoption on
a runtime change; ship file-scaffold + MCP tools now.

### 7. Merge ownership — host/markdown now, primitive later

The `.agents/settings.json` `permissions` allow/deny/ask merge is performed
host-side per ductus.md prose. `merge-permissions` serves only the Claude shape;
022's `coverage-residue-cleanup` scenario notes that the Auggie, Antigravity and
OpenCode formats each walk the prose instead, and leaves extending the primitive
as not urgent. ductus.md documents the additive merge so the markdown-only path
is faithful.

### 8. The `antigravity` layout touches the whole bootstrap, not just scaffolding

Discovered mid-implement (Task 2): the layout branch is not confined to
`framework/bootstrap/ductus.md` §Per-Agent Scaffolding. Because the Antigravity `ductus` installer is a
*transformed* skill (`.agents/skills/ductus/SKILL.md`, frontmatter `name:
ductus` plus the body) rather than a verbatim copy of `ductus.md`, several other
bootstrap
sections that assume the `claude-style` `commands/ductus.md` install must also
branch on `layout`:

- **ductus.md Self-Update Check** — byte-compares the installed file against
  upstream; for `antigravity` it must strip the added frontmatter and compare the
  body, and the stale-write path must write the transformed skill, not raw
  `ductus.md` (otherwise every run reports stale and installs a broken skill).
- **Post-Write Integrity Check** — for `claude-style`, asserts a frontmatter
  block carrying `description:` followed by a body starting `# ductus`; for
  `antigravity` it checks the `SKILL.md` frontmatter (`name: ductus`) + body.
- **Placeholder-substitution exception**, **intermediate-dir creation**,
  **`parity.strict-files` frontmatter**, and the **CLAUDE.md shared-file step**
  (`claude-style` only) likewise branch.

All are folded into Task 2; the Self-Update and Integrity branches are required
for Antigravity to bootstrap at all.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/bootstrap/ductus.md` | Modify | `layout` profiles + Antigravity registry row; §Derived values + §Per-Agent Scaffolding branch (skill transform, rules, `mcp_config.json`); §Permission Setup branch (`.agents/settings.json`); MCP-registration branch; `.gitignore` block adds `.agents/`; §Agent Selection/detection unchanged (keys on `config_dir`) |
| `framework/bootstrap/configure/antigravity.md` | Create | Antigravity permission set → `.agents/settings.json` (`allow/deny/ask`, action grammar, `mcp(ductus/*)`) |
| `scripts/gen-configure-mcp.sh` | Modify | Third splice target: emit `mcp(ductus/*)` Antigravity block; keep the cross-agent in-sync invariant |
| `framework/templates/project/gitignore` | Modify | Add `.agents/` to the managed block |
| `README.md` | Modify | Antigravity curl-bootstrap snippet + add Antigravity to the supported-agents list |
| `specs/012-multi-agent-govern/spec.md` | Modify | Signpost note pointing to 028 (cross-spec impact; mirrors `007 → 012`) |
| `scripts/tests/test-gen-configure-mcp.sh` (or extend) | Create/Modify | Assert the Antigravity `mcp(ductus/*)` block is emitted and stays in sync |
| `specs/028-antigravity-agent/data-model.md` | Create | Registry schema + Antigravity artifact schemas (this plan) |

## Trade-offs

- **Layout profiles vs per-field columns** — profiles chosen (keeps the common
  case a one-row append; isolates divergence). Cost: each new profile means a new
  branch set rather than new column values. `032-opencode-agent` and
  `064-pi-host-support` have each paid it since, adding `opencode` and `pi`.
- **Runtime `exec` deferred** — Antigravity ships on the markdown-only path now;
  adopters with ductus installed do not get the deterministic command-walk speedup
  for Antigravity until 022's `Host` is extended. ductus MCP tools are unaffected.
  Limitation documented; tracked as a 022 follow-up.
- **Host-side merge vs runtime primitive** — host/prose, the same posture as
  Auggie's `toolPermissions` (Technical Decision 7).
- **Project-prefixed flat skill names** (`/{project}-specify`) — avoids
  collisions and matches Antigravity's flat namespace; loses the `:` visual
  grouping of `/{project}:specify`.
- **Global plugin + marketplace deferred** — adoption is workspace `.agents/`
  only; a one-command global install is future work.

## Data Model

See [data-model.md](data-model.md) — the generalized registry schema (with the
`layout` field and profile-derived values) and the Antigravity artifact schemas
(`.agents/skills/<name>/SKILL.md`, `.agents/settings.json`).
