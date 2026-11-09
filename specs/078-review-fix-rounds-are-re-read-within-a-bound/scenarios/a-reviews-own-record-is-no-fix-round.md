---
section: "The scope is derived"
---

# A-reviews-own-record-is-no-fix-round

## Context

`/{project}:review` takes its first fix-round base in step 10, before any fix, and step 11's `write-review` then rewrites the feature's `review.md`, so step 12's first `brief-re-read` found it changed. `review.md` is a markdown file in the reviewed spec's own directory, so the round was `exempt` (`runtime/src/primitives/brief_re_read.rs`, `classify`), and a run that fixed nothing recorded `fix-rounds: 1`. That contradicted this spec's Edge Case, as it then read, that a run whose findings were all deferred or waived has no fix round. A confirmed deferral's write — a task or scenario, here or in another spec — lands in step 10 too, inside the same round. This spec's first review (2026-11-04) found both, and the owner routed them here (2026-11-04).

## Behavior

- **The reviewed feature's own `review.md` is never a fix round's path.** `brief-re-read` and `check-prescriptions` leave it out of the paths they derive, since it is the command's own output, regenerated whole on every run. A run that fixed and deferred nothing therefore finds round 1 `empty` and records `fix-rounds: 0`, unless another spec's commit lands during it, which makes the round `exempt` (spec 079).
- **A deferral's write is a round's path like any other write.** It is a written requirement, so a path is `exempt` when it is a markdown file in the reviewed spec's own directory, other than `analysis.md`, that is not a durable contract, and re-read otherwise — a scenario, or anything in another spec's directory.
- **This spec's Edge Case is restated to match**: a run whose findings were all waived has no fix round; one with deferrals has the rounds their writes make.
- **The `[review] re-reads-off` reason is printed on every run** (AC14), round 1 `empty` included: `brief-re-read` reports it whatever the round's kind, and the host prints it before it stops.

## Edge Cases

- **Another session edits the feature's `review.md` between the base and a round.** It is still left out: a round never re-reads the record, whoever wrote it.
- **The record is the only change after a fix round.** The round after it is `empty`, so the loop ends.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
