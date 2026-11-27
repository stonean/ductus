---
section: "Record freshness"
---

# Review-staleness-on-done-specs

## Context

A review record's `reviewed-digest` names the durable contracts the review read, and `compute-review-scope` compares it with those files as they are now, putting each changed one in the scope of the next review that runs (AC3). The comparison gates nothing: a changed contract owes an analysis and no review (spec 081), and the completion gate has no transition to check on a `done` spec (AC4).

Without an exemption, the comparison reads a corpus-wide rename as a change to every contract the rename touched. Measured 2026-08-16 over the 46 `done` specs, 19 read stale, every one a consequence of 049's `govern → ductus` sweep and none a real change: `017-derive-dont-ask`'s three changed contracts all changed in `9da4a7a`, that sweep's commit.

## Behavior

**A mechanical sweep changes no contract.** The comparison, `analyze_subjects::freshness_of` over `is_review_contract` (a `scenarios/*.md` file or `data-model.md`), reached through `review_freshness`, drops a changed contract whose change since `reviewed-against` was a mechanical sweep, through `primitives::mechanical_sweep`, the exemption the analysis record's freshness applies too. When the two trees it needs are unavailable, the exemption does not run and every candidate is kept.

**An abbreviated `reviewed-against` resolves.** The recorded sha is resolved with `revparse_single`, so a short sha names its commit rather than a zero-padded id that matches nothing.

**The comparison reads the spec's own contracts.** It covers the spec's `scenarios/*.md` and `data-model.md`, never its plan's Affected Files, which old specs list with shared surfaces such as `AGENTS.md` that every later spec touches: tried that way on 2026-08-16, it blocked 34 of the corpus's 48 specs.

**The `review-state-drift` arms keep their grandfather rule.** A `done` spec with no review record at all predates `/{project}:review` and is exempt, keyed on the absence of `review.md`: keyed on a record block in `spec.md`, which spec 057 moved out, the predicate would hold of every spec and the family would exempt the whole corpus while exiting green.

## Edge Cases

- A mechanical sweep (a rename, a citation's line number removed) that touches a durable contract without changing its meaning puts no contract in a review's scope.
- `reviewed-against` names a commit absent from a shallow clone: the exemption cannot run, so every candidate is kept rather than dropped.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Does a `done` spec's review owe a re-run when its contracts change?** No: a changed contract owes an analysis and no review (spec 081). It joins the scope of the next review that runs.
