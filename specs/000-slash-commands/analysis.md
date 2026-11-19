---
spec: 000-slash-commands
last-run: 2026-11-19T02:16:59Z
analyzed-against: b90ba7b7d7404050a96f7cc496a92fc725c39fb0
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 6
analyzed-digest:
  plan.md: 9d04757c0ea2e4ba28d7d0d381a8094ff61dd8a0d2bba0124b4178bf48a750b4
  review.md: 6928e132da5cc5e89bf99b177233392ef50d364453a72f788b778528b36ec5a5
  scenarios/a-done-stop-says-the-request-is-not-built.md: a578e1a0f2c9dfbaee173a31734aa64e2d42afd5d10f41df72fd9c47cd9b3402
  scenarios/analyze-reads-the-layout-derived-command-set.md: 134abf405d47afc5d4b055bd91105293db6ba2b434a82d0c1ea2a908a1f2e4d7
  scenarios/clarify-one-at-a-time.md: 74531377a3ba6862619c2a4aaf861084da5b53d69ac09a39f28740176f5c078d
  scenarios/command-autocomplete-summary.md: de4ef40b8d2a8587be508d333d502198cae4c312639b282ce363d8bb7253fa66
  scenarios/criterion-route-after-draft.md: 7fa00d35986c315ec41a85794276370181240fd34f822374a03cf816cacd7063
  scenarios/dashboard-dependencies-column.md: d1ce2860d34e9be0e4f34bb67651645e92b709bca1a1572833259923b9da0428
  scenarios/implement-skips-planned-prompt.md: adeb014c4192e84543d733fcf27f12907c2cada0d4950af9782a6168a6fac1c3
  scenarios/implement-stops-below-planned.md: 4e74cbbf5c625ee2b7415a3bc682f996d1b8356b2d25cb3203e8dcf08d9e0f81
  scenarios/scenario-without-task-visibility.md: a28dc9524faff8ff2ebca2c58c3ef125536694b9fe24efd88555703d13d81b34
  scenarios/target-argument-parsing.md: b91e1ec533ba1730f5b3760219ef3817c6238298ae8c351670a6edc9b6bb6db3
  scenarios/target-clear-flag.md: 1a64e209badd9131c6874a3786b01cc599e422965d21d9e7fac72acde67b114b
  scenarios/target-names-the-step-the-gate-would-name.md: 0d3ff3d98704f5df2d560838f193b1daadf073a62e5039ce9b215259b41bd31b
  scenarios/validation-gates.md: ad40135b38a38f59e616f28469869744f79196b625742bb013111475d72e4a07
  spec.md: 16789c8b16ecf492048453800ef09a24bd46101201afbbbb5589ae79be39670e
  tasks.md: 40a98a58684463ac4fe162fedc33ff6e028b5207caa947b62e89e728d4958405
unexamined-by-reason:
  never-existed: 2
  root-absent: 1
  ships-to-adopter: 3
rule-digest:
  accessibility-frontend.md: 5c1e7a9b5f625f14e6a1879c145ee30e25c5f5aff04ae4c853ee589e21b1efbb
  api-backend.md: 0caddd70fd1a546d80a5e77d254a38c36d158fcad464ee2493491a23cde06ea5
  concurrency-backend.md: 15209bb99ee1c7a63cfcde08a73f6de5df857834776cea43fadb7f0f1040370c
  configuration-cross.md: 2a807ba8facdfd1c3738383e6d44a1f6f2c6f91b387eac0ca5ed90d75dc9e178
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 0522a169c04113521bad128dfef283cd7e910264bf49c4a5320f27ba625fcdee
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
blocking: false
dispositions:
  fixed: 7
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 000-slash-commands

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 6 unexamined target(s). Dispositions: 7 fixed, 0 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 2
- root-absent: 1
- ships-to-adopter: 3

## Fixed in this run

- grounding — Overview states each command derives from a working Go backend adopter project's commands, with no citation or hedge (§Grounding (advisory)). — `specs/000-slash-commands/plan.md` — **fixed** (decided by the rule)
- grounding — Directory location rationale asserts files under .claude/commands/ would be treated as active slash commands in this repo, an uncited claim about host behavior (§Grounding (advisory)). — `specs/000-slash-commands/plan.md` — **fixed** (decided by the rule)
- grounding — Trade-offs states one file per command matches how Claude Code discovers and lists slash commands, an uncited claim about an external system (§Grounding (advisory)). — `specs/000-slash-commands/plan.md` — **fixed** (decided by the rule)
- grounding — A done stop decision states the constitution sits at its byte ceiling, a measurable fact with no command or figure cited (§Grounding (advisory)). — `specs/000-slash-commands/plan.md` — **fixed** (decided by the rule)
- grounding — Template References states each project copies the ductus templates into specs/templates/ during bootstrap, a descriptive claim naming no source (§Grounding (advisory)). — `specs/000-slash-commands/spec.md` — **fixed** (decided by the rule)
- plan-consistency — Technical Decision 'Parameterization approach' states what is done but no rationale for choosing it (§Plan consistency). — `specs/000-slash-commands/plan.md` — **fixed** (decided by the rule)
- grounding — Problem: 'Adopter projects have built their own slash commands from scratch, duplicating the pipeline logic that should be shared' is a descriptive claim about existing reality with no citation or assumption marker. — `specs/000-slash-commands/spec.md` — **fixed** (decided by the rule)
