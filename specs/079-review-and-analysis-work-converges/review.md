---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-09T14:57:37Z
reviewed-against: bed3c01658952a9e28c2807a4839ffc7aa54b957
diff-base: 347c3f70fb430ea625b61f49a9dca9680a18b590
must-violations: 0
should-violations: 0
low-confidence: 2
examined: 58
scope: 84
excluded: 220
contract-breaks: 0
skipped-passes: []
fix-rounds: 4
re-reads: 3
diff-checked: 1
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 4b3a9af90b0d67323bbb7ec3838715fed768dfc8a365667fe96359b9bd21027c
blocking: false
dispositions:
  fixed: 26
  deferred: 0
  waived: 19
  undispositioned: 0
decisions:
  - key: "bug: a first review that stopped early still drops unchanged plan entries from the next scope, so what its skipped pass never read stays unread, against the module doc's 'stopping early leaves nothing permanently unread'. — `runtime/src/primitives/compute_review_scope.rs:201`"
    outcome: waived
    reason: AC6 keeps an unchanged plan entry only on the first review, and that run was the first; what its skipped pass did not read is named in the record's skipped-passes, and restricting a run is the operator's choice.
    decided-at: 2026-11-05T21:43:57Z
    decided-by: rule
  - key: "perf: check-prescriptions counts an exempt path as unprescribed, so a round that applied exactly the prescribed edits and also touched spec.md, plan.md or tasks.md differs and costs another re-read of paths the check had just matched. — `runtime/src/primitives/check_prescriptions.rs:82`"
    outcome: waived
    reason: "No criterion covers it: AC13 covers the brief, and this costs one re-read on a rare round that also edits the spec's own text."
    decided-at: 2026-11-05T21:43:57Z
    decided-by: rule
  - key: "bug: a spec directory renamed while in-progress counts a record carried from before the rename as a run of this reopening, which can report a false over-bound. — `runtime/src/primitives/measure_reopenings.rs:708`"
    outcome: waived
    reason: A rename mid-reopening is rare, it over-reports rather than hides, no gate reads the count, and the module doc states that a renamed spec's history starts at the rename.
    decided-at: 2026-11-05T21:43:57Z
    decided-by: rule
  - key: "other: cross_service.rs's second assertion and tests/parity/cross-service/expected.txt are now a copy of the golden whose only effect is requiring a re-bless in two places, as the file's own doc says. — `runtime/tests/cross_service.rs:60`"
    outcome: waived
    reason: plan.md's Affected Files keeps this assertion on purpose, and 030's plan and tasks name the capture, so removing it is a design choice no criterion here asks for.
    decided-at: 2026-11-05T21:43:57Z
    decided-by: rule
  - key: "bug: /amend's reopen check runs classify-spec-edit only on a delta that excludes a modified scenario and a data model, the two edits §spec-lifecycle says always reopen, and its line then says no scenario edit exists. — `framework/commands/amend.md:58`"
    outcome: waived
    reason: The delta predates this spec, AC15 governs what classify-spec-edit returns and not which edits reach it, and widening the delta changes what the reconcile pass offers, which is a requirement of its own.
    decided-at: 2026-11-05T21:43:57Z
    decided-by: rule
  - key: step 8 names no family — `framework/commands/analyze.md`
    outcome: waived
    reason: It points at the description, as AC9 asks.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: "`1. [ ]`, `> - [ ]`, `* [ ]` and `+ [ ]` are no boxes to `find_checkbox_line` — `runtime/src/primitives/mod.rs`"
    outcome: waived
    reason: One documented grammar, `mark-task`'s too, every surface alike, and 0 such lines here.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: a claim carried only in a reference's label reads `mechanical` — `runtime/src/primitives/derive_references.rs`
    outcome: waived
    reason: The scrub removes label and target by its stated design, and `plan.md`'s known limitations say so.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: a reference added as an ordered item or a table row reads `reopens` — `runtime/src/primitives/classify_spec_edit.rs`
    outcome: waived
    reason: It errs toward reopening.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: a reference-only edit reads `mechanical` while staleness reads stale — `runtime/src/primitives/classify_spec_edit.rs`
    outcome: waived
    reason: Exempting references from the digest is a change of its own, with 0 occurrences since no spec here carries `references:`.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: "`cross-spec-impact:` names neither 000 nor 035, though the round edited their scenarios and reopened 035 — `specs/079-review-and-analysis-work-converges/spec.md`"
    outcome: waived
    reason: Each is a correction of a claim, which owes no back-link and names this spec in its commit, as `plan.md`'s Cross-spec impact says; AC12's list is of specs whose surface this changes.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: "`an_open_break_names_its_file` refuses a blank-path break whose deferral releases it, though only a break no deferral releases must name its path — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: It fails closed, the host passes the path it has, and the condition predates this run's fixes.
    decided-at: 2026-11-06T21:57:41Z
    decided-by: rule
  - key: "bug: an empty diff-base records scope 0 without consulting compute-review-scope, so a spec never in-progress records a first review's examined beside scope 0 — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: The data model, write-review's description and the owner's task-20 choice (2026-11-07) define an empty diff-base as scope 0, and deriving it instead is a requirement of its own.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: write-review reports every compute-review-scope error as an invalid diff-base, an unreadable plan included — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: The message carries the underlying error, and telling a base's errors from the scope's is a change to the error contract no criterion asks for.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: process-decisions' expired notice does not mark the decisions naming a file the scope holds, which step 9 re-checks — `runtime/src/primitives/process_decisions.rs`"
    outcome: waived
    reason: The owner chose the host's re-check against the file (task 22, 2026-11-07), and marking those decisions in the notice is a requirement of its own.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "bug: deferring an observation to an existing open task exempts that task's earlier boxes from the open-box refusal — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: "No command reaches it: step 1 stops before a run's deferral exists, and ductus exec writes no deferral, so only a hand-driven write does."
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "convention: owed-run prose on a task whose boxes are all ticked is dropped by prune-tasks while the runs are owed, and on a task with no box it is never dropped — `runtime/src/primitives/prune_tasks.rs`"
    outcome: waived
    reason: What a spec owes is derived by check-review-gate, which the prose only names, so pruning it loses no gate, and where owed runs live is a requirement of its own.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "bug: 026's, 058's and 078's ordering constraints lost the box /implement and the gate read, so no step holds a review to them — `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md`"
    outcome: waived
    reason: "No gate orders one spec's runs after another's: the owner sets the closing order (task 12, 2026-11-09), and a gate for it is a requirement of its own."
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: an analysis whose own chore fix appends a task cannot record, since only a deferral's task is exempt — `runtime/src/primitives/write_analysis.rs`"
    outcome: waived
    reason: AC28's exception covers a deferral alone, as written, and a fix that appends a task is no chore.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: no test pins REOPENING_WINDOW_DAYS's 30 days, both default-window tests deriving the window from the constant — `runtime/src/primitives/measure_reopenings.rs`"
    outcome: waived
    reason: It is outside this scope and in another session's active file (spec 080, 01b97a87), and the constant is defined once and named in the data model.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: 041's prune scenario rests a plan section's removal on its changing no claim, the removed test — `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md`"
    outcome: waived
    reason: It records a decision taken under the old rule, and its conclusion holds, since §spec-lifecycle gives a plan.md edit no back-edge.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: Family 24's guidance says a repair in a scenario or data model reopens, while the same repair made across files reads as a sweep — `scripts/audit/rename-sweep-residue.sh`"
    outcome: waived
    reason: Family 24 reports each site on its own, and §spec-lifecycle says a typo fixed in one file reopens.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: 063's split Resolved Question calls a split a meaningful edit because it moves a requirement, which holds only for a criterion or scenario — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md`"
    outcome: waived
    reason: A split moves requirements, which are criteria or scenarios, so its conclusion holds.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: 063's plan justifies 058's step-number sync as mechanical because it changes no claim — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`"
    outcome: waived
    reason: It records a decision, and its conclusion holds under §spec-lifecycle's case (a).
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
  - key: "other: 050's plan says AC13 settles whether an edit that changes no claim is mechanical — `specs/050-constitution/plan.md`"
    outcome: waived
    reason: It states the question AC13 settled, which the plan's following sentences now answer with the default and classify-spec-edit's test.
    decided-at: 2026-11-09T14:57:37Z
    decided-by: rule
---

# Review — 079-review-and-analysis-work-converges

## Summary

Review of this spec's closing window from `347c3f70` (`base-source: last-review`, the commit holding its last review record), its passes read at `1879d9f7` by four independent read-only agents, each following one behavior across its files and driving it with the binary on fixtures: a run waiting for the task list (`29ec6cc2`, `46c449d5`, `55824d6d`'s task lists), what a record holds and when the gate holds (`33072ca1`, `101a6e8e`), whose change a commit is and where the window starts (`80f6012d`, `81285165`, `1abe8f97`), and this spec's artifacts, its data model checked whole, under the 12 rule files `discover-rule-files` selects. The five paths every waiting window holds were read in this session: `AGENTS.md` whole; the three runtime workflows by their window diffs and a script over each whole file, every `uses:` at a full SHA, the cargo tools at `--locked` versions and the zig wheel by hash (SEC-DEPS-001); and `runtime/Cargo.lock` by its window's changes, its 259 registry packages each checksummed and `cargo audit` clean. `scripts/check-runtime-currency.sh` found `syn` one release behind, so `bed3c016`, naming no spec, took `syn` 3.0.10 during the run and was read here; `01b97a87` is 080's alone and is excluded.

`examined` is 58 of the 75 paths in scope when the passes ran. The other 17 are generated copies, 14 under `.claude/commands/ductus/`, `.ductus/reference/plan.md` and two under `.pi/prompts/`, not opened: each is generated from a source read in full, and `gen-claude-commands.sh` run into a scratch copy matched every one. `write_review.rs`, `check_review_gate.rs` and `compute_review_scope.rs` were read by the window's hunks and the regions they reach, not whole. `scope` reads 84 because the fixes added 9 paths the passes read only in part or not at all (`framework/constitution.md`, `README.md`, `docs/slash-commands.md`, 050's `spec.md`, `plan.md` and scenario `a-declared-cross-spec-impact-gates-done`, `classify_spec_edit.rs`, `scripts/audit/rename-sweep-residue.sh` and the review-basic fixture's spec), which the fix rounds' re-reads read instead.

The passes found 3 SHOULD violations. One, `QUAL-TEST-001` on `empty_scope_does_not_suppress_the_derived_denominator`, which asserted nothing about the scope it is named for, is fixed before this record and so outside its counts; two are low-confidence, below. Of 45 observations, 11 of them the fix rounds' NOTEs, 26 are fixed, 5 of them broken contracts, 13 are waived by the rule with their reasons, and 6 are disposed of by stored decisions whose issues still stand. The broken contracts: `/analyze`'s step 1 stopped a `done` spec holding a box before `--fix` could revert it; both commands' step 1 tied the own-deferral exception to the writer rather than the stop; `without_tasks` matched a deferred task at a phase heading of a phased list; M25 was listed as an equivalent mutant though a committed test kills it; and §spec-lifecycle called a one-file typo fix in a scenario or data model mechanical, against `classify-spec-edit` and this spec's Edge Cases and Resolved Questions, the observation spec 023's review waived to this one (`3bbee201`), so 050's AC13 is annotated. Each new test has a mutant killed by id (M268, M269, M270, and M25 by its new filter), and the review-basic golden was re-blessed with its five `diff-base` values alone moving. `scope-attribution.py` then ran whole over the fixed tree, 266 of 266 killed with `EXPECTED_SURVIVORS` empty.

Four fix rounds from base `20694a47`, the snapshot `start-fix-rounds` took before any fix. Round 1, the 24 paths the batch changed and 3 exempt, was re-read by an independent agent from the runtime's brief alone and found 5 SHOULD and 6 NOTE; each SHOULD was fixed by the edit it prescribed, which round 2's `check-prescriptions` matched. Round 3, the 3 paths the NOTE fixes changed, found 1 SHOULD and 5 NOTE. That was the budget's second re-read with a finding, so the owner was asked, and chose another re-read (2026-11-09). Round 4, the prescribed fix and 1 exempt path, re-read clean, and the next brief was empty.

Not re-checked, for want of a run or a source: the figures other commits state (`33072ca1`'s 108 tests and its mutant kills, `29ec6cc2`'s M261 to M264 and its 313, 236 and 77 runs, the Resolved Question's projection of 0 of 30 reviews, tasks 22 to 25's counts), and `docs/developing-ductus.md`'s claim that a test binary built in a removed worktree is reused through `CARGO_MANIFEST_DIR`.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-TEST-001 — the MCP wire test's scope assertion never meets a derived scope

- **File**: `runtime/tests/mcp.rs:1363-1404`
- **Rule**: A test SHOULD fail when the behavior it names is removed or broken.
- **Finding**: 33072ca1 set its diff-base to an empty string in a directory with no repository, so the scope is the empty base's constant and the only assertion, is_number, holds whatever the derivation does; as a wire test it still catches a dropped field.
- **Auto-fixable**: no

### LOW-CONFIDENCE: QUAL-CLAIM-001 — a tasks.md that is not a regular file reads as none

- **File**: `runtime/src/primitives/read_tasks.rs:142-160`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish examined the subject and found nothing from could not examine the subject.
- **Finding**: refuse_while_tasks_are_open returns early unless tasks.md is a regular file, so a directory named tasks.md or a dangling link records as though nothing were owed; the gate uses the same is_file test, and no task list has that shape.
- **Auto-fixable**: no

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- bug: /analyze's step 1 stops on any open box, a done spec's too, so --fix never reaches the revert the analyze reference's Review state drift promises for a done spec holding one — `framework/commands/analyze.md` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: both commands' step 1 attach the own-deferral exception to the writer's refusal, not to the stop, so analyze's re-check after a deferral to the spec's own task halts, against AC28's Except — `framework/commands/review.md` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: without_tasks opens a deferred task's section at a numbered heading of any level, so in a phased list a phase numbered like the deferred task exempts the tasks under it, against AC28 — `runtime/src/primitives/read_tasks.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: M25's expected-survivor reason says hiding the transition's parent changes no result, but a root transition errors and a merge admits its side branch, and the committed a_root_transition_commit_falls_back_to_itself kills it — `scripts/mutation/scope-attribution.py` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention: §spec-lifecycle calls a one-file typo fix in a scenario or data model mechanical, while classify-spec-edit, which it names the decider, reads it as reopens, as this spec's Edge Cases and Resolved Questions decided — `framework/constitution.md` (breaks the spec's contract) — **fixed** (decided by the rule)
- convention: both commands' step 1 halt the run rather than this feature's, so under --all the first spec with an open box ends the walk — `framework/commands/analyze.md` — **fixed** (decided by the rule)
- convention: README and docs/slash-commands.md say /analyze runs at any time, false since a run waits for every box — `README.md` — **fixed** (decided by the rule)
- convention: plan.md's A run waits for the work omits the own-deferral exception the code, the data model and AC28 carry — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- convention: the Errors docs of write-review's and write-analysis's run omit the open-box refusal, and write-analysis's the tasks.md read error — `runtime/src/primitives/write_analysis.rs` — **fixed** (decided by the rule)
- other: no test holds write-analysis's half of the own-deferral exception, so a mutant passing it no deferral survives — `runtime/src/primitives/write_analysis.rs` — **fixed** (decided by the rule)
- other: the Resolved Question on when runs happen counted open boxes by a method that also counted the template's commented boxes, and its pattern search missed the three boxes task 26 found — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- convention: resolve_scope's doc and its test's doc say compute-review-scope gives an empty base a scope of zero, while a first review's scope still holds the plan's entries — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- convention: write-review's description reads as refusing an empty diff-base, which records scope 0 — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- convention: review-basic's fixture spec says write-review consumes the seeded diff-base, a seed 33072ca1 removed — `runtime/tests/fixtures/review-basic/specs/007-review/spec.md` — **fixed** (decided by the rule)
- convention: the changelog's byte figures for the reference cut name no commit or method and no longer hold at HEAD — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- convention: the review reference says a path in another spec's directory that a commit naming no spec changed leaves the scope, though the spec's own plan naming it keeps it, and its Excluded from scope line says only another spec's commits — `framework/reference/review.md` — **fixed** (decided by the rule)
- convention: reopen-measure.py's docstring says dashboard.rs renders the Reopenings line, which pipeline_view.rs renders since spec 080 — `scripts/mutation/reopen-measure.py` — **fixed** (decided by the rule)
- convention: classify-spec-edit's module doc and check-review-freshness's description say a sweep agrees with staleness, true only once the sweep is committed — `runtime/src/primitives/classify_spec_edit.rs` — **fixed** (decided by the rule)
- other: AC4's annotation dates all six holds to 5f67ede2, though the stale digest came in 6aef0472 and a finding left open in 68b643c8 — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- convention: the data model's blocked-by additions omit fix-round-loop-open, which task 3 added — `specs/079-review-and-analysis-work-converges/data-model.md` — **fixed** (decided by the rule)
- bug: an empty diff-base records scope 0 without consulting compute-review-scope, so a spec never in-progress records a first review's examined beside scope 0 — `runtime/src/primitives/write_review.rs` — **waived**: The data model, write-review's description and the owner's task-20 choice (2026-11-07) define an empty diff-base as scope 0, and deriving it instead is a requirement of its own. (decided by the rule)
- other: write-review reports every compute-review-scope error as an invalid diff-base, an unreadable plan included — `runtime/src/primitives/write_review.rs` — **waived**: The message carries the underlying error, and telling a base's errors from the scope's is a change to the error contract no criterion asks for. (decided by the rule)
- other: process-decisions' expired notice does not mark the decisions naming a file the scope holds, which step 9 re-checks — `runtime/src/primitives/process_decisions.rs` — **waived**: The owner chose the host's re-check against the file (task 22, 2026-11-07), and marking those decisions in the notice is a requirement of its own. (decided by the rule)
- bug: deferring an observation to an existing open task exempts that task's earlier boxes from the open-box refusal — `runtime/src/primitives/decisions.rs` — **waived**: No command reaches it: step 1 stops before a run's deferral exists, and ductus exec writes no deferral, so only a hand-driven write does. (decided by the rule)
- convention: owed-run prose on a task whose boxes are all ticked is dropped by prune-tasks while the runs are owed, and on a task with no box it is never dropped — `runtime/src/primitives/prune_tasks.rs` — **waived**: What a spec owes is derived by check-review-gate, which the prose only names, so pruning it loses no gate, and where owed runs live is a requirement of its own. (decided by the rule)
- bug: 026's, 058's and 078's ordering constraints lost the box /implement and the gate read, so no step holds a review to them — `specs/078-review-fix-rounds-are-re-read-within-a-bound/tasks.md` — **waived**: No gate orders one spec's runs after another's: the owner sets the closing order (task 12, 2026-11-09), and a gate for it is a requirement of its own. (decided by the rule)
- other: an analysis whose own chore fix appends a task cannot record, since only a deferral's task is exempt — `runtime/src/primitives/write_analysis.rs` — **waived**: AC28's exception covers a deferral alone, as written, and a fix that appends a task is no chore. (decided by the rule)
- other: no test pins REOPENING_WINDOW_DAYS's 30 days, both default-window tests deriving the window from the constant — `runtime/src/primitives/measure_reopenings.rs` — **waived**: It is outside this scope and in another session's active file (spec 080, 01b97a87), and the constant is defined once and named in the data model. (decided by the rule)
- bug: a first review that stopped early still drops unchanged plan entries from the next scope, so what its skipped pass never read stays unread, against the module doc's 'stopping early leaves nothing permanently unread'. — `runtime/src/primitives/compute_review_scope.rs:201` — **waived**: AC6 keeps an unchanged plan entry only on the first review, and that run was the first; what its skipped pass did not read is named in the record's skipped-passes, and restricting a run is the operator's choice. (decided by the rule)
- step 8 names no family — `framework/commands/analyze.md` — **waived**: It points at the description, as AC9 asks. (decided by the rule)
- a reference added as an ordered item or a table row reads `reopens` — `runtime/src/primitives/classify_spec_edit.rs` — **waived**: It errs toward reopening. (decided by the rule)
- a reference-only edit reads `mechanical` while staleness reads stale — `runtime/src/primitives/classify_spec_edit.rs` — **waived**: Exempting references from the digest is a change of its own, with 0 occurrences since no spec here carries `references:`. (decided by the rule)
- `cross-spec-impact:` names neither 000 nor 035, though the round edited their scenarios and reopened 035 — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Each is a correction of a claim, which owes no back-link and names this spec in its commit, as `plan.md`'s Cross-spec impact says; AC12's list is of specs whose surface this changes. (decided by the rule)
- `an_open_break_names_its_file` refuses a blank-path break whose deferral releases it, though only a break no deferral releases must name its path — `runtime/src/primitives/write_review.rs` — **waived**: It fails closed, the host passes the path it has, and the condition predates this run's fixes. (decided by the rule)
- convention: AC28 says both commands stop at step 1, while /analyze's step 1 now goes on for a done spec holding a box — `specs/079-review-and-analysis-work-converges/spec.md` — **fixed** (decided by the rule)
- convention: Family 24's fix guidance says a residue repair changes no claim and leaves a done spec done, a test the constitution no longer states — `scripts/audit/rename-sweep-residue.sh` — **fixed** (decided by the rule)
- convention: 050's cross-spec-impact scenario and 063's Resolved Question cite §spec-lifecycle's removed test as current — `specs/050-constitution/scenarios/a-declared-cross-spec-impact-gates-done.md` — **fixed** (decided by the rule)
- convention: task 25's closing paragraph says the review reads the code the mutation lists ran against, which this round changed — `specs/079-review-and-analysis-work-converges/tasks.md` — **fixed** (decided by the rule)
- convention: 050's AC13 has no period before its new annotation — `specs/050-constitution/spec.md` — **fixed** (decided by the rule)
- convention: the data model's blocked-by sentence leaves its message ambiguous between fix-round-loop-open and review-owed — `specs/079-review-and-analysis-work-converges/data-model.md` — **fixed** (decided by the rule)
- other: 041's prune scenario rests a plan section's removal on its changing no claim, the removed test — `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md` — **waived**: It records a decision taken under the old rule, and its conclusion holds, since §spec-lifecycle gives a plan.md edit no back-edge. (decided by the rule)
- other: Family 24's guidance says a repair in a scenario or data model reopens, while the same repair made across files reads as a sweep — `scripts/audit/rename-sweep-residue.sh` — **waived**: Family 24 reports each site on its own, and §spec-lifecycle says a typo fixed in one file reopens. (decided by the rule)
- other: 063's split Resolved Question calls a split a meaningful edit because it moves a requirement, which holds only for a criterion or scenario — `specs/063-oversized-artifacts-warn-with-a-fix/spec.md` — **waived**: A split moves requirements, which are criteria or scenarios, so its conclusion holds. (decided by the rule)
- other: 063's plan justifies 058's step-number sync as mechanical because it changes no claim — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — **waived**: It records a decision, and its conclusion holds under §spec-lifecycle's case (a). (decided by the rule)
- other: 050's plan says AC13 settles whether an edit that changes no claim is mechanical — `specs/050-constitution/plan.md` — **waived**: It states the question AC13 settled, which the plan's following sentences now answer with the default and classify-spec-edit's test. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/link.md` — 030-cross-service-references
- `.ductus/reference/analyze.md` — 008-security-rules, 017-derive-dont-ask, 030-cross-service-references, 077-security-cross-rules
- `.ductus/reference/status.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `.githooks/pre-commit` — 080-deterministic-runtime-efficiency
- `.github/workflows/generators.yml` — 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-implement.md` — 080-deterministic-runtime-efficiency
- `.pi/prompts/ductus-link.md` — 030-cross-service-references
- `.pi/prompts/ductus-status.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `docs/cross-service-references.md` — 030-cross-service-references
- `docs/rules.md` — 008-security-rules, 077-security-cross-rules
- `docs/runtime.md` — 080-deterministic-runtime-efficiency
- `framework/bootstrap/configure/auggie.md` — 022-deterministic-runtime, 023-govern-refinement, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `framework/bootstrap/configure/claude.md` — 022-deterministic-runtime, 023-govern-refinement, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `framework/bootstrap/ductus-procedure.md` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `framework/bootstrap/ductus.md` — 029-bootstrap-runtime-autowire, 080-deterministic-runtime-efficiency
- `framework/bootstrap/govern.md` — 029-bootstrap-runtime-autowire, 080-deterministic-runtime-efficiency
- `framework/commands/implement.md` — 080-deterministic-runtime-efficiency
- `framework/commands/link.md` — 030-cross-service-references
- `framework/commands/status.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `framework/reference/analyze.md` — 008-security-rules, 017-derive-dont-ask, 030-cross-service-references, 077-security-cross-rules
- `framework/reference/status.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `framework/rules/accessibility-frontend.md` — 069-accessibility-frontend-rules
- `framework/rules/api-backend.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `framework/rules/concurrency-backend.md` — 038-concurrency-backend-rules
- `framework/rules/configuration-cross.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `framework/rules/performance-frontend.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 048-govern-acquired-runtime, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `framework/rules/quality-cross.md` — 036-quality-cross-rules
- `framework/rules/security-backend.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 048-govern-acquired-runtime, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `framework/rules/security-cross.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 077-security-cross-rules
- `framework/rules/security-frontend.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `framework/runtime-tools.txt` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `framework/templates/project/errors.md` — 008-security-rules
- `install.sh` — 080-deterministic-runtime-efficiency
- `runtime/src/hook.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/interpreter/payload.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/lib.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/main.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/mcp/server.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/apply_manifest.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/bounded_lists.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/check_artifacts.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/check_corpus_links.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/check_unfolded_specs.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/dashboard.rs` — 022-deterministic-runtime, 030-cross-service-references, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/derive_boundary.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/derive_references.rs` — 030-cross-service-references
- `runtime/src/primitives/lint_markdown.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/mark_task.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/measure_reopenings.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/merge_permissions.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/merge_status_hook.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/mod.rs` — 022-deterministic-runtime, 030-cross-service-references, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/pipeline_view.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/prune_plan.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/prune_project.rs` — 008-security-rules
- `runtime/src/primitives/read_spec.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/render_status.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/primitives/resolve_references.rs` — 030-cross-service-references
- `runtime/src/primitives/rewrite_spec_links.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/primitives/tool_loading.rs` — 080-deterministic-runtime-efficiency
- `runtime/src/schema/config_keys.rs` — 030-cross-service-references
- `runtime/src/schema/registry.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/src/schema/services.rs` — 030-cross-service-references
- `runtime/tests/common/corpus.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/common/mod.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/config_unknown_keys.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/cross_service.rs` — 030-cross-service-references
- `runtime/tests/golden/implement-basic.jsonl` — 080-deterministic-runtime-efficiency
- `runtime/tests/golden/plan-basic.jsonl` — 080-deterministic-runtime-efficiency
- `runtime/tests/golden/status-basic.jsonl` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/tests/hook_status.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/line_ending_discipline.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/measure_reopenings.rs` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `runtime/tests/merge_status_hook.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/mixed_corpus.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/result_sizes.rs` — 080-deterministic-runtime-efficiency
- `runtime/tests/specs_root_override.rs` — 080-deterministic-runtime-efficiency
- `scripts/audit/broken-relative-links.sh` — 080-deterministic-runtime-efficiency
- `scripts/audit/lib.sh` — 080-deterministic-runtime-efficiency
- `scripts/audit/manifest-destination-links.sh` — 080-deterministic-runtime-efficiency
- `scripts/audit/review-freshness.sh` — 080-deterministic-runtime-efficiency
- `scripts/check-release-tag.sh` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `scripts/gen-claude-commands.sh` — 080-deterministic-runtime-efficiency
- `scripts/measure-result-sizes.sh` — 080-deterministic-runtime-efficiency
- `scripts/mutation/cross-service-references.py` — 030-cross-service-references
- `scripts/mutation/destination-checks.py` — 080-deterministic-runtime-efficiency
- `scripts/mutation/merge-status-hook.py` — 080-deterministic-runtime-efficiency
- `scripts/mutation/scope-results-bounded.py` — 080-deterministic-runtime-efficiency
- `scripts/mutation/tool-loading.py` — 080-deterministic-runtime-efficiency
- `scripts/tests/test-check-release-tag.sh` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `scripts/tests/test-gen-claude-commands.sh` — 080-deterministic-runtime-efficiency
- `specs/000-slash-commands/scenarios/a-done-stop-says-the-request-is-not-built.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/000-slash-commands/scenarios/target-names-the-step-the-gate-would-name.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/analysis.md` — 008-security-rules
- `specs/008-security-rules/data-model.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 048-govern-acquired-runtime, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/plan.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 048-govern-acquired-runtime, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/review.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/scenarios/a-custom-credential-header-does-not-follow-a-redirect.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/scenarios/rules-are-as-short-as-their-required-content.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/scenarios/single-host-appliance-rule-fit.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/scenarios/x-frame-options-carries-one-strength.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/008-security-rules/spec.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/010-agent-autonomy/analysis.md` — 010-agent-autonomy
- `specs/010-agent-autonomy/review.md` — 010-agent-autonomy
- `specs/010-agent-autonomy/spec.md` — 010-agent-autonomy
- `specs/014-reclarify-backedge/analysis.md` — 014-reclarify-backedge
- `specs/014-reclarify-backedge/review.md` — 014-reclarify-backedge
- `specs/014-reclarify-backedge/spec.md` — 014-reclarify-backedge
- `specs/015-tarball-fetch/analysis.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/review.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/spec.md` — 015-tarball-fetch
- `specs/016-cross-cutting-rules/analysis.md` — 016-cross-cutting-rules
- `specs/016-cross-cutting-rules/plan.md` — 016-cross-cutting-rules
- `specs/016-cross-cutting-rules/review.md` — 016-cross-cutting-rules
- `specs/016-cross-cutting-rules/spec.md` — 016-cross-cutting-rules
- `specs/017-derive-dont-ask/data-model.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/017-derive-dont-ask/spec.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 077-security-cross-rules
- `specs/019-config-decisions/analysis.md` — 019-config-decisions
- `specs/019-config-decisions/review.md` — 019-config-decisions
- `specs/019-config-decisions/spec.md` — 019-config-decisions
- `specs/020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/020-code-review/scenarios/review-reports-a-defect-under-its-owner.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/022-deterministic-runtime/data-model.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/scenarios/a-closed-stdout-is-a-quiet-exit.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/022-deterministic-runtime/scenarios/apply-manifest-substitution-contract.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/022-deterministic-runtime/scenarios/coverage-expansion-primitives.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/023-govern-refinement/analysis.md` — 023-govern-refinement
- `specs/023-govern-refinement/plan.md` — 023-govern-refinement
- `specs/023-govern-refinement/review.md` — 023-govern-refinement
- `specs/023-govern-refinement/scenarios/configure-retires-formerly-canonical-entries.md` — 023-govern-refinement
- `specs/023-govern-refinement/scenarios/living-specs.md` — 023-govern-refinement
- `specs/023-govern-refinement/spec.md` — 023-govern-refinement
- `specs/029-bootstrap-runtime-autowire/analysis.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/review.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/state-a-deterministic-path-forcing.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 029-bootstrap-runtime-autowire
- `specs/030-cross-service-references/analysis.md` — 030-cross-service-references
- `specs/030-cross-service-references/data-model.md` — 030-cross-service-references
- `specs/030-cross-service-references/plan.md` — 030-cross-service-references
- `specs/030-cross-service-references/review.md` — 030-cross-service-references
- `specs/030-cross-service-references/spec.md` — 030-cross-service-references
- `specs/032-opencode-agent/analysis.md` — 032-opencode-agent
- `specs/032-opencode-agent/data-model.md` — 032-opencode-agent
- `specs/032-opencode-agent/plan.md` — 032-opencode-agent
- `specs/032-opencode-agent/review.md` — 032-opencode-agent
- `specs/032-opencode-agent/spec.md` — 032-opencode-agent
- `specs/034-performance-backend-rules/analysis.md` — 034-performance-backend-rules
- `specs/034-performance-backend-rules/review.md` — 034-performance-backend-rules
- `specs/034-performance-backend-rules/spec.md` — 034-performance-backend-rules
- `specs/035-groom-session-target/analysis.md` — 035-groom-session-target
- `specs/035-groom-session-target/review.md` — 035-groom-session-target
- `specs/035-groom-session-target/spec.md` — 035-groom-session-target
- `specs/036-quality-cross-rules/analysis.md` — 036-quality-cross-rules
- `specs/036-quality-cross-rules/data-model.md` — 036-quality-cross-rules
- `specs/036-quality-cross-rules/plan.md` — 036-quality-cross-rules
- `specs/036-quality-cross-rules/review.md` — 036-quality-cross-rules
- `specs/036-quality-cross-rules/scenarios/a-bound-test-pins-the-stated-figure.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/036-quality-cross-rules/spec.md` — 036-quality-cross-rules
- `specs/036-quality-cross-rules/tasks.md` — 036-quality-cross-rules
- `specs/037-observability-backend-rules/analysis.md` — 037-observability-backend-rules
- `specs/037-observability-backend-rules/review.md` — 037-observability-backend-rules
- `specs/037-observability-backend-rules/spec.md` — 037-observability-backend-rules
- `specs/038-concurrency-backend-rules/analysis.md` — 038-concurrency-backend-rules
- `specs/038-concurrency-backend-rules/plan.md` — 038-concurrency-backend-rules
- `specs/038-concurrency-backend-rules/review.md` — 038-concurrency-backend-rules
- `specs/038-concurrency-backend-rules/scenarios/lock-the-parent-before-rewriting-its-children.md` — 038-concurrency-backend-rules
- `specs/038-concurrency-backend-rules/spec.md` — 038-concurrency-backend-rules
- `specs/039-reliability-backend-rules/analysis.md` — 039-reliability-backend-rules
- `specs/039-reliability-backend-rules/review.md` — 039-reliability-backend-rules
- `specs/039-reliability-backend-rules/spec.md` — 039-reliability-backend-rules
- `specs/045-decision-state-drift-detection/analysis.md` — 045-decision-state-drift-detection
- `specs/045-decision-state-drift-detection/review.md` — 045-decision-state-drift-detection
- `specs/045-decision-state-drift-detection/spec.md` — 045-decision-state-drift-detection
- `specs/048-govern-acquired-runtime/spec.md` — 008-security-rules, 020-code-review, 048-govern-acquired-runtime, 068-api-backend-rules, 070-performance-frontend-rules
- `specs/059-project-in-a-repository-subdirectory/analysis.md` — 059-project-in-a-repository-subdirectory
- `specs/059-project-in-a-repository-subdirectory/plan.md` — 059-project-in-a-repository-subdirectory
- `specs/059-project-in-a-repository-subdirectory/review.md` — 059-project-in-a-repository-subdirectory
- `specs/059-project-in-a-repository-subdirectory/spec.md` — 059-project-in-a-repository-subdirectory
- `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/an-assessment-sees-the-loaded-rule-files.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/an-assessment-sees-the-rule-it-asks-about.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/061-updates-track-the-latest-release-tag/analysis.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/plan.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/review.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/spec.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/tasks.md` — 061-updates-track-the-latest-release-tag
- `specs/062-concurrent-session-targets/plan.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/066-audit-records-and-the-completion-gate/scenarios/a-review-states-what-it-read.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/scenarios/an-empty-scope-flag-does-not-decide-the-denominator.md` — 066-audit-records-and-the-completion-gate
- `specs/067-analyze-and-audit-checks/tasks.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/068-api-backend-rules/analysis.md` — 068-api-backend-rules
- `specs/068-api-backend-rules/plan.md` — 068-api-backend-rules
- `specs/068-api-backend-rules/review.md` — 068-api-backend-rules
- `specs/068-api-backend-rules/spec.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/068-api-backend-rules/tasks.md` — 068-api-backend-rules
- `specs/069-accessibility-frontend-rules/analysis.md` — 069-accessibility-frontend-rules
- `specs/069-accessibility-frontend-rules/plan.md` — 069-accessibility-frontend-rules
- `specs/069-accessibility-frontend-rules/review.md` — 069-accessibility-frontend-rules
- `specs/069-accessibility-frontend-rules/spec.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/069-accessibility-frontend-rules/tasks.md` — 069-accessibility-frontend-rules
- `specs/070-performance-frontend-rules/analysis.md` — 070-performance-frontend-rules
- `specs/070-performance-frontend-rules/plan.md` — 070-performance-frontend-rules
- `specs/070-performance-frontend-rules/review.md` — 070-performance-frontend-rules
- `specs/070-performance-frontend-rules/spec.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/070-performance-frontend-rules/tasks.md` — 070-performance-frontend-rules
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/analysis.md` — 076-waived-and-deferred-findings-are-distinct-dispositions
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/review.md` — 076-waived-and-deferred-findings-are-distinct-dispositions
- `specs/076-waived-and-deferred-findings-are-distinct-dispositions/spec.md` — 076-waived-and-deferred-findings-are-distinct-dispositions
- `specs/077-security-cross-rules/analysis.md` — 077-security-cross-rules
- `specs/077-security-cross-rules/data-model.md` — 008-security-rules, 017-derive-dont-ask, 020-code-review, 077-security-cross-rules
- `specs/077-security-cross-rules/plan.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/077-security-cross-rules/review.md` — 008-security-rules, 077-security-cross-rules
- `specs/077-security-cross-rules/spec.md` — 000-slash-commands, 008-security-rules, 017-derive-dont-ask, 020-code-review, 022-deterministic-runtime, 036-quality-cross-rules, 060-exec-analyze-assesses-each-loaded-rule, 067-analyze-and-audit-checks, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 077-security-cross-rules
- `specs/077-security-cross-rules/tasks.md` — 008-security-rules, 077-security-cross-rules
- `specs/080-deterministic-runtime-efficiency/data-model.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/plan.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/research.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/spec.md` — 080-deterministic-runtime-efficiency
- `specs/080-deterministic-runtime-efficiency/tasks.md` — 022-deterministic-runtime, 062-concurrent-session-targets, 080-deterministic-runtime-efficiency

## Unexamined governance

*None.*
