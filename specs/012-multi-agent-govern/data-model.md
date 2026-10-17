---
title: "012-multi-agent-govern — data-model"
---

# 012 — Multi-Agent Ductus Data Model

The agent registry is a structured, in-file data model carried by `framework/bootstrap/ductus.md`: a table of supported agents that the unified bootstrap iterates over during scaffolding. There is no database and no language-level type — the "data" is markdown rows the prompt reads at run time. This file states the registry's contract; the **Agent Registry** and **Derived values** sections of `framework/bootstrap/ductus.md` hold the rows and the per-layout formulas.

## Agent Registry

Each row describes one supported agent. 012 shipped five fields per row; `028-antigravity-agent` added the sixth, `layout`, which is what lets an agent whose tree differs structurally join the registry.

| Field | Type | Required | Description |
| --- | --- | --- | --- |
| `key` | string | yes | Lowercase identifier. Stable across releases. Used in `--agents={key}` flags, the `framework/bootstrap/configure/{key}.md` permission source path, and in user-facing prompt text. Must be unique within the registry. |
| `name` | string | yes | Human-readable label shown in `AskUserQuestion` prompts and the post-scaffolding summary. Example: "Claude Code", "Auggie". |
| `config_dir` | string | yes | Project-relative directory where the agent stores its config and commands. Example: `.claude`, `.augment`. Used to detect existing adoptions and to compute every per-agent destination path. |
| `layout` | string | yes | The agent's directory and command shape. Selects which column of the layout-derived values applies to the agent; the layouts are the columns of **Derived values** in `framework/bootstrap/ductus.md`. |
| `settings_template` | object (JSON) | yes | Bootstrap entries merged into the agent's settings file if missing, in the agent's native settings format. Contains only the entries `/ductus` itself needs — not the full permission set — and is empty for an agent whose settings carry no permission gate. |
| `rules_file_note` | string | yes | Short note about the agent's relationship to its native rules file (`CLAUDE.md` or `AGENTS.md`). Surfaced in the post-scaffolding summary. Example: "Claude Code reads `CLAUDE.md` natively." |

### Derived values

These are computed from the registry by convention; they are not stored as fields.

- **Permission source** — `framework/bootstrap/configure/{key}.md`, the same formula for every layout.
- **Layout-derived values** — the per-layout paths and behaviors, such as the command path, the bootstrap install path and the settings file, each selected by `layout`. For a `claude-style` agent the project commands live in `{config_dir}/commands/{project}/` and the bootstrap is installed at `{config_dir}/commands/ductus.md`.

Session state is not derived per agent: every adopter keeps it at `.ductus/session.toml`.

### Initial population

012 shipped two rows: `claude` (Claude Code, config directory `.claude`) and `auggie` (Auggie, `.augment`), both reading `CLAUDE.md` natively. The registry in `framework/bootstrap/ductus.md` holds the current rows.

## Adding a new agent

Within an existing layout, a new agent is a registry row plus the files and entries its values point at; the rest of the bootstrap references registry values, not agent names. **Adding a new agent** in `framework/bootstrap/ductus.md` is the canonical list. An agent introducing a new `layout` costs more: it branches the scaffolding, permission-setup and bootstrap sections, so the one-row-append contract holds only within an existing layout.

## Invariants

- `key` values are unique across the registry.
- `key` is a valid filename component — lowercase letters, digits, hyphens only — because it is interpolated into a path (`framework/bootstrap/configure/{key}.md`).
- `config_dir` is a project-relative path, no trailing slash, no leading `./`.
- `layout` names a layout that **Derived values** defines.
- `settings_template` is valid JSON and matches the agent's native settings format. Merging the template into an existing settings file must preserve all entries the user or `/{project}:configure` has previously written.
- The set of rows is intrinsic to `framework/bootstrap/ductus.md` — no runtime registration, no external file. Adding an agent is a code change to the bootstrap, reviewed in PR.
