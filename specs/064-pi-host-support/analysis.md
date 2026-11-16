---
spec: 064-pi-host-support
last-run: 2026-11-16T22:27:32Z
analyzed-against: 2a4542dc9bf0dbd50f41be831ec1001c7894a84e
hard-fail: 0
blocking-findings: 0
advisory: 6
unexamined: 24
analyzed-digest:
  plan.md: 4284bdac93e239db2ea41e23cae4d88751464f255b35100eccdd43e63a79de43
  review.md: 90cb9b67e19c2a763dd6a11444fe29c7184bb019f6c5458bd887ce4c351abbd2
  scenarios/pi-layout-is-dispatched.md: 0d0d2a6867c610e4d55252aa82fdf7a449ab0735ec16f47484a8c8dc332595fc
  scenarios/the-bridge-closes-its-server-at-shutdown.md: f852164bef881ed3a6f33eea81347d0bd23334182d9a5e810afcd46086a77f07
  scenarios/the-bridge-fails-loudly-and-recovers.md: 67570df65ceffa494936a5282eb62c100f14c125065562038aaa771077b1bf45
  spec.md: 69a71a473c7bde26131fd4538ba557a237d7ef44537809f06a6d82a971e34f0f
  tasks.md: a24b6d010388c3bdd7ada9ec6eaa388a027d22e2ac980fb59c7174e41e59a94c
unexamined-by-reason:
  anchor-target-unjudgeable: 1
  never-existed: 4
  not-a-live-claim: 1
  root-absent: 14
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
  BE-INPUT-002: {verdict: addressed, cite: "plan.md#D4 — `configure/pi.md`: verify-and-repair"}
  BE-INPUT-003: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
  BE-INPUT-004: {verdict: finding, tier: must, at: "plan.md#D3 — One runtime change: Pi's command-file candidate", message: "plan.md D3 adds the pi command-file candidate {cli-config-dir}/prompts/{project}-{name}.md, built from the unscreened cli-config-dir and [host] project and opened with no canonical containment, as are the claude-style candidate beside it and their four readers"}
  BE-RACE-001: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
  BE-RACE-002: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
  BE-RACE-003: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
  BE-TIMEOUT-001: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
  BE-TRACE-002: {verdict: finding, tier: should, at: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server", message: "plan.md wraps configure/pi.md's fetch and the bridge's calls to its runtime child, externally dependent work, in no span"}
  CFG-CONST-002: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
  CFG-CONST-003: {verdict: addressed, cite: "plan.md#D2 — Tool surface: zero-dependency extension bridge over the runtime's MCP server"}
blocking: false
dispositions:
  fixed: 0
  deferred: 1
  waived: 6
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-TRACE-002: plan.md wraps configure/pi.md's fetch and the bridge's calls to its runtime child, externally dependent work, in no span"
    outcome: waived
    reason: The runtime and its bridge are a per-agent stdio process with no tracing pipeline; adding the first span belongs to a spec about the runtime's observability, as 022's analysis decided for fetch-archive.
    decided-at: 2026-11-11T01:07:44Z
    decided-by: rule
  - key: grounding — spec.md's Verified Pi Layout states pi's permissions, native-rules, settings and trust behavior with no source, where plan.md grounds the command-surface and skills rows in pi's dist sources and the runtime-tools row cites pi's CHANGELOG
    outcome: waived
    reason: A host behavior whose re-sourcing needs pi's documentation or source or a run, which is research rather than a chore.
    decided-at: 2026-11-11T01:07:44Z
    decided-by: rule
  - key: grounding — spec.md's Compatibility Contract says pi has no deferred-tool mechanism, with no source
    outcome: waived
    reason: a pi host behavior whose source needs pi's documentation, its source or a run, which is research rather than a chore
    decided-at: 2026-11-11T13:28:01Z
    decided-by: rule
  - key: grounding — plan.md D2 names pi's built-in tools read and bash, with no source
    outcome: waived
    reason: a pi host behavior whose source needs pi's documentation, its source or a run, which is research rather than a chore
    decided-at: 2026-11-11T13:28:01Z
    decided-by: rule
  - key: grounding — plan.md D2's 0.3 s answer-time measurement states no spec, invocation or timing method
    outcome: waived
    reason: re-measuring with a stated method is research rather than a chore, and the figure only shows the 120 s bound sits more than two orders of magnitude above the runtime's answer time, a margin no plausible method moves
    decided-at: 2026-11-11T13:28:01Z
    decided-by: rule
  - key: grounding — spec.md and plan.md D1 say pi loads skill descriptions into the system prompt and prompt-matches them, with no source for that behavior
    outcome: waived
    reason: a pi host behavior whose source needs pi's documentation, its source or a run, which is research rather than a chore
    decided-at: 2026-11-11T13:28:01Z
    decided-by: rule
  - key: "rule-assessment — BE-INPUT-004: plan.md D3 adds the pi command-file candidate {cli-config-dir}/prompts/{project}-{name}.md, built from the unscreened cli-config-dir and [host] project and opened with no canonical containment, as are the claude-style candidate beside it and their four readers"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    task: "40"
    reason: Host::command_file_candidates and its four readers are 022's runtime core; 022 task 40 contains every candidate and screens both values (owner, 2026-11-16)
    decided-at: 2026-11-16T22:27:32Z
    decided-by: andy@stone.dev
---

# Analysis — 064-pi-host-support

## Summary

0 hard-fail, 0 blocking, 6 advisory; not blocking. 24 unexamined target(s). Dispositions: 0 fixed, 1 deferred, 6 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-TRACE-002: plan.md wraps configure/pi.md's fetch and the bridge's calls to its runtime child, externally dependent work, in no span — `specs/064-pi-host-support/plan.md` — **waived**: The runtime and its bridge are a per-agent stdio process with no tracing pipeline; adding the first span belongs to a spec about the runtime's observability, as 022's analysis decided for fetch-archive.
- grounding — Verified Pi Layout's Native rules row (`AGENTS.md` loaded natively, regardless of project trust) names no source; the provenance blockquote cites docs for settings, security, trust and permissions but not for rules loading — `specs/064-pi-host-support/spec.md` — **waived**: A host behavior whose re-sourcing needs pi's documentation or source or a run, which is research rather than a chore.
- grounding — Compatibility Contract states 'Pi has no permission prompts and no deferred-tool mechanism' as fact with no citation — `specs/064-pi-host-support/spec.md` — **waived**: a pi host behavior whose source needs pi's documentation, its source or a run, which is research rather than a chore
- grounding — plan.md D2 names pi built-ins (`read`, `bash`, ...) as collision surface with no citation — `specs/064-pi-host-support/plan.md` — **waived**: a pi host behavior whose source needs pi's documentation, its source or a run, which is research rather than a chore
- grounding — plan.md D2 says check-artifacts and compute-review-scope on the largest spec each answered in about 0.3 s (2026-10-28) with no spec named, invocation or timing method — `specs/064-pi-host-support/plan.md` — **waived**: re-measuring with a stated method is research rather than a chore, and the figure only shows the 120 s bound sits more than two orders of magnitude above the runtime's answer time, a margin no plausible method moves
- grounding — spec.md D1 and plan.md D1 say skills are description-loaded into the system prompt and prompt-matched; plan D1 cites interactive-mode.js only for the skill:<name> and enableSkillCommands facts — `specs/064-pi-host-support/plan.md` — **waived**: a pi host behavior whose source needs pi's documentation, its source or a run, which is research rather than a chore

## Deferred to other specs

- rule-assessment — BE-INPUT-004: plan.md D3 adds the pi command-file candidate {cli-config-dir}/prompts/{project}-{name}.md, built from the unscreened cli-config-dir and [host] project and opened with no canonical containment, as are the claude-style candidate beside it and their four readers — `specs/064-pi-host-support/plan.md` — **deferred** to `specs/022-deterministic-runtime/tasks.md` task 40

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1
- never-existed: 4
- not-a-live-claim: 1
- root-absent: 14
- ships-to-adopter: 4

## Fixed in this run

*None.*
