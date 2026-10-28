---
spec: 051-branch-scoped-spec-numbering
last-run: 2026-10-28T20:22:39Z
analyzed-against: c1cf51ffa655635ef1b56b977023762f596c1aaa
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 7
analyzed-digest:
  data-model.md: 0c6bb89482f5cb6b361a487471de563db41f13f31f8de867cee50161b52b4ee9
  plan.md: 02a01f32b02dbe1cfcebc75ef4b5d10d450f6fd12073c631ebd128889787135a
  review.md: a479871aca6988706a57c9d0c88130f610a9194976fe4021a805e137922fe5bc
  scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md: 6ae5b1d58174acda9aadcc1d96024e65f28eea998dd2fc857a84616991c9bc5b
  scenarios/a-pending-fold-never-holds-done.md: 602c9f86e24d0a2cfa9cdebf78bf3d32e09255b00b189cd882d89d99e433e4b2
  scenarios/fold-target-checked-before-the-rewrite.md: 40bfa2011fb9af497ae85dac6f23f2ad8719dddcc8c5f5654dda7560aade7f32
  scenarios/rewrites-preserve-line-endings.md: 1ab09161757ade0d1f4a3959f574e54d68dab19f275ec84b345b8d16a75ccd26
  scenarios/the-numbering-grammar-reaches-every-surface.md: d4f11334754c65e3cf73346d224aaf8d782f5385a266a2bf22d3339a1079089a
  spec.md: 38a2cd297e57369b8b332cd1ce0f14cd226392fe71835e09da7f6e1217b50793
  tasks.md: ebcd446975320fcfc86a404f035d5d28d88e6c8f916d9f587a6e1b6752d4cf46
unexamined-by-reason:
  not-a-live-claim: 5
  root-absent: 1
  ships-to-adopter: 1
blocking: false
dispositions:
  fixed: 4
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 051-branch-scoped-spec-numbering

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 7 unexamined target(s). Dispositions: 4 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

- not-a-live-claim: 5
- root-absent: 1
- ships-to-adopter: 1

## Fixed in this run

- rule-assessment — BE-INPUT-004: plan.md takes host-supplied feature names into paths it creates, rewrites and removes, committing them to a name grammar only and not pointing at 022's plan Path arguments — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed**
- rule-assessment — BE-PAYLOAD-001: data-model.md's rewrite-spec-links and check-unfolded-specs results return collections with no stated maximum and no pointer to 022's project-bounded result statement — `specs/051-branch-scoped-spec-numbering/data-model.md` — **fixed**
- rule-assessment — BE-RACE-003: plan.md says concurrent creation under one identifier is safe because the loser is refused, naming no mechanism that makes the existence check and the creation one step — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed**
- grounding — plan.md says specs/system.md does not exist in this repository, naming no command — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed**
