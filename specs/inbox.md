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
- [ ] convention: 026's plan.md Affected Files still lists scripts/audit/registry-equivalence.sh, deleted when Family 3 was retired by 043 — so compute-review-scope resolves a plan-affected set that is larger than the real modified set and wins the larger-of rule, scoping the review to files the change never touched while omitting the ones it did. — `specs/026-framework-self-audit/plan.md` (captured during review of 026-framework-self-audit)
- [ ] bug: gen-spec-deps.sh corrupts YAML block-list `dependencies:` frontmatter into invalid YAML while reporting success — given a `dependencies:` key whose value is an indented block-list item, it rewrites the key to `dependencies: []` and leaves the orphaned list item beneath it, then prints `Updated <path>` and exits 0. Every spec in this repo uses the inline flow form, which is why it has never surfaced here; an adopter who hand-writes block style has the file silently corrupted on commit by the pre-commit hook. Found while building Family 22's fixture. — `.ductus/scripts/gen-spec-deps.sh` (captured during review of 026-framework-self-audit)
