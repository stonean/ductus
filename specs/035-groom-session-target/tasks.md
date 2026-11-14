---
next-task: 8
---

# 035 — Groom sets the session target from the routed item Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 7. Review and analyze the correction spec 079 made

Spec 079 task 18 corrected scenario `confirmation-names-reopen`, which said a spec-edit route never reopens; a scenario edit stales the review. It owes `/ductus:review`, then `/ductus:analyze`, then the completion gate, which run once no box in this file is open (spec 079).

- **Done when**: a review and an analysis are recorded on this spec, and `/ductus:implement` has returned it to `done`.
