# Inbox

<!-- Rules:
     - Do not frontfill bugs that are not being actively worked on.
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

- Architectural exploration: re-frame the runtime's LLM extension points (`writeCode`, `writeSpecBody`, `assessSpecQuality`, future multi-turn points) as named Anthropic-style Skills the host loads at the seam, rather than ad-hoc JSON envelopes. Potential benefits: structural cache anchoring (Skills are a natural cache boundary); third-party hosts integrate against an emerging Skills protocol instead of ductus-specific JSON; `constitution-excerpts` becomes a bundled resource rather than an inline string array. Speculative — depends on Anthropic's Skills protocol stabilizing and is a larger redesign than 022's current scope. Revisit after the writeCode payload-bundling scenario on 022 ships and the cache-anchored shape proves out the pattern. Surfaced 2026-05-19 during runtime-improvement investigation. **On hold per user 2026-07-11.**
- [ ] Framework gap — `/{project}:review` has no durable home for a finding that maps to no rule. The five passes bucket into MUST / SHOULD / low-confidence / waived, and `write-review` renders exactly that set; anything else a reviewer notices has nowhere to go but the free-text Summary, which is regenerated wholesale on the next run and is per-spec besides, so a cross-cutting observation put there is both erased and misfiled. §brownfield-inbox's automatic capture is the intended route, but it depends on the agent *remembering* to call `append-inbox` at the moment of noticing, and nothing detects an uncaptured observation — the silent-degradation shape AGENTS.md's second Design Principle forbids. Observed 2026-08-16: I surfaced the done-spec review-staleness gap (now 022 task 88) and recorded it only in 017's review Summary and a commit message; it survived because the operator asked where it was. Proposed fix, derived rather than remembered: give `write-review` an `observations` array that appends each entry to `specs/inbox.md` (dedup-guarded) as a side effect and renders it in its own report section, so recording an observation in the report *is* the capture and the two cannot diverge.
- [ ] Framework gap — work arriving through conversation skips the routing the inbox path enforces. `/{project}:groom` implements a five-route decision tree that matches an item to an existing spec, adds a scenario, and takes the back-edge. `/{project}:specify` has no equivalent: it goes straight to the create-feature gate with no check for whether the work belongs to an existing rule surface or to 022. So AGENTS.md's two routing rules — "add a rule to its surface's home spec via the back-edge, do not spawn a new spec" and "route runtime work to 022 via the back-edge" — are enforced only for items that happen to enter via the inbox. They also live in AGENTS.md §Workflow, which no command loads as normative criteria (`/{project}:review` loads Code Style, Testing, Gotchas, Boundaries), so at the one moment routing can be got wrong there is nothing in context stating the rule. Observed 2026-08-16: I proposed a new spec for a `check-artifacts` family change, which the runtime-work rule routes to 022 as a scenario; the operator caught it. Proposed fix: have `/{project}:specify` run the same routing decision groom already owns before scaffolding — derive candidate surfaces (rule files by category, 022 for runtime work) and make "new spec" the confirmed choice over "scenario on NNN" rather than the default. Consider also whether §Workflow's routing rules belong in a section commands load.
- [ ] When implementing without `--auto`, on completing the current task: if next step(s) exist, prompt the operator to confirm continuing on the next recommended step — this shows where the work stands and lets them simply reply "yes" to continue, or type instructions instead to go a different direction
