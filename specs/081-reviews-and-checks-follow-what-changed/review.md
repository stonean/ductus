---
spec: 081-reviews-and-checks-follow-what-changed
last-run: 2026-11-25T19:58:46Z
reviewed-against: 861c18ae239001c17472c284220cd2ba05b62922
diff-base: 11a20ebdd66a15992ce76740377c292929bdfffe
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 7
unread:
  - .ductus/reference/review.md
  - .ductus/docs/agents/developing-ductus.md
  - framework/reference/review.md
  - runtime/CHANGELOG.md
  - specs/081-reviews-and-checks-follow-what-changed/analysis.md
  - specs/081-reviews-and-checks-follow-what-changed/plan.md
excluded: 554
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
criteria-judged: 14
criteria-met: 14
criteria-verdicts:
  - label: AC1
    digest: ada20579cefaf6c9ed310d24780f5d6ed9ce1043886b84e58da1b082ef61ba41
  - label: AC2
    digest: 84506f0447315a4fba0fef5653724ab1e2fbef2440903c438d06dd8a9058923b
  - label: AC3
    digest: 58d582a146e0c170cf16c877bdea2621b8f2eaf21c3dfba194f6b947750ffaf3
  - label: AC4
    digest: 4d5c321d70a4f8f4fcb5aabb1d1ad949dc2c66f280b0815282319e1dc1f2f3b2
  - label: AC5
    digest: b5dde1cc393751c8960b0c7cc89d6043749710f5cc333d84347e37bbf9b1cf50
  - label: AC6
    digest: 0d5be3eb979e5fa9199394466a0dea3794c313d55585d448e610e2d267e14bfe
  - label: AC7
    digest: 59c12298f241c2e322bed0c8c509f84989dc1cb5a9b44a18caa843d577c5ab9f
  - label: AC8
    digest: a03015e07628887df831e86363e45ede4309cca470cf410fd2663a12550f732e
  - label: AC9
    digest: ddef81d8aeee4267824839a53a0b8a197a69c0a8ed64eb41cb7fb45e12622fac
  - label: AC10
    digest: 48e49705b5dc3c14b819cf9fdc23396f1418923569468b60a46b58b17106365e
  - label: AC11
    digest: 99f30dcaebee3b9d8022d5befb784d581b358d10e19c608e089a9d19ca468382
  - label: AC12
    digest: 8f7fb7dbd7b9528ece61066f9efa371e1c90ed5ee5bb1b35a35eaf720b86021c
  - label: AC13
    digest: 35df1e25afe51f2c66216656d2f3dab323b7a25270dfb71bd2676b9ff3e2b23e
  - label: AC14
    digest: dd72146b3d50c131d74e457aee74c21d167c1b561b19f9fa3dc7aee08f9fde9e
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 0
  undispositioned: 0
decisions:
  - key: "other — on a run with an empty `diff-base` (a spec never `in-progress`), `write-review` refuses every non-empty `unread` as outside the scope, though `compute-review-scope` returns the plan entries as that scope (`scope: [src/a.rs]`, fixture repo3): the message tells the host to list only entries compute returned, and the entry is one. `resolve_scope` returns `BTreeSet::new()` for the empty base before any scope is derived, while its own doc says that scope still holds a first review's plan entries. AC12 refuses \"an entry the run's scope does not hold\"; this one is held. — `runtime/src/primitives/write_review.rs:828-831`"
    outcome: waived
    reason: "No contract break: an empty diff-base records scope 0 (spec 079; resolve_scope's doc says so), so the run's scope holds no entry and AC12's refusal applies as written; such a record credits nothing, since the credit needs its diff-base to be a commit; and framework/reference/review.md §Window and coverage tells the host to pass the current commit as the base. Only the refusal's wording, which points at compute-review-scope's list, misleads here."
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: "other — a credit survives the revert of the record that granted it: `read_records` reads each record from the commit that wrote it, and a later commit deleting that review.md does not withdraw the credit, although `last_review_commit` treats a record reverted away as no earlier review ('what the reverted run read has no standing'). Driven: fixtures-A/f3 (record committed, then `git rm` of it) still answers `scope: []`, `review-owed: false` for the chore path. AC13 asks only for a record 'committed in the window', which the reverted record was, so the contract holds; the inconsistency is with 079's revert rule. — `runtime/src/primitives/compute_review_scope.rs:827-874`"
    outcome: waived
    reason: "Observation about the pipeline's own machinery in framework work, and AC13's own text ('a review record committed in the window') is met: the run did read the path. Closing it needs a decision (should a revert withdraw a read?) that a spec owner makes, not a mechanical fix; deferring it would not be fed back by any gate."
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: "other — a credited path that the window deleted is not listed in `excluded`: `split_changes` skips deleted paths, so the credit shows only as `review-owed: false` and the path stays in `deleted`. AC13 says a credited path 'is listed in `excluded` under the spec whose review read it'. Driven: fixtures-A/f8 (chore deletes src/old.rs, record `unread: []`) answers `excluded: [specs/002-y/review.md]`, `deleted: [src/old.rs]`, `review-owed: false`; f9 (record names it unread) answers `review-owed: true`. The same holds for another spec's deleted path under 079, so the behavior is the existing one. — `runtime/src/primitives/compute_review_scope.rs:351-377`"
    outcome: waived
    reason: A deletion has nothing to read and was never listed in `excluded` under 079 either; the gate's answer (no review owed) is the one AC13 asks for, so only the sentence's reach is loose. Observation about the pipeline's machinery, not a break of what the criterion protects.
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: "other — `invalidate-review` keeps `unread` on an invalidated record (its 24-entry SCALARS list does not name it), so a record reading `last-run: null` still states what the run it disowns read; confirmed on fixture (`unread:\\n  - src/mine.rs` survives beside `last-run: null`). `read_records` requires `last_run.is_some()`, so no credit follows. — `runtime/src/primitives/invalidate_review.rs:117-142`"
    outcome: waived
    reason: a pipeline-machinery observation outside 081's plan files; the credit's `last-run` guard makes the kept field inert, and 066 owns the invalidation surface
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: other — `unread` accepts duplicate entries and writes each (`[src/mine.rs, src/mine.rs]` rendered twice); `compute-review-scope` reads the list as a set, so nothing differs. — `runtime/src/primitives/write_review.rs:1360-1375`
    outcome: waived
    reason: harmless to every reader; deduplicating adds a requirement the spec does not state
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: other — no test pins `--unread-json` hydration (`hydrate_write_review`) or that a run's two writes carry one `unread`; both hold on the built binary (loop-open write then final write each render the same list), as the sibling `--*-json` flags are equally unpinned. — `runtime/src/main.rs:380`
    outcome: waived
    reason: verified by driving the built binary; the sibling write-review flags have no hydration test either, so a pin here is a new test surface, not a fix to 081's code
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: "other — write-review records `unread: []` beside `examined: 1` and `scope: 4` (driven on a fixture): the two self-reported claims contradict each other, and the record then credits every path of another spec's commit in its window as read. Nothing compares `examined` plus the length of `unread` with `scope`. AC12 asks for the in-scope refusal only, and the `examined` doc already says nothing proves the passes ran, so this is the same self-report the pipeline has always taken. — `runtime/src/primitives/write_review.rs:496-497`"
    outcome: waived
    reason: "an observation about the pipeline's own machinery made while reviewing framework work: AC12 requires the scope-membership refusal and a run that states nothing writing nothing, both of which hold, and `examined` is documented as a caller's unverified claim (write_review.rs and WriteReviewArgs::examined); a consistency refusal would be a new requirement on 081, not a breach of its contract, and does not make this review's own result wrong."
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: other — render_unread repeats render_open_findings' block-list loop (`key:` then `  - {yaml_string}` per entry). A shared helper would serve both, but the two differ in the empty case (`[]` against omitted), and it would edit 079's renderer for a three-line loop. — `runtime/src/primitives/write_review.rs:1360-1384`
    outcome: waived
    reason: "an observation about the pipeline's own machinery that adds no requirement: two three-line loops whose empty cases differ, and extracting one touches spec 079's render_open_findings for no behavior change."
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: other — The credit's conditions (a record committed in the window, its window holding the commit, no skipped pass, budget stop or open fix-round loop, no unread means no credit) are restated in COMPUTE_REVIEW_SCOPE, framework/reference/review.md, the 020 and 066 data models, the CHANGELOG entry and the plan. Constitution drift-prevention prefers a pointer to the canonical source (the primitive's compiled description) over copies, and each copy can drift. The copies agree today (checked against read_records in compute_review_scope.rs). — `specs/066-audit-records-and-the-completion-gate/data-model.md:123`
    outcome: waived
    reason: Task 9 of 081 and AC11/AC12 require the reference and both data models to state unread and the credit; the copies match the code now. Pipeline-machinery observation on framework work, waived rather than deferred per AGENTS.md Workflow.
    decided-at: 2026-11-13T17:38:07Z
    decided-by: rule
  - key: 079's `AC6` sentence on a commit belonging to no spec is not annotated with 081's read credit of 2026-11-13 (AC13), though 081's AC5 asks only for the 2026-11-11 changes — `specs/079-review-and-analysis-work-converges/spec.md`
    outcome: waived
    reason: 081's AC5 is met as written; the fix lies in 079's criteria, a spec this run must not reopen, and is reported to the parent.
    decided-at: 2026-11-19T22:36:52Z
    decided-by: rule
---

# Review — 081-reviews-and-checks-follow-what-changed

## Summary

Review of 081's code changed since its last review, from `11a20ebd` (`last-review`) to `861c18ae`, for 084's task 19. The five passes read the spec's own change, the hunks of three commits (`f894be65`, `e54acd35`, `4959acf0`) in `.ductus/docs/agents/developing-ductus.md`, against the five loaded rule files, each read whole (231,962 bytes): no finding and one observation. The Review bullet briefed subagent B on "the other nine rule files", while the paragraph the same slice adds above it says a review loads five since 020's AC17, so B's files are two. A chore, fixed in the run by the rule: the bullet now names `quality-cross.md` and `configuration-cross.md`. The passes were re-run over that fix's diff, against the same five rule files read whole: no finding and no observation, the bullet agreeing with the earlier paragraph and with `discover-rule-files`, which selects those five here. One fix round, re-read by an independent agent from `brief-re-read`'s brief: clean, and the next round empty. The new bullets' fetch counts, token figures and incident dates are session history, which no source at hand could re-check. The criteria step judged all 14 criteria met against the tree; AC10, AC13 and AC14 were judged in the part 084 left in force. Stored decisions: 10 retained on files this scope does not hold; none names `.ductus/docs/agents/developing-ductus.md`. Examined 1 of 7: `specs/081-reviews-and-checks-follow-what-changed/spec.md`, read whole by the criteria step. Unread, by the operator's decision to read each spec's own change (084's `tasks.md`, task 19): `.ductus/docs/agents/developing-ductus.md`, read at the slice's hunks and the Review bullet's and Sizing paragraph's lines; `framework/reference/review.md` and `specs/081-reviews-and-checks-follow-what-changed/plan.md`, read in part; and `.ductus/reference/review.md`, `runtime/CHANGELOG.md` and 081's `analysis.md`, not opened by the passes.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

10 of 506 commits in the window no spec claimed.

14 of 14 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: `runtime/src/primitives/compute_review_scope.rs` `paths_owing_a_review`/`owes_a_review` (is_project_artifact); `a_reopening_that_changed_code_owes_a_review_and_a_contract_edit_does_not` and `an_artifact_only_window_owes_a_review_while_a_criterion_is_unjudged` (the part 066 task 27 superseded) pass.
- AC2 — met: `runtime/src/primitives/check_review_gate.rs` has no `ReviewStale` block (grep of `runtime/src` finds none); `a_contract_change_since_the_review_holds_done_only_on_the_analysis` proves AnalyzeStale for a contract edit and ReviewOwed naming `src/code.rs` for code.
- AC3 — met: `runtime/src/primitives/check_review_freshness.rs` and `scripts/audit/review-freshness.sh` are absent, `scripts/audit/run-all.sh` registers no Family 19, `ductus check-review-freshness` is an unrecognized subcommand, and `RETIRED_ALLOW` in `runtime/src/primitives/merge_permissions.rs` holds `mcp__ductus__check-review-freshness` (suite incl. `the_retired_set_is_configure_claudes_step_4` passes).
- AC4 — met: `framework/constitution.md` §grounding blast-radius bullet states review for code, analyze for artifacts and the owed checks; `framework/reference/review.md` §Window and coverage and the `reviewed-digest` paragraph say a changed contract stales no review.
- AC5 — met: `specs/079-review-and-analysis-work-converges/spec.md` AC4 and AC6 carry 'Superseded in part (spec 081)' annotations and the Resolved Questions on scenario edits and attribution say 'Spec 081 (owner, 2026-11-11)' and point to 081.
- AC6 — met: `.github/workflows/framework-checks.yml` and `.github/workflows/generators.yml` key the cache on runtime sources, date files back on an exact hit and print 'No build of these runtime sources is cached' otherwise; `.github/workflows/runtime.yml` filters list only `specs/066-audit-records-and-the-completion-gate/data-model.md` (read by `runtime/src/primitives/write_analysis.rs`); `AGENTS.md` local-checks rule asks for no build. Judged from workflow text; no CI run read.
- AC7 — met: The behavior lives in `runtime/src/primitives/compute_review_scope.rs`, `framework/constitution.md`, `framework/reference/review.md`, `framework/commands/review.md` and the `framework/bootstrap/configure/claude.md` retired list; only the CI cache and the local-checks bullet are repo-only, and the latter is mirrored in the constitution; no test drives this criterion.
- AC8 — met: `is_project_artifact` in `runtime/src/primitives/mod.rs` (test `the_spec_root_ductus_and_the_agent_files_are_artifacts`); `a_configured_spec_root_is_the_artifact_root` and `a_subdirectory_project_reads_its_artifacts_from_the_project_root` in `compute_review_scope.rs` pass; the 066 task 27 criteria-owing part is covered by `an_artifact_only_window_owes_a_review_while_a_criterion_is_unjudged`.
- AC9 — met: `attribute_window` in `runtime/src/primitives/compute_review_scope.rs` with `CommitClaims::named`; `a_named_commits_code_goes_to_every_spec_it_names` and `a_spec_claiming_a_named_commit_only_by_its_directory_reviews_only_that_directory` pass, both directions.
- AC10 — met: Part still in force (attribution as under 079): `a_planned_path_of_an_unnamed_multi_spec_commit_goes_to_the_plan_naming_it` and `an_unplanned_path_of_a_multi_spec_commit_naming_none_goes_to_every_claimant` pass; the part spec 084 superseded (a code-only commit naming no spec staying in scope and owing a review) is replaced by `a_code_commit_naming_no_spec_leaves_an_open_specs_scope`, which passes.
- AC11 — met: `ductus compute-review-scope --help` (from `runtime/src/schema/descriptions.rs`) states the attribution rule and the artifact set (spec root, `.ductus/`, `AGENTS.md`, `CLAUDE.md`); `framework/reference/review.md` §Window and coverage states both.
- AC12 — met: `runtime/src/primitives/write_review.rs` `refuse_unread_outside_the_scope`/`render_unread`; `unread_is_written_whenever_the_run_states_it` and `an_unread_entry_outside_the_scope_is_refused` pass; `framework/commands/review.md` step 11, `framework/constitution.md` review table and `specs/020-code-review/data-model.md` list `unread` (066's data model describes it in prose, not a table row).
- AC13 — met: Only the part spec 084 left in force is judged: a path of a commit naming no spec and touching no spec directory leaves scope, is listed in `excluded` and owes no review, proven by `a_code_commit_naming_no_spec_leaves_an_open_specs_scope` and `another_specs_record_neither_keeps_nor_credits_an_unclaimed_path` in `compute_review_scope.rs`; the credit mechanism itself was superseded by 084 (its AC6).
- AC14 — met: The 081 credit fixtures were superseded by 084; the replacement `another_specs_record_neither_keeps_nor_credits_an_unclaimed_path` passes over `unread: []`, a listed unread entry, no `unread` and a skipped pass, each leaving the path excluded with no review owed, and `a_later_change_of_the_specs_own_brings_an_unclaimed_path_back` keeps a path another commit of the spec changed in scope.

## Observations

- convention: the Review bullet in `.ductus/docs/agents/developing-ductus.md` still briefs subagent B on "the other nine rule files", while the paragraph the same slice adds above it (since 020's AC17) says a review loads five files, so B's rule files are now two, `quality-cross.md` and `configuration-cross.md`; seven of the nine are judged by analysis alone. A maintainer who follows the bullet briefs B on files the review does not load. — `.ductus/docs/agents/developing-ductus.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md`
- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/clarify.md`
- `.claude/commands/ductus/configure.md`
- `.claude/commands/ductus/consolidate.md`
- `.claude/commands/ductus/fold.md`
- `.claude/commands/ductus/groom.md`
- `.claude/commands/ductus/help.md`
- `.claude/commands/ductus/implement.md`
- `.claude/commands/ductus/prune.md`
- `.claude/commands/ductus/review.md`
- `.claude/commands/ductus/specify.md`
- `.claude/commands/ductus/status.md`
- `.ductus/config.toml`
- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/clarify.md`
- `.ductus/reference/fold.md`
- `.ductus/reference/groom.md`
- `.ductus/reference/implement.md`
- `.ductus/reference/prune.md`
- `.gitattributes`
- `.githooks/pre-push`
- `.github/workflows/framework-checks.yml`
- `.github/workflows/runtime-currency.yml`
- `.github/workflows/runtime-release.yml`
- `.github/workflows/runtime.yml`
- `.pi/prompts/ductus-amend.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-clarify.md`
- `.pi/prompts/ductus-consolidate.md`
- `.pi/prompts/ductus-fold.md`
- `.pi/prompts/ductus-groom.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-implement.md`
- `.pi/prompts/ductus-prune.md`
- `.pi/prompts/ductus-review.md`
- `.pi/prompts/ductus-specify.md`
- `.pi/prompts/ductus-status.md`
- `AGENTS.md`
- `README.md`
- `docs/analyze.md`
- `docs/rules.md`
- `docs/shared-constitution.md`
- `docs/slash-commands.md`
- `framework/bootstrap/configure/claude.md`
- `framework/bootstrap/ductus-procedure.md`
- `framework/bootstrap/ductus.md`
- `framework/bootstrap/govern.md`
- `framework/bootstrap/hooks/ductus-pre-push`
- `framework/bootstrap/hooks/pre-push`
- `framework/commands/amend.md`
- `framework/commands/analyze.md`
- `framework/commands/clarify.md`
- `framework/commands/consolidate.md`
- `framework/commands/fold.md`
- `framework/commands/groom.md`
- `framework/commands/help.md`
- `framework/commands/implement.md`
- `framework/commands/prune.md`
- `framework/commands/review.md`
- `framework/commands/specify.md`
- `framework/commands/status.md`
- `framework/constitution.md`
- `framework/migrations.toml`
- `framework/migrations/audit-record-relocate.md`
- `framework/migrations/criterion-label-backfill.md`
- `framework/migrations/project-review-record.md`
- `framework/migrations/remove-legacy-constitutions-region.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/clarify.md`
- `framework/reference/fold.md`
- `framework/reference/groom.md`
- `framework/reference/implement.md`
- `framework/reference/prune.md`
- `framework/rules/accessibility-frontend.md`
- `framework/rules/api-backend.md`
- `framework/rules/concurrency-backend.md`
- `framework/rules/configuration-cross.md`
- `framework/rules/reliability-backend.md`
- `framework/rules/security-backend.md`
- `framework/rules/security-cross.md`
- `framework/rules/security-frontend.md`
- `framework/runtime-tools.txt`
- `framework/templates/ci/adopter-generators.yml`
- `framework/templates/spec/spec.md`
- `install.sh`
- `runtime/Cargo.lock`
- `runtime/Cargo.toml`
- `runtime/src/interpreter/analyze_tally.rs`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/main.rs`
- `runtime/src/mcp/server.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/append_inbox.rs`
- `runtime/src/primitives/append_question.rs`
- `runtime/src/primitives/bounded_lists.rs`
- `runtime/src/primitives/brief_re_read.rs`
- `runtime/src/primitives/check_artifact_size.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_command_flags.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_departures.rs`
- `runtime/src/primitives/check_orphaned_references.rs`
- `runtime/src/primitives/check_owed_work.rs`
- `runtime/src/primitives/check_prescriptions.rs`
- `runtime/src/primitives/check_review_gate.rs`
- `runtime/src/primitives/check_step_references.rs`
- `runtime/src/primitives/check_stuck.rs`
- `runtime/src/primitives/check_unreviewed_code.rs`
- `runtime/src/primitives/commit_attribution.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/criterion_verdicts.rs`
- `runtime/src/primitives/dashboard.rs`
- `runtime/src/primitives/decisions.rs`
- `runtime/src/primitives/departures.rs`
- `runtime/src/primitives/derive_references.rs`
- `runtime/src/primitives/derive_routing_candidates.rs`
- `runtime/src/primitives/discover_rule_files.rs`
- `runtime/src/primitives/fix_rounds.rs`
- `runtime/src/primitives/guarded.rs`
- `runtime/src/primitives/invalidate_review.rs`
- `runtime/src/primitives/label_criteria.rs`
- `runtime/src/primitives/line_citations.rs`
- `runtime/src/primitives/lint_markdown.rs`
- `runtime/src/primitives/mark_criterion.rs`
- `runtime/src/primitives/mark_task.rs`
- `runtime/src/primitives/measure_reopenings.rs`
- `runtime/src/primitives/mechanical_sweep.rs`
- `runtime/src/primitives/merge_managed_block.rs`
- `runtime/src/primitives/merge_permissions.rs`
- `runtime/src/primitives/merge_status_hook.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/owed_work.rs`
- `runtime/src/primitives/pipeline_view.rs`
- `runtime/src/primitives/process_decisions.rs`
- `runtime/src/primitives/process_waivers.rs`
- `runtime/src/primitives/prune_plan.rs`
- `runtime/src/primitives/prune_project.rs`
- `runtime/src/primitives/prune_tasks.rs`
- `runtime/src/primitives/read_re_read.rs`
- `runtime/src/primitives/read_spec.rs`
- `runtime/src/primitives/read_tasks.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/remove_inbox_item.rs`
- `runtime/src/primitives/resolve_anchor.rs`
- `runtime/src/primitives/resolve_constitutions.rs`
- `runtime/src/primitives/review_subject.rs`
- `runtime/src/primitives/rewrite_spec_links.rs`
- `runtime/src/primitives/rule_verdicts.rs`
- `runtime/src/primitives/set_status.rs`
- `runtime/src/primitives/shipped_rule_ids.txt`
- `runtime/src/primitives/spec_links.rs`
- `runtime/src/primitives/start_fix_rounds.rs`
- `runtime/src/primitives/write_analysis.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/primitives/write_review/departure_waivers.rs`
- `runtime/src/primitives/write_review/moved_scope.rs`
- `runtime/src/primitives/write_review/project_record.rs`
- `runtime/src/primitives/write_review/seed.rs`
- `runtime/src/schema/config_keys.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/extensions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/src/schema/protocol.rs`
- `runtime/src/schema/registry.rs`
- `runtime/tests/bounded_list_gates.rs`
- `runtime/tests/check_step_references_cli.rs`
- `runtime/tests/check_unreviewed_code_cli.rs`
- `runtime/tests/crlf_preservation.rs`
- `runtime/tests/derive_dependencies_golden.rs`
- `runtime/tests/derive_references_golden.rs`
- `runtime/tests/exec_subprocess.rs`
- `runtime/tests/fix_rounds.rs`
- `runtime/tests/golden/implement-basic.jsonl`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/golden/specify-basic.jsonl`
- `runtime/tests/line_ending_discipline.rs`
- `runtime/tests/mcp.rs`
- `runtime/tests/measure_reopenings.rs`
- `runtime/tests/mechanical_sweep_parity.rs`
- `runtime/tests/merge_status_hook.rs`
- `runtime/tests/parity.rs`
- `runtime/tests/result_sizes.rs`
- `runtime/tests/review_command.rs`
- `runtime/tests/walker.rs`
- `scripts/analyze/briefs.py`
- `scripts/analyze/check.py`
- `scripts/analyze/detect.py`
- `scripts/analyze/judge-brief.md`
- `scripts/analyze/record.py`
- `scripts/analyze/split.py`
- `scripts/analyze/step15-brief.md`
- `scripts/audit/README.md`
- `scripts/audit/adopter-shell-behavior.sh`
- `scripts/audit/archive-contents.sh`
- `scripts/audit/cross-doc-consistency.sh`
- `scripts/audit/done-spec-criteria.sh`
- `scripts/audit/permission-wildcard-position.sh`
- `scripts/audit/rename-sweep-residue.sh`
- `scripts/audit/run-all.sh`
- `scripts/audit/ssot-invariants.sh`
- `scripts/audit/step-reference-integrity.sh`
- `scripts/audit/sweep-target-manifest-parity.sh`
- `scripts/check-release-tag.sh`
- `scripts/mutation/analysis-design-fix-067.py`
- `scripts/mutation/command-reference-split.py`
- `scripts/mutation/commit-counts.py`
- `scripts/mutation/compare-and-set-066.py`
- `scripts/mutation/criteria-in-review-066.py`
- `scripts/mutation/criterion-ticks-outside-the-digest.py`
- `scripts/mutation/departure-waivers.py`
- `scripts/mutation/departures.py`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/exec-scenario-anchors-067.py`
- `scripts/mutation/freshness-results-have-a-bounded-size.py`
- `scripts/mutation/installed-file-modes.py`
- `scripts/mutation/line-citation-strip-066.py`
- `scripts/mutation/loop-inputs-screened.py`
- `scripts/mutation/moved-scope-066.py`
- `scripts/mutation/never-owed-084.py`
- `scripts/mutation/owed-work-084.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/plan-edit-takes-no-back-edge.py`
- `scripts/mutation/process-decisions-results-bounded.py`
- `scripts/mutation/project-record-084.py`
- `scripts/mutation/prune-project.py`
- `scripts/mutation/quoted-heading-forms-067.py`
- `scripts/mutation/reachable-trigger-bar-078.py`
- `scripts/mutation/relocate-merge-children-057.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/review-022-fixes-2026-11-19.py`
- `scripts/mutation/review-022-task-41.py`
- `scripts/mutation/review-022-task-42.py`
- `scripts/mutation/review-022-task-46.py`
- `scripts/mutation/review-departures-bounded.py`
- `scripts/mutation/review-read-credit.py`
- `scripts/mutation/reviews-follow-what-changed.py`
- `scripts/mutation/rewrite-spec-links.py`
- `scripts/mutation/routing-path-claims.py`
- `scripts/mutation/rule-files-left-to-analysis-020.py`
- `scripts/mutation/rule-verdicts-converge.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/site-anchors-020.py`
- `scripts/mutation/status-count-084.py`
- `scripts/mutation/stderr-warnings-022.py`
- `scripts/mutation/step-references-reach.py`
- `scripts/mutation/suffixed-task-number.py`
- `scripts/mutation/unreviewed-code-084.py`
- `scripts/mutation/waiver-notices-flatten.py`
- `scripts/mutation/waivers-bounded.py`
- `scripts/tests/test-analyze-scripts.py`
- `scripts/tests/test-archive-contents.sh`
- `scripts/tests/test-audit-path-containment.sh`
- `scripts/tests/test-check-release-tag.sh`
- `scripts/tests/test-done-spec-owed-work.sh`
- `scripts/tests/test-run-workflow-steps.sh`
- `specs/000-slash-commands/review.md`
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md`
- `specs/000-slash-commands/scenarios/scenario-without-task-visibility.md`
- `specs/008-security-rules/analysis.md`
- `specs/008-security-rules/plan.md`
- `specs/008-security-rules/review.md`
- `specs/008-security-rules/scenarios/a-cache-partition-is-purged-when-the-session-ends.md`
- `specs/008-security-rules/scenarios/a-statement-carries-one-obligation-keyword.md`
- `specs/008-security-rules/scenarios/a-throwaway-credential-owes-no-revocation-path.md`
- `specs/008-security-rules/scenarios/an-absent-origin-on-a-cookie-connection-is-flagged.md`
- `specs/008-security-rules/scenarios/be-deps-005-flags-what-its-statement-requires.md`
- `specs/008-security-rules/scenarios/rules-are-as-short-as-their-required-content.md`
- `specs/008-security-rules/scenarios/security-rules-flag-what-their-sources-support.md`
- `specs/008-security-rules/scenarios/the-tag-check-reads-every-entry-above-the-newest-version.md`
- `specs/008-security-rules/spec.md`
- `specs/008-security-rules/tasks.md`
- `specs/010-agent-autonomy/plan.md`
- `specs/011-brownfield-process/spec.md`
- `specs/012-multi-agent-govern/review.md`
- `specs/012-multi-agent-govern/spec.md`
- `specs/013-text-first-artifacts/plan.md`
- `specs/013-text-first-artifacts/scenarios/criterion-identifiers.md`
- `specs/014-reclarify-backedge/plan.md`
- `specs/014-reclarify-backedge/scenarios/reopen-after-informal-edits.md`
- `specs/014-reclarify-backedge/spec.md`
- `specs/015-tarball-fetch/analysis.md`
- `specs/015-tarball-fetch/plan.md`
- `specs/015-tarball-fetch/review.md`
- `specs/015-tarball-fetch/spec.md`
- `specs/015-tarball-fetch/tasks.md`
- `specs/016-cross-cutting-rules/spec.md`
- `specs/017-derive-dont-ask/analysis.md`
- `specs/017-derive-dont-ask/data-model.md`
- `specs/017-derive-dont-ask/plan.md`
- `specs/017-derive-dont-ask/review.md`
- `specs/017-derive-dont-ask/scenarios/a-path-list-commit-leaves-the-shared-index-current.md`
- `specs/017-derive-dont-ask/scenarios/a-throwaway-credential-is-not-a-secret.md`
- `specs/017-derive-dont-ask/spec.md`
- `specs/017-derive-dont-ask/tasks.md`
- `specs/018-adopter-owned-pre-commit/spec.md`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/plan.md`
- `specs/020-code-review/review.md`
- `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/scenarios/waiver-file-lists.md`
- `specs/020-code-review/spec.md`
- `specs/020-code-review/tasks.md`
- `specs/022-deterministic-runtime/analysis.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/plan.md`
- `specs/022-deterministic-runtime/review.md`
- `specs/022-deterministic-runtime/scenarios/a-closed-stdout-is-a-quiet-exit.md`
- `specs/022-deterministic-runtime/scenarios/ask-consolidation.md`
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md`
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md`
- `specs/022-deterministic-runtime/scenarios/groom-command-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/022-deterministic-runtime/scenarios/routing-ranks-by-claimed-path.md`
- `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`
- `specs/022-deterministic-runtime/scenarios/skipscanner-inline-code-exemption.md`
- `specs/022-deterministic-runtime/scenarios/spec-side-parser-hardening.md`
- `specs/022-deterministic-runtime/scenarios/specify-routes-before-scaffolding.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/023-govern-refinement/analysis.md`
- `specs/023-govern-refinement/plan.md`
- `specs/023-govern-refinement/review.md`
- `specs/023-govern-refinement/scenarios/configure-inert-write-path-entries.md`
- `specs/023-govern-refinement/scenarios/configure-permission-pattern-safety.md`
- `specs/023-govern-refinement/scenarios/living-specs.md`
- `specs/023-govern-refinement/spec.md`
- `specs/023-govern-refinement/tasks.md`
- `specs/024-rule-loader/analysis.md`
- `specs/024-rule-loader/plan.md`
- `specs/024-rule-loader/review.md`
- `specs/024-rule-loader/spec.md`
- `specs/025-rule-opt-out/review.md`
- `specs/025-rule-opt-out/spec.md`
- `specs/026-framework-self-audit/analysis.md`
- `specs/026-framework-self-audit/plan.md`
- `specs/026-framework-self-audit/review.md`
- `specs/026-framework-self-audit/scenarios/family-23-sweep-target-manifest-parity.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-40-deprecated-rules-are-removed-on-time.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/030-cross-service-references/plan.md`
- `specs/030-cross-service-references/spec.md`
- `specs/034-performance-backend-rules/analysis.md`
- `specs/034-performance-backend-rules/review.md`
- `specs/034-performance-backend-rules/spec.md`
- `specs/035-groom-session-target/scenarios/confirmation-names-reopen.md`
- `specs/035-groom-session-target/scenarios/reopen-done-spec-on-scenario.md`
- `specs/037-observability-backend-rules/analysis.md`
- `specs/037-observability-backend-rules/plan.md`
- `specs/037-observability-backend-rules/review.md`
- `specs/037-observability-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/analysis.md`
- `specs/038-concurrency-backend-rules/plan.md`
- `specs/038-concurrency-backend-rules/review.md`
- `specs/038-concurrency-backend-rules/scenarios/a-declared-loss-tolerant-message-owes-no-outbox.md`
- `specs/038-concurrency-backend-rules/spec.md`
- `specs/038-concurrency-backend-rules/tasks.md`
- `specs/039-reliability-backend-rules/analysis.md`
- `specs/039-reliability-backend-rules/plan.md`
- `specs/039-reliability-backend-rules/review.md`
- `specs/039-reliability-backend-rules/scenarios/expendable-messages-may-be-dropped-when-recorded.md`
- `specs/039-reliability-backend-rules/spec.md`
- `specs/039-reliability-backend-rules/tasks.md`
- `specs/041-task-pruning/analysis.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/review.md`
- `specs/041-task-pruning/scenarios/a-plan-edit-does-not-reopen-a-done-spec.md`
- `specs/041-task-pruning/scenarios/a-suffixed-task-number-is-a-task.md`
- `specs/041-task-pruning/scenarios/prune-all-walks-every-spec.md`
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`
- `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md`
- `specs/041-task-pruning/spec.md`
- `specs/046-scenario-open-question-visibility/plan.md`
- `specs/046-scenario-open-question-visibility/spec.md`
- `specs/047-analyze-findings-durability/scenarios/analyze-record-freshness.md`
- `specs/047-analyze-findings-durability/spec.md`
- `specs/048-govern-acquired-runtime/analysis.md`
- `specs/048-govern-acquired-runtime/plan.md`
- `specs/048-govern-acquired-runtime/review.md`
- `specs/048-govern-acquired-runtime/scenarios/pointer-follows-the-store.md`
- `specs/048-govern-acquired-runtime/spec.md`
- `specs/048-govern-acquired-runtime/tasks.md`
- `specs/049-rename-govern-to-ductus/plan.md`
- `specs/049-rename-govern-to-ductus/review.md`
- `specs/049-rename-govern-to-ductus/spec.md`
- `specs/050-constitution/analysis.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/review.md`
- `specs/050-constitution/scenarios/a-canonical-source-is-pointed-at-not-copied.md`
- `specs/050-constitution/scenarios/a-decision-is-attributed-to-whoever-made-it.md`
- `specs/050-constitution/scenarios/a-declared-cross-spec-impact-gates-done.md`
- `specs/050-constitution/scenarios/a-primitive-is-probed-in-a-local-clone.md`
- `specs/050-constitution/scenarios/a-retired-filename-leaves-a-decision-record.md`
- `specs/050-constitution/scenarios/adopter-practices-classified-for-promotion.md`
- `specs/050-constitution/spec.md`
- `specs/050-constitution/tasks.md`
- `specs/051-branch-scoped-spec-numbering/analysis.md`
- `specs/051-branch-scoped-spec-numbering/plan.md`
- `specs/051-branch-scoped-spec-numbering/review.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-fold-owed-to-another-tree-does-not-hold-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/a-pending-fold-never-holds-done.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/fold-target-checked-before-the-rewrite.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/rewrites-preserve-line-endings.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/051-branch-scoped-spec-numbering/tasks.md`
- `specs/054-remove-supersession/plan.md`
- `specs/054-remove-supersession/spec.md`
- `specs/055-shared-constitution/analysis.md`
- `specs/055-shared-constitution/plan.md`
- `specs/055-shared-constitution/review.md`
- `specs/055-shared-constitution/spec.md`
- `specs/055-shared-constitution/tasks.md`
- `specs/057-analyze-artifact-and-record-relocation/analysis.md`
- `specs/057-analyze-artifact-and-record-relocation/data-model.md`
- `specs/057-analyze-artifact-and-record-relocation/review.md`
- `specs/057-analyze-artifact-and-record-relocation/scenarios/a-merge-carries-each-keys-whole-value.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/058-findings-route-at-discovery/analysis.md`
- `specs/058-findings-route-at-discovery/data-model.md`
- `specs/058-findings-route-at-discovery/plan.md`
- `specs/058-findings-route-at-discovery/review.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/review.md`
- `specs/062-concurrent-session-targets/review.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/a-plan-trim-does-not-reopen-a-done-spec.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/scenarios/project-files-are-measured.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-commit-after-the-scope-was-resolved-refuses-the-write.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-write-result-states-what-it-wrote.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/process-waivers-file-lists.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-scope-includes-uncommitted-work.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/analysis.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/review.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-none-list-item-is-no-open-question.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-plan-heading-is-matched-without-regard-to-case.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-record-cites-a-removed-rule.md`
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`
- `specs/067-analyze-and-audit-checks/scenarios/anchor-reference-kinds.md`
- `specs/067-analyze-and-audit-checks/scenarios/check-artifacts-skipped-targets.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-results-are-bounded.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-unstaged-drift-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/disposition-drift-is-advisory.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md`
- `specs/067-analyze-and-audit-checks/scenarios/shared-constitution-references-are-judged.md`
- `specs/067-analyze-and-audit-checks/scenarios/traverse-deps-cycle-check.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/068-api-backend-rules/analysis.md`
- `specs/068-api-backend-rules/plan.md`
- `specs/068-api-backend-rules/review.md`
- `specs/068-api-backend-rules/scenarios/repr-003-bounded-numbers-and-patch-001-format-home.md`
- `specs/068-api-backend-rules/spec.md`
- `specs/068-api-backend-rules/tasks.md`
- `specs/069-accessibility-frontend-rules/analysis.md`
- `specs/069-accessibility-frontend-rules/plan.md`
- `specs/069-accessibility-frontend-rules/review.md`
- `specs/069-accessibility-frontend-rules/scenarios/verifications-flag-only-what-the-statement-covers.md`
- `specs/069-accessibility-frontend-rules/spec.md`
- `specs/069-accessibility-frontend-rules/tasks.md`
- `specs/070-performance-frontend-rules/analysis.md`
- `specs/070-performance-frontend-rules/review.md`
- `specs/070-performance-frontend-rules/spec.md`
- `specs/071-command-reference-split/analysis.md`
- `specs/071-command-reference-split/plan.md`
- `specs/071-command-reference-split/review.md`
- `specs/071-command-reference-split/spec.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/073-decided-rule-departures-are-recorded/analysis.md`
- `specs/073-decided-rule-departures-are-recorded/data-model.md`
- `specs/073-decided-rule-departures-are-recorded/plan.md`
- `specs/073-decided-rule-departures-are-recorded/review.md`
- `specs/073-decided-rule-departures-are-recorded/spec.md`
- `specs/073-decided-rule-departures-are-recorded/tasks.md`
- `specs/075-agents-md-holds-what-every-session-needs/plan.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/analysis.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/data-model.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/review.md`
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/spec.md`
- `specs/077-security-cross-rules/analysis.md`
- `specs/077-security-cross-rules/data-model.md`
- `specs/077-security-cross-rules/plan.md`
- `specs/077-security-cross-rules/review.md`
- `specs/077-security-cross-rules/scenarios/url-pii-query-only-and-oauth-stricter-than-rfc-9700.md`
- `specs/077-security-cross-rules/spec.md`
- `specs/077-security-cross-rules/tasks.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/analysis.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/data-model.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/review.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/research.md`
- `specs/079-review-and-analysis-work-converges/review.md`
- `specs/079-review-and-analysis-work-converges/scenarios/the-excluded-section-lists-paths-only.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/analysis.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/review.md`
- `specs/080-deterministic-runtime-efficiency/scenarios/groom-derives-routing-candidates.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`
- `specs/082-rule-assessments-converge-across-runs/analysis.md`
- `specs/082-rule-assessments-converge-across-runs/data-model.md`
- `specs/082-rule-assessments-converge-across-runs/plan.md`
- `specs/082-rule-assessments-converge-across-runs/review.md`
- `specs/082-rule-assessments-converge-across-runs/spec.md`
- `specs/082-rule-assessments-converge-across-runs/tasks.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/analysis.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/plan.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/review.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/spec.md`
- `specs/083-commits-are-attributed-to-specs-without-a-subject-convention/tasks.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/data-model.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/plan.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/spec.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/tasks.md`
- `specs/085-the-audit-record-gate-reads-records-through-the-runtime/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/plan.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/spec.md`
- `specs/086-review-records-hold-only-what-their-scope-judged/tasks.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/plan.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/spec.md`
- `specs/087-parts-moved-out-of-a-project-file-are-read-measured-and-pruned-as-that-file/tasks.md`
- `specs/README.md`
- `specs/inbox.md`
- `specs/review.md`
- `version`

## Unexamined governance

*None.*
