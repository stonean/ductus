---
spec: 064-pi-host-support
last-run: 2026-11-11T01:07:44Z
analyzed-against: baac81e18f901cec281f34d8c969efa0e3397a60
hard-fail: 0
blocking-findings: 1
advisory: 2
unexamined: 0
analyzed-digest:
  plan.md: 328cef25058eca31fb5fc27f3b1cb06b6018e61b210b9bbf6dce8883d3e8efbb
  review.md: b9986d4a274aa3c982b947aa88a3c4aa9b955976dc308205d3cd1448c4dc9ef4
  scenarios/pi-layout-is-dispatched.md: 0d0d2a6867c610e4d55252aa82fdf7a449ab0735ec16f47484a8c8dc332595fc
  scenarios/the-bridge-fails-loudly-and-recovers.md: 67570df65ceffa494936a5282eb62c100f14c125065562038aaa771077b1bf45
  spec.md: 3b0b01da80ebc8b35e4107bdffb2001ef615d367b1dc2c9432631403733f308e
  tasks.md: b0232a37ab5cbcf0df8ac21253f905573db170677025d662f1d6297639d0eb66
blocking: true
dispositions:
  fixed: 5
  deferred: 1
  waived: 2
  undispositioned: 1
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
---

# Analysis — 064-pi-host-support

## Summary

0 hard-fail, 1 blocking, 2 advisory; blocking. 0 unexamined target(s). Dispositions: 5 fixed, 1 deferred, 2 waived, 1 undispositioned. An undispositioned advisory finding does not hold `done`; a hard-fail or blocking one holds it until the re-check no longer produces it.

## Hard failures

*None.*

## Blocking findings

- rule-assessment — BE-INPUT-003: configure/pi.md step 1's latest-release lookup interpolates `$DUCTUS_REPO` unquoted into its curl command line, and Source resolution applies no grammar to that variable, so a value holding whitespace or a glob changes the command's structure; ductus.md's own lookup has the same form — `specs/064-pi-host-support/plan.md` — **undispositioned**

## Advisory findings

- rule-assessment — BE-TRACE-002: plan.md wraps configure/pi.md's fetch and the bridge's calls to its runtime child, externally dependent work, in no span — `specs/064-pi-host-support/plan.md` — **waived**: The runtime and its bridge are a per-agent stdio process with no tracing pipeline; adding the first span belongs to a spec about the runtime's observability, as 022's analysis decided for fetch-archive. (decided by the rule)
- grounding — spec.md's Verified Pi Layout states pi's permissions, native-rules, settings and trust behavior with no source, where plan.md grounds the command-surface and skills rows in pi's dist sources and the runtime-tools row cites pi's CHANGELOG — `specs/064-pi-host-support/spec.md` — **waived**: A host behavior whose re-sourcing needs pi's documentation or source or a run, which is research rather than a chore. (decided by the rule)

## Deferred to other specs

- rule-assessment — BE-INPUT-004: spec.md D4 and plan.md D3 add a candidate path built from ductus exec's command name, which run_exec and locate_command_file join into every candidate path with no screen — `specs/064-pi-host-support/plan.md` — **deferred** to `specs/022-deterministic-runtime/tasks.md` task 30

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

- rule-assessment — BE-TIMEOUT-001: plan.md named no bound for configure/pi.md's fetch of the bridge — `specs/064-pi-host-support/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-DATA-001: plan.md committed configure/pi.md's fetch of the bridge to no TLS 1.2 floor — `specs/064-pi-host-support/plan.md` — **fixed** (decided by the rule)
- rule-assessment — BE-DATA-011: plan.md committed configure/pi.md's fetch of the bridge to no certificate and host-name verification — `specs/064-pi-host-support/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md D3 said the pre-existing host tests pass unmodified, which AC8 records was never true — `specs/064-pi-host-support/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md's added fetch sentence named ductus.md's policy as set for every fetch and `$DUCTUS_REPO` as quoted in both command lines — `specs/064-pi-host-support/plan.md` — **fixed** (decided by the rule)
