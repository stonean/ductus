---
spec: 025-rule-opt-out
last-run: 2026-10-20T03:10:31Z
analyzed-against: 036f03cb52093d36add21e9d737f2c4e8824ea30
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 10
analyzed-digest:
  plan.md: fd84c40b55b8cfb56007beffb7fc0da7b6e153e59b8d42783758305b7f8889d9
  review.md: 4466eb7d4de9abfd30177306b18b3f4b31a73aa450fa5010d6a86836d397b223
  scenarios/a-malformed-disabled-entry-fails-fast.md: 7ee5da88cdbed0431f83d6012c89f0fa07d3683589fe42a0490def27061d8100
  spec.md: a931eb603774ecaa53fa30246579d8f689e650d27262de71f7f03fc47b24c838
  tasks.md: 3538181dfd2511785603a8e163b905c03126c251307eed5e8ecd692243b8b48f
unexamined-by-reason:
  never-existed: 2
  not-a-live-claim: 8
blocking: false
dispositions:
  fixed: 3
  routed: 1
  discarded: 0
  undispositioned: 0
decisions:
  - key: "rule-assessment — CFG-ENV-012: 025 introduces [[review.disabled-rule-files]] entries whose unrecognized fields are accepted silently, without a warning naming them"
    outcome: routed
    target: specs/019-config-decisions/tasks.md
    decided-at: 2026-10-20T03:10:31Z
    decided-by: andy@stone.dev
---

# Analysis — 025-rule-opt-out

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 10 unexamined target(s). Dispositions: 3 fixed, 1 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — CFG-ENV-012: 025 introduces [[review.disabled-rule-files]] entries whose unrecognized fields are accepted silently, without a warning naming them — `specs/025-rule-opt-out/spec.md` — **routed** to `specs/019-config-decisions/tasks.md`

## Unexamined targets

- never-existed: 2
- not-a-live-claim: 8

## Fixed in this run

- rule-assessment — BE-INPUT-011: the /status disabled-rule-files callout rendered each committed file name raw, so a newline in one forged a dashboard line — `specs/025-rule-opt-out/plan.md` — **fixed**
- anchor — plan.md cites §Malformed and duplicate waivers, §Output and §Behavior, sections of review.md the constitution does not carry — `specs/025-rule-opt-out/plan.md` — **fixed**
- grounding — plan.md says in the present tense that analyze.md does not read .ductus/config.toml and that runtime warnings cover malformed entries — `specs/025-rule-opt-out/plan.md` — **fixed**
