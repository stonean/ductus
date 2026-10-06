---
spec: 065-configurable-source-repository
last-run: 2026-10-06T14:34:15Z
analyzed-against: d0c979a2c96b0ecdd27bc819c5725a7e10ea7ce0
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 0
analyzed-digest:
  plan.md: 4364a3c519b5241e198def02edecebcd0d5215cadba66ec3baff88ae62caca27
  review.md: cf62af9c1b034a9c6b673e364816937c855685fd682999a00d9d362c7f9b9d8f
  scenarios/a-non-canonical-origin-is-announced.md: caf57cb05fd97724d2f3d158f7a1ea3a9010664a018c4d01f089dbdfb7736b01
  spec.md: ed836c429e30aeb20d019e6b3c18bc830e3988295aa02b906c3eb946f89fecb6
  tasks.md: ca3976d974b951be47a2235f0d69318351993051c6edb687058ab273499d026a
blocking: false
dispositions:
  fixed: 5
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 065-configurable-source-repository

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 0 unexamined target(s). Dispositions: 5 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

- rule-assessment — CFG-ENV-001: the spec introduces DUCTUS_REPO without committing to read-once resolution or naming its default as a constant — `specs/065-configurable-source-repository/spec.md` — **fixed**
- rule-assessment — CFG-ENV-002: the spec introduces DUCTUS_REPO without naming the environment-variable inventory or carrying its update as a task — `specs/065-configurable-source-repository/spec.md` — **fixed**
- grounding — Behavior names four exec-time fetch sites; the bootstrap names DUCTUS_REPO at eight fetch commands — `specs/065-configurable-source-repository/spec.md` — **fixed**
- grounding — Behavior and Resolved Questions say self-url-resolution derives from `stonean/ductus/archive/…` with no family change; it reads the codeload tar.gz URL and its pattern was extended to unwrap the DUCTUS_REPO default — `specs/065-configurable-source-repository/spec.md` — **fixed**
- grounding — plan.md Overview, D1, D3 and D5 state the pre-061 model: /main/ fetch URLs, an archive regex the family does not use, and a live-on-main ship — `specs/065-configurable-source-repository/plan.md` — **fixed**
