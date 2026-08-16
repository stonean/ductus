# 028 — Antigravity Agent Support Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Generalize the registry with `layout` profiles

- [x] Add a `layout` column to `framework/bootstrap/ductus.md` §Agent Registry; set existing rows `claude` / `auggie` to `claude-style`
- [x] Add the `antigravity` row (`config_dir: .agents`, `layout: antigravity`, `settings_template`, `rules_file_note`) per [data-model.md](data-model.md)
- [x] Rewrite §Derived values as a profile table (command/skill path, invocation, `ductus` install path, MCP file, settings file, permission shape, rules location, native rules file, cleanup glob)
- [x] Update §"Adding a new agent" to note that a `claude-style` agent stays a one-row append
- Done when: the registry + derived-values sections describe all three agents by profile, and a hypothetical `.claude`-style agent is still a pure row append

## 2. Branch ductus.md for the `antigravity` layout

Scaffolding (§Per-Agent Scaffolding):

- [x] For `layout: antigravity`, transform each `framework/commands/<name>.md` → `.agents/skills/{project}-<name>/SKILL.md` (set `name`, carry `description`, substitute `{project}`/`{cli-config-dir}`, preserve gate prompts)
- [x] Scaffold the `ductus` installer to `.agents/skills/ductus/SKILL.md` (placeholders kept literal)
- [x] Scaffold domain rule files to `.agents/rules/<name>.md` (mirror of `specs/rules/`)
- [x] Branch slash-command cleanup to prune stale `.agents/skills/{project}-*/` dirs

Bootstrap flow (added after mid-implement discovery — see plan §Technical Decision 8):

- [x] Branch the **ductus.md Self-Update Check**: per-layout install path; compare the installed `SKILL.md` body (frontmatter stripped) against upstream, and on the stale-write path write the transformed skill — not raw `ductus.md`
- [x] Branch the **Post-Write Integrity Check**: for `antigravity`, verify the `SKILL.md` (frontmatter `name: ductus` + body) rather than the `# ductus` first line
- [x] Branch the placeholder-substitution "keep literal" exception to the `antigravity` ductus-skill path
- [x] Update the **CLAUDE.md** shared-file step: ship `CLAUDE.md` for `claude-style` only; note Antigravity reads `AGENTS.md`
- [x] Make `parity.strict-files` (frontmatter) layout-aware — evaluated: left as-is (unenforced metadata; only a future-use comment in `manifest-parity.sh`; antigravity skill paths not added as they would reference files absent in claude-only repos)
- [x] Add the `antigravity` skills dir to intermediate-dir creation
- [x] Guard **Workflow recommendation** to skip for `antigravity` (deferred)
- Done when: every layout-assuming section of `ductus.md` branches on `layout`, the `claude-style` flow is unchanged byte-for-byte, and an Antigravity bootstrap (install → self-update check → scaffold) is internally consistent

## 3. Create `framework/bootstrap/configure/antigravity.md`

- [x] Author the configure command writing `.agents/settings.json` `permissions.allow/deny/ask` in the action grammar (shell `command(...)` allows/denies; file ops omitted)
- [x] Include the `<!-- generated:mcp-allow:start/end -->` markers for the generator
- Done when: `configure/antigravity.md` exists with the canonical non-MCP permission set and the marker block

## 4. Emit the Antigravity MCP block from `gen-configure-mcp.sh`

- [x] Add `antigravity.md` as a third splice target; emit a single `mcp(ductus/*)` allow entry between the markers
- [x] Run the generator; verify `claude.md` / `auggie.md` output is unchanged and `antigravity.md` is populated
- Done when: `scripts/gen-configure-mcp.sh` updates all three sources and the pre-commit invariant (drift fails) covers Antigravity

## 5. Branch Permission Setup + MCP registration in ductus.md

- [x] Branch §Permission Setup to seed `.agents/settings.json` from the `antigravity` `settings_template`
- [x] Branch the MCP-registration step — N/A: ductus.md has no MCP-registration step (`.mcp.json` is not scaffolded for any agent; ductus is an optional out-of-band install). The `.agents/mcp_config.json` wiring is documented in §Permission Setup as a README/runtime concern and ships in the README (Task 7), parallel to `.mcp.json`
- [x] Document the additive merge for both `.agents/` files (host/markdown path) — settings.json in §Permission Setup + configure/antigravity.md; mcp_config.json additive note in §Permission Setup
- Done when: ductus.md describes the two-file ductus wiring and the settings seed for Antigravity, additively

## 6. Add `.agents/` to the managed `.gitignore` block

- [x] Add `.agents/` to `framework/templates/project/gitignore` (`.agents/*` + `!.agents/skills/`, mirroring the Claude `commands` carve-out)
- [x] Add `.agents/` to the `merge-managed-block` content described in ductus.md §Shared Files (the canonical block is the template above; updated the illustrative pattern lists in ductus.md too)
- Done when: a fresh adoption gitignores `.agents/` alongside `.claude/`

## 7. Document the Antigravity bootstrap in README

- [x] Add an Antigravity curl snippet (install the `ductus` skill into `.agents/skills/ductus/SKILL.md`) — a transform snippet (strip ductus.md frontmatter + prepend `name: ductus`), since the `ductus` skill is not ductus.md verbatim
- [x] Add Antigravity to the supported-agents list / paths summary
- Done when: README documents adopting Antigravity with no second curl needed for additional agents

## 8. Cross-spec signpost on 012

- [x] Add a signpost note to `specs/012-multi-agent-govern/spec.md` pointing to 028 (registry generalized to layout profiles), mirroring the `007 → 012` pattern (in a blockquote, so `gen-spec-deps` skips the link — no 012↔028 cycle)
- Done when: 012 carries the signpost; `gen-spec-deps` derives no cycle

## 9. Tests

- [x] Extend/create `scripts/tests/test-gen-configure-mcp.sh` to assert the `antigravity.md` `mcp(ductus/*)` block is emitted and in sync
- [x] Run the full generator + audit gate; confirm green
- Done when: the Antigravity generator invariant is covered and `scripts/audit/run-all.sh` passes

## 10. Validation

- [x] `npx markdownlint-cli2` clean across the feature dir and changed framework files
- [x] Walk the acceptance criteria; confirm each is satisfied
- Done when: all spec acceptance criteria are met and lint is clean

<!-- Out of scope (tracked, not a task here): extending ductus `exec`'s `Host`
     command resolution to the `.agents/skills/<name>/SKILL.md` layout — a 022
     runtime change. Antigravity ships on the markdown-only path; ductus MCP tools
     are unaffected. See plan.md Technical Decision 6. -->
