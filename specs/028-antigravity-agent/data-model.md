# 028 — Antigravity Agent Support Data Model

Structures introduced or modified by this feature: the generalized **Agent
Registry** schema and the three Antigravity **scaffolded-artifact** schemas. All
are markdown/JSON artifacts (no database).

## Agent Registry (generalized)

The registry in `framework/bootstrap/ductus.md` §Agent Registry gains a `layout`
column. Existing fields are unchanged; `layout` selects the derived-value set.

| Field | Type | Notes |
| --- | --- | --- |
| `key` | string | registry key; `configure/{key}.md` source path |
| `name` | string | display name |
| `config_dir` | string | per-agent config root (`.claude`, `.augment`, `.agents`) |
| `layout` | enum `claude-style` \| `antigravity` | selects command/skill location, MCP-wiring file, settings format, rules location, native rules file |
| `settings_template` | JSON | bootstrap-only permission seed, in the layout's native shape |
| `rules_file_note` | string | which file the agent reads natively |

### Profile-derived values

| Derived value | `claude-style` | `antigravity` |
| --- | --- | --- |
| Command/skill path | `{config_dir}/commands/{project}/<name>.md` | `.agents/skills/{project}-<name>/SKILL.md` |
| Invocation | `/{project}:<name>` | `/{project}-<name>` |
| `ductus` install path | `{config_dir}/commands/ductus.md` | `.agents/skills/ductus/SKILL.md` |
| MCP-wiring file | `.mcp.json` | `.agents/mcp_config.json` |
| Settings file | `{config_dir}/settings.local.json` | `.agents/settings.json` |
| Permission shape | `permissions.allow/deny` (Claude) / `toolPermissions[]` (Auggie) | `permissions.allow/deny/ask` (action grammar) |
| Rules location | filesystem `specs/rules/` | `.agents/rules/<name>.md` |
| Native rules file | `CLAUDE.md` | `AGENTS.md` |
| Cleanup glob | `*.md` | `{project}-*/` skill dirs |

Rows: `claude` (`.claude`, `claude-style`), `auggie` (`.augment`,
`claude-style`), `antigravity` (`.agents`, `antigravity`).

## `.agents/skills/{project}-<name>/SKILL.md`

Dir-form skill (one directory per skill). Frontmatter + procedure body.

```markdown
---
name: {project}-<name>
description: <one-line, carried from the source command's frontmatter>
---

<the command procedure body, with {project} / {cli-config-dir} substituted;
 approval-gate prompts preserved verbatim>
```

- `name` — flat, project-prefixed; drives the `/{project}-<name>` invocation.
- `description` — lifted from the source `framework/commands/<name>.md`
  frontmatter.
- `ductus` installer skill keeps `{project}` / `{cli-config-dir}` literal.

## `.agents/mcp_config.json`

ductus server definition (local stdio). Additive: ductus adds the `ductus` key if
absent, preserving any adopter servers.

```json
{
  "mcpServers": {
    "ductus": {
      "command": "ductus",
      "args": ["mcp"]
    }
  }
}
```

## `.agents/settings.json`

Permissions in Antigravity's action grammar. Three arrays; entries are
`action(target)` strings. Additive merge — ductus installs the canonical set and
dedups, preserving adopter entries (mirrors the Claude/Auggie configure posture).

```json
{
  "permissions": {
    "allow": [
      "mcp(ductus/*)",
      "command(git add)",
      "command(git commit)",
      "command(curl)",
      "command(npx markdownlint-cli2)",
      "command(scripts/gen-)"
    ],
    "deny": [
      "command(rm -rf)",
      "command(git push --force)"
    ],
    "ask": []
  }
}
```

- `mcp(ductus/*)` — one entry covers every ductus tool (vs Claude's per-tool list);
  emitted by `gen-configure-mcp.sh`.
- `command(<prefix>)` — token-prefix match (anchored per-token regex).
- `read_file`/`write_file` — generally omitted; workspace files auto-allowed.
- Global form lives at `~/.gemini/antigravity-cli/settings.json`; ductus targets
  the workspace `.agents/settings.json`.

## Notes

- Detection (`/ductus` §Agent Selection) is unchanged — it keys on `config_dir`
  existing in the project (`.agents/` for Antigravity).
- The global plugin schema (`plugin.json` + `skills/` + `rules/` +
  `mcp_config.json` under `~/.gemini/config/plugins/`) is **out of scope** — the
  deferred marketplace path, not ductus's adoption surface.
