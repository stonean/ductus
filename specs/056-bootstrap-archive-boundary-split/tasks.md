---
next-task: 12
---

# 056 — Bootstrap archive-boundary split Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 9. The walker places every archive-half section

- [x] Implement the behavior described in `scenarios/the-walker-places-every-archive-half-section.md`

Implemented in `824546a6` (landed on `main` 2026-11-12), `0f5be269` and `9a753a97`, and verified by the scratch-adopter bootstrap `579bd9b8` records.

- **Done when**: the scenario's described behavior is correctly implemented and verified by a scratch-adopter bootstrap walked in the stated order

## 10. Exercise the shared bootstrap window's review fixes in a scratch-adopter bootstrap

- [x] The walk takes 3a, 3b, 4a to 4e and 11 in order, each archive-half section at its place (367c8817)

- **Done when**: the batched run 003's matching task names shows each outcome, which the commit ticking this task states; a defect it finds is fixed first

## 11. Correct the criteria 056's pre-done verification found stale

- [x] AC11 is not met as written. It says plan.md states the reduction, but 063e50ea's prune moved the post-split measurement to the commit record: 146,953 to 110,417 bytes, 24.9%, recorded in 0e3a9c56 and 063e50ea (`git show ca85c8b8:framework/bootstrap/ductus.md | wc -c` gives 110,417). Annotate AC11 naming the figure and those commits
- [x] AC10 counts six `lint-*.sh` scripts, and there are seven now. State the set without a count
- [x] AC1's `no step that requires the archive` reads false of step 4's `apply-manifest` passes. It means no step that runs before extraction, so say that

- **Done when**: AC1, AC10 and AC11 state the tree, as corrections outside a reopen; then 056's analysis runs (scripts/analyze/), then its gate
