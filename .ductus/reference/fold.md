# fold — markdown-only reference

With no ductus runtime registered, the host performs the same walk and the same writes with its own file tools (Read, Edit, Write) — no shell-pipeline substitution — one contract, two paths (§runtime-host-integration).

**Where a named section lives.** This file is read alongside its command, `fold.md`, which is already loaded whenever this one is. A section named below that has no heading in this file is in the command file, along with the numbered Instructions steps this file cites. A bare `§name` is a heading in this file or a marker in the constitution, `.ductus/constitution.md`; a `§name` with another file's path on the same line is a heading in that file.

## Enumerating branch-scoped specs

List the directories under the configured `[paths] specs-root` whose names carry a `.` before the first `-` — the branch-scoped form `{identifier}.{n}-{slug}`, as distinct from the sequential `NNN-slug`. Read each one's `folds-into` and `status` from its `spec.md` frontmatter. Count every feature directory scanned, not just the branch-scoped ones, so an empty result reads as *examined and none staged* rather than *nothing was looked at*.

A `folds-into` naming a spec absent from this tree is reported as unresolved-here, never as broken. Before the merge that absence is the ordinary state — the target lives on the upstream branch — and this enumeration cannot tell which tree it is looking at. The distinction is settled at retirement, in the tree where both specs exist.

## Choosing the route

Two routes, and the choice is about the **body** of the branch-scoped spec:

- **`body-edit`** — the content states or refines what the upstream spec already means. It belongs in that spec's own body, under a named section. Name a section the upstream spec actually has; inventing one puts content where no reader of the spec's structure will look for it.
- **`scenario`** — the content is a durable behavioral requirement, edge case, or contract that the upstream spec covers at a high level but does not describe in detail. It belongs as its own scenario under that spec, exactly as `/ductus:amend` and `/ductus:groom` create one.

The branch-scoped spec's **own scenarios are not routed**. A scenario is already an organizational split, so each crosses over as a scenario under the upstream spec whichever route the body took. They inform the decision — a body whose substance is already covered by a scenario it ships with reads differently from one that stands alone — without being subjects of it.

This vocabulary is deliberately separate from the inbox router's five routes (`rule`, `spec`, `scenario`, `chore`, `discard`). That set answers *where in the corpus does this work belong*; a fold has already been told where, by `folds-into` at creation. What is undecided is only the shape.

## Reopening a `done` upstream spec

Folding into a `done` spec sets it to `in-progress` — the back-edge §spec-lifecycle already defines ("Backward via new scenario" / "Backward via meaningful body edit"), so the fold adds no new transition (AC25).

- **Only from `done`.** Any other status is left unchanged; the fold never advances a status.
- **Guarded.** Confirm the on-disk status is still `done` immediately before writing, and surface a discrepancy rather than overwriting it. On the runtime path `set-status`'s `from: done` argument is that guard.
- **Named in the one confirmation**, not prompted separately, so the operator consents to the status change before it happens.
- **The upstream spec cannot return to `done` on the old review**, and the fold is what makes that true. A folded-in *scenario* is a durable contract, so the pre-`done` gate's staleness check would catch that route on its own — but a **body-edit** fold writes only `spec.md`, which that check deliberately excludes, so the staleness check sees nothing moved and the spec would complete on a review that never saw the code the fold brought with it. Step 10 therefore invalidates the upstream spec's recorded review explicitly, on **either** route and at **any** status, rather than relying on a check whose subject the body-edit route does not touch (AC24). The gate then blocks on *not reviewed*, which is the accurate reason.

## Re-pointing inbound pointers

Every pointer naming the retiring directory moves in the same action that retires it — body links across the spec root and any `folds-into` field naming it alike. The frontmatter field is included deliberately: it is the one pointer whose job is to survive until the merge, so repairing body links and leaving it behind would break exactly the thing the field exists for (AC33).

Matching is by whole path segment, so a directory whose name merely shares a prefix is untouched, and a cross-service URL naming another repository's spec is never rewritten — how it resolves there is that repository's business.

`dependencies:` and `references:` are **not** edited. They are derived from body links, and the pre-commit generators rewrite them from the corrected bodies on the next commit, which is why the report recommends committing before folding the next spec (AC23). `check-orphaned-references` is the verifier afterwards, not the repair path: it reports rather than repairs because a migration knows only its own hop, whereas a fold knows both endpoints and so can rewrite (AC22).

## Retiring the directory

Removal is the last step and the only irreversible one; everything before it is a write another write can undo.

- **The fold target must exist**, and a directory holding no `spec.md` does not satisfy that — it is not a home content can have landed in. A refusal leaves the branch-scoped spec exactly where it was. The same check runs one step earlier, in `rewrite-spec-links`, so by here it is a second line of defense rather than the first: this primitive is callable on its own, and the guard on the irreversible step does not rest on what a caller promises to have checked.
- **A sequential feature is refused on this path.** The `NNN-slug` form is permanent; a spec in it is completed, never retired. The refusal is *gated*, not absent: `retire-feature` carries an explicit opt-in that only `/ductus:consolidate` passes, after naming both specs and confirming the content loss with the operator. Fold does not pass it and has no argument that would, so from here the refusal behaves exactly as it always did — a mistyped feature name meets it rather than removing a permanent directory.
- **An already-absent directory is not a failure.** It means a previous run finished this spec, so a re-run converges instead of halting — which is what the resumption contract in `fold.md` is for.
