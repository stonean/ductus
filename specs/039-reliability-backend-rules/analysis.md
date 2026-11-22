---
spec: 039-reliability-backend-rules
last-run: 2026-11-22T02:11:49Z
analyzed-against: d90e9b31587eb09f9f33c2eafc710b9dbba4be5c
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 1
analyzed-digest:
  plan.md: ea3eecc68f6228f777486e5929a0685298cc987ebaa047e675ae813cb1592bd8
  review.md: 0d4bbbad4a121f1f043d57d18ce7b33e35da3c2a42f83e9b85c322f1d6c7968c
  scenarios/expendable-messages-may-be-dropped-when-recorded.md: 03e0030d79ae0938b4375c3ec2fc2950e08a41c96817f0a4b54d93536d66d22e
  spec.md: eb763bab29054e1ddc09e48287e82df5755e20c9ac75ec40381b31fd00587c69
  tasks.md: b73c498c0abf39cea79c781cfc37cad7a853bc35a70bc02efdd621b05e511ee7
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
  "plan.md#039 — Backend reliability rules Plan": 66e1b8709e700310
  "plan.md#Affected Files": 2d06ac8e9cb80792
  "plan.md#Cross-spec impact": ba19a20da5387017
  "plan.md#Manifest wiring — `framework/bootstrap/ductus.md`": 6acfacfac986dd0c
  "plan.md#Overview": 5cc60cd4d7bb83c9
  "plan.md#Rule set": fda7225f0749d785
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The rule file — `framework/rules/reliability-backend.md`": 5aab978af7b5cfb7
  "plan.md#Trade-offs": c76ffaeafdbed8e1
  "plan.md#What this feature does NOT touch": 9cce771a975a420b
  "scenarios/expendable-messages-may-be-dropped-when-recorded.md": 5473ad62d52d0e72
  "scenarios/expendable-messages-may-be-dropped-when-recorded.md#Behavior": 46602330219883ff
  "scenarios/expendable-messages-may-be-dropped-when-recorded.md#Context": d3285433708a17dd
  "scenarios/expendable-messages-may-be-dropped-when-recorded.md#Edge Cases": 984314eae386c267
  "scenarios/expendable-messages-may-be-dropped-when-recorded.md#Expendable-messages-may-be-dropped-when-recorded": 17640b5a45b80966
  "scenarios/expendable-messages-may-be-dropped-when-recorded.md#Open Questions": 9fc5979d590cdb97
  "scenarios/expendable-messages-may-be-dropped-when-recorded.md#Resolved Questions": b5f7b98fb3491576
  "spec.md": f0cb1ef05372b816
  "spec.md#039 — Backend reliability rules": 910812bcb4e35901
  "spec.md#Acceptance Criteria": 2499ee253160f1bc
  "spec.md#Boundaries (cross-reference, do not duplicate)": fc55226724b7f9be
  "spec.md#Motivation": bf103665a50c918a
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": df720ef156f83ce1
  "spec.md#Rule set scope": df531fab288d902c
  "spec.md#Severity posture": 7d58d235259c3653
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 3
  undispositioned: 0
decisions:
  - key: grounding — Technical Decisions / 'What this feature does NOT touch' states as fact that `BE` is already registered in `scripts/lint-rule-ids.sh` and that the IDs match its regex unchanged, with no command or file-and-symbol citation.
    outcome: waived
    reason: "advisory and no chore: true of the tree, since `scripts/lint-rule-ids.sh` registers BE among its surfaces in `id_re` (checked 2026-11-22); the claim bounds what 039 leaves untouched, and no gate reads a citation there"
    decided-at: 2026-11-22T02:11:49Z
    decided-by: rule
  - key: grounding — Motivation states as fact that reliability failures 'had nowhere to be promoted' and were 're-litigated per feature or surfaced only when a dependency degraded', citing no artifact.
    outcome: waived
    reason: "advisory and no chore: background in the Motivation describing the gap the rule file closes, not a claim about the system as it stands; the rule file's existence ends it"
    decided-at: 2026-11-22T02:11:49Z
    decided-by: rule
  - key: grounding — Acceptance Criteria AC10 states what PostgreSQL, MySQL, MongoDB and SQL Server timeouts reach as fact in the spec body, citing no source there; the plan says the rule's Source line carries it, but the spec does not point at it.
    outcome: waived
    reason: "advisory and no chore: the criterion restates what BE-TIMEOUT-003's Verification states, and the rule's **Source:** line in `framework/rules/reliability-backend.md` cites the PostgreSQL, MySQL 8.4, MongoDB and Microsoft Learn pages for all four engines (§drift-prevention: a pointer, never a copy); each review's criteria step judges AC10 against the tree"
    decided-at: 2026-11-22T02:11:49Z
    decided-by: rule
---

# Analysis — 039-reliability-backend-rules

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 1 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Technical Decisions / 'What this feature does NOT touch' states as fact that `BE` is already registered in `scripts/lint-rule-ids.sh` and that the IDs match its regex unchanged, with no command or file-and-symbol citation. — `specs/039-reliability-backend-rules/plan.md` — **waived**: advisory and no chore: true of the tree, since `scripts/lint-rule-ids.sh` registers BE among its surfaces in `id_re` (checked 2026-11-22); the claim bounds what 039 leaves untouched, and no gate reads a citation there (decided by the rule)
- grounding — Motivation states as fact that reliability failures 'had nowhere to be promoted' and were 're-litigated per feature or surfaced only when a dependency degraded', citing no artifact. — `specs/039-reliability-backend-rules/spec.md` — **waived**: advisory and no chore: background in the Motivation describing the gap the rule file closes, not a claim about the system as it stands; the rule file's existence ends it (decided by the rule)
- grounding — Acceptance Criteria AC10 states what PostgreSQL, MySQL, MongoDB and SQL Server timeouts reach as fact in the spec body, citing no source there; the plan says the rule's Source line carries it, but the spec does not point at it. — `specs/039-reliability-backend-rules/spec.md` — **waived**: advisory and no chore: the criterion restates what BE-TIMEOUT-003's Verification states, and the rule's **Source:** line in `framework/rules/reliability-backend.md` cites the PostgreSQL, MySQL 8.4, MongoDB and Microsoft Learn pages for all four engines (§drift-prevention: a pointer, never a copy); each review's criteria step judges AC10 against the tree (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- ships-to-adopter: 1

## Fixed in this run

*None.*
