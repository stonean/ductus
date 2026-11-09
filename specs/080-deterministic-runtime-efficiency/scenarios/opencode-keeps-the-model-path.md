---
section: "`/{project}:status` without a model turn"
---

# Opencode-keeps-the-model-path

## Context

OpenCode runs `/{project}:status` on the model path: `/ductus` installs `framework/commands/status.md` as `.opencode/command/{project}/status.md`, which OpenCode invokes as `/{project}/status`, and the model emits `render-status`'s view. The spec left OpenCode on that path until a run on a version with a model-free mechanism settled one. The run on OpenCode 1.17.8 that `research.md` records under *OpenCode's TUI-plugin slash command* is that run.

## Behavior

- OpenCode keeps the model path for `/{project}/status`: no plugin, hook or bootstrap change ships for it, and it runs the command as it did before this spec (AC10).
- The mechanism exists on 1.17.8: a TUI plugin listed in `.opencode/tui.json`, a module of `@opencode-ai/plugin`'s `{ id, tui }` shape, registers a slash command through `api.command.register` whose `onSelect` runs local code, such as `.ductus/bin/ductus render-status`, and shows the result with `api.ui.toast`, with no session and no model turn.
- It is not used, because `api.command` is marked deprecated in `@opencode-ai/plugin` 1.17.8's types, to be removed in OpenCode v2 in favor of `api.keymap.registerLayer`, and the TUI-plugin API opencode.ai documents now (`@opencode/plugin/tui`, `Plugin.define`, `cli.json`) is absent from the 1.17.8 binary, so a plugin built on 1.17.8 would be rebuilt for v2 (owner, 2026-11-09).

## Edge Cases

- An installed command keeps its name over a plugin's alias: with `scratch/status` among a plugin command's aliases, submitting `/scratch/status` ran the template on the model path. So a model-free `/{project}/status` would also need `/ductus` to stop installing `status.md` for OpenCode, or to give the plugin's command a distinct name.
- The view's hints spell `/{project}:<name>`, while OpenCode names a command `/{project}/<name>`, so on OpenCode a hint names no command. On Pi the bridge respells each hint as Pi names the command.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Does OpenCode get a model-free `/{project}:status`?** Not now: the mechanism is verified on 1.17.8, and OpenCode keeps the model path (owner, 2026-11-09). This settles the spec's deferral of OpenCode to a scenario under it.
  - **Deferred:** shipping a TUI plugin for it, triggered by an installed OpenCode that serves the v2 TUI-plugin API (`@opencode/plugin/tui`).
  - **Set aside:** a plugin on 1.17.8's `api.command`, which its types mark for removal in v2.
