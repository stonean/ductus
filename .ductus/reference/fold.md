# fold — reference

**Where a named section lives.** A section named below with no heading here is in the command file, `fold.md`, with the numbered steps this file cites. A bare `§name` is a heading here or a constitution marker; one with another file's path on its line is a heading there, unless it is a constitution marker.

## Enumerating branch-scoped specs

`check-unfolded-specs` (step 1) lists them. A `folds-into` naming a spec absent from this tree is reported as unresolved-here, never as broken: before the merge that absence is the ordinary state, and the distinction is settled at retirement, in the tree where both specs exist.

## Choosing the route

Two routes, and the choice is about the **body** of the branch-scoped spec:

- **`body-edit`** — the content states or refines what the upstream spec already means. It belongs in that spec's own body, under a named section. Name a section the upstream spec actually has; inventing one puts content where no reader of the spec's structure will look for it.
- **`scenario`** — the content is a durable behavioral requirement, edge case, or contract that the upstream spec covers at a high level but does not describe in detail. It belongs as its own scenario under that spec, exactly as `/ductus:amend` and `/ductus:groom` create one.

The branch-scoped spec's **own scenarios are not routed**. A scenario is already an organizational split, so each crosses over as a scenario under the upstream spec whichever route the body took. They inform the decision — a body whose substance is already covered by a scenario it ships with reads differently from one that stands alone — without being subjects of it.

This vocabulary is deliberately separate from the inbox router's five routes (`rule`, `spec`, `scenario`, `chore`, `discard`). That set answers *where in the corpus does this work belong*; a fold has already been told where, by `folds-into` at creation. What is undecided is only the shape.

## Reopening a `done` upstream spec

Folding into a `done` spec sets it to `in-progress` — the back-edge §spec-lifecycle already defines ("Backward via new scenario" / "Backward via meaningful body edit"), so the fold adds no new transition (AC25).

- **Only from `done`.** Any other status is left unchanged; the fold never advances a status.
- **Guarded.** `set-status`'s `from: done` argument surfaces a discrepancy rather than overwriting it.
- **Named in the one confirmation**, not prompted separately, so the operator consents to the status change before it happens.
- **The upstream spec cannot return to `done` on the old review.** A folded-in *scenario* is a durable contract the pre-`done` gate's staleness check would catch, but a **body-edit** fold writes only `spec.md`, which that check excludes. Step 10 therefore invalidates the upstream spec's recorded review on **either** route and at **any** status (AC24), and the gate then blocks on *not reviewed*, the accurate reason. That review's scope holds the code of a staging spec folded below `done`, wherever its window starts, since no review has read it; one that reached `done` had its code read by its own review (`compute-review-scope`).

## Re-pointing inbound pointers

`rewrite-spec-links` moves every pointer naming the retiring directory in one action, the `folds-into` field included (AC33); its description states what it rewrites, what it leaves and what stops it. It reads each referrer the way `check-orphaned-references` does, so it re-points what that check would report.

What the step 14 report rests on: `check-orphaned-references` reports whatever a skipped referrer still names, or lists it among the files it could not read, and `check-corpus-links`, which reads through a symbolic link, reports a link in a skipped spec-root link that no longer resolves. A `kept-tails` reference lands on a file of that name the fold did not write, or names a scenario the retiring directory did not hold, and the check sees only that the file exists, so the report names it for the operator. `mentions` is the prose, code spans, link labels and `cross-spec-impact:` entries the rewrite does not move.

`dependencies:` and `references:` are **not** edited: the pre-commit generators rewrite them from the corrected bodies on the next commit, which is why the report recommends committing before folding the next spec (AC23). `check-orphaned-references`, which `/ductus:analyze` runs on every invocation, is the verifier afterwards, not the repair path (AC22).

## Retiring the directory

Removal is the last step and the only irreversible one, and `retire-feature`'s description states its refusals.

- **A refusal leaves the branch-scoped spec exactly where it was.**
- **A sequential feature is refused on this path.** Fold does not pass the opt-in that only `/ductus:consolidate` passes, and has no argument that would, so a mistyped feature name meets the refusal rather than removing a permanent directory.
- **An already-absent directory is not a failure.** A re-run converges, which is what the resumption contract in `fold.md` is for.
