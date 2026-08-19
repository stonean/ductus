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
`/ductus:analyze`'s `review-state-drift` family, which tests the same two fields.
The gap was invisible to tooling and surfaced only because the user asked
whether the reviews had been run.

## Behavior

A fifth gate check, ordered last because it is the weakest claim: the other
four say a review is missing or failing, this one says a passing review is out
of date.

`ReviewGateBlock::ReviewStale` fires when one of the spec's **durable
contracts** — a `scenarios/*.md` file or `data-model.md` — changed between
`review.reviewed-against` and `HEAD`. The message names the count, the short
sha, and up to three paths; the guidance names the command that clears it.

**The scope was wrong on the first cut and measurement caught it.** The
initial version used the plan's **Affected Files**, reasoning that Family 19
could afford a narrow rule (it judges every spec at release) while the gate
could afford a wide one (it judges one spec at completion). That reasoning was
never tested. Run across this repo, the Affected-Files rule blocked **34 of 48**
specs — old specs list shared surfaces (`AGENTS.md`, `README.md`,
`framework/bootstrap/ductus.md`) that every later spec also touches, so
completing spec 004 was blocked by spec 042 having edited `AGENTS.md`. A gate
that blocks seven specs in eight is one people route around, which is the
failure this scenario's own prose warned about. The durable-contract rule
blocks **0 of 48** once reviews are current.

So the two enforcement points now apply the *identical* rule rather than
deliberately different ones — `is_durable_contract` here mirrors
`scripts/audit/review-freshness.sh` exactly. `tasks.md` and `plan.md` are
excluded because the first is ephemeral by construction
([§tasks-phase](../../../framework/constitution.md#tasks-phase)) and the second
churns as Affected Files are revised; `review.md` and `spec.md` because
`write-review` touches both, so counting them would make every review stale the
instant it was recorded.

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
  They now share one definition of stale; an earlier draft scoped them
  differently and that difference was the defect, not the design.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
