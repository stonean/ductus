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
- [ ] bug: `compute-review-scope` returns an unusable scope and a polluted captured-issues list — `runtime/src/primitives/compute_review_scope.rs` (captured during 045). **(a) The plan-affected parse is not a table parse.** Running it against `017-derive-dont-ask` returns entries like `"File"` (a table *header* cell, repeated 8 times), ``"constitution.md` (root)"``, and ``"specs/000-016/spec.md` (and one `spec-and-plan.md` if any)"``. It appears to take any backticked token from the Affected Files section rather than the first cell of each body row, so header rows, parenthetical qualifiers, and prose ranges all land in the scope. Because the command selects *whichever set is larger* between plan-affected and modified-since, a spec with a long historical Affected Files table (013 and 017 both qualify) gets the garbage set chosen over the accurate one — so the review scopes files that do not exist and misses the ones that actually changed. **(b) `captured-issues` takes raw added lines, not inbox bullets.** Adding the shipped `<!-- Rules: … -->` comment block back to `specs/inbox.md` made it report ~30 captured issues, one per comment line, including bare `-` lines inside the comment. The inbox primitives already share a comment- and fence-aware bullet grammar (`iter_bullets` / `count_inbox_bullets` in `runtime/src/primitives/mod.rs`) that exists precisely to stop this; `compute-review-scope` does not use it. **Impact:** (a) silently reviews the wrong files, which is a review that reads as clean without having examined the change — the `QUAL-CLAIM-001` shape at the command level rather than the primitive level. (b) is cosmetic but noisy, and it makes the Captured issues section of `review.md` useless on any commit that touches a comment block. **Routing:** both are `compute-review-scope` behavior, so a scenario under `022-deterministic-runtime`; (b) is a two-line fix (reuse the shared bullet iterator), (a) needs a real table parse plus a decision about whether "larger set wins" is the right selection rule at all — an accurate small set is more useful than an inflated wrong one, which suggests the rule should prefer modified-since when the plan table fails to parse cleanly.
