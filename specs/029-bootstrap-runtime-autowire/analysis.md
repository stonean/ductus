---
spec: 029-bootstrap-runtime-autowire
last-run: 2026-11-14T02:41:26Z
analyzed-against: de9f01d0256ee948144fe9adc5aaa04afd5a2240
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 3
analyzed-digest:
  plan.md: cd25efecd0b3c0440829628602c5b9093dde8634fce66ba3ac7ea35605643908
  review.md: 3dc3225ee5648467859a43ecbc14d50cb952833b06e17db38629223d467faaa1
  scenarios/archive-fetch-direct-codeload.md: 0c333420c198f9fd69c3deb351da92c157df213d27e4c0f9dbbe09e70d2456cd
  scenarios/project-inputs-asked-once.md: 8c36e81564e833083b10b6bdee4440d27fd556deabb7b35df9a5d3bc97c1f411
  scenarios/runtime-probe-parity-audit.md: 584a4570a0b5ea5b345fa1b9f9612759f2b6d847087210470a41029782c60d12
  scenarios/state-a-deterministic-path-forcing.md: e95d7fa36a82af236c90dd03f582c15d3d34c72691706382d668b42359f92fea
  spec.md: acd5c55c6ecfef9a8a36e418b9cd3a99bda33a5d07f6a019fa158ab78afe29c2
  tasks.md: 715dad36bf66b5f4397855d7b9bb489d6f089387a51c931bca60b0aa0c2ac092
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
  security-backend.md: d89d6dad3ab51586ae4e68944b0c8e9ad4d5da721560f10590de3cdecc904d3f
  security-cross.md: dc98261801405fccaf5de9eb8f8ed9b020ebc78fcb5bfb124778a5fcc287950b
  security-frontend.md: e648707661636c2461485b362dfacc38cdfb9c770731e81599aee6929271a90e
rule-verdicts:
  BE-DATA-001: {verdict: addressed, cite: "scenarios/archive-fetch-direct-codeload.md#Behavior"}
  BE-DATA-011: {verdict: addressed, cite: "scenarios/archive-fetch-direct-codeload.md#Behavior"}
  BE-INPUT-001: {verdict: addressed, cite: "scenarios/state-a-deterministic-path-forcing.md#Behavior"}
  BE-INPUT-004: {verdict: addressed, cite: "scenarios/state-a-deterministic-path-forcing.md#Behavior"}
  BE-INPUT-008: {verdict: addressed, cite: "spec.md#MCP Wiring"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "scenarios/archive-fetch-direct-codeload.md#Behavior"}
  CFG-ENV-002: {verdict: addressed, cite: "scenarios/project-inputs-asked-once.md#Context"}
  CFG-ENV-007: {verdict: addressed, cite: "scenarios/project-inputs-asked-once.md#Behavior"}
  CFG-ENV-012: {verdict: finding, tier: should, at: "scenarios/project-inputs-asked-once.md#Behavior", message: "The new [project] config table's loader treatment of an unrecognized key (fail, or warn by name) is not stated."}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: "rule-assessment — CFG-ENV-012: The new [project] config table's loader treatment of an unrecognized key (fail, or warn by name) is not stated."
    outcome: waived
    reason: "advisory and no change to 029's design: the [project] section's loader already reports an unrecognized key by name without failing, as for every ductus-owned section (runtime/src/schema/config_keys.rs, warn_unknown_keys; 022's scenario config-readers-name-unknown-keys and 019's an-unknown-config-key-is-named own that behavior)"
    decided-at: 2026-11-14T02:41:26Z
    decided-by: rule
  - key: "grounding — Motivation states as fact that the model interprets a \"~20k-token procedure\", a measured figure about the existing bootstrap with no stated method, source or assumption marker"
    outcome: waived
    reason: a magnitude given as the motivation; no criterion or requirement depends on it, and re-measuring the markdown reference path's token cost is no mechanical chore
    decided-at: 2026-11-14T02:41:26Z
    decided-by: rule
---

# Analysis — 029-bootstrap-runtime-autowire

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 3 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — CFG-ENV-012: The new [project] config table's loader treatment of an unrecognized key (fail, or warn by name) is not stated. — `specs/029-bootstrap-runtime-autowire/scenarios/project-inputs-asked-once.md` — **waived**: advisory and no change to 029's design: the [project] section's loader already reports an unrecognized key by name without failing, as for every ductus-owned section (runtime/src/schema/config_keys.rs, warn_unknown_keys; 022's scenario config-readers-name-unknown-keys and 019's an-unknown-config-key-is-named own that behavior) (decided by the rule)
- grounding — Motivation states as fact that the model interprets a "~20k-token procedure", a measured figure about the existing bootstrap with no stated method, source or assumption marker — `specs/029-bootstrap-runtime-autowire/spec.md` — **waived**: a magnitude given as the motivation; no criterion or requirement depends on it, and re-measuring the markdown reference path's token cost is no mechanical chore (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- root-absent: 3

## Fixed in this run

*None.*
