---
spec: 000-slash-commands
scenario: analyze-reads-the-layout-derived-command-set
last-run: 2026-10-07T00:43:43Z
reviewed-against: 974794278dbfaa8ceef9dbbbc8041268d90c5fb1
diff-base: e09b6f670827ccf1081edc71d3ddcd1903f5001c
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 28
skipped-passes: []
reviewed-digest:
  scenarios/analyze-reads-the-layout-derived-command-set.md: 5e69626c6c04595004bd61f6fccb854e1615f4f6d91699ed7049bbe75f14dff0
  scenarios/clarify-one-at-a-time.md: 74531377a3ba6862619c2a4aaf861084da5b53d69ac09a39f28740176f5c078d
  scenarios/command-autocomplete-summary.md: de4ef40b8d2a8587be508d333d502198cae4c312639b282ce363d8bb7253fa66
  scenarios/criterion-route-after-draft.md: b08bf71669de7980442f8246be334fac4440d43c1920eca95c770878f5c45386
  scenarios/dashboard-dependencies-column.md: e471757b1a1f7167129935144362f08c47c5da2dea585405a85f9ce2bcbc8f67
  scenarios/implement-skips-planned-prompt.md: adeb014c4192e84543d733fcf27f12907c2cada0d4950af9782a6168a6fac1c3
  scenarios/scenario-without-task-visibility.md: 912b16e99355b5fa17a6fd86fc4e640ffbe91375cf95394618c8954136e5960d
  scenarios/target-argument-parsing.md: b91e1ec533ba1730f5b3760219ef3817c6238298ae8c351670a6edc9b6bb6db3
  scenarios/target-clear-flag.md: b7dd0b1df765a600403715a0012afacc9c13749122cd15d7d56ad2b69252eb52
  scenarios/validate-fix-mode.md: ce87d6a7473de97a20fb7cf4120a8a6e8bce588db355e7f1f39cd3baf40f447f
  scenarios/validation-gates.md: 07dc0f7d1d5c494503cca763dbfcfd75e497395621cf43d0b20b73b0048ef934
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 000-slash-commands

## Summary

All five passes ran over task 19's change, now stamped against 97479427, the commit that carries it; 0 MUST, 0 SHOULD, 0 low-confidence, no observations; not blocking. Reviewed with `--since=e09b6f67`, the commit before this work: the default window opens at 000's 2026-08 reopen and holds hundreds of files of other specs' already-reviewed history. Read: `framework/commands/analyze.md`, the `runtime/CHANGELOG.md` entry for spec 000, and 000's `plan.md`, `tasks.md` and `scenarios/analyze-reads-the-layout-derived-command-set.md`. Not counted though in scope: the generated copies `.claude/commands/ductus/analyze.md` and `.pi/prompts/ductus-analyze.md`, held identical to their source by `scripts/gen-claude-commands.sh --check`; 022's runtime, script and spec files, which share the commit and are reviewed under 022; and the plan's original `commands/*.md` rows, paths that no longer exist since the command sources moved under `framework/commands/`. No other document repeats the claude-style read path the change replaced.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
