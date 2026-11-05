---
section: "The warning names a fix"
---

# A-plan-trim-does-not-reopen-a-done-spec

## Context

[041 — Spec Directory Pruning](../../041-task-pruning/spec.md)'s `a-plan-edit-does-not-reopen-a-done-spec` made an edit confined to a `done` spec's `plan.md` or `tasks.md`, adding no unchecked task, an edit that takes no back-edge under §spec-lifecycle. `check-artifact-size` still told the operator of an oversized `done` plan that trimming it "reopens this done spec unless the trim changes no claim". That was the old rule, under which a plan correction took the back-edge. A plan trim now never does, so the advice named a cost that no longer exists.

## Behavior

- **A `plan.md` trim on a `done` spec does not reopen it.** The fix's `on-done` effect is `never`, and the warning says "does not reopen this done spec" beside the trim.
- **Every other effect follows `classify-spec-edit`.** Splitting and promoting reopen the spec, pruning `tasks.md` does not, trimming `spec.md` reopens it only if the trim changes a criterion or an open question, and trimming `data-model.md` always does.

## Edge Cases

- **An oversized `tasks.md`.** Its trim fix trims `spec.md`, not the task list, so it takes `if-requirement-changes`.
- **A spec short of `done`.** No fix carries an `on-done` effect, as before.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
