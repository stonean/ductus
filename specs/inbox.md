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
- [ ] Framework gap — a `done` spec's review can go stale with nothing reporting it. `check-review-gate`'s `review-stale` check fires only on the `in-progress → done` transition, so once a spec is `done` its recorded verdict is never re-evaluated when its durable contracts change underneath it. `check-artifacts`' review-state-drift family covers a *missing* or *blocking* review on a done spec, not a *stale* one, so nothing surfaces the drift either. Observed 2026-08-16: 017 was found stale on 4 durable contracts, 3 of which (data-model.md, detect-dependency-cycles.md, skip-prose-cross-references.md) changed during the 0.28.0 cycle while it sat at `done` — its verdict described code from 2026-08-02 and only surfaced because an unrelated back-edge forced a transition. The same is likely true of other done specs swept in that cycle. Needs a design decision, not a mechanical fix: extend the review-state-drift family to evaluate staleness on done specs (advisory, since it would reopen nothing by itself), or accept that a done spec's verdict is a point-in-time record and say so explicitly. Surfaced by /ductus:review on 017.
