---
description: Fold a branch-scoped spec into its upstream home and retire the staging directory.
argument-hint: "[feature]"
---

# Fold

Fold a branch-scoped spec's content into the upstream spec it names, then retire the staging directory.

## Purpose

Pipeline discharge for the branch-scoped directory form. A spec numbered `{identifier}.{n}-{slug}` exists so two branches can each create a spec without claiming the same sequential number ([051 — Branch-scoped spec numbering](https://github.com/stonean/ductus/blob/main/specs/051-branch-scoped-spec-numbering/spec.md)); it is a **staging** form, not a destination. This command is how it is discharged: the content moves into the upstream spec its `folds-into` names, every inbound pointer moves with it, and the directory is removed.

The branch-scoped form therefore has no `done` state at all. It is retired, not completed — which is why `/{project}:status` reports a spec carrying `folds-into` as pending rather than `done`, and why `/{project}:implement`'s pre-`done` gate blocks while the field is present. The fold is the outstanding work; this is the command that clears it.

Run it **after the merge**, on the upstream branch. That is the first tree in which both specs exist, and so the first moment the fold target's existence can be checked at all — before it, the target normally lives on a branch this tree cannot see.

## Context

Parse `$ARGUMENTS` for an optional feature identifier — a feature number, branch identifier, partial name, or full directory name — and resolve it through `resolve-feature` (`ambiguous` and `not-found` are domain outcomes to surface, not errors to swallow). It overrides the session target.

With no argument, use the session target from `.ductus/session.toml` — the spec `/{project}:status` surfaced as carrying a pending fold. If no target can be resolved, stop and tell the user to run `/{project}:target` first, pointing at `/{project}:status` for the list of specs with an outstanding fold.

## Scope Boundaries

- This command writes to the **upstream** spec (its body or a new scenario under it, its `tasks.md`, and its frontmatter status on a `done` → `in-progress` reopen), re-points inbound pointers across the spec root, removes the branch-scoped feature directory, and rewrites `.ductus/session.toml`. It does NOT read or modify source code or test files — folding moves a specification, not an implementation.
- Read only the targeted branch-scoped spec (with its scenarios) and the upstream spec named by its `folds-into`. Do NOT read plans, data models, or source code.
- `dependencies:` and `references:` are never hand-edited here. They are derived from body links, and the pre-commit generators regenerate them from the rewritten bodies on the next commit.
- Reference: §spec-lifecycle, §numbering, §scenarios, §cross-spec-impact, §text-first-artifacts, §spec-phase (spec-root resolution) (constitution loaded by `/{project}:target` — do not re-read).

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write) per the Markdown-only reference below.

Process specs **one at a time**. Complete steps 3–13 for one branch-scoped spec — route, confirm, writes, retirement — before returning to step 3 for the next. That granularity is the atomicity this command offers, and it is the granularity the primitives give, each being individually atomic; a transaction spanning the whole run is not something the runtime provides.

**An interrupted fold is completed by re-running it against the same spec.** The runtime has no multi-file transaction, so an interruption between two of one spec's writes leaves that spec partly folded — the recovery is not a rollback but a second run, and every write is built so that a second run is a no-op where the first already landed:

| Step | What makes a re-run safe |
| --- | --- |
| 6 — body edit | The routing step already holds both documents. Content already present in the target section is recognized and **not** folded a second time; say so at the confirmation prompt when a re-run is what this is. |
| 7 — `create-scenario` | A slug already taken under the upstream spec is reported rather than overwritten. |
| 8 — `append-task` | Deduped on the scenario pointer: a task already referencing `scenarios/{slug}.md` is returned as-is with `appended: false`, and `tasks.md` is unchanged. |
| 9 — `set-status` | Guarded on `from: done`; a spec already reopened is `in-progress` and is left alone. |
| 10 — `invalidate-review` | Already-invalidated is `invalidated: false`, a domain outcome. |
| 11 — `rewrite-spec-links` | Idempotent by construction: once re-pointed, no link names the retiring directory, so the second pass rewrites nothing. It also refuses an absent target *before* writing, which is what keeps that idempotence from becoming a trap — a rewrite that ran against nothing could not be undone by re-running. |
| 12 — `retire-feature` | Already-absent is `retired: false`, a domain outcome. |

Specs *other* than the one in flight are untouched by an interruption, because each spec's writes and retirement complete before the next begins. The one in flight is recoverable rather than atomic, and that distinction is stated here rather than left for an operator to discover mid-recovery.

1. Invoke `check-unfolded-specs` to enumerate every branch-scoped spec still present in this tree, each with its declared `folds-into` and status. Report the set — this is the backlog the run is working through, and naming it up front is what lets the operator see how much remains after the targeted spec. An empty `unfolded` with a non-zero `examined` means there is nothing to fold: report that and stop. A spec whose `folds-into` does not resolve in this tree is reported with that noted, and is **not** filtered out — the target may simply be a typo, and the refusal in step 11 is where that is settled, before any pointer moves.

2. Resolve the spec to fold (host responsibility): the `$ARGUMENTS` override through `resolve-feature`, otherwise the session target. Confirm it appears in step 1's `unfolded` list. When it does not, stop and say which condition failed — the target is not a branch-scoped directory, or it declares no `folds-into`. Do not proceed on a spec that names no upstream home: folding one would mean inventing a destination it never claimed, which is exactly the choice this command exists to have recorded in advance.

3. Invoke `read-spec` (with `include-body`) against the targeted spec to load its frontmatter and body. The frontmatter's `folds-into` is the fold target, and it is the **only** source for it: the field is the record of where this spec belongs, so accepting a destination from an argument or from conversation would fold a spec somewhere it never claimed to go.

4. <!-- llm:routeFold --> Decide the shape the content takes in its upstream home: a **body edit** under a named section of the upstream spec, or a **scenario** created under it. The host receives the branch-scoped spec and its scenarios, the upstream spec with its section headings and status, and the two-route vocabulary; it returns the route plus the section or scenario slug it names. The routing is semantic — the vocabulary is deliberately not `routeInboxItem`'s, which answers where work belongs in the corpus, while this answers what shape content whose home is already known takes on arrival (see **Choosing the route** below).

5. Invoke `gate-confirm` to confirm the routing with the user, and do not proceed until the user approves. This single confirmation is the consent for everything the fold performs, so it must name all of it: the route and its destination, the reopen when the upstream spec is `done`, the retirement of the branch-scoped directory, and the re-targeting that follows it. No write happens before this step (AC18).

6. <!-- llm:writeSpecBody --> **Body-edit route only** — fold the content into the named section of the upstream spec. The host receives the upstream spec and the target section and returns the section's new body. Skip this step on the scenario route.

7. Invoke `create-scenario` for each scenario the fold creates under the upstream spec: on the scenario route, the one step 4 named; and, on **either** route, one per scenario the branch-scoped spec carries. A scenario is already an organizational split of a spec, so it crosses over as a scenario whichever route the body took — the route decides the body's shape, never its scenarios'. The primitive frames each file; write the folded content into it.

8. Invoke `append-task` against the upstream spec's `tasks.md`, once per scenario created in step 7, referencing that scenario — the same pairing `/{project}:groom` and `/{project}:amend` perform on their scenario routes. A body-edit route that created no scenario appends no task: a body edit states what the spec means, which is not by itself work to implement.

9. Invoke `set-status` with `from: done`, `to: in-progress` when the upstream spec's status is `done`. This is the documented back-edge §spec-lifecycle already defines, not a new one — the same guarded call `/{project}:groom` and `/{project}:amend` make. The `from` guard surfaces a concurrent edit as an operational error rather than a silent overwrite. An upstream spec at `draft`, `clarified`, `planned`, or `in-progress` is left alone, and a second fold into an already-reopened spec finds it `in-progress` and does nothing here.

10. Invoke `invalidate-review` against the **upstream** spec, on every route. A fold brings the branch-scoped spec's content — and the code that implements it — into a spec whose recorded review never saw either, so that review no longer describes what it claims to. The pre-`done` gate cannot work this out for itself: its staleness check diffs the spec's *durable contracts*, and a body-edit fold writes only `spec.md`, which that check deliberately excludes. Without this step such a spec returns to `done` on a pre-fold review (AC24). Run it regardless of the upstream status — a fold into a spec that was already `in-progress` leaves exactly the same stale review behind, and step 9 does not fire there. Operator-recorded waivers survive: an invalidation says the review is out of date, not that a judgement about a finding was withdrawn. `invalidated: false` means there was no current review to clear.

11. Invoke `rewrite-spec-links` with `from` set to the branch-scoped directory and `to` set to the fold target — the feature alone on a body-edit route, `{feature}/{scenario}` on a scenario route, so inbound links land on the file the content actually reached. It re-points body links across the spec root **and** any `folds-into` field naming the retiring directory, in one action. Frontmatter indexes are left alone by design; the result's `examined` count is what bounds an empty `rewritten` as *nothing pointed here* rather than *nothing was checked*. **The primitive refuses before writing anything when the target is not a home** — a directory holding a `spec.md`, the same terms step 12 enforces — so a corpus-wide rewrite can never point every inbound link at a spec nobody can open. That refusal lives in the primitive rather than in this step's prose because recovery is not available for this one: the rewrite is idempotent in the *wrong* direction, so once links are re-pointed nothing names the retiring directory and a re-run finds nothing to repair.

12. Invoke `retire-feature` with the branch-scoped feature and its fold target to remove the directory. It refuses when the target names no spec, so a retirement can never strand content (AC28) — and it keeps that check even though step 11 has already established the same fact, because the primitive is callable on its own and the guard on the one irreversible step must not rest on a caller's promise. By the time this step runs, a `folds-into` that step 1 reported as unresolved has already been settled: step 10 needs the target and step 11 refuses without it, so a refusal *here* means the target went missing mid-run rather than that the question was never asked. Surface it, and do not retry against a different target the operator did not record. `retired: false` means the directory was already gone, which is a re-run of an interrupted fold converging, not a failure. **This call never passes the opt-in that permits removing a sequential directory.** That refusal is gated rather than removed — `/{project}:consolidate` passes the opt-in, having named both specs and confirmed the removal with the operator — so a mistyped feature name during a fold still meets it unchanged, which is the protection the refusal exists for. Fold has no way to retire a sequential spec and gains none here.

13. Invoke `write-session` to re-target the session at the upstream spec — the feature, and the scenario when step 7 created one under it. The targeted directory no longer exists after step 12, and §concurrent-features forbids leaving the session pointing at it. **Re-target rather than clear**, and that rule states why this command takes the re-target branch while `/{project}:consolidate` takes the other: the content moved to a spec that continues the work, so the new target is a fact about where the operator's attention went rather than a guess at it. Preserve the existing `cli-config-dir` as every session write does.

<!-- audit:ignore-promotion -->
14. Report the fold (host responsibility): the spec folded, its route and destination, the upstream spec's status change if one happened, the counts from `rewrite-spec-links`, and the new session target. Then name the branch-scoped specs still un-folded from step 1's enumeration and ask whether to continue with the next — returning to step 3 for it, since each spec's writes and retirement complete before the next begins. Recommend committing before continuing: `dependencies:` and `references:` across the corpus are regenerated by the pre-commit hook from the rewritten body links, so the first commit after a fold is what makes those indexes correct (AC23).

## Markdown-only reference

With no ductus runtime registered, the host performs the same walk and the same writes with its own file tools (Read, Edit, Write) — no shell-pipeline substitution — one contract, two paths (§runtime-host-integration).

### Enumerating branch-scoped specs

List the directories under the configured `[paths] specs-root` whose names carry a `.` before the first `-` — the branch-scoped form `{identifier}.{n}-{slug}`, as distinct from the sequential `NNN-slug`. Read each one's `folds-into` and `status` from its `spec.md` frontmatter. Count every feature directory scanned, not just the branch-scoped ones, so an empty result reads as *examined and none staged* rather than *nothing was looked at*.

A `folds-into` naming a spec absent from this tree is reported as unresolved-here, never as broken. Before the merge that absence is the ordinary state — the target lives on the upstream branch — and this enumeration cannot tell which tree it is looking at. The distinction is settled at retirement, in the tree where both specs exist.

### Choosing the route

Two routes, and the choice is about the **body** of the branch-scoped spec:

- **`body-edit`** — the content states or refines what the upstream spec already means. It belongs in that spec's own body, under a named section. Name a section the upstream spec actually has; inventing one puts content where no reader of the spec's structure will look for it.
- **`scenario`** — the content is a durable behavioral requirement, edge case, or contract that the upstream spec covers at a high level but does not describe in detail. It belongs as its own scenario under that spec, exactly as `/{project}:amend` and `/{project}:groom` create one.

The branch-scoped spec's **own scenarios are not routed**. A scenario is already an organizational split, so each crosses over as a scenario under the upstream spec whichever route the body took. They inform the decision — a body whose substance is already covered by a scenario it ships with reads differently from one that stands alone — without being subjects of it.

This vocabulary is deliberately separate from the inbox router's five routes (`rule`, `spec`, `scenario`, `chore`, `discard`). That set answers *where in the corpus does this work belong*; a fold has already been told where, by `folds-into` at creation. What is undecided is only the shape.

### Reopening a `done` upstream spec

Folding into a `done` spec sets it to `in-progress` — the back-edge §spec-lifecycle already defines ("Backward via new scenario" / "Backward via meaningful body edit"), so the fold adds no new transition (AC25).

- **Only from `done`.** Any other status is left unchanged; the fold never advances a status.
- **Guarded.** Confirm the on-disk status is still `done` immediately before writing, and surface a discrepancy rather than overwriting it. On the runtime path `set-status`'s `from: done` argument is that guard.
- **Named in the one confirmation**, not prompted separately, so the operator consents to the status change before it happens.
- **The upstream spec cannot return to `done` on the old review**, and the fold is what makes that true. A folded-in *scenario* is a durable contract, so the pre-`done` gate's staleness check would catch that route on its own — but a **body-edit** fold writes only `spec.md`, which that check deliberately excludes, so the staleness check sees nothing moved and the spec would complete on a review that never saw the code the fold brought with it. Step 10 therefore invalidates the upstream spec's recorded review explicitly, on **either** route and at **any** status, rather than relying on a check whose subject the body-edit route does not touch (AC24). The gate then blocks on *not reviewed*, which is the accurate reason.

### Re-pointing inbound pointers

Every pointer naming the retiring directory moves in the same action that retires it — body links across the spec root and any `folds-into` field naming it alike. The frontmatter field is included deliberately: it is the one pointer whose job is to survive until the merge, so repairing body links and leaving it behind would break exactly the thing the field exists for (AC33).

Matching is by whole path segment, so a directory whose name merely shares a prefix is untouched, and a cross-service URL naming another repository's spec is never rewritten — how it resolves there is that repository's business.

`dependencies:` and `references:` are **not** edited. They are derived from body links, and the pre-commit generators rewrite them from the corrected bodies on the next commit, which is why the report recommends committing before folding the next spec (AC23). `check-orphaned-references` is the verifier afterwards, not the repair path: it reports rather than repairs because a migration knows only its own hop, whereas a fold knows both endpoints and so can rewrite (AC22).

### Retiring the directory

Removal is the last step and the only irreversible one; everything before it is a write another write can undo.

- **The fold target must exist**, and a directory holding no `spec.md` does not satisfy that — it is not a home content can have landed in. A refusal leaves the branch-scoped spec exactly where it was. The same check runs one step earlier, in `rewrite-spec-links`, so by here it is a second line of defense rather than the first: this primitive is callable on its own, and the guard on the irreversible step does not rest on what a caller promises to have checked.
- **A sequential feature is refused on this path.** The `NNN-slug` form is permanent; a spec in it is completed, never retired. The refusal is *gated*, not absent: `retire-feature` carries an explicit opt-in that only `/{project}:consolidate` passes, after naming both specs and confirming the content loss with the operator. Fold does not pass it and has no argument that would, so from here the refusal behaves exactly as it always did — a mistyped feature name meets it rather than removing a permanent directory.
- **An already-absent directory is not a failure.** It means a previous run finished this spec, so a re-run converges instead of halting — which is what the resumption contract above is for.
