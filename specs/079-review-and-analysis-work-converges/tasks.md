---
next-task: 41
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

- [ ] The section lists each excluded path alone; recording only a count and a pointer was weighed and set aside because AC3 has the record name every path
- [ ] No existing record is rewritten: each shrinks at its spec's next review, since rewriting `review.md` would stale every spec's analysis
- [ ] Spec 081's AC13 lost its clause that a credited path is listed under a spec, and 081 was reopened for it

- **Done when**: the spec's Resolved Questions state each decision with its reason and its measured effect
