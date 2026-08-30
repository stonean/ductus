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
- [ ] Chore: sweep the README for current functionality — add /fold to the Commands section, document /specify's --branch, --branch-id, and --fold-into flags (other rows document theirs), cover the branch-scoped {identifier}.{n}-{slug} numbering form, and note that /status reports pending folds. Leave /audit absent — it is maintainer-only by design. The recurrence guard is homed at 026 scenario readme-command-parity; this is the one-time correction it will first surface.
- [ ] other: `merge-permissions` `revoke` serves only the Claude permission shape, so the other three hosts have no retirement path — a formerly-canonical entry that ever ships in `configure/opencode.md`, `configure/auggie.md`, or `configure/antigravity.md` would survive in adopter trees indefinitely, the exact gap spec 023's retirement just closed for `claude.md`. Not urgent: none of the three currently carries a retired entry, and their permission grammars never expressed the two shapes that were retired (a wildcard before the subcommand, a path-scoped `Write(...)`). Shares the single-format limitation already recorded on 022's `coverage-residue-cleanup` scenario, and would be resolved by the same per-format merge primitive that scenario names as future work if that path becomes hot — retirement is one more reason it might. Audit Family 32 is likewise claude-only for the same reason. — `runtime/src/primitives/merge_permissions.rs`, `framework/bootstrap/configure/` (captured during 023-govern-refinement)
