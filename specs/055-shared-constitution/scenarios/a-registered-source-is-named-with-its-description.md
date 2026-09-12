---
section: "Shape of the setting"
---

# A-registered-source-is-named-with-its-description

## Context

`ConstitutionEntry` carries an optional `description` — a human- or agent-facing note on what a registered source is for. It is parsed, documented in `framework/bootstrap/ductus.md`'s config schema, shown in the manual's worked example, and recorded in this spec's `data-model.md`. It reaches nothing: no primitive result, no report, no command output. Its doc comment says *"Informational only — no runtime behavior depends on it."*

`ServiceEntry.description` is the near-identical field on the registry this one was modelled on, and `dashboard` renders it. So an adopter who writes a description for a service sees it; an adopter who writes one for a constitution sees it only by reopening the config file they just wrote it in.

That asymmetry matters more here than it would for services, because of what this spec's **Shape of the setting** section already requires: **the source is attributable** — *"A contributor reading a rule can tell which source it came from, and a run says which sources it loaded."* AC4 says the same. A run currently satisfies that by naming an alias. An alias is a config key someone chose, often a bare org name, and it says nothing about what the document governs. `description` is the field that would answer that question, and it is the one field the reporting path drops.

## Behavior

`resolve-constitutions` carries `description` through to `ConstitutionRecord`, so it reaches the surfaces that name a source rather than stopping at the parser. The field stays optional and stays informational — no resolution behavior depends on it, and an entry without one is unchanged.

Where a source is named, the description is rendered with it when present:

- `/{project}:target` step 4 reports loaded sources by alias, and reports each `skipped` entry by alias with its reason. Both gain the description when the entry has one, so a contributor reading the session's opening lines can tell what a source governs without opening `.ductus/config.toml`.
- `write-review`'s `## Unexamined governance` section already names each source that could not be read, one bullet per entry. Those bullets gain the description, which is the difference between an operator recognising which checkout they are missing and having to go look the alias up.
- `write-analysis` is **not** changed, and the reason is worth recording because it looked like a symmetric case. Its record is a reason-keyed **count** (`constitution-unresolved: N` inside `unexamined-by-reason`) — it names no source, so there is no line for a description to make actionable. Giving that record per-source names is a change to the record's shape, with its own consumers and its own audit families, and it is a separate subject from this one.

Absent is absent, not empty: an entry with no description renders exactly as it does today, so a project that never writes one sees no change. That keeps AC1's unset-is-unchanged posture intact one level down.

## Edge Cases

- **No description on any entry.** Every surface renders as it does now. The field is additive and its absence is the common case.
- **A description long enough to break a one-line report.** The reports are single-line by contract, so the render truncates rather than wrapping — the same posture the disabled-rule-file notice takes when it collapses internal whitespace to keep its line single.
- **A description containing a newline.** TOML multi-line strings make this reachable; internal whitespace is collapsed before rendering, for the same reason.
- **A skipped entry.** The description still renders. It comes from the config, not from the checkout, so it is available precisely when the document is not — which is the case where it helps most.
- **`duplicate-paths`.** Two aliases naming one checkout may carry different descriptions; both are reported with the existing warning, since the descriptions are what would let an operator see which registration is the redundant one.
- **The dashboard.** `dashboard` renders `[services]` descriptions but knows nothing of `[constitutions]`; this scenario does not add it there. Registered constitutions are a session-loading concern surfaced by `/{project}:target`, not a pipeline-status one, and widening the dashboard is a separate decision with its own subject.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
