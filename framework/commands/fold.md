---
description: Fold a branch-scoped spec into its upstream home and retire the staging directory.
argument-hint: "[feature]"
---

# Fold

Fold a branch-scoped spec's content into the upstream spec it names, then retire the staging directory.

## Purpose

Pipeline discharge for the branch-scoped directory form (§numbering): the content moves into the upstream spec its `folds-into` names, every inbound pointer it reads moves with it, and the directory is removed.

**Done first, then fold.** A staging spec reaches `done` like any other, since a pending fold never holds it short of `done` (§spec-lifecycle), and the work is complete once the fold has run. `/{project}:status` reports a spec carrying `folds-into` as pending at every status, and offers this command as the next action once the spec is `done`. Neither status gates the other, so a spec folded before it reaches `done` still folds.

Run it **after the merge**, on the upstream branch: the first tree in which both specs exist, and so the first moment the fold target's existence can be checked.

## Context

Parse `$ARGUMENTS` for an optional feature identifier — a feature number, branch identifier, partial name, or full directory name — and resolve it through `resolve-feature` (`ambiguous` and `not-found` are domain outcomes to surface). It overrides the session target.

With no argument, use the session target — resolve it with `resolve-session` and display any notices and unreadable session files it returns — the spec `/{project}:status` surfaced as carrying a pending fold. If no target can be resolved, stop and tell the user to run `/{project}:target` first, pointing at `/{project}:status` for the list of specs with an outstanding fold.

## Scope Boundaries

- This command writes to the **upstream** spec (its body or a new scenario under it, its `tasks.md`, and its frontmatter status on a `done` → `in-progress` reopen, when the fold leaves it owing work), re-points inbound pointers across the spec root and in the files `check-orphaned-references` reads outside it (adopter-owned root files such as `README.md` and `.githooks/pre-commit`), removes the branch-scoped feature directory, and re-points every session that named it — the shared default `.ductus/session.toml` and any per-process target under `.ductus/sessions/`. It does NOT read or modify source code or test files — folding moves a specification, not an implementation.
- Read only the targeted branch-scoped spec (with its scenarios) and the upstream spec named by its `folds-into`. Do NOT read plans, data models, or source code.
- `dependencies:` and `references:` are never hand-edited here; the pre-commit generators regenerate them on the next commit.
- Reference: §spec-lifecycle, §numbering, §scenarios, §cross-spec-impact, §text-first-artifacts, §spec-phase (spec-root resolution) (constitution loaded by `/{project}:target` — do not re-read).

## Instructions

> **For agent runtimes**: the Invoke steps below call the ductus runtime's MCP tools; without the server, stop and say to restart or re-run `/ductus` (§runtime-host-integration).

**Where a named section lives.** This command's reference is `.ductus/reference/fold.md`. A section named here, or cited elsewhere as part of this command, that has no heading in this file is there. When a step points at one, invoke `read-section` with that path and every section the step names, in one call, rather than reading the whole file.

Process specs **one at a time**. Complete steps 3–13 for one branch-scoped spec — route, confirm, writes, retirement — before returning to step 3 for the next. That granularity is the atomicity this command offers.

**An interrupted fold is completed by re-running it against the same spec.** An interruption between two of one spec's writes leaves that spec partly folded, and a second run, not a rollback, is the recovery:

| Step | What makes a re-run safe |
| --- | --- |
| 6 — body edit | The routing step already holds both documents. Content already present in the target section is recognized and **not** folded a second time; say so at the confirmation prompt when a re-run is what this is. |
| 7 — `create-scenario` | A slug already taken under the upstream spec is never overwritten: when the file holds the scenario this fold carries, it is the earlier run's write and the fold goes on; otherwise the fold stops before step 8 and reports the slug. |
| 8 — `append-task` | Deduped on the scenario pointer: `appended: false`, and `tasks.md` is unchanged. |
| 9 — `set-status` | Guarded on `from: done`; a spec already reopened is left alone, and one the fold left owing nothing stays `done`. |
| 10 — `invalidate-review` | Already-invalidated is `invalidated: false`, a domain outcome. A `changed-since-read` error means another writer changed `review.md` after the call read it, so nothing was written: run the step again, which reads the record as it now is. |
| 11 — `rewrite-spec-links` | Idempotent once a run's `refused-count` is zero: once re-pointed, no pointer names the retiring directory. A file another writer changed after the sweep read it is **refused**, left as that writer made it, and still names the directory, so a run whose `refused-count` is not zero did not complete and running the step again re-reads and re-points what is left; step 12 waits for a run whose `refused-count` is zero. A write that fails part-way is the gap: the files before it stay re-pointed and that run's `kept-tails` are lost, as are those of every run before a re-run, so a report says its kept tails are unknown, not that there are none, unless it carries each run's. |
| 12 — `retire-feature` | Already-absent is `retired: false`, a domain outcome. |

Specs *other* than the one in flight are untouched by an interruption; the one in flight is recoverable rather than atomic.

1. Invoke `check-unfolded-specs` and report the set, reading `unfolded` from the file its `lists-file` names when its `form` is `file`: the backlog the run is working through. A zero `unfolded-count` with a non-zero `examined` means there is nothing to fold: report that and stop. A spec whose `folds-into` does not resolve in this tree is reported with that noted, and is **not** filtered out, since the refusal in step 11 settles it before any pointer moves.

2. Resolve the spec to fold (host responsibility): the `$ARGUMENTS` override through `resolve-feature`, otherwise the session target. Confirm it appears in step 1's `unfolded` list. When it does not, stop and say which condition failed — the target is not a branch-scoped directory, or it declares no `folds-into`.

3. Invoke `read-spec` (with `include-body`) against the targeted spec to load its frontmatter and body. The frontmatter's `folds-into` is the fold target and the **only** source for it: never accept a destination from an argument or from conversation.

4. <!-- llm:routeFold --> Decide the shape the content takes in its upstream home: a **body edit** under a named section of the upstream spec, or a **scenario** created under it. The host receives the branch-scoped spec and its scenarios, the upstream spec with its section headings and status, and the two-route vocabulary; it returns the route plus the section or scenario slug it names (see **§Choosing the route** in `.ductus/reference/fold.md`).

5. Invoke `gate-confirm` to confirm the routing with the user, and do not proceed until the user approves. This single confirmation is the consent for everything the fold performs, so it must name all of it: the route and its destination, the reopen when the upstream spec is `done` — conditional, since it happens only if the fold leaves the upstream spec an unchecked task or an unticked criterion, which a scenario route always does, and a body edit does when the folded text adds a criterion or the branch-scoped spec carries a scenario, whose task step 8 appends — the retirement of the branch-scoped directory, and the re-targeting that follows it. No write happens before this step (AC18).

6. <!-- llm:writeSpecBody --> **Body-edit route only** — fold the content into the named section of the upstream spec. The host receives the upstream spec and the target section and returns the section's new body. Skip this step on the scenario route.

7. Invoke `create-scenario` for each scenario the fold creates under the upstream spec: on the scenario route, the one step 4 named; and, on **either** route, one per scenario the branch-scoped spec carries, under its own slug, which is where step 11 re-points a reference to it. The primitive frames each file; write the folded content into it. **A slug already taken under the upstream spec is never overwritten.** Read the file that holds it: when it holds the scenario this fold carries, it is an interrupted run's earlier write, and the fold goes on; when it holds anything else, stop the fold here, before step 8, and report the slug, since continuing would re-point references to the carried scenario at an unrelated one, and step 12 would then delete the only copy. The operator renames the branch-scoped spec's scenario, since renaming the upstream one would leave its own inbound links on the carried scenario, and re-runs the fold. A reference that still names the old slug lands in `kept-tails` at step 11, and step 14 reports it for the operator to re-point.

8. Invoke `append-task` against the upstream spec's `tasks.md`, once per scenario created in step 7, referencing that scenario. A body-edit route that created no scenario appends no task: a body edit states what the spec means, which is not by itself work to implement.

<!-- audit:ignore-promotion -->
9. Reopen a `done` upstream spec only for owed work (host responsibility: the exec walker cannot bind the upstream spec, as step 13 says, nor branch on an answer, so this step dispatches nothing there). When the upstream spec's status is `done`, invoke the check-owed-work primitive against it once steps 6 to 8 have written, and invoke the set-status primitive with `from: done`, `to: in-progress` only when it reports `owes: true` — a scenario's appended task, or a criterion the body edit added unticked (§spec-lifecycle). A body-edit fold that adds neither leaves it `done`, and step 14 reports which happened. An upstream spec at any other status is left alone.

<!-- audit:ignore-promotion -->
10. Reset the upstream spec's review only with a reopen (host responsibility, for step 9's reason). Invoke the invalidate-review primitive against the **upstream** spec when it is `in-progress` after step 9, on either route: a fold brings in content and code its recorded review never saw. An upstream spec step 9 left `done`, or at a status below `in-progress`, keeps its review record as it stood (spec 084; **§Reopening a `done` upstream spec** in `.ductus/reference/fold.md`).

11. Invoke `rewrite-spec-links` with `from` set to the branch-scoped directory and `to` set to the fold target — the feature alone on a body-edit route, `{feature}/{scenario}` on a scenario route. Keep its `rewritten`, `refused`, `skipped`, `kept-tails` and `mentions` for step 14 (each from the file `lists-file` names when `form` is `file`), reading a zero `rewritten-count` against `examined` (AC22; **§Re-pointing inbound pointers** in `.ductus/reference/fold.md`). When `refused-count` is not zero (the count is inline in both forms; past the read size the list is in the file `lists-file` names), stop here: each listed file was changed by another writer after the sweep read it, so it was left as that writer made it and still names the retiring directory, and the sweep did not complete. Say so, and run this step again, which reads each file as it now is; go on to step 12 only once a run's `refused-count` is zero, since retiring the directory first would leave those pointers dangling. **The primitive refuses before writing anything when the target is not a home**, so a corpus-wide rewrite can never point every inbound link at a spec nobody can open; re-running could not repair that, since nothing would name the retiring directory any more.

12. Invoke `retire-feature` with the branch-scoped feature and its fold target to remove the directory (AC28). Step 10 needs the target and step 11 refuses without it, so a refusal *here* means the target went missing mid-run: surface it, and do not retry against a different target the operator did not record. `retired: false` means a re-run of an interrupted fold is converging. **This call never passes the opt-in that permits removing a sequential directory**, so a mistyped feature name during a fold still meets the refusal; only `/{project}:consolidate` passes it.

<!-- audit:ignore-promotion -->
13. Re-point every session that named the folded spec (host responsibility: the exec walker seeds its target from the session, so it cannot bind the folded and upstream features apart, and this step dispatches nothing there — the reduction is documented rather than silent). Call the retarget-sessions primitive with `from` set to the branch-scoped directory, `cause` set to `fold`, and the upstream feature and its repo-relative directory as `feature` and `path` — plus `scenario` and `scenario-path` when step 7 created one under it. §concurrent-features forbids leaving any session pointing at the directory step 12 removed. The shared default changes without a notice, since every unidentified process reads it, and the result names it among the sessions it changed. **Re-target rather than clear**: the content moved to a spec that continues the work, so the new target is a fact about where the operator's attention went.

<!-- audit:ignore-promotion -->
14. Report the fold (host responsibility): the spec folded, its route and destination, the upstream spec's status change if one happened, or that a `done` upstream spec stayed `done` with its review record because the fold left it owing nothing, each file `rewrite-spec-links` re-pointed with its count, and each one it refused and the run that finally re-pointed it — a root file such as `README.md` named like any other — each `skipped` file with its reason, each `kept-tails` reference, for the operator to point at the file that now holds what it meant, each `mentions` line, the sweep the operator owes — a present-tense pointer to the folded spec is corrected, a past-tense record of it stays — and the new session target — naming every session step 13 re-pointed (`default` is the shared default) and any `unreadable` file it left in place, since that file may still name the folded spec (spec 062, AC22). Then name the branch-scoped specs still un-folded from step 1's enumeration and ask whether to continue with the next, returning to step 3 for it. Recommend committing before continuing: the pre-commit hook regenerates `dependencies:` and `references:` from the rewritten body links (AC23).
