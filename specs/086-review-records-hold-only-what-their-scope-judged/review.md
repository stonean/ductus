---
spec: 086-review-records-hold-only-what-their-scope-judged
last-run: 2026-11-27T16:41:48Z
reviewed-against: f5a499f6e8430770ad2bc37ebc9ee17d0bffc4ec
diff-base: 2ef8d85c21ae2975ee2449e846da56bfe458fce5
must-violations: 0
should-violations: 0
low-confidence: 2
examined: 37
scope: 59
unread:
  - runtime/CHANGELOG.md
  - runtime/src/schema/primitives.rs
  - runtime/src/primitives/write_review.rs
  - runtime/src/primitives/mod.rs
  - runtime/src/primitives/compute_review_scope.rs
  - runtime/src/interpreter/mod.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/main.rs
  - scripts/mutation/scope-attribution.py
  - runtime/tests/mcp.rs
  - .claude/commands/ductus/configure.md
  - .claude/commands/ductus/help.md
  - .claude/commands/ductus/prune.md
  - .claude/commands/ductus/review.md
  - .pi/prompts/ductus-help.md
  - .pi/prompts/ductus-prune.md
  - .pi/prompts/ductus-review.md
  - .ductus/reference/prune.md
  - .ductus/reference/review.md
  - runtime/src/mcp/server.rs
  - runtime/tests/fixtures/review-basic/specs/007-review/spec.md
  - specs/084-reopen-for-owed-code-review-every-code-change/tasks.md
excluded: 62
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
criteria-judged: 6
criteria-met: 6
criteria-verdicts:
  - label: AC1
    digest: 81b1cd952b6fdde480b1ef24c1699737eda8dad28feeb5c64d3dac151772ee15
  - label: AC2
    digest: 4d094aa1b05536b95974cb3d49d8f490e7b49b5625ee4d97707e60186143c463
  - label: AC3
    digest: 5983772b166cd1a4dfc4c61e6848e36f9f9a00314e872e8a42e6a5cbcb91d7dc
  - label: AC4
    digest: 46473273a3ec0b8254c6a5e2c045fa5172d4fdac49eb6efe68bf3f3d4f7989b8
  - label: AC5
    digest: 3b4ab8bce0da1a2e4dec029c852b6dfa46dd738c6e30e5d8e44b832cd7f12646
  - label: AC6
    digest: 2a5b0aade14c668259c6b6f87e7c081f072fa860cd557564611dc699437f3c86
blocking: false
dispositions:
  fixed: 11
  deferred: 0
  waived: 3
  undispositioned: 0
decisions:
  - key: "perf: `prune-review` walks `HEAD`'s whole history for every record it examines, even one with no decision or waiver anchored to a file, though `LazyOwnership` walks only when an entry reaches the ownership test — `runtime/src/primitives/prune_review.rs`"
    outcome: waived
    reason: "a stated design choice: 086's Cost section times the walk, and the result reports `ownership` on every record it examines, which a lazy walk would leave absent"
    decided-at: 2026-11-27T16:37:34Z
    decided-by: rule
  - key: "bug: a `fixed` observation whose chore fix deletes or renames the file it anchors makes `write-review` refuse the whole write, since a deleted file leaves the scope — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: AC1 requires refusing an outcome on a file the scope does not hold; such an observation is recorded with no path, which `write-review` accepts, so nothing is lost
    decided-at: 2026-11-27T16:37:34Z
    decided-by: rule
  - key: "other: `RunScope::holds` accepts a file listed in `excluded` when a plan pattern in the scope matches it, while the same file named exactly by the plan is refused, so one record can hold a finding on a path it also lists as excluded — `runtime/src/primitives/compute_review_scope.rs`"
    outcome: waived
    reason: "086's own rule: a file belongs to a spec when its plan names it by the match `compute-review-scope` applies to a plan entry (spec Behavior; plan, Rejected: membership by literal scope entry in the refusal)"
    decided-at: 2026-11-27T16:37:34Z
    decided-by: rule
---

# Review — 086-review-records-hold-only-what-their-scope-judged

## Summary

086's first review: diff base `2ef8d85c` from its reopen (`base-source: reopen`, not overridden), head `f5a499f6`, ownership examined, so the scope's 57 entries hold the plan's Affected Files and every file a commit attributed to 086 changed. Four parts ran all five passes over the scope, split by behavior, each reading the five loaded rule files whole (231,962 bytes); a fifth part judged the six criteria, all met. The passes read 37 of those 57 entries whole. The record's scope holds 59: this run's fixes to `runtime/tests/fixtures/review-basic/specs/007-review/spec.md` and `specs/084-reopen-for-owed-code-review-every-code-change/tasks.md`, outside the 57, joined it as uncommitted work, and both are unread, the passes having read only the lines the fixes changed. Unread besides: ten files read in regions, the lines 086's commits wrote at `HEAD` by `git blame` with 12 lines of context (`runtime/CHANGELOG.md`, `runtime/src/schema/primitives.rs`, `runtime/src/primitives/write_review.rs`, `runtime/src/primitives/mod.rs`, `runtime/src/primitives/compute_review_scope.rs`, `runtime/src/interpreter/mod.rs`, `runtime/src/schema/descriptions.rs`, `runtime/src/main.rs`, `scripts/mutation/scope-attribution.py`, `runtime/tests/mcp.rs`); nine generated copies, generated from sources read whole, with `scripts/gen-claude-commands.sh` re-run and in sync (`.claude/commands/ductus/` `configure.md`, `help.md`, `prune.md` and `review.md`, `.pi/prompts/` `ductus-help.md`, `ductus-prune.md` and `ductus-review.md`, `.ductus/reference/` `prune.md` and `review.md`); and `runtime/src/mcp/server.rs`, whose lines 881 to 1179 were viewed with boilerplate lines filtered out. Fixed in the run: three QUAL-TEST-001 SHOULDs, tests that passed with their behavior removed (the `dropped` list in `process-waivers`' and `process-decisions`' file form, and `Pass::place`'s guard on an unreadable scope), each now failing on a mutant (`waivers-bounded` M18 and M19, `process-decisions-results-bounded` M8 and M9, `review-drops-unowned-086` M12, all killed); and a fourth, the departure test whose tail could not fail, cut to its refusal and renamed `the_rule_firing_in_a_covered_file_outside_the_scope_is_refused` (`outcomes-outside-the-scope-086` M8 killed). Eleven observations were fixed and three waived by the rule. Over the fixed tree `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (4,087 passed) and every mutation list's anchor check passed, and the four parts re-ran their passes over the fix diff and found nothing new.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

2 of 38 commits in the window no spec claimed.

6 of 6 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-CLAIM-001 — a first review's recorded scope carries no sign that the owned files were not examined

- **File**: `runtime/src/primitives/write_review.rs:922-977`
- **Site**: resolve_scope
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish *examined the subject and found nothing* from *could not examine the subject*. When a code path skips part of its subject, cannot reach it, or has no basis to inspect it, its output SHOULD say so — a distinct return variant, an accompanying status or guidance field, or a message naming what was not examined — not a bare zero, empty collection, or success string.
- **Finding**: On a first review in a history that cannot be read whole, `compute-review-scope` leaves out every file the spec's commits changed and reports `ownership: not-examined`, but `write-review` does not carry that state, so the record's `scope` and `examined` read as a complete denominator; only `compute-review-scope`'s own result and the host's Summary, which the review reference tells it to write, say so. Low confidence: 086's plan (Not examined) chose to report it on `compute-review-scope`'s result alone.
- **Auto-fixable**: no

### LOW-CONFIDENCE: QUAL-TEST-001 — the project-record half of AC3's fixture asserts an absence no code path could break

- **File**: `runtime/src/primitives/write_review.rs:4004-4060`
- **Site**: tests::dropped_entries_leave_the_record_and_the_project_record_gains_none
- **Rule**: A test SHOULD fail when the behavior it names is removed or broken. A test that passes whether or not the code under test behaves as named SHOULD NOT be added or counted as coverage.
- **Finding**: The test ends by asserting that `specs/review.md` does not exist; a feature-subject `write-review` never writes the project record, so that assertion holds with or without 086, and the test's name claims it. Low confidence: its dropped and retained assertions do fail with the behavior removed, and the absence assertion guards a future write.
- **Auto-fixable**: no

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: `refuse_outcomes_outside_the_scope` in `runtime/src/primitives/write_review.rs`, called for a feature record, the project record and an empty diff base, refuses before any write a finding's file or an observation's path (its `:line` stripped, an empty path exempt) that `RunScope::holds` rejects, naming the argument and the file; tests `an_outcome_on_a_file_outside_the_scope_is_refused_writing_nothing`, `an_empty_diff_base_refuses_an_anchored_outcome_naming_the_base_to_pass` and `a_project_review_refuses_an_outcome_outside_its_scope`; `outcomes-outside-the-scope-086` 9 of 9 killed
- AC2 — met: `Classified::place_unread` in `runtime/src/primitives/process_decisions.rs` and `Pass::place` in `runtime/src/primitives/process_waivers.rs` drop an unfired entry on a file on disk outside the scope when `LazyOwnership::belongs` is false, with its own notice and `dropped-count`, and retain it when it belongs or ownership is not examined (a shallow cut); tests `a_silent_decision_on_a_file_that_does_not_belong_is_dropped_and_named`, `a_waiver_on_a_file_that_does_not_belong_is_dropped_and_named`, the two shallow-history tests, the restricted and `since` tests, and `a_file_a_plan_entry_names_belongs`; `review-drops-unowned-086` 11 of 11 and `spec-ownership-086` 9 of 9 killed
- AC3 — met: the same classifiers drop a decision and a waiver on code no spec claims, and `dropped_entries_leave_the_record_and_the_project_record_gains_none` passes them as expired and the record loses them; `Record::edits` in `runtime/src/primitives/prune_review.rs` drops them between reviews (`a_preview_names_each_entry_on_a_file_that_does_not_belong_and_writes_nothing`); prune's steps 4 and 10 and review's steps 8, 9 and 11 wire them; no feature run writes `specs/review.md`, so that half holds by construction
- AC4 — met: `RunScope::why_unread` in `runtime/src/primitives/compute_review_scope.rs`, through `still_on_disk`, gives no reason for a file gone from disk, so `process-decisions` expires its decision before any ownership test; test `a_silent_decision_on_a_deleted_file_expires_and_is_not_dropped`, and `prune-review` leaves it; `review-drops-unowned-086` M5 and `prune-review-086` M2 killed
- AC5 — met: `Record::edits`, `splice_frontmatter` and `rewrite_excluded` in `runtime/src/primitives/prune_review.rs` drop each decision and waiver anchor on a file on disk that does not belong, name each in `drops`, and rewrite each claimant line to its bare path, the re-parse comparing every other field and both lists; tests `an_apply_drops_them_and_rewrites_each_claimant_line_leaving_every_other_byte`, `a_record_with_nothing_to_drop_or_rewrite_is_not_written` and `only_a_line_naming_claimant_specs_under_its_heading_is_rewritten`; `prune-review-086` 69 of 69 killed
- AC6 — met: `first_review_reads` in `runtime/src/primitives/compute_review_scope.rs` adds, when the record at `HEAD` names no run, each plan entry and each existing file in the project that `spec_ownership`'s owned set names, and reports `ownership` with `ownership-reason`, `not-examined` on a cut history with the plan entries alone; tests `a_first_review_reads_each_file_the_specs_commits_changed` and `a_cut_history_gives_a_first_review_its_plan_alone_and_says_so`; `first-review-scope-086` 8 of 8 killed

## Observations

- perf: `prune-review`, `process-waivers` and `process-decisions`, which 086 lets walk the whole history, were served on the async handler rather than through `dispatch_blocking`, as every other full-history walk is — `runtime/src/mcp/server.rs` — **fixed** (decided by the rule)
- convention: `PruneReviewResult`'s `form` doc named `rewrites` among the lists moved to the file, though rewrites are a count only (`rewrites-count`) — `runtime/src/schema/primitives.rs` — **fixed** (decided by the rule)
- convention: the vacuity guard `the_parts_and_move_modes_each_grow_their_result_past_the_plain_call` covered no `Prep::Unowned` row, so a `write_unowned` that built nothing would measure an empty list as bounded — `runtime/tests/result_sizes.rs` — **fixed** (decided by the rule)
- convention: the doc comment of `prune_command_gate_blocks_on_the_exec_path` said the prune command parses as steps 1 to 10 with gate-confirm at step 6, where its body asserts 1 to 12 and step 7 — `runtime/tests/walker.rs` — **fixed** (decided by the rule)
- other: prune's step 6 exit for nothing to prune excepted a result carrying `unexamined-tail` but not a `prune-review` record whose ownership was not examined, so its stop line could read as clean over records not examined — `framework/commands/prune.md` — **fixed** (decided by the rule)
- convention: the `/prune` entry said its `review.md` drops are the ones the spec's next review would drop, which holds only for an unrestricted review — `docs/slash-commands.md` — **fixed** (decided by the rule)
- convention: doc comments still gave the waiver classifier four lists after 086 made them five (`Classified`, the bounded-lists test, `Pass::place`, and the header of `scripts/mutation/waivers-bounded.py`) — `runtime/src/primitives/process_waivers.rs` — **fixed** (decided by the rule)
- convention: `Classified::place_unread`'s doc said a restricted run reaches it, though a restricted run retains earlier as not evaluated, so only a `since` run does — `runtime/src/primitives/process_decisions.rs` — **fixed** (decided by the rule)
- convention: the Waivers section said `process-waivers` applies, expires and retains, omitting the drop, and the Deferrals section said a deferral retires only when its finding stops firing or its work is complete — `framework/reference/review.md` — **fixed** (decided by the rule)
- convention: the `review-basic` fixture's `spec.md` said its pipeline runs against a single-commit repo, though 086's parity harness stages the harness and the spec directory in separate commits — `runtime/tests/fixtures/review-basic/specs/007-review/spec.md` — **fixed** (decided by the rule)
- convention: a task named `framework/commands/prune.md` step 9 for the step that decides on `check-owed-work`, which 086's prune changes renumbered to step 11 — `specs/084-reopen-for-owed-code-review-every-code-change/tasks.md` — **fixed** (decided by the rule)
- perf: `prune-review` walks `HEAD`'s whole history for every record it examines, even one with no decision or waiver anchored to a file, though `LazyOwnership` walks only when an entry reaches the ownership test — `runtime/src/primitives/prune_review.rs` — **waived**: a stated design choice: 086's Cost section times the walk, and the result reports `ownership` on every record it examines, which a lazy walk would leave absent (decided by the rule)
- bug: a `fixed` observation whose chore fix deletes or renames the file it anchors makes `write-review` refuse the whole write, since a deleted file leaves the scope — `runtime/src/primitives/write_review.rs` — **waived**: AC1 requires refusing an outcome on a file the scope does not hold; such an observation is recorded with no path, which `write-review` accepts, so nothing is lost (decided by the rule)
- other: `RunScope::holds` accepts a file listed in `excluded` when a plan pattern in the scope matches it, while the same file named exactly by the plan is refused, so one record can hold a finding on a path it also lists as excluded — `runtime/src/primitives/compute_review_scope.rs` — **waived**: 086's own rule: a file belongs to a spec when its plan names it by the match `compute-review-scope` applies to a plan entry (spec Behavior; plan, Rejected: membership by literal scope entry in the refusal) (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.ductus/reference/analyze.md`
- `.githooks/pre-commit`
- `.github/workflows/framework-checks.yml`
- `framework/constitution.md`
- `framework/reference/analyze.md`
- `runtime/src/primitives/backfill_section_digest.rs`
- `runtime/src/primitives/check_rule_ids.rs`
- `runtime/src/primitives/rule_verdicts.rs`
- `scripts/analyze/analyze_lib.py`
- `scripts/analyze/briefs.py`
- `scripts/analyze/check.py`
- `scripts/analyze/combine.py`
- `scripts/analyze/record.py`
- `scripts/analyze/scenario-brief.md`
- `scripts/analyze/split.py`
- `scripts/lint-governing-size.sh`
- `scripts/mutation/rule-judgments-088.py`
- `scripts/mutation/rule-verdicts-converge.py`
- `scripts/tests/test-analyze-scripts.py`
- `scripts/tests/test-lint-governing-size.sh`
- `specs/020-code-review/scenarios/waiver-expiry.md`
- `specs/020-code-review/spec.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/scenarios/a-confirmed-gate-authorizes-the-writes-after-it.md`
- `specs/022-deterministic-runtime/scenarios/parser-walker-conventions.md`
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/plan.md`
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`
- `specs/041-task-pruning/spec.md`
- `specs/050-constitution/plan.md`
- `specs/050-constitution/scenarios/a-decision-is-attributed-to-whoever-made-it.md`
- `specs/050-constitution/scenarios/a-primitive-is-probed-in-a-local-clone.md`
- `specs/050-constitution/scenarios/findings-route-by-scope.md`
- `specs/050-constitution/spec.md`
- `specs/058-findings-route-at-discovery/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/rule-judging-runs-from-repo-scripts.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/spec.md`
- `specs/060-exec-analyze-assesses-each-loaded-rule/tasks.md`
- `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-base-includes-the-transition-commit.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/072-governing-documents-stay-within-one-read/spec.md`
- `specs/073-decided-rule-departures-are-recorded/review.md`
- `specs/075-agents-md-holds-what-every-session-needs/plan.md`
- `specs/075-agents-md-holds-what-every-session-needs/spec.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/plan.md`
- `specs/084-reopen-for-owed-code-review-every-code-change/spec.md`
- `specs/088-rule-judgments-cost-only-what-changed/analysis.md`
- `specs/088-rule-judgments-cost-only-what-changed/data-model.md`
- `specs/088-rule-judgments-cost-only-what-changed/plan.md`
- `specs/088-rule-judgments-cost-only-what-changed/review.md`
- `specs/088-rule-judgments-cost-only-what-changed/spec.md`
- `specs/088-rule-judgments-cost-only-what-changed/tasks.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/plan.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/spec.md`
- `specs/089-the-pipeline-s-own-cost-is-stated-and-bounded/tasks.md`

## Unexamined governance

*None.*
