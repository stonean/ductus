---
section: "The review record"
---

# A-commit-after-the-scope-was-resolved-refuses-the-write

## Context

An adopter on 0.61.0 reported that the review scope moves under a running review. `write-review` re-derives the scope at write time (`resolve_scope` runs `compute-review-scope` against the run's `diff-base` and the current tree), so a commit landing between the run's `compute-review-scope` and its `write-review` joins a scope the passes never saw. Since spec 081 a commit naming no spec is credited as read in every other window once a committed record whose window, `diff-base` to `reviewed-against`, holds it leaves its path out of `unread`: a host that passes the write-time HEAD as `reviewed-against` credits a path nobody read.

## Behavior

- `compute-review-scope`'s result carries `head`: the HEAD its committed part was resolved against.
- The review records that sha as `reviewed-against`: `/{project}:review` passes the `head` of the scope its passes were briefed with, so the record's window ends where what the passes saw ends.
- `write-review` refuses, writing nothing, when a commit after `reviewed-against`, up to the current HEAD, changed a path its derived scope holds. The refusal's size is bounded whatever moved: it names how many scope paths moved, the first three, each with up to three of the commits that changed it, then how many paths more, and the command that lists every one. The run re-runs `compute-review-scope`, reads each moved path and each path the new scope adds, or lists it in `unread`, and writes again with the new `head`: a moved path the scope already held is not added by the re-run, so reading only the added paths would credit content no pass read.
- A commit after `reviewed-against` that changed no path in the scope does not refuse the write.
- A `reviewed-against` naming no commit, or one HEAD does not descend from, is an argument error, never read as no movement.
- With an empty `diff-base`, a spec never `in-progress`, none of these checks runs: the scope is empty, so no path in it can move, and a record whose `diff-base` names no commit credits nothing (the agent's recommendation, adopted by the operator, 2026-11-22).

## Edge Cases

- Uncommitted work is in the scope at write time and is never credited (credit reads commits), so another session's uncommitted edit is no refusal.
- A path the passes read uncommitted that another session committed mid-review refuses: the committed content may differ from what was read.
- A commit after the write and before the record's own commit is outside this check; the record's window ends at `reviewed-against`, so nothing credits it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
