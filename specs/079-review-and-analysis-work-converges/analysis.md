---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-09T16:01:45Z
analyzed-against: b0d8ea876ffa24292d115df3ccaa326529142403
hard-fail: 0
blocking-findings: 0
advisory: 9
unexamined: 2
analyzed-digest:
  data-model.md: 4b3a9af90b0d67323bbb7ec3838715fed768dfc8a365667fe96359b9bd21027c
  plan.md: e9ab7eac73ae9a6bf699539e0907597a240d10fefbe3219225f4b9888c679367
  research.md: 61b8849d8667b2917709e2fbc610d0a8c12641719420727d2112ee773b34e19b
  review.md: cf62343097a830d156f82f69437227f52f614461d381eeebd0a395f8a099dde1
  spec.md: 7c1a44eda7cb97262ff322e5e98bc77d81f8a8734a710889502fbcfdbb9e6d23
  tasks.md: 636bb0affc645fe5ed361d80c6040795ae5e1f842ec2eb3f184d07edacd2a6db
blocking: false
dispositions:
  fixed: 13
  deferred: 0
  waived: 9
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
---

# Analysis — 079-review-and-analysis-work-converges

## Summary

0 hard-fail, 0 blocking, 9 advisory; not blocking. 2 unexamined target(s). Dispositions: 13 fixed, 0 deferred, 9 waived, 0 undispositioned.

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
- artifact-size — `specs/079-review-and-analysis-work-converges/spec.md` is 87355 bytes, 2 read pages at the 50000-byte read size — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: A trim of this size is no chore, and splitting this spec would part the criteria AC1 and AC20 measure together; the size blocks nothing. (decided by the rule)
- artifact-size — `specs/079-review-and-analysis-work-converges/tasks.md` is 58420 bytes, 2 read pages at the 50000-byte read size — `specs/079-review-and-analysis-work-converges/tasks.md` — **waived**: Pruning its done sections before done drops tasks 21, 23 and 26, which dispose of the misses AC1 and AC20 cite, so it waits for /ductus:prune once the spec is done. (decided by the rule)

## Deferred to other specs

*None.*

## Unexamined targets

2 target(s) unexamined; no breakdown recorded.

## Fixed in this run

- grounding — spec.md says a fix round diffed the whole working tree against its base, with no source — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md says no stored key has a file-less shape, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md says no spec here carries references:, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md says the constitution had 57 spare bytes, with no method — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- grounding — plan.md says 30 commits touch a spec directory without naming one, with no source — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says the exec walker no-ops review step 12, with no source — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says no spec here carries references:, with no method — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says clap's help renderer turns {n} into a newline, with no source — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says the constitution holds 49,936 of 50,000 bytes, present tense and with no method — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says 35 commits name a spec without touching its directory, with no source — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says 0 of this history's 5 renames fell inside a reopening, with no source — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md says nothing enforces a reopen today, which its own paragraph on review-state drift qualifies — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- applicable-rules — the CFG-CONST-001 citation says the sample is a named constant, which the data model says is a criterion — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
