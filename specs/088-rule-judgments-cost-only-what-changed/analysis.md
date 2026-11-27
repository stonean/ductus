---
spec: 088-rule-judgments-cost-only-what-changed
last-run: 2026-11-27T00:18:16Z
analyzed-against: 812b7a890e96e1128a31b154e6f71fcd1ce2bed0
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 2
analyzed-digest:
  data-model.md: 048859896e2a703ed2b4fa0e6bc003f746e10fa2a62c7632463a8abc8730764e
  plan.md: aea4c6216bbfc648583264e85a18c25e2b9e8a8c69b5fd01bff0d3b4768e39c4
  review.md: 90b188b8dd0dbb7b02d653b6ede4f8c31a1629143a3391e40da4c9e62454bcbd
  spec.md: 5f5a62a41e6d21813edcd76b1647400f5184d88a2aa51895bcdf14129c03ecd8
  tasks.md: 3a4abc0a8cea56673feb2bfa0bb0bb0c2a6cc1031ce925f0121248cd654014d5
unexamined-by-reason:
  root-absent: 2
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
  "data-model.md": e3b0c44298fc1c14
  "data-model.md#088 — Rule judgments cost only what changed Data Model": 821def2eea8b57e5
  "data-model.md#Record fields": 2f6fa6c806fc7aaa
  "data-model.md#`backfill-section-digest`": 6e30b149e1f88a81
  "data-model.md#`discover-rule-files` with a `feature`: `rejudged-files`": 74d5a32548577f18
  "plan.md": e3b0c44298fc1c14
  "plan.md#088 — Rule judgments cost only what changed Plan": 7af42861fcea065b
  "plan.md#A changed rule file is compared rule by rule at `analyzed-against` (AC2)": 83d564c062d4dca4
  "plan.md#Affected Files": db76bd64446278fc
  "plan.md#Cross-spec impact": ebc10c22d96d19e9
  "plan.md#Data Model": 2d620264ae2ac773
  "plan.md#Overview": bc9743888dffee0a
  "plan.md#Primitive interface changes, classified under `BE-COMPAT-001`": c33d94bcd6153701
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Tests: each behavior fails with its condition removed": 692126048e2cf747
  "plan.md#The backfill is one corpus-wide primitive (AC3)": 1aa4a4334a752cf5
  "plan.md#The migration and this repository's records": ca1f32ff04d93ae6
  "plan.md#The narrowing lives in `standing`, which every caller shares": cf29514fb2ac2b63
  "plan.md#Trade-offs": 8dc6d977ee64999a
  "plan.md#What a run costs, and what it retires (089, AC4)": b50d443cd33bc010
  "plan.md#`write-analysis` records the two counts (AC1)": b28c7a728acfc28d
  "spec.md": 517ccde8bb7cabc3
  "spec.md#088 — Rule judgments cost only what changed": cb853396b62d645b
  "spec.md#Acceptance Criteria": e688691ce9767ee2
  "spec.md#Applicable Rules": 98e74abd7041bb4d
  "spec.md#Behavior": b18130632d4fbe36
  "spec.md#Cost": 93a98d54368da50c
  "spec.md#Edge Cases": 238793ee935b091b
  "spec.md#Motivation": a139ecee198696aa
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": ed4c13f09f3fbc57
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Primitive interface changes, classified under `BE-COMPAT-001`"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#`backfill-section-digest`"}
  BE-INPUT-004: {verdict: addressed, cite: "data-model.md#`backfill-section-digest`"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#`backfill-section-digest`"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#The backfill is one corpus-wide primitive (AC3)"}
  BE-RACE-003: {verdict: addressed, cite: "data-model.md#`backfill-section-digest`"}
  BE-TXN-002: {verdict: addressed, cite: "data-model.md#`backfill-section-digest`"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "spec.md#Applicable Rules"}
  QUAL-TEST-001: {verdict: addressed, cite: "plan.md#Tests: each behavior fails with its condition removed"}
rules-judged: 306
rules-carried: 0
blocking: false
dispositions:
  fixed: 7
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 088-rule-judgments-cost-only-what-changed

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 2 unexamined target(s). Dispositions: 7 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

- root-absent: 2

## Fixed in this run

- grounding — Technical Decisions, The backfill is one corpus-wide primitive, Pinned records: 'The runtime holds no reader of `[pinned] files`' is a claim of absence about current code, carrying no search or symbol citation (the cited ductus.md passage shows the host reads it, not that no primitive does). — `specs/088-rule-judgments-cost-only-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — Trade-offs, Rejected: ignoring `[pinned] files`: 'write-analysis does not read it' is stated as fact about current code with no file-and-symbol citation. — `specs/088-rule-judgments-cost-only-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — Technical Decisions, The narrowing lives in standing: 'the gate blocks done on [unjudged-rules]' (also in Trade-offs, Rejected: narrowing in discover-rule-files alone) asserts the gate's current behavior without naming the check or file that reads unjudged-rules. — `specs/088-rule-judgments-cost-only-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — Technical Decisions, Primitive interface changes, classified under `BE-COMPAT-001`: '`version` reads 0.64.0' states a current fact about the repository without naming the file that holds `version` (AGENTS.md's Workflow bullet names `version` beside `runtime/Cargo.toml`, so which file is not settled by the cite); the same claim recurs uncited in The migration section's The entry bullet. — `specs/088-rule-judgments-cost-only-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — Trade-offs, Rejected: the runtime reading `[pinned] files` itself: 'No primitive reads it' is stated as fact about current code with no file-and-symbol citation. — `specs/088-rule-judgments-cost-only-what-changed/plan.md` — **fixed** (decided by the rule)
- grounding — Motivation, Measured: 'write-analysis's `judged-rules` argument is not stored' is stated as fact about the record's schema with no file-and-symbol citation. — `specs/088-rule-judgments-cost-only-what-changed/spec.md` — **fixed** (decided by the rule)
- rule-assessment — BE-COMPAT-001: the plan changes `write-analysis`'s record, `discover-rule-files`' `verdicts` and adds `backfill-section-digest` without classifying any of it as additive or breaking, which the rule's Verification requires — `specs/088-rule-judgments-cost-only-what-changed/plan.md` — **fixed**
