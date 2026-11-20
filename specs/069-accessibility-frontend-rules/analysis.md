---
spec: 069-accessibility-frontend-rules
last-run: 2026-11-20T04:24:26Z
analyzed-against: 58de25c8356ab5779f84ad855371877fd114be79
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 1
analyzed-digest:
  plan.md: df2dccc974e132f37abbd4897e1bd1b89765e4ae470894c129cdfd310906b055
  review.md: be131aca3939c78f577fc4270f2fd4ae14c51cbd6918b7741d3331f9705cf7f1
  scenarios/verifications-flag-only-what-the-statement-covers.md: 4a7f7d2da98dd304b9da859c1eacb3b59fc65695a4a8b1ca3825ec806b7b26df
  spec.md: c7125aca1bbd160c9c34679b63181e92530870917b8aa242fbe9dc3f5dc2d56b
  tasks.md: bce7be1c6396bb3752cf2f477e41126f35aac91e463545dc7b5066dc9e89147d
unexamined-by-reason:
  ships-to-adopter: 1
rule-digest:
  accessibility-frontend.md: c2d9b6869062f8d6e7511b4f6f03f5c08c745c1139c360de668ca025464afea7
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 75143f9bd0dec8d62b3a8ed592f9a8c29a4192d863330fb7a92424029878fa50
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: 511a68b25ceb148f8292cda2a829aaff03a38bc3453732e33a70bd5d6acf5df5
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 5fddb21530faa805cb00f23c929e9402548ed0d6a0ed3e8e9a910561b74a45a3
blocking: false
dispositions:
  fixed: 5
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 069-accessibility-frontend-rules

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 1 unexamined target(s). Dispositions: 5 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

- ships-to-adopter: 1

## Fixed in this run

- grounding — grounding — the plan's claim that no check holds a rule file to any size, and that `[artifacts] read-size-bytes` governs spec artifacts, cited no source — `specs/069-accessibility-frontend-rules/plan.md` — **fixed** (decided by the rule)
- grounding — grounding — the plan's description of what `runtime.yml`'s corpus test and `framework-checks.yml`'s self-audit do cited no test or step — `specs/069-accessibility-frontend-rules/plan.md` — **fixed** (decided by the rule)
- grounding — grounding — Enforcement's claim that `/{project}:analyze` loads every rule file regardless of stack cited no source — `specs/069-accessibility-frontend-rules/spec.md` — **fixed** (decided by the rule)
- grounding — grounding — Enforcement's claim that a surfaces list without frontend stops installation cited no source — `specs/069-accessibility-frontend-rules/spec.md` — **fixed** (decided by the rule)
- grounding — grounding — Enforcement's claim that many Verifications name affected-files snippets gave no count or method — `specs/069-accessibility-frontend-rules/spec.md` — **fixed** (decided by the rule)
