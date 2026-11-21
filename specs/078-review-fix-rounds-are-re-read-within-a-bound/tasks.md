---
next-task: 21
---

# 078 — Review fix rounds are re-read within a bound Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 15. Close 078's reopening

`check-review-gate` holds only on the analysis, so no review is owed. The pre-done verification ran first (2026-11-14): every criterion holds and no `spec.md` edit is needed; `data-model.md` now names `fix-rounds`' `excluded` field, and 079's task 28 was completed and pruned (`a70ff74c`), so nothing waits on it. It owes `/ductus:analyze`, each live finding dispositioned, and the completion gate. `## Applicable Rules` cites `QUAL-CLAIM-001`, exempt as a code-pattern rule, and `CFG-ENV-001`, `-002` and `-008`, so step 14 reads their verdicts. The owner's stance for this closing (2026-11-04) stands: fix mechanical findings in the run; waive an observation about machinery outside 078's code, with its reason; defer nothing into 078.

- **Done when**: `analysis.md` is current with no live hard-fail or blocking finding, so `/ductus:implement`'s completion gate can run

## 20. AC16 and AC17: a re-read raises only reachable cases, and the budget stop quotes the runtime's figures

- [x] Update `plan.md`'s design record for AC16 and AC17 first. An adopter's review on 0.60.0 ran six fix rounds, four re-reads and two diff checks, over about four hours; each later re-read's first finding was in the previous round's own fix, the contrived cases were NOTEs the bar already names ("a contrived input"), and the operator approved four more re-reads, each asked as "one more" with no count or time. The adopter's own rule that fixed every NOTE made it worse, and is not ductus's to fix.
- [x] AC16: the severity bar in `brief-re-read`'s brief (`runtime/src/primitives/brief_re_read.rs`) and this spec's Behavior bullet on it require a trigger shown possible in the project for any severity, matching 020's AC18 wording for the passes; a constructed case is no finding.
- [x] AC17: `start-fix-rounds` records the loop's start in the round directory; `brief-re-read` returns `rounds`, `re-reads-with-findings` and the elapsed time since that start, counted from the round directory, never from the host. Step 12 of `framework/commands/review.md` and `framework/reference/review.md` §Fix-round loop's budget bullet quote them at the stop. Fixture: three rounds briefed, with the figures asserted, failing with the counting removed. Correct this spec's Behavior §The budget ("where the host reports it, the re-reads' cost") and its data model's `start-fix-rounds` and `brief-re-read` sections.
- [x] 067's task 44 also corrects this spec's Resolved Question on which fixes the loop covers; land the two without overwriting each other.

- **Done when**: The brief's bar states the trigger requirement, the fixture proves the three figures both ways, and step 12, the reference, this spec's Behavior and data model quote the runtime's figures.
