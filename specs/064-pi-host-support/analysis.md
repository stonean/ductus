---
spec: 064-pi-host-support
last-run: 2026-11-19T20:35:23Z
analyzed-against: 5597e0d1ea79eb67903f2ae91c0d140b574b020d
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 26
analyzed-digest:
  plan.md: d49d989518572800d5b6e726d36ace2fdd66ab3b74e74c2be56b052b2b7480bb
  review.md: bc2c99bcd0e5d747def0929d639356c261f4a4fa3069d4b403ae49d466c31b57
  scenarios/colon-form-commands-run-on-pi.md: c07825a14968e579c817cd098a61de4c07123b4548adacbda74815da8cc30180
  scenarios/pi-layout-is-dispatched.md: 0d0d2a6867c610e4d55252aa82fdf7a449ab0735ec16f47484a8c8dc332595fc
  scenarios/the-bridge-closes-its-server-at-shutdown.md: f852164bef881ed3a6f33eea81347d0bd23334182d9a5e810afcd46086a77f07
  scenarios/the-bridge-fails-loudly-and-recovers.md: 67570df65ceffa494936a5282eb62c100f14c125065562038aaa771077b1bf45
  spec.md: e09c3ae887fc7eaf270202d07054e1f6bcc770bc46de1071fc17ab72169fc5dd
  tasks.md: d0fb97c562d30de375dfa75dcdb7763bcee5ba8fdbe994b4344b45b78ebbd556
unexamined-by-reason:
  anchor-target-unjudgeable: 1
  never-existed: 4
  not-a-live-claim: 1
  root-absent: 16
  ships-to-adopter: 4
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
  BE-DATA-001: {verdict: addressed, cite: "plan.md#D4 — `configure/pi.md`: verify-and-repair"}
  BE-DATA-011: {verdict: addressed, cite: "plan.md#D4 — `configure/pi.md`: verify-and-repair"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
  BE-RACE-002: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
  BE-TRACE-002: {verdict: finding, tier: should, at: "plan.md#D4 — `configure/pi.md`: verify-and-repair", message: "plan.md wraps configure/pi.md's fetch and the bridge's calls to its runtime child, externally dependent work, in no span"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-TRACE-002: plan.md wraps configure/pi.md's fetch and the bridge's calls to its runtime child, externally dependent work, in no span"
    outcome: waived
    reason: The runtime and its bridge are a per-agent stdio process with no tracing pipeline; adding the first span belongs to a spec about the runtime's observability, as 022's analysis decided for fetch-archive.
    decided-at: 2026-11-11T01:07:44Z
    decided-by: rule
---

# Analysis — 064-pi-host-support

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 26 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-TRACE-002: plan.md wraps configure/pi.md's fetch and the bridge's calls to its runtime child, externally dependent work, in no span — `specs/064-pi-host-support/plan.md` — **waived**: The runtime and its bridge are a per-agent stdio process with no tracing pipeline; adding the first span belongs to a spec about the runtime's observability, as 022's analysis decided for fetch-archive. (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1
- never-existed: 4
- not-a-live-claim: 1
- root-absent: 16
- ships-to-adopter: 4

## Fixed in this run

*None.*
