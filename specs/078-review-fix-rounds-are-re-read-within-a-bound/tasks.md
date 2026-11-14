---
next-task: 20
---

# 078 — Review fix rounds are re-read within a bound Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 15. Close 078 after 079 is done

This task waited until `079-review-and-analysis-work-converges` was `done` (owner, 2026-11-04), which it reached at `2d36f7ce` (2026-11-09). Read 079's Resolved Questions first: its answer on whether 078's own loop changes, and every rule it changed, govern this task. `analysis.md` predates tasks 10–14 and the second review, so the completion gate holds at `analyze-findings` until this run rewrites it. The review record was current on 2026-11-04 and is not now: at `e144d2a9` `check-review-gate` blocks on `review-stale`, since 079's `0a35facc`, `10302897`, `9129d914` and `64d0be93` changed `data-model.md` and the scenario `a-reviews-own-record-is-no-fix-round` after this spec's review (`d62e72f3`), and `compute-review-scope` reports `review-owed: true` from that base with a scope of 30, so a review comes first. Unless 079 replaced it, the owner's stance for this closing analysis and gate (2026-11-04) applies: fix mechanical findings in the run; waive an observation about machinery outside 078's code, with its reason; defer nothing into 078. If 066 task 17 has edited line 137 of `data-model.md` by then, the review is stale: review only that change, with `--since` set to the commit before it.

It owes `/ductus:analyze` on 078, each live finding dispositioned, which runs once no box in this file is open (spec 079). `check-review-gate` decides whether a review comes first, and its ticked criteria are verified against the tree before either run (spec 079 task 26, owner, 2026-11-09).

The closing review ran at `494a1f3f`, and the analysis after it deferred `CFG-ENV-001` to task 17 (owner, 2026-11-09). Task 17's change had its own review (`5a24d0dc`) and analysis (`acc75e71`), and 079's task 28 waived the miss a simulated closing then read, 4 reviews and 3 analyses (owner, 2026-11-09). The completion gate's criteria verification at `bfd9c6b0` found the two defects tasks 18 and 19 fix (owner, 2026-11-09), so the gate owes a review of that change and an analysis after it, and before `done` 079's task 28 is corrected to the counts a simulated closing then reads (`measure-reopenings --since e7fcde37`), with the added runs' cause.

- **Done when**: `analysis.md` is current with no live hard-fail or blocking finding, so `/ductus:implement`'s completion gate can run
