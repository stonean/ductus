# Inbox

<!-- Rules:
     - Do not frontfill bugs that are not being actively worked on.
     - A bug or omission inside the scope of the spec currently in progress does NOT belong
       here — it becomes a task on that spec's tasks.md. The inbox is for todos with no
       home yet; an in-progress spec is already the home, so an item logged here is routed
       straight back to it (constitution §brownfield-inbox, scope decides the destination).
     - Nothing a pipeline run finds is written here. /review, /analyze, and /implement fix,
       route, or discard their own findings in the run that found them (constitution
       §brownfield-inbox, Finding dispositions); this file holds what a person logs.
     - Write specs for areas being actively touched — let adoption spread naturally.
     - As specs are written, items migrate from here into spec updates or new scenarios.
     - Chores (project maintenance with no feature home — lint/formatting cleanup,
       dependency cleanup, repo hygiene) may be logged here too; /groom does them in the
       grooming pass and removes them. They clear when done, not by migrating to a spec.
     - The brownfield backlog drains toward empty as adoption completes; the file
       persists as long as people keep logging todos.
     - Status notes do NOT belong here. Every item must be routable by /groom to one of
       its five routes (rule, spec, scenario, chore, discard); a "where things stand"
       or "what to do next" note matches none of them, so it would be walked and
       re-discarded on every pass forever. Pipeline state is derived — read it from
       /status, tasks.md, and git — not narrated into the backlog.

     Format each item as a checkbox list entry, recorded with /log, with a brief
     description and any relevant context:
        `- [ ] {Brief description of the issue and any relevant context}`

     When an item is migrated, remove it from this list. -->

- [ ] Dead file references remain in specs/ outside the plans the 2026-10-16 sweep fixed. That sweep corrected every plan.md Affected Files row naming a missing file (75 rows, 20 plans) and the prose in those plans naming the same files. Not yet done: (1) a backticked-path scan of spec bodies, scenarios and data models found ~610 unresolved candidates across 50 specs, unclassified -- many are adopter-side paths (.augment/, .agents/), examples or abbreviations, so the genuine count is unknown; (2) plans outside the 20 still name the retired generators (018 and 042 mention gen-spec-deps.sh and friends); (3) 019's plan still designs the [workflows] config section 043 removed. Proposed check, 067-owned: a twin of criterion-path-existence over plan.md Affected Files and backticked repo paths in done specs, excluding adopter destinations the way ships-to-adopter does -- it would turn the ~610 into an exact list and stop recurrence, since no check reads these paths today.
