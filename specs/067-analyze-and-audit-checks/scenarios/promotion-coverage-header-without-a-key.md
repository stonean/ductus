---
section: "Audit checks"
---

# Promotion-coverage-header-without-a-key

## Context

A classification table whose header row has an empty first cell (for example `| | Why |`) pushes no key, and the delimiter row that follows then dropped the previous table's last real key, so an entry the table covers was reported unclassified.

## Behavior

- A delimiter row removes a key only when the row before it was pushed as a key; a header with an empty first cell contributes and removes none.

## Edge Cases

- A table whose header has a key in its first cell is unchanged.
- The measurement stays a notice.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
