---
spec: 000-slash-commands
last-run: 2026-10-16T14:49:05Z
reviewed-against: dc1b1050fca756b547b893aa076e48366a4e9066
diff-base: 492783157d73f00878c513256dc762cb1eb9cf90
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 2
scope: 13
skipped-passes: []
reviewed-digest:
  scenarios/analyze-reads-the-layout-derived-command-set.md: 5e69626c6c04595004bd61f6fccb854e1615f4f6d91699ed7049bbe75f14dff0
  scenarios/clarify-one-at-a-time.md: 74531377a3ba6862619c2a4aaf861084da5b53d69ac09a39f28740176f5c078d
  scenarios/command-autocomplete-summary.md: de4ef40b8d2a8587be508d333d502198cae4c312639b282ce363d8bb7253fa66
  scenarios/criterion-route-after-draft.md: b08bf71669de7980442f8246be334fac4440d43c1920eca95c770878f5c45386
  scenarios/dashboard-dependencies-column.md: e471757b1a1f7167129935144362f08c47c5da2dea585405a85f9ce2bcbc8f67
  scenarios/implement-skips-planned-prompt.md: adeb014c4192e84543d733fcf27f12907c2cada0d4950af9782a6168a6fac1c3
  scenarios/scenario-without-task-visibility.md: a28dc9524faff8ff2ebca2c58c3ef125536694b9fe24efd88555703d13d81b34
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

0 MUST, 0 SHOULD at dc1b1050, against all eleven rule files (the 208 rules' Statement and Verification fields). The window changes one scenario's prose and the status line, both read in full; no code changed, so no rule applies. Not examined: the plan's ten Affected Files rows under `commands/` name paths that no longer exist (sources moved to `framework/commands/`, and `about`, `setup` and `next` were renamed or retired), and `framework/commands/analyze.md` did not change in this window.

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
