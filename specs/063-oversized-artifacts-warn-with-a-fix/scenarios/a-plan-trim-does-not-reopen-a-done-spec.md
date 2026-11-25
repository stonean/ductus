---
section: "The warning names a fix"
---

# A-plan-trim-does-not-reopen-a-done-spec

## Context

[041 — Spec Directory Pruning](../../041-task-pruning/spec.md)'s `a-plan-edit-does-not-reopen-a-done-spec` made an edit confined to a `done` spec's `plan.md` or `tasks.md`, adding no unchecked task, an edit that takes no back-edge under §spec-lifecycle. `check-artifact-size` still told the operator of an oversized `done` plan that trimming it "reopens this done spec unless the trim changes no claim". That was the old rule, under which a plan correction took the back-edge. A plan trim now never does, so the advice named a cost that no longer exists. Spec 084 then replaced the per-fix effect with one rule, stated once.

## Behavior

- **A `plan.md` trim on a `done` spec does not reopen it.** It leaves no unchecked box in `tasks.md` and no unticked criterion in `spec.md`, and the warning names no reopen beside the trim.
- **The warning states the rule once.** After its fixes, a `done` spec's warning says that a fix reopens the spec only if it leaves an unchecked box in its `tasks.md` or an unticked criterion in its `spec.md`, and no fix names a reopen of its own.

## Edge Cases

- **An oversized `tasks.md`.** Its trim fix trims `spec.md`, not the task list, and the same one sentence follows its fixes.
- **A spec short of `done`.** The warning says nothing of reopening, as before.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
