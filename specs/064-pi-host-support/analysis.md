---
spec: 064-pi-host-support
last-run: 2026-10-28T18:53:01Z
analyzed-against: d1853444f01e2623b1b4bc7b92629fed3dde2825
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 23
analyzed-digest:
  plan.md: 843d2130db6192377e68a31a964e7b1f66ec502481e7704eed84d5e827fe5010
  review.md: aee9a043043b1e10117bb137fe1ec4cedb9175efc44fdc08b9005a303be4e28e
  scenarios/pi-layout-is-dispatched.md: 0d0d2a6867c610e4d55252aa82fdf7a449ab0735ec16f47484a8c8dc332595fc
  scenarios/the-bridge-fails-loudly-and-recovers.md: 67570df65ceffa494936a5282eb62c100f14c125065562038aaa771077b1bf45
  spec.md: 931538568dfbfcecfc38e8e6c18a0ce177ea7b14673dfbecee14f819679ea5b4
  tasks.md: b0232a37ab5cbcf0df8ac21253f905573db170677025d662f1d6297639d0eb66
unexamined-by-reason:
  never-existed: 4
  not-a-live-claim: 1
  root-absent: 14
  ships-to-adopter: 4
blocking: false
dispositions:
  fixed: 3
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 064-pi-host-support

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 23 unexamined target(s). Dispositions: 3 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

- never-existed: 4
- not-a-live-claim: 1
- root-absent: 14
- ships-to-adopter: 4

## Fixed in this run

- rule-assessment — BE-RACE-001: plan.md D2 shares the spawned runtime, its handshake state and the in-flight requests across concurrent tool calls without naming how that state is synchronized — `specs/064-pi-host-support/plan.md` — **fixed**
- rule-assessment — BE-RACE-003: plan.md D2's respawn on the first call after an exit is initialize-on-first-use with no stated mechanism for calls that arrive together — `specs/064-pi-host-support/plan.md` — **fixed**
- grounding — plan.md says the timeout is well above what the deterministic server takes to answer, with no measurement — `specs/064-pi-host-support/plan.md` — **fixed**
