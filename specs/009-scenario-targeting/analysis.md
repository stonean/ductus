---
spec: 009-scenario-targeting
last-run: 2026-10-17T03:35:28Z
analyzed-against: db2fa965654930ec587c347810ee17bcb027bd5c
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 2
analyzed-digest:
  plan.md: 7765f9095d8544b318ade70a76c0c42aee2de44c0b2f4b4f769e1c59ce7b8fa6
  review.md: 49b536c2ed32a72e09d35c43c5579f615e1be3839da84b68a8b9f612638b6b4e
  spec.md: 10c1a209f77c9b1bd4c4dfabab0d1540144d68cf1361bf64d7e382c5f411fc1a
  tasks.md: b6dcce805cf7a405ca5643b6bc6029a184fad7c001a44709488a540391e2acb0
unexamined-by-reason:
  not-a-live-claim: 1
  ships-to-adopter: 1
blocking: false
dispositions:
  fixed: 4
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 009-scenario-targeting

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 2 unexamined target(s). Dispositions: 4 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

- not-a-live-claim: 1
- ships-to-adopter: 1

## Fixed in this run

- grounding — plan.md says the session file is a JSON structure; it is .ductus/session.toml — `specs/009-scenario-targeting/plan.md` — **fixed**
- grounding — plan.md names a question command and a scenario command that are routes of amend since 023 — `specs/009-scenario-targeting/plan.md` — **fixed**
- grounding — plan.md says command parity is kept by paired edits; the generator regenerates the dogfooded copies — `specs/009-scenario-targeting/plan.md` — **fixed**
- grounding — plan.md describes the bootstrap as plural ductus files kept in parity across variants — `specs/009-scenario-targeting/plan.md` — **fixed**
