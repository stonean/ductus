---
spec: 043-workflows-sunset
last-run: 2026-10-16T19:38:46Z
analyzed-against: 2ce89f9d91c5ef1d03f1294506ade742e1aa8ef3
hard-fail: 0
blocking-findings: 0
advisory: 1
unexamined: 12
analyzed-digest:
  plan.md: 1629c40fac40d1043390c021e6b582464cec5a625a8d6088f4dea9a73729aab5
  review.md: 0cbe165c42b9fb0e32bdc5bab666596b747fb0225c906379087e192c5aaac3d8
  spec.md: 801068322a0b4181ce34bc289576bdc54252a83e738341492a7708575ec594a2
  tasks.md: 36badbc9407ecd8b544527a3f6803cad63e340098617816c1f43aa7a0afaa04f
unexamined-by-reason:
  never-existed: 1
  not-a-live-claim: 10
  root-absent: 1
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 1
  undispositioned: 0
decisions:
  - key: "body-path-existence — names `framework/workflows`, which existed in this repository and no longer resolves: \"The acceptance sweep greps live artifacts (`framework/`, `scripts/`, `runtime/`, `.github/`, `README.md`, `AGENTS.md`, `.claude/commands/`, spec bodies) for feature tokens — `framework/workflows`, `workflows/registry.json`, `declined_categories`, `[workflows]`, `Workflow recommendation`, `workflow-filename-rename`, `skills-to-workflows`, `:workflows:` — and classifies each remaining hit as (a) archived-recipe text in `CHANGELOG.md`, (b) sunset annotation, (c) GitHub Actions path, (d) generic prose, or (e) 043's own artifacts; anything else fails the sweep. The prose-claim pass then greps by meaning (\"scaffold\", \"recommend\", \"declined\") for behavioral claims token greps miss.\""
    outcome: discarded
    reason: "Correct text: `framework/workflows` is one of the sweep's grep tokens — a search term the plan lists, not a claim the path exists. Operator-approved discard (2026-10-16)."
    decided-at: 2026-10-16T19:38:46Z
    decided-by: andy@stone.dev
---

# Analysis — 043-workflows-sunset

## Summary

0 hard-fail, 0 blocking, 1 advisory; not blocking. 12 unexamined target(s). Dispositions: 0 fixed, 0 routed, 1 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

- body-path-existence — names `framework/workflows`, which existed in this repository and no longer resolves: "The acceptance sweep greps live artifacts (`framework/`, `scripts/`, `runtime/`, `.github/`, `README.md`, `AGENTS.md`, `.claude/commands/`, spec bodies) for feature tokens — `framework/workflows`, `workflows/registry.json`, `declined_categories`, `[workflows]`, `Workflow recommendation`, `workflow-filename-rename`, `skills-to-workflows`, `:workflows:` — and classifies each remaining hit as (a) archived-recipe text in `CHANGELOG.md`, (b) sunset annotation, (c) GitHub Actions path, (d) generic prose, or (e) 043's own artifacts; anything else fails the sweep. The prose-claim pass then greps by meaning ("scaffold", "recommend", "declined") for behavioral claims token greps miss." — `specs/043-workflows-sunset/plan.md` — **discarded**: Correct text: `framework/workflows` is one of the sweep's grep tokens — a search term the plan lists, not a claim the path exists. Operator-approved discard (2026-10-16).

## Unexamined targets

- never-existed: 1
- not-a-live-claim: 10
- root-absent: 1

## Fixed in this run

*None.*
