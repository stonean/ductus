---
spec: 002-project-scaffolding
last-run: 2026-11-11T15:45:25Z
analyzed-against: 9918f26071c97838466721df302db24e04a8d26a
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 6
analyzed-digest:
  plan.md: 642bda2e6db11996d8e98ec024f49b277821c3c7b6b0504a9785c5d2f6ffa301
  review.md: a75680a50b17a01f02dab1d0769c4cd14cbbbfd831b78e9df2e8c840c835a43f
  scenarios/claude-md-template-imports-load.md: 492ad4f5d1fd09e471feaf9ced0a8b7ba018c09e6a18c169b502dc258f8c530f
  spec.md: dc2dc65d753ba93217fddf89bca62ea72f4b690750bc6577d3747a65f82dffa3
  tasks.md: 29a39e1a8ef8d870df9a57b8f4674177155b2d316c161914c96cf2a317eb9ec9
unexamined-by-reason:
  root-absent: 4
  ships-to-adopter: 2
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 002-project-scaffolding

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 6 unexamined target(s). Dispositions: 1 fixed, 0 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Unexamined targets

- root-absent: 4
- ships-to-adopter: 2

## Fixed in this run

- grounding — plan.md says the init command (spec 003) copies the templates into new projects and renames gitignore during the copy, where /ductus:init was retired on 2026-09-15 and /ductus installs the gitignore block and CLAUDE.md — `specs/002-project-scaffolding/plan.md` — **fixed** (decided by the rule)
