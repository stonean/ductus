---
next-task: 16
---

# 030 — Cross-Service References Tasks

Tasks derived from the [plan](plan.md). Complete in order. Tests are first-class — each implementation task carries its own verification.

## 14. Review and analyze the corrections spec 079 made

Spec 079 task 10 corrected `data-model.md`, scenario `referenced-service-spec-root`, AC10 and the plan, which stated a markdown-only path; a scenario or data-model edit stales the review. It owes `/ductus:review`, then `/ductus:analyze`, then the completion gate, which run once no box in this file is open (spec 079).

The review is recorded at `8339e0f6`. It fixed the broken contract it found as the owner chose (2026-11-07): `data-model.md` and `/link` called a repo under two aliases a registry-validation finding while `Services::duplicate_repos` had no caller, and `resolve-references` now returns `duplicate-repos`, which `/analyze` step 13 reports. Rejected then: the contract made to match the code, with the dead helper removed, which leaves a hand-edited duplicate unreported; and a deferral to a new task here. The analysis that followed deferred three findings to task 15.

- **Done when**: a review and an analysis are recorded on this spec, and `/ductus:implement` has returned it to `done`.
