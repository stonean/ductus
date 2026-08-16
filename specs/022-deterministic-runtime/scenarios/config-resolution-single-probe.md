---
section: "Follow-on scenarios"
---

# Config-resolution-single-probe

## Context

Spec 042 made config resolution new-wins: read `.ductus/config.toml` when it
exists, else the legacy root `.govern.toml`. Two primitives need both halves of
that answer — the path to read, and the repo-relative name to render as the
provenance tag on a notice. `discover-rule-files` and `dashboard` each obtained
them from two separate calls: `paths::config_path(repo)` at load time, and
`paths::config_display_name(repo)` again at render time.

That is two existence probes at two different moments. A `/ductus` migration
landing between them would let the primitive read one file and attribute its
contents to the other — the notice would name `.ductus/config.toml` while the
disabled-rule-file list came from the legacy root file, or the reverse.

Carried forward across two review runs of
[042](../../042-consolidate-govern-per-project-files-under-govern-directory/review.md)
as `BE-RACE-001`, each time recorded as low-confidence rather than closed. The
mitigations are real — the pipeline is serial per constitution
[§concurrent-features](../../../framework/constitution.md#concurrent-features),
the migration runs only inside `/ductus`, and writes are atomic tempfile+rename
— but they are all arguments about *who else is running*, not about the
primitive being correct on its own terms.

## Behavior

`schema/paths.rs` gains `resolve_config(repo) -> (PathBuf, &'static str)`: one
existence probe yielding both the path to read and the name to render.

Both primitives resolve once and carry the name forward with the parsed
content:

- `discover_rule_files::load_ductus_toml` returns `(DuctusToml, &'static str)`;
  the name flows into `apply_disabled_filter` instead of a second probe.
- `dashboard::load_config` returns `(DashboardConfig, &'static str)`; the name
  is threaded through `render_markdown` into `render_callouts`.

`config_path` and `config_display_name` remain for callers that need only one
half (`host.rs`, `resolve_references`). Their doc comments now direct callers
needing both to `resolve_config`.

The name is threaded as a function argument rather than added to
`DashboardConfig`, which is a serialized MCP result type — the provenance tag
is a render-time concern, not part of the wire contract.

## Edge Cases

- **Neither config file present** — `resolve_config` returns the legacy path
  and the legacy name, matching `config_path`'s documented behavior; the
  caller's `is_file()` check still yields the config-absent defaults, and the
  name is never rendered because the notice requires a config that was read.
- **Both files present** — new-wins, exactly as before; the single probe makes
  the choice once so the tag cannot disagree with the read.
- **Migration lands between resolution and the file open** — still possible,
  and deliberately not addressed: the read would fail or return the legacy
  file's content, which is the config-absent / stale-read case the caller
  already handles. What this closes is the narrower defect of the primitive
  *contradicting itself* within one result.
- **`DashboardConfig` wire shape** — unchanged. No golden re-bless, no MCP
  schema change.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
