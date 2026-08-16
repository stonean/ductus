# 019 — Config-Persisted Decisions Data Model

Schema declaration for the `.ductus/config.toml` file. This file is the canonical reference for the framework; the README documents the same schema for adopters.

> **Note (post-completion):** paths in the live schema below reflect [042-consolidate-govern-per-project-files-under-govern-directory](../042-consolidate-govern-per-project-files-under-govern-directory/spec.md) (`.govern.toml` → `.ductus/config.toml`) and [044-relocate-constitution-under-govern-directory](../044-relocate-constitution-under-govern-directory/spec.md) (the pinned example's constitution path). The `[workflows]` section below is historical — removed by [043-workflows-sunset](../043-workflows-sunset/spec.md); see the post-completion note in [spec.md](spec.md). Its prose (including the `.govern.toml` literals inside it) is left as the decision-time record.

## File location and lifecycle

- Path: `.ductus/config.toml` (inside the committed `.ductus/` directory at the project root).
- Optional. If absent, `/ductus` uses default behavior for every key.
- Created lazily by `/ductus` when a user picks `Skip and don't ask again` and no file yet exists.
- Adopters may commit it (durable across clones) or `.gitignore` it (per-clone). Both are coherent.
- Format: TOML. Parse errors are a hard abort in `/ductus`.

## Sections

The file is a flat collection of top-level sections. There is no umbrella namespace (`[settings]`, `[decisions]`, etc.). Each section is keyed to the thing it governs, with internal keys chosen to fit that domain's vocabulary.

### `[pinned]` — file pinning (existing)

Unchanged by this spec. Documented here for completeness because it's a sibling to the new section.

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

### `[workflows]` — workflow recommendation declines (new)

Records categories the user has chosen to permanently decline at the per-category workflow recommendation prompt defined in [005-workflows](../005-workflows/spec.md).

| Key | Type | Required | Description |
| --- | --- | --- | --- |
| `declined_categories` | array of strings | no | Workflow categories `/ductus` will not re-prompt for. Matched case-insensitively against the registry-derived category list at decline-check time. |

```toml
[workflows]
declined_categories = ["Linting", "Formatting"]
```

#### Allowed values

The category list is the canonical set defined in [005-workflows](../005-workflows/spec.md):

- `Linting`
- `Formatting`
- `Testing`
- `Migrations`
- `Code Review`
- `Deployment`

Matching is case-insensitive — `"linting"`, `"Linting"`, and `"LINTING"` are equivalent. Storage is recommended in title case for human readability, but `/ductus` does not normalize the user's chosen casing.

#### Unrecognized entries

Entries that don't match any of the canonical category names (typos, removed categories, free-form notes) are reported once each in the post-scaffolding summary as:

```text
unrecognized workflow decline: "{value}" (in .govern.toml)
```

They do not abort the run, do not affect prompts, and are not auto-removed.

#### Empty section / empty key

- A `[workflows]` section with no `declined_categories` key is equivalent to no section at all — every category prompt fires normally.
- A `declined_categories` key with an empty array (`= []`) is also equivalent to no section — no categories are suppressed.

## Future sections (out of scope for this spec)

The flat-section layout is additive. Future specs that introduce new persisted-decision domains add their own top-level sections — examples deferred from spec 019:

- `[agents]` — recorded preferences for the agent-selection prompt.
- `[cleanup]` — recorded preferences for legacy-file cleanup confirmations.

Each new domain chooses its own keys to fit its decision shape (boolean toggles, arrays, structured records). There is no requirement for future domains to use a `declined_*` naming convention.

Adopters are not expected to author future sections by hand. Each future section, like `[workflows]` here, is created by `/ductus` when its corresponding prompt option is exercised.

## Schema validation

`/ductus` does not run a schema validator over `.ductus/config.toml`. Validation is per-key, ad-hoc, at the point each section is consumed:

- `[pinned] files` — entries that don't match a known manifest path are silently no-op (today's behavior, unchanged).
- `[workflows] declined_categories` — entries that don't match a registry-derived category name are surfaced in the post-scaffolding summary (per this spec).

There is no commit hook or `/ductus:analyze` rule for `.ductus/config.toml`. The post-scaffolding summary is the only enforcement layer, by design.

## Backwards compatibility

Projects with an existing `.govern.toml` containing only `[pinned]` continue to work without modification. The `[workflows]` section is purely additive — neither `/ductus` nor any other framework component requires it to exist.

Removing the `[workflows]` section from an existing `.govern.toml` (manually) reverts that project to today's prompt behavior on the next `/ductus` run.
