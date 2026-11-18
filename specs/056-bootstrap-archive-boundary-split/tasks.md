---
next-task: 11
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
