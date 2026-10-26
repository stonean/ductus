---
spec: 027-bootstrap-migration-registry
last-run: 2026-10-26T17:36:02Z
analyzed-against: c0952038a758b7a36c6c57ebb856367fd7655085
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 10
analyzed-digest:
  plan.md: 77b8d64b02840376ed8ad8f2abce36b813a9ea2f5bd6b47849d401343ae88781
  review.md: 29bbdaa1535b07a7dc3731ba10e21eba42cf423243d90d11f7254e69f86b4a43
  scenarios/migration-chain-reference-integrity.md: b187024e6ed8fee90fe31be0f2bbcd4786782f8e4395a89c09142f20218f914d
  scenarios/migrations-apply-once-per-repo.md: ef57af25a1eb071076c60b005ffdbf9f3a1b2afad889ba939b230679b9d4f9b4
  spec.md: 22446f6225641a473ed72a13a33a9596629e3bb419a894211f955f400683a0ba
  tasks.md: ff144324ac4ec523c5ef57afaf0922196affd4889c659017111d8d1683aaf360
unexamined-by-reason:
  not-a-live-claim: 4
  root-absent: 5
  ships-to-adopter: 1
blocking: false
dispositions:
  fixed: 1
  routed: 0
  discarded: 4
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

0 hard-fail, 0 blocking, 4 advisory; not blocking. 10 unexamined target(s). Dispositions: 1 fixed, 0 routed, 4 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- plan-record — plan.md §Known Limitations is outside the design record — `specs/027-bootstrap-migration-registry/plan.md` — **discarded**: deliberately part of this plan's design: it records the v1 limits the registry shipped with (uniform back-fill sunsets, discipline-only removal coverage and CHANGELOG append, a loosely specified procedure-file shape), the case analyze's Plan record section names as a discard (operator decision, 2026-10-16)
- grounding — spec.md and plan.md say TOML parsers don't guarantee array-of-tables order preservation across rewrites — `specs/027-bootstrap-migration-registry/plan.md` — **discarded**: rationale for the resolved Registry-order decision in the design record; the order rule it supports (introduced_in by SemVer, lexicographic tie-break on id) does not depend on it (operator decision, 2026-10-16)
- grounding — plan.md says BSD and GNU sort both support -V — `specs/027-bootstrap-migration-registry/plan.md` — **discarded**: rationale for a SemVer-compare option the plan offered; the shipped loop compares in the bootstrap prose and no script depends on sort -V (operator decision, 2026-10-16)
- grounding — plan.md says /audit invokes the audit orchestrator via run-generator — `specs/027-bootstrap-migration-registry/plan.md` — **discarded**: describes the wiring the plan designed against, in the design record; it holds, per framework/commands/audit.md step 1 (operator decision, 2026-10-16)

## Unexamined targets

- not-a-live-claim: 4
- root-absent: 5
- ships-to-adopter: 1

## Fixed in this run

- spec-requirements — spec.md carries a point-in-time Completion record (2026-08-17) reading Closed. while the frontmatter reads in-progress — `specs/027-bootstrap-migration-registry/spec.md` — **fixed**
