---
spec: 038-concurrency-backend-rules
last-run: 2026-11-22T02:11:42Z
analyzed-against: 3bc7fa88f0a23f2186bcd8b02bcc51a6bc01fcd2
hard-fail: 0
blocking-findings: 0
advisory: 3
unexamined: 1
analyzed-digest:
  plan.md: 45f02c01401e106b47f777c1ab74f3f0157d8e6de7f6e1d413171581021e3e00
  review.md: 6116ab45c60b0d099251e56e184bc425a657feac1abd937f07ea0ad5b7f3e102
  scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md: 1d82638ef829a2b345d14a253f1bea673ce1aa730d0b44110fd17799ed1c929d
  scenarios/lock-the-parent-before-rewriting-its-children.md: 81701adaf3813bbd84be93d692c6f65e309c58822c3229991be923461aef601b
  spec.md: b66dff4edf05d9113e0280cd941b8dd0f0c3bb7d104855502b1094567a9c0a69
  tasks.md: 345663a8e4ab99e5854915a7987f6926a489b96db4790b3db77216ae496ea496
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
  "plan.md#038 — Backend concurrency rules Plan": fb7298074f1f5034
  "plan.md#Affected Files": 3019e2d58c0dd148
  "plan.md#Cross-spec impact": 887dca372e217f5b
  "plan.md#Manifest wiring — `framework/bootstrap/ductus.md`": 971d0b8e72157a00
  "plan.md#Overview": 1439635a7b10d76c
  "plan.md#Rule set": 9b43cb0f37745b7e
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The rule file — `framework/rules/concurrency-backend.md`": 773c33fa31add8f5
  "plan.md#Trade-offs": 63db7ccd6ec065a6
  "plan.md#What this feature does NOT touch": 69be4ed1d79bbd57
  "scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md": 761f3959ea5ea586
  "scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md#A-declared-loss-tolerant-message-owes-no-outbox": b3560543325afe65
  "scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md#Behavior": b2cb8d2720d7dd35
  "scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md#Context": 31eb31bde1db6545
  "scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md#Edge Cases": 8b18df9d1af9c99f
  "scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md#Open Questions": 9fc5979d590cdb97
  "scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/lock-the-parent-before-rewriting-its-children.md": 761f3959ea5ea586
  "scenarios/lock-the-parent-before-rewriting-its-children.md#Behavior": 070d57d1e0eac020
  "scenarios/lock-the-parent-before-rewriting-its-children.md#Context": ec0728a90b90cde8
  "scenarios/lock-the-parent-before-rewriting-its-children.md#Edge Cases": d30a7024ef3974b6
  "scenarios/lock-the-parent-before-rewriting-its-children.md#Lock-the-parent-before-rewriting-its-children": d9ff8e8aabfada2a
  "scenarios/lock-the-parent-before-rewriting-its-children.md#Open Questions": 20edda8881516e4e
  "scenarios/lock-the-parent-before-rewriting-its-children.md#Resolved Questions": d0a540a6c12c2163
  "spec.md": 8d5c1fc524368007
  "spec.md#038 — Backend concurrency rules": eabb83f864c83562
  "spec.md#Acceptance Criteria": 5b23a127174e5556
  "spec.md#Boundaries (cross-reference, do not duplicate)": aad68c6f47b77640
  "spec.md#Motivation": 6befdaf0eeec0a10
  "spec.md#Open Questions": 20edda8881516e4e
  "spec.md#Resolved Questions": 06cb9258b739e215
  "spec.md#Rule set scope": c095dbeb8a35ac08
  "spec.md#Severity posture": 1f6c506c464a45b8
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 3
  undispositioned: 0
decisions:
  - key: grounding — spec.md Motivation states as fact that the backend surface had zero concurrency coverage when the spec was written, citing no rule-file listing or command
    outcome: waived
    reason: a historical claim about the corpus when the spec was written, not a present-tense claim about the system; git history of framework/rules holds its source and the spec's own Resolved Questions record the decision
    decided-at: 2026-11-20T04:17:41Z
    decided-by: rule
  - key: grounding — Motivation states the constitution names concurrency twice, with only a bare section pointer (§rules) and no quoted location for the second naming.
    outcome: waived
    reason: "advisory and no chore: both namings sit in the section the pointer names: the constitution's §rules lists concurrency among a rule's cross-cutting concerns, and its When to write a rule subsection among the governance-recognized categories; nothing rests on the count"
    decided-at: 2026-11-22T02:11:42Z
    decided-by: rule
  - key: grounding — Acceptance Criteria AC6 states the rule file does not mention pools or connections at all, a descriptive claim about the file's content with no command or query cited.
    outcome: waived
    reason: "advisory and no chore: a criterion about the rule file's own content, which every review's criteria step judges against the tree and records the evidence for; a citation inside the criterion would be a second, unchecked copy of that evidence"
    decided-at: 2026-11-22T02:11:42Z
    decided-by: rule
---

# Analysis — 038-concurrency-backend-rules

## Summary

0 hard-fail, 0 blocking, 3 advisory; not blocking. 1 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 3 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Motivation states the backend surface had zero coverage of concurrency when written, a descriptive claim about the rule corpus with no cited query or file (hedged only by time). — `specs/038-concurrency-backend-rules/spec.md` — **waived**: a historical claim about the corpus when the spec was written, not a present-tense claim about the system; git history of framework/rules holds its source and the spec's own Resolved Questions record the decision (decided by the rule)
- grounding — Motivation states the constitution names concurrency twice, with only a bare section pointer (§rules) and no quoted location for the second naming. — `specs/038-concurrency-backend-rules/spec.md` — **waived**: advisory and no chore: both namings sit in the section the pointer names: the constitution's §rules lists concurrency among a rule's cross-cutting concerns, and its When to write a rule subsection among the governance-recognized categories; nothing rests on the count (decided by the rule)
- grounding — Acceptance Criteria AC6 states the rule file does not mention pools or connections at all, a descriptive claim about the file's content with no command or query cited. — `specs/038-concurrency-backend-rules/spec.md` — **waived**: advisory and no chore: a criterion about the rule file's own content, which every review's criteria step judges against the tree and records the evidence for; a citation inside the criterion would be a second, unchecked copy of that evidence (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- ships-to-adopter: 1

## Fixed in this run

*None.*
