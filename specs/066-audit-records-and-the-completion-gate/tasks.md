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

- [ ] Implement the behavior described in `scenarios/scope-results-have-a-bounded-size.md`
- [ ] Resolve the scenario's open question through `/ductus:clarify`, then bound `compute-review-scope` and `diff-cross-spec` by the chosen mechanism, state the maximum in `data-model.md`, and update `framework/commands/review.md` step 1 and every other consumer
- [ ] A test per primitive shows a result past the bound reaching every entry on the MCP and the exec paths, and fails when the bound is removed

- **Done when**: both results carry a stated maximum, every entry stays reachable on both paths, and the 2026-10-27 analysis's BE-PAYLOAD-001 finding no longer fires

## 8. Disposition: the Family 19 parity tests run git in their fixtures under the hook's GIT_DIR

- [ ] Found 2026-10-28 by 051's round-4 commit, made from a linked worktree: git exports GIT_DIR to a hook, absolute in a linked worktree, and runtime/tests/mechanical_sweep_parity.rs spawns git and scripts/audit/review-freshness.sh in fixture repositories without clearing it, so they act on the outer repository. Reproduced: with GIT_DIR set to the worktree's git directory, cargo test --test mechanical_sweep_parity fails rust_and_family_19_agree_on_a_built_sweep, rust_and_family_19_agree_on_a_renumbered_directory, rust_and_family_19_refuse_an_over_limit_window_alike and family_19s_proxy_arm_lists_both_sides_of_a_move_under_any_config; unset, 7 of 7 pass. The main checkout's hook passed them on 38fc39f6, so only a commit from a linked worktree meets it
- [ ] Clear GIT_DIR, GIT_WORK_TREE and GIT_INDEX_FILE on every Command that test file spawns, with a test that sets GIT_DIR to another repository

- **Done when**: the four tests pass when GIT_DIR names another repository, as they do with it unset, and each fails when its clearing is removed

## 9. Implement scenario: checkbox-state-is-outside-the-analysis-digest

- [ ] Implement the behavior described in `scenarios/checkbox-state-is-outside-the-analysis-digest.md`

- **Done when**: The scenario's open question is resolved, a criterion tick after the analyze run no longer surprises the gate by the chosen design, and a fixture test proves the gate's answer in both directions (a tick, and any other spec.md edit).

## 10. Implement scenario: review-scope-reads-every-cell-path-and-sets-deleted-files-apart

- [ ] Implement the behavior described in `scenarios/review-scope-reads-every-cell-path-and-sets-deleted-files-apart.md`

- **Done when**: The scenario's open question is resolved, review-scope-includes-uncommitted-work is amended to match, and fixture tests cover a two-path Affected Files cell and a file deleted since the base.
