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
- [ ] rule: `AGENTS.md` carries adopter-beneficial rules in a file adopters never receive. It is contributor-only — the Shared Files manifest ships `framework/constitution.md` → `.ductus/constitution.md`, the rule files, the hooks and the templates, but never this repo's `AGENTS.md` — so a rule learned here that is true for *any* ductus project reaches nobody. Surveyed 2026-08-17: 56 entries, of which ~12 are strongly universal (both §Design Principles entries; a markdown link in a spec body creating a `dependencies:` edge; `git checkout -- specs/{feature}/` silently reverting uncommitted pipeline state; never `git add -A` because it sweeps untracked `/specify` drafts; never hand-writing an `AC{n}` label; `create-scenario`'s auto-appended question scaffolding; routing a new rule to its surface's home spec instead of a new spec; re-opening a done spec via `set-status` for on-disk-edit-only cases; the prose-claim sweep that identifier sweeps miss; `fetch-depth: 0` for history-reading CI checks; treating `.ductus/config.toml` as a shared database rather than one spec's schema; and recording a superseded acceptance criterion in the spec body), ~10 borderline, and ~24 genuinely ductus-only (trunk-based commits, the retired repo name, the `runtime/` tag loop, the agent registry, cargo/rustup gotchas, primitive wiring sites). §recommendations was promoted to the constitution on 2026-08-17 as the first instance and is the model: canonical text in the constitution, a contributor-side mirror in `AGENTS.md` pointing at it. The rest needs a spec — this is a governed artifact that ships to every adopter, and promoting a dozen entries is not a one-pass edit. Prompted by the operator asking whether the recommendation rule belonged in the constitution.
- [ ] convention: criterion drift is structurally invisible on any spec that sits `in-progress` indefinitely. `check-artifacts`' criterion-path-existence family is scoped to `done` specs by design (a mid-implementation criterion may legitimately name a path not yet created), but a spec gated on an operator-run criterion — 048 on AC10 — never reaches `done`, so its criteria are never examined and drift accumulates for exactly the specs that sit longest. Both defects this review found in 048's criteria (AC16 ticked-and-false, AC11 a latent would-fire) were invisible to every automated check while the spec stayed open, and surfaced only from reading the criteria against the tree by hand. Worth considering whether the family should also examine an `in-progress` spec whose criterion is already ticked — a ticked criterion is a completed claim regardless of the parent spec's status. — `runtime/src/primitives/check_artifacts.rs` (captured during review of 048-govern-acquired-runtime)
