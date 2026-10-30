---
spec: 051-branch-scoped-spec-numbering
last-run: 2026-10-30T02:02:34Z
analyzed-against: 1525487937ed3f824f1a3cfef06c76c04c22da7c
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 0
analyzed-digest:
  data-model.md: c20019d89348ccca1008dacbf028c328aa0836c93eec6042f5f5c37c7ba9448e
  plan.md: 51186cb570887f97dfd357337277d3ccb44c6376e36dbf0555811e5c9b1ec0e6
  review.md: 07491611471d62ec036aedf68b2cc1307b628c2f2ebc2af32e32f550cf687b08
  scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md: 6ae5b1d58174acda9aadcc1d96024e65f28eea998dd2fc857a84616991c9bc5b
  scenarios/a-fold-target-is-held-to-the-name-grammar.md: d2f77d8a21f7353c434f5f29cf9e47a144581ccc3ee49787bb9a505a6461dc5b
  scenarios/a-link-inside-the-target-stays-inside-it.md: a367c3a7c1e260f98b27eb73a87958625422a4e458b758399fd5fa526eb2e1a4
  scenarios/a-pending-fold-never-holds-done.md: 602c9f86e24d0a2cfa9cdebf78bf3d32e09255b00b189cd882d89d99e433e4b2
  scenarios/fold-target-checked-before-the-rewrite.md: 40bfa2011fb9af497ae85dac6f23f2ad8719dddcc8c5f5654dda7560aade7f32
  scenarios/rewrites-preserve-line-endings.md: 1ab09161757ade0d1f4a3959f574e54d68dab19f275ec84b345b8d16a75ccd26
  scenarios/the-numbering-grammar-reaches-every-surface.md: d4f11334754c65e3cf73346d224aaf8d782f5385a266a2bf22d3339a1079089a
  scenarios/the-spec-root-is-read-before-the-first-write.md: cd12facf1f7af5a4d7a241a909fe34f69fd1bc02984e066e80bc55e7b3be9106
  spec.md: 9384f1834b5289684104f1f6897ca1986e03830d0672f5c631d204ea30988673
  tasks.md: 9a2c5ddb8065674dc0508b70eca235329255fee9efbcd089cd92848b8ec75838
blocking: false
dispositions:
  fixed: 2
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Analysis — 051-branch-scoped-spec-numbering

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 0 unexamined target(s). Dispositions: 2 fixed, 0 routed, 0 discarded, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Unexamined targets

*None — every target was examined.*

## Fixed in this run

- rule-assessment — BE-RACE-003: spec.md's edge case says two contributors creating under one identifier at once are safe because the second is refused, though only an identical name is refused: two different names both succeed with the same number, which the spec neither states nor calls harmless — `specs/051-branch-scoped-spec-numbering/spec.md` — **fixed**
- grounding — plan.md's known limitation says in-repo links are re-pointed before retirement, so the exposure is outside the repository, uncited and broader than the files fold-back reads — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed**
