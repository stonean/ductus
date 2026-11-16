---
next-task: 38
---

# 051 — Branch-scoped spec numbering Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Spent task sections are pruned per §tasks-phase; git history holds them, and the plan's Technical Decisions hold the decisions they settled.

## 36. Name AC15's check, then analyze

`check-review-gate` holds only on the analysis, so no review is owed. The pre-done verification ran first (2026-11-14): every criterion holds or is annotated except AC15's second clause, whose own annotation says the dogfooded `.githooks/pre-commit` and `scripts/lint-frontmatter.sh` copies of the feature-directory grammar are held by no check. The owner chose (2026-11-14) to defer that check to 067's task 34 (`112d96b5`), which AC15's annotation names before the analysis runs, since a `spec.md` edit after it would stale it. The verification also found `retire-feature`'s description (`runtime/src/schema/descriptions.rs`) saying the pre-done gate blocks on a fold, which AC35 and `check_review_gate.rs` contradict; it is routed to session `3fd29c6a` as a `chore(022)`, so it puts no code in 051's review window.

- **Done when**: AC15's annotation names the 067 task, an analysis is recorded on this spec, and the owner has approved `done`.

## 37. invalidate-review and rewrite-spec-links guard their read-modify-write (BE-TXN-002, owner decision)

- [x] `invalidate-review`'s rewrite of `review.md` and `rewrite-spec-links`' rewrite of each referrer guard the window between reading the file and renaming the result over it with the compare-before-persist helper 041's task 27 adds to `runtime/src/primitives/mod.rs`: the file is read again just before the rename, and the write is refused when it changed since the primitive read it. `rewrite-spec-links` reads every pending file before its first write, so today its window spans the whole sweep; a referrer it refuses is named in the result beside the ones it wrote, so `/{project}:fold` and `/{project}:consolidate` can report a sweep that did not complete. The owner decided on 2026-11-16 to close the BE-TXN-002 class by fixing each owner's primitives rather than waiving them, reopening this spec. Run after 041's task 27 lands
- [x] 051's data model states what a refused write reports for each primitive and its plan names the guard; `INVALIDATE_REVIEW`'s and `REWRITE_SPEC_LINKS`' descriptions in `runtime/src/schema/descriptions.rs` state it
- [x] A test per primitive in which a file changes between its read and its write shows the write refused and the other writer's change kept, each failing under its mutant

- **Done when**: Neither primitive overwrites a change made between its read and its write, and a refused referrer is named in the result, proven by tests their mutants turn red; the whole `cargo test` passes; and 051's next review reads the change.
