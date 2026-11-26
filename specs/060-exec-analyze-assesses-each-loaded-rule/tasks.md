---
next-task: 16
---

# 060 — Exec analyze assesses each loaded rule Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 14. Rule judging runs from repository scripts (scenario rule-judging-runs-from-repo-scripts)

- [x] Implement the behavior described in `scenarios/rule-judging-runs-from-repo-scripts.md`

- **Done when**: The scenario's described behavior is correctly implemented and tested.

## 15. The scripts judge each scenario's Context and Behavior

Worked once 088 and 089 are `done` and before the release tag (the owner's decision, 2026-11-26). Found in 085's analysis: none of 086 to 089 has a scenario, so no analysis before then depends on it.

- [ ] `scripts/analyze/` briefs the host's judgment of each scenario's Context and Behavior that `framework/reference/analyze.md` §Scenario consistency (advisory) asks for, each scenario read whole, as the step-15 reader's brief does for its items; `check.py` holds that read account to every scenario, and `record.py` records each finding as an Advisory `scenario-consistency` finding
- [ ] `scenarios/rule-judging-runs-from-repo-scripts.md`'s Behavior names that brief among the judgments the scripts brief, and `scripts/tests/test-analyze-scripts.py` fails when it is removed

- **Done when**: an analysis of a spec with scenarios run from `scripts/analyze/` records a judgment of every scenario's Context and Behavior, proven by a test that fails without it
