---
spec: 000-slash-commands
last-run: 2026-10-17T13:14:43Z
reviewed-against: e2fd392b3aaed2a593441cc4eee19eab8b8b88d3
diff-base: 013acb74db244b6fcf14721a13548cbc0575bb6e
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 12
scope: 21
skipped-passes: []
reviewed-digest:
  scenarios/analyze-reads-the-layout-derived-command-set.md: 5e69626c6c04595004bd61f6fccb854e1615f4f6d91699ed7049bbe75f14dff0
  scenarios/clarify-one-at-a-time.md: 74531377a3ba6862619c2a4aaf861084da5b53d69ac09a39f28740176f5c078d
  scenarios/command-autocomplete-summary.md: de4ef40b8d2a8587be508d333d502198cae4c312639b282ce363d8bb7253fa66
  scenarios/criterion-route-after-draft.md: b08bf71669de7980442f8246be334fac4440d43c1920eca95c770878f5c45386
  scenarios/dashboard-dependencies-column.md: e471757b1a1f7167129935144362f08c47c5da2dea585405a85f9ce2bcbc8f67
  scenarios/implement-skips-planned-prompt.md: adeb014c4192e84543d733fcf27f12907c2cada0d4950af9782a6168a6fac1c3
  scenarios/implement-stops-below-planned.md: 4e74cbbf5c625ee2b7415a3bc682f996d1b8356b2d25cb3203e8dcf08d9e0f81
  scenarios/scenario-without-task-visibility.md: a28dc9524faff8ff2ebca2c58c3ef125536694b9fe24efd88555703d13d81b34
  scenarios/target-argument-parsing.md: b91e1ec533ba1730f5b3760219ef3817c6238298ae8c351670a6edc9b6bb6db3
  scenarios/target-clear-flag.md: b7dd0b1df765a600403715a0012afacc9c13749122cd15d7d56ad2b69252eb52
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

Re-review after the notes check rewrote 000 (cffe953b, e2fd392b) and restored implement's lost gate (a5c89a01): spec.md, a deleted scenario and a new one staled the review. Diff base 013acb74, the parent of the reopen commit; scope 21 entries. Rule files: all 11 under framework/rules/ loaded — quality-cross.md and configuration-cross.md read in full, the nine surface files at rule-ID and Statement level, since the scope is markdown prose plus command sources and no surface rule's trigger fires on it. Examined 12. Read in full: 000's spec.md and its implement-stops-below-planned scenario; 010's spec.md and plan.md; 028's spec.md. Read in part: framework/commands/implement.md (Purpose, Context and Flags, Instructions steps 1-4 including the restored stop, Setup and Stuck-detection details); framework/commands/plan.md (its Gate section and keep-or-replace prompt); framework/commands/clarify.md (the gate table and the status branches of its first steps); framework/commands/analyze.md (Instructions, read in full earlier in this session); framework/commands/help.md (Purpose and Scope Boundaries, for AC4); AGENTS.md (Workflow and Gotchas, plus the two passages edited this round); 000's tasks.md (task 20). Not examined, 9: the two implement.md mirrors, regenerated from a source read here; framework/bootstrap/configure/claude.md, framework/commands/specify.md, status.md and target.md, unchanged since the diff base (status and target checked only by grep for the next-action behavior AC7 names); runtime/tests/golden/implement-basic.jsonl, compared with its sha fields normalized and otherwise byte-identical, not read; the deleted validate-fix-mode scenario; and 028's review.md, a record written minutes earlier. The restored stop sits in step 4 beside the transition it guards, so no step renumbers; the parity suite passed after a one-test re-bless. The quality pass found the utility list's count stale against the six utilities 000 named, fixed before this record (e2fd392b). 0 MUST, 0 SHOULD, 0 low-confidence; no open observations.

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
