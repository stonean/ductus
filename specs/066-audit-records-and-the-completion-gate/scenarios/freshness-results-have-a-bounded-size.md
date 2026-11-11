---
section: "Record freshness"
---

# Freshness-results-have-a-bounded-size

## Context

`check-review-freshness` (task 11, retired by spec 081) returned `freshness.paths`, and `write-review`'s `analyze-freshness` and `check-review-gate`'s freshness answers carry the same `RecordFreshness` shape (`runtime/src/schema/primitives.rs`). The data model's only bound is "`paths` holds at most the spec's durable contracts", which is no size limit, and it is inaccurate: `freshness_of` (`runtime/src/primitives/analyze_subjects.rs`) also lists each recorded digest key no longer on disk. `BE-PAYLOAD-001` requires a response that returns a collection to commit to a maximum size. 078's second review (2026-11-04) found it in this spec's task 11 and deferred it here.

## Behavior

- **Every result that carries `RecordFreshness.paths` has a stated maximum size**, stated in `data-model.md` beside the result's shape, by the mechanism `scope-results-have-a-bounded-size` settles: `write-review`'s `analyze-freshness`, and `check-review-gate`'s freshness answers.
- **The stated contents match what the list holds**: the changed subject files, and each recorded key no longer on disk.
- **A bound never hides a changed path.** A result past the bound gives the caller a way to reach every path, and never reads as a short complete list.

## Edge Cases

- **A spec whose freshness lists fit.** The result is returned whole, as now.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
