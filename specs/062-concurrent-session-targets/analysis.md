---
spec: 062-concurrent-session-targets
last-run: 2026-11-18T13:07:02Z
analyzed-against: d793782d76f6450f20e84d63c4a25e9770a9a4e8
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 2
analyzed-digest:
  data-model.md: 18dc633f1739c9f32b632d67921378c8d2225a39e257076c8ad9cc88b45d6c9f
  plan.md: c9f6780af99ce78a368818f00e05ae31e3f0deb7b3ad3732e742f858b915b2d1
  review.md: 8fcb169d49bab581eb6fa0db348689ed3fd72eaf75bf8119333cafffd3428554
  scenarios/first-writers-create-the-ignore-file-once.md: 150b95396db7f7e774f4da1726618b953070531f1810c4272a5c080f7e47c0b8
  spec.md: 61277a4c97121bf594664e9d94a640099fbdc7a7b1d7f37d166f160e021ae262
  tasks.md: 0470763b7f991104538c90010af84e0df993f61bffc87490a10cf3cdc6a9e6e3
unexamined-by-reason:
  root-absent: 2
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
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Identity"}
  BE-INPUT-002: {verdict: addressed, cite: "spec.md#Process identity"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Storage: confine each process to its own file"}
  BE-INPUT-008: {verdict: addressed, cite: "data-model.md#Per-process record: `.ductus/sessions/{key}.toml`"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#`dashboard` and the exec seed"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-RACE-002: {verdict: addressed, cite: "plan.md#Storage: confine each process to its own file"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-RETRY-001: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-RETRY-003: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#`write-session`: own file plus default"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  CFG-CONST-004: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#One session core, four callers"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-007: {verdict: addressed, cite: "plan.md#Identity"}
  CFG-ENV-008: {verdict: addressed, cite: "plan.md#One session core, four callers"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 062-concurrent-session-targets

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
