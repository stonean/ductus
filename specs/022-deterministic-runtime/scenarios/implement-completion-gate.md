---
section: "Follow-on scenarios"
---

# Implement-completion-gate

## Context

`/ductus:implement`'s numbered procedure covers the per-task loop, but the entire completion gate exists only as markdown-only prose with no numbered steps and no primitives: the all-tasks-complete tally, the all-acceptance-criteria tally, scenario-task completeness, the `review.last-run`/`review.blocking` gate reads and halt messages, the cross-spec `git diff` filter, and the final in-progress→done `set-status`. The LLM re-walks all of it on every run that reaches completion — the last minutes-long mechanical stretch on the pipeline's hottest command — and it orphans `mark-criterion`, which is fully wired and tested but referenced by zero command prose (the 2026-07-11 coverage review's top finding).

## Behavior

The completion gate is numbered, parseable steps in `/ductus:implement`'s Instructions: `read-tasks` tallies task completion (step 9); `diff-cross-spec` reports the cross-spec impact (step 10); the review gate is the `check-review-gate` primitive (step 11; `coverage-expansion-primitives`), which reads the review record from `review.md` and halts with its canonical blocked message when the spec is unreviewed or blocking or a criterion stands unticked; the final transition invokes `set-status` (in-progress → done, step 13) only after the user-approval gate (step 12). Criteria verification is `/ductus:review`'s criteria step: the typed `verifyCriteria` extension point (`<!-- llm:verifyCriteria -->`) judges whether each criterion is met, and `mark-criterion` flips each verdict.

## Edge Cases

- The cross-spec `git diff` filter is owned by the `diff-cross-spec` primitive (`coverage-expansion-primitives`), invoked at steps 7 and 10.
- A criterion that fails verification is unticked by `/ductus:review` and reported — never batch-marked.
- When `tasks.md` still has unchecked tasks, the gate stops before the review gate (step 9).

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
