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
- [ ] bug: an adopter-owned `create`-strategy file outside the three the rename repairs can carry a stale framework path that nothing fixes and nothing reports — `runtime/src/primitives/check_orphaned_references.rs` REFERRERS (found by a real adopter bootstrap). Reproducing shape: an adopter whose `specs/system.md` references the constitution's path (a link, a tree diagram) is migrated by `ductus-rename`; step 3 rewrites `.govern/constitution.md` references in exactly `CLAUDE.md`, `AGENTS.md` and `README.md`, and `check-orphaned-references` examines those three plus `.githooks/pre-commit`. `specs/system.md` is `create`-strategy and adopter-owned like all four, so nothing rescaffolds it, no migration step names it, and the post-batch orphan check does not look at it — the run completes clean while the file points at a directory that no longer exists. The current scope is principled (the framework repairs only references it wrote itself) but the blind spot is not: an adopter-authored reference to a framework path is invisible to both halves. Observed on a live run, where a host caught it by reading rather than by any check. Candidate fix: add `specs/system.md` to REFERRERS so the orphan check at least *reports* it, leaving repair to the adopter, since the framework did not author the reference. Runtime change — routes to 022 via the back-edge per AGENTS.md, and needs a version bump plus a ductus-v tag to reach anyone.
