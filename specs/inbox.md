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
- [ ] chore: `specs/045-decision-state-drift-detection/spec.md:133` reads "a the derive-don't-ask principle (`017-derive-dont-ask`) violation in miniature" — residue from a token-substitution sweep where the replacement did not fit the sentence. The matching instance in that spec's `plan.md` was repaired in `4070589`; `plan.md` is a design record rather than a durable contract, so it reopened nothing. This one sits in the **spec body** of a `done` spec, and repairing it would trigger the `done → in-progress` back-edge to correct a word that changes no claim, states no requirement, and alters no behaviour. §spec-lifecycle enumerates three mechanical-edit cases — uniform rename sweep, cross-service reference, criterion-label assignment — and a pure typo repair is none of them, so read strictly it reopens the spec. That disproportion is the actual question, and it is a constitution question rather than a judgement call to make by acting: does an edit that changes no claim count as mechanical? `050-constitution` is its home, and its final open question already asks what other constitution work belongs in scope. Blocked on that ruling by choice, not by difficulty — the edit itself is one word.
- [ ] bug: nothing exercises the shipped adopter shell against an adopter-shaped tree, and two silent defects reached adopters through that gap on 2026-08-17 — `config_path_of` resolving a converged adopter to a nonexistent legacy config (`a9188c7`), and the pre-commit hook's `label-criteria` backstop guarding on `PATH` when the runtime lives in the store (`a4c897e`). Both exited 0, both were invisible to `scripts/audit/run-all.sh`, the test suite, and `/ductus:review`, and both were masked in this repo because the copy dogfooded here differs from the copy the manifest installs (see AGENTS.md §Gotchas). The check that would catch the class is behavioral, not a grep: build a fixture with `.ductus/config.toml` carrying a **non-default** `[paths] specs-root`, a `.ductus/bin/ductus` pointer, no `ductus` on `PATH`, and one staged spec with an unlabelled criterion; then assert the generators enumerate the configured root and the hook's backstop actually assigns `AC1:`. A throwaway version of exactly this fixture reproduced both bugs and confirmed both fixes, so the shape is known to work. Home is `026-framework-self-audit` via the back-edge (a new audit family, per AGENTS.md §Workflow on adding a rule to its surface's home spec) rather than a new spec. Not filed as a chore because it is new capability, not maintenance.
