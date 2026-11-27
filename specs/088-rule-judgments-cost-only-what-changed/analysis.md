---
spec: 088-rule-judgments-cost-only-what-changed
last-run: 2026-11-27T01:14:44Z
analyzed-against: 983a63924a09f542de646dbdb13ffd56dc4fdfdf
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 2
analyzed-digest:
  data-model.md: 16f35f2d6ef5050a71927161866f13d7218722f3b210512e9a1811c5d738748e
  plan.md: 53792919226a22179509ce5536b13fd22fc74dd65ea351ebba64bfcf6aa4782f
  review.md: 3860fce3b21f1dafd29ed23d2cf2f09e66798819913bb455d368b3fb25f0a6ac
  spec.md: 30fd8403b5bab8afb8412baf90e7502512db1601b1e1e1dfc39adf0662d4023e
  tasks.md: 20a56efdd494bab51085142a6724a032a188b2309d447f2c6683f449b9d84234
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
  "data-model.md#`backfill-section-digest`": 0cd25469380d51bb
  "data-model.md#`discover-rule-files` with a `feature`: `rejudged-files`": 97667c25f3808f46
  "plan.md": e3b0c44298fc1c14
  "plan.md#088 — Rule judgments cost only what changed Plan": 7af42861fcea065b
  "plan.md#A changed rule file is compared rule by rule at `analyzed-against` (AC2)": 75aa12faa898c0e0
  "plan.md#Affected Files": 016552e721c20b2c
  "plan.md#Cross-spec impact": ebc10c22d96d19e9
  "plan.md#Data Model": 2d620264ae2ac773
  "plan.md#Overview": bc9743888dffee0a
  "plan.md#Primitive interface changes, classified under `BE-COMPAT-001`": c33d94bcd6153701
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Tests: each behavior fails with its condition removed": 967f52b31744df58
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
  "spec.md#Cost": 774cbfb9f54bc3d3
  "spec.md#Edge Cases": 1918a556ac31f48a
  "spec.md#Motivation": a139ecee198696aa
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": d46e35e8b310373e
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
  fixed: 0
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 088-rule-judgments-cost-only-what-changed

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 2 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

*None.*
