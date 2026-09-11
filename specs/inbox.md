# Inbox

<!-- Rules:
     - Do not frontfill bugs that are not being actively worked on.
     - A bug or omission inside the scope of the spec currently in progress does NOT belong
       here — it becomes a task on that spec's tasks.md. The inbox is for findings with no
       home yet; an in-progress spec is already the home, so an item logged here is routed
       straight back to it (constitution §brownfield-inbox, scope decides the destination).
     - Write specs for areas being actively touched — let adoption spread naturally.
     - As specs are written, items migrate from here into spec updates or new scenarios.
     - Chores (project maintenance with no feature home — lint/formatting cleanup,
       dependency cleanup, repo hygiene) also live here; /groom recognizes them and leaves
       them in place. They clear when done, not by migrating to a spec.
     - The brownfield backlog drains toward empty as adoption completes; incidental
       capture is ongoing, so the file persists while work keeps surfacing issues.
     - Status notes do NOT belong here. Every item must be routable by /groom to one of
       its five routes (rule, spec, scenario, chore, discard); a "where things stand"
       or "what to do next" note matches none of them, so it would be walked and
       re-discarded on every pass forever. Pipeline state is derived — read it from
       /status, tasks.md, and git — not narrated into the backlog.

     Format each item as a checkbox list entry with a brief description and any relevant
     context. Three forms are in use:

     1. Manual entry (via /log) — the simple form below:
        `- [ ] {Brief description of the issue and any relevant context}`

     2. Auto-captured finding (an agent recorded this automatically while working a task,
        per §brownfield-inbox Automatic issue capture). Lead with a category so /groom can
        route it, and include a source pointer:
        `- [ ] {category}: {summary} — {file:line or area} (captured during {NNN-feature})`
        Categories: security, leak (memory/resource), convention, bug, perf, other.
        Security issues and leaks are the highest-priority captures.

     3. Audit finding written by /ductus — stricter form (see
        specs/008-security-rules/spec.md): `- [ ] {Rule ID}: {artifact} does not address — {summary}`.

     When an item is migrated, remove it from this list. -->

- [ ] bug: the clap subcommand enum in `runtime/src/main.rs` is the one primitive-wiring surface nothing pins — deleting a `Command` variant and its dispatch arm leaves the full suite green, so a primitive can be missing from the `ductus <name>` CLI undetected. Every other site is pinned (registry set-equality in tests/mcp.rs, the `#[tool]` listing, `dispatch_handles_every_registry_primitive`, and `PRIMITIVE_NAMES` being a direct alias). A test iterating `PRIMITIVE_REGISTRY` against the clap command list would close it. (found 2026-09-11 during 055)
- [ ] security: the decision not to call `validate_no_traversal` on a `[constitutions.*]` path rests on the claim that the value is hand-edited committed config — true today, but `/ductus:link` already sets the precedent for a command that writes a registry table programmatically. If anything ever writes `[constitutions]`, that trust claim silently becomes false and nothing checks it. Either keep hand-edit-only as a stated invariant or add the traversal check. — `runtime/src/primitives/resolve_constitutions.rs` (captured during review of 055-shared-constitution)
- [ ] convention: `schema/constitutions.rs` and `schema/services.rs` are near-identical registry modules, and `duplicate_paths` / `duplicate_repos` differ only in which field they group by. Deliberate at introduction (modelling on services was the point), but a third registry would make the case for one generic group-by-field helper. — `runtime/src/schema/constitutions.rs` (captured during review of 055-shared-constitution)
- [ ] other: `ConstitutionEntry.description` is parsed, documented in the config schema, and never surfaced — it reaches no primitive result and no report, unlike `ServiceEntry.description`, which the dashboard renders. Either surface it where a source is named or drop it from the schema. — `runtime/src/schema/constitutions.rs` (captured during review of 055-shared-constitution)
- [ ] convention: anchor-resolution — `§Project-level consistency` in `framework/commands/analyze.md` (steps 9 and 10) names that file's own `### Project-level consistency` section, not a constitution marker, so /ductus:analyze's project-level anchor check reports it unresolved against `framework/constitution.md`. The constitution's only near-match is `#### Project-level opt-out`, a different section. Every other installed command file resolves 0 unresolved. Either switch the two references to plain section-name prose (the `§` sigil is the constitution's notation) or give analyze.md its own `<!-- §... -->` marker. — `framework/commands/analyze.md` (captured during /ductus:analyze)
