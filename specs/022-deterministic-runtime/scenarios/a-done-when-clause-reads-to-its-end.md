---
section: "The primitive library"
---

# A-done-when-clause-reads-to-its-end

## Context

`read-tasks` records a task's `Done when` clause from the bullet's first line only. Repro (0.59.11 and HEAD f24c55c6, 2026-10-25): a tasks.md task `## 99. Probe` with `- [ ] one` and `- **Done when**: first line` followed by an indented continuation `second line` returns `"done-when": "first line"`. A clause wrapped across lines — as long completion conditions are — reaches every reader of `done-when` truncated. Scenario `done-when-authoring-forms` settled which label forms are a clause, not where a clause ends.

## Behavior

- A `Done when` clause's body is its first line joined with each following indented continuation line of the same bullet, in order, separated by single spaces.
- The clause ends at the first line that is not such a continuation: a new bullet at any depth, a blank line, a heading, or a less-indented line.
- `mark-task`, which shares the clause recognizer, treats the continuation lines as part of the clause and never as subtasks, so the read/mark index contract holds.

## Edge Cases

- A single-line clause: unchanged.
- A continuation line that itself opens with a list marker: a bullet, so it ends the clause.
- The bulletless form `Done when: …` followed by an indented line: the same joining rule.
- A continuation inside an HTML comment: ignored, as the shared tasks.md parsers ignore comment content.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
