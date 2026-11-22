---
spec: 070-performance-frontend-rules
last-run: 2026-11-22T02:12:32Z
analyzed-against: abcb9909d3ead89070e184689d09c7852d7a13be
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 1
analyzed-digest:
  plan.md: 4e9a4170f026f55ce53ac99c5ae6574c6f31c138665b3be5ab8af09b059d523d
  review.md: 7dfa156b78b494ef1ef69942955800ba2fae7530c577d5399a076c9da7e0b0aa
  spec.md: 979d1a1616ad84169b2505463307fadf0aeae678e3548faf45202cb5d8dbd021
  tasks.md: 46341aa1e5778aa95b5510257d7a29de9f808600b23c2e302f93bb825e4bd24b
unexamined-by-reason:
  ships-to-adopter: 1
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
  "plan.md#070 — Performance rules (frontend) Plan": 2ce298e0192a4001
  "plan.md#Affected Files": 05c5c2a1c6638116
  "plan.md#Cross-spec impact": cbec397c5674e3f5
  "plan.md#Every Source is re-fetched after the edits (AC3, AC4)": e240d446a71dc9a7
  "plan.md#Five rules move to SHOULD under their IDs (AC11)": 3bbf07888f663234
  "plan.md#Overview": c5f63e9542f7a910
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The header states the severity posture (AC11)": ac3c943f4e74af24
  "plan.md#The remaining criteria are verified last (AC1, AC2, AC3's schema, AC5–AC10)": 2aafdd171876a85b
  "plan.md#Trade-offs": 456873dbb31910e6
  "plan.md#`FE-VITALS-001` publishes its budgets in `specs/system.md` (AC12)": f84b7f51cbc6b744
  "spec.md": 2923d953e573a699
  "spec.md#070 — Performance rules (frontend)": d8f669a21da1c7c1
  "spec.md#Acceptance Criteria": 1df24245bca2bbdf
  "spec.md#Boundaries (cross-reference, do not duplicate)": 0954c123fa3112a5
  "spec.md#Enforcement": f9b0da77930bbe9e
  "spec.md#ID format": fd2d8eb59999ead4
  "spec.md#Motivation": fed2bd40190f0504
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 4057270af5f528aa
  "spec.md#Rule set scope": 9da2a3cdc5f75072
  "spec.md#Severity posture": 9472f7b701988ef4
rule-verdicts:
  FE-PII-003: {verdict: addressed, cite: "spec.md#Boundaries (cross-reference, do not duplicate)"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 3
  undispositioned: 0
decisions:
  - key: grounding — Enforcement states that /ductus:analyze loads every rule file present, regardless of stack or [rules] surfaces, as fact about current behavior with no citation to a spec, primitive or command source.
    outcome: waived
    reason: "advisory and no chore: summarizes 024's AC9 and `discover-rule-files`' compiled description (`all` selects every rule file in the directory and validates neither `[rules] surfaces` nor the disabled entries); Enforcement points at another spec's contract, which a citation would copy"
    decided-at: 2026-11-22T02:12:32Z
    decided-by: rule
  - key: grounding — Enforcement states that listing the file under [pinned] files only stops updates from overwriting a customized copy, as fact about the installer with no citation.
    outcome: waived
    reason: "advisory and no chore: summarizes the installer's contract in `framework/bootstrap/ductus.md` §Project Configuration (`[pinned] files` turns a manifest entry's `update` into `skip`); Enforcement points at it, and a citation would copy it"
    decided-at: 2026-11-22T02:12:32Z
    decided-by: rule
  - key: grounding — Rule set scope, Boundaries (accessibility-frontend.md) states that neither file cites the other and no rule overlaps, an absence claim about an existing file with no command or search cited.
    outcome: waived
    reason: "advisory and no chore: an absence claim that a grep re-derives: neither `performance-frontend.md` nor `accessibility-frontend.md` names the other (checked 2026-11-22); it records the boundary the spec draws, and a review of either file reads the files themselves"
    decided-at: 2026-11-22T02:12:32Z
    decided-by: rule
---

# Analysis — 070-performance-frontend-rules

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 1 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Enforcement states that /ductus:analyze loads every rule file present, regardless of stack or [rules] surfaces, as fact about current behavior with no citation to a spec, primitive or command source. — `specs/070-performance-frontend-rules/spec.md` — **waived**: advisory and no chore: summarizes 024's AC9 and `discover-rule-files`' compiled description (`all` selects every rule file in the directory and validates neither `[rules] surfaces` nor the disabled entries); Enforcement points at another spec's contract, which a citation would copy (decided by the rule)
- grounding — Enforcement states that listing the file under [pinned] files only stops updates from overwriting a customized copy, as fact about the installer with no citation. — `specs/070-performance-frontend-rules/spec.md` — **waived**: advisory and no chore: summarizes the installer's contract in `framework/bootstrap/ductus.md` §Project Configuration (`[pinned] files` turns a manifest entry's `update` into `skip`); Enforcement points at it, and a citation would copy it (decided by the rule)
- grounding — Rule set scope, Boundaries (accessibility-frontend.md) states that neither file cites the other and no rule overlaps, an absence claim about an existing file with no command or search cited. — `specs/070-performance-frontend-rules/spec.md` — **waived**: advisory and no chore: an absence claim that a grep re-derives: neither `performance-frontend.md` nor `accessibility-frontend.md` names the other (checked 2026-11-22); it records the boundary the spec draws, and a review of either file reads the files themselves (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- ships-to-adopter: 1

## Fixed in this run

*None.*
