---
next-task: 18
---

# 045 — Decision-State Drift Detection Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 17. Record the analysis once 067's symlink-screen task exists

`check-review-gate` holds only on the analysis, so no review is owed. The pre-done verification ran first (2026-11-14): all 19 criteria hold and no `spec.md` edit is needed. The analysis's judging (2026-11-14) found one gating finding, `BE-INPUT-004` (MUST): `lexical_in_project` in `runtime/src/primitives/check_artifacts.rs` refuses a `..` that leaves the project, then `exists()` follows a symbolic link out of it, which the sibling-link family already refuses. The owner chose (2026-11-14) to defer that gap to 067's task 33 (`112d96b5`), and `plan.md`'s §What the families read now states the delivered lexical screen (067's task 30, `05838d8c`) and names task 33. The stored deferral to task 27 releases nothing now and is superseded by the new one. That plan edit changes a design subject, so all 310 rules and the step-15 items are judged once more before the record; a finding that run raises on text the edit did not touch goes to the owner, not to another run (owner, 2026-11-14). The first judging's other findings: the uncited prior-art claim, waived by rule; the three body-path advisories, under their stored waivers; this file's stale ordering note, fixed by rule (deleted).

- **Done when**: the analysis is recorded with `BE-INPUT-004` deferred to the named 067 task, and the owner has approved `done`.
