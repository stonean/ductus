# 084 — Reopen for owed code, review every code change Plan

Implements [084 — Reopen for owed code, review every code change](spec.md).

## Overview

The work lands in the order the spec's Resolved Questions fix: the reopen rule first, with `classify-spec-edit` retired in the change that leaves it unread (AC12), then the sweep of what states the old one, then the machinery that owes a review to every code change, then delivery.

- **The rule.** A shared owed-work helper and a `check-owed-work` primitive answer whether a spec holds an unchecked box or an unticked criterion. `/ductus:amend`, `/ductus:groom`, `/ductus:fold` and `/ductus:prune` reopen a `done` spec from that answer, `classify-spec-edit` is removed in the same change, and the constitution states the rule in place of the third back-edge.
- **The sweep.** Every live site that states the old rule is corrected. Under the new rule this leaves each `done` spec `done`.
- **Reviews owed wherever code changed.** The review window starts where the last review ended. Code no spec claims leaves open specs' scopes for a project record, `specs/review.md`, which is seeded at the update. A `check-unreviewed-code` primitive reports the code no review is going to read, and runs from a shipped pre-push hook, from the CI template and from `/ductus:status`. `/ductus:review` reviews a `done` spec's code as the whole review, and asks the owner before a ticked criterion changes. An update reports an adopter's backlog and leaves its configuration alone.
- **Delivery.** `check-artifact-size` stops naming per-fix reopens. Each surface owner gains its back-link, and this repository's own hook moves from `warn` to `block` once its backlog drains.

## Technical Decisions

### One owed-work test, read everywhere

`runtime/src/primitives/owed_work.rs` holds one test and does no I/O: given a spec's body and its `tasks.md` text, `owed_work` returns the unchecked boxes and the unticked criteria, and each half is callable alone. It calls the two parsers every current reader already uses: `read_tasks::unchecked_lines` (any heading, outside fences and HTML comments, 079's AC27 test) and `read_spec::parse_checkboxes` over "Acceptance Criteria". Four readers read the boxes through it: `check_artifacts::check_unchecked_boxes`, `check_review_gate::unchecked_task_block` and `read_tasks::refuse_while_tasks_are_open` (the refusal `write-review` and `write-analysis` share), each by `describe_unchecked`, which moves into the helper, and `measure_reopenings::waiting_features`. Two read the criteria: `check_artifacts::check_unticked_criteria` and `check_review_gate::unticked_criterion_block`. The helper returns the two lists apart, and each reader keeps the half it read. `waiting_features` reads the boxes alone: an `in-progress` spec's criteria stay unticked until its review, so the combined answer would leave no spec waiting on the gate. So no reader loses a property it enforced by construction (`QUAL-DELEG-001`).

`check-owed-work` (feature in; `owes`, `unchecked-boxes`, `unticked-criteria` out) exposes it, reading the working tree. Called with `all` in place of a feature, it reports every `done` spec with what each owes, and how many it examined; a call naming neither is refused (the agent's recommendation, adopted by the operator, 2026-11-24). So the CI template's step and Family 27 each call it once, and neither parses the corpus itself (§runtime-boundary, principle 3; the agent's recommendation, adopted by the operator, 2026-11-24). `measure-reopenings` (AC16) calls the same helper on blobs at each commit.

### The reopen decision reads state, not a diff

`framework/commands/amend.md` step 5, which reads what edits made outside the command left on disk, and `framework/commands/groom.md` step 7, after its confirmed spec edit, invoke `check-owed-work` in place of `classify-spec-edit`. On a `done` spec that owes work, each confirms through `gate-confirm`, naming the boxes and criteria, then calls `set-status` with `from: done`. The text that changes is fixed by the census in the spec's Resolved Questions:

- amend's step 5 and its intro;
- the criterion route's "meaningful body edit" wording;
- the Scope Boundaries line naming the diff read;
- the status-table row keyed on `classify-spec-edit`;
- groom's step 7, its step 3 and Scope line;
- `framework/reference/groom.md` §Reopening a `done` spec's last bullet.

Amend's scenario and criterion routes and groom's step 6 write `in-progress` directly today, after appending the task or criterion that owes the work. Each calls `check-owed-work` first too, so no command writes a reopen it did not read from the primitive.

AC3 reaches `/ductus:amend` as it reaches `/ductus:review`: when its criterion route would change a ticked criterion of a `done` spec, it asks the owner which side is correct before writing, by the prompt review's step 7.3 uses. The corpus sweep corrects ticked criteria in bulk, so it asks the owner once whether it may decide each, and records the answer in its commit.

`framework/commands/fold.md` step 9 invokes `check-owed-work` on the upstream spec after the fold's writes, and calls `set-status` with `from: done` only when it owes work. A scenario's appended task or an added unticked criterion reopens it; a body-edit fold that adds neither leaves it `done`. Step 10's `invalidate-review` runs when the upstream spec is `in-progress` after step 9, as today, and not on one left `done`, where a reset record would be review-state drift and fail the adopter CI template's audit-record gate. Step 5 is the fold's one consent and comes before any write, when a body-edit route cannot yet say whether the folded text adds a criterion. Its prompt therefore names the reopen as conditional, the upstream spec reopening if the fold leaves a task or an unticked criterion, and step 9 reports which happened. `framework/reference/fold.md` §Reopening a `done` upstream spec states this, and 051's AC24 and AC25 carry "Superseded in part (spec 084)" (the agent's recommendation, adopted by the operator, 2026-11-24). Code a staging spec folded below `done` brought stays owed without the reopen: `compute-review-scope` already puts it in the upstream spec's scope wherever the window starts (`WindowClaims::folded_in`), and `check-unreviewed-code` reports it under the upstream spec by the same rule.

`framework/commands/prune.md` step 9 decides on `check-owed-work`'s answer for the tree the run left, in place of a second `prune-plan` preview. `prune-plan`'s `reopen-required` stays as the preview's prediction, made before anything is written, of whether the plan's moves add an unchecked box (`read_tasks::adds_unchecked_box`, over the same parser). So every command that writes a reopen reads it from the one primitive (AC2). `/ductus:analyze --fix`'s revert differs in form only: it acts on `check-artifacts`' review-state drift, which reads the same helper.

A deferral to a `done` spec is decided the same way. `framework/commands/analyze.md` step 19, `framework/commands/review.md` step 10 for a deferral to another spec, and `framework/reference/implement.md`'s disposition task each perform a `done → in-progress` reopen on every deferral to a `done` spec today. Each invokes `check-owed-work` on that spec after the deferral's write and reopens it only when it owes work, with `framework/reference/analyze.md` §Finding dispositions and `framework/reference/review.md` §Deferrals stating it. A deferral's home is a task, a scenario or a criterion (§design-principles), each of which leaves owed work, so the primitive confirms what the route implies. A spec-edit home that leaves none reopens nothing, and the command says no gate will read it.

### A scenario's open question on a `done` spec is reported, not reverted

`/ductus:analyze --fix` sets a `done` spec back to `in-progress` when a scenario under it holds an unresolved open question (046's AC23 and AC30), and `/ductus:amend` writes such a question without reading the parent's status. That reopening holds no unchecked box and no unticked criterion. The finding stays: `check-artifacts`' `scenario-open-questions` family is blocking at `done`, with its two exits, and the gate still holds a spec short of `done` while one stands. Only the revert goes (the agent's recommendation, adopted by the operator, 2026-11-24). The spec reopens when the answer adds a task or a criterion. The text that changes:

- `framework/commands/analyze.md`'s description, its Purpose paragraph and its report step, which name the revert;
- the `--fix` paragraph of `framework/reference/analyze.md` §Scenario open questions (blocking at `done`, advisory otherwise);
- `docs/analyze.md`'s account of what `--fix` reverts;
- the constitution's new back-edge bullet, which says a scenario's question arriving after `done` is reported and reopens nothing, where the second back-edge said it took that edge.

The runtime's behavior does not change: the host performs the revert, and the family's finding is unchanged. Any doc comment in `check_artifacts.rs` that names the revert is corrected. 046's AC23 and AC30 carry "Superseded in part (spec 084)".

### The constitution, size-neutral

The current text of §spec-lifecycle's second and third back-edges, the "Any other edit there reopens" paragraph and the operational bullet on syncing a canonical record (1,651 bytes together, without their line terminators) is replaced by:

- a back-edge bullet, **Backward via owed code**, carrying `` `done` → `in-progress` `` and stating the state rule, who writes it (the writing command, `/review` on an untick or a deferral to its own spec's work), that a scenario's open question arriving after `done` is reported and reopens nothing, and that any other edit leaves a spec `done`;
- a shortened sentence on living specs.

The bullet keeps the `- **Backward via` form with a backticked `` `done` → `in-progress` ``, because `scripts/audit/cross-doc-consistency.sh` family 1c (`derive_constitution_back_edges`) derives its edge set from those bullets and holds it equal to `BACK_EDGES`. That script's `sub_1c` comment, which names "a meaningful body edit", is rewritten with it. `BACK_EDGES` is unchanged, since the edge is still `done → in-progress`.

The same commit corrects what the replacement leaves false: the count in "Three back-edges exist", the branch-scoped paragraph's "reopens by one of the two above", and §drift-prevention's two bullets that cite the mechanical class by §spec-lifecycle. The mechanical-sweep rule itself keeps two readers (below), so its statement, which the third back-edge held as cases (a) to (c), moves into §drift-prevention's sweep bullet within the same budget. If the budget does not hold it, it goes to `framework/reference/analyze.md` and the bullet points there.

§implement-phase's "reopened and corrected", §drift-prevention's bullets on a meaningful edit and on the diff computing a sweep's reopens, §cross-spec-impact's `classify-spec-edit` clause and §bug-handling's fourth route (AC10) change in the same commit. `framework/constitution.md` is 49,990 bytes against the 50,000-byte ceiling `scripts/lint-governing-size.sh` reads from the index, so the commit must leave it no larger. The 1,651 bytes removed are the budget. `AGENTS.md` (49,905 bytes) is held the same way.

### The review window starts where the last review ended

In `runtime/src/primitives/compute_review_scope.rs`, `resolve_base` takes the last counted review's end, `last_review_commit` walking `HEAD` without hiding the reopen commit, whatever the reopen.

- `held_base`'s clamp to `transition_parent` and the carried-in-record branch of `last_review_commit` are removed, and an uncommitted reopen no longer skips the last review: `transition_is_uncommitted` is consulted only when no counted review exists (AC7).
- One reopen still shapes the start: when the commit finishing the last review is also the commit that took the spec out of `done`, the window starts at that commit's parent. A record committed with a reopen is of a run from before it, and the reopen flow commits the status change with the work it authorises, which that run did not read (the agent's recommendation, adopted by the operator, 2026-11-24). Rejected: starting at that commit, which leaves the work committed with it read by no review.
- A held record whose `reviewed-against` names no commit the repository holds is passed over, as one stating none is: the removed clamp also sent such a record back to the reopen, and a window cannot start at a commit that is not there.
- With no counted review, an `in-progress` spec starts at the reopen as today: `HEAD` while the transition is uncommitted (`UncommittedTransition`), else the transition's parent (`Reopen`, `ReopenPastPartialReview`). Dropping `UncommittedTransition` outright would send that case to an older transition, or to an empty base for a spec never `in-progress` in history. A spec `done` at `HEAD` starts at the commit that set it `done` or added it as `done` (`BaseSource::ReachedDone`), found by a new `check_stuck` lookup beside `walk_reopenings` that reuses `status_of_commit` and takes the commit to read from. Before, that case resolved to the parent of the spec's last `in-progress` transition (`BaseSource::Reopen`), or to an empty base for a spec never `in-progress` in history (`BaseSource::None`). A `done` spec whose every recorded run was passed over keeps the reopening's start (`ReopenPastPartialReview`), so what those runs left unread stays in the window (079's AC5).
- The walk for the last review now reads all of history, so each record is parsed once per blob, and a shallow clone that cannot place the start is an error: one whose first commit already holds a run with no later commit ending one, or whose first commit lies inside the `done` pass the window would start at.
- The tests that asserted the old start are inverted, those for a review older than the reopen and for a held start before it among them; the one for a record in the reopen commit keeps its start, by the rule above. The module doc, `COMPUTE_REVIEW_SCOPE`, `BaseSource`'s doc comments, `framework/reference/review.md` §Window and coverage (its account of the window's start, and its sentence that an edit to a `done` spec's scenario or data model still reopens it), `framework/commands/review.md`'s `--since` flag row and `runtime/tests/golden/review-basic.jsonl` follow. `scripts/mutation/scope-attribution.py`'s mutants anchored on the clamp and the carried-in branch go with them.

### Code no spec claims leaves open specs' scopes

`attribute_window`'s branch for a commit with no claimant records its paths in `window.others`, so `split_changes` and `paths_owing_a_review` leave them out of the spec's scope. Recording nothing would keep them in, since both treat a path the walk placed nowhere as the spec's own. `fix_rounds::changed_only_by_other_specs`, which counts an unclaimed commit's paths as the reviewed spec's own, takes the same rule, so a review's fix rounds and its scope agree. `WindowClaims::take_unclaimed`, `ReadRecord`, `read_records` and `commits_between`, 081's crediting, have no other caller and are removed, with the mutation lists anchored on them: `scripts/mutation/review-read-credit.py`, `reviews-follow-what-changed.py` and `commit-counts.py`. `WindowCommits::unclaimed`, which 083's AC13 count reads, stays. Uncommitted paths still join the scope whole in `split_changes`, since uncommitted work has no commit to attribute (Trade-offs).

### The project record

`write-review` takes `feature` as optional. The subject, a feature or the project (`review_subject::ReviewSubject`), is resolved once at the top of `run_between`, and the record's path, its `spec:` label, the rounds directory and the list files' key all come from it, so there is one path through the primitive. Only an absent `feature` names the project: an empty one is refused. Without it the record is `{specs-root}/review.md`, written whole through `write_if_unchanged` like a feature's (`BE-TXN-002`), and skipping exactly what has no meaning there:

- `spec.md`, criteria and departures;
- `tasks.md`'s refusal;
- the contracts digest and analysis freshness.

`criteria` and `scenario` are refused when supplied, never dropped. The waiver and decision lists, the loop record, the `unread` refusal, the moved-scope guard and the empty-scope refusal apply as for a feature. The project record has no work of its own, so no deferral can name it and every deferral to a spec's pending work releases its finding. `process-waivers` and `process-decisions` take the same optional `feature`, since the record keeps both lists (the agent's recommendation, adopted by the operator, 2026-11-24); the project has no analysis record, and a departure waiver in its record has no entry and expires.

Its scope is the code no spec claims after the record's window start, at the run's head, computed by `check_unreviewed_code::project_scope`, which `check-unreviewed-code` later exposes. It reads `HEAD`'s tree and commits only, never the disk, so uncommitted unclaimed work stays with open specs (Trade-offs). A commit is the project's by `commit_attribution::project_share`, the unclaimed test `WindowCommits::of` counts by, and the specs that may claim one are listed from the commit's tree. What a project record leaves open or left unread is held in scope, as a spec's is. `diff-base` is the commit the window starts after: the record's own start when it matches, otherwise used verbatim as a spec's `--since` is, and the record states the one used.

The window starts as a spec's does, through `compute_review_scope::last_review_commit`, which takes the record and an optional spec: after the commit finishing the newest review that read its scope, which holds that run's fixes, or at the `reviewed-against` of one whose record is held (the agent's recommendation, adopted by the operator, 2026-11-24). Rejected: a reviewed record starting at its `reviewed-against`, which left every project review that fixed code owing another for its own fix. With no such review the window starts after the commit that added the record to history, found as `check_stuck::find_done_commit` finds a `done` pass. That is a seed's start: the install or update that writes the seed is committed after it, with the hooks, command files and lint configuration it brought, and those are code naming no spec (`is_project_artifact` counts only the spec root, `.ductus/`, `AGENTS.md` and `CLAUDE.md`). Starting at the seed's `reviewed-against` would stop the first push after every install and update on the framework's own files. A shallow cut at the record, or a commit holding none, places no window: a distinct state, never an empty scope.

A `seed` flag writes the record with `last-run: null` and `reviewed-against` at `HEAD`, the commit substituted by the runtime, never transcribed. It takes no other argument, a feature or a finding refusing it, and leaves an existing record, seeded or reviewed, as it is, saying which in its result, so a second run writes nothing. In a repository with no commit it states no `reviewed-against`, which nothing reads (the agent's recommendation, adopted by the operator, 2026-11-24).

`fix_rounds::RoundSubject` takes an optional feature. `round_paths` leaves out `{specs-root}/review.md` for the project subject (AC14), whose share of a landing is a spec's inverted: a commit no spec claims is its own, and one a spec claims is another review's. `validate_subject`, `subject_rounds_dir` (a project subject uses `.ductus/review-rounds/project`, a name no feature directory can take) and the callers `start_fix_rounds::run`, `brief_re_read::run`, `read_re_read::run` (with `round_dir_to_read`) and `check_prescriptions` accept it. A project round's brief names no spec, and no path of it is exempt.

`/ductus:review --project` reviews under the project record, and `--all` includes it. The command reads its window start and its paths from `check-unreviewed-code`'s `unclaimed`, so its text lands with that primitive and no second primitive reads the record (the agent's recommendation, adopted by the operator, 2026-11-24). The primitive reports one window, so the command's `--project` takes no `--since`, though `write-review` itself still takes any base verbatim (above): the agent's choice in task 9, not put to the operator, and the operator's to overturn. Under `ductus exec`, `project=true` drops the session's target from the walk's context and the walk stops at its scope step, saying to run the review through the host: the linear walk cannot put `unclaimed` in that step's place, and four more of its steps need a spec (the agent's recommendation, adopted by the operator, 2026-11-24). Rejected: a full exec walk of a project review, five special cases in the walker and a golden of its own. Rejected: `compute-review-scope` answering for the project when called with no feature, which gives the record a second reader and one more primitive an optional feature. The record's shape is in [data-model.md](data-model.md).

### `check-unreviewed-code`

A new primitive (`runtime/src/primitives/check_unreviewed_code.rs`) reads the committed tree at `at` (default `HEAD`), never the working tree, since a push carries commits, and the hook passes a pushed commit that need not be `HEAD`. So everything it reads comes from that tree: the spec directories, each spec's status, each claimant's `plan.md` and each review record. The attribution it shares with `compute-review-scope` reads the disk and `HEAD` today: `Attributor::new` lists the spec root on disk, `claimant_plan` reads `plan.md` from disk, and the walks start at `HEAD`. The shared function therefore takes its source and its walk's start: the working tree and `HEAD` for a review's scope, a commit and its tree for the check. The source is the `Attributor`'s, which lists the spec directories and reads each claimant's plan (`plan_of`); the caller walks its commits and hands each one's claims to `SpecShare::split`, which returns the spec's paths and the rest. It reports:

- **for each `done` spec**, the code paths changed after its window start (the decision above), by commits that claim it;
- **for code no spec claims**, the code paths changed after the project record's window start (the decision above).

Code is every path for which `is_project_artifact` is false. A path a commit gives an `in-progress` spec is not reported, while that spec is `in-progress`: the pass defers its review to that spec's gate and discharges nothing, so once the spec is `done` a path it shared with another `done` spec is that spec's to review. A changed path the commit's tree no longer holds is reported apart, in `deleted`, as `compute_review_scope::paths_owing_a_review` counts a spec's own deletions. A `done` spec's paths include what a staging spec folded into it below `done` brought (`folded_in`). A staging spec's commits that name it without touching its directory are claimed by no spec once a fold removes it, so the project's entry reports them too, beside what a fold below `done` brings its upstream and what the staging spec's own review read: it alone reports what the staging spec changed after that review and a commit naming it after the fold, and telling those from the rest would take the removed spec's record from history in each reader of the project's share (`unclaimed_code`, a project fix round's `project_share`, `commits-unclaimed`), so a fold re-reads that already-read code (the agent's recommendation, adopted by the operator, 2026-11-25). A consolidated spec leaves no `folds-into`, so its named-only commits are the project's alike. The per-path claim rule now inside `attribute_window` moves to `commit_attribution` as one function both read, so the two never disagree (`QUAL-DELEG-001`).

A subject whose history is cut (`PrimitiveError::ShallowHistory`, as `diff_cross_spec` already catches it) is reported under `not-examined` with its reason, never as clean (`QUAL-CLAIM-001`). So is the project subject at a commit whose tree holds no project record: the window for code no spec claims cannot be placed, and the reason names the seed. A branch cut before the seed is examined once it holds the seed's commit, by a merge or a rebase (Trade-offs). So is a spec whose `spec.md` the tree lacks or whose status does not read, and a subject whose record does not parse. With `since`, the primitive walks the first-parent commits from `at` back to `since` (`Revwalk::simplify_first_parent`) and counts those at which it reports code, for bar (c) (AC17), beside how many it walked and at how many a subject was not examined.

A `done` spec's window is placed by `compute_review_scope::base_at`, the history half of `resolve_base`, which takes the commit and reads the disk only for a review's uncommitted transition. Placing one walks all of history, and the check places one per `done` spec, so three things keep it to one read of each object: `check_stuck::TreeListings`, a scope in which the spec root's tree is listed once per tree id and a commit's history walked once; `Attributor::may_claim`, which names every spec that could claim a commit from its subject and the spec directories' tree ids, so the diff is read only for a commit that may be the spec's; and `last_review_commit` reading a parent's `spec.md` only when the record does not already answer. At `3caa8357` (5,071 commits, 75 `done` specs, release build, one run each) the check took 22.6 s before them and 2.0 s after, with the same result.

`[review] unreviewed-push` is read by a reader shaped like `fix_rounds::re_reads_off`, returning `block` when absent. Any other value fails with a new `PrimitiveError` variant naming the key and the file (`CFG-ENV-001`, `CFG-ENV-008`). The key joins the `review` section of `runtime/src/schema/config_keys.rs` and `framework/bootstrap/ductus.md` §Project Configuration, the inventory `the_installer_documents_the_registry` holds equal to it (`CFG-ENV-002`). The CLI exits 1 under `block` when anything is reported or not examined, and 0 under `warn`.

### `/ductus:review` on a `done` spec

`framework/commands/review.md` changes in three places, with `framework/reference/review.md` §Criteria step and §Deferrals stating the rules (AC3, AC5):

- **Step 7.3.** On a `done` spec, before `mark-criterion` changes a ticked criterion judged not met, it asks the owner which side is correct through `gate-confirm`, unless the invocation instructed it to decide. Code correct: the criterion's text is corrected, stays ticked, and goes to `write-review` judged met with its new text. Code wrong: it is unticked and `set-status` writes `in-progress`.
- **Step 10.** A deferral to the spec's own work appends its task and writes `in-progress`, as a deferral to another `done` spec's work does today. No gating finding is recorded without a disposition.
- **Scope Boundaries.** It gains the one status the review writes on its own spec.

066's scenario `criteria-are-verified-in-review` gains the `done`-spec exception (Cross-spec impact). `runtime/tests/golden/*-basic.jsonl` moved by a `review.md` edit are re-blessed alone and read.

### The pre-push hook

`framework/bootstrap/hooks/ductus-pre-push` (managed, `# managed-by: ductus` on line 2, `update`) and an outer stub `framework/bootstrap/hooks/pre-push` (`create`) follow `ductus-pre-commit`'s shape:

- ROOT from its own directory (spec 059);
- `.ductus/bin/ductus`, else `ductus` on `PATH`;
- a bounded `--help` capability probe for `check-unreviewed-code`;
- a missing or too-old runtime halts naming it and `--no-verify`.

For each line git gives on stdin, it runs `check-unreviewed-code --at <local sha>`, skipping a ref the push deletes (an all-zero local sha). The CLI's exit status is the hook's. Shipping it adds:

- two manifest rows in `framework/bootstrap/ductus.md` §Shared Files;
- `framework/bootstrap/ductus-procedure.md` §Hook Installation's file list, `chmod` line, report lines, manual-integration snippet and `pinned.files` paragraph;
- the `chmod` seed entries in the `claude` Agent Registry row, and `install.sh`'s copy of that row's seed, which `scripts/audit/installer-registry-parity.sh` compares with it;
- the same edits in `framework/bootstrap/govern.md`, held byte-identical to `ductus.md` by Family 21 (`scripts/audit/transitional-bootstrap-parity.sh`);
- a `scripts/audit/adopter-shell-behavior.sh` fixture for each AC8 case, a shallow clone among them.

`.github/workflows/runtime.yml`'s path triggers do not name it: no runtime test reads it, and Family 22 runs in framework-checks, which has no path filter, on every push (the agent's recommendation, adopted by the operator, 2026-11-24).

No migration is needed to install it: the `update` row and the `create` stub land on the next `/ductus` run.

### The seeded project record

A migration (`framework/migrations/project-review-record.md`, with its `framework/migrations.toml` entry ordered after every existing one) runs `write-review --seed` once per adopter. `framework/bootstrap/ductus-procedure.md` does the same on a fresh install, so the window for code no spec claims starts after the commit that carries the update (the agent's recommendation, adopted by the operator, 2026-11-24). This repository's seed lands with the project-record task.

After the seed, the migration runs `check-unreviewed-code` once and reports its result: each `done` spec owing a review with its path count, and `[review] unreviewed-push` with its two values. It writes no configuration. The default stays `block` for a project that updates as for a new one, so an adopter with a backlog reviews it or sets `warn` itself, knowing which (the agent's recommendation, adopted by the operator, 2026-11-24). A fresh install has no `done` spec to report. A later update's own commit is unclaimed code like any other, and owes a project review of the files it changed.

### The CI template and the audit backstop

`framework/templates/ci/adopter-generators.yml`'s checkout gains `fetch-depth: 0`, with the comment §design-principles requires of a history reader, and two steps:

- one runs `check-unreviewed-code` with its exit status, printing a `::warning` annotation per reported path under `warn` (AC9);
- one runs `check-owed-work`'s corpus form once and fails naming each `done` spec's boxes and criteria (AC15).

Both honour the job's commented `working-directory`. The pinned `DUCTUS_VERSION` predates both primitives, so the pin and its sha move in the release commit that ships them (AC13).

`scripts/audit/done-spec-criteria.sh` (Family 27) calls `check-owed-work`'s corpus form in place of its inline python parse of frontmatter and criteria. It then reports a `done` spec's unchecked box as it reports its criterion, through the runtime's one test. The corpus form keeps what the family does today: an untracked spec is skipped and counted, and an empty enumeration fails. A new `scripts/tests/test-done-spec-owed-work.sh`, modelled on `test-audit-path-containment.sh`'s `new_tree` and `run_family`, proves each condition fails present and passes absent. It is run from `.github/workflows/framework-checks.yml`.

### The status count

`dashboard` gains an `unreviewed-code` block from `check-unreviewed-code` at `HEAD`, with a state for found, none and not examined, beside `reopenings`. `pipeline_view::summary` renders it as a line before the Reopenings line, never omitted or cut (`whole`), as `reopenings_line` and `inbox_line` are.

### Measuring the bar

`measure_reopenings::OpenPass` gains `held_owed`, ORed at every commit the walk observes for the open pass. It reads `spec.md` and `tasks.md` at that directory version through `blob_holds` and the owed-work helper, memoized per blob as `statuses` is. A file that is not UTF-8, or a `spec.md` whose frontmatter does not split, shows no work, so an unread version counts toward never owed, the bar's defect, never toward held work. `Reopening` and each `not-counted` entry with a start carry `held-owed-work`, and `ReopeningCounts` carries `never-owed-count` over every closed reopening, counted or not, that carries it false (AC16; the agent's recommendation, adopted by the operator, 2026-11-25), so a reopening that ran no review or analysis still reaches the bar's defect count. A `no-in-progress-commit` entry has no pass to read, so it carries none and is not judged. Bar (c) is `check-unreviewed-code --since` (above), its `commits-reporting` plus `commits-not-examined`, since the hook acts on both states and the two counts are disjoint (the agent's recommendation, adopted by the operator, 2026-11-25).

### Retiring `classify-spec-edit`

In the change that moves amend, groom, fold and prune onto `check-owed-work`, the one that leaves it unread (AC12), `runtime/src/primitives/classify_spec_edit.rs` is removed from the seven registration sites `docs/developing-ductus.md` §Adding or removing a runtime primitive names:

- `PRIMITIVE_REGISTRY`;
- its description constant;
- its `#[tool]`;
- its `dispatch_primitive` arm;
- its `Command`;
- `framework/runtime-tools.txt`;
- its `runtime/tests/result_sizes.rs` case.

Its schema types go with them. `scripts/gen-configure-mcp.sh` regenerates the configure allow blocks. `merge_permissions::RETIRED_ALLOW` and `framework/bootstrap/configure/claude.md`'s list of tools the server no longer serves gain it, as 081's AC3 did for `check-review-freshness`.

Its mutants in `scripts/mutation/reopen-measure.py`, `review-022-task-46.py` and `line-citation-strip-066.py` go. So do its other mentions. Not comments: `descriptions::PRUNE_PLAN`'s compiled text, the `runtime/tests/mcp.rs` case on its description, and its row in `runtime/src/interpreter/mod.rs`'s `feature_argument_cases` table, a site beyond the seven. `check_artifact_size.rs`'s mention is `on_done`'s own doc, which goes with that function (below). Two compiled texts state the correction class the constitution no longer defines, and are reworded: `descriptions::RELOCATE_AUDIT_RECORDS` and the cross-spec-impact message in `check_review_gate.rs`. Comment-only:

- `runtime/src/mcp/server.rs`'s doc comment on the blocking-dispatch list;
- `relocate_audit_records.rs`;
- `read_tasks.rs`;
- `analyze_subjects.rs`;
- `line_citations.rs`;
- `criterion_verdicts.rs`;
- `prune_plan.rs`.

`mechanical_sweep.rs`'s module doc and `runtime/tests/mechanical_sweep_parity.rs` name the two readers the rule keeps, both through `analyze_subjects::exempt_renames`: the analysis record's freshness (`freshness_of`) and the review record's digest (`review_freshness`, called from `compute_review_scope::contracts_against_digest`). `framework/reference/review.md` §Record the run in `review.md` says `reviewed-against` is read for the rename exemption, which is true and stays (AC12).

### `check-artifact-size` names no per-fix reopen

`check_artifact_size::on_done` and the `OnDone` enum go, with `OnDone`'s variant docs in `runtime/src/schema/primitives.rs`, its mutants in `scripts/mutation/reopen-measure.py` and `scripts/mutation/plan-edit-takes-no-back-edge.py`: in a `done` spec no size fix leaves an unchecked box or an unticked criterion, so every cell reads the same. A `done` spec's warning states the state rule once, and the tests asserting the per-fix table are rewritten to it (AC18).

### This repository's hook

`.githooks/pre-push` runs the check with this repository's runtime, as `.githooks/pre-commit` locates it, the agent's recommendation adopted by the operator, 2026-11-24. `.ductus/config.toml` sets `[review] unreviewed-push = "warn"` while the measured backlog of `done` specs owing a review drains. The last task removes it, so 084 does not reach `done` at `warn`.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/owed_work.rs` | Create | The owed-work helper |
| `runtime/src/primitives/check_owed_work.rs` | Create | `check-owed-work` |
| `runtime/src/primitives/check_unreviewed_code.rs` | Create | `check-unreviewed-code`, its `[review] unreviewed-push` reader and the `since` count |
| `runtime/src/primitives/classify_spec_edit.rs` | Delete | Retired (AC12) |
| `runtime/src/primitives/compute_review_scope.rs` | Modify | Window start (AC7); unclaimed code out of scope (AC6) |
| `runtime/src/primitives/check_stuck.rs` | Modify | The commit that set a spec `done` |
| `runtime/src/primitives/commit_attribution.rs` | Modify | The per-path claim rule, shared |
| `runtime/src/primitives/write_review.rs`, `runtime/src/primitives/write_review/` | Modify | The project record and its seed |
| `runtime/src/primitives/review_subject.rs` | Create | The review subject: a feature or the project |
| `runtime/src/primitives/fix_rounds.rs`, `start_fix_rounds.rs`, `brief_re_read.rs`, `read_re_read.rs`, `check_prescriptions.rs`, `process_waivers.rs`, `process_decisions.rs`, `decisions.rs`, `write_analysis.rs` | Modify | A project subject (AC14); the shared claim rule in fix rounds |
| `runtime/src/primitives/measure_reopenings.rs` | Modify | `held-owed-work`, `never-owed-count` (AC16) |
| `runtime/src/primitives/dashboard.rs`, `pipeline_view.rs`, `render_status.rs` | Modify | The status count (AC9) |
| `runtime/src/primitives/check_artifacts.rs`, `check_review_gate.rs`, `read_tasks.rs`, `read_spec.rs` | Modify | Read the shared helper |
| `runtime/src/primitives/check_artifact_size.rs` | Modify | No per-fix reopen (AC18) |
| `runtime/src/primitives/merge_permissions.rs` | Modify | `RETIRED_ALLOW` |
| `runtime/src/primitives/mod.rs`, `mechanical_sweep.rs`, `analyze_subjects.rs`, `line_citations.rs`, `criterion_verdicts.rs`, `prune_plan.rs`, `relocate_audit_records.rs` | Modify | Module list and stale comments |
| `runtime/src/schema/primitives.rs`, `descriptions.rs`, `registry.rs`, `config_keys.rs` | Modify | Args, results, descriptions, registry, the config key |
| `runtime/src/main.rs`, `runtime/src/mcp/server.rs`, `runtime/src/interpreter/mod.rs` | Modify | Registration |
| `runtime/tests/` | Modify | `result_sizes.rs`, `mcp.rs`, `mechanical_sweep_parity.rs`, `golden/*-basic.jsonl`, new fixtures |
| `runtime/CHANGELOG.md` | Modify | `[Unreleased]` entries (AC13) |
| `framework/runtime-tools.txt` | Modify | Two primitives added, one retired |
| `framework/constitution.md` | Modify | §spec-lifecycle, §implement-phase, §drift-prevention, §cross-spec-impact, §bug-handling (AC1, AC10) |
| `framework/commands/amend.md`, `groom.md`, `analyze.md`, `review.md`, `status.md`, `prune.md`, `consolidate.md`, `help.md`, `fold.md` | Modify | The rule (amend, groom, fold, prune and each deferral decide from `check-owed-work`; `/analyze --fix` reverts for owed work alone), the review changes, the sweep |
| `framework/reference/groom.md`, `review.md`, `implement.md`, `fold.md`, `analyze.md`, `audit.md` | Modify | The same |
| `framework/bootstrap/hooks/ductus-pre-push`, `framework/bootstrap/hooks/pre-push` | Create | The shipped hook (AC8) |
| `framework/bootstrap/ductus.md`, `framework/bootstrap/govern.md`, `framework/bootstrap/ductus-procedure.md`, `framework/bootstrap/configure/claude.md`, `install.sh` | Modify | Manifest rows, installation, the config inventory, seeds, retired tools |
| `framework/migrations/project-review-record.md`, `framework/migrations.toml` | Create, Modify | The seed for existing adopters, and the backlog report |
| `framework/templates/ci/adopter-generators.yml` | Modify | Full history, two steps (AC9, AC15) |
| `scripts/audit/done-spec-criteria.sh`, `cross-doc-consistency.sh`, `ssot-invariants.sh`, `rename-sweep-residue.sh`, `adopter-shell-behavior.sh`, `run-all.sh`, `README.md` | Modify | Family 27, family 1c, the sweep, the hook fixture |
| `scripts/tests/test-done-spec-owed-work.sh` | Create | AC15's test |
| `scripts/mutation/reopen-measure.py`, `review-022-task-46.py`, `line-citation-strip-066.py`, `review-read-credit.py`, `reviews-follow-what-changed.py`, `commit-counts.py`, `scope-attribution.py`, `plan-edit-takes-no-back-edge.py`, `loop-inputs-screened.py`, `process-decisions-results-bounded.py`, `waiver-notices-flatten.py`, `waivers-bounded.py`, `project-record-084.py`, `unreviewed-code-084.py` | Modify, Create | Retired and re-anchored mutants; the project record's |
| `framework/migrations/audit-record-relocate.md`, `criterion-label-backfill.md` | Modify | The sweep: each calls its edit a correction under §spec-lifecycle |
| `.github/workflows/framework-checks.yml` | Modify | The new test |
| `.githooks/pre-push`, `.ductus/config.toml`, `specs/review.md` | Create, Modify, Create | This repository's hook, its `warn`, its seed |
| `AGENTS.md`, `README.md`, `specs/README.md`, `docs/slash-commands.md`, `docs/analyze.md`, `docs/developing-ductus.md` | Modify | The sweep |
| `specs/008-*`, `011-*`, `013-*`, `014-*`, `016-*`, `022-*`, `023-*`, `026-*`, `030-*`, `035-*`, `041-*`, `047-*`, `049-*`, `050-*`, `051-*`, `054-*`, `063-*`, `066-*`, `067-*`, `073-*`, `079-*`, `080-*`, `081-*` | Modify | The sweep and AC11's notes |
| `specs/018-*`, `020-*`, `022-*`, `046-*`, `050-*`, `051-*`, `057-*`, `066-*`, `067-*`, `079-*`, `081-*`, `083-*` | Modify | Back-links discharging `cross-spec-impact:` |

## Data Model

[data-model.md](data-model.md) defines the project record and the fields 084 adds to primitive results; once built, each primitive's compiled description is canonical for its own result (§drift-prevention).

## Trade-offs

- **Rejected: keeping `classify-spec-edit` to decide a reopen.** That kept the diff rule the spec replaces.
- **Rejected: fold as an exception to the state rule.** A body-edit fold would then be a reopening that never held owed work, which the bar's first measure counts as a defect.
- **Rejected: starting an existing adopter at `warn`, or seeding its `done` specs at the update.** The first leaves the move to `block` to someone remembering it; the second leaves code changed before the update never reviewed.
- **Rejected: a scenario's open question as a third reopen condition, or as a task `/ductus:amend` appends.** The first makes the rule inexact and the bar's first measure carry an exception; the second starts a reopen for each question, where reporting starts none.
- **Rejected: a new parse for owed work.** It would be a second copy of the two parsers the readers share. That includes extending Family 27's inline python scan to `tasks.md`, or listing `done` specs in the CI template by parsing frontmatter, either of which §runtime-boundary principle 3 rules out.
- **Rejected: reviewing unclaimed code inside open specs.** 081's crediting survives only as long as that does. The spec moves it to one record.
- **Rejected: a hook that parses `.ductus/config.toml` itself.** §runtime-boundary principle 3 rules out a shell parse, and the runtime is required.
- **Rejected: bar (c) from a hook log or the remote's push history.** See the spec's Resolved Questions.
- **Limitation: uncommitted unclaimed code still joins an open spec's scope.** Uncommitted work has no commit to attribute, as `split_changes` has it today.
- **Limitation: a framework update after the first owes a project review.** Its commit brings changed hooks and command files, which are code no spec claims; only the commit that adds the seed is covered.
- **Limitation: unclaimed code committed between a project review's last write and the commit of its record is owed to no review.** The next window starts after that commit, and the moved-scope guard reads only up to the write. A spec's window has the same gap.
- **Limitation: a branch cut before the seed is not examined.** Its tree holds no project record, so under `block` its push stops until it holds the seed's commit.
- **Limitation: an adopter with a backlog is stopped at its first push after the update.** The update's report says so before that push, with the key that turns the block into a warning.
- **Limitation: a `done` spec can hold an open scenario question until an analysis reports it.** Nothing but `/ductus:analyze` reads it once `--fix` no longer reverts.
- **Limitation: `since` mode is slow.** It runs the whole check once per first-parent commit in its window. It is a measurement, run when the bar is read, not on every push.
- **Limitation: the CI template cannot run either new step until a release ships the primitives.** Its pin moves in that release's commit.
- **Limitation: the estimate's blind spot stays in `held-owed-work`.** A box added and ticked in one commit is not seen.

## Open Questions Resolved

- **The project record's start** is its seed at the update, not the first commit. Seeding leaves 0 of this repository's 208 unclaimed code commits owed at landing, measured by `git log` over non-merge commits naming no spec and touching no spec directory and a path outside the project's artifacts, at `1590941c`. (The agent's recommendation, adopted by the operator, 2026-11-24.) A re-derivation in this plan's review counted 210 by one reading of "naming no spec" and 194 to 200 by others, so the test's exact form is the runtime's.
- **A seed's window starts after the commit that adds it** (the agent's recommendation, adopted by the operator, 2026-11-24). The seed is written before the install or update is committed, so a start at its `reviewed-against` would report that commit's own hooks and command files and stop the first push after every install and update. Rejected: accepting that and saying so; and counting installed framework files as artifacts, which needs a record of what was installed that an adopter's tree does not hold, and changes 081's definition of code.
- **This repository runs the hook**, so it meets the cost it ships (the agent's recommendation, adopted by the operator, 2026-11-24).
- **It lands at `warn` and moves to `block` once its backlog drains**, by the last task (the agent's recommendation, adopted by the operator, 2026-11-24). The backlog at `1590941c`: 9 `done` specs, 000, 012, 021, 025, 049, 060, 062, 069 and 081, with 20 commits. The method: non-merge commits from each spec's current `diff-base` to `HEAD` whose subject names the spec and that change a path outside the project's artifacts. It approximates the new window, which AC7 may shift. A re-derivation in this plan's review did not reproduce the figure from `diff-base` (51 specs, 170 commits) and came close from the last commit changing each spec's `review.md` (8 specs, 19 commits: the nine less 049), so the figure rests on that base rather than the one stated. The task that adds this repository's hook records what `check-unreviewed-code` reports.
- **A fold reopens its upstream spec only for owed work, and resets its review only when it reopens it** (the agent's recommendation, adopted by the operator, 2026-11-24). 051 reset the review on every fold so the upstream spec was reviewed again; code from a staging spec folded below `done` is now owed through the check, without the reopen.
- **An existing adopter updates to `block`, with its backlog reported** (the agent's recommendation, adopted by the operator, 2026-11-24). This repository's own backlog was measured above; an adopter's is reported by the update, which changes no configuration.
- **A commit with no project record is not examined for unclaimed code**, by `QUAL-CLAIM-001`: an absent record places no window, and reading it as nothing owed would pass every branch cut before the seed.
- **A scenario's open question on a `done` spec is reported and reopens nothing** (the agent's recommendation, adopted by the operator, 2026-11-24). An open question is an undecided requirement, not owed code; the spec reopens when its answer adds a task or a criterion.
- **A deferral to a `done` spec reopens it through the same primitive**, by AC2: three commands reopened on every such deferral, and none read the spec's state.
- **A missing runtime halts the hook** whatever the key says, since the runtime is bundled and therefore required (owner, 2026-11-24).

## Cross-spec impact

Each spec in `cross-spec-impact:` gains a back-link to this spec in its `spec.md`, read by `check-artifacts`' cross-spec-impact drift as discharging. For 079 and 081 it sits under `## See also`, where `derive-dependencies` induces no edge, since this spec's body links both and an edge back would close a cycle. The owed back-links:

- **018:** the pre-push hook beside its pre-commit hook.
- **020:** the review command's `--project` flag and its status write.
- **022:** the `dashboard` count and the configuration key.
- **046:** `--fix`'s revert on a scenario's open question, its AC23 and AC30.
- **050:** the constitution's new rule.
- **051:** fold's reopen and review reset on a `done` upstream spec, its AC24 and AC25.
- **057:** the CI template's new steps.
- **066:** the project record in the review record's schema, and its scenario `criteria-are-verified-in-review`'s `done`-spec exception.
- **041:** its scenario `a-suffixed-task-number-is-a-task`, whose numbering edge case no longer reports an order.
- **067:** `review-state-drift` read through the shared helper, and task consistency blocking a repeated number, not an order.
- **078:** its four loop primitives and its round directory take the project subject.
- **079:** AC5, AC15, AC25 and the retirement.
- **081:** AC10 and AC13.
- **083:** the unclaimed-commit count staying while unclaimed code leaves open specs' scopes.
