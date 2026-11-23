---
next-task: 44
---

# 079 — Review and analysis work converges Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 37. Measure what stored rule verdicts save when a sweep reopens `done` specs (B3)

- [x] The owner chose to record this here on 2026-11-13 (B3): corpus-wide sweeps reopen `done` specs in bulk, and each reopened spec's next analysis judges every rule again unless its stored verdicts are current; whether that re-judging changes any verdict could not be measured, since no analysis had yet stored verdicts across a sweep
- [x] Measure how many re-judged verdicts differ from the stored ones over the committed re-check pairs whose later run reused no verdict, stating the population, the denominator (rules judged) and the method. The owner chose this population on 2026-11-16 in place of the first corpus-wide sweep, since none had reopened `done` specs that store verdicts; the figure is in `research.md` under its task 37 heading
- [x] Put the figure to the owner with what it implies for reusing verdicts across a sweep-started reopening, and record the decision

- **Done when**: the measurement is recorded in 079's artifacts with its method, population and units, and the owner's decision on it is recorded beside it

## 38. The workflows' steps get a declared local command (owner decision, 2026-11-13)

- [x] A script under `scripts/` runs every `run:` step of the workflow files it is given, each in `bash --noprofile --norc -eo pipefail` from the repository root, and records PASS or FAIL from the step's own exit status, continuing past a failure. A step carrying an `if:` is reported SKIP by name. It prints each workflow's count of steps found, run and skipped, and exits non-zero on any FAIL, and also when a workflow yields no step, so a parse that found nothing never reads as a pass. Found when two sessions ran `framework-checks.yml` and `generators.yml` with a hand-rolled scratchpad runner, which §grounding forbids reporting as the project's command
- [x] A test on a fixture workflow holds a passing step, a failing step that reads FAIL, an `if:` step that reads SKIP, and a workflow with no step that fails the run; each case fails with its behavior removed. The test runs in the workflow that runs the script tests
- [x] AGENTS.md's local-checks rule names the command where it says to run `framework-checks.yml`'s and `generators.yml`'s steps, and its `runtime/` list runs step (u)'s mutation-anchor check before the commit: a comment 063's review moved in `host_config.rs` broke mutation M9's anchor, which no runtime check caught until the workflow steps ran after committing (`99c6affe`)

- **Done when**: Running the command on `framework-checks.yml` and `generators.yml` at a clean HEAD reports every step PASS or SKIP with the counts per workflow, its test passes and fails under each case's mutant, and AGENTS.md's local-checks rule names it.

## 39. The excluded section lists paths only

- [x] Implement the behavior described in `scenarios/the-excluded-section-lists-paths-only.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested

## 40. The owner's decisions of 2026-11-19 on the excluded section are recorded where this spec keeps decisions

- [x] The section lists each excluded path alone; recording only a count and a pointer was weighed and set aside because AC3 has the record name every path
- [x] No existing record is rewritten: each shrinks at its spec's next review, since rewriting `review.md` would stale every spec's analysis
- [x] Spec 081's AC13 lost its clause that a credited path is listed under a spec, and 081 was reopened for it

- **Done when**: the spec's Resolved Questions state each decision with its reason and its measured effect

## 41. A held record that read its scope moves the window's start to its `reviewed-against` (the agent's recommendation, adopted by the operator, 2026-11-23)

- [x] `last_review_commit` (`runtime/src/primitives/compute_review_scope.rs`) takes a record `holds_done_until_reviewed` holds, with no skipped pass, no budget stop and `unread` present and empty, as ending the window at its `reviewed-against`, no earlier than the reopen's base; a clean record keeps the commit finishing it, and a held one stating no `unread`, or a non-empty one, finishes nothing
- [x] `BaseSource` (`runtime/src/schema/primitives.rs`) gains `held-review` and `reopen-past-partial-review`, the second when every run since the reopen was passed over for not reading its scope; `COMPUTE_REVIEW_SCOPE` (`runtime/src/schema/descriptions.rs`), the module doc comments and `framework/reference/review.md` §Window and coverage state the rule; `runtime/CHANGELOG.md` records it as breaking, since `base-source` gains values
- [x] Fixture tests: a held record, one per hold (a SHOULD, an open finding, a contract break, a criterion not met, the open mark, an unwaived round), stating `unread: []`, reads `held-review`, and its next scope holds a fix committed after `reviewed-against`, its `open-findings` and a code path changed after it, but not an older path the run read; with `unread` absent or non-empty, or a skipped pass, it reads `reopen-past-partial-review`; each case fails with its behavior removed, and the window mutants in `scripts/mutation/scope-attribution.py` are updated and killed
- [x] An adopter-shaped fixture reproduces the report first: a reopening whose first review is held by the open mark, a SHOULD, a deferred contract break and a criterion not met, followed by task commits, reads `base-source: reopen` before the change and `held-review` after it

- **Done when**: AC5's text holds against the tree: `compute-review-scope` on each fixture reads the start AC5 names, the tests and mutants pass, `framework/reference/review.md` and the description state the rule, and `runtime/CHANGELOG.md` has its entry.

## 42. What a review left unread joins the next review's scope (AC30; the agent's recommendations, adopted by the operator, 2026-11-23)

- [x] `compute` (`runtime/src/primitives/compute_review_scope.rs`) holds in scope, wherever the window starts and out of `excluded`, each entry the record at `HEAD` lists in `unread` and each the record the window starts at lists there (`ReviewEnd::Finished` carries its record's `unread`): an entry on disk joins as the path it is, a plan entry as the entry it is, and one gone or leaving the project does not; none joins `owed_by`, so `review-owed` reads as before
- [x] `COMPUTE_REVIEW_SCOPE` (`runtime/src/schema/descriptions.rs`), the module doc comment, `framework/reference/review.md` §Window and coverage and its installed copy state it; `runtime/CHANGELOG.md`'s Unreleased entry gains it
- [x] Fixture tests, each failing with its behavior removed: a clean record listing an older path in `unread` holds it in the next scope with `review-owed` false; one passed over by a later partial run still has the start record's entry held; an entry gone from disk, one leaving the project and a plan pattern each behave as stated; new mutants in `scripts/mutation/scope-attribution.py` killed

- **Done when**: AC30's text holds against the tree: each fixture's scope holds the entries AC30 names and its `review-owed` reads as before, the tests and mutants pass, and the description, the reference and the changelog state it.

## 43. write-review's denominator carries what the record the window starts at leaves, as the run's scope did

- [x] Task 41's held-start carry left `resolve_scope` (`runtime/src/primitives/write_review.rs`) behind: it resolves the scope with `since` set to the run's own diff base, which reads `base-source: since` and carries nothing from the record the window starts at, so after a held start and a later partial run the record's `scope` misses the carried open findings the passes read, an `unread` entry naming one is refused, and the moved-scope guard does not watch them. `resolve_scope` takes the default scope when it resolves to the run's own diff base, and `since` otherwise
- [x] A test: a held start record's open finding, under a later partial run, is in the scope `write-review` derives, an `unread` entry naming it is accepted, and the case fails with the behavior removed; a mutant in `scripts/mutation/scope-attribution.py` killed

- **Done when**: `write-review`'s derived scope equals `compute-review-scope`'s for the run's own window on the fixture, and its test and mutant pass.
