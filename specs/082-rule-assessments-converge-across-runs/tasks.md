---
next-task: 15
---

# 082 — Rule assessments converge across runs Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 14. AC7: a stored verdict stands while the sections it concerns are unchanged

- [x] Update `plan.md`'s design record for AC7 first: the record field holding one digest per heading section of each design subject (measured 2026-11-21 over 77 specs: median 34 headings across `spec.md`, `plan.md`, `data-model.md` and scenarios, max 586, about 1.4 KB median at 16-hex digests); how `discover-rule-files`' `verdicts` hands the judge each rule with the sections changed since its verdict; how `write-analysis` records a finding at an unchanged section as the advisory contradiction alone. The decision and its rejected options are this spec's Resolved Questions (AC3's annotation and the two marked Superseded, 2026-11-21).
- [x] Update `data-model.md`, 066's data model and the constitution's analysis-record row for the new field, and add 050 and 066 to `cross-spec-impact:`.
- [x] Implement in the runtime (`rule_verdicts`, `discover-rule-files`, `write-analysis`) with AC7's fixture, proven to fail with the section digests removed (`QUAL-CLAIM-001`).
- [x] Update `framework/reference/analyze.md` §Stored rule verdicts and steps 11, 12 and 22 of `framework/commands/analyze.md` to state the section rule and the advisory contradiction.

- **Done when**: AC7's fixture passes and fails with the section digests removed, and the plan, data model, reference and command state the rule.
