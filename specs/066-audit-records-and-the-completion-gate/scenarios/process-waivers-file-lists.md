---
section: "The review record"
---

# Process-waivers-file-lists

## Context

[020's waiver-file-lists](../../020-code-review/scenarios/waiver-file-lists.md) lets a waiver's `file` hold a list of paths, each its own `(rule, file)` anchor. Three primitives read the recorded waivers, and all three parsed `file` as one string: `process-waivers` classifies them, `write-review` prunes and re-renders them, and `invalidate-review` re-renders them when it clears a record. A list failed that parse, and `read_recorded_list` in `runtime/src/primitives/mod.rs` turns a failed entry into a failed list, so the runtime halted on a record written correctly under 020's shape.

## Behavior

- The waiver readers accept `file` as a string or a list of strings. `process-waivers` walks each entry's paths in order as separate anchors, so `applied`, `expired` and `retained` still carry one `WaiverRef` per anchor and the result shape does not change. Its notices are per anchor.
- A `file` that is absent, blank, an empty list, or a list holding a blank path is reported `missing 'file'`. A list item that is not a string fails the parse, as any non-string known field does.
- Duplicate detection runs over anchors across all entries, first claim wins. A later claim is reported with the existing notice text, and the same entry's other paths are still classified.
- `write-review`'s prune removes each expired anchor's path from every entry that lists it, keeping each entry's remaining paths and every other field, adopter extras included, and drops an entry left with no path. It renders one path as a scalar and several as a block list, each path through the same quoting as every other known field, so a rendered record re-parses into the same waivers ([write-review-known-field-quoting](write-review-known-field-quoting.md)).
- `invalidate-review` round-trips a list unchanged; it prunes nothing.

## Edge Cases

- An entry missing `rule` names no anchor and prunes nothing, whatever its `file` holds.
- Pruning compares anchors only, so a malformed entry — one missing `reason`, say — that lists an expired anchor loses that path and keeps the rest.
- On a dimension-restricted run, the anchors of one entry can land in different buckets: a listed path whose rule fired is applied, and one whose rule did not is retained.
- A one-path list written by hand re-renders as a scalar, which re-parses to the same anchor.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
