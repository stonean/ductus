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

- [ ] 048's second re-read round (2026-10-29) found `rewrite-spec-links` passing over a symbolic link under the spec root unlisted, so a pointer in it dangled after `/{project}:consolidate` with `skipped: []`. The owner chose to list it in `skipped` (051's `the-spec-root-is-read-before-the-first-write`, edge case on symbolic links), and AC7 was widened to say so, which reopened 052. Owed, in order: review 052's window from its reopen, leaning on 048's review as the in-depth one per AGENTS.md; `/ductus:analyze` 052; the completion gate, verifying AC7 against the tree

- **Done when**: 052's review and analysis records are current, AC7 holds against the tree, and the completion gate passes
