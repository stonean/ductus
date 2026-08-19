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
- [ ] bug: `/ductus` cannot acquire the runtime on a greenfield adoption — the pin it reads does not exist yet. `framework/bootstrap/ductus.md` §Runtime acquisition Branch 2 step 1 (line ~253) reads `{staging-dir}/ductus-main/version` and **halts** when absent ("guessing a version … silently installs a runtime the framework was never tested against"). But §Pre-flight Phase (line ~167) states it runs "before Pre-run Migrations and the full archive fetch" and that both its checks "run on a small fetch or no fetch"; §Self-update check's small fetch pulls only `ductus.md` into the temp dir, and §Archive fetch and extract — the only step that creates `ductus-main/` — is at line ~586. So every first-time adopter (State B by definition) reaches acquisition with no pin on disk and, followed faithfully, halts. Verified empirically 2026-08-19 against a real greenfield adoption: installer OK, self-update `current`, staging dir contained only `ductus.md.upstream`, no `ductus-main/version`; supplying the pin by hand let the rest of acquisition pass (digest verified, 0.31.0 installed, pointer + MCP wiring + scaffold + `create-feature` all worked). Pre-existing, introduced with 048's acquisition work (a4a3358). Candidate fixes: fetch the one-line `version` from raw in the pre-flight small fetch alongside `ductus.md`, or move the archive fetch ahead of acquisition. Affects AC1/AC10 of 048.
