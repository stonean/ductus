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
- [ ] bug: spec 005's acceptance criterion asserts workflow files "sit directly under `framework/workflows/`", which spec 043 deleted — a `done` spec's contract naming a path that no longer resolves. Surfaced by `check-artifacts` `criterion-path-existence` on 2026-08-03; one of only two true positives in a 21-finding sweep. Route: back-edge 005 and reword the criterion to past tense, or record 043 as superseding it.
- [ ] bug: spec 025's acceptance criterion asserts `.govern.toml` hygiene "belongs to `scripts/lint-govern-toml.sh` (single-purpose tool)", but that script was never built — a `done` spec's contract naming a path that does not exist. Surfaced by `check-artifacts` `criterion-path-existence` on 2026-08-03; the second of two true positives in a 21-finding sweep. Route: back-edge 025 and either build the linter or reword the criterion to drop the claim.
- [ ] convention: `check-artifacts` `criterion-path-existence` reports 19 findings across specs 000/003/008/012/018/044 for paths `govern` creates in an *adopter's* checkout (`specs/templates/`, `specs/rules/*`, `specs/system.md`, `specs/errors.md`, `specs/events.md`, `.githooks/govern-pre-commit`, `.govern/constitution.md`). Each criterion is correct and satisfied in the repo it describes; they fail only because this repo is the framework source rather than an adopter. 045's data-model triaged them as "a dogfooding artifact, not a check defect" and deferred. Every `/gov:analyze` run re-reports all 19, so the real cost is that genuine findings hide in the noise. Route: suppress by matching the candidate path against the §Shared Files manifest destinations (present only in the framework repo), recorded in `skipped` rather than dropped.
- [ ] other: `/gov:analyze` has no durable sink for its findings. `/gov:review` writes `review.md` and sets `review:` frontmatter, so its findings survive the session and are auditable later; `/gov:analyze` is read-only and its advisory findings exist only in the invoking session's output. A comprehensive sweep on 2026-08-03 produced 21 `criterion-path-existence` findings that were visible nowhere in git until logged here by hand — losing the session would have hidden them until someone re-ran a full audit. This is the asymmetry that makes §brownfield-inbox auto-capture load-bearing rather than optional, and relying on the agent to remember is the failure mode §drift-prevention exists to remove. Route: a spec — either an `analyze.md`-written artifact mirroring `review.md`, or a required auto-capture step in the command's procedure.
