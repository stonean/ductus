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
- [ ] bug: `lint-markdown` fails whenever `npx` is a shell function rather than a binary — the primitive spawns `npx` directly, so nvm's lazy-loader shim (where `command -v npx` prints `npx` with no path) yields `I/O error on <repo>: No such file or directory (os error 2)`. Hits both the MCP tool at runtime and `runtime/tests/mcp.rs:86` (`lint_markdown_returns_violations_array`), which fails on a clean tree in such an environment; the reported path is the repo root, which misdirects toward a missing fixture rather than a missing executable. Consider resolving the binary via a login shell or `node_modules/.bin` lookup, and surfacing spawn failure as "npx not found" distinctly from a fixture I/O error (captured during 023-govern-refinement task 21)
- [ ] convention: the `permissions.allow` section of `configure/claude.md` now contains a literal `Bash(git -C * status *)` inside its rationale prose, as the counter-example explaining why the seven `-C` entries were removed. On the markdown-only path an agent builds the canonical allow array by reading this section, and one that pattern-matches `Bash(...)` loosely rather than reading only the bulleted entries could re-add the exact entry the prose forbids. Family 29 anchors to the bullet form and is unaffected; the exposure is the host-side read. Consider whether the counter-example should sit outside the canonical section, or whether the markdown-only prose should state that only bullet entries are canonical. — `framework/bootstrap/configure/claude.md:50` (captured during review of 023-govern-refinement)
