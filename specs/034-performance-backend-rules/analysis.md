---
spec: 034-performance-backend-rules
last-run: 2026-11-22T02:11:29Z
analyzed-against: 1e3f37ae0da221035141ee9a1a9bae6ad63e79f0
hard-fail: 0
blocking-findings: 0
advisory: 5
unexamined: 1
analyzed-digest:
  plan.md: b02f8036d5182a22e50b4ef19a16be0e92870ea77f86eeee5cdca083da70c134
  review.md: 4d6308cffb9f8cffaf582104f9fb013a4ce955e2adc30c2d04157e6c82068220
  spec.md: dee1dcffda095b8d57f7b5a8029949b7c21d6621dbefae96163663d36536cbcd
  tasks.md: c7648f1a1bc67db6687278765b10ae5fb4e0aab0e153e3ff8799aa57144718b7
unexamined-by-reason:
  anchor-target-unjudgeable: 1
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
  "plan.md#034 — Backend performance rules Plan": 05ebdecfe9be3cd8
  "plan.md#Affected Files": cd61585fbf2e5c67
  "plan.md#Categories and abbreviations": 64d488c062e7ee42
  "plan.md#Installation": d856a4476140a283
  "plan.md#Overview": a0310e4c1dcc19d0
  "plan.md#Rule list": 08a8b0c2e997ba84
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Trade-offs": 84529a117023eb65
  "plan.md#Verification style": 5058892a689bea49
  "spec.md": 57e8bb88772b3cae
  "spec.md#034 — Backend performance rules": d1be79882b1bf041
  "spec.md#Acceptance Criteria": ed32b361c2a333dd
  "spec.md#Boundaries (cross-reference, do not duplicate)": 6e3d688008eb557d
  "spec.md#Motivation": bdfea73e747384ca
  "spec.md#Resolved Questions": 35db79574c1b6ec7
  "spec.md#Rule set scope": 4b404a03e7d452ff
  "spec.md#Severity posture": a2940164e1ba1c4d
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 5
  undispositioned: 0
decisions:
  - key: "grounding — Technical Decisions, Rule list, BE-QUERY-004 row: states as fact that first populating a database is 'the different case PostgreSQL's guidance addresses with COPY or a single transaction', with no link or fetched source."
    outcome: waived
    reason: "advisory and no chore: the text summarizes BE-QUERY-004, whose **Source:** line in `framework/rules/performance-backend.md` cites PostgreSQL's Populating a Database page; the plan's rule-list row points at the rule, and a second citation would copy it (§drift-prevention)"
    decided-at: 2026-11-22T02:11:29Z
    decided-by: rule
  - key: "grounding — Technical Decisions, Rule list, BE-QUERY-003 row: states as fact that 'OFFSET bounds memory but not work', with no source or assumption marker."
    outcome: waived
    reason: "advisory and no chore: the text summarizes BE-QUERY-003, whose **Source:** line in `framework/rules/performance-backend.md` cites PostgreSQL's LIMIT and OFFSET page among its sources; the plan's rule-list row points at the rule, and a second citation would copy it (§drift-prevention)"
    decided-at: 2026-11-22T02:11:29Z
    decided-by: rule
  - key: "grounding — Technical Decisions, Installation: states that 033's surface filter selects the file under backend 'automatically via the -backend.md suffix', a claim about existing behavior carrying no file-and-symbol citation."
    outcome: waived
    reason: "advisory and no chore: a summary of the selection 024 and 033 own, stated canonically in the constitution's §rules (Filename suffix) and `discover-rule-files`' compiled description; 034's design rests only on the file's `-backend.md` suffix"
    decided-at: 2026-11-22T02:11:29Z
    decided-by: rule
  - key: "grounding — Rule set scope, BE-PAYLOAD-003 row: the BREACH carve-out names an external attack class and its 'Vary: Accept-Encoding' requirement with no fetched source in the spec."
    outcome: waived
    reason: "advisory and no chore: the text summarizes BE-PAYLOAD-003, whose **Source:** line in `framework/rules/performance-backend.md` cites RFC 9110 §12.5.5 (Vary) and the BREACH paper; the spec's scope row points at the rule, and a second citation would copy it (§drift-prevention)"
    decided-at: 2026-11-22T02:11:29Z
    decided-by: rule
  - key: "grounding — Motivation: states that the frontend surface shipped performance-frontend.md covering Core Web Vitals, bundle size and image/font discipline, with no link to the file or its spec."
    outcome: waived
    reason: "advisory and no chore: background in the Motivation naming the sibling file 034 mirrors, `framework/rules/performance-frontend.md`, which 070 owns; no requirement rests on its description"
    decided-at: 2026-11-22T02:11:29Z
    decided-by: rule
---

# Analysis — 034-performance-backend-rules

## Summary

0 hard-fail, 0 blocking, 5 advisory; not blocking. 1 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 5 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Technical Decisions, Rule list, BE-QUERY-004 row: states as fact that first populating a database is 'the different case PostgreSQL's guidance addresses with COPY or a single transaction', with no link or fetched source. — `specs/034-performance-backend-rules/plan.md` — **waived**: advisory and no chore: the text summarizes BE-QUERY-004, whose **Source:** line in `framework/rules/performance-backend.md` cites PostgreSQL's Populating a Database page; the plan's rule-list row points at the rule, and a second citation would copy it (§drift-prevention) (decided by the rule)
- grounding — Technical Decisions, Rule list, BE-QUERY-003 row: states as fact that 'OFFSET bounds memory but not work', with no source or assumption marker. — `specs/034-performance-backend-rules/plan.md` — **waived**: advisory and no chore: the text summarizes BE-QUERY-003, whose **Source:** line in `framework/rules/performance-backend.md` cites PostgreSQL's LIMIT and OFFSET page among its sources; the plan's rule-list row points at the rule, and a second citation would copy it (§drift-prevention) (decided by the rule)
- grounding — Technical Decisions, Installation: states that 033's surface filter selects the file under backend 'automatically via the -backend.md suffix', a claim about existing behavior carrying no file-and-symbol citation. — `specs/034-performance-backend-rules/plan.md` — **waived**: advisory and no chore: a summary of the selection 024 and 033 own, stated canonically in the constitution's §rules (Filename suffix) and `discover-rule-files`' compiled description; 034's design rests only on the file's `-backend.md` suffix (decided by the rule)
- grounding — Rule set scope, BE-PAYLOAD-003 row: the BREACH carve-out names an external attack class and its 'Vary: Accept-Encoding' requirement with no fetched source in the spec. — `specs/034-performance-backend-rules/spec.md` — **waived**: advisory and no chore: the text summarizes BE-PAYLOAD-003, whose **Source:** line in `framework/rules/performance-backend.md` cites RFC 9110 §12.5.5 (Vary) and the BREACH paper; the spec's scope row points at the rule, and a second citation would copy it (§drift-prevention) (decided by the rule)
- grounding — Motivation: states that the frontend surface shipped performance-frontend.md covering Core Web Vitals, bundle size and image/font discipline, with no link to the file or its spec. — `specs/034-performance-backend-rules/spec.md` — **waived**: advisory and no chore: background in the Motivation naming the sibling file 034 mirrors, `framework/rules/performance-frontend.md`, which 070 owns; no requirement rests on its description (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1

## Fixed in this run

*None.*
