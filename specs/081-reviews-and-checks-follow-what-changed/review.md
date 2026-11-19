---
spec: 081-reviews-and-checks-follow-what-changed
last-run: 2026-11-19T22:36:52Z
reviewed-against: 9826d5f21aaa791af18df8a70fab0b7e5fcb2e12
diff-base: 64946fe729a18e8ffe3c86f7276dfa658909f84f
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 5
unread:
  - .ductus/reference/review.md
  - framework/reference/review.md
  - runtime/CHANGELOG.md
  - specs/081-reviews-and-checks-follow-what-changed/plan.md
excluded: 34
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
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
    digest: c4e71a70729fdb0206f970d3da9376b17c0b4180ca9f2de19d051cff92646a7f
  - label: AC11
    digest: 99f30dcaebee3b9d8022d5befb784d581b358d10e19c608e089a9d19ca468382
  - label: AC12
    digest: 8f7fb7dbd7b9528ece61066f9efa371e1c90ed5ee5bb1b35a35eaf720b86021c
  - label: AC13
    digest: 18c0cde89f84eb2df1d3d9f1cdb2d2a0be886a79a6c2153a322b7ca91d56eb2d
  - label: AC14
    digest: 279e204a95da01794f7703ac2b5cbe8dd9b446c5b6b55633898e5796108eb1b1
blocking: false
dispositions:
  fixed: 2
  deferred: 0
  waived: 2
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

Window 64946fe7 (the 2026-11-19 reopen of AC13) to HEAD. The five passes ran over the scope's design file specs/081-reviews-and-checks-follow-what-changed/spec.md (20,618 bytes, read whole before round 2's edit) against all twelve rule files read whole (408,526 bytes), specs/system.md and AGENTS.md Gotchas to Design Principles: no violation, one observation matching a stored waiver. The criteria step judged all 14 criteria one by one against the tree (409 tests passed in cargo test --lib over compute_review_scope, check_review_gate, write_review and commit_attribution, dev profile); AC6 as first read was not met, since runtime.yml's filters list the 066 data model, and its text was corrected through round 2. Round 1 fixed stale claims that a credited path is excluded under a spec (framework/reference/review.md and its copy, runtime/CHANGELOG.md, plan.md), and a fresh re-read of its diff found nothing; round 2 edited only spec.md and plan.md (exempt) and I read its four changed lines myself, so spec.md's round-2 text and the four fix paths are named unread, not examined. The eight stored decisions whose keys name runtime files outside this window are retained untouched.

2 fix round(s): 1 re-read, 0 diff-checked, 1 exempt, 0 not re-read. Exempt: round 2.

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

- AC1 — met: `compute_review_scope` `paths_owing_a_review` keeps only non-artifact paths (`is_project_artifact` in `runtime/src/primitives/mod.rs`), and `owes_a_review` adds the record-holds and unjudged-criterion clauses; tests `a_reopening_that_changed_code_owes_a_review_and_a_contract_edit_does_not` and `an_artifact_only_window_owes_a_review_while_a_criterion_is_unjudged` pass (409 passed across compute_review_scope, check_review_gate, write_review, commit_attribution)
- AC2 — met: `check_review_gate` has no review-stale check and `ReviewGateBlock` no such variant; test `a_contract_change_since_the_review_holds_done_only_on_the_analysis` proves both directions (a scenario edit blocks AnalyzeStale only, a `src/code.rs` commit blocks ReviewOwed)
- AC3 — met: `check-review-freshness` is an unrecognized subcommand of the built binary and absent from `tools/list`; `scripts/audit/run-all.sh` registers no Family 19; `RETIRED_ALLOW` in `runtime/src/primitives/merge_permissions.rs` lists the entry and agrees with `framework/bootstrap/configure/claude.md` step 4 (compared by reading, the equality test was not in the run's filter)
- AC4 — met: `framework/constitution.md` states review for code, analyze for artifacts and the check-selection rule; `framework/reference/review.md` says a changed contract stales no review
- AC5 — met: 079's `spec.md` AC4 and AC6 carry the 'Superseded in part (spec 081)' annotations and the two Resolved Questions end with the 081 sentences of 2026-11-11; the later credit is unannotated in 079, which AC5 does not ask for (observation waived)
- AC6 — met: `framework-checks.yml` and `generators.yml` key the `runtime/target` cache on `Cargo.lock`, `Cargo.toml`, `rust-toolchain.toml` and `runtime/src/**`, date tracked files back on an exact hit and print `No build of these runtime sources is cached, so this run compiles one.` on a miss; push `b2421e14` (spec-only) ran both workflows with 'Build the runtime' at 1 s and no `runtime.yml` run; `runtime.yml`'s filters list only `specs/066-audit-records-and-the-completion-gate/data-model.md` among the spec files, which the corrected criterion text names; the criterion as first read (no exception) was not met and its text is corrected, see the observations
- AC7 — met: every behavior change lands in `runtime/src`, `framework/constitution.md`, `framework/reference/review.md`, `framework/commands/review.md` and `framework/bootstrap/configure/claude.md`, with `runtime/CHANGELOG.md` [Unreleased] entries; the repo-only files carry cache and maintenance, not pipeline behavior
- AC8 — met: `is_project_artifact` tests `the_spec_root_ductus_and_the_agent_files_are_artifacts`, `every_other_path_is_code`, `a_configured_spec_root_is_the_artifact_root` and `a_subdirectory_project_reads_its_artifacts_from_the_project_root` pass; the 066 clause is proven by `an_artifact_only_window_owes_a_review_while_a_criterion_is_unjudged`
- AC9 — met: `attribute_window` gives a named commit's outside paths to the named specs and a directory-only claimant its own directory; tests `a_named_commits_code_goes_to_every_spec_it_names` and `a_spec_claiming_a_named_commit_only_by_its_directory_reviews_only_that_directory` pass
- AC10 — met: `attribute_window` keeps 079's rule for an unnamed commit and `take_unclaimed` credits only on a record; tests `a_code_commit_naming_no_spec_owes_the_review_of_each_window_holding_it` and `a_path_no_complete_record_read_stays_and_owes_the_review` pass
- AC11 — met: `COMPUTE_REVIEW_SCOPE` in `runtime/src/schema/descriptions.rs` and the bullets of `framework/reference/review.md` Window and coverage state the attribution, the credit and the artifact set
- AC12 — met: `write_review` `resolve_scope` refuses an `unread` entry outside the scope and `render_unread` writes `[]`, the list or nothing; tests `unread_is_written_whenever_the_run_states_it` and `an_unread_entry_outside_the_scope_is_refused` pass; `framework/commands/review.md` step 11, the constitution table and the 020 and 066 data models list `unread`
- AC13 — met: `compute_review_scope` `attribute_window` -> `take_unclaimed` credits a path of a no-spec commit touching no spec directory when `read_records` holds a committed record (last-run set, no skipped pass, budget stop or open loop, `unread` stated without the path) whose `commits_between(diff_base, reviewed_against)` holds the commit; `split_changes` lists it in `excluded` as a path alone and it owes no review; an uncredited path of the commit stays in scope; tests `a_path_a_complete_record_read_is_credited_and_owes_no_review` and `a_record_credits_each_path_it_did_not_name_unread` pass; agrees with 079's scenario `the-excluded-section-lists-paths-only`
- AC14 — met: the fixture tests `a_path_a_complete_record_read_is_credited_and_owes_no_review`, `a_path_no_complete_record_read_stays_and_owes_the_review` (unread-listed, no `unread`, skipped pass, budget stop, open loop), `a_record_whose_window_does_not_hold_the_commit_credits_nothing` and `a_record_whose_diff_base_is_the_commit_credits_nothing` pass

## Observations

- a credited path the window deleted is not listed in `excluded`, though AC13 says a credited path is listed in `excluded`: it stays in `deleted` with `review-owed: false` — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **waived**: A deletion has nothing to read and was never listed in `excluded` under 079 either; the gate's answer (no review owed) is the one AC13 asks for, so only the sentence's reach is loose. Observation about the pipeline's machinery, not a break of what the criterion protects. (decided by the rule)
- the `unread` bullet of `framework/reference/review.md` (and its installed copy `.ductus/reference/review.md`), the Unreleased entry of `runtime/CHANGELOG.md` and `plan.md` still said a credited path is excluded under the reading spec or the claimants' names, which 079's scenario `the-excluded-section-lists-paths-only` and AC13 no longer say — `framework/reference/review.md` — **fixed** (decided by the rule)
- AC6 said `runtime.yml` does not run for a push confined to the project's artifacts, but its `paths:` filters list `specs/066-audit-records-and-the-completion-gate/data-model.md`, which a test in `runtime/src/primitives/write_analysis.rs` reads (spec 066 task 26, after 081's plan); the Motivation's 'no Rust test read the checkout's artifacts' was likewise a claim about `1cc1dd52` only — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **fixed** (decided by the rule)
- 079's `AC6` sentence on a commit belonging to no spec is not annotated with 081's read credit of 2026-11-13 (AC13), though 081's AC5 asks only for the 2026-11-11 changes — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: 081's AC5 is met as written; the fix lies in 079's criteria, a spec this run must not reopen, and is reported to the parent. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/plan.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/plan.md`
- `runtime/src/main.rs`
- `runtime/src/primitives/check_step_references.rs`
- `runtime/src/primitives/tool_loading.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/tests/check_step_references_cli.rs`
- `scripts/audit/README.md`
- `scripts/audit/step-reference-integrity.sh`
- `scripts/mutation/step-references-bounded.py`
- `scripts/mutation/step-references-reach.py`
- `scripts/tests/test-step-reference-integrity.sh`
- `specs/020-code-review/data-model.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/041-task-pruning/tasks.md`
- `specs/051-branch-scoped-spec-numbering/tasks.md`
- `specs/057-analyze-artifact-and-record-relocation/tasks.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-bare-port-span-names-its-remedy.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/071-command-reference-split/scenarios/reference-sections-are-reached-by-a-step.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`

## Unexamined governance

*None.*
