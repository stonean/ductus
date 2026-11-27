---
spec: 086-review-records-hold-only-what-their-scope-judged
last-run: 2026-11-27T17:02:59Z
analyzed-against: 086cffca2eefec59f63fde3ddfffebbf18736979
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 0
analyzed-digest:
  plan.md: 7bb5dc290da3aba5644565c045cbbe34c60c310cbf08b3aabe2119b61d643c64
  review.md: 1765b3652587bc2683a1a787bd08fdf27d883c6f528ffc9787477d9df083ef63
  spec.md: 3dd417b85764e18a867bc186d9f072d30631557a619a31e16e0c51b5b81001d9
  tasks.md: c66ccde7d675703299a847a45b1c045f6246e29bbb3ce4eddcbf50e4428a57fa
rule-digest:
  accessibility-frontend.md: 20fb956e35bb749f061090adf631329bafe3e37662a4e47b0076b697784bbf50
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 050a5b8dd375fe4790f5d92753db27ae53ac730db1557cbee84b923b72499ca9
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: c9d90cf6bfa6e4f5492e6d1a49e073fbee467ba08c7788bf6c7224ae8bd5d411
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 9a200916d11a5597a6860d1538840bf7d9aa573bddce953a0bef604379d1451b
section-digest:
  "plan.md": e3b0c44298fc1c14
  "plan.md#086 — Review records hold only what their scope judged Plan": 8426b6432a9e578b
  "plan.md#A first review reads what the spec's commits changed (AC6)": 452522285fc1dd7b
  "plan.md#Affected Files": ab5aceb680a59ae7
  "plan.md#Compatibility": 0a3c551ab293dfd9
  "plan.md#Cross-spec impact": f8ce8a29bc4c31f3
  "plan.md#Data Model": 9286be7a1e099bf4
  "plan.md#Implementation follows 084's `done`": 43ca9da3263b899f
  "plan.md#One definition of \"belongs\"": 5c5e93a4f025b0f8
  "plan.md#Overview": 45db8bf4e59f3096
  "plan.md#Review drops what does not belong (AC2, AC3, AC4)": 5db8fb194236c3ed
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Trade-offs": 8d994d9b0775bb7b
  "plan.md#`/prune` runs it": 4952592f8a6b9c9f
  "plan.md#`prune-review` reduces a record between reviews (AC3, AC5)": 255f78dd24873ffa
  "plan.md#`write-review` refuses an outcome outside the scope (AC1)": 57feb64970fba378
  "spec.md": 60aa461e77b44734
  "spec.md#086 — Review records hold only what their scope judged": 285fa108c76f77af
  "spec.md#Acceptance Criteria": 8be186cb577c28fb
  "spec.md#Applicable Rules": 8385b0762ee2fcb4
  "spec.md#Behavior": fff408522a6cf1f8
  "spec.md#Cost": c32599e6e21c8e97
  "spec.md#Edge Cases": 562d594ba4187e75
  "spec.md#Motivation": d47d1ee5bcb4c562
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 63ff6d473f436806
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Compatibility"}
  BE-ERR-002: {verdict: addressed, cite: "plan.md#`write-review` refuses an outcome outside the scope (AC1)"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#`prune-review` reduces a record between reviews (AC3, AC5)"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#A first review reads what the spec's commits changed (AC6)"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#`prune-review` reduces a record between reviews (AC3, AC5)"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "spec.md#Cost"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#`prune-review` reduces a record between reviews (AC3, AC5)"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#`prune-review` reduces a record between reviews (AC3, AC5)"}
  BE-TXN-004: {verdict: addressed, cite: "spec.md#Edge Cases"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "plan.md#Review drops what does not belong (AC2, AC3, AC4)"}
  QUAL-DELEG-001: {verdict: addressed, cite: "spec.md#Applicable Rules"}
  QUAL-TEST-001: {verdict: addressed, cite: "spec.md#Applicable Rules"}
rules-judged: 306
rules-carried: 0
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 086-review-records-hold-only-what-their-scope-judged

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 0 unexamined target(s). Dispositions: 3 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

*None — every target was examined.*

## Fixed in this run

- grounding — §Technical Decisions, `prune-review`: "pi's bridge lists the server's tools when it loads, so it takes no entry" asserts how an existing integration behaves and names no file, symbol or command (the bridge is `.pi/extensions/ductus.ts` per the constitution) and carries no assumption marker. — `specs/086-review-records-hold-only-what-their-scope-judged/plan.md` — **fixed** (decided by the rule)
- grounding — §Technical Decisions, One definition of "belongs" (One spec, cheaply): "`Attributor::belongs_to` would not, since a commit that only renames the spec's directory keeps the directory's tree id" states how an existing function and git behave as fact, with no test, command or file cited. — `specs/086-review-records-hold-only-what-their-scope-judged/plan.md` — **fixed** (decided by the rule)
- grounding — §Cost, What it retires: "nothing but `write-review`, which renders it, and `prune-review` reads a `## Excluded from scope` line" is a claim of absence about the existing code with no search, command or file cited. — `specs/086-review-records-hold-only-what-their-scope-judged/spec.md` — **fixed** (decided by the rule)
