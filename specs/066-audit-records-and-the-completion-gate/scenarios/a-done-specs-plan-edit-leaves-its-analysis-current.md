---
section: "Record freshness"
---

# A-done-specs-plan-edit-leaves-its-analysis-current

## Context

[041 — Spec Directory Pruning](../../041-task-pruning/spec.md)'s `a-plan-edit-does-not-reopen-a-done-spec` made an edit confined to a `done` spec's `plan.md` or `tasks.md`, adding no unchecked task, an edit that takes no back-edge under §spec-lifecycle. The analyze record's freshness still compared both files. A `done` spec whose plan was corrected therefore stayed `done` with a stale analysis, and the stale row asked for a re-run that could not reopen the spec for the edit. That was the reopen cost the plan rule removed, moved into a record.

## Behavior

- **On a `done` spec, analyze freshness ignores an in-place edit to `plan.md` or `tasks.md`.** `analyze_subjects::analyze_freshness` reads the spec's `status`, and for `done` drops those two from the changed paths while the files exist. A plan or tasks edit leaves the record current. An edit to `spec.md`, a scenario, `data-model.md` or `review.md` still stales it.
- **A spec short of `done` keeps both in the comparison.** The pre-`done` gate reads its analysis against them, so an edit there after the analysis still stales it.
- **The recorded digest is unchanged.** `write-analysis` still digests every analyze subject, `plan.md` and `tasks.md` included, so a spec reopened later is judged against what its last run read.
- **A deleted `plan.md` or `tasks.md` still stales it.** Analyze reports a missing one at `planned` or later as blocking, so only an edit in place is ignored.

## Edge Cases

- **A spec whose `spec.md` does not parse.** Its status is unknown, so it is not `done`, and both files stay in the comparison: the stricter reading.
- **The status flip that closes a spec.** It edits `spec.md`, so it still stales the record written just before it, as it did before this change; nothing gates a `done` spec's analysis freshness, and `/{project}:review`'s analyze row reports it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
