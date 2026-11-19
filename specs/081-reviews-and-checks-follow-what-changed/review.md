---
spec: 081-reviews-and-checks-follow-what-changed
last-run: 2026-11-19T03:06:07Z
reviewed-against: 5979ce6e660387735d1d6e812cd27d0d1ef47e12
diff-base: 24d3a8f4ca50e3c6d0e483bce82dbaafa157a171
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 1
scope: 1
unread: []
excluded: 107
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 0
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
    digest: e9a1bf6aaf385d0877478412e14a6c0a12de870b5359457b5475bf0ea7ad19f7
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
    digest: ebadb6be65b05214a72287b6df665f467935f11acc15eb85c2aa348742e13534
  - label: AC14
    digest: 279e204a95da01794f7703ac2b5cbe8dd9b446c5b6b55633898e5796108eb1b1
blocking: false
dispositions:
  fixed: 2
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
---

# Review — 081-reviews-and-checks-follow-what-changed

## Summary

Window 24d3a8f4 (base-source reopen, 066 task 27b) to 5979ce6e. The scope holds one artifact, spec.md (19,818 bytes), read whole by the passes, so 1 of 1 examined and unread is empty. The nine backend and cross rule files were read whole (285,795 bytes) and all five passes ran: no finding. The criteria step judged all 14 criteria one by one against the tree and history, with the proving unit tests run (compute_review_scope 134, check_review_gate 96, write_review 156 and merge_permissions 48 passed) and merge-permissions driven on a fixture: all 14 met. Two observations were fixed by rule: the Behavior bullet on a commit naming no spec now names AC13's credit, and the artifact-set Resolved Question states its method, which re-derives 664, 425, 437 and 444 commits at 6158d7df, each 2 below the figures first recorded, with the difference of 12 unchanged and all 12 confirmed AGENTS.md-only. One stored decision expired, its key naming no file; nine are retained, each keyed to a file outside the scope. Unexamined: the Motivation's and Resolved Questions' other dated measurements (the 12-spec census, CI durations, the 58, 7, 37 and 16 commit counts, the 49-reopening and 8-of-61 figures), which were not re-derived.

1 fix round(s): 0 re-read, 0 diff-checked, 1 exempt, 0 not re-read. Exempt: round 1.

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

- AC1 — met: `compute_review_scope` tests `a_reopening_that_changed_code_owes_a_review_and_a_contract_edit_does_not` (scenarios/a.md and data-model.md give review_owed false, code true) and `an_artifact_only_window_owes_a_review_while_a_criterion_is_unjudged` (066 task 27 clause) pass in `cargo test --lib compute_review_scope` (134 passed)
- AC2 — met: `check_review_gate` test `a_contract_change_since_the_review_holds_done_only_on_the_analysis` gives blocked_by AnalyzeStale for a scenario edit and ReviewOwed naming `src/code.rs` for a code change; no `review is stale` string remains in runtime/src or framework; 96 check_review_gate tests pass
- AC3 — met: `ductus check-review-freshness` is an unrecognized subcommand; `scripts/audit/run-all.sh` has no Family 19 and `framework/runtime-tools.txt` no entry; `RETIRED_ALLOW` in `merge_permissions.rs` lists it and the CLI with --canonical on a fixture settings file reported revoked [mcp__ductus__check-review-freshness] and removed it
- AC4 — met: `framework/constitution.md` blast-radius rule says review reads code, analyze the artifacts, and a change confined to artifacts owes their checks and no build/test/lint/CI; `framework/reference/review.md` says a changed contract owes an analysis (reviewed-digest decides no staleness) and no longer says a scenario or data-model edit stales a review
- AC5 — met: 079 spec.md: Resolved Questions on scenario edits and on attribution each end with a Spec 081 (owner, 2026-11-11) reversal/narrowing; AC4 annotation `Superseded in part (spec 081): a stale digest owes no review`; AC6 annotation names the named-commit rule
- AC6 — met: `.github/workflows/generators.yml` and `framework-checks.yml` key `runtime-cache` on Cargo.lock, Cargo.toml, rust-toolchain.toml and `runtime/src/**`, touch sources back on a hit and print `No build of these runtime sources is cached, so this run compiles one.` on a miss; gh runs 43110200293 and 43110200122 on aec09d69 (spec-only) show Reuse step success and Build the runtime 2 s and 1 s; `runtime.yml` push paths exclude `specs/`; `AGENTS.md` local-checks rule says no build, no cargo, no code workflow for artifacts
- AC7 — met: Commits 33688944, 401e474b and 58878cfb land every behavior change in `runtime/src`, `framework/constitution.md`, `framework/reference/`, `framework/commands/` and `framework/bootstrap/configure/claude.md`; the repo-only files they touch are the CI cache, `scripts/` mutants and `AGENTS.md`, which carry no pipeline behavior
- AC8 — met: Tests `an_artifact_outside_the_spec_root_owes_no_review_and_code_does` (.ductus/, AGENTS.md, CLAUDE.md no code; docs/, sub/AGENTS.md, README.md code), `a_configured_spec_root_is_the_artifact_root` and `a_subdirectory_project_reads_its_artifacts_from_the_project_root` pass; 066 task 27 clause proven by `an_artifact_only_window_owes_a_review_while_a_criterion_is_unjudged`
- AC9 — met: Tests `a_named_commits_code_goes_to_every_spec_it_names` and `a_spec_claiming_a_named_commit_only_by_its_directory_reviews_only_that_directory` pass: `src/code.rs` of `fix(002)` is in 002's scope and excluded for 001 even though 001's plan lists it, while 001 keeps its own directory's `notes.md`
- AC10 — met: Test `a_code_commit_naming_no_spec_owes_the_review_of_each_window_holding_it` (feat: touching only `src/unnamed.rs` stays in scope and review_owed true) and the credit tests `a_path_a_complete_record_read_is_credited_and_owes_no_review` pass; `a_subject_naming_none_leaves_each_claimants_directory_to_it` keeps 079 attribution for unnamed multi-spec commits
- AC11 — met: `ductus compute-review-scope --help` (descriptions.rs) states the named-commit rule, the credit rule, the artifact set (spec root, .ductus/, AGENTS.md, CLAUDE.md) and review-owed; `framework/reference/review.md` Window and coverage bullets state the attribution, the unread credit and the artifact set
- AC12 — met: `write_review` tests `unread_is_written_whenever_the_run_states_it` (None/[]/entry) and `an_unread_entry_outside_the_scope_is_refused` pass; `framework/commands/review.md` step 11 passes `unread`; `framework/constitution.md` review-record table, `specs/066-audit-records-and-the-completion-gate/data-model.md` and `specs/020-code-review/data-model.md` list `unread`
- AC13 — met: Tests `a_path_a_complete_record_read_is_credited_and_owes_no_review` (leaves scope, `excluded` names 002-y, no review owed), `a_record_credits_each_path_it_did_not_name_unread`, `a_record_whose_diff_base_is_the_commit_credits_nothing`, `a_later_change_no_record_read_keeps_the_path` pass
- AC14 — met: `a_path_no_complete_record_read_stays_and_owes_the_review` loops unread-listed, no `unread`, skipped-passes, budget-stop and fix-rounds-open records, each keeping the path in scope with review_owed; `a_record_whose_window_does_not_hold_the_commit_credits_nothing` covers the out-of-window case; all pass

## Observations

- convention — Behavior's bullet on a commit naming no spec says it is attributed as 079 attributes it, unchanged, and stays in every window holding it, though the spec's own AC10 and AC13 add the credit that ends that once a committed review read its paths — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **fixed** (decided by the rule)
- convention — the Resolved Question on the project's artifacts records 666, 427, 439 and 446 commits at `6158d7df` with no method; the method now stated re-derives 664, 425, 437 and 444, each 2 lower, with the difference of 12 unchanged — `specs/081-reviews-and-checks-follow-what-changed/spec.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/implement.md` — 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/review.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/audit.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `.ductus/reference/implement.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/review.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `.github/workflows/runtime-acquisition.yml` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime-release.yml` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime.yml` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `.pi/prompts/ductus-amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-implement.md` — 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-review.md` — 066-audit-records-and-the-completion-gate
- `AGENTS.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `README.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `docs/developing-ductus.md` — 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate
- `docs/runtime.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `docs/slash-commands.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/bootstrap/ductus-procedure.md` — 027-bootstrap-migration-registry
- `framework/bootstrap/ductus.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `framework/bootstrap/govern.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `framework/commands/amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `framework/commands/implement.md` — 066-audit-records-and-the-completion-gate
- `framework/commands/review.md` — 066-audit-records-and-the-completion-gate
- `framework/constitution.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/migrations.toml` — 027-bootstrap-migration-registry
- `framework/reference/audit.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/reference/implement.md` — 066-audit-records-and-the-completion-gate
- `framework/reference/review.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `runtime/CHANGELOG.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 073-decided-rule-departures-are-recorded
- `runtime/README.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `runtime/src/interpreter/mod.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/interpreter/payload.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/check_review_gate.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `runtime/src/primitives/compute_review_scope.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `runtime/src/primitives/criterion_verdicts.rs` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `runtime/src/primitives/derive_routing_candidates.rs` — 079-review-and-analysis-work-converges
- `runtime/src/primitives/line_citations.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/write_review.rs` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `runtime/src/schema/descriptions.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `runtime/src/schema/extensions.rs` — 022-deterministic-runtime, 048-govern-acquired-runtime, 066-audit-records-and-the-completion-gate
- `runtime/src/schema/primitives.rs` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `runtime/tests/fixtures/implement-basic/.govern.session.toml` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/implement-basic/stdin.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/review-basic/stdin.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/implement-basic.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/review-basic.jsonl` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `runtime/tests/mcp.rs` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `runtime/tests/review_command.rs` — 066-audit-records-and-the-completion-gate
- `scripts/audit/README.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/audit/done-spec-criteria.sh` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/audit/migration-coverage.sh` — 027-bootstrap-migration-registry
- `scripts/mutation/criteria-in-review-066.py` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `scripts/mutation/fix-round-record.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/line-citation-strip-066.py` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/mutation/reopen-measure.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/reviews-follow-what-changed.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/scope-attribution.py` — 066-audit-records-and-the-completion-gate
- `scripts/tests/test-migration-coverage.sh` — 027-bootstrap-migration-registry
- `specs/000-slash-commands/analysis.md` — 000-slash-commands
- `specs/000-slash-commands/plan.md` — 000-slash-commands
- `specs/000-slash-commands/review.md` — 000-slash-commands
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/000-slash-commands/scenarios/validation-gates.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/000-slash-commands/spec.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/020-code-review/analysis.md` — 020-code-review
- `specs/020-code-review/data-model.md` — 020-code-review, 022-deterministic-runtime
- `specs/020-code-review/plan.md` — 020-code-review
- `specs/020-code-review/review.md` — 020-code-review, 022-deterministic-runtime
- `specs/020-code-review/spec.md` — 020-code-review, 022-deterministic-runtime
- `specs/022-deterministic-runtime/data-model.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/022-deterministic-runtime/scenarios/implement-completion-gate.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md` — 020-code-review, 022-deterministic-runtime
- `specs/022-deterministic-runtime/spec.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/027-bootstrap-migration-registry/analysis.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/plan.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/review.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/a-projects-own-constitution-is-not-relocated.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/migration-chain-reference-integrity.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/migrations-apply-once-per-repo.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/spec.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/tasks.md` — 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/analysis.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/plan.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/review.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/spec.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire
- `specs/048-govern-acquired-runtime/data-model.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/plan.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/state-b-continues-in-session.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/spec.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/tasks.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/analysis.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/review.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges
- `specs/073-decided-rule-departures-are-recorded/plan.md` — 073-decided-rule-departures-are-recorded
- `specs/079-review-and-analysis-work-converges/analysis.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/plan.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/review.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/spec.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges

## Unexamined governance

*None.*
