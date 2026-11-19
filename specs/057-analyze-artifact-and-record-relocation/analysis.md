---
spec: 057-analyze-artifact-and-record-relocation
last-run: 2026-11-19T22:51:38Z
analyzed-against: 1e9110c30cf3218bc27afae79c4d51e2bcd113ba
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 5
analyzed-digest:
  data-model.md: 9d6dc21acae6e3e128006a20b2c379d5f9987020d2c21b9e0c885f9fc94dd365
  plan.md: ad5c8600ebf2c8f10f0493970331c03356b77b11d7d98f61fbbca8bc776ef3ec
  review.md: 3b82385de2dc22ef015f7be566add749b95ca2a81065ff38e2b851a2c5d8aa21
  scenarios/a-merge-carries-each-keys-whole-value.md: c0cc9145d9f13e8deb06d0befe848a21bcd48b3c6d6ad24cba2ae2e648fc4138
  spec.md: 9736a05c7076160f483d9a10a1f4789cedf7628377d4987f21b6c19ec15fb29b
  tasks.md: 466f8f9ea0f74d46d48938de520c7f02f4565c2bd7516132d793048085a0f37e
unexamined-by-reason:
  not-a-live-claim: 2
  root-absent: 3
rule-digest:
  accessibility-frontend.md: 5c1e7a9b5f625f14e6a1879c145ee30e25c5f5aff04ae4c853ee589e21b1efbb
  api-backend.md: 0caddd70fd1a546d80a5e77d254a38c36d158fcad464ee2493491a23cde06ea5
  concurrency-backend.md: 15209bb99ee1c7a63cfcde08a73f6de5df857834776cea43fadb7f0f1040370c
  configuration-cross.md: 2a807ba8facdfd1c3738383e6d44a1f6f2c6f91b387eac0ca5ed90d75dc9e178
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 0522a169c04113521bad128dfef283cd7e910264bf49c4a5320f27ba625fcdee
  security-backend.md: 9873cc885c4bdc76eff79a7269a5a6858a0b84c3010ff4b1bc27e7989f708183
  security-cross.md: bf70990f24e7e67926772be0d95deb7e1c1492bc12bdcb650a23b25385bcd285
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-COMPAT-001: {verdict: addressed, cite: "plan.md#Trade-offs"}
  BE-ERR-002: {verdict: addressed, cite: "data-model.md#Errors"}
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#`ReviewBlock` and `AnalyzeBlock` keep their shape and change their owner"}
  BE-INPUT-004: {verdict: addressed, cite: "data-model.md#`relocate-audit-records` result"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#`relocate-audit-records` result"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#The corpus sweep is a primitive, invoked per spec"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#The corpus sweep is a primitive, invoked per spec"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#The corpus sweep is a primitive, invoked per spec"}
  BE-TXN-004: {verdict: addressed, cite: "data-model.md#Errors"}
  QUAL-CLAIM-001: {verdict: addressed, cite: "spec.md#Applicable Rules"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: grounding — Motivation states as fact that the audit family resolving `reviewed-against` never compares the counts on either side, naming no file, symbol or command for that audit family (Grounding advisory).
    outcome: waived
    reason: A dated historical account of the pre-relocation state this spec removed (the retired check-review-agreement and its audit family), which no command can reproduce now; it motivates the change and gates no criterion.
    decided-at: 2026-11-19T22:51:38Z
    decided-by: rule
  - key: grounding — The sequencing section states that three raw parsers own `waivers` with `#[serde(flatten)]` and that `schema/primitives.rs` carries no `serde_norway::Value`, without naming the three parsers or a command that substantiates it (Grounding advisory).
    outcome: waived
    reason: A rationale for a task ordering already delivered, recording why the waivers field stayed open-schema; the claim names the field and the file it concerns, a search for serde(flatten) in runtime/src/primitives finds the parsers, and no criterion depends on it.
    decided-at: 2026-11-19T22:51:38Z
    decided-by: rule
---

# Analysis — 057-analyze-artifact-and-record-relocation

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 5 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Motivation states as fact that the audit family resolving `reviewed-against` never compares the counts on either side, naming no file, symbol or command for that audit family (Grounding advisory). — `specs/057-analyze-artifact-and-record-relocation/spec.md` — **waived**: A dated historical account of the pre-relocation state this spec removed (the retired check-review-agreement and its audit family), which no command can reproduce now; it motivates the change and gates no criterion. (decided by the rule)
- grounding — The sequencing section states that three raw parsers own `waivers` with `#[serde(flatten)]` and that `schema/primitives.rs` carries no `serde_norway::Value`, without naming the three parsers or a command that substantiates it (Grounding advisory). — `specs/057-analyze-artifact-and-record-relocation/plan.md` — **waived**: A rationale for a task ordering already delivered, recording why the waivers field stayed open-schema; the claim names the field and the file it concerns, a search for serde(flatten) in runtime/src/primitives finds the parsers, and no criterion depends on it. (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- not-a-live-claim: 2
- root-absent: 3

## Fixed in this run

*None.*
