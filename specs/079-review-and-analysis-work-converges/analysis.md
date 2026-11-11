---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-11T23:07:18Z
analyzed-against: 4f50cd32dc4ccab80c163cb3b04881321d91864b
hard-fail: 0
blocking-findings: 0
advisory: 12
unexamined: 4
analyzed-digest:
  data-model.md: 357b07dc46364739ff35833bc6ebacf66de41f115c108333de6448f24ddcb48b
  plan.md: 95f5ea11738fb9b8e3d2a31bd91d8f3b2584da47b871d86871dbc028c17f6313
  research.md: 61b8849d8667b2917709e2fbc610d0a8c12641719420727d2112ee773b34e19b
  review.md: cf62343097a830d156f82f69437227f52f614461d381eeebd0a395f8a099dde1
  spec.md: 42460e029b92a9dd0d264b9a23b5e6b982379faa8ee3995b85b655e7f5692a20
  tasks.md: 15eeed0a34336b84082b78a65f92e379660fbf07a3ca17b928e424a6b226b5dc
unexamined-by-reason:
  anchor-target-unjudgeable: 1
  ships-to-adopter: 3
blocking: false
dispositions:
  fixed: 7
  deferred: 0
  waived: 12
  undispositioned: 0
decisions:
  - key: grounding — spec.md says 12 arguments of four primitives have no CLI flag and 15 command pointers offered the CLI, with no method
    outcome: waived
    reason: Neither bbff7a53's message nor research.md records how they were counted, re-counting reads the CLI schema, which is outside this command's scope, and the decision rests on the fixture at 46d0dd6e the next sentence cites.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — spec.md says three --help texts were one line of 52, 63 and 61 bytes, with no method
    outcome: waived
    reason: Measuring needs the binary's output at 46d0dd6e's parent, outside this command's scope, and the figures only illustrate a one-line description since replaced.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — spec.md says a rejected option retires 2,857 lines, with no method
    outcome: waived
    reason: 5f81add7, which wrote it, records no method, and the decision rests on the MUSTs the re-reads found.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — spec.md says a rejected wording saves 254 bytes against the chosen one's 38, with no method
    outcome: waived
    reason: The choice rests on each option naming what it gives up, not on the bytes.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — spec.md says 35 commits name a spec without touching its directory and 30 the reverse, with no method
    outcome: waived
    reason: b9077c1a, which wrote it, gives no method, re-deriving the counts is a new measurement, and the rule needs only that both kinds of commit exist.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — spec.md says renaming 078 meant a sweep of 13 files and 46 lines, with no method
    outcome: waived
    reason: Neither b13364b2 nor 50a63f60 records the method, the sweep was never made, and the decision rests on its three stated reasons.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: grounding — plan.md says 3 of 70 analysis records held a plan-record decision, with no method
    outcome: waived
    reason: 6aef0472, which wrote it, gives no method; re-counted there, a grep for plan-record matches 3 of 70 records and a count of its keys 2, and the decision rests on how rare such a decision is either way.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: artifact-size — `specs/079-review-and-analysis-work-converges/spec.md` is 87355 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: A trim of this size is no chore, and splitting this spec would part the criteria AC1 and AC20 measure together; the size blocks nothing.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: artifact-size — `specs/079-review-and-analysis-work-converges/tasks.md` is 58420 bytes, 2 read pages at the 50000-byte read size
    outcome: waived
    reason: Pruning its done sections before done drops tasks 21, 23 and 26, which dispose of the misses AC1 and AC20 cite, so it waits for /ductus:prune once the spec is done.
    decided-at: 2026-11-09T16:01:45Z
    decided-by: rule
  - key: "rule-assessment — BE-REPR-002: 079's new fields and enum values cite no project-wide casing convention in specs/system.md"
    outcome: waived
    reason: This project declares no casing convention (it has no specs/system.md), and 079's new fields and values are kebab-case, as 022 fixes for tool names and as 058's, 078's and 080's analyses decided.
    decided-at: 2026-11-11T23:07:18Z
    decided-by: rule
  - key: grounding — spec.md says the owner waived 14 observations the recommendations had proposed deferring, with no source
    outcome: waived
    reason: The 14 come from that session's recommendations, which no record keeps; 078's review at d62e72f3 holds 19 waivers a person made, a count that does not separate them, and the decision rests on a deferral into the reviewed spec guaranteeing another cycle, not on the count.
    decided-at: 2026-11-11T23:07:18Z
    decided-by: rule
  - key: grounding — spec.md says a pattern search found 21 boxes in 18 task lists, with no pattern or commit
    outcome: waived
    reason: The pattern was not recorded and the boxes it found were rewritten as prose, so re-deriving is a new measurement at a commit nobody recorded, and the decision rests on such boxes existing, which task 26's three further finds confirm.
    decided-at: 2026-11-11T23:07:18Z
    decided-by: rule
---

# Analysis — 079-review-and-analysis-work-converges

## Summary

0 hard-fail, 0 blocking, 12 advisory; not blocking. 4 unexamined target(s). Dispositions: 7 fixed, 0 deferred, 12 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md says 12 arguments of four primitives have no CLI flag and 15 command pointers offered the CLI, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Neither bbff7a53's message nor research.md records how they were counted, re-counting reads the CLI schema, which is outside this command's scope, and the decision rests on the fixture at 46d0dd6e the next sentence cites. (decided by the rule)
- grounding — spec.md says three --help texts were one line of 52, 63 and 61 bytes, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Measuring needs the binary's output at 46d0dd6e's parent, outside this command's scope, and the figures only illustrate a one-line description since replaced. (decided by the rule)
- grounding — spec.md says a rejected option retires 2,857 lines, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: 5f81add7, which wrote it, records no method, and the decision rests on the MUSTs the re-reads found. (decided by the rule)
- grounding — spec.md says a rejected wording saves 254 bytes against the chosen one's 38, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: The choice rests on each option naming what it gives up, not on the bytes. (decided by the rule)
- grounding — spec.md says 35 commits name a spec without touching its directory and 30 the reverse, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: b9077c1a, which wrote it, gives no method, re-deriving the counts is a new measurement, and the rule needs only that both kinds of commit exist. (decided by the rule)
- grounding — spec.md says renaming 078 meant a sweep of 13 files and 46 lines, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Neither b13364b2 nor 50a63f60 records the method, the sweep was never made, and the decision rests on its three stated reasons. (decided by the rule)
- grounding — plan.md says 3 of 70 analysis records held a plan-record decision, with no method — `specs/079-review-and-analysis-work-converges/plan.md` — **waived**: 6aef0472, which wrote it, gives no method; re-counted there, a grep for plan-record matches 3 of 70 records and a count of its keys 2, and the decision rests on how rare such a decision is either way. (decided by the rule)
- artifact-size — `specs/079-review-and-analysis-work-converges/spec.md` is 88561 bytes, 2 read pages at the 50000-byte read size — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: A trim of this size is no chore, and splitting this spec would part the criteria AC1 and AC20 measure together; the size blocks nothing. (decided by the rule)
- artifact-size — `specs/079-review-and-analysis-work-converges/tasks.md` is 72500 bytes, 2 read pages at the 50000-byte read size — `specs/079-review-and-analysis-work-converges/tasks.md` — **waived**: Pruning its done sections before done drops tasks 21, 23 and 26, which dispose of the misses AC1 and AC20 cite, so it waits for /ductus:prune once the spec is done. (decided by the rule)
- rule-assessment — BE-REPR-002: 079's new fields and enum values cite no project-wide casing convention in specs/system.md — `specs/079-review-and-analysis-work-converges/data-model.md` — **waived**: This project declares no casing convention (it has no specs/system.md), and 079's new fields and values are kebab-case, as 022 fixes for tool names and as 058's, 078's and 080's analyses decided. (decided by the rule)
- grounding — spec.md says the owner waived 14 observations the recommendations had proposed deferring, with no source — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: The 14 come from that session's recommendations, which no record keeps; 078's review at d62e72f3 holds 19 waivers a person made, a count that does not separate them, and the decision rests on a deferral into the reviewed spec guaranteeing another cycle, not on the count. (decided by the rule)
- grounding — spec.md says a pattern search found 21 boxes in 18 task lists, with no pattern or commit — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: The pattern was not recorded and the boxes it found were rewritten as prose, so re-deriving is a new measurement at a commit nobody recorded, and the decision rests on such boxes existing, which task 26's three further finds confirm. (decided by the rule)

## Deferred to other specs

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1
- ships-to-adopter: 3

## Fixed in this run

- rule-assessment — BE-COMPAT-001: 079 changes diff-cross-spec's, compute-review-scope's, brief-re-read's and check-review-gate's results and the two writers' refusals, and no artifact classifies the change as additive or breaking — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-REPR-001: data-model.md gives measure-reopenings' since and run-decisions' last-run no timestamp representation — `specs/079-review-and-analysis-work-converges/data-model.md` — **fixed** (decided by the rule)
- rule-assessment — BE-INPUT-002: spec.md says the new primitives screen feature before reading, which states no allowlist — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- cross-spec-consistency — data-model.md states compute-review-scope's pre-081 review-owed rule as current — `specs/079-review-and-analysis-work-converges/data-model.md` — **fixed** (decided by the rule)
- grounding — spec.md says AGENTS.md asks for every push workflow's steps to run locally, present tense and with no commit — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md says the cut-off review read 240 of 295 files, with no source — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- grounding — plan.md says some first sentences run to a thousand bytes, with no source — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
