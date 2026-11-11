---
spec: 043-workflows-sunset
last-run: 2026-11-11T16:06:10Z
analyzed-against: 03d18ed6eb61492a50f71577409d5b90b28c1b3b
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 14
analyzed-digest:
  plan.md: b3098f6079c1fe146d37669e6c258c30f5f34fbe937de93255eab3423be06b71
  review.md: 09aaa0b7b3fac4de426954ebcdf84bc82a7d6a916e637e578ef78ecfe7170577
  spec.md: 5d2755732d7f8bd0e8277cdb64143fb3c2658623d8f43e1039635b0fe5425658
  tasks.md: 83e425359d89226b719156baa0c8abd79969d63f4bebeabe853cd8e4f4e10492
unexamined-by-reason:
  never-existed: 1
  not-a-live-claim: 12
  root-absent: 1
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 1
  undispositioned: 0
decisions:
  - key: "body-path-existence — names `framework/workflows`, which existed in this repository and no longer resolves: \"The acceptance sweep greps live artifacts (`framework/`, `scripts/`, `runtime/`, `.github/`, `README.md`, `AGENTS.md`, `.claude/commands/`, spec bodies) for feature tokens — `framework/workflows`, `workflows/registry.json`, `declined_categories`, `[workflows]`, `Workflow recommendation`, `workflow-filename-rename`, `skills-to-workflows`, `:workflows:` — and classifies each remaining hit as (a) archived-recipe text in `CHANGELOG.md`, (b) sunset annotation, (c) GitHub Actions path, (d) generic prose, or (e) 043's own artifacts; anything else fails the sweep. The prose-claim pass then greps by meaning (\"scaffold\", \"recommend\", \"declined\") for behavioral claims token greps miss.\""
    outcome: waived
    reason: "Correct text: `framework/workflows` is one of the sweep's grep tokens — a search term the plan lists, not a claim the path exists. Operator-approved discard (2026-10-16)."
    decided-at: 2026-10-16T19:38:46Z
    decided-by: andy@stone.dev
---

# Analysis — 043-workflows-sunset

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 14 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 1 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- body-path-existence — names `framework/workflows`, which existed in this repository and no longer resolves: "The acceptance sweep greps live artifacts (`framework/`, `scripts/`, `runtime/`, `.github/`, `README.md`, `AGENTS.md`, `.claude/commands/`, spec bodies) for feature tokens — `framework/workflows`, `workflows/registry.json`, `declined_categories`, `[workflows]`, `Workflow recommendation`, `workflow-filename-rename`, `skills-to-workflows`, `:workflows:` — and classifies each remaining hit as (a) archived-recipe text in `CHANGELOG.md`, (b) sunset annotation, (c) GitHub Actions path, (d) generic prose, or (e) 043's own artifacts; anything else fails the sweep. The prose-claim pass then greps by meaning ("scaffold", "recommend", "declined") for behavioral claims token greps miss." — `specs/043-workflows-sunset/plan.md` — **waived**: Correct text: `framework/workflows` is one of the sweep's grep tokens — a search term the plan lists, not a claim the path exists. Operator-approved discard (2026-10-16).

## Deferred to other specs

*None.*

## Unexamined targets

- never-existed: 1
- not-a-live-claim: 12
- root-absent: 1

## Fixed in this run

- rule-assessment — BE-INPUT-004: the workflows-sunset procedure deletes paths built from {config_dir} and {project}, and the plan stated no screen of those values — `specs/043-workflows-sunset/plan.md` — **fixed** (decided by the rule)
