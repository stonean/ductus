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

- [x] Implement the behavior described in `scenarios/findings-cost-in-proportion-to-risk.md`

Start with `/ductus:target 058-findings-route-at-discovery/findings-cost-in-proportion-to-risk` and `/ductus:clarify`; the owner pre-authorized the scenario's `Recommended:` answers (2026-10-26). A record's `dispositions:` map does not split its counts by tier, but the gating findings already gate through other checks — an analyze hard-fail or blocking finding keeps `blocking: true` until the re-check no longer produces it, and review MUST and SHOULD violations have their own counts — so read `check-review-gate`'s undispositioned check against that before changing it, and keep every other gate check as is. The constitution text to change is §brownfield-inbox Finding dispositions (the gate bullet) and the §implement-phase outstanding-SHOULD bullet, within the ceiling in `scripts/lint-governing-size.sh` (then the file's exact size), so it removes as many bytes as it adds; `AGENTS.md` (about 3 KB under its 50,000 ceiling) gets the fix-round entry's scope.

- **Done when**: the scenario's described behavior is correctly implemented and tested

## 2. Remove the guidance comment and placeholder from inboxes that kept them

- [ ] Add the later `framework/migrations.toml` entry and its `framework/migrations/{id}.md` procedure as the plan's **The later migration** describes, satisfying AC35
- [ ] Verify it and `inbox-guidance-refresh`'s removal branch with a real adopter bootstrap from a subject adopted before 0.53.0, then tick AC22 and AC35

Waits on `013-text-first-artifacts`' task for `scenarios/the-precheck-ignores-what-the-same-run-wrote.md`, and must not be pushed before it lands: the Frontmatter Migration's precheck refuses a tree a Pre-run Migration dirtied under `specs/`, and no entry after 0.53.0 writes there, so this one would put every adopter past 0.53.0 in front of that refusal, a `--ref=main` adopter on the push. It takes the version of the release that ships it, which may be past 0.60.0 by then; when other entries already carry that version, its id must sort after the newest of theirs. A subject adopted before 0.53.0 crosses both inbox entries, so one bootstrap verifies both; it meets the same refusal until 013's task lands. After this task, 058's review shares `bca9a2fd`'s window with 006's, which `specs/006-bug-workflow/review.md` reviewed in depth at `797799ba`: review only what changed since, per `AGENTS.md`'s entry on one commit reopening several specs.

- **Done when**: a real adopter bootstrap from a pre-0.53.0 subject leaves an inbox with no guidance comment and no placeholder, every other item byte-identical, and a second run writes nothing
