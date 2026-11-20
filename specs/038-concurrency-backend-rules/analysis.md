---
spec: 038-concurrency-backend-rules
last-run: 2026-11-20T04:17:41Z
analyzed-against: e5b1f3c36ca0861ee52434aee76ec616c5690230
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 1
analyzed-digest:
  plan.md: c1b586d4834516f588d58398694110e5cd241934f0324c320c742d2e76a42d54
  review.md: 6294995393f5c35cc7789244f78cfbe16faa5cfb3329a6d39f80f9abe9166108
  scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md: 1d82638ef829a2b345d14a253f1bea673ce1aa730d0b44110fd17799ed1c929d
  scenarios/lock-the-parent-before-rewriting-its-children.md: 81701adaf3813bbd84be93d692c6f65e309c58822c3229991be923461aef601b
  spec.md: 696d7a782085a54afde6c8503cbe1d578b1b6b64d5ec895c1613a05b92989f7a
  tasks.md: 345663a8e4ab99e5854915a7987f6926a489b96db4790b3db77216ae496ea496
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
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: grounding — spec.md Motivation states as fact that the backend surface had zero concurrency coverage when the spec was written, citing no rule-file listing or command
    outcome: waived
    reason: a historical claim about the corpus when the spec was written, not a present-tense claim about the system; git history of framework/rules holds its source and the spec's own Resolved Questions record the decision
    decided-at: 2026-11-20T04:17:41Z
    decided-by: rule
---

# Analysis — 038-concurrency-backend-rules

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 1 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — spec.md Motivation states as fact that the backend surface had zero concurrency coverage when the spec was written, citing no rule-file listing or command — `specs/038-concurrency-backend-rules/spec.md` — **waived**: a historical claim about the corpus when the spec was written, not a present-tense claim about the system; git history of framework/rules holds its source and the spec's own Resolved Questions record the decision (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- ships-to-adopter: 1

## Fixed in this run

*None.*
