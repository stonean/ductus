---
spec: 027-bootstrap-migration-registry
last-run: 2026-10-16T19:45:45Z
analyzed-against: 500a3af9f7a5eaa8b551ef38215d29e11e78156d
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 10
analyzed-digest:
  plan.md: e610f371a63d9a3109b0faebcf707aa7768c8048bce5b69e8a74834057b6241d
  review.md: 29bbdaa1535b07a7dc3731ba10e21eba42cf423243d90d11f7254e69f86b4a43
  scenarios/migration-chain-reference-integrity.md: b187024e6ed8fee90fe31be0f2bbcd4786782f8e4395a89c09142f20218f914d
  scenarios/migrations-apply-once-per-repo.md: ef57af25a1eb071076c60b005ffdbf9f3a1b2afad889ba939b230679b9d4f9b4
  spec.md: e243b40cc1f72f26594c90c21e2d0fe9dfa4517a856e51c25a4f3969d2b9fb3d
  tasks.md: ff144324ac4ec523c5ef57afaf0922196affd4889c659017111d8d1683aaf360
unexamined-by-reason:
  not-a-live-claim: 4
  root-absent: 5
  ships-to-adopter: 1
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
decisions:
  - key: plan-record — plan.md §Known Limitations is outside the design record
    outcome: discarded
    reason: "deliberately part of this plan's design: it records the v1 limits the registry shipped with (uniform back-fill sunsets, discipline-only removal coverage and CHANGELOG append, a loosely specified procedure-file shape), the case analyze's Plan record section names as a discard (operator decision, 2026-10-16)"
    decided-at: 2026-10-16T02:24:44Z
    decided-by: andy@stone.dev
  - key: grounding — spec.md and plan.md say TOML parsers don't guarantee array-of-tables order preservation across rewrites
    outcome: discarded
    reason: rationale for the resolved Registry-order decision in the design record; the order rule it supports (introduced_in by SemVer, lexicographic tie-break on id) does not depend on it (operator decision, 2026-10-16)
    decided-at: 2026-10-16T02:24:44Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says BSD and GNU sort both support -V
    outcome: discarded
    reason: rationale for a SemVer-compare option the plan offered; the shipped loop compares in the bootstrap prose and no script depends on sort -V (operator decision, 2026-10-16)
    decided-at: 2026-10-16T02:24:44Z
    decided-by: andy@stone.dev
  - key: grounding — plan.md says /audit invokes the audit orchestrator via run-generator
    outcome: discarded
    reason: describes the wiring the plan designed against, in the design record; it holds, per framework/commands/audit.md step 1 (operator decision, 2026-10-16)
    decided-at: 2026-10-16T02:24:44Z
    decided-by: andy@stone.dev
---

# Analysis — 027-bootstrap-migration-registry

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 10 unexamined target(s). Dispositions: 0 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

- not-a-live-claim: 4
- root-absent: 5
- ships-to-adopter: 1

## Fixed in this run

*None.*
