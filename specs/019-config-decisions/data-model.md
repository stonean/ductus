# 019 — Config-Persisted Decisions Data Model

Conventions for the `.ductus/config.toml` file. The installer's **Project Configuration** section (`framework/bootstrap/ductus.md`) documents every section; this file records what they have in common.

## File location and lifecycle

- Path: `.ductus/config.toml` (inside the committed `.ductus/` directory at the project root).
- Written by `/ductus`, which records the `[host]` block on every run and persists the inputs it collects on first run, so an adopted project always has one.
- Committed: it is the team-shared store. Per-contributor state lives in the gitignored `.ductus/session.toml`.
- Format: TOML. Parse errors are a hard abort in `/ductus`.

## Sections

The file is a flat collection of top-level sections. There is no umbrella namespace (`[settings]`, `[decisions]`, etc.). Each section is keyed to the thing it governs, with internal keys chosen to fit that domain's vocabulary. Every section is optional; an absent section means default behavior.

### `[pinned]` — file pinning

| Key | Type | Required | Description |
| --- | --- | --- | --- |
| `files` | array of strings | no | Destination paths (post-placeholder-resolution) for files `/ductus` should treat as `skip` instead of `update`. |

```toml
[pinned]
files = [
  ".claude/commands/myapp/implement.md",
  ".ductus/constitution.md",
]
```

Every other section is declared by the spec that introduced it and documented with the rest in the installer's Project Configuration section.

## Future sections

The flat-section layout is additive. A spec that introduces a new persisted-decision domain adds its own top-level section and chooses keys that fit its decision shape (boolean toggles, arrays, structured records); there is no required naming convention. Domains without a section yet include recorded preferences for the agent-selection prompt (`[agents]`) and for legacy-file cleanup confirmations (`[cleanup]`).

## Schema validation

`/ductus` does not run a schema validator over `.ductus/config.toml`. Validation is per key, at the point each section is consumed — for example, `[pinned] files` entries that match no manifest path are a no-op, and `[rules] surfaces` fails fast on an unrecognized member.

There is no commit hook for `.ductus/config.toml`. Two checks each read one section: `030-cross-service-references` gave `/ductus:analyze` a rule that reads the `[services]` registry and reports a **broken** reference — registered, checked out, target spec does not resolve — as an Advisory finding, and `/ductus:audit` Family 17 (`host-namespace-parity.sh`) reads `[host] project` to verify the slash-command namespace this repo renders matches the one it installed. Neither validates the file as a whole.
