---
next-task: 16
---

# 054 — Remove supersession Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 13. Close the 2026-10-29 reopen

Reopened done → in-progress by 3297fbd6, the fix round of 051's 2026-10-29 review, which bounded §Motivation's claim that consolidation re-points every inbound pointer: it now reads "re-points every inbound pointer it reads first, lest it dangle". That sentence is the whole reopen. 051's review read it in depth: the sweep was that review's fix round, the first round was re-read independently, and "lest it dangle" came in the second, prose-only round. `check-review-gate` then held 054's review current — spec.md is outside the review digest — and only the analysis stale. Since then, commits no spec claims in its window make `compute-review-scope` report `review-owed: true` (at `7c341fa6`), so `check-review-gate` decides whether a review comes first, and its ticked criteria are verified against the tree before either run (spec 079 task 26, owner, 2026-11-09). They were verified at `ce217f0a` (task 14), so the review `check-review-gate` names comes next, then the analysis.

- **Done when**: 054's analysis is current, the completion gate passes, and 054 is back at `done`
