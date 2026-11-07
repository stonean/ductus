---
spec: 023-govern-refinement
last-run: 2026-11-07T23:45:39Z
analyzed-against: 3bbee201be004d45f652ba30cf62ac63b4c80b31
hard-fail: 0
blocking-findings: 0
advisory: 2
unexamined: 2
analyzed-digest:
  plan.md: 09a50336cdd8edc2261073f295243006797c38cf8be670e5c4d9e23f09a34ded
  review.md: cfbe5d046e077c070a9ae160c124c7a605cd43690815f0d1b623f75c5d6f8d93
  scenarios/configure-dedup-permissions.md: e7706e99a8d13e53cefd7d392da70c91038774d2daab0472a7c82d453e2451d5
  scenarios/configure-inert-write-path-entries.md: fb2b3383713428e9006dff42edbb89a6bab99995f612161eefee69412c757f9d
  scenarios/configure-permission-pattern-safety.md: 6c6f71741c7fdfcd96b9f6017b649eb6396dd0edc6b90944372f3078d12b18f0
  scenarios/configure-retires-formerly-canonical-entries.md: ef54224f1b004bf5ee9d4342159f7de06c0d3b5dddea7f2296fe44d5a5d2a920
  scenarios/extend-existing-scenario-task.md: 1a0b674ce341509402b4a598e791bbed510e2f758be5e646786683340927155a
  scenarios/living-specs.md: 22074e14ac4eb3f659b7d4ef362019e1ce7af18d32ef7a1509434ecd6d29c825
  spec.md: 6a311af0210384229d8518c6c8c8ecb2ddf4ca64f9bbe19a808eca6a2cb22a74
  tasks.md: b5665b23b9ba6599ddbd3d2b48e68c48d6b43d886841d7026c40215d76c05b81
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 2
  undispositioned: 0
decisions:
  - key: grounding — plan.md states that scripts/gen-help-tables.sh's existing splice logic handles marker renames without code changes, citing no line of the script
    outcome: waived
    reason: a design-time claim about a script this spec's task 13 then changed and ran in sync; grounding it means reading the script, which this command does not do, so it is no chore
    decided-at: 2026-11-07T23:45:39Z
    decided-by: rule
  - key: grounding — spec.md states that GitHub Spec Kit assigns /analyze the artifact-consistency role and that its name has become the emerging-standard term, citing no source
    outcome: waived
    reason: a claim about an outside project's naming, given as the rename's motivation; grounding it means fetching that project, which this command does not do, and the rename stands on its own
    decided-at: 2026-11-07T23:45:39Z
    decided-by: rule
---

# Analysis — 023-govern-refinement

## Summary

0 hard-fail, 0 blocking, 2 advisory; not blocking. 2 unexamined target(s). Dispositions: 3 fixed, 0 deferred, 2 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- grounding — plan.md states that scripts/gen-help-tables.sh's existing splice logic handles marker renames without code changes, citing no line of the script — `specs/023-govern-refinement/plan.md` — **waived**: a design-time claim about a script this spec's task 13 then changed and ran in sync; grounding it means reading the script, which this command does not do, so it is no chore (decided by the rule)
- grounding — spec.md states that GitHub Spec Kit assigns /analyze the artifact-consistency role and that its name has become the emerging-standard term, citing no source — `specs/023-govern-refinement/spec.md` — **waived**: a claim about an outside project's naming, given as the rename's motivation; grounding it means fetching that project, which this command does not do, and the rename stands on its own (decided by the rule)

## Deferred to other specs

*None.*

## Unexamined targets

2 target(s) unexamined; no breakdown recorded.

## Fixed in this run

- rule-assessment — BE-INPUT-004: plan.md plans create-scenario writing scenarios/{slug}.md from a caller-supplied slug without committing to a containment check — `specs/023-govern-refinement/plan.md` — **fixed** (decided by the rule)
- grounding — spec.md's Motivation states in the present tense that ductus exposes 14 user-visible slash commands across 5 categories, citing no source, a count of the surface this spec changed — `specs/023-govern-refinement/spec.md` — **fixed** (decided by the rule)
- grounding — spec.md section 6 states that /configure currently has no entries for the optional runtime's MCP tools and that adopters today hit the per-call prompt, which this spec's AC14 to AC16 then delivered — `specs/023-govern-refinement/spec.md` — **fixed** (decided by the rule)
