# 019 — Config-Persisted Decisions Tasks

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

## 1. An unknown config key is named (CFG-ENV-012)

- [ ] Implement the behavior described in `scenarios/an-unknown-config-key-is-named.md`
- [ ] The runtime reports unknown keys in ductus-owned sections of .ductus/config.toml and .ductus/session.toml by name, section, and file, without failing the run
- [ ] Recognized keys are declared in one place shared by every reader of a section
- [ ] A test fails if a mistyped key in a ductus-owned section is silently dropped
- [ ] 019's data-model and framework/bootstrap/ductus.md §Project Configuration state the behavior

- **Done when**: A mistyped key in a ductus-owned config section is reported by name on the run that reads it, and 019 is re-reviewed back to done.
