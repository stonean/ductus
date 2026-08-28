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
- [ ] convention: `specs/042-consolidate-govern-per-project-files-under-govern-directory/review.md` contradicts itself in prose — its Summary says "1 low-confidence note retained (probe-to-use race …)" while its own frontmatter records `low-confidence: 0` and its `## Low-confidence findings` section reads "*None remaining.* The finding below is **resolved**" (resolved 2026-08-02 under 022's config-resolution-single-probe). The Summary is the pre-resolution narrative left unswept when the finding closed. Family 31 (review block agreement) does not catch it by design — its subject is frontmatter only, since a report's narrative legitimately discusses counts and a prose check would report every review that describes its own findings. Candidate: no new family; the sweep obligation belongs to whoever resolves a finding in a report. Surfaced 2026-08-28 while implementing Family 31, which found the frontmatter half of the same drift (`spec.md` carried `low-confidence: 1`, now reconciled to the report's `0`).
- [ ] bug: `scalar()` in `scripts/audit/review-freshness.sh` (Family 19) uses `\s*` after the key name, and `\s` matches a newline — so on a bare empty value (`reviewed-against:` with nothing after the colon) the greedy run walks onto the next line and returns *that* line's content instead of None. Verified directly: the helper returns `'must-violations: 7'` for a frontmatter block whose `reviewed-against:` is empty. The effect is a wrong finding rather than a missed one — Family 19 would report an unresolvable-sha finding instead of deferring the null to `check-review-gate` as its scenario specifies — so it fails in the safe direction but with a misleading message. Latent today: no spec currently carries a bare empty `reviewed-against` (the template writes `null`, which parses correctly). Family 31 carried the same helper by copy and fixed it there with `[ \t]*`; the two copies have now diverged in correctness, which is the duplication cost REUSE-001 names in this review. Fix with `[ \t]*` in Family 19, or fold both into the runtime primitive that review proposes. — `scripts/audit/review-freshness.sh` (captured during review of 026-framework-self-audit)
