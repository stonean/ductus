# 019 — Config-Persisted Decisions Plan

Implements [019 — Config-Persisted Decisions](spec.md).

## Overview

`.ductus/config.toml` is the project's configuration and persisted-decisions store: a flat collection of top-level sections, each keyed to the thing it governs. `/ductus` and the pipeline commands read the sections they own, and a decision the operator makes once is written back as a section or key so it is not asked again. The installer's **Project Configuration** section (`framework/bootstrap/ductus.md`) documents every section; [data-model.md](data-model.md) records the conventions they share.

## Technical Decisions

### Persisted decisions are config sections

A decision worth remembering is stored in `.ductus/config.toml`, never in a separate state file, so configuration and decisions share one committed, reviewable store. Each section is keyed to the thing it governs and chooses keys that fit its own vocabulary; there is no umbrella namespace.

### Writes leave the rest of the file intact

A write changes only the keys or the managed block its section owns and preserves everything else in the file — other sections, comments, ordering.

### Validation is per key, at the point of use

There is no schema validator over the whole file. Each consumer validates the values of the keys it reads when it reads them, so a malformed value in one section never blocks a command that reads another.

### Unknown keys are reported by name, never dropped

`CFG-ENV-012` and [the unknown-key scenario](scenarios/an-unknown-config-key-is-named.md): a key in a `ductus`-owned section that no reader recognizes is reported by name, with its section and the file, and the run continues, because teammates can run different `ductus` versions against one committed file. Each runtime reader deserializes only its own section, and serde drops every key its struct does not name, so the recognized keys are declared once, in `runtime/src/schema/config_keys.rs`: plain keys per section; the fields of each named entry under `[constitutions]` and `[services]`; the fields of each `[[review.disabled-rule-files]]` entry; `[host] cli-config-dir`, which `Host::load` still reads as a fallback; and `[workflows]`, retired by 043, which recognizes none. A test in every reader compares the fields its structs read with the registry (`fields_of` reads a struct's field names through serde), so a key added to a reader without a registry entry fails a test, and `the_installer_documents_the_registry` holds the installer's §Project Configuration to the same set, so a host reading the file by hand reports what the runtime reports.

The check reads key names only. A section that is not a table, an entry that is not a table, or an array-of-tables key that is not an array is its reader's to report; and a document that does not parse reports nothing here, since its reader's parse error is the report. A key that needs quoting is shown quoted and escaped, as TOML writes it, so a newline or a terminal escape in a key cannot forge a line (`BE-INPUT-011`).

Where it is reported is the owner's decision of 2026-10-20 (the scenario's Resolved Questions): `discover-rule-files` adds a notice for each unknown key in `[rules]` and `[review]`, so `/{project}:review` reports them on the run that reads them; `check-artifact-size` does the same for `[artifacts]`, so `/{project}:clarify`, `/{project}:plan` and `/{project}:analyze` report them; `dashboard` names every unknown key in the file in one callout, so `/{project}:status` lists them all; `Host::load`, `Paths::load`, `resolve-constitutions`, `resolve-references`, `derive-references` and `check-orphaned-references` write the same line to stderr, once per process; and `/ductus` names those in the sections it reads in its post-scaffolding summary. The session file is not examined (the second Resolved Question): its top level is the exec walker's open seed.

### The runtime half is owned by 022 and 063

`AGENTS.md`'s routing entry gives a shared runtime surface to 022 and a feature primitive to its feature. The registry, `discover-rule-files` and `dashboard` are 022's, recorded in its scenario `config-readers-name-unknown-keys` and its task 5; `check-artifact-size` is 063's, recorded in its scenario `an-unknown-artifacts-key-is-named` and its task 13. Each links back here, which discharges this spec's `cross-spec-impact:` on both. This spec keeps the requirement, the convention in [data-model.md](data-model.md), and the installer and README documentation.

`dashboard`'s `config` gains `unknown-keys`. That is a change to a primitive's result, which `BE-COMPAT-001` asks a plan to classify: it adds a field and removes or retypes none, so a consumer reading the result as it did keeps working, but this project's judgment is that a result change ships at a pre-1.0 minor bump, which the release carrying 063's, 025's and 022's breaking changes is already owed (Cargo's caret requirements treat `0.y` as the compatibility boundary). The notices `discover-rule-files` and `check-artifact-size` add are new members of existing lists and change no shape.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/bootstrap/ductus.md` | Modify | Project Configuration documents the store, every section and key, and how an unknown key is reported |
| `framework/bootstrap/govern.md` | Modify | The retired bootstrap path, held byte-identical to `ductus.md` |
| `framework/bootstrap/ductus-procedure.md` | Modify | Post-Scaffolding Output names unknown keys |
| `framework/commands/status.md` | Modify | The rendering reference gains the unknown-keys callout |
| `framework/commands/review.md` | Modify | Rule-file selection names unknown `[rules]` and `[review]` keys |
| `framework/commands/clarify.md`, `framework/commands/plan.md`, `framework/commands/analyze.md` | Modify | The size check's markdown-only path names unknown `[artifacts]` keys; `/analyze` renders them as notices |
| `README.md` | Modify | The `.ductus/config.toml` section documents the store for adopters |
| `specs/019-config-decisions/data-model.md` | Create | The conventions every section follows |
| `runtime/src/schema/config_keys.rs` | Create | The registry of recognized keys and the unknown-key report (022's surface) |
| `runtime/src/primitives/discover_rule_files.rs` | Modify | Unknown-key notices for `[rules]` and `[review]` (022's surface) |
| `runtime/src/primitives/dashboard.rs` | Modify | `config.unknown-keys` and its callout (022's surface) |
| `runtime/src/primitives/check_artifact_size.rs` | Modify | Unknown-key notices for `[artifacts]` (063's surface) |
| `runtime/src/host.rs`, `runtime/src/schema/paths.rs`, `runtime/src/primitives/resolve_constitutions.rs`, `runtime/src/primitives/resolve_references.rs`, `runtime/src/primitives/derive_references.rs`, `runtime/src/primitives/check_orphaned_references.rs` | Modify | The stderr warning for the section each reads, and a test holding its keys to the registry (022's surface) |
| `runtime/src/schema/primitives.rs`, `runtime/src/mcp/server.rs` | Modify | `DashboardConfig.unknown_keys`, and the three tool descriptions (022's surface) |
| `.github/workflows/runtime.yml` | Modify | `framework/bootstrap/ductus.md` triggers the runtime suite, whose registry test reads it |

## Trade-offs

- **No structured TOML validator.** Values are validated per key at the point of use. The one file-wide check, `dashboard`'s, reads key names only, so a malformed value is reported by the command that reads it and by no other.
- **Over MCP, stderr reaches the server log, not the agent.** A typo in `[host]`, `[paths]`, `[services]`, `[constitutions]` or `[migrations]` reaches an agent-driven run only through stderr, and the operator on the next `/status`. Giving every reader a notice list instead would have reopened 063, 062, 055, 030 and 067 for their result contracts; the owner chose the narrower surface.
- **The session file is not examined.** A typo there goes unreported. Its top level seeds the exec walker's context, so it has no closed set of keys, and it is per-contributor and gitignored, so the several-versions case the rule exists for does not arise.
- **README and bootstrap drift risk.** Two places describe `.ductus/config.toml`. The installer's Project Configuration is canonical, and the README stays an adopter-facing summary.
