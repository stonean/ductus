---
section: "The analyze record"
---

# A-write-result-states-what-it-wrote

## Context

An adopter on 0.61.0 reported that a structured write's result describes the call, not the artifact. `write-analysis` returns `blocking`, `unexamined`, `dispositions` and the waived rules, but not the `hard-fail`, `blocking-findings` and `advisory` it records, and it records the first two net of the live findings a deferral releases, so they can differ from what the host passed. `write-review` returns its violation counts, `dispositions`, `examined`, `scope` and the criteria counts, but not `excluded`, `unread` or the `decisions:` it keeps, so a decision it dropped shows only by reading `review.md` back. `append-task` already returns what landed: the task's number and whether it was appended.

## Behavior

- `write-analysis`'s result carries `hard-fail`, `blocking-findings` and `advisory` as the record holds them.
- `write-review`'s result carries `excluded`, the count the record holds; `unread-count`, the entries in the record's `unread`, absent when the run stated none; and `decisions`, the entries the record's `decisions:` holds after the write.
- Each value is the written record's, never an argument echoed back, so a result and its record cannot disagree.

## Edge Cases

- A refused write returns no result, so no count describes a record that was not written.
- The CLI prints the same result, so a host passing structured arguments as files reads the same fields.
- Counts only: no list is added, so the result's size bound is unchanged.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
