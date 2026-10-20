---
section: "Edge Cases"
---

# An-unknown-artifacts-key-is-named

## Context

[019](../../019-config-decisions/scenarios/an-unknown-config-key-is-named.md) requires a key in a `ductus`-owned section of `.ductus/config.toml` that no reader recognizes to be reported by name on the run that reads it (`CFG-ENV-012`), and its owner decision of 2026-10-20 names `check-artifact-size` as one of the readers that reports through its notices. It reads `[artifacts]`, and `/{project}:clarify`, `/{project}:plan` and `/{project}:analyze` show its notices. Until now a mistyped key there — `read-size-byte = 80000` — was dropped without a word, and the check ran at the default.

## Behavior

- `check-artifact-size` adds a notice for each key in `[artifacts]` that the shared registry (022's `config-readers-name-unknown-keys`) does not recognize, naming the key, the section and the file: `unknown config key: [artifacts] read-size-byte (.ductus/config.toml) — this version of ductus does not read it`. The check still runs, at the threshold the recognized keys give.
- `data-model.md`'s `notices` row names the notice.

## Edge Cases

- `[artifacts] read-size-byte = 80000` with no `read-size-bytes`: the check runs at 50,000 bytes, and the notice names `read-size-byte`.
- A malformed `read-size-bytes` still halts, as `an-invalid-threshold-fails-fast` says; the halt is the louder report, so no notice accompanies it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
