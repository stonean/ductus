---
spec: 051-branch-scoped-spec-numbering
last-run: 2026-11-12T00:32:34Z
analyzed-against: ab2ad0853b359dfe50598581bc9302daf093ef2a
hard-fail: 0
blocking-findings: 0
advisory: 0
unexamined: 9
analyzed-digest:
  data-model.md: c20019d89348ccca1008dacbf028c328aa0836c93eec6042f5f5c37c7ba9448e
  plan.md: 93412ca8dfe6de114efe0a2d9722863184baa8da2b8cc47d0171ba52aa81fc0a
  review.md: 07491611471d62ec036aedf68b2cc1307b628c2f2ebc2af32e32f550cf687b08
  scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md: 015b0ba736e27e91d0d86b37c65e6f7d0d947b5773cf6495313a8ff337b93a47
  scenarios/a-fold-target-is-held-to-the-name-grammar.md: d2f77d8a21f7353c434f5f29cf9e47a144581ccc3ee49787bb9a505a6461dc5b
  scenarios/a-link-inside-the-target-stays-inside-it.md: a367c3a7c1e260f98b27eb73a87958625422a4e458b758399fd5fa526eb2e1a4
  scenarios/a-pending-fold-never-holds-done.md: dc5c2daa3fd78c7251edf856c21f5d36737e3725d91c808231e112f84502e8b5
  scenarios/fold-target-checked-before-the-rewrite.md: 40bfa2011fb9af497ae85dac6f23f2ad8719dddcc8c5f5654dda7560aade7f32
  scenarios/rewrites-preserve-line-endings.md: 1ab09161757ade0d1f4a3959f574e54d68dab19f275ec84b345b8d16a75ccd26
  scenarios/the-numbering-grammar-reaches-every-surface.md: d4f11334754c65e3cf73346d224aaf8d782f5385a266a2bf22d3339a1079089a
  scenarios/the-spec-root-is-read-before-the-first-write.md: cd12facf1f7af5a4d7a241a909fe34f69fd1bc02984e066e80bc55e7b3be9106
  spec.md: e3b76ece780b50f64a911f5e9d532112395bfe14f80b0dfebddad6ad26872a77
  tasks.md: 9a2c5ddb8065674dc0508b70eca235329255fee9efbcd089cd92848b8ec75838
unexamined-by-reason:
  not-a-live-claim: 5
  root-absent: 2
  ships-to-adopter: 2
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Analysis — 051-branch-scoped-spec-numbering

## Summary

0 hard-fail, 0 blocking, 0 advisory; not blocking. 9 unexamined target(s). Dispositions: 3 fixed, 0 deferred, 0 waived, 0 undispositioned.

## Hard failures

*None.*

## Blocking findings

*None.*

## Advisory findings

*None.*

## Deferred to other specs

*None.*

## Unexamined targets

- not-a-live-claim: 5
- root-absent: 2
- ships-to-adopter: 2

## Fixed in this run

- grounding — plan.md §Fold-back and §Trade-offs, and spec.md's Fold-back-interrupted edge case: "the runtime provides per-file atomic writes, not multi-file transactions" cites no source — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed** (decided by the rule)
- grounding — plan.md §Reopening: "the same guarded call /ductus:groom and /ductus:amend already make" cites neither command — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed** (decided by the rule)
- decision-resolution — plan.md §Reopening explains the explicit invalidation by a staleness check reading durable contracts, which spec 081 retired (§decision-resolution: the plan still describes the prior state) — `specs/051-branch-scoped-spec-numbering/plan.md` — **fixed** (decided by the rule)
