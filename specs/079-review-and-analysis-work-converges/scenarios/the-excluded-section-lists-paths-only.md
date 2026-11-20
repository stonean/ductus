---
section: "Behavior"
---

# The-excluded-section-lists-paths-only

## Context

AC3 makes the record name every path a review's scope and its fix rounds left out. The first delivery also named each path's claiming specs, which were 88.6% of the section's bytes across the 76 `specs/*/review.md` records at `b7b07500` (method: section split on level-2 headings, UTF-8 byte length), and which no primitive, command or check reads.

## Behavior

- `review.md`'s `## Excluded from scope` lists each path the scope left out as one bullet holding the path, and each path a fix round left out as a bullet holding the path and its round. It names no claiming spec.
- A reader who needs a path's claimants gets them from the commits that touched it.
- A record written before this keeps its claimant lists until its spec's next review, which regenerates the section.

## Edge Cases

- A run that excluded nothing still writes `*None.*`.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
