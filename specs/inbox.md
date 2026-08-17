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
- [ ] bug: the stale-`ductus.md` pre-flight abort tells a pre-rename adopter to re-run a command they do not have. `framework/bootstrap/ductus.md:409` hardcodes the closing line **Start a new session and re-run `/ductus` to pick up the changes.**, but the self-update refreshes the installed copy *in place* — for an adopter whose bootstrap sits at `{config_dir}/commands/govern.md`, the file is still `govern.md` after the overwrite, and `ductus.md` does not exist until the `ductus-rename` migration moves it in the *next* session's batch. So the instruction names a dead command on exactly the path a pre-rename adopter takes: following it literally does nothing, and the migration chain never starts. Found 2026-08-17 during a real adopter bootstrap (048 AC10) against a pre-042 subject — legacy root config, root `constitution.md`, `workflows/`, and an `.mcp.json` naming the retired server key and bare command. The operator recovered by guessing the old command name; that guess is the defect. Fix: name the command the adopter actually has installed rather than the canonical one, deriving it from the installed path the self-update just wrote.
- [ ] bug: the self-update write target contradicts its own prose for a pre-rename adopter. `framework/bootstrap/ductus.md:381` says to write the freshly fetched upstream to "the agent's installed `ductus` file (overwrite)" and then names the path `{config_dir}/commands/ductus.md` — but a pre-rename adopter's installed file is `{config_dir}/commands/govern.md`, so prose and path disagree. Writing the literal path leaves the adopter with two command files, a stale `govern.md` beside a fresh `ductus.md`, and `ductus-rename` step 46 then moves `commands/govern.md` -> `commands/ductus.md`, clobbering the fresh copy with the stale one. Observed 2026-08-17 during a real adopter bootstrap (048 AC10): the agent wrote in place to `govern.md`, which is the correct behaviour and what the rename migration expects, so the literal path is the bug rather than the deviation. Fix: state the target as the installed path the staleness comparison already resolved, and keep the canonical filename owned solely by `ductus-rename`. Pairs with the abort-message bug above — same session, same adopter shape.
