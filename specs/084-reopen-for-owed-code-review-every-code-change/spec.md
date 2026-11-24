---
status: draft
dependencies: [079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed]
cross-spec-impact: [050-constitution, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed]
next-criterion: 14
---

# 084 — Reopen for owed code, review every code change

A `done` spec reopens only when it owes code: an unchecked task or an unticked acceptance criterion. A code change owes a review whatever its spec's status, derived from history and recorded without a status change. The two are one definition: a spec is open only while code is owed, and a review is owed wherever code changed.

## Motivation

- §spec-lifecycle's third back-edge sent a `done` spec to `in-progress` on any edit under its directory that `classify-spec-edit` did not class as `correction` or `mechanical` ([079](../079-review-and-analysis-work-converges/spec.md)'s AC15 and AC25), so a typo in a scenario, any data-model edit and any other file reopened it, and each reopening owed an analysis, and often a review, before `done`.
- **Baseline.** `measure-reopenings --since 2026-11-11 --at 47cae941`, run with the installed runtime on 2026-11-23, reported 154 closed reopenings, 328 review and analysis runs (145 reviews, 183 analyses), 26 reopenings over 079's AC1 bound and 6 waiting, over 1,293 commits (`git rev-list --count --since=2026-11-11 47cae941`): 14.2 analysis runs per 100 commits. 079's Motivation measured 406 `done` → `in-progress` transitions over all history at `7b49fc80`; [081](../081-reviews-and-checks-follow-what-changed/spec.md)'s Motivation found nine of 12 gate-blocked specs first blocked by their own scenario or data-model edit.
- **Estimate of this spec's rule, same run.** 74 of the 154 reopenings held neither an unchecked box in `tasks.md` nor an unticked criterion in `spec.md` at any commit touching the spec's directory from the reopening's start to its end (`git rev-list start^..end -- specs/{feature}`, each file read with `git show` and matched on a line beginning `- [ ]`), so they would not have reopened; they held 34 of the 145 review runs and 79 of the 183 analysis runs. A box added and ticked in one commit is not seen by this method.
- `/{project}:review` was where the rule files were applied to code, and its result was read only by the gate on the way to `done`. Once a `done` spec reopened only for owed code, a fix bringing code in line with an existing criterion, a refactor and a dependency change would leave every spec `done` with nothing asking for their review. 081 already owed a review for code alone, and `framework/commands/review.md` step 1 already ran on a `done` spec, but nothing reported that a `done` spec's code had changed since its review; `compute-review-scope`'s description set the diff base no earlier than the spec's last reopen (079's AC5), past code committed while the spec was `done`; and code naming no spec was held only by specs that were open (081's AC10 and AC13; spec 083 counts a window's commits no spec claimed).

## Behavior

### A spec is open only while code is owed

- A `done` spec is `in-progress` exactly when its `tasks.md` holds an unchecked box (079's AC27 test: under any heading, outside a fence or comment) or its `spec.md` holds an unticked acceptance criterion. The rule reads the spec's state, not an edit's diff.
- So a new criterion reopens a `done` spec, a new scenario reopens it through the unchecked task it appends, and an edit to a scenario's text, a data model, prose or any other file does not. A code change with no spec change reopens no spec. The back-edge to `draft` on a new open question is unchanged.
- **Correcting a ticked criterion.** When a ticked criterion and the code disagree and the code is correct, the criterion's text is corrected, stays ticked, and the spec stays `done`; when the code is wrong it is a bug, and §bug-handling's scenario and task reopen the spec. Which side is correct is asked of the owner before the text changes, by default; an agent given an explicit instruction to decide, such as `--auto`, decides and says so in the commit.
- An edit that does not reopen owes nothing while the spec is `done`: no analysis and no review runs for it. When the spec next reopens, its analysis and review read the changed files, as `analyzed-digest` and `reviewed-digest` already arrange.
- The rule changes before any code is removed: no part of `classify-spec-edit` is deleted while a caller reads it.

### A review is owed wherever code changed

- A code change owes a review that can be run and recorded without changing any spec's status, and what is owed is derived from history, never remembered.
- One deterministic check reports the code nothing is going to review: code a `done` spec claims, or that no spec claims, with no committed review covering it. Code an `in-progress` spec claims passes, since that spec's gate reviews it before `done`, so work in hand owes no review per push.
- Code a `done` spec claims is reviewed under that spec, in its own `review.md`, and the spec stays `done` unless the review finds what reopens it. Code no spec claims is reviewed under one project record, `{specs-root}/review.md`, beside `inbox.md` and `system.md`, in the review record's schema. The project record is regenerated whole on each run, so it holds the latest run and its `reviewed-against` marks how far unclaimed code has been read; it has no status, no gate and no criteria, and the check is its only reader. Code naming no spec no longer needs every open spec to hold it.
- A review of a `done` spec's code is the whole review: the rule passes and the criteria step both run, with no second mode. A criterion judged not met is unticked, and the unticked criterion reopens the spec.
- A reopen does not move a spec's review window past code it claims that no committed review has read: the window starts where the spec's last review ended.
- A pre-push hook, shipped beside `framework/bootstrap/hooks/ductus-pre-commit`, runs the check and blocks the push by default. Pre-push and not pre-commit, since a review reads commits and its record is committed after the code it read. `[review] unreviewed-push` in `.ductus/config.toml` takes `"block"` or `"warn"`; absent means `block`, and any other value stops the hook with an error naming the key, never falling back to a warning.
- The hook is skippable and exists only where hooks are installed, so `framework/templates/ci/adopter-generators.yml` runs the same check as a step, as it backs the pre-commit hook today. `/{project}:status` shows the check's count.
- A fix to code an existing criterion already describes is reviewed with no scenario, so §bug-handling's fourth route adds a scenario only when the requirement needs more precision.

## Edge Cases

- A push let through as a warning, or with the hook skipped, leaves code unreviewed: the check keeps reporting it, and the window starting where the spec's last review ended still covers it.
- A `done` spec holding an unticked criterion or an unchecked box was already review-state drift, which `/{project}:analyze --fix` sets back to `in-progress` (`check_review_drift` and `check_unchecked_boxes` in `runtime/src/primitives/check_artifacts.rs`, read at `47cae941`; neither reads a digest or a criterion's text): this spec's condition. A review recording a MUST on a `done` spec is review-state drift too.
- A later `/{project}:analyze` of a `done` spec still judges the record it writes, so a blocking finding a non-reopening edit introduced is reported then as analyze-state drift, which `--fix` does not revert (`framework/reference/analyze.md` §Analyze state drift (blocking)).

## Acceptance Criteria

- [ ] AC1: `framework/constitution.md` §spec-lifecycle states that a `done` spec is `in-progress` exactly when its `tasks.md` holds an unchecked box or its `spec.md` an unticked acceptance criterion, in place of the third back-edge and of the correction and mechanical classes, and keeps the back-edge to `draft` on a new open question.
- [ ] AC2: Every command that edits a spec decides a reopen from the spec's state through one runtime primitive, and a test proves both directions: an edit to a `done` spec's scenario text, data model, prose or another file leaves it `done`, and an added criterion or an appended unchecked task reopens it.
- [ ] AC3: A command that finds a ticked criterion disagreeing with the code asks the owner which is correct before changing either, unless explicitly instructed to decide, such as by `--auto`, when it decides and says so in the commit; a criterion whose code is correct has its text corrected and stays ticked, and its spec stays `done`.
- [ ] AC4: One runtime primitive reports each path a `done` spec claims, or no spec claims, changed by a commit no committed review covers, and does not report a path an `in-progress` spec claims. Its result distinguishes finding none from being unable to examine its subject, and fixtures prove each direction for a `done` spec's code, unclaimed code and an `in-progress` spec's code.
- [ ] AC5: `/{project}:review` of a `done` spec's unreviewed code runs the rule passes and the criteria step, records in that spec's `review.md`, and leaves the spec `done` unless it unticks a criterion.
- [ ] AC6: Code no spec claims is reviewed under `{specs-root}/review.md`, in the review record's schema, regenerated whole on each run, with no status, gate or criteria, and `compute-review-scope` no longer puts a path no spec claims in an open spec's scope.
- [ ] AC7: A spec's review window starts where its last review ended, whether or not the spec reopened since, and a fixture proves it: code committed while the spec is `done`, then a reopen, and the next review's scope holds that code.
- [ ] AC8: A pre-push hook shipped and installed beside `framework/bootstrap/hooks/ductus-pre-commit` runs the check and exits non-zero naming the code it reports; with `[review] unreviewed-push = "warn"` it prints the same report and exits zero; absent, the key means `"block"`; any other value stops the hook with an error naming the key and the file. A test proves each of the four.
- [ ] AC9: `framework/templates/ci/adopter-generators.yml` runs the check as a step, and `/{project}:status` shows its count on every run.
- [ ] AC10: §bug-handling's fourth route adds a scenario only when the requirement needs more precision, and says a fix to code an existing criterion describes is reviewed without one.
- [ ] AC11: 079's AC5 and AC15 and 081's AC10 and AC13 carry "Superseded in part (spec 084)", and no live artifact in `AGENTS.md`'s sweep-target set states that an edit's diff reopens a `done` spec, each site read whole rather than found by search alone.
- [ ] AC12: No part of `classify-spec-edit` is removed while a caller reads it, and each part no caller reads once the rule changes is removed in the change that leaves it unread.
- [ ] AC13: Every change this spec makes to how the pipeline behaves reaches adopters through what a `ductus-v<version>` release ships (the runtime, `framework/commands/`, `framework/reference/`, `framework/constitution.md`, `framework/bootstrap/` and `framework/templates/`), never only through this repository's `AGENTS.md`, `scripts/` or workflows.

## Applicable Rules

- `QUAL-CLAIM-001` — the unreviewed-code check distinguishes finding none from being unable to examine its subject.
- `QUAL-DELEG-001` — the reopen decision moves from `classify-spec-edit`'s diff classes to the owed-work test review-state drift applies.
- `QUAL-TEST-001` — each fixture fails with its behavior removed.
- `CFG-ENV-001` — `[review] unreviewed-push` is optional, with `block` as its named default.
- `CFG-ENV-002` — the key is listed in the configuration inventory.
- `CFG-ENV-008` — a value other than `block` or `warn` fails naming the key, never falling back to the default.

## Open Questions

- In what order does the work land: the reopen rule's change first, so the sweep's edits to `done` specs' scenarios and data models reopen none of them, or the sweep first? Answer with what each ordering starts in reopens and review runs; the groom handoff recommended the rule first.
- When `/{project}:review` judges a `done` spec's ticked criterion not met, does it ask the owner which side is correct before unticking, as the correction rule's default asks, or untick it and leave any correction to the reopened spec, as the whole-review rule says?
- Which command writes `in-progress` when a `done` spec comes to hold owed work: the command whose write adds it, as `/{project}:amend` and `/{project}:groom` do for a scenario today, `/{project}:review` after unticking a criterion (it changes no status today), or `/{project}:analyze --fix`'s review-state drift alone? And does CI run the `done`-spec drift checks, so a missed write is caught without anyone running `/{project}:analyze`?
- `compute-review-scope`'s `review-owed` is true for a ticked criterion whose text the last review did not judge met (066 task 27; 066's scenario `criteria-are-verified-in-review`), which a corrected criterion is, but only the gate reads it: does the unreviewed-code check also report a corrected criterion on a spec that stays `done`?
- Does any reader of the spec root take a top-level `{specs-root}/review.md` for part of a feature?
- Which of `classify-spec-edit`'s callers decide a reopen, and which reuse its mechanical-edit rule only for record freshness (`framework/commands/amend.md` step 5, `framework/commands/groom.md` step 7, `framework/commands/prune.md`, `framework/commands/consolidate.md`, and `analyze_subjects.rs`, `mechanical_sweep.rs`, `line_citations.rs`, `check_artifact_size.rs`, `criterion_verdicts.rs`, `prune_plan.rs`, `read_tasks.rs` and `relocate_audit_records.rs` under `runtime/src/primitives/`), and what does the new rule leave each doing?
- How is the bar below measured: does `measure-reopenings` report a closed reopening that never held owed work (bar a), and how are the pushes the hook blocked or warned on counted (bar c), since a blocked push leaves no commit and the runtime keeps no state the markdown cannot reconstruct?
- Does the CI step honour `[review] unreviewed-push`, or always fail on what the check reports?
- Which specs beyond 050, 066, 079 and 081 own a surface this spec changes (the hooks, the analyze checks, the status view, the CI template, the runtime core) and belong in `cross-spec-impact:`?

## Resolved Questions

- **Behavior's rules** are the owner's decisions of 2026-11-23, recorded in inbox items 1 and 2 at `a5001e65`: the state rule, the correction of a ticked criterion, an edit that does not reopen owing nothing, the one check with its pre-push hook, CI step and status count, where each review is recorded, a `done` spec's review being the whole review, and the config key.
- **One new spec for both items**, rather than new criteria on 079, or on 079 and 081: the agent's recommendation through `/ductus:groom`, adopted by the operator, 2026-11-24. Routed here, the items reopen no spec; new criteria on 079 would reopen it, and a split would reopen 079 and 081.
- **Landing order.** This spec lands before the three inbox items logged after its two at `a5001e65` (rule-assessment cost, a cost statement in §design-principles, governing-size ceilings), so its effect is readable before those land (owner, 2026-11-23).
- **The bar for starting those three (owner, 2026-11-23).** Over the two weeks from the day this spec reaches `done`, read from the commit that sets it, by the Motivation's baseline and estimate methods: (a) no closed reopening that never held an unchecked box or an unticked criterion, which the rule makes impossible, so any is a defect in this change; (b) analysis runs per 100 commits down by at least a third from 14.2; (c) the pushes the hook blocked or warned on, counted, with no target, since that is the cost this spec adds. A rate and not a count, since commits per week had been rising. If (b) misses, the cost lies in how specs reference each other rather than in the reopen rule, and that is examined before the three items start.
