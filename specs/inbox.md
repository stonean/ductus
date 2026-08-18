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
- [ ] bug: a retired-name MCP server stays live through the very run that retires it, and nothing tells the host to stop calling it — `framework/bootstrap/ductus.md` §State B (captured during 048-govern-acquired-runtime, from a real adopter bootstrap). Reproducing shape: an adopter whose MCP config still names the retired `gvrn` server key *and* who still has a `gvrn` binary installed (`ductus-rename` deliberately does not touch an installed binary), so the host's tool inventory carries `mcp__gvrn__*` for the whole migrating run. State-A detection is correctly namespace-scoped to `mcp__ductus__*`, so the run classifies State B and acquires — but §State B tells the host to invoke remaining primitives through the pointer CLI without saying the retired tools are off-limits, and `grep gvrn framework/bootstrap/ductus.md` returns nothing at all. Two silent effects observed on one run: (1) the retired `write-session` primitive's path resolver predates the `.ductus/` tier, so it wrote a stray `.govern/session.toml` that only got undone because the host happened to notice; (2) the permission entry the host added for that retired tool landed *after* `ductus-rename` step 10's rewrite had already passed, so `mcp__gvrn__write-session` survived in the adopter's settings while the other 41 converged — the run's "41 rewritten" was accurate for the entries existing when it ran. Candidate fix: one line in §State B directing the host to ignore non-`ductus`-namespaced MCP tools for the remainder of the run and use the pointer CLI.
