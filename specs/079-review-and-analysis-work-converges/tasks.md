---
next-task: 39
---

# 079 — Review and analysis work converges Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 37. Measure what stored rule verdicts save when a sweep reopens `done` specs (B3)

- [ ] The owner chose to record this here on 2026-11-13 (B3): corpus-wide sweeps reopen `done` specs in bulk, and each reopened spec's next analysis judges every rule again unless its stored verdicts are current; whether that re-judging changes any verdict could not be measured, since no analysis had yet stored verdicts across a sweep
- [ ] At the first corpus-wide sweep that reopens `done` specs whose `analysis.md` stores `rule-verdicts`, measure for each sweep-started reopening how many re-judged verdicts differ from the stored ones, stating the population (which reopenings, by `measure-reopenings`' attribution), the denominator (rules judged) and the method
- [ ] Put the figure to the owner with what it implies for reusing verdicts across a sweep-started reopening, and record the decision

- **Done when**: the measurement is recorded in 079's artifacts with its method, population and units, and the owner's decision on it is recorded beside it

## 38. The workflows' steps get a declared local command (owner decision, 2026-11-13)

- [ ] A script under `scripts/` runs every `run:` step of the workflow files it is given, each in `bash --noprofile --norc -eo pipefail` from the repository root, and records PASS or FAIL from the step's own exit status, continuing past a failure. A step carrying an `if:` is reported SKIP by name. It prints each workflow's count of steps found, run and skipped, and exits non-zero on any FAIL, and also when a workflow yields no step, so a parse that found nothing never reads as a pass. Found when two sessions ran `framework-checks.yml` and `generators.yml` with a hand-rolled scratchpad runner, which §grounding forbids reporting as the project's command
- [ ] A test on a fixture workflow holds a passing step, a failing step that reads FAIL, an `if:` step that reads SKIP, and a workflow with no step that fails the run; each case fails with its behavior removed. The test runs in the workflow that runs the script tests
- [ ] AGENTS.md's local-checks rule names the command where it says to run `framework-checks.yml`'s and `generators.yml`'s steps

- **Done when**: Running the command on `framework-checks.yml` and `generators.yml` at a clean HEAD reports every step PASS or SKIP with the counts per workflow, its test passes and fails under each case's mutant, and AGENTS.md's local-checks rule names it.
