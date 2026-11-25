# 084 — Reopen for owed code, review every code change Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## Phase A — The rule

### 1. The owed-work helper and `check-owed-work`

Model: opus, with fable as advisor (owner, 2026-11-24).

- [x] Create `runtime/src/primitives/owed_work.rs` calling `read_tasks::unchecked_lines` and `read_spec::parse_checkboxes`, and move `check_artifacts::check_unchecked_boxes`, `check_artifacts::check_unticked_criteria`, `check_review_gate::unchecked_task_block`, `check_review_gate::unticked_criterion_block` and `measure_reopenings::waiting_features` onto it, each keeping the half it read: `waiting_features` the boxes alone
- [x] Add `check-owed-work` at the seven registration sites `docs/developing-ductus.md` names, with its compiled description
- [x] Its corpus form: with `all`, every `done` spec that owes work, `examined-count` and `skipped`
- [x] Test both directions and add a mutant per condition, each killed

- **Done when**: `check-owed-work` reports a `done` fixture's unchecked box and unticked criterion, reports `owes: false` for one with neither, its corpus form names only the owing `done` specs and counts every one examined, every former reader passes its own tests through the helper, and the mutants are killed.

### 2. Commands reopen from state, and `classify-spec-edit` retires

Model: opus, with fable as advisor (owner, 2026-11-24).

`check-owed-work` refuses a call naming both `feature` and `all`, and `ductus exec` dispatches a backticked name with the walker's context, which holds the session target's `feature`: read what else the context carries before backticking a step. A step asking about a spec other than the target, as fold's upstream spec or a deferral's target, leaves the name unbackticked behind `<!-- audit:ignore-promotion -->`.

- [x] `framework/commands/amend.md` step 5, its intro, its criterion route's wording, its Scope Boundaries line and its status-table row call `check-owed-work` in place of `classify-spec-edit`; its scenario and criterion routes call it before they write `in-progress`
- [x] `framework/commands/groom.md` steps 3 and 7 and its Scope line, and `framework/reference/groom.md` §Reopening a `done` spec, the same; its step 6 calls it before it writes `in-progress`
- [x] A ticked criterion amend would change on a `done` spec, disagreeing with the code, is asked of the owner first (AC3)
- [x] `framework/commands/fold.md` step 9 reopens a `done` upstream spec only when `check-owed-work` reports owed work, step 10 resets its review only when it is `in-progress` after step 9, step 5's consent names the reopen as conditional, and `framework/reference/fold.md` §Reopening a `done` upstream spec says so
- [x] `framework/commands/prune.md` step 9 decides on `check-owed-work` for the tree the run left; `prune-plan`'s `reopen-required` stays the preview's prediction
- [x] A deferral to a `done` spec reopens it only when `check-owed-work` reports owed work: `framework/commands/analyze.md` step 19, `framework/commands/review.md` step 10, `framework/reference/implement.md`'s disposition task, and `framework/reference/analyze.md` §Finding dispositions
- [x] `/ductus:analyze --fix` no longer reverts a `done` spec for a scenario's open question: `framework/commands/analyze.md`'s description and report step, `framework/reference/analyze.md` §Scenario open questions and `docs/analyze.md`; the finding and the gate's block stay
- [x] Remove `classify_spec_edit.rs` and its seven registration sites, its schema types, `framework/runtime-tools.txt`'s line, and regenerate `framework/bootstrap/configure/*.md`
- [x] `merge_permissions::RETIRED_ALLOW` and `framework/bootstrap/configure/claude.md`'s list of tools no longer served
- [x] Its mutants, its comment-only mentions, `PRUNE_PLAN`'s text, the `runtime/tests/mcp.rs` case and the `feature_argument_cases` row; `RELOCATE_AUDIT_RECORDS` and the gate's cross-spec-impact message reworded; `mechanical_sweep.rs`'s module doc and `runtime/tests/mechanical_sweep_parity.rs` name the rule's two remaining readers
- [x] `scripts/gen-claude-commands.sh` re-run, and any golden the edits move re-blessed alone and read

- **Done when**: no command calls `classify-spec-edit`, the runtime serves none on the CLI or MCP and `framework/runtime-tools.txt` lists none, amend, groom, fold, prune and the three deferral paths reopen a `done` spec only when `check-owed-work` reports owed work, a walk of amend's and groom's steps on a `done` fixture with a scenario-text edit leaves it `done`, a body-edit fold into a `done` fixture leaves it `done` with its review record unchanged, a `done` fixture with a scenario's open question keeps its status under `--fix` and is still reported, and `cargo test` passes with the module gone.

### 3. The constitution states the rule

Model: opus, with fable as advisor (owner, 2026-11-24).

Task 2 found that only the retired classifier applied the mechanical rule's cross-service-reference case (b): `derive_references::ReferenceScrub` went dead with it, and neither remaining reader, the analysis record's freshness nor the review record's digest (`mechanical_sweep.rs`, `analyze_subjects.rs`), reads a reference. So the statement moved into §drift-prevention carries the sweep (a) and the line-number strip (c) alone. Four live sites cite the rule as `§spec-lifecycle case (a)` or `case (c)` and are re-pointed in the same commit: `runtime/src/primitives/mechanical_sweep.rs` (twice), `runtime/src/primitives/analyze_subjects.rs`, `scripts/audit/sweep-target-manifest-parity.sh`.

- [x] §spec-lifecycle: one `- **Backward via owed code**` bullet in place of the second and third back-edges, the "Any other edit there reopens" paragraph and the canonical-record bullet, naming `/review`'s one status write
- [x] §implement-phase, §drift-prevention, §cross-spec-impact and §bug-handling's fourth route (AC10)
- [x] "Three back-edges exist", "one of the two above" and §drift-prevention's two bullets citing the mechanical class; the mechanical-sweep rule's statement moves into §drift-prevention, or to `framework/reference/analyze.md` if the budget does not hold it
- [x] `scripts/audit/cross-doc-consistency.sh` family 1c's `sub_1c` comment; `BACK_EDGES` is unchanged

- **Done when**: `scripts/lint-governing-size.sh` passes on the staged constitution, family 1c derives the same edge set, `run-all.sh` exits 0 after the commit, and no sentence of the constitution says an edit's diff reopens a `done` spec.

## Phase B — The sweep

### 4. Sites outside the spec corpus

Model: opus, with fable as advisor (owner, 2026-11-24).

Blocked on task 3: the sweep corrects what states the old rule only once the new one stands.

Task 3 moved the mechanical-sweep rule from §spec-lifecycle to §drift-prevention, without the cross-service-reference case. Two sites here still point at it in §spec-lifecycle: `AGENTS.md`'s live-artifact-set bullet ("the scope of §spec-lifecycle case (a)") and `specs/README.md`'s "mechanical-vs-meaningful boundary". Re-point both by meaning.

- [x] Read each whole and correct it: `AGENTS.md` (held to its ceiling), `README.md`, `specs/README.md`, `docs/slash-commands.md`, `docs/analyze.md`
- [x] `framework/commands/prune.md`, `consolidate.md`, `help.md`, `fold.md`; `framework/reference/implement.md`, `review.md`, `fold.md`
- [x] `scripts/audit/ssot-invariants.sh`, `rename-sweep-residue.sh`, `scripts/audit/README.md`
- [x] `framework/migrations/audit-record-relocate.md` and `criterion-label-backfill.md`, whose third steps call their edit a correction under §spec-lifecycle

- **Done when**: no site outside the spec corpus states that an edit's diff reopens a `done` spec, `scripts/stale-literals.sh` finds none, and the census's sites in the spec's Resolved Questions are each read whole.

### 5. Sites in the spec corpus

Model: opus, with fable as advisor (owner, 2026-11-24).

Blocked on task 3: under the old rule each edit here to a `done` spec's scenario or criterion would reopen it.

Task 4 deleted three `AGENTS.md` bullets that stated the retired rule and re-keyed 050's `plan.md` classification table to match (W21 re-keyed; W14, W15 and W24 removed). 050's plan prose introducing that table still calls W14 and W15 "already promoted" as case (b) and a correction.

Add 000 to the sweep: its scenario `criterion-route-after-draft` says a `done` spec's criterion route "either takes the documented back-edge or refuses", where task 2 gave `/ductus:amend`'s criterion route a third case, the owner-confirmed correction of a ticked criterion, which reopens nothing; and its Resolved Questions derive the reopen from the third back-edge.

- [x] Ask the owner once whether this sweep may decide each ticked criterion's correction (AC3), and record the answer in the commit
- [x] Correct the sites in 008, 011, 013, 022, 023, 026, 030, 035, 041, 047, 049, 050, 051, 054 and 063, each read whole
- [x] And those the plan's review found by a search for `classify-spec-edit`: 014, 016, 066, 067, 073 and 080
- [x] 079: AC5, AC6, AC15 and AC25, with AC27's reader list corrected, and its other sites; 081: AC10, AC13 and AC14; 066: AC6's reader list corrected; 051: AC24 and AC25; 046: AC23 and AC30; each criterion's note reads "Superseded in part (spec 084)"

- **Done when**: AC11 holds, `grep -rn classify.spec.edit` over the live-artifact set finds only historical records and `check_artifact_size.rs`, which task 17 clears, every swept spec's status is unchanged, and `check-artifacts` reports no new finding on any of them.

## Phase C — Reviews owed wherever code changed

### 6. The review window starts where the last review ended

Model: fable (owner, 2026-11-24).

- [x] `compute_review_scope::resolve_base` and `last_review_commit` stop hiding the reopen; `held_base`'s clamp and the carried-in branch go, and `UncommittedTransition` stays only for a spec with no counted review
- [x] A `check_stuck` lookup for the commit that set a spec `done` or added it as `done`, for a `done` spec no review recorded
- [x] Invert the tests asserting the old start; update the module doc, `COMPUTE_REVIEW_SCOPE`, `BaseSource`'s docs, `framework/reference/review.md` §Window and coverage, `framework/commands/review.md`'s `--since` row, `runtime/tests/golden/review-basic.jsonl` and `scripts/mutation/scope-attribution.py`

- **Done when**: AC7's two fixtures pass and fail with the change reverted.

### 7. Code no spec claims leaves open specs' scopes

Model: fable (owner, 2026-11-24).

- [x] `attribute_window`'s no-claimant branch records its paths in `window.others`, so they leave the scope; `take_unclaimed`, `ReadRecord`, `read_records` and `commits_between` go, with `scripts/mutation/review-read-credit.py`, `reviews-follow-what-changed.py` and `commit-counts.py`
- [x] Move the per-path claim rule into `commit_attribution`, taking its source (the working tree or a commit's tree) and its walk's start, for `fix_rounds::changed_only_by_other_specs` and task 9 to share
- [x] Rewrite 081's AC13 and AC14 fixtures to the new rule

- **Done when**: a fixture with a code commit naming no spec leaves it out of an `in-progress` spec's scope, `commits-unclaimed` still counts it, and AC6's second half holds.

### 8. The project record

Model: fable (owner, 2026-11-24).

- [x] `write-review` with no `feature` writes `{specs-root}/review.md` as `data-model.md` states, and `--seed` writes the seed
- [x] The function that finds the code no spec claims after the project record's window start, a seed's starting after the commit that added it; the project review's scope reads it, and task 9 exposes it
- [x] `fix_rounds::RoundSubject` takes an optional feature, `round_paths` leaves out `{specs-root}/review.md`, and its callers, `read_re_read::run` among them, accept a project subject (AC14)
- [x] Seed this repository's `specs/review.md`

- **Done when**: AC6 and AC14 hold, a project review over a fixture's unclaimed commit records it, and its fix rounds leave its own record out.

### 9. `check-unreviewed-code`

Model: fable (owner, 2026-11-24).

Blocked on tasks 6 to 8: it reads their window, their claim rule and the project record.

Handed on from task 8, which built `check_unreviewed_code.rs`'s `project_window`, `unclaimed_code` and `project_scope` and reads them at `HEAD` from `write-review`. Owed here:

- `Attributor::at_tree` lists the specs from the commit's tree, and `plan_of` still reads the disk: a `done` spec's share at a commit needs its claimants' plans from that tree.
- `project_window` and `compute_review_scope::last_review_commit` take `at`, and task 8 only ever passes `HEAD`: the fixture with an `at` that is not `HEAD` is their first test off it.
- `NotPlaced` carries no reason text: `not-examined` words it, and the reason for `NoRecord` names the seed.
- `unclaimed_code` leaves out a path the commit's tree no longer holds. A deletion is still a change, so report it as `compute_review_scope::paths_owing_a_review` counts a spec's own deletions.
- Under `ductus exec` the walker seeds the session's `feature` into every primitive, so `--project` has to drop it, or each review primitive reviews the targeted feature.
- `framework/commands/review.md`'s Scope Boundaries names `specs/NNN/review.md` and `.ductus/review-rounds/{feature}/` as what a run writes: a `--project` run writes `{specs-root}/review.md` and `.ductus/review-rounds/project/`.

- [x] The primitive at `at`, reading spec directories, statuses, plans and review records from that commit's tree and never the disk, with `done-specs`, `unclaimed`, `not-examined` and `mode`, and its seven registration sites
- [x] `[review] unreviewed-push`'s reader, its `PrimitiveError` variant, `runtime/src/schema/config_keys.rs` and `framework/bootstrap/ductus.md` §Project Configuration
- [x] The `since` count over first-parent commits (AC17)
- [x] `framework/commands/review.md` gains `--project`, and `--all` includes it: a project review reads its window start and paths from this primitive's `unclaimed` (moved here from task 8, which builds what it writes; owner, 2026-11-24)
- [x] Fixtures for AC4's directions, among them a shallow history, a tree holding no project record, a staging spec folded below `done`, an `at` that is not `HEAD`, and a seed whose adding commit holds code

- **Done when**: AC4 and AC17 hold, an invalid key value fails naming the key and the file, and each fixture fails with its behavior removed.

### 10. `/ductus:review` on a `done` spec

Model: opus, with fable as advisor (owner, 2026-11-24).

- [x] Step 7.3 asks the owner before a `done` spec's ticked criterion judged not met changes, corrects or unticks it, and writes `in-progress` on an untick
- [x] Step 10 writes `in-progress` on a deferral to the spec's own work, and records no gating finding without a disposition
- [x] Scope Boundaries, `framework/reference/review.md` §Criteria step and §Deferrals, and goldens re-blessed and read

- **Done when**: AC3 and AC5 hold in the command and reference text, and the goldens match.

### 11. The pre-push hook

Model: opus, with fable as advisor (owner, 2026-11-24).

Handed on from task 9: the CLI's exit status is the hook's (`unreviewed_code_blocks` in `runtime/src/main.rs`), and it prints the report on stderr, one line per subject, path and subject not examined, before the result is bounded. `runtime/tests/check_unreviewed_code_cli.rs` proves each AC8 case on the binary but the shallow clone and the missing runtime, which are the hook fixtures' to prove.

- [x] `framework/bootstrap/hooks/ductus-pre-push` and `framework/bootstrap/hooks/pre-push`, shaped as the pre-commit pair
- [x] `framework/bootstrap/ductus.md` §Shared Files rows and the `claude` registry row's `chmod` seed, mirrored in `framework/bootstrap/govern.md` and `install.sh`; `framework/bootstrap/ductus-procedure.md` §Hook Installation
- [x] A `scripts/audit/adopter-shell-behavior.sh` fixture for each AC8 case; `.github/workflows/runtime.yml`'s path triggers stay as they are, since no runtime test reads the hook (the agent's recommendation, adopted by the operator, 2026-11-24)
- [x] A scratch-adopter bootstrap, per `docs/developing-ductus.md`'s installer section

- **Done when**: AC8 holds on an adopter-shaped tree, each case proven, a shallow clone and a missing runtime among them.

### 12. The seed migration

Model: opus, with fable as advisor (owner, 2026-11-24).

- [x] `framework/migrations/project-review-record.md` and its `framework/migrations.toml` entry, ordered after every existing one
- [x] `framework/bootstrap/ductus-procedure.md` seeds the record on a fresh install
- [x] After the seed, the migration runs `check-unreviewed-code` once and reports the backlog with `[review] unreviewed-push` and its two values, writing no configuration

- **Done when**: a scratch adopter behind the release gains a seeded `{specs-root}/review.md` on update and on a fresh bootstrap, a second run writes nothing, and one with a `done` spec's code changed since its review is shown that spec and the key with its configuration unchanged, and the check at the commit carrying the update reports none of that commit's files.

### 13. The CI template and the audit backstop

Model: opus, with fable as advisor (owner, 2026-11-24).

Both call `check-owed-work --all`, which names each spec it cannot judge in `skipped`. Family 27 fails today on a spec it cannot read or that resolves outside the repository, and counts an untracked one, so the step and the script fail on every skip reason but `untracked` (`QUAL-CLAIM-001`).

Handed on from task 9: under `warn` the CLI exits 0 with the same stderr report, a path line being `check-unreviewed-code:   {path}`, so the step's per-path annotation reads those lines or the result's `done-specs` and `unclaimed`.

- [x] `framework/templates/ci/adopter-generators.yml`: `fetch-depth: 0` with its comment, the unreviewed-code step, and a `done`-spec owed-work step calling `check-owed-work`'s corpus form once
- [x] `scripts/audit/done-spec-criteria.sh` calls the same corpus form in place of its python parse; `scripts/tests/test-done-spec-owed-work.sh`, run from `framework-checks.yml`

- **Done when**: AC9's CI half and AC15 hold, each step run by `scripts/run-workflow-steps.py` on a fixture fails with its condition present and passes without it.

### 14. The status count

Model: opus, with fable as advisor (owner, 2026-11-24).

Handed on from task 9: the check took 2.0 s here at `3caa8357` (5,071 commits, 75 `done` specs, release build, one run), and `dashboard` runs on every `/ductus:status`, so measure the view's time with the block added. `check_unreviewed_code::compute` returns every list whole.

- [x] `dashboard`'s `unreviewed-code` block, `pipeline_view`'s line, `framework/commands/status.md` and `docs/slash-commands.md`

- **Done when**: `/ductus:status` shows the count in each of its states on every run.

### 15. Measuring the bar

Model: opus, with fable as advisor (owner, 2026-11-24).

- [x] `measure_reopenings`' `held-owed-work` and `never-owed-count`, with a fixture for each direction

- **Done when**: AC16 holds, and `measure-reopenings --since 2026-11-11 --at 47cae941` reproduces the estimate's 74 never-owed reopenings or the commit names why it differs.

### 16. This repository's hook

Model: opus, with fable as advisor (owner, 2026-11-24).

Handed on from task 9: at `3caa8357` the check reports 9 `done` specs (000, 012, 021, 025, 049, 060, 062, 069 and 081) with 81 paths, the nine the plan measured, and 023 and 048 as not examined, which task 23 owns. A subject not examined blocks as reported code does, so `block` (task 19) waits on task 23 too.

- [x] `.githooks/pre-push`, and `[review] unreviewed-push = "warn"` in `.ductus/config.toml`

- **Done when**: a push from this checkout runs the check and warns on the backlog, and the commit records the `done` specs and commit count it reports.

### 24. `check-unreviewed-code --since` over the bar's window

Model: opus, with fable as advisor (owner, 2026-11-25).

- [x] `runtime/src/primitives/check_unreviewed_code.rs`, `runtime/src/primitives/check_stuck.rs` — `--since HEAD~30` at `3caa8357` took 41.7 s and 297 MB peak (`/usr/bin/time -l`, release build, one run): 1.4 s a commit, so bar (c)'s two weeks, about 1,300 commits by the spec's baseline, would take about 30 minutes, and `TreeListings` keeps each tip's walk and parents until the scope closes, so memory grows with every commit walked. Bound what a `since` walk keeps between commits, and measure again over a window of at least 300 commits
- [x] Decide whether a commit whose only unexamined subject is one `at` also fails to examine counts apart: all 30 of those commits read `not-examined`, for 023's and 048's records (task 23), so `commits-reporting` was 0

- **Done when**: a `since` walk's peak memory does not grow with the commits walked, its time and memory over at least 300 commits are recorded in the commit, and AC17's fixture still passes.

### 25. A removed spec's named-only commits and the project's scope

Model: opus, with fable as advisor (owner, 2026-11-25).

- [x] `runtime/src/primitives/commit_attribution.rs` — inferred from `Attributor::claims`, not yet run: a subject's spec claims a commit only while the attributor's source holds that spec's directory or the commit touches it, so once a staging spec is folded, or a spec consolidated away, a commit that only names it reads as claimed by no spec at any later tree. `check-unreviewed-code` would then report its code under `unclaimed` when it lies after the project record's window start, beside the upstream spec's entry for a fold below `done`, and for a fold from `done` too. Task 9's fold fixture seeds the record after the staging spec's commits, so it does not show this. Build the fixture with the seed first, and decide from what it shows whether the project's scope leaves such a commit to the spec that took the removed one's place

- **Done when**: a fixture with the seed before a staging spec's named-only commit states what the check reports after the fold, and the check either leaves that code to the upstream spec or the plan says why the project reviews it too.

### 26. The exec walk leaves a `done` spec's ticked criterion judged not met

Model: opus, with fable as advisor (owner, 2026-11-25).

- [x] `runtime/src/interpreter/mod.rs` — `mark_each_verdict` sets every criterion to its `verifyCriteria` verdict, so under `ductus exec` a `done` spec's ticked criterion judged not met is unticked with nobody asked and no status written, against AC3 and AC5 (found by task 10, 2026-11-24). On a `done` spec, read from step 7.1's `read-spec` result in the walk's bindings, it leaves such a criterion as it is, and `write-review` still records it not met, so the record's `blocking: true` is the review-state drift `/ductus:analyze --fix` reverts: the host path's rule when nobody can be asked (the agent's recommendation, adopted by the operator, 2026-11-24)
- [x] `framework/commands/review.md` step 7.3 and `framework/reference/review.md` §Criteria step say what the exec walk does, and `scripts/gen-claude-commands.sh` is re-run

- **Done when**: a unit test proves the exec walk keeps a `done` spec's ticked criterion judged not met ticked and still unticks an `in-progress` spec's, each failing with its behavior removed, the interpreter's mutants pass `scripts/mutation/run.py`, and `runtime/tests/golden/review-basic.jsonl` matches.

### 27. `check-unreviewed-code`'s guidance names the review command unsubstituted

Model: opus, with fable as advisor (owner, 2026-11-25).

- [x] `runtime/src/main.rs` — the CLI's last stderr line prints `` `/{project}:review` `` literally (found by task 11, 2026-11-24), where `pipeline_view` substitutes the project's namespace; batch with task 26's runtime change

- **Done when**: the line names the project's own review command, and a test fails with the substitution removed.

### 28. A scratch-adopter bootstrap for the hook, the seed and the CI template

Model: opus (owner, 2026-11-25).

- [x] `framework/bootstrap/`, `framework/migrations/`, `framework/templates/ci/` — tasks 11 and 12 exercised the manifest rows with `apply-manifest` and the migration's steps by hand (a6216034's message), not a `/ductus --ref=main` run by an agent reading the procedure, which `AGENTS.md` asks for after such a change; deferred to run once with task 13's template change (owner, 2026-11-24). Run it per `docs/developing-ductus.md` §Exercising an installer change without a push, on a subject behind the release: an update that gains `.githooks/ductus-pre-push`, `.githooks/pre-push` and a seeded `specs/review.md` with the migration's backlog report, and a first adoption that gains the seed

- **Done when**: both runs end as the body item says, a second update writes nothing, and the commit records each run's outcome.

### 29. Every closed reopening is judged for owed work

Model: opus, with fable as advisor (owner, 2026-11-25).

- [x] `runtime/src/primitives/measure_reopenings.rs`, `runtime/src/schema/primitives.rs` — task 15 (204126e0) gave `held-owed-work` to the counted `reopenings` alone and left `not-counted` ones outside `never-owed-count`, as `data-model.md` and the plan scoped it, where AC16 reads "for each closed reopening" and `closed-count` holds both lists. So a reopening with no new run (`no-recorded-run`), the likeliest shape of a spurious reopen, reached no defect count. `NotCountedReopening` gains `held-owed-work` for a pass with a start (`no-recorded-run`, `record-unparseable`), absent for `no-in-progress-commit`, which has no pass to read and is not judged, and `never-owed-count` counts every closed reopening whose `held-owed-work` is false (the agent's recommendation, adopted by the operator, 2026-11-25). None was not counted at `47cae941` or at `204126e0`
- [x] `data-model.md` §`measure-reopenings` additions, `plan.md` §Measuring the bar and `MEASURE_REOPENINGS` say so; fixtures for a not-counted reopening each way and a squashed one, with their mutants

- **Done when**: a `no-recorded-run` reopening that never held owed work counts in `never-owed-count` and one that held it does not, a `no-in-progress-commit` entry carries no `held-owed-work`, each fixture fails with its behavior removed, and the mutants are killed.

### 30. The block report's fallback when the namespace cannot be read

- [x] `runtime/src/main.rs`, `runtime/tests/check_unreviewed_code_cli.rs` — `check-unreviewed-code`'s call site prints the `{project}` placeholder when `Host::load` fails, as for a `.ductus/session.toml` that does not parse, and no test covers that arm: M44 mutates only the success arm (found by tasks 26 and 27's landing, 2026-11-25; the agent's recommendation to add a task, adopted by the operator, 2026-11-25). A CLI test with a session file that does not parse, whose block report names the placeholder, and a mutant on the fallback arm in `scripts/mutation/unreviewed-code-084.py`

- **Done when**: a test fails with the fallback arm removed or changed, and its mutant is killed.

### 33. Uncommitted paths another open spec's plan holds leave a spec's review scope

- [ ] `runtime/src/primitives/compute_review_scope.rs` — `split_changes` takes every uncommitted path into the scope whole ("Uncommitted work cannot be attributed, so it joins whole"), so another session's uncommitted work on another spec in the same tree joins every open spec's scope, and `check-review-gate` holds a spec's `done` as review-owed on files it never touched (an adopter's report, 2026-11-25; the plan's Trade-offs recorded it as a limitation). An uncommitted path leaves the scope, listed in `excluded`, only when the Affected Files of another spec at `planned` or `in-progress` holds it, by name or by a pattern, and the spec's own plan does not, so an unplanned edit still joins every open scope (AC23; the agent's recommendation, adopted by the operator, 2026-11-25). A test for each direction, each failing with its behavior removed, and their mutants killed
- [ ] `COMPUTE_REVIEW_SCOPE`, `framework/reference/review.md` §Window and coverage's "Uncommitted work stays" sentence, the plan's Limitation bullet and its `BE-COMPAT-001` paragraph (`plan.md` is 49,980 bytes, 20 under `[artifacts] read-size-bytes`, so the edit trims what it needs to fit), 066's scenario `review-scope-includes-uncommitted-work` and 066's 084 signpost, and a `runtime/CHANGELOG.md` entry

- **Done when**: AC23 holds: a fixture whose uncommitted path another `in-progress` spec's plan holds leaves it out of the scope, and one no other open plan holds keeps it in, each failing with its behavior removed; the mutants are killed; and `check-artifacts` on 084 and 066 reports no new finding.

## Phase D — Delivery

### 17. `check-artifact-size` names no per-fix reopen

Model: opus, with fable as advisor (owner, 2026-11-24).

- [x] `on_done` and `OnDone` go, with `OnDone`'s schema docs and their mutants; a `done` spec's warning states the state rule once; the table's tests rewritten

- **Done when**: AC18 and AC12 hold, no result or description names a per-fix reopen, and `grep -rn classify.spec.edit` over the live-artifact set finds only historical records.

### 18. Back-links and the changelog

Model: opus, with fable as advisor (owner, 2026-11-24).

- [x] Each of 018, 020, 022, 046, 050, 051, 057, 066, 067, 078, 079, 081 and 083 gains the back-link the plan's Cross-spec impact names, 079's and 081's under `## See also`
- [x] 066's scenario `criteria-are-verified-in-review` gains the `done`-spec exception
- [x] 079's `data-model.md` §`measure-reopenings`: its `reopenings` and `not-counted` rows name `held-owed-work`, and its counts row and its sentence on what `dashboard` carries name `never-owed-count`, as tasks 15 and 29 leave them (the agent's recommendation, adopted by the operator, 2026-11-25)
- [x] `runtime/CHANGELOG.md` `[Unreleased]` entries for every change adopters see (AC13)

- **Done when**: `check-artifacts` reports no undischarged `cross-spec-impact:` entry on 084 and no new finding on the thirteen, and the changelog names each primitive added and retired.

### 19. This repository blocks

Model: opus, with fable as advisor (owner, 2026-11-24).

This is the last task. It waits on the `done` specs `check-unreviewed-code` names, reviewed outside this spec's tasks, since no task schedules `/ductus:review` (spec 079). At `68c420a5` it named 9 specs and 81 paths and examined every subject. 025, 062, 012, 000 and 069 were reviewed since, in that order, each record committed and pushed, so at `716f7378` it names 049, 060, 081 and 021, 74 paths, owed in that order. One session reviews them one by one, smallest first, each from a clean worktree of `origin/main` (owner, 2026-11-25). One worktree serves them in turn only because each record is committed with its fixes before the next spec's first `write-review`, as `docs/developing-ductus.md` §Sizing a review's parts requires in a shared tree. The passes are batched: one part over the eight small windows, two over 021's, and one criteria part per spec. They read each spec's own change, the hunks of the commits that gave it its paths: about 365 KB at `68c420a5`, 303 KB of it 021's. Each entry an older record left `unread`, such as `runtime/CHANGELOG.md`, is named `unread` again, which still moves the window (`compute_review_scope::recorded_run`), and that spec's next review reads it (both the agent's recommendations, adopted by the operator, 2026-11-25). Once 084 reaches `done`, tell the owner the date of the commit that set it: the bar's two weeks start there (spec, Resolved Questions, the bar for starting the three items).

- [ ] Remove `[review] unreviewed-push = "warn"` from `.ductus/config.toml` once `check-unreviewed-code` reports nothing here

- **Done when**: the key is gone and a push from this checkout passes the hook at `block`.

### 20. Disposition out-of-spec finding: merge_status_hook's fake `claude` can fail to exec with ETXTBSY

- [x] `runtime/tests/merge_status_hook.rs` — `claude_printing` writes a script and execs it at once; on Linux a parallel test's fork can inherit the still-open write handle, so the exec fails `ExecutableFileBusy` ("Text file busy"). It failed `the_probe_reads_the_claude_on_the_inherited_path` in run 45445822818 (`build + test (ubuntu-latest)`, 210e9a8f, whose own change was comments only); macOS passed the same suite

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

### 21. Disposition out-of-spec finding: two lib tests exec a freshly written script and can hit ETXTBSY

Model: opus, with fable as advisor (owner, 2026-11-25).

- [x] `runtime/src/primitives/merge_status_hook.rs`'s `the_probe_reads_a_version_and_names_each_failure` (three scripts) and `runtime/src/primitives/lint_markdown.rs`'s `a_linter_that_outlives_its_timeout_is_stopped` (one stub) write a script with `fs::write` and exec it at once, inside the lib test binary, the race task 20 fixed in `runtime/tests/merge_status_hook.rs` (68132f95). No failure so far: none of the 40 failed `runtime` runs from 2026-08-03 to 2026-11-23 failed on it (`gh run view --log-failed` grepped for `Text file busy`; every hit was the `pointer-running-cp` diagnostic). Task 20's retried warm-up exec does not fit here, since two of the scripts run `sleep 5`; a candidate, not yet measured, is writing each script from a child process, so no write handle exists in the test process for another thread's fork to inherit — fixed (2026-11-25): each test module gains a `write_script` that writes from a child `/bin/sh` (`printf %s "$2" > "$1"`) and sets the mode in-process, and both tests call it. Measured on Linux (docker `rust:1.99.1`, aarch64, run by the operator): 16 threads each wrote and singly exec'd a fresh script 300 times beside 8 threads spawning `/bin/true`; the old in-process write failed `ExecutableFileBusy` on 429, 461 and 435 of 4,800 execs, the helper's body on 0 each run. A helper in each module owes `merge-status-hook.py`'s 9 mutants, all killed, where one in `runtime/src/primitives/mod.rs` would owe 64; no other lib test execs a script it wrote (a search for `from_mode(0o755)`)

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

### 22. Disposition out-of-spec finding: six mutants in two lists do not die on unchanged `HEAD`

Model: opus, with fable as advisor (owner, 2026-11-25).

- [x] `scripts/mutation/departure-waivers.py`, `scripts/mutation/site-anchors-020.py` — at `a6cba0be`, before task 8's change, `scripts/mutation/run.py` from a worktree of that commit reports `departure-waivers` M5, M8 and M11 and `site-anchors-020` M19 and M39 as SURVIVED, and `departure-waivers` M14 as a compile error (2026-11-24). Neither list names an expected survivor, so five tests pass with the behavior they name removed and one mutant proves nothing. Found by task 8's run of every mutant reaching the files it changed; `run.py --check`, the only run CI makes, validates anchors and runs none. At `abc70306` the six read as at `a6cba0be`. `site-anchors-020`: waived, no defect: the list has named M19 and M39 expected survivors, each with its reason, since `a641a31e` (2026-11-22), and `run.py` exits 0 on them; the finding read its per-mutant SURVIVED lines. `departure-waivers`: fixed (2026-11-25). M5's clause `&& !holds(entries, &entry.rule, file)` in `write_review.rs` was dead, since `holds` reads only a waiver's `file` and every waiver `entries` holds there is a hand waive storing `file: None`; the clause and M5 are removed. M11's check opening `process_waivers.rs`'s `departure_gone` was dead, since only a marked waiver's anchors reach it; the check, the function's then-unused `waiver` parameter and M11 are removed, and M1 is re-anchored on the function's next statement (both the agent's recommendation, adopted by the operator, 2026-11-25). M8 is killed by a new test, `a_second_entry_for_a_rule_waives_nothing_the_first_leaves_counted`; M14's replacement now covers the field's second use, and `waiver-notices-flatten.py`'s M5 is re-anchored on the changed call. Of the 240 mutants in 17 lists anchored in the two changed files, `departure-waivers.py` whole among them, 238 were killed and 2 survived, `site-anchors-020`'s expected M19 and M39

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

### 23. Disposition out-of-spec finding: 023's and 048's review records are not valid YAML

Model: opus, with fable as advisor (owner, 2026-11-25).

- [x] `specs/023-govern-refinement/review.md`, `specs/048-govern-acquired-runtime/review.md` — the waiver `reason:` that `ec41a6c2` rewrote by hand in each record's `decisions:` list is an unquoted scalar holding a colon and a space ("… 2026-11-21: that prompt …"), so the frontmatter fails to parse (`serde_norway`: "mapping values are not allowed in this context", at the 140th line of 023's block and the 467th of 048's). Both specs are `done`, so `check-review-gate` answers already-done before it reads the record, and `check-artifacts --feature 023-govern-refinement` lists no finding on it. Found by task 9's first run of `check-unreviewed-code` here, which reports both specs as not examined (2026-11-24) — fixed (2026-11-25): each record's `reason:` is double-quoted, its text byte-identical, a chore. In a `git clone --local` with the fix committed, `check-unreviewed-code` examines 74 subjects where it examined 72 at `89d5d8d7`, reads `found` where it read `not-examined`, and names neither spec, still 9 `done` specs and 81 paths; `check-artifacts` on each reports at the fix what it reported before it, 023's two advisory `plan.md` path findings and none on 048

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task.

### 31. 084's plan classifies its contract changes under `BE-COMPAT-001`

Model: opus, with fable as advisor (owner, 2026-11-25).

- [x] `specs/084-reopen-for-owed-code-review-every-code-change/plan.md` — the plan classifies none of the primitive contracts 084 changes, which `BE-COMPAT-001`'s Verification asks of a plan that modifies one, and which `AGENTS.md` says fires on a primitive's contract (found by task 17, 2026-11-25; the agent's recommendation to add a task, adopted by the operator, 2026-11-25). Among them: `classify-spec-edit` retired, `check-artifact-size`'s `fixes[].on-done` removed, `compute-review-scope`'s window start and its scope for code no spec claims, `/ductus:analyze --fix` no longer reverting a `done` spec for a scenario's open question, and the primitives and result fields added. Derive the whole set from 084's commits, read each against the primitive's compiled description, and give the plan one paragraph classifying each as additive or as breaking within major version zero, as 057's, 058's, 063's and 066's plans do. `plan.md` is 521 bytes under `[artifacts] read-size-bytes` (50,000; `wc -c`, 49,479 bytes once task 32 added its Cross-spec impact line), so the paragraph trims what it needs to fit, or the commit says why it does not

- **Done when**: the plan states each primitive contract 084 changes as additive or breaking under `BE-COMPAT-001`, and `check-artifact-size` on 084 reports `plan.md`'s size.

### 32. `check-artifacts`' task numbering blocks a repeated number, not an order

Model: opus, with fable as advisor (owner, 2026-11-25).

- [x] `runtime/src/primitives/check_artifacts.rs`, `runtime/src/schema/descriptions.rs`, `runtime/src/primitives/mod.rs`, `framework/reference/analyze.md`, `scripts/mutation/suffixed-task-number.py`, `specs/041-task-pruning/scenarios/a-suffixed-task-number-is-a-task.md` — `check_task_consistency` blocks a `tasks.md` whose task numbers do not rise down the file, which no reader needs: `/ductus:implement` walks file order, and `mark-task`, `append-task` and `prune-tasks` need a number only to be unique. `append-task` itself produces the finding whenever it adds a task to a phase before the last, as it did with 084's tasks 24 to 30, so `check-artifacts` on 084 reports "task numbering is not strictly increasing: task 17 follows task 30" (found by task 18's baseline run, 2026-11-25). The check blocks a number that repeats, which `mark-task`'s first-match lookup could never tick past, and no longer an order (AC22; the agent's recommendation, adopted by the operator, 2026-11-25). Its two numbering tests are rewritten to the rule, each failing with it removed; `CHECK_ARTIFACTS`, `TaskNumber`'s doc comment, analyze's §Task consistency bullet and 041's scenario edge case state it; the mutants anchored on the numbering check are re-anchored and killed; a changelog entry; and 067's signpost names the change, and 041's `spec.md` gains a blockquoted `> **Signpost:**` linking `../084-reopen-for-owed-code-review-every-code-change/spec.md`, which discharges 041's `cross-spec-impact:` entry and adds no `dependencies:` edge, since the gate reads a link on a pointer line and never a prose mention

- **Done when**: AC22 holds, `check-artifacts` on 084 reports no `task-consistency` finding with no task block moved, the mutants are killed, and in a `git clone --local` with 084 set to `done` `check-artifacts` on 084 reports no undischarged `cross-spec-impact:` entry, 041 included (the method `4f706fde`'s message records).
