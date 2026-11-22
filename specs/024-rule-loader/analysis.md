---
spec: 024-rule-loader
last-run: 2026-11-22T02:11:22Z
analyzed-against: b6c26fffd1399d90f2aa9c7d299d26cecae081b3
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 15
analyzed-digest:
  plan.md: 271ee1adf6bb10e8741efc373265586f7ff3d7192a05b13648374811bbb0f0fe
  review.md: 1e08a3f91291565a6110145bd3972ef2307247438344991253a9f504529e34c8
  spec.md: 0c9022cf365e0f6d1f45c13943a0f0b321a6a64235f79e02633a68cd1b02f705
  tasks.md: ad5f5f639894ee997ce69c1ca39da750cd3553cb0378469f6e12e56cc60dd573
unexamined-by-reason:
  never-existed: 1
  not-a-live-claim: 12
  ships-to-adopter: 2
rule-digest:
  accessibility-frontend.md: 4d7f5ac09d457c99774f5fc341222a464d2b461cb0467cd58a13dc31c928359b
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 050a5b8dd375fe4790f5d92753db27ae53ac730db1557cbee84b923b72499ca9
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: 511a68b25ceb148f8292cda2a829aaff03a38bc3453732e33a70bd5d6acf5df5
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 5fddb21530faa805cb00f23c929e9402548ed0d6a0ed3e8e9a910561b74a45a3
section-digest:
  "plan.md": e3b0c44298fc1c14
  "plan.md#024 — Stack-aware rule-file loader for `/ductus:review` Plan": 79f175e842fc2340
  "plan.md#Affected Files": b306a76f192fc230
  "plan.md#CI lint (`scripts/lint-rule-filenames.sh`)": 2051d4de30a78cda
  "plan.md#Considered and rejected": f8a7699a7cceb327
  "plan.md#Constitution edit": afee18bce008cee4
  "plan.md#Discovery algorithm (shared between `/ductus:review` and `/ductus:analyze`)": 982562caa3e2d9a4
  "plan.md#Known limitations": 8622c04b5a6e9e8a
  "plan.md#Overview": 76abcca97b8159e7
  "plan.md#Stdout notices": fed57f5bf7f33ff6
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Trade-offs": 6d48e49804e54ced
  "plan.md#`AGENTS.md` fallback narrowed (not removed)": b15e51e2b8161040
  "plan.md#`configuration.md` → `configuration-cross.md` rename": 81ba7a293c428af3
  "spec.md": 418af52f7bb2e69b
  "spec.md#024 — Stack-aware rule-file loader for `/ductus:review`": 4ba9934ed3c4ca80
  "spec.md#Acceptance Criteria": 30b70cfcc800809a
  "spec.md#Affected files": ab3059e70b10e85b
  "spec.md#Motivation": f3af948744b99570
  "spec.md#Non-goals": b4ddd8adc456a20d
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": d50d97c0264e67b5
rule-verdicts:
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#`AGENTS.md` fallback narrowed (not removed)"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "body-path-existence — names `framework/rules/configuration.md`, which existed in this repository and no longer resolves: \"| `framework/commands/implement.md` | Edit | Update §Scope Boundaries' `framework/rules/configuration.md` reference |\""
    outcome: waived
    reason: "advisory and no chore: the Affected Files row records the edit that swept the retired path out of `framework/commands/implement.md`, so naming the retired name is the row's subject (AC4), as the plan's rename section states"
    decided-at: 2026-11-22T02:11:22Z
    decided-by: rule
---

# Analysis — 024-rule-loader

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 15 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- body-path-existence — names `framework/rules/configuration.md`, which existed in this repository and no longer resolves: "| `framework/commands/implement.md` | Edit | Update §Scope Boundaries' `framework/rules/configuration.md` reference |" — `specs/024-rule-loader/plan.md` — **waived**: advisory and no chore: the Affected Files row records the edit that swept the retired path out of `framework/commands/implement.md`, so naming the retired name is the row's subject (AC4), as the plan's rename section states (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- never-existed: 1
- not-a-live-claim: 12
- ships-to-adopter: 2

## Fixed in this run

*None.*
