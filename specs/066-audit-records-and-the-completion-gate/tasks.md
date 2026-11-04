# 066 — Audit Records and the Completion Gate Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Move the scenarios

- [x] `git mv` the sixteen scenarios task 142 of `022-deterministic-runtime` places here, remap each `section:` to a heading of this spec, and rewrite each link into 022 as a `../../022-deterministic-runtime/` path; a link into `067-analyze-and-audit-checks` becomes a backticked slug

- **Done when**: the sixteen files sit under `scenarios/` with their history intact (`a-done-spec-has-no-transition-to-gate`, `a-multi-number-renumber-is-one-rewrite`, `a-renamed-file-contributes-its-rewrites`, `a-review-states-what-it-read`, `an-empty-scope-flag-does-not-decide-the-denominator`, `process-waivers-file-lists`, `review-base-includes-the-transition-commit`, `review-scope-includes-uncommitted-work`, `review-scope-plan-affected-table-format`, `review-scope-union`, `review-staleness-gate`, `review-staleness-on-done-specs`, `the-committed-tree-horizon`, `the-cross-spec-impact-gate`, `write-analysis-and-the-second-gate`, `write-review-known-field-quoting`), each `section:` names a heading of `spec.md`, and `check-corpus-links` reports no broken link.

## 2. Move the data-model sections

- [x] Move the `write-analysis`, `check-review-gate`, `diff-cross-spec`, `process-decisions` and both `write-review` sections out of 022's data model, keeping every shape and field meaning and pointing at the scenario where the rationale lives
- [x] Add the `compute-review-scope` and `process-waivers` shapes from `runtime/src/schema/primitives.rs`

- **Done when**: `data-model.md` holds the seven primitives' shapes, 022's data model holds none of them, and the file is at or under 50,000 bytes.

## 3. Verify the criteria against the tree

- [x] Verify AC1–AC5 against `runtime/src/primitives/` and tick each that holds

- **Done when**: every acceptance criterion is ticked, each verified against the code that implements it.

## 4. State the reasons write-analysis adds to the breakdown itself

- [x] Add to `data-model.md`'s **Derived, never accepted** list the two reasons `write_analysis.rs` adds to `unexamined-by-reason` itself: `constitution-unresolved` (one per registered shared constitution that did not resolve) and `constitution-registry-unreadable` (a `[constitutions]` registry that will not parse), both unexamined inputs rather than skipped targets

- **Done when**: the data model names every reason the writer can record, and the closed reason table stays the set a caller supplies

## 5. Implement scenario: a-done-specs-plan-edit-leaves-its-analysis-current

- [x] Implement the behavior described in `scenarios/a-done-specs-plan-edit-leaves-its-analysis-current.md`
- [x] `analyze_subjects::analyze_freshness` drops an in-place `plan.md` or `tasks.md` edit from a `done` spec's changed paths (`without_plan_edits`), keeping a deleted one; `a_plan_or_tasks_edit_does_not_stale_a_done_specs_record`, `a_plan_edit_stales_an_open_spec_and_a_spec_edit_stales_a_done_one` and `a_done_specs_deleted_plan_still_stales_its_record` hold each direction, and mutations M2 and M3 in `scripts/mutation/plan-edit-takes-no-back-edge.py` kill them (for `041-task-pruning`'s `a-plan-edit-does-not-reopen-a-done-spec`)
- [x] §Record freshness and `data-model.md` §Record freshness state the `done` exclusion

- **Done when**: The scenario's behavior holds in `runtime/src/primitives/analyze_subjects.rs`, its tests fail with the behavior removed, and 066's spec and data model describe the comparison as it runs.

## 6. Implement scenario: only-gating-findings-hold-done

- [x] Implement the behavior described in `scenarios/only-gating-findings-hold-done.md`
- [x] `check_review_gate::disposition_block` blocks on the review record's unwaived MUST and SHOULD counts and the analysis's hard-fail and blocking counts, never on `dispositions.undispositioned`; `undispositioned_advisory_findings_do_not_hold_done`, `an_outstanding_should_holds_done`, `a_gating_count_beside_a_false_blocking_flag_still_holds_done` and `a_predating_record_outranks_an_outstanding_should` hold each direction, and mutations M1, M2, M4 and M5 in `scripts/mutation/findings-cost-in-proportion-to-risk.py` kill them (for `058-findings-route-at-discovery`'s `findings-cost-in-proportion-to-risk`)
- [x] `data-model.md`'s gate row 11 and §The completion gate describe the check as it runs

- **Done when**: The scenario's behavior holds in `runtime/src/primitives/check_review_gate.rs`, its tests fail with the behavior removed, and 066's spec and data model describe the check as it runs.

## 7. Scope results have a bounded size (BE-PAYLOAD-001)

Mutation list: `scripts/mutation/scope-results-bounded.py`; the walker and MCP tests it cannot run fail under its M1, run by hand as its docstring says.

- [x] Implement the behavior described in `scenarios/scope-results-have-a-bounded-size.md`
- [x] Resolve the scenario's open question through `/ductus:clarify`, then bound `compute-review-scope` and `diff-cross-spec` by the chosen mechanism, state the maximum in `data-model.md`, and update `framework/commands/review.md` step 1 and every other consumer
- [x] A test per primitive shows a result past the bound reaching every entry on the MCP and the exec paths, and fails when the bound is removed

- **Done when**: both results carry a stated maximum, every entry stays reachable on both paths, and the 2026-10-27 analysis's BE-PAYLOAD-001 finding no longer fires

## 8. Disposition: the Family 19 parity tests run git in their fixtures under the hook's GIT_DIR

- [x] Found 2026-10-28 by 051's round-4 commit, made from a linked worktree: git exports GIT_DIR to a hook, absolute in a linked worktree, and runtime/tests/mechanical_sweep_parity.rs spawns git and scripts/audit/review-freshness.sh in fixture repositories without clearing it, so they act on the outer repository. Reproduced: with GIT_DIR set to the worktree's git directory, cargo test --test mechanical_sweep_parity fails rust_and_family_19_agree_on_a_built_sweep, rust_and_family_19_agree_on_a_renumbered_directory, rust_and_family_19_refuse_an_over_limit_window_alike and family_19s_proxy_arm_lists_both_sides_of_a_move_under_any_config; unset, 7 of 7 pass. The main checkout's hook passed them on 38fc39f6, so only a commit from a linked worktree meets it — fixed
- [x] Clear GIT_DIR, GIT_WORK_TREE and GIT_INDEX_FILE on every Command that test file spawns — its git commands through `git_command` in `runtime/tests/common/mod.rs`, added by 051's 2026-10-29 review for `consolidate_hook.rs`, and the same three on the `review-freshness.sh` command — with a test that sets GIT_DIR to another repository

- **Done when**: the four tests pass when GIT_DIR names another repository, as they do with it unset, and each fails when its clearing is removed

## 9. Implement scenario: checkbox-state-is-outside-the-analysis-digest

- [x] Implement the behavior described in `scenarios/checkbox-state-is-outside-the-analysis-digest.md`

- **Done when**: The scenario's open question is resolved, a criterion tick after the analyze run no longer surprises the gate by the chosen design, and a fixture test proves the gate's answer in both directions (a tick, and any other spec.md edit).

## 10. Implement scenario: review-scope-reads-every-cell-path-and-sets-deleted-files-apart

- [x] Implement the behavior described in `scenarios/review-scope-reads-every-cell-path-and-sets-deleted-files-apart.md`

- **Done when**: The scenario's open question is resolved, review-scope-includes-uncommitted-work is amended to match, and fixture tests cover a two-path Affected Files cell and a file deleted since the base.

## 11. Decide whether Family 19 delegates to the runtime, now that its stated reason has lapsed

- [x] runtime/tests/mechanical_sweep_parity.rs and scenarios/review-staleness-on-done-specs.md — both say Family 19 keeps its own implementation because the self-audit's CI job has no Rust toolchain and no runtime build, so calling the runtime would make the gate depend on the artifact it gates; `runtime-release.yml`'s `audit` job now installs the pinned toolchain and `scripts/audit/check-zero.sh` runs `cargo build --release` there, so the reason no longer holds (048's review, 2026-10-29). Decide under §runtime-boundary's eligibility default whether Family 19 calls the runtime, and rewrite both statements to match the answer

- **Done when**: Family 19's implementation follows a decision recorded in 066's plan, and no artifact gives a reason for it that the audit job contradicts

## 12. The Family 19 parity test fails, rather than passing unrun, outside a git checkout

- [x] `rust_and_family_19_agree_on_every_done_spec` in `runtime/tests/mechanical_sweep_parity.rs` prints "skipping: not a git checkout" and returns when the tree has no `.git`, so `cargo test` reports it `ok` unrun: 048's second re-read round on 2026-10-29 saw exactly that in a `git archive` export of HEAD (§design-principles: a check that cannot run never looks like one that passed; prefer a test that fails loudly on a missing input). Make it panic naming why, as `rewrite_spec_links.rs`'s permission tests do since 048's review, and check whether any other test in that file skips the same way. The pattern predates 048's window (`db393e03`); CI has `.git`, so CI is unaffected

- **Done when**: Run outside a git checkout, the parity test fails with a message naming why; `cargo test --locked` passes in this repository

## 13. Implement scenario: process-waivers-notices-flatten-stored-text

- [x] Implement the behavior described in `scenarios/process-waivers-notices-flatten-stored-text.md`
- [x] Print each `process-waivers` notice's `rule`, `file` and `reason` through `flatten_line`, classifying on the stored values as before
- [x] Add a test that plants an escape in each field of a duplicate, a retained and an expired waiver

- **Done when**: the test fails when any notice prints a stored field as written, and `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` pass under `runtime/`.

## 14. Implement scenario: process-decisions-results-have-a-bounded-size

`bounded_lists::place` (`runtime/src/primitives/bounded_lists.rs`, task 7) is the shared helper: a primitive's `run` builds the inline result from a crate-private `compute` holding the whole lists, and `place` decides the form, as `compute_review_scope.rs` does.

- [x] Implement the behavior described in `scenarios/process-decisions-results-have-a-bounded-size.md`
- [x] Bound `process-decisions`' `matched`, `expired`, `retained`, `lapsed` and `notices` lists by the mechanism task 7 settles, and state the maximum in `data-model.md`
- [x] A test shows a result past the bound reaching every entry, and fails when the bound is removed

- **Done when**: every list in the result carries a stated maximum, every entry stays reachable, and 076's 2026-11-03 analysis's BE-PAYLOAD-001 finding no longer fires.

## 15. Implement scenario: the-fix-round-loop-is-recorded-and-gated

`write-review` reads `[review] re-reads-off` through `fix_rounds::re_reads_off` (`runtime/src/primitives/fix_rounds.rs`, from 078 task 2), which returns the reason on one line or halts with `PrimitiveError::InvalidReReadsOff`.

- [x] Implement the behavior described in `scenarios/the-fix-round-loop-is-recorded-and-gated.md`
- [x] Add the `fix-rounds` argument to `WriteReviewArgs` and derive the six record fields and the Summary line in `runtime/src/primitives/write_review.rs`, refusing the malformed entries the scenario lists
- [x] Add check 12, `fix-round-not-re-read`, and the passing guidance for a record without the loop fields to `runtime/src/primitives/check_review_gate.rs`
- [x] Name the fields in `framework/constitution.md`'s audit-record table, cutting as many bytes as the row adds, and their meaning in `framework/reference/review.md`; add the check to `data-model.md`'s `check-review-gate` list
- [x] Tests: each derived count, the absent-versus-empty list, the switch's waiver, each refusal, and the gate blocking and passing; each fails with its behavior removed

- **Done when**: `write-review` records the loop's fields from `fix-rounds` alone, `check-review-gate` blocks an un-re-read round without a waiver, and the constitution passes `scripts/lint-governing-size.sh`.

## 16. Disposition out-of-spec finding: the seeds' compute-review-scope CLI allows outlive the step that ran them

- [ ] framework/bootstrap/ductus.md — the Claude, Auggie and OpenCode seeds in §Agent Registry's `settings_template` column (and `framework/bootstrap/govern.md`'s copy) allow `.ductus/bin/ductus compute-review-scope *` and its `.exe` twin, and §Permission Setup says the seed carries that rule because `/{project}:review` re-runs the call through the binary. Task 7 removes that re-run from `framework/commands/review.md` step 1, so no step runs the command and the entries grant what nothing needs; a project that already holds them keeps them, since the seed merge is additive, unless `/{project}:configure`'s retired lists remove them. Found 2026-11-03 by task 7's consumer sweep

- **Done when**: the finding is fixed, deferred, or waived, with a waiver's reason written on the task

## 17. Implement scenario: a-supplied-summary-carries-the-loops-account

- [ ] Implement the behavior described in `scenarios/a-supplied-summary-carries-the-loops-account.md`
- [ ] Change `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`'s "The generated Summary renders" line to say the Summary, supplied or generated

- **Done when**: a supplied Summary carries the loop's account whenever the run passes fix-rounds, a run without them renders it as given, and a test fails when the append is removed

## 18. Fix two notes 078's review re-read found in 066 task 9's change

- [ ] Restore the first line of `the_review_record_remains_a_subject`'s doc comment in `runtime/src/primitives/analyze_subjects.rs` ("The exclusion is the record's alone — the *review* record stays a"), which now opens mid-sentence
- [ ] Split `every_other_spec_edit_still_moves_its_digest`'s two ticks into separate edits, so normalizing a fenced checkbox inside the criteria alone fails it

- **Done when**: the doc comment reads whole, and each tick in the test moves the digest on its own, shown by a mutation that normalizes only the fenced checkbox

## 19. Fix what 078's review re-read found in 066 task 10's change

Found by 078's review round 3 (2026-11-04); the round's result file is gitignored and the next 078 review clears it, so each item states its edit. Re-check each against the tree before applying it.

- [ ] [0] 066 `data-model.md`'s inline `compute-review-scope` example lists two `modified-since` paths: set its `"modified-since-count": 1,` to `2`
- [ ] [1] `a_result_past_the_read_size_names_a_file_holding_every_list` (`runtime/src/primitives/compute_review_scope.rs`) asserts `deleted` on a fixture that deletes nothing: build it with `repo_in_progress_with_an_untouched_file()` and `fs::remove_file` its `src/old.rs` before computing, assert `whole.deleted == ["src/old.rs"]`, the inline result's `deleted` equal to it and its `deleted_count` 1; add to `scripts/mutation/review-scope-cells-and-deletions.py`, each filtered to that test, M7 (`deleted_count: scope.deleted.len(),` → `deleted_count: 0,`), M8 (the list file's `"deleted": scope.deleted,` → `"deleted": Vec::<String>::new(),`) and M9 (`deleted: Some(scope.deleted.clone()),` → `deleted: Some(Vec::new()),`)
- [ ] [2] `compute_review_scope.rs`'s module doc defines `scope` as the plain union: its bullet's `commits. Both, because either alone can omit what the review exists to look at.` becomes `commits, less the files deleted since it, which no review can read. Both, because either alone can omit what the review exists to look at.`
- [ ] [3] `scenarios/review-scope-union.md`'s edge case "An empty `plan-affected` leaves `scope` equal to `modified-since`" gains "less the paths the window deleted"
- [ ] [4] `framework/commands/review.md`'s Inputs **Scope** bullet: "committed or not — see step 1" becomes "committed or not, less the files since deleted — see step 1"
- [ ] Disposition note [5]: 066 `spec.md` line 27 and `framework/reference/implement.md` line 75 still describe the scope as a plain union
- [ ] Disposition note [6]: a window holding only deletions under an empty plan resolves `scope-count` 0, so step 1 sends it to the nothing-to-review-yet report though `deleted-count` is above zero

- **Done when**: each finding's edit is applied or the finding otherwise fixed, and each note fixed, deferred or waived with its reason on the task
