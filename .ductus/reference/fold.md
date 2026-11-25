# fold — reference

**Where a named section lives.** A section named below with no heading here is in the command file, `fold.md`, with the numbered steps this file cites. A bare `§name` is a heading here or a constitution marker; one with another file's path on its line is a heading there, unless it is a constitution marker.

## Choosing the route

Two routes, and the choice is about the **body** of the branch-scoped spec:

- **`body-edit`** — the content states or refines what the upstream spec already means. It belongs in that spec's own body, under a named section. Name a section the upstream spec actually has; inventing one puts content where no reader of the spec's structure will look for it.
- **`scenario`** — the content is a durable behavioral requirement, edge case, or contract that the upstream spec covers at a high level but does not describe in detail. It belongs as its own scenario under that spec, exactly as `/ductus:amend` and `/ductus:groom` create one.

The branch-scoped spec's **own scenarios are not routed**. A scenario is already an organizational split, so each crosses over as a scenario under the upstream spec whichever route the body took. They inform the decision — a body whose substance is already covered by a scenario it ships with reads differently from one that stands alone — without being subjects of it.

This vocabulary is deliberately separate from the inbox router's five routes (`rule`, `spec`, `scenario`, `chore`, `discard`). That set answers *where in the corpus does this work belong*; a fold has already been told where, by `folds-into` at creation. What is undecided is only the shape.

## Reopening a `done` upstream spec

Folding into a `done` spec sets it to `in-progress` only when the fold leaves it owing work — an unchecked box in its `tasks.md` or an unticked criterion in its `spec.md` — which `check-owed-work` reports after the fold's writes (§spec-lifecycle), so the fold adds no new transition (AC25, superseded in part by spec 084). A scenario route always leaves its task; a body-edit fold leaves owed work when the folded text adds a criterion or the branch-scoped spec carries a scenario, whose task step 8 appends, and one that leaves neither leaves the upstream spec `done`.

- **Only from `done`.** Any other status is left unchanged; the fold never advances a status.
- **Guarded.** `set-status`'s `from: done` argument surfaces a discrepancy rather than overwriting it.
- **Named in the one confirmation, as conditional**, not prompted separately, so the operator consents to the status change before it happens. Step 5 comes before any write, when a body-edit route cannot yet say whether the folded text adds a criterion, so its prompt names the reopen as happening if the fold leaves a task or an unticked criterion, and step 14 reports which happened.
- **The review record is reset with the reopen, and only then.** A fold brings requirements, and the pre-`done` gate owes a review for code alone (spec 081), so neither a folded-in *scenario* nor a **body-edit** fold, which writes only `spec.md`, would make one owed. Step 10 therefore invalidates the upstream spec's recorded review on **either** route when it is `in-progress` after step 9 (AC24, superseded in part by spec 084), and the gate then blocks on *not reviewed*, the accurate reason. That review's scope holds the code of a staging spec folded below `done`, wherever its window starts, since no review has read it; one that reached `done` had its code read by its own review (`compute-review-scope`). An upstream spec left `done` keeps its record as it stood: a reset record on a `done` spec would be review-state drift, which the adopter CI template's audit-record gate fails on.

## Re-pointing inbound pointers

`rewrite-spec-links` moves every pointer naming the retiring directory in one action, the `folds-into` field included (AC33); its description states what it rewrites, what it leaves and what stops it. It reads each referrer the way `check-orphaned-references` does, so it re-points what that check would report.

**A refused file is the sweep not having finished.** Each write is guarded (`BE-TXN-002`): a file another writer changed after the sweep read it is left as that writer made it and named in `refused`, never counted `rewritten`, and the others are still written, so the result says which is which (`BE-TXN-004`). Steps 11 and 12 therefore go on to `retire-feature` only from a run whose `refused-count` is zero (the count is inline in both forms; past the read size the list is in the file `lists-file` names), and the re-run that gets there re-points each refused file from its content as it now is. The kept tails of the files a first run wrote are not listed by the re-run, since nothing in them names the directory any more, so the step 14 report carries each run's `kept-tails`.

What the step 14 report rests on: `check-orphaned-references` reports whatever a skipped referrer still names, or lists it among the files it could not read, and `check-corpus-links`, which reads through a symbolic link that stays inside the repository (one leaving it is skipped `outward-link` and never read), reports a link in a skipped spec-root link that no longer resolves. A `kept-tails` reference lands on a file of that name the fold did not write, or names a scenario the retiring directory did not hold, and the check sees only that the file exists, so the report names it for the operator. `mentions` is the prose, code spans, link labels and `cross-spec-impact:` entries the rewrite does not move.

`dependencies:` and `references:` are **not** edited: the pre-commit generators rewrite them from the corrected bodies on the next commit, which is why the report recommends committing before folding the next spec (AC23). `check-orphaned-references`, which `/ductus:analyze` runs on every invocation, is the verifier afterwards, not the repair path (AC22).
