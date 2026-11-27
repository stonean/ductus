---
next-task: 58
---

# 050 — Constitution Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 55. Every spec's spent task sections are pruned (AC27, owner decision 2026-11-27)

- [ ] Run `/ductus:prune --all`, keeping pending tasks, so no checked task's text survives in any spec: 612 checked boxes across the 83 `tasks.md` files at `cfb0bfd9`, by `grep -c '^\s*- \[x\]'` over each
- [x] Message the other session before pruning a spec it is working, 066 among them

Only `tasks.md` is pruned: `/ductus:prune --all`'s `plan.md` and `review.md` reductions are not applied (the agent's recommendation, adopted by the operator, 2026-11-27). 066's prune waits until the other lane reports 066 `done`, its request of 2026-11-27, since a prune while it is `in-progress` stales its analysis.

- **Done when**: no `specs/*/tasks.md` holds a checked task section, every pending task is kept, each pruned file carries a `next-task:` counter past its highest number, and `scripts/audit/run-all.sh` passes after the commit

## 56. Each ticked criterion with a Superseded note states its current requirement (AC27, owner decision 2026-11-27)

- [x] Rewrite each ticked criterion carrying a Superseded note to the requirement that holds now, removing the note, or delete it where its behaviour is gone: 179 criteria in 48 specs at `cfb0bfd9`, the ticked `AC{n}:` lines of `specs/*/spec.md` matching `uperseded`
- [x] Ask the owner, one criterion at a time, where the current requirement cannot be read from the note and its source; message the other session before editing a spec it is working

The set is every ticked criterion carrying such a note in any wording, not only `uperseded`: 59 more candidates in 31 specs at `b92e7d20` read "Narrowed by", "since retired", "no longer holds", "Changed by" or "replaced by" (the agent's recommendation, adopted by the operator, 2026-11-27). 066's are the other lane's, which rewrites them before 066's review. The links back to 020 in 024's AC2, AC3 and AC6, 034's, 037's, 038's and 039's AC7, 068's AC10, 069's AC11 and 070's AC9 discharge 020's `cross-spec-impact:`, so a rewrite keeps them.

- **Done when**: no ticked criterion in `specs/*/spec.md` carries a Superseded note, each rewritten criterion holds what its spec requires now, and `check-owed-work` reads each touched spec as owing what it owed before

## 57. Every other note of a superseded requirement states what holds (AC27; the agent's recommendation, adopted by the operator, 2026-11-27)

- [x] Rewrite each note of a superseded requirement outside a criterion (in a spec body, a Resolved Question, a scenario, a plan, a data model or a research file) to what holds now, or remove it where nothing does, in the same per-spec pass as task 56: about 250 lines in about 60 specs at `b92e7d20`, matched by task 56's wording
- [x] Keep a blockquoted signpost that points at the spec which changed this one; where it notes a superseded requirement, rewrite the requirement and keep any link that discharges a `cross-spec-impact:`, the 066 signposts in 000, 020, 079 and 081 among them; message the other session before editing a spec it is working

066's eight candidates are the other lane's, which judges them before 066's review record (the agent's recommendation, adopted by the operator, 2026-11-27, relayed by the other lane). The matcher never reads zero, since the words also name a spec's own subject, a record of what happened or a signpost: what remains is reported as the classified `not-a-note` lines and signposts, with the matcher stated. `AGENTS.md`'s *ever true* bullet stops annotating a never-true criterion and points at §implement-phase (the agent's recommendation, adopted by the operator, 2026-11-27).

- **Done when**: no spec artifact but `review.md` and `analysis.md` notes a superseded requirement, every `cross-spec-impact:` back-link the 2026-11-27 baseline found still stands, and `check-artifacts` reports no finding the baseline did not
