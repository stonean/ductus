---
next-task: 20
---

# 078 — Review fix rounds are re-read within a bound Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 15. Close 078's reopening

`check-review-gate` holds only on the analysis, so no review is owed. The pre-done verification ran first (2026-11-14): every criterion holds and no `spec.md` edit is needed; `data-model.md` now names `fix-rounds`' `excluded` field, and 079's task 28 was completed and pruned (`a70ff74c`), so nothing waits on it. It owes `/ductus:analyze`, each live finding dispositioned, and the completion gate. `## Applicable Rules` cites `QUAL-CLAIM-001`, exempt as a code-pattern rule, and `CFG-ENV-001`, `-002` and `-008`, so step 14 reads their verdicts. The owner's stance for this closing (2026-11-04) stands: fix mechanical findings in the run; waive an observation about machinery outside 078's code, with its reason; defer nothing into 078.

- **Done when**: `analysis.md` is current with no live hard-fail or blocking finding, so `/ductus:implement`'s completion gate can run
