---
spec: 062-concurrent-session-targets
last-run: 2026-11-19T13:28:35Z
analyzed-against: ca029bc311be75578872e0698287d94fed9df416
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 3
analyzed-digest:
  data-model.md: 2f139ba060dd47cd8145c26285e922cf1c4953880bfc2a11f1f64426749916ab
  plan.md: 05f372e7377d6cac6a25dfdce2c17eef79433a68adbb752b04b6b63202356b6d
  review.md: be70f306f88de4480e0079d1084fea2c8de84592436e1fc206854f13a829ddfc
  scenarios/first-writers-create-the-ignore-file-once.md: 10654412621bdf8d3809f10d1559179ba335acba2ea2bca183a8042a12a79ca6
  spec.md: 97172c7a3808fc2ffb13b88e7137b7d53054c5b73c24ffd91badcd8073585350
  tasks.md: 0470763b7f991104538c90010af84e0df993f61bffc87490a10cf3cdc6a9e6e3
unexamined-by-reason:
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
  BE-ERR-002: {verdict: addressed, cite: "data-model.md#`retarget-sessions`"}
  BE-INPUT-001: {verdict: addressed, cite: "data-model.md#`retarget-sessions`"}
  BE-INPUT-002: {verdict: addressed, cite: "spec.md#Process identity"}
  BE-INPUT-004: {verdict: addressed, cite: "plan.md#Storage: confine each process to its own file"}
  BE-INPUT-008: {verdict: addressed, cite: "data-model.md#Per-process record: `.ductus/sessions/{key}.toml`"}
  BE-LOCK-001: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "plan.md#`dashboard` and the exec seed"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-RACE-002: {verdict: addressed, cite: "plan.md#Storage: confine each process to its own file"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-REPR-001: {verdict: addressed, cite: "plan.md#`write-session`: own file plus default"}
  BE-RETRY-001: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-RETRY-003: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  BE-TXN-002: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  CFG-CONST-001: {verdict: addressed, cite: "plan.md#`write-session`: own file plus default"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#One session core, four callers"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#Synchronization: one advisory lock (`BE-RACE-001`)"}
  CFG-ENV-001: {verdict: addressed, cite: "plan.md#One session core, four callers"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-007: {verdict: addressed, cite: "plan.md#Identity"}
blocking: false
dispositions:
  fixed: 5
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 062-concurrent-session-targets

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 3 unexamined target(s). Dispositions: 5 fixed, 0 deferred, 0 waived, 0 undispositioned.

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

- root-absent: 3

## Fixed in this run

- grounding — §Synchronization gave about four hundred lock attempts in ten seconds with no method; it now states the arithmetic from `retry_bound`'s bounds, each wait averaging half its bound, about 405 (§Grounding (advisory); owner approval for the Group B directory). — `specs/062-concurrent-session-targets/plan.md` — **fixed** (decided by the rule)
- grounding — §Synchronization said the jitter comes from the standard library's randomly seeded hasher with no source; it now cites `RandomState::new`, documented as initialized with random keys in `library/std/src/hash/random.rs` (§Grounding (advisory); owner approval for the Group B directory). — `specs/062-concurrent-session-targets/plan.md` — **fixed** (decided by the rule)
- grounding — §Storage pointed the shared default's field order at `SessionRecord` in `runtime/src/primitives/write_session.rs`, which that file no longer holds; it now names `DefaultRecord` in `runtime/src/session.rs`, as the review made the data model say (§Grounding (advisory); owner approval for the Group B directory). — `specs/062-concurrent-session-targets/plan.md` — **fixed** (decided by the rule)
- grounding — §The shared default said a restarted agent comes back with a new platform identity, with no source; it now says one that does, as each new Claude Code session does, resumes the most recent target, and one that keeps its identity resumes its own (§Grounding (advisory); owner approval for the Group B directory). — `specs/062-concurrent-session-targets/spec.md` — **fixed** (decided by the rule)
- grounding — The Motivation said running two or more agent processes at once was the common case for the operator who raised it, with no source; it now attributes it to the owner's report recorded in `25c946ea` (§Grounding (advisory); owner approval for the Group B directory). — `specs/062-concurrent-session-targets/spec.md` — **fixed** (decided by the rule)
