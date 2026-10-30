# 052 — Spec consolidation Tasks

Tasks derived from the [plan](plan.md). Complete in order.

<!-- Each task should be small enough to implement and verify independently.
     Mark subtasks as they are completed. Every task MUST close with a
     `- **Done when**: …` line stating its completion condition — the
     tooling reads this exact form to confirm the task is fully specified.
     Example:

## 1. Create sessions table migration

- [ ] Write SQL migration for `sessions` table
- [ ] Run migration and verify schema

- **Done when**: the migration applies cleanly and `sessions` matches the data model.

## 2. Implement session store

- [ ] Create `shared/auth/session.go` with Create, Get, Delete methods
- [ ] Write store integration tests against real PostgreSQL

- **Done when**: all store methods are covered by passing integration tests.

## 3. Update README link to migration guide

- [ ] Edit `README.md` to point at the new path

- **Done when**: the README link resolves to the new path.

-->

## 1. Re-review after 048's review found the partial's record misstated what it left unread

- [x] `review.md` (a21e707a) says its 7 unread files are each unchanged since the 2026-09-13 full review, but all 7 changed since (`git log 8c01d6f5..6940509c`), and 446fbf2d rewrote `framework/commands/consolidate.md`'s Purpose, the surface AC25 names: re-review, reading `consolidate.md` and the other changed files, and describe anything left unread truthfully
- [x] `spec.md:65` says prune destroys `tasks.md`, which the project classes as ephemeral; since 041 prune also removes `plan.md` sections outside the design record once their durable pieces move home — correct the sentence as 446fbf2d corrected `consolidate.md`'s copy

- **Done when**: 052's review is recorded over the changed files with a Summary that states truthfully what was and was not read, its analysis is current, and 052 is back at `done`.

## 2. AC7 names a symbolic link under the spec root — 048's second re-read round

- [ ] 048's second re-read round (2026-10-29) found `rewrite-spec-links` passing over a symbolic link under the spec root unlisted, so a pointer in it dangled after `/{project}:consolidate` with `skipped: []`. The owner chose to list it in `skipped` (051's `the-spec-root-is-read-before-the-first-write`, edge case on symbolic links), and AC7 was widened to say so, which reopened 052. Task 3 landed in edd08387, in the same window. The window's in-depth reads are two records: 048's review (2f79a9ee to ced8c7ca, with its re-read rounds) and 051's 2026-10-29 review (edd08387, ea859f7c and 3297fbd6). 3297fbd6, that review's fix round, also edited this spec: the summary line, Motivation and AC7 are bounded to the files the rewrite reads, and AC9 and the content-move scenario are qualified to a link that climbs to the spec root. The scenario edit is what stales 052's review. Owed, in order: review 052's window from its reopen, reading only what those two records did not, per AGENTS.md; `/ductus:analyze 052`; the completion gate, verifying AC7 and AC9 against the tree

- **Done when**: 052's review and analysis records are current, AC7 holds against the tree, and the completion gate passes

## 3. Implement scenario: the content move and the consolidation commit together

- [x] Implement the behavior described in `scenarios/the-content-move-and-the-consolidation-commit-together.md`

- **Done when**: `consolidate.md` states the order and both checkpoint routes, steps 2 and 7 report what the scenario names, AC9 states what the command writes into the target, and `runtime/tests/consolidate_hook.rs` drives the move-then-consolidate and copy-checkpoint flows through the shipped pre-commit hook and fails against the self-cycle
