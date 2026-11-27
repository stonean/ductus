---
status: done
dependencies: []
cross-spec-impact: [022-deterministic-runtime, 063-oversized-artifacts-warn-with-a-fix]
next-criterion: 11
---

# 019 — Config-Persisted Decisions

> **Note ([043-workflows-sunset](../043-workflows-sunset/spec.md)):** the `[workflows] declined_categories` section this spec first carried went with the workflows feature. The config-persistence *mechanism* this spec established (sections as persisted decisions, additive writes, the schema data-model) is what remains.

`.ductus/config.toml` is the project's persisted-decisions store, not only a pin file. Interactive choices `/ductus` makes that would otherwise be asked again on every run are recorded in it, so a decision made once is respected without re-prompting.

This feature established `.ductus/config.toml` as that store; declining workflow recommendations was its motivating use case.

## Motivation

Two things are true about `/ductus`:

- It is designed to be re-run frequently — every time the framework moves forward, every adopter is encouraged to re-run to pull the changes.
- It surfaces interactive prompts for opt-in scaffolding (agent selection) that the adopter has already answered in a previous run.

Without persistence, the second contradicts the first: the more often a user re-runs `/ductus`, the more often they are asked the same question. The pragmatic outcome is that adopters either (a) silently accept defaults to make the noise stop, or (b) avoid running `/ductus` because the prompts are tedious. Both are bad — the first defeats the explicit-consent design of workflow scaffolding, the second strands adopters on stale framework versions.

Persisted declines fix this: the user answers once, the answer is recorded, and subsequent runs respect it without re-prompting.

## Behavior

A decision the operator makes once, which `/ductus` or a pipeline command would otherwise ask again, is written to `.ductus/config.toml` as a section or a key in the section keyed to the thing it governs, and later runs respect it without re-prompting. A write changes only the keys or the managed block its section owns, so other sections, comments and ordering survive.

The operator undoes a recorded decision by editing `.ductus/config.toml` and removing the entry — a deliberate, file-edit action with no hidden flag. A recorded decision is permanent: there is no TTL, framework-version trigger, or auto-cleanup. A stale or unrecognized entry does not error and does not abort the run: a key in a `ductus`-owned section that no reader recognizes is reported by name (`CFG-ENV-012`; scenario `an-unknown-config-key-is-named`) and the run proceeds. A file that does not parse aborts the run with a clear error.

## Schema

`.ductus/config.toml` is a flat collection of top-level sections, `[pinned]` among them. Domains stay flat at the top level — there is no umbrella `[settings]` or `[decisions]` namespace.

```toml
[pinned]
files = [".claude/commands/myapp/implement.md"]
```

Each decision domain gets its own top-level section keyed to the thing it governs (e.g., `[agents]`), with internal keys chosen to fit that domain's vocabulary rather than forced into a generic shape. A section's keys are declared in the registry (`runtime/src/schema/config_keys.rs`) and documented in the installer's Project Configuration.

## Acceptance Criteria

- [x] AC9: If `.ductus/config.toml` is malformed (TOML parse error), `/ductus` aborts with a clear error message — same fail-loud posture as today.
- [x] AC10: The README's Configuration section documents `.ductus/config.toml`, with the `[pinned]` example.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Schema shape.** Top-level section per domain, with domain-specific keys. Mirrors the existing `[pinned] files = [...]` structure — top-level section keyed by the thing it governs, contents shaped to that thing's vocabulary. Each domain chooses its own keys (a future `[agents]` section might use `excluded` or `defaults` depending on what its decision actually is) rather than forcing a generic `declined = [...]` shape on every domain. Rejected: a `[decisions.*]` umbrella misclassifies `[pinned]` (which is configuration, not a decision), and a `[settings.*]` umbrella commits to expanding into non-decision config without a current need. Additive — future domains nest at the top level without disturbing existing sections.
- **Existing `[pinned]` relationship.** `[pinned]` stays as a top-level section, unchanged. Resolved by the schema-shape answer above: there is no umbrella to migrate under. `[pinned]` and every other section are siblings, not nested. This avoids any breaking change to existing `.ductus/config.toml` files.
- **TTL or permanence.** Permanent until manually edited. Matches the existing `[pinned]` model (no expiration in the same file). Time-based expiration ("re-ask after N runs/days") would re-introduce the prompt fatigue the feature exists to eliminate. Version-based expiration is impractical because `/ductus` runs continuously on `main` — there is no clean version boundary that wouldn't degenerate into "re-prompt on every framework commit." The genuine reasons to reconsider a recorded decision are user-driven (team adopts a tool, language changes), not calendar-driven; those cases are exactly when an adopter would naturally edit `.ductus/config.toml`. Tradeoff accepted: stale entries can rot in `.ductus/config.toml` indefinitely.
