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
- [ ] bug: `scripts/audit/broken-relative-links.sh` (Family 26) resolves a root-absolute link target against the filesystem root, not the repo root — `os.path.join(here, '/specs/x.md')` discards `here` entirely, exactly the defect just fixed in `check-corpus-links`. No such link exists in this corpus today, so the family is not currently wrong about anything; it would be the moment one is written, and it would be wrong in the dangerous direction on a machine that happens to hold that absolute path. — `scripts/audit/broken-relative-links.sh` (captured during review of 022-deterministic-runtime)
- [ ] other: Family 26 and the new `check-corpus-links` primitive now perform substantially the same check over overlapping subjects — the family covers the whole repository including maintainer-only files, the primitive covers the spec corpus and is what adopters actually run. The scenario deliberately left delegation undecided ('Family 26 is not necessarily retired by this'), and the Family 30 shape — logic in the runtime, script as entry point — is the obvious candidate. Deciding it would also collapse the divergence recorded in the observation above, since there would be one implementation to fix rather than two to keep in step. — `scripts/audit/broken-relative-links.sh` (captured during review of 022-deterministic-runtime)
- [ ] convention: Family 33 recognizes a documented command by the backticked token `/name`, so a command that only ever appeared inside a wider code span — `/specify --supersedes`, say — would be reported as undocumented. The failure is loud and in the safe direction (a false finding a maintainer resolves by adding the bare token), but it is an undocumented constraint on how the README may write a command name, and nothing states it where an author would meet it. — `scripts/audit/readme-command-parity.sh` (captured during review of 026-framework-self-audit)
- [ ] convention: `/{project}:consolidate` never re-targets the session, so an operator whose session pointed at the removed spec is left pointing at a directory that no longer exists until they run `/{project}:target`. This is deliberate and the command reports it — consolidating a spec asserts its content belongs with the target, not that the operator's next work does — but `/{project}:fold` does re-target in the same situation, and the two commands giving opposite answers to the same stranded-session problem is worth a decision recorded somewhere, rather than two commands that each look locally reasonable. — `framework/commands/consolidate.md` (captured during review of 052-spec-supersession-and-consolidation)
