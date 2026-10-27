# 058 — Findings route at discovery Tasks

Tasks derived from the [plan](plan.md). Complete in order.

<!-- Each task should be small enough to implement and verify independently.
     Mark subtasks as they are completed. Every task MUST close with a
     `- **Done when**: …` line stating its completion condition — the
     tooling reads this exact form to confirm the task is fully specified.

     A task body may also carry working notes — what the next session needs
     to resume it: the mechanics, the order, what is left on disk — placed
     as §tasks-phase says. Write them as prose on the pending task they
     concern, so they go when the task is pruned. Put an ordering constraint
     on the task that must wait, never above the first task or under a
     heading of its own, which outlive every task; and never write a note as
     a checkbox, which would count toward the task's completion. Anything
     that must outlast the task belongs in its durable home first. Example:

## 1. Create sessions table migration

- [ ] Write SQL migration for `sessions` table
- [ ] Run migration and verify schema

- **Done when**: the migration applies cleanly and `sessions` matches the data model.

## 2. Implement session store

Blocked on task 1: the store tests run against the migrated schema.

- [ ] Create `shared/auth/session.go` with Create, Get, Delete methods
- [ ] Write store integration tests against real PostgreSQL

- **Done when**: all store methods are covered by passing integration tests.

## 3. Update README link to migration guide

- [ ] Edit `README.md` to point at the new path

- **Done when**: the README link resolves to the new path.

-->

## 1. Findings cost in proportion to risk

- [ ] Implement the behavior described in `scenarios/findings-cost-in-proportion-to-risk.md`

Start with `/ductus:target 058-findings-route-at-discovery/findings-cost-in-proportion-to-risk` and `/ductus:clarify`; the owner pre-authorized the scenario's `Recommended:` answers (2026-10-26). A record's `dispositions:` map does not split its counts by tier, but the gating findings already gate through other checks — an analyze hard-fail or blocking finding keeps `blocking: true` until the re-check no longer produces it, and review MUST and SHOULD violations have their own counts — so read `check-review-gate`'s undispositioned check against that before changing it, and keep every other gate check as is. The constitution text to change is §brownfield-inbox Finding dispositions (the gate bullet) and the §implement-phase outstanding-SHOULD bullet, within the ceiling in `scripts/lint-governing-size.sh` (53,813 bytes, the file's exact size), so it removes as many bytes as it adds; `AGENTS.md` (about 3 KB under its 50,000 ceiling) gets the fix-round entry's scope.

- **Done when**: the scenario's described behavior is correctly implemented and tested
