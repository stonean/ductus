---
title: "014-reclarify-backedge — tasks"
next-task: 10
---

# 014 — Re-clarify Back-Edge Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 9. Review and analyze the correction spec 079 made

Spec 079 task 10 corrected `scenarios/reopen-after-informal-edits.md`, since `classify-spec-edit` now spares a correction the prompt; a scenario edit stales the review. It owes `/ductus:review`, then `/ductus:analyze`, then the completion gate, which run once no box in this file is open (spec 079).

- **Done when**: a review and an analysis are recorded on this spec, and `/ductus:implement` has returned it to `done`.
