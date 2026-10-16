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

There is no schema validator over the whole file. Each consumer validates the keys it reads when it reads them, so a malformed value in one section never blocks a command that reads another.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/bootstrap/ductus.md` | Modify | Project Configuration documents the store and its sections |
| `README.md` | Modify | The `.ductus/config.toml` section documents the store for adopters |
| `specs/019-config-decisions/data-model.md` | Create | The conventions every section follows |

## Trade-offs

- **No structured TOML validator.** Validation is per key at the point of use, so no single check reads the whole file; a value no consumer reads is never reported.
- **README and bootstrap drift risk.** Two places describe `.ductus/config.toml`. The installer's Project Configuration is canonical, and the README stays an adopter-facing summary.
