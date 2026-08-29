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
- [ ] convention: rewrite-spec-links rewrites a file line-wise with `lines()` + a bare `\n`, so a rewrite on a CRLF checkout converts the whole file to LF — a one-link change lands as a whole-file diff. Two siblings preserve the ending deliberately (create_feature::stamp_fold_target detects `\r\n`; derive_references picks its line_ending the same way) and check_stuck carries CRLF regression tests, so the convention is established and this is the one writer that departs from it. A shared line-ending-preserving rewrite helper would settle it in one place rather than three. — `runtime/src/primitives/rewrite_spec_links.rs` (captured during review of 022-deterministic-runtime)
- [ ] other: /ductus:fold rewrites corpus-wide links (step 10) before retire-feature enforces that the fold target exists (step 11), and step 11's own prose names that refusal as the answer to an unresolved folds-into — so the refusal is documented as reachable from a state where inbound links have already been re-pointed at a spec that does not exist, leaving the corpus edited and the staging directory still present. The window looks narrow (the body-edit write at step 6 and create-scenario at step 7 both need the target to exist, so most routes fail earlier), but narrow-by-accident is not the same as closed, and AC29 promises fully-folded-or-untouched per spec. Either check the target before the rewrite, or record why step 11 cannot fire once step 10 has run. — `framework/commands/fold.md` (captured during review of 051-branch-scoped-spec-numbering)
