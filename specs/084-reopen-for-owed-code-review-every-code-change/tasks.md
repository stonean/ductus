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

- [ ] `compute_review_scope::resolve_base` and `last_review_commit` stop hiding the reopen; `held_base`'s clamp and the carried-in branch go, and `UncommittedTransition` stays only for a spec with no counted review
- [ ] A `check_stuck` lookup for the commit that set a spec `done` or added it as `done`, for a `done` spec no review recorded
- [ ] Invert the tests asserting the old start; update the module doc, `COMPUTE_REVIEW_SCOPE`, `BaseSource`'s docs, `framework/reference/review.md` §Window and coverage, `framework/commands/review.md`'s `--since` row, `runtime/tests/golden/review-basic.jsonl` and `scripts/mutation/scope-attribution.py`

- **Done when**: AC7's two fixtures pass and fail with the change reverted.

### 7. Code no spec claims leaves open specs' scopes

Model: fable (owner, 2026-11-24).

- [ ] `attribute_window`'s no-claimant branch records its paths in `window.others`, so they leave the scope; `take_unclaimed`, `ReadRecord`, `read_records` and `commits_between` go, with `scripts/mutation/review-read-credit.py`, `reviews-follow-what-changed.py` and `commit-counts.py`
- [ ] Move the per-path claim rule into `commit_attribution`, taking its source (the working tree or a commit's tree) and its walk's start, for `fix_rounds::changed_only_by_other_specs` and task 9 to share
- [ ] Rewrite 081's AC13 and AC14 fixtures to the new rule

- **Done when**: a fixture with a code commit naming no spec leaves it out of an `in-progress` spec's scope, `commits-unclaimed` still counts it, and AC6's second half holds.

### 8. The project record

Model: fable (owner, 2026-11-24).

- [ ] `write-review` with no `feature` writes `{specs-root}/review.md` as `data-model.md` states, and `--seed` writes the seed
- [ ] The function that finds the code no spec claims after the project record's window start, a seed's starting after the commit that added it; the project review's scope reads it, and task 9 exposes it
- [ ] `fix_rounds::RoundSubject` takes an optional feature, `round_paths` leaves out `{specs-root}/review.md`, and its callers, `read_re_read::run` among them, accept a project subject (AC14)
- [ ] `framework/commands/review.md` gains `--project`, and `--all` includes it
- [ ] Seed this repository's `specs/review.md`

- **Done when**: AC6 and AC14 hold, a project review over a fixture's unclaimed commit records it, and its fix rounds leave its own record out.

### 9. `check-unreviewed-code`

Model: fable (owner, 2026-11-24).

Blocked on tasks 6 to 8: it reads their window, their claim rule and the project record.

- [ ] The primitive at `at`, reading spec directories, statuses, plans and review records from that commit's tree and never the disk, with `done-specs`, `unclaimed`, `not-examined` and `mode`, and its seven registration sites
- [ ] `[review] unreviewed-push`'s reader, its `PrimitiveError` variant, `runtime/src/schema/config_keys.rs` and `framework/bootstrap/ductus.md` §Project Configuration
- [ ] The `since` count over first-parent commits (AC17)
- [ ] Fixtures for AC4's directions, among them a shallow history, a tree holding no project record, a staging spec folded below `done`, an `at` that is not `HEAD`, and a seed whose adding commit holds code

- **Done when**: AC4 and AC17 hold, an invalid key value fails naming the key and the file, and each fixture fails with its behavior removed.

### 10. `/ductus:review` on a `done` spec

Model: opus, with fable as advisor (owner, 2026-11-24).

- [ ] Step 7.3 asks the owner before a `done` spec's ticked criterion judged not met changes, corrects or unticks it, and writes `in-progress` on an untick
- [ ] Step 10 writes `in-progress` on a deferral to the spec's own work, and records no gating finding without a disposition
- [ ] Scope Boundaries, `framework/reference/review.md` §Criteria step and §Deferrals, and goldens re-blessed and read

- **Done when**: AC3 and AC5 hold in the command and reference text, and the goldens match.

### 11. The pre-push hook

Model: opus, with fable as advisor (owner, 2026-11-24).

- [ ] `framework/bootstrap/hooks/ductus-pre-push` and `framework/bootstrap/hooks/pre-push`, shaped as the pre-commit pair
- [ ] `framework/bootstrap/ductus.md` §Shared Files rows and the `claude` registry row's `chmod` seed, mirrored in `framework/bootstrap/govern.md` and `install.sh`; `framework/bootstrap/ductus-procedure.md` §Hook Installation
- [ ] `.github/workflows/runtime.yml`'s path triggers, and a `scripts/audit/adopter-shell-behavior.sh` fixture for each AC8 case
- [ ] A scratch-adopter bootstrap, per `docs/developing-ductus.md`'s installer section

- **Done when**: AC8 holds on an adopter-shaped tree, each case proven, a shallow clone and a missing runtime among them.

### 12. The seed migration

Model: opus, with fable as advisor (owner, 2026-11-24).

- [ ] `framework/migrations/project-review-record.md` and its `framework/migrations.toml` entry, ordered after every existing one
- [ ] `framework/bootstrap/ductus-procedure.md` seeds the record on a fresh install
- [ ] After the seed, the migration runs `check-unreviewed-code` once and reports the backlog with `[review] unreviewed-push` and its two values, writing no configuration

- **Done when**: a scratch adopter behind the release gains a seeded `{specs-root}/review.md` on update and on a fresh bootstrap, a second run writes nothing, and one with a `done` spec's code changed since its review is shown that spec and the key with its configuration unchanged, and the check at the commit carrying the update reports none of that commit's files.

### 13. The CI template and the audit backstop

Model: opus, with fable as advisor (owner, 2026-11-24).

Both call `check-owed-work --all`, which names each spec it cannot judge in `skipped`. Family 27 fails today on a spec it cannot read or that resolves outside the repository, and counts an untracked one, so the step and the script fail on every skip reason but `untracked` (`QUAL-CLAIM-001`).

- [ ] `framework/templates/ci/adopter-generators.yml`: `fetch-depth: 0` with its comment, the unreviewed-code step, and a `done`-spec owed-work step calling `check-owed-work`'s corpus form once
- [ ] `scripts/audit/done-spec-criteria.sh` calls the same corpus form in place of its python parse; `scripts/tests/test-done-spec-owed-work.sh`, run from `framework-checks.yml`

- **Done when**: AC9's CI half and AC15 hold, each step run by `scripts/run-workflow-steps.py` on a fixture fails with its condition present and passes without it.

### 14. The status count

Model: opus, with fable as advisor (owner, 2026-11-24).

- [ ] `dashboard`'s `unreviewed-code` block, `pipeline_view`'s line, `framework/commands/status.md` and `docs/slash-commands.md`

- **Done when**: `/ductus:status` shows the count in each of its states on every run.

### 15. Measuring the bar

Model: opus, with fable as advisor (owner, 2026-11-24).

- [ ] `measure_reopenings`' `held-owed-work` and `never-owed-count`, with a fixture for each direction

- **Done when**: AC16 holds, and `measure-reopenings --since 2026-11-11 --at 47cae941` reproduces the estimate's 74 never-owed reopenings or the commit names why it differs.

### 16. This repository's hook

Model: opus, with fable as advisor (owner, 2026-11-24).

- [ ] `.githooks/pre-push`, and `[review] unreviewed-push = "warn"` in `.ductus/config.toml`

- **Done when**: a push from this checkout runs the check and warns on the backlog, and the commit records the `done` specs and commit count it reports.

## Phase D — Delivery

### 17. `check-artifact-size` names no per-fix reopen

Model: opus, with fable as advisor (owner, 2026-11-24).

- [ ] `on_done` and `OnDone` go, with `OnDone`'s schema docs and their mutants; a `done` spec's warning states the state rule once; the table's tests rewritten

- **Done when**: AC18 and AC12 hold, no result or description names a per-fix reopen, and `grep -rn classify.spec.edit` over the live-artifact set finds only historical records.

### 18. Back-links and the changelog

Model: opus, with fable as advisor (owner, 2026-11-24).

- [ ] Each of 018, 020, 022, 046, 050, 051, 057, 066, 067, 079, 081 and 083 gains the back-link the plan's Cross-spec impact names, 079's and 081's under `## See also`
- [ ] 066's scenario `criteria-are-verified-in-review` gains the `done`-spec exception
- [ ] `runtime/CHANGELOG.md` `[Unreleased]` entries for every change adopters see (AC13)

- **Done when**: `check-artifacts` reports no undischarged `cross-spec-impact:` entry on 084 and no new finding on the twelve, and the changelog names each primitive added and retired.

### 19. This repository blocks

Model: opus, with fable as advisor (owner, 2026-11-24).

This is the last task. The about 9 `done` specs measured as owing a review are reviewed outside this spec's tasks, since no task schedules `/ductus:review` (spec 079); this task waits on them. Once 084 reaches `done`, tell the owner the date of the commit that set it: the bar's two weeks start there (spec, Resolved Questions, the bar for starting the three items).

- [ ] Remove `[review] unreviewed-push = "warn"` from `.ductus/config.toml` once `check-unreviewed-code` reports nothing here

- **Done when**: the key is gone and a push from this checkout passes the hook at `block`.
