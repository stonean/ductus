---
section: "Follow-on scenarios"
---

# Review-staleness-gate

## Context

`check-review-gate` asked four questions: does the feature's markdown lint,
do its scenarios carry open questions, has a review run, did it pass. None of
them asked whether the review still *applies*.

So a review recorded against one commit and never re-run reads as a pass
forever. Hand-editing a `review.md` to mark findings resolved produces a record
that satisfies every automated check while describing a diff that is gone.

That shipped. `gvrn-v0.26.2` was tagged at `334907f` while this spec's review
read `reviewed-against: 1f7ee722` — three commits back, covering none of the
adopter-scope suppression it released. `check-review-gate` passed. So did
`/gov:analyze`'s `review-state-drift` family, which tests the same two fields.
The gap was invisible to tooling and surfaced only because the user asked
whether the reviews had been run.

## Behavior

A fifth gate check, ordered last because it is the weakest claim: the other
four say a review is missing or failing, this one says a passing review is out
of date.

`ReviewGateBlock::ReviewStale` fires when a file the spec's plan declares under
**Affected Files** changed between `review.reviewed-against` and `HEAD`. The
message names the count, the short sha, and up to three paths; the guidance
names the command that clears it.

Scoped to the plan's declared surface rather than the repo. A repo-wide test
would mark every review stale on the next unrelated commit, which is the
fastest way to teach people to route around a gate. An entry naming a directory
matches everything beneath it, matching how `compute-review-scope` reads the
same list — `read_plan_affected` is now `pub(crate)` and shared rather than
reimplemented.

Two exclusions, both bookkeeping rather than subject matter: the spec's own
`review.md` and `spec.md`. `write-review` touches both, so counting them would
make every review stale the instant it was recorded.

## Edge Cases

- **Fails open on anything it cannot determine** — no git repository, a
  `reviewed-against` that will not parse or does not resolve, a plan with no
  **Affected Files** table. A gate that blocked on its own inability to check
  is one people disable; the four honest checks still run. This is the opposite
  posture from Family 17's derivation, and deliberately so: there an empty
  result meant *checking nothing*, here it means *blocking nothing*.
- **Never-reviewed specs are unaffected.** A null `reviewed-against` is
  `NotReviewed`'s finding, raised earlier; this check returns without an
  opinion.
- **Ordering is load-bearing.** Staleness is evaluated after `blocking`, so a
  spec with MUST violations reports those rather than a staleness message that
  would send the reader to re-run a review that will fail again.
- **A stale review is not a wrong review.** The finding says the verdict no
  longer covers the code, not that the code is defective. The fix is to re-run
  the review, which may well reproduce the same clean result.
- **The release-time half lives in `/{project}:audit` Family 19**
  (`review-freshness`), which asks the same question of every `done` spec at
  once. Two enforcement points for one rule, matching how blocking semantics is
  already built from three mutually reinforcing mechanisms rather than one.
  They deliberately scope differently — see that family for why the
  release-time rule reads durable contracts instead of Affected Files.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
