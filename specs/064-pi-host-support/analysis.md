---
spec: 064-pi-host-support
last-run: 2026-11-11T13:28:01Z
analyzed-against: 2411dca9546740f5ac9e93713c33d9a57f6109b6
hard-fail: 0
blocking-findings: 0
advisory: 6
unexamined: 2
analyzed-digest:
  plan.md: ca053f1fec0a17f43d1ce7b1c78e29d974ecb2261f29e9dbb4888a001cd3db55
  review.md: 77cb20f8288f1c86b18bd3b4640f086b2bc5ebc452838bddda62bb7125e06ea4
  scenarios/pi-layout-is-dispatched.md: 0d0d2a6867c610e4d55252aa82fdf7a449ab0735ec16f47484a8c8dc332595fc
  scenarios/the-bridge-fails-loudly-and-recovers.md: 67570df65ceffa494936a5282eb62c100f14c125065562038aaa771077b1bf45
  spec.md: 69a71a473c7bde26131fd4538ba557a237d7ef44537809f06a6d82a971e34f0f
  tasks.md: 6d4c96ae728dedfb77b4eb2b0c481400b67ea114c70a35a1999b22fc052892ad
unexamined-by-reason:
  no-readable-state: 2
blocking: false
dispositions:
  fixed: 3
  deferred: 1
  waived: 6
  undispositioned: 0
decisions:
  - key: "rule-assessment — BE-INPUT-004: spec.md D4 and plan.md D3 add a candidate path built from ductus exec's command name, which run_exec and locate_command_file join into every candidate path with no screen"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    task: "30"
    reason: exec's command-name handling is 022's surface, and 064 appended only one of its four candidate shapes (owner, 2026-11-11)
    decided-at: 2026-11-11T01:07:44Z
    decided-by: andy@stone.dev
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
---

# Analysis — 064-pi-host-support

## Summary

0 hard-fail, 0 blocking, 6 advisory; not blocking. 2 unexamined target(s). Dispositions: 3 fixed, 1 deferred, 6 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- rule-assessment — BE-TRACE-002: plan.md wraps configure/pi.md's fetch and the bridge's calls to its runtime child, externally dependent work, in no span — `specs/064-pi-host-support/plan.md` — **waived**: The runtime and its bridge are a per-agent stdio process with no tracing pipeline; adding the first span belongs to a spec about the runtime's observability, as 022's analysis decided for fetch-archive. (decided by the rule)
- grounding — spec.md's Verified Pi Layout states pi's permissions, native-rules, settings and trust behavior with no source, where plan.md grounds the command-surface and skills rows in pi's dist sources and the runtime-tools row cites pi's CHANGELOG — `specs/064-pi-host-support/spec.md` — **waived**: A host behavior whose re-sourcing needs pi's documentation or source or a run, which is research rather than a chore. (decided by the rule)
- grounding — spec.md's Compatibility Contract says pi has no deferred-tool mechanism, with no source — `specs/064-pi-host-support/spec.md` — **waived**: a pi host behavior whose source needs pi's documentation, its source or a run, which is research rather than a chore (decided by the rule)
- grounding — plan.md D2 names pi's built-in tools read and bash, with no source — `specs/064-pi-host-support/plan.md` — **waived**: a pi host behavior whose source needs pi's documentation, its source or a run, which is research rather than a chore (decided by the rule)
- grounding — plan.md D2's 0.3 s answer-time measurement states no spec, invocation or timing method — `specs/064-pi-host-support/plan.md` — **waived**: re-measuring with a stated method is research rather than a chore, and the figure only shows the 120 s bound sits more than two orders of magnitude above the runtime's answer time, a margin no plausible method moves (decided by the rule)
- grounding — spec.md and plan.md D1 say pi loads skill descriptions into the system prompt and prompt-matches them, with no source for that behavior — `specs/064-pi-host-support/spec.md` — **waived**: a pi host behavior whose source needs pi's documentation, its source or a run, which is research rather than a chore (decided by the rule)

## Deferred to other specs

- rule-assessment — BE-INPUT-004: spec.md D4 and plan.md D3 add a candidate path built from ductus exec's command name, which run_exec and locate_command_file join into every candidate path with no screen — `specs/064-pi-host-support/plan.md` — **deferred** to `specs/022-deterministic-runtime/tasks.md` task 30

## Unexamined targets

- no-readable-state: 2

## Fixed in this run

- grounding — spec.md's Compatibility Contract item 3 says ductus exec dispatches primitives in-process and never touches the host's tool registry, with no source — `specs/064-pi-host-support/spec.md` — **fixed** (decided by the rule)
- grounding — plan.md D7 says pi 0.99.0 added its own MCP client, with no source — `specs/064-pi-host-support/plan.md` — **fixed** (decided by the rule)
- grounding — spec.md's Resolved Questions say every other host wraps bare names in a server namespace, with no source — `specs/064-pi-host-support/spec.md` — **fixed** (decided by the rule)
