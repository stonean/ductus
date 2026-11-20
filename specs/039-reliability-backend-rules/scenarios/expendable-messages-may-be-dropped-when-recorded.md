---
section: "Acceptance Criteria"
---

# Expendable-messages-may-be-dropped-when-recorded

## Context

`BE-RETRY-005` bars discarding a retry-exhausted message silently and otherwise requires a dead-letter destination or a recorded halt. For a message whose loss costs nothing (a heartbeat, a superseded cache-refresh notice), retaining and redriving it is cost without value.

## Behavior

- A message the spec declares expendable MAY be dropped once its redelivery bound is exhausted, when the drop is recorded where the team responsible for the consumer can see it. The MAY is its own sentence: the Statement keeps one tier per obligation sentence, and the MUST for every other message is unchanged.
- `/{project}:analyze` does not flag a plan whose declared-expendable message is dropped and the drop recorded. It still flags a discard of a message the spec does not declare expendable, and a drop of a declared-expendable one that is not recorded.
- The redelivery bound is still stated for every consumer; expendability relaxes only the disposition.

## Edge Cases

- A spec that declares every message expendable still states the redelivery bound and where each drop is recorded.
- A drop recorded nowhere the responsible team can see is a silent discard, flagged as before.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
