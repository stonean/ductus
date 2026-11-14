---
next-task: 31
---

# 062 — Concurrent session targets Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 29. Analyze the corrections spec 079 made

`check-review-gate` holds only on the analysis, so no review is owed. The pre-done verification ran first (2026-11-14): every criterion holds or is annotated, and no `spec.md` edit is needed. It owes `/ductus:analyze` and the completion gate. `## Applicable Rules` cites `BE-RACE-001` and `BE-RACE-002`, so step 14 reads their verdicts; both fired as `addressed` in the 2026-11-14 judging.

The owner deferred 062's `BE-INPUT-004` finding (2026-11-14) to 022's task 34 (`5973924b`): the design sanitizes the identity to a slug and states no canonical containment, the gap the owner ruled a finding on 045, and the *Path arguments* section of 022's `plan.md` leaves the session files out of the sites that open a path. The plan now names that task (§Storage) and carries the owner-approved rationale for "Command files". It owes one re-judge of all 310 rules and the step-15 items after those two edits, with judge A briefed that `BE-INPUT-004` is a MUST finding deferred to 022's task 34, then the record: "Command files" fixed and `BE-INPUT-004` deferred, `decided-by` the owner. A finding that re-judge raises on text no edit touched goes to the owner, not to another run (owner, 2026-11-14).

A design change beyond what the code does, a blocking deferral, `done` and a push carrying another session's commits are the owner's to decide (2026-11-10).

- **Done when**: an analysis is recorded on this spec, and `/ductus:implement` has returned it to `done`.
