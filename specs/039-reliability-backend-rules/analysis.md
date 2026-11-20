---
spec: 039-reliability-backend-rules
last-run: 2026-11-20T04:33:26Z
analyzed-against: 79add1eb2c0d3a252ac03a501e7c625e13f21e32
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 1
analyzed-digest:
  plan.md: fe713ce14ec0cb37c41f3e962cb1d4844d7cbcb576cbb0c53e8f1122eb2dab3b
  review.md: ddac5ef03683ae0b558c1d226dd1cbc48228ee26a08c3217d55493a264f69724
  scenarios/expendable-messages-may-be-dropped-when-recorded.md: 03e0030d79ae0938b4375c3ec2fc2950e08a41c96817f0a4b54d93536d66d22e
  spec.md: 482f9e09af51d6455e51746e511e2dc7eeb3774193d566d5208f37b4a20edcc5
  tasks.md: b73c498c0abf39cea79c781cfc37cad7a853bc35a70bc02efdd621b05e511ee7
unexamined-by-reason:
  ships-to-adopter: 1
rule-digest:
  accessibility-frontend.md: 39a0edea7e2cad9ed30c6e3d5c2edccddfecd8a6b77151df4c71c9b20a901cd4
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
rule-verdicts:
  CFG-CONST-003: {verdict: addressed, cite: "spec.md#Boundaries (cross-reference, do not duplicate)"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: grounding — Acceptance Criteria AC10 states as fact that MySQL max_execution_time reaches read-only SELECT statements only, with no source cited or assumption marker (Grounding).
    outcome: waived
    reason: "the criterion restates what the rule's Verification states, and the rule's own **Source:** line cites the MySQL 8.4 manual and Microsoft Learn pages that substantiate it (constitution §drift-prevention: a pointer to the canonical source, never a copy); rewording AC10 to add a citation would reopen its review for no change in the requirement"
    decided-at: 2026-11-20T04:33:26Z
    decided-by: rule
  - key: grounding — Acceptance Criteria AC10 states as fact that SQL Server's query timeout is a client driver setting, with no source cited or assumption marker (Grounding).
    outcome: waived
    reason: "the criterion restates what the rule's Verification states, and the rule's own **Source:** line cites the MySQL 8.4 manual and Microsoft Learn pages that substantiate it (constitution §drift-prevention: a pointer to the canonical source, never a copy); rewording AC10 to add a citation would reopen its review for no change in the requirement"
    decided-at: 2026-11-20T04:33:26Z
    decided-by: rule
---

# Analysis — 039-reliability-backend-rules

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 1 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Acceptance Criteria AC10 states as fact that MySQL max_execution_time reaches read-only SELECT statements only, with no source cited or assumption marker (Grounding). — `specs/039-reliability-backend-rules/spec.md` — **waived**: the criterion restates what the rule's Verification states, and the rule's own **Source:** line cites the MySQL 8.4 manual and Microsoft Learn pages that substantiate it (constitution §drift-prevention: a pointer to the canonical source, never a copy); rewording AC10 to add a citation would reopen its review for no change in the requirement (decided by the rule)
- grounding — Acceptance Criteria AC10 states as fact that SQL Server's query timeout is a client driver setting, with no source cited or assumption marker (Grounding). — `specs/039-reliability-backend-rules/spec.md` — **waived**: the criterion restates what the rule's Verification states, and the rule's own **Source:** line cites the MySQL 8.4 manual and Microsoft Learn pages that substantiate it (constitution §drift-prevention: a pointer to the canonical source, never a copy); rewording AC10 to add a citation would reopen its review for no change in the requirement (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- ships-to-adopter: 1

## Fixed in this run

*None.*
