---
spec: 077-security-cross-rules
last-run: 2026-11-22T12:45:28Z
analyzed-against: efe64adec171481dd595f0d02c8f676c3ffaabce
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 2
analyzed-digest:
  data-model.md: 83dd2b54037204cd50870705a4abfd7a3e0e7e416d43a4ef7915bbd578657416
  plan.md: 177622ff1d4f10a568f5651b49a304b6cd07e8a7e1123a429e3406b7766e7709
  review.md: 98ce2665936f9ee6f852c8067995def3494ce397778e484907c939988bd3ca29
  scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md: 5a85f93c9f4244f5a7616e1a2b6141143ff66140831c46830e43be32b3acbe8c
  spec.md: afc3a3ed03a9c4e8970706cfeaddd078500ac8d9bc2651686529f4947fbe21bf
  tasks.md: 07b92d874dd802ab791bd776553e12c5377e236a87152062d1a4b25bf89a02fb
unexamined-by-reason:
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
  security-backend.md: f892282383d7377ded2e5fd1ee3e30421d0d6ad3f9bb1fd354ff0d9d114e807a
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 5fddb21530faa805cb00f23c929e9402548ed0d6a0ed3e8e9a910561b74a45a3
section-digest:
  "data-model.md": e3b0c44298fc1c14
  "data-model.md#077 — Security cross rules Data Model": 56e956ebf742a873
  "data-model.md#Category abbreviations": a9dc2b7776d0db05
  "data-model.md#Rule set": e0e202f7cf813b13
  "data-model.md#`SEC` rule ID format": 4c3f5b22526613cb
  "data-model.md#`SEC` surface": 101c408f16931333
  "plan.md": e3b0c44298fc1c14
  "plan.md#077 — Security cross rules Plan": e994386e682f07bf
  "plan.md#A deprecated rule is a pointer": ca1623c6c7913bab
  "plan.md#Affected Files": a013a199d6bb82cf
  "plan.md#Cross-spec impact": c36027404a1a3f9e
  "plan.md#Data Model": 5923d691d9558526
  "plan.md#Distribution": dc70963b48d2025b
  "plan.md#Moved rules keep their text": 75011b9c36d0a8e2
  "plan.md#Open Questions Resolved": 062ca150b6bdfb5b
  "plan.md#Overview": cd25820e2e6f9b40
  "plan.md#Re-pointing": 891cabcc39aefe35
  "plan.md#Registering `SEC`": 4555c4c01c797d87
  "plan.md#Sweep": 3a672f753eebd10d
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#Trade-offs": 19e7f9b409a37454
  "plan.md#Verification": d3994239ab49c2c0
  "plan.md#`SEC-LOG-001` states its own value set": b50a2ef6f5e2ec06
  "scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md": 761f3959ea5ea586
  "scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md#Behavior": 1df7f20b7c55430c
  "scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md#Context": feaf6258c4c46392
  "scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md#Edge Cases": bea3d72db766c16c
  "scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md#Open Questions": 9fc5979d590cdb97
  "scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md#Url-pii-query-only-and-oauth-stricter-than-rfc-9700": 9626b86f1402e176
  "spec.md": d8c3eb84e9dea5df
  "spec.md#077 — Security cross rules": bfcd6dd36f24102c
  "spec.md#Acceptance Criteria": 1aec2450a660fadd
  "spec.md#Deprecation and re-pointing": 9e380daa078723e2
  "spec.md#Distribution": ae46ed9e03af602a
  "spec.md#Edge cases": 477af5687b6543a8
  "spec.md#Motivation": 2d8cd2b7699d4aa5
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 9ae45db448fde7fa
  "spec.md#Rule set scope": 3f8660249f9dbd1f
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 077-security-cross-rules

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 2 unexamined target(s). Dispositions: 3 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

- ships-to-adopter: 2

## Fixed in this run

- grounding — Technical Decisions, 'Re-pointing': 'observability-backend.md ... installs on the backend surface, beside security-backend.md' is stated as fact about the installer with no citation to the manifest or surface filter. — `specs/077-security-cross-rules/plan.md` — **fixed** (decided by the rule)
- grounding — Trade-offs, 'CFG-ENV-006 names both owners directly': 'a chain of yields is what 008's rounds found hardest to read alone' asserts what past rounds found with no cited record. — `specs/077-security-cross-rules/plan.md` — **fixed** (decided by the rule)
- grounding — Edge cases, review routing paragraph: 'configuration-cross.md has the quality pass' describes how review treats that file with no citation to framework/reference/review.md or the pass definition. — `specs/077-security-cross-rules/spec.md` — **fixed** (decided by the rule)
