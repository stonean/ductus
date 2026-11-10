---
section: "`/{project}:status` without a model turn"
---

# Antigravity-keeps-the-model-path

## Context

Antigravity runs `/{project}:status` on the model path: `/ductus` installs `framework/commands/status.md` as the skill `.agents/skills/{project}-status/SKILL.md`, and the model emits `render-status`'s view. The spec left Antigravity on that path until a run on a version with a model-free mechanism settled one: from `agy` 1.4.11, a `PreInvocation` hook returning `errorMessage` with `terminationBehavior: "terminate"`.

## Behavior

- Antigravity keeps the model path for `/{project}:status`: no hook, skill or bootstrap change ships for it, and it runs the command as it did before this spec (AC10).
- No run was made: the installed `agy` is 1.0.9, which lacks the `PreInvocation` hook, and the owner chose to record this scenario without one (owner, 2026-11-09). So whether that hook sees the raw command name, rather than the expanded skill, is unverified.

## Edge Cases

- The view's hints spell `/{project}:<name>`, while the bootstrap's §Derived values gives Antigravity's invocation as `/{project}-<name>`, so on Antigravity a hint names no command, as on OpenCode. On Pi the bridge respells each hint as Pi names the command. Waived for this spec, as on OpenCode (owner, 2026-11-10; plan *Auggie, OpenCode and Antigravity*).

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Does Antigravity get a model-free `/{project}:status`?** Not now: recorded without a run, and Antigravity keeps the model path (owner, 2026-11-09). This settles the spec's deferral of Antigravity to a scenario under it.
  - **Deferred:** a `PreInvocation` hook for it, triggered by a run on `agy` 1.4.11 or later that shows the hook sees the raw command name.
