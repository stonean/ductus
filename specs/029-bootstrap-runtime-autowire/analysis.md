---
spec: 029-bootstrap-runtime-autowire
last-run: 2026-11-19T00:44:50Z
analyzed-against: efd3d488116e0752e8e7fb94f5c712c42f6263c5
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 7
analyzed-digest:
  plan.md: 9101bf2dbd922e758e456f47d0bc3c2e975a6e6f4566cb9ade9c494bab3d4b29
  review.md: 515186f1c572f6bb16fb0d2ebc5f3cca133715f26472a6a6804cabc613f4ef66
  scenarios/archive-fetch-direct-codeload.md: 0c333420c198f9fd69c3deb351da92c157df213d27e4c0f9dbbe09e70d2456cd
  scenarios/project-inputs-asked-once.md: 8c36e81564e833083b10b6bdee4440d27fd556deabb7b35df9a5d3bc97c1f411
  scenarios/runtime-probe-parity-audit.md: 584a4570a0b5ea5b345fa1b9f9612759f2b6d847087210470a41029782c60d12
  scenarios/state-a-deterministic-path-forcing.md: 6826e091652444615f197ea687ff2e25abad353a806148408c143c4641a561a8
  spec.md: b95d4805e0ef00d9e58108175b60165e9881fb2d1d2755a76a1f84d7616cd799
  tasks.md: 61479c531cffb8aa19a700f5bb387af7b7019df626b32c8e8c40400ab0d37751
unexamined-by-reason:
  anchor-target-unjudgeable: 4
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
  fixed: 8
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "rule-assessment — CFG-ENV-012: The new [project] config table's loader treatment of an unrecognized key (fail, or warn by name) is not stated."
    outcome: waived
    reason: "advisory and no change to 029's design: the [project] section's loader already reports an unrecognized key by name without failing, as for every ductus-owned section (runtime/src/schema/config_keys.rs, warn_unknown_keys; 022's scenario config-readers-name-unknown-keys and 019's an-unknown-config-key-is-named own that behavior)"
    decided-at: 2026-11-14T02:41:26Z
    decided-by: rule
---

# Analysis — 029-bootstrap-runtime-autowire

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 7 unexamined target(s). Dispositions: 8 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — CFG-ENV-012: The new [project] config table's loader treatment of an unrecognized key (fail, or warn by name) is not stated. — `specs/029-bootstrap-runtime-autowire/scenarios/project-inputs-asked-once.md` — **waived**: advisory and no change to 029's design: the [project] section's loader already reports an unrecognized key by name without failing, as for every ductus-owned section (runtime/src/schema/config_keys.rs, warn_unknown_keys; 022's scenario config-readers-name-unknown-keys and 019's an-unknown-config-key-is-named own that behavior) (decided by the rule)

## Deferred to other specs

*None.*

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 4
- root-absent: 3

## Fixed in this run

- grounding — Motivation states as fact that the markdown-only reference path is a '~20k-token procedure' with no measurement, method or citation. — `specs/029-bootstrap-runtime-autowire/spec.md` — **fixed** (decided by the rule)
- grounding — Motivation states as fact that 'MCP servers are loaded at session start', an external host behavior, with no source cited and no assumption marker; the whole design rests on it. — `specs/029-bootstrap-runtime-autowire/spec.md` — **fixed** (decided by the rule)
- grounding — State A section states that Claude Code surfaces tool names in a deferred-tool reminder before exposing schemas, with no source cited. — `specs/029-bootstrap-runtime-autowire/spec.md` — **fixed** (decided by the rule)
- grounding — Resolved Questions 'claude-style .mcp.json location' states that repo-root .mcp.json is Claude Code's documented project-level MCP config, with no link to the documentation or to the §MCP registration row. — `specs/029-bootstrap-runtime-autowire/spec.md` — **fixed** (decided by the rule)
- grounding — Technical Decisions 'ductus tool permissions granted at wiring time' states that exact-match dedup leaves the wildcard and enumerated entries both, a claim about the merge step's behavior with no citation of where that step is defined. — `specs/029-bootstrap-runtime-autowire/plan.md` — **fixed** (decided by the rule)
- lint-markdown — review.md line 243: MD037 spaces inside emphasis markers, from a criterion note's unbackticked mcp__ductus__* span — `specs/029-bootstrap-runtime-autowire/review.md` — **fixed** (decided by the rule)
- lint-markdown — review.md line 259: MD034 bare URL, from a criterion note's unbackticked codeload URL — `specs/029-bootstrap-runtime-autowire/review.md` — **fixed** (decided by the rule)
- resolve-anchor — the Resolved Question on the .mcp.json location cited §MCP registration, which does not begin with its heading's full text, MCP registration (per-agent) — `specs/029-bootstrap-runtime-autowire/spec.md` — **fixed** (decided by the rule)
