---
next-task: 57
---

# 050 — Constitution Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 55. Every spec's spent task sections are pruned (AC27, owner decision 2026-11-27)

- [ ] Run `/ductus:prune --all`, keeping pending tasks, so no checked task's text survives in any spec: 612 checked boxes across the 83 `tasks.md` files at `cfb0bfd9`, by `grep -c '^\s*- \[x\]'` over each
- [x] Message the other session before pruning a spec it is working, 066 among them

Only `tasks.md` is pruned: `/ductus:prune --all`'s `plan.md` and `review.md` reductions are not applied (the agent's recommendation, adopted by the operator, 2026-11-27). 066's prune waits until the other lane reports 066 `done`, its request of 2026-11-27, since a prune while it is `in-progress` stales its analysis.

- **Done when**: no `specs/*/tasks.md` holds a checked task section, every pending task is kept, each pruned file carries a `next-task:` counter past its highest number, and `scripts/audit/run-all.sh` passes after the commit

## 56. Each ticked criterion with a Superseded note states its current requirement (AC27, owner decision 2026-11-27)

- [ ] Rewrite each ticked criterion carrying a Superseded note to the requirement that holds now, removing the note, or delete it where its behaviour is gone: 179 criteria in 48 specs at `cfb0bfd9`, the ticked `AC{n}:` lines of `specs/*/spec.md` matching `uperseded`
- [ ] Ask the owner, one criterion at a time, where the current requirement cannot be read from the note and its source; message the other session before editing a spec it is working

The set is every ticked criterion carrying such a note in any wording, not only `uperseded`: 59 more candidates in 31 specs at `b92e7d20` read "Narrowed by", "since retired", "no longer holds", "Changed by" or "replaced by" (the agent's recommendation, adopted by the operator, 2026-11-27). 066's are the other lane's, which rewrites them before 066's review. The links back to 020 in 024's AC2, AC3 and AC6, 034's, 037's, 038's and 039's AC7, 068's AC10, 069's AC11 and 070's AC9 discharge 020's `cross-spec-impact:`, so a rewrite keeps them.

- **Done when**: no ticked criterion in `specs/*/spec.md` carries a Superseded note, each rewritten criterion holds what its spec requires now, and `check-owed-work` reads each touched spec as owing what it owed before
