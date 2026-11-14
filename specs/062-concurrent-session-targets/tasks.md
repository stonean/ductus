---
next-task: 31
---

# 062 — Concurrent session targets Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 29. Analyze the corrections spec 079 made

Spec 079 task 10 annotated AC24 and corrected `spec.md` and `plan.md`, since no markdown-only path remains; no code, scenario or data model of this spec changed in it, but `compute-review-scope` reports `review-owed: true`, with earlier commits naming this spec and an unclaimed commit in its window, so the gate holds a review for it. It owes `/ductus:analyze`, then the completion gate, which run once no box in this file is open (spec 079). `check-review-gate` decides whether a review comes first, and its ticked criteria are verified against the tree before either run (spec 079 task 26, owner, 2026-11-09).

Its criteria were verified at `9ebca7b0` (2026-11-09): none fails, AC15 changed in part, and eight passages in the spec body, scenario and data model are false now. The review's passes and the analysis's rule and grounding steps ran there too. The review read in depth what 062 claims after `3bbee201`, since 023's and 030's reviews read task 10's shared window: 0 MUST and 0 SHOULD, twelve observations, none a broken contract. The analysis found two blocking rule findings — `BE-PAYLOAD-001`, the session tools' lists state no bound, and `BE-REPR-001`, `peers[].last-used` names no representation — and four advisory. The owner delegated closing it as 055 was closed (2026-11-10): fix every observation the disposition rule decides, state the lists' bound by 022 `plan.md`'s `BE-PAYLOAD-001` statement and the timestamp's representation as a real `write-session` result shows it, correct the false passages by tense or fact, and annotate AC1, AC15, AC19 and AC20, all before the review and analysis records; a design change beyond what the code does, a blocking deferral, `done` and a push carrying another session's commits are still asked.

- **Done when**: an analysis is recorded on this spec, and `/ductus:implement` has returned it to `done`.
