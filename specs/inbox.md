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
<<<<<<< HEAD

- [ ] criterion-path-existence ships-to-adopter set misses per-agent scaffold paths (064's pi AC1/AC3 flagged advisory against a dogfooded tree): 032 avoids the same finding only because .opencode/ isn't dogfooded here — the Shared Files destination set (adopter_destinations in check_artifacts.rs) excludes .pi/prompts/ductus.md and .pi/settings.json, which /ductus writes at adoption but no generator materializes in this tree. Fix options: extend adopter_destinations to the per-agent scaffold paths, or annotate install-destination criteria as ships-to-adopter. Recorded from 064's analyze 2026-09-22 (advisory 3, unexamined 2 root-absent).
- [ ] Review staleness's two halves still build different repo-wide rewrite sets: Family 19 (scripts/audit/review-freshness.sh) runs `git diff`, which detects renames, so a renamed file's substitutions count toward the repo-wide pairs; the transition gate's SweepIndex::build (runtime/src/primitives/mechanical_sweep.rs) uses git2's tree diff with no rename detection, so the same file is a delete plus an add and contributes nothing. Surfaced 2026-10-05 on PR #5's 063->064/064->065 renumber, where it was the reason the halves disagreed (the chaining that turned the disagreement into a wrong verdict is fixed — 022 scenario a-multi-number-renumber-is-one-rewrite). Pick one rule for both, and let mechanical_sweep_parity pin it with a renamed-directory fixture.
- [ ] /analyze's command-frontmatter checks name the claude-style shape only: framework/commands/analyze.md:428-436 read `{cli-config-dir}/commands/{project}/help.md`, every `.md` in `{cli-config-dir}/commands/{project}/`, and `{cli-config-dir}/commands/ductus.md`. Under the §Derived values table that path is right for claude-style alone — OpenCode installs to `command/{project}/` (singular), Antigravity to `skills/{project}-<name>/SKILL.md`, and Pi (064) to flat `prompts/{project}-<name>.md` with the self-install at `prompts/ductus.md` — so for those adopters the checks read a directory that does not exist. Point the steps at the §Derived values Command/skill path and ductus install path rows instead of restating one layout's shape. Surfaced 2026-10-05 reviewing PR #5, when the dogfooded .pi/prompts/ductus-analyze.md came out naming `.pi/commands/ductus/`.
=======
>>>>>>> 7a5da6ee112275a12643e6a045c97709f221aca5
