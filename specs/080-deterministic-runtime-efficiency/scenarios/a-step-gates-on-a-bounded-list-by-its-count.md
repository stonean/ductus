---
section: "Behavior"
---

# A-step-gates-on-a-bounded-list-by-its-count

## Context

A bounded result past `[artifacts] read-size-bytes` has `form: "file"`: each moved list is absent, its `{list}-count` stays inline, and `lists-file` holds the lists whole (AC7). Steps written for the inline form said "an empty `unfolded`" or "an empty `rewritten`", "a non-empty `updated`", "its open-questions list carries the count", and read `tasks`, `specs`, `sections` or `findings` without naming the file, so in file form a list that is absent reads as an empty one.

## Behavior

- A command step that asks whether a bounded list is empty reads the list's `-count`, which both forms carry.
- A command step that reads a bounded list's contents says to read them from the file `lists-file` names when `form` is `file`, in the phrase the existing steps use.
- A test fails when a command step states a bounded list's emptiness by the list rather than its count.

## Edge Cases

- A step that already names the file form, or needs only a count, is unchanged.
- `check-review-gate`'s `unchecked-task` stays the inline test that a spec with an unchecked box does not reach `done`, whatever form `read-tasks` returned.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
