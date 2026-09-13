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

- [ ] convention: audit the spec corpus for stale `.govern.toml` references and rewrite only the live ones — measured 2026-09-13 at 413 hits / 105 files, of which the 69 in `runtime/src/` (the legacy-fallback resolution ladder), 14 in `framework/migrations/` (a migration must name the file it migrates), and 7 each in `framework/commands|bootstrap` and `scripts/audit/` are all **correct** and must not be swept; the question is the 316 hits / 72 files under `specs/`, where a living `spec.md` or scenario stating present-tense adopter guidance is drift (020's AC11 told adopters to configure a file 049 renamed — fixed in 317d678) while a `plan.md`, `review.md` or `tasks.md` recording what was decided when `.govern.toml` was current is history and stays. A blanket find-and-replace breaks the migration procedures and the runtime ladder, so this is per-hit judgment, not a rename sweep (captured during review of 020-code-review)
