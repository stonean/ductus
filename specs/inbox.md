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

     3. Audit finding written by /govern — stricter form (see
        specs/008-security-rules/spec.md): `- [ ] {Rule ID}: {artifact} does not address — {summary}`.

     When an item is migrated, remove it from this list. -->

- Architectural exploration: re-frame the runtime's LLM extension points (`writeCode`, `writeSpecBody`, `assessSpecQuality`, future multi-turn points) as named Anthropic-style Skills the host loads at the seam, rather than ad-hoc JSON envelopes. Potential benefits: structural cache anchoring (Skills are a natural cache boundary); third-party hosts integrate against an emerging Skills protocol instead of govern-specific JSON; `constitution-excerpts` becomes a bundled resource rather than an inline string array. Speculative — depends on Anthropic's Skills protocol stabilizing and is a larger redesign than 022's current scope. Revisit after the writeCode payload-bundling scenario on 022 ships and the cache-anchored shape proves out the pattern. Surfaced 2026-05-19 during runtime-improvement investigation. **On hold per user 2026-07-11.**
- [ ] Scenario added outside `/amend` and committed is invisible on a `done` spec: `/amend`'s re-open precondition detects only *uncommitted* deltas (`git status --porcelain`, untracked `??` scenario files — `framework/commands/amend.md:51-53`) and fires only on `done` specs (`:68`), while `/analyze`'s scenario→task mapping family explicitly does not flag any scenario under a `done` spec (tasks may be pruned — `framework/commands/analyze.md`, Scenario consistency). The `scenario-open-questions` family (046) covers the case where the hand-added scenario carries questions, but a committed question-free scenario with no task leaves the spec at `done` with unimplemented behavior and nothing surfacing it. Note the ambiguity is real, not obviously a defect: §scenarios says an implemented scenario stays as documentation, so nothing distinguishes "already shipped, documented after the fact" from "unimplemented work". Surfaced 2026-08-14 while clarifying 022/scenario-open-question-signal.
- [ ] Advisory: anchor-resolution — 2 unresolved `§` references in `specs/022-deterministic-runtime/spec.md` (`§The primitive library` at :44/:90/:184/:217, `§LLM extension points` at :45/:185) — the spec uses `§` for its own section names, but `resolve-anchor` tokenizes only the first word and resolves against constitution markers, so multi-word self-references report as unresolved `The` and `LLM`. Pre-existing and cosmetic; the fix is either a spec convention (stop using `§` for in-spec sections) or a scanner change (ignore `§` refs whose first token is not a known marker). — specs/022-deterministic-runtime/spec.md (captured during /gov:analyze)
- [ ] No command appends a task when an **existing** scenario gains new behavior. `/gov:amend`'s scenario route is the only path that calls `append-task`, and it runs `create-scenario` first, which refuses on slug conflict — so a scenario whose requirement is extended (e.g. via `/gov:clarify` resolving one of its open questions into new behavior) has no route to a task. `/gov:plan` gates on `clarified` and an `in-progress` spec fails it; `/gov:implement` reads `tasks.md` and finds nothing to do. This leaves §implement-phase's "if new work is discovered, add it as a task first" with no mechanism on the reopen cycle, and the workaround is calling the `append-task` primitive directly. Same shape as 046's gap: a reachable pipeline state with no command routing out of it. Surfaced 2026-08-14 while clarifying 022/scenario-open-question-signal.
