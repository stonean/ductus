---
section: "The completion gate"
---

# Criteria-are-verified-in-review

## Context

`/{project}:implement`'s completion gate is the pipeline's only step that judges code against a spec's acceptance criteria: after `/{project}:review` and `/{project}:analyze` have recorded clean runs, it judges each criterion (`verifyCriteria`) and ticks each that holds (`mark-criterion`). Review loads the criteria, but none of its five passes returns a verdict on one, and it reads only its window, the spec's change since its last review (`compute-review-scope`). Analyze reads no code. A ticked criterion survives a reopen, so nothing re-earns it before the gate.

An adopting project reported the cost on 2026-11-18, confirmed against this repository at `0adf54ba`. On a reopened spec with 24 ticked criteria and every task done, review and analyze passed, and the gate then found three criteria that no longer held, each in code older than the reopen and outside the review window. Fixing the code owed another review, whose record stales the analysis, so the step meant to be a switch sent the spec back through both audits. Since this spec's task 9 an untick no longer stales the analysis (`checkbox-state-is-outside-the-analysis-digest`), but a fix still does, and the gate judges every criterion again on each run.

## Behavior

- `/{project}:review` returns a verdict, met or not met, for every acceptance criterion of the spec, wherever its code lies, with its evidence: the path and symbol that implement it and the test that proves it. Code outside the window is read for the verdict and does not join the window.
- A ticked criterion judged not met is a blocking finding, fixed, deferred or waived as any blocking finding is, and review unticks it. An unticked criterion judged met is ticked by review.
- The review record states how many criteria were judged and how many met.
- The completion gate makes no semantic judgment. `check-review-gate` blocks `done` on an unticked criterion as it does on an unchecked task, and after a clean review and a clean analysis the transition writes nothing but the spec's status.

## Edge Cases

- A criterion review cannot judge, for want of code, a test or a fetched source, is reported as not judged and counted apart from met; it is never ticked.
- A fixture spec with a ticked criterion that the code contradicts, in a file outside the window, is caught by review, and the same fixture with the code corrected passes. The check is proven to fail before it is trusted.
- An unticked criterion on an `in-progress` spec whose tasks are all ticked is held by the gate's tick check. Since 079's AC28 a review cannot record while a box in `tasks.md` is open, so no review that ran before the last task can leave one unjudged.

## Open Questions

- A reopening that changes no code owes no review (spec 081), yet under this scenario only a review ticks a criterion. When `/{project}:amend` adds a criterion to a `done` spec, or a correction unticks one, with no code changed, is a review owed while any criterion is unticked, or does something else judge it?

## Resolved Questions

*None yet.*
