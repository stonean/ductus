---
next-task: 18
---

# 045 — Decision-State Drift Detection Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 17. Record the analysis once 067's symlink-screen task exists

`check-review-gate` holds only on the analysis, so no review is owed. The pre-done verification ran first (2026-11-14): all 19 criteria hold and no `spec.md` edit is needed. The analysis's judging (2026-11-14) found one gating finding, `BE-INPUT-004` (MUST): `lexical_in_project` in `runtime/src/primitives/check_artifacts.rs` refuses a `..` that leaves the project, then `exists()` follows a symbolic link out of it, which the sibling-link family already refuses. The owner chose (2026-11-14) to defer that gap to a new 067 task, which session `3fd29c6a` adds, and to correct `plan.md`'s §What the families read sentence, which still says the probe "is not screened" and that 067's task 27 (delivered as task 30, `05838d8c`) adds the screen, to state the delivered lexical screen and name the new task. The stored deferral to task 27 releases nothing now and is superseded by the new one. The record's other findings: the uncited prior-art claim, waived by rule; the three body-path advisories, under their stored waivers; this file's stale ordering note, fixed by rule (deleted).

- **Done when**: the analysis is recorded with `BE-INPUT-004` deferred to the named 067 task, and the owner has approved `done`.
