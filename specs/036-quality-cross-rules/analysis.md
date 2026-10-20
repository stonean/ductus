---
spec: 036-quality-cross-rules
last-run: 2026-10-20T00:44:50Z
analyzed-against: fbf2497e550737c108a21963077c9c2ab665f71a
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 2
analyzed-digest:
  data-model.md: 06945d4447d0897ad4339ddb3cabb05d433a5ce46410291828161cf0337874e1
  plan.md: 084707eca97c5c2502d7d34bb58e8bf5e9e12fbcc20c1769828bc14c388f7231
  review.md: 9f31fbf2f246cfdfbfade366c63609d978d9aebe612b5c4d1d7d062da2906a39
  spec.md: c8ca518b905b663e1ece107175de584f2523f8c11a2e9c5a1b98c362ea600d06
  tasks.md: 808050b711f7ddbcc33b3f277a8f7e3aa0f858a9a2bfca736b29043d9c5784a7
unexamined-by-reason:
  root-absent: 2
blocking: false
dispositions:
  fixed: 2
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 036-quality-cross-rules

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 2 unexamined target(s). Dispositions: 2 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

- root-absent: 2

## Fixed in this run

- grounding — spec.md Motivation asserted in the present tense that no rule file covers silent stubs, which quality-cross.md now does — `specs/036-quality-cross-rules/spec.md` — **fixed**
- grounding — spec.md Rule set scope gave configuration-cross.md's ID grammar as CFG-{CONST|ENV}, which now includes FLAG — `specs/036-quality-cross-rules/spec.md` — **fixed**
