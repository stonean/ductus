---
section: "Record freshness"
---

# A-done-specs-plan-edit-leaves-its-analysis-current

## Context

[041 — Spec Directory Pruning](../../041-task-pruning/spec.md)'s `a-plan-edit-does-not-reopen-a-done-spec` made an edit confined to a `done` spec's `plan.md` or `tasks.md`, adding no unchecked task, an edit that takes no back-edge under §spec-lifecycle. The analyze record's freshness still compared both files. A `done` spec whose plan was corrected therefore stayed `done` with a stale analysis, and the stale row asked for a re-run that could re-verify nothing, since the spec cannot be reopened for the edit. That was the reopen cost the plan rule removed, moved into a record.

## Behavior

- **On a `done` spec, analyze freshness does not compare `plan.md` or `tasks.md`.** `analyze_subjects::analyze_freshness` reads the spec's `status`, and for `done` judges the record on every other analyze subject. A plan or tasks edit leaves the record current. An edit to `spec.md`, a scenario, `data-model.md` or `review.md` still stales it.
- **A spec short of `done` keeps both in the comparison.** The pre-`done` gate reads its analysis against them, so an edit there after the analysis still stales it.
- **The recorded digest is unchanged.** `write-analysis` still digests every analyze subject, `plan.md` and `tasks.md` included, so a spec reopened later is judged against what its last run read.
- **A recorded path the comparison no longer counts is not a deleted subject.** On a `done` spec, a `plan.md` the record digested and the tree no longer holds is a plan edit too, and does not stale the record.

## Edge Cases

- **A spec whose `spec.md` does not parse.** Its status is unknown, so it is not `done`, and both files stay in the comparison: the stricter reading.
- **The status flip that closes a spec.** It edits `spec.md`, so it still stales the record written just before it, as it did before this change; nothing gates a `done` spec's analysis freshness, and `/{project}:review`'s analyze row reports it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
