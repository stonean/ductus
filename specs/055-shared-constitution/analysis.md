---
spec: 055-shared-constitution
last-run: 2026-11-22T02:11:56Z
analyzed-against: 6478a2e5b84644356fa5e354c088b5af2448fb44
hard-fail: 0
blocking-findings: 0
advisory: 4
unexamined: 6
analyzed-digest:
  data-model.md: a8134b0d6cb8dff8f7cab2925bdf0843b17dfc794a5631f8f6c02ae9e7380c99
  plan.md: 461b492a7226e728d54fa76a028bb41f9d3245c9dc3ad57acb9dfd3eefa60ee4
  review.md: b47bb10c66440b822debf01f63a542f394ec86d58a4723d3cc28bc5546f30a59
  scenarios/a-registered-source-is-named-with-its-description.md: f04ffbef1f2349e9f23d8446c20490833b592ae5edb7dad11ed94bbe9c2166f3
  scenarios/target-reports-each-constitutions-size.md: 7661b1fccbf76a02af4467a2a40ebf845cfc302b6104f84b831ff25238b6bd3c
  spec.md: bc6515eacb40037087497d69d9308c0b4593568c112a5220c75945b8f17318c9
  tasks.md: 042ddb25085dd34107baadcde910d8b9e1c099ede622949b2dbf13c380656511
unexamined-by-reason:
  anchor-target-unjudgeable: 1
  root-absent: 3
  ships-to-adopter: 2
rule-digest:
  accessibility-frontend.md: 4d7f5ac09d457c99774f5fc341222a464d2b461cb0467cd58a13dc31c928359b
  api-backend.md: 8e496f92a50b68b1ae8b1954e30a85dd142dc65d3b3882089f9c48560096d800
  concurrency-backend.md: 2650d82125bd67945fd327f7fb9f76c57ca10d165925733139afa3cd50562dd4
  configuration-cross.md: 050a5b8dd375fe4790f5d92753db27ae53ac730db1557cbee84b923b72499ca9
  observability-backend.md: c36af8f004e95bafdb260d1a639ece53d64099855dfe6bd2e1754795d83be6b2
  performance-backend.md: f14131dc59688f068afb1c239aa5a6031b163bfac1ec702f2d28df808e26b814
  performance-frontend.md: 892bcab669f003264587558fe8234c3efcdfd835e93b38c9eeb07060f1e9e037
  quality-cross.md: fe166676f0783399c7c2fba6182c65784cd5c429b02d831911823cad1bf608a6
  reliability-backend.md: 26c045df79b6a99319bd7dfa09490b4a2d5ac742eee70a6b3ad0235d49093d2a
  security-backend.md: 511a68b25ceb148f8292cda2a829aaff03a38bc3453732e33a70bd5d6acf5df5
  security-cross.md: 26e287f06cdca2744d0972cbf7cf6a48f27d869526dd86c7c375ac4bdc0dadce
  security-frontend.md: 5fddb21530faa805cb00f23c929e9402548ed0d6a0ed3e8e9a910561b74a45a3
section-digest:
  "data-model.md": e3b0c44298fc1c14
  "data-model.md#055 — Shared Constitution Data Model": 22256b3f45aba1c4
  "data-model.md#Config schema": b07c47499e8a592e
  "data-model.md#Notes": bee14c8e129e03c5
  "data-model.md#Primitive result shape": 1b07ead688c8e717
  "data-model.md#Resolution outcomes": eebae528f673c086
  "plan.md": e3b0c44298fc1c14
  "plan.md#055 — Shared Constitution Plan": 348f1262112227e1
  "plan.md#AC8 is a runtime change, not a markdown edit": 8539eb4f4f93d083
  "plan.md#Affected Files": f2a50eedca6c676f
  "plan.md#Both bootstrap twins take every edit": 33caa80ac845f2de
  "plan.md#Data Model": 81f3857a79f9f907
  "plan.md#Editing `target.md` stales a parity golden": e46a383f0eaa96d9
  "plan.md#One new primitive: `resolve-constitutions`": b1c7e3edb5465b44
  "plan.md#Overview": b1ccc785b86c7e7d
  "plan.md#Precedence is documented, never enforced": 338eb2401fca1ac5
  "plan.md#Technical Decisions": ad85a1d406c0b799
  "plan.md#The managed block uses `merge-managed-block`": 5901be900f9cc159
  "plan.md#The registry reader is `load_services` with a different table name": d10f13fe5c04da72
  "plan.md#Trade-offs": 6e2c4e10922c6cdc
  "plan.md#Validation happens at configuration time, and adds no command": b351f3f7520d6c43
  "plan.md#`/ductus:target` loads what the rules file did not; the managed block imports under `claude-style`": 462cf32ea9ad486b
  "scenarios/a-registered-source-is-named-with-its-description.md": 512789d16be983fb
  "scenarios/a-registered-source-is-named-with-its-description.md#A-registered-source-is-named-with-its-description": feba7a6b7be887e3
  "scenarios/a-registered-source-is-named-with-its-description.md#Behavior": 6698b0b1bf47cc53
  "scenarios/a-registered-source-is-named-with-its-description.md#Context": ca9e40ea41588dca
  "scenarios/a-registered-source-is-named-with-its-description.md#Edge Cases": 071ddef6ccaf4460
  "scenarios/a-registered-source-is-named-with-its-description.md#Open Questions": 9fc5979d590cdb97
  "scenarios/a-registered-source-is-named-with-its-description.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/target-reports-each-constitutions-size.md": 512789d16be983fb
  "scenarios/target-reports-each-constitutions-size.md#Behavior": 5ba06a6f2ebd6ba3
  "scenarios/target-reports-each-constitutions-size.md#Context": a60b1722ebb76bdb
  "scenarios/target-reports-each-constitutions-size.md#Edge Cases": 631ed4c37ca9a56a
  "scenarios/target-reports-each-constitutions-size.md#Open Questions": 9fc5979d590cdb97
  "scenarios/target-reports-each-constitutions-size.md#Resolved Questions": b5f7b98fb3491576
  "scenarios/target-reports-each-constitutions-size.md#Target-reports-each-constitutions-size": 8bff27dd273e92cf
  "spec.md": ac4467a02031ad5c
  "spec.md#055 — Shared constitution": 67a4235b98807ec7
  "spec.md#Acceptance Criteria": c1999b3f3b2ef669
  "spec.md#Applicable Rules": 34fe38803e9de8e7
  "spec.md#Edge cases": cd9ab73b0a3de9c7
  "spec.md#Motivation": 3cfa42962d7283c8
  "spec.md#Non-Goals": 8914cfe34adddff6
  "spec.md#Open Questions": 14123c4c170fc722
  "spec.md#Resolved Questions": d6d17cd6ce54e676
  "spec.md#See also": 0114375ea2c6c987
  "spec.md#Shape of the setting": 0f02003383295ab4
rule-verdicts:
  BE-INPUT-001: {verdict: addressed, cite: "plan.md#Validation happens at configuration time, and adds no command"}
  BE-INPUT-002: {verdict: addressed, cite: "data-model.md#Config schema"}
  BE-INPUT-004: {verdict: waived, reason: "a registry `path` names a shared-constitution checkout outside the project by design, so no base directory exists to contain it; it is the operator's own configuration, and only the fixed `constitution.md` is joined to it", decided-by: "andy@stone.dev", decided-at: "2026-11-14T03:29:58Z"}
  BE-INPUT-008: {verdict: addressed, cite: "data-model.md#Config schema"}
  BE-PAYLOAD-001: {verdict: addressed, cite: "data-model.md#Primitive result shape"}
  CFG-ENV-001: {verdict: addressed, cite: "data-model.md#Config schema"}
  CFG-ENV-002: {verdict: addressed, cite: "plan.md#Affected Files"}
  CFG-ENV-003: {verdict: addressed, cite: "data-model.md#Config schema"}
  CFG-ENV-008: {verdict: addressed, cite: "data-model.md#Config schema"}
  CFG-ENV-012: {verdict: finding, tier: should, at: "data-model.md#Config schema", message: "The new [constitutions.<alias>] section does not state how its loader treats an unrecognized key (fail naming key and file, or warn naming it); only missing/empty/malformed repo, path and alias are specified."}
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: "grounding — Resolved Questions, 'Who validates a shared constitution?': states as fact that /ductus:audit is maintainer-only and checks this repository's cross-artifact drift, and that adopters get markdownlint and nothing more, with no citation to a command source or artifact."
    outcome: waived
    reason: "advisory and no chore: a Resolved Question's premise that `framework/commands/audit.md`'s own header states (maintainer-only, not scaffolded into adopter projects); the decision it supports, no validator of its own for a shared constitution, stands on 055's criteria"
    decided-at: 2026-11-22T02:11:56Z
    decided-by: rule
  - key: "grounding — Resolved Questions, 'What happens when the source is unreachable?': states as fact that a missing [services] checkout only downgrades a cross-service link to plain navigation, with no citation to 030 or the resolve-references primitive."
    outcome: waived
    reason: "advisory and no chore: a premise about 030's cross-service references, whose behavior 030's spec and `resolve-references`' compiled description own; the decision it supports, an unreachable source skipped and reported, stands on 055's own criteria"
    decided-at: 2026-11-22T02:11:56Z
    decided-by: rule
  - key: "grounding — Resolved Questions, 'Is the source pinned to a version?': states as fact that [services] takes the posture of recording no ref, with no citation to 030's data model or link.md."
    outcome: waived
    reason: "advisory and no chore: a premise about 030's `[services]` registry, whose schema 030's data model owns (the constitution's canonical-sources table); the decision it supports, recording no ref, is 055's own"
    decided-at: 2026-11-22T02:11:56Z
    decided-by: rule
  - key: "rule-assessment — CFG-ENV-012: The new [constitutions.<alias>] section does not state how its loader treats an unrecognized key (fail naming key and file, or warn naming it); only missing/empty/malformed repo, path and alias are specified."
    outcome: waived
    reason: "Unrecognized-key handling in .ductus/config.toml is project-wide runtime behaviour, not restated per section: the notice names a key inside a [constitutions.<alias>] table and the file, verified on a fixture (unknown config key: [constitutions.acme] descripton)."
    decided-at: 2026-11-22T02:11:56Z
    decided-by: rule
---

# Analysis — 055-shared-constitution

## Summary

0 hard-fail, 0 blocking, 4 advisory; not blocking. 6 unexamined target(s). Dispositions: 0 fixed, 0 deferred, 4 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — Resolved Questions, 'Who validates a shared constitution?': states as fact that /ductus:audit is maintainer-only and checks this repository's cross-artifact drift, and that adopters get markdownlint and nothing more, with no citation to a command source or artifact. — `specs/055-shared-constitution/spec.md` — **waived**: advisory and no chore: a Resolved Question's premise that `framework/commands/audit.md`'s own header states (maintainer-only, not scaffolded into adopter projects); the decision it supports, no validator of its own for a shared constitution, stands on 055's criteria (decided by the rule)
- grounding — Resolved Questions, 'What happens when the source is unreachable?': states as fact that a missing [services] checkout only downgrades a cross-service link to plain navigation, with no citation to 030 or the resolve-references primitive. — `specs/055-shared-constitution/spec.md` — **waived**: advisory and no chore: a premise about 030's cross-service references, whose behavior 030's spec and `resolve-references`' compiled description own; the decision it supports, an unreachable source skipped and reported, stands on 055's own criteria (decided by the rule)
- grounding — Resolved Questions, 'Is the source pinned to a version?': states as fact that [services] takes the posture of recording no ref, with no citation to 030's data model or link.md. — `specs/055-shared-constitution/spec.md` — **waived**: advisory and no chore: a premise about 030's `[services]` registry, whose schema 030's data model owns (the constitution's canonical-sources table); the decision it supports, recording no ref, is 055's own (decided by the rule)
- rule-assessment — CFG-ENV-012: The new [constitutions.<alias>] section does not state how its loader treats an unrecognized key (fail naming key and file, or warn naming it); only missing/empty/malformed repo, path and alias are specified. — `specs/055-shared-constitution/data-model.md` — **waived**: Unrecognized-key handling in .ductus/config.toml is project-wide runtime behaviour, not restated per section: the notice names a key inside a [constitutions.<alias>] table and the file, verified on a fixture (unknown config key: [constitutions.acme] descripton). (decided by the rule)

## Deferred to other specs

*None.*

## Waived rules

- BE-INPUT-004 (MUST) — a registry `path` names a shared-constitution checkout outside the project by design, so no base directory exists to contain it; it is the operator's own configuration, and only the fixed `constitution.md` is joined to it — decided by `andy@stone.dev` at `2026-11-14T03:29:58Z`

## Contradictions

*None.*

## Unexamined targets

- anchor-target-unjudgeable: 1
- root-absent: 3
- ships-to-adopter: 2

## Fixed in this run

*None.*
