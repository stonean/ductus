---
next-task: 37
---

# 051 — Branch-scoped spec numbering Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Spent task sections are pruned per §tasks-phase; git history holds them, and the plan's Technical Decisions hold the decisions they settled.

## 36. Name AC15's check, then analyze

`check-review-gate` holds only on the analysis, so no review is owed. The pre-done verification ran first (2026-11-14): every criterion holds or is annotated except AC15's second clause, whose own annotation says the dogfooded `.githooks/pre-commit` and `scripts/lint-frontmatter.sh` copies of the feature-directory grammar are held by no check. The owner chose (2026-11-14) to defer that check to a new 067 task, which session `3fd29c6a` adds; AC15's annotation then names it, before the analysis runs, since a `spec.md` edit after it would stale it. The verification also found `retire-feature`'s description (`runtime/src/schema/descriptions.rs`) saying the pre-done gate blocks on a fold, which AC35 and `check_review_gate.rs` contradict; it is routed to session `3fd29c6a` as a `chore(022)`, so it puts no code in 051's review window.

- **Done when**: AC15's annotation names the 067 task, an analysis is recorded on this spec, and the owner has approved `done`.
