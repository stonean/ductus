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

- Architectural exploration: re-frame the runtime's LLM extension points (`writeCode`, `writeSpecBody`, `assessSpecQuality`, future multi-turn points) as named Anthropic-style Skills the host loads at the seam, rather than ad-hoc JSON envelopes. Potential benefits: structural cache anchoring (Skills are a natural cache boundary); third-party hosts integrate against an emerging Skills protocol instead of ductus-specific JSON; `constitution-excerpts` becomes a bundled resource rather than an inline string array. Speculative — depends on Anthropic's Skills protocol stabilizing and is a larger redesign than 022's current scope. Revisit after the writeCode payload-bundling scenario on 022 ships and the cache-anchored shape proves out the pattern. Surfaced 2026-05-19 during runtime-improvement investigation. **On hold per user 2026-07-11.**
- [ ] bug: the three-digit spec-number rule survives in five shell copies the runtime's single predicate does not reach, so a spec numbered past 999 is silently skipped by them. `parse_feature_dir` now accepts `1000-slug` and every runtime surface follows, but `.githooks/pre-commit:86` and the shipped `framework/bootstrap/hooks/ductus-pre-commit:116` both select staged specs with `[0-9][0-9][0-9]-`, so `label-criteria` never runs on such a spec and spec 013's labelling backstop is dead for it; `scripts/lint-frontmatter.sh:40-42`, `scripts/audit/sibling-coupling.sh:31,58,127` and `scripts/audit/introducing-drift.sh:70` glob the same shape and skip the directory entirely, exiting 0 while never having seen it — a check that cannot run looking like one that passed. Latent rather than live (this repo is at 051), but the runtime now creates the directory that triggers it. Routing decided with the user: a **scenario**, not a new spec; 051 is the candidate home, since it established `parse_feature_dir` as the one place the rule lives and these are the copies it did not reach. Fix as one change rather than five — the hooks can match three digits followed by any run, and the audit scripts should ask the runtime rather than glob. The two hook copies are separate files (AGENTS.md: the dogfooded copy is not the copy adopters get), and audit Family 22 already covers the hook's shape-matching.
