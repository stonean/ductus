---
spec: 079-review-and-analysis-work-converges
last-run: 2026-11-06T21:57:41Z
reviewed-against: 2f6be50893b11c6a1e7254666d18e4c65b0d424a
diff-base: 6aef0472e78513efc0e7c6b3c55239c0f45cbf0c
must-violations: 0
should-violations: 0
low-confidence: 1
examined: 89
scope: 127
excluded: 15
contract-breaks: 0
skipped-passes: []
fix-rounds: 4
re-reads: 4
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 27f6e645976eaac47d06ba97caf63867bf6471c30ab7aa70183796bbba102538
blocking: false
dispositions:
  fixed: 52
  deferred: 0
  waived: 12
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
---

# Review — 079-review-and-analysis-work-converges

## Summary

Fresh review closing this spec's cut-off run, over the window since `6aef0472`, the commit holding the first review's fixes, named by `--since` because the passes at `5f67ede2` and the flow pass read what came before it. Its passes ran at `1a0623f2`, by three independent read-only agents following one behavior each with the release binary: what a run records, where a window starts and what the gate holds, and what reopens with every shipped sentence, under the 12 rule files `discover-rule-files` selects. `research.md`'s *What the fresh review found* holds each finding's input, result and fix under the ids task 18 uses.

`examined` is 89 of the 116 paths in scope when the passes ran. The other 27 are generated copies under `.claude/`, `.ductus/` and `.pi/`, not opened, each generated from a source read in full, with the generators re-run and reported in sync. `scope` reads 127 because the fixes added 11 paths the passes never read, `runtime/src/main.rs`, `runtime/src/schema/registry.rs`, `runtime/src/primitives/remove_inbox_item.rs` and eight under 000, 022, 035 and 041, which the fix rounds re-read rather than the passes.

The fix rounds start from base `71e679295323cb200054e8e2effb287819483deb`, the snapshot taken at `1a0623f2` before the batch, not a fresh one after it landed, since a round is derived from a base that predates its fixes and a fresh snapshot would brief an empty round and leave the batch unread (`framework/reference/review.md`, *Fix-round loop*). The fixes landed in `64d0be93` and `2f6be508`, which that base reaches as committed work.

The passes found 4 SHOULD violations, `QUAL-CLAIM-001` three times and `QUAL-TEST-001` once, all fixed before this record and so outside its counts. One low-confidence finding stands on `was_folded`. Of 64 observations, 52 are fixed and 12 waived, each by the rule or, for O1 and 035's scenario, as the owner chose. The fixed hold the 4 violations, 16 contract breaks the passes found and 3 the rounds found, each with a test shown to fail without it and a mutant, those 2 choices, the mechanical list and the rounds' other findings. Every mutant this run added or re-anchored was killed by id, after M229, M241, M245 and M256 first survived: three missing tests, and one redundant check, removed. 5 waivers are new and 5 stored ones are carried byte-identical; the stored one on the gate's order expired, since every review check now runs ahead of the analyze checks. Nothing is undispositioned.

Four fix rounds, each re-read by an independent agent from the runtime's brief alone. Round 1, the 44 paths the batch and its landings changed, found 1 MUST, 4 SHOULD and 7 NOTE: a stored break matched by its key with no path left the record counting it and listing nothing, which passed the gate, reproduced on a fixture and fixed with two tests and a mutant each. Round 2, 16 paths, found 4 SHOULD and 4 NOTE, one of them a gap in that fix. That was the budget's second re-read with a finding, so the owner was asked, and chose another round (2026-11-06). Round 3, 5 paths, found 2 SHOULD, both wording with prescribed edits; checking them by diff would have left `budget-stop: true` and a second full review owed, so the owner chose a re-read instead (2026-11-07). Round 4, 4 paths, found no MUST or SHOULD, and the next brief read `empty`. The re-reads took about 431,500, 270,000, 189,700 and 150,900 tokens and 29, 19, 11 and 6 minutes, as the host reported them. Each round's NOTE is an observation above; round 4's repeats round 2's on this record's heading, fixed in this run's own input.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: QUAL-CLAIM-001 — A staging spec whose frontmatter does not parse at removal reads as no fold, with no notice

- **File**: `runtime/src/primitives/compute_review_scope.rs:816`
- **Rule**: A result that reports a clean, empty, or in-sync state SHOULD distinguish examined the subject and found nothing from could not examine the subject.
- **Finding**: `was_folded` reads a removed staging spec's frontmatter to find `folds-into`; one that does not parse reads as no fold, so what it brought stays out of scope and nothing says so. `/fold`'s `read-spec` should stop on it first, so reaching this needs a by-hand removal, which is why it is recorded low-confidence.
- **Auto-fixable**: no

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- QUAL-CLAIM-001, SHOULD: `write-review --empty-scope` over a record the gate calls unreadable wrote `must-violations: 0` and "Zero findings", as if none existed; it refuses now — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- QUAL-CLAIM-001, SHOULD: a tip dated before the window ended the walk, so one closing read `nothing-to-compare`; such a commit is skipped now — `runtime/src/primitives/measure_reopenings.rs` — **fixed** (decided by the rule)
- QUAL-CLAIM-001, SHOULD: after a slug rename `since-sha` held but the count and first box read the new path, turning `commit-count: 4, stuck: true` into `1, false`; each commit's path resolves by number now — `runtime/src/primitives/check_stuck.rs` — **fixed** (decided by the rule)
- QUAL-TEST-001, SHOULD: `a_reference_added_or_removed_as_a_list_item_is_mechanical` never removed one; it does now — `runtime/src/primitives/classify_spec_edit.rs` — **fixed** (decided by the rule)
- one `--waive` through a run's two writes stored two identical entries and read `decisions-by-person: 2`, while dropping it from the last write brought the finding back; an anchor recorded now applies and is not stored again — `runtime/src/primitives/write_review.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- a deferral of a break to a work item alone: the next run matched it with no break flag, wrote `contract-breaks: 0` and the gate passed with the break unfixed; the stored entry carries `contract-break: true` now — `runtime/src/primitives/decisions.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- `ductus exec review since=HEAD` over a record counting a MUST wrote 0, since the walker never sets `empty-scope` and the refusal read only the flag; a derived empty scope refuses too — `runtime/src/primitives/write_review.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- `src/b.rs:3:9` read a person's decision as expired with the file unread, and a break listed a path the next scope could not stat; one shared stripper, repeating — `runtime/src/primitives/decisions.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- a plan's `src/*.rs` in scope read a decision on `src/b.rs` as retained, "did not read"; the scope is tested through `PlanEntries` now — `runtime/src/primitives/compute_review_scope.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- close, reopen, code, a commit with `status: [` or `spec.md` removed, then its repair moved the base to before the repair, so one analysis passed; the reopening carries through a status-less commit now — `runtime/src/primitives/check_stuck.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- code with the flip, a run, then `git revert` to the run the reopening carried set the base at the revert with scope 0; the window stays at the reopen now — `runtime/src/primitives/compute_review_scope.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- reverting away the only run left the base at the reverted run with scope 0; no record at `HEAD` starts at the reopen now — `runtime/src/primitives/compute_review_scope.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- a staging spec renamed before its fold lost commits touching its old slug; the directory entry matches by `FeatureForm` now — `runtime/src/primitives/commit_attribution.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- with an uncommitted rename the gate blocked naming `src/b.rs` while the scope did not hold it; the record resolves through `feature_file_in_tree` now — `runtime/src/primitives/compute_review_scope.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- a rename-only reopening holding a scenario owed a review; a contract its digest still describes owes none now, and with no digest it still does — `runtime/src/primitives/compute_review_scope.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- a box in a `~~~` fence counted as owed, against AC27; both fences are recognized, only its own kind closing one — `runtime/src/primitives/mod.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- a link ahead of a reference borrowed its target, so a prose change between them read `mechanical`, and multibyte text there panicked; a URL is taken only when it follows that `](` now — `runtime/src/primitives/derive_references.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- the spec-edit reopen on `/groom` had no step and no consent, and read an error as staying `done`; step 7 now classifies the written edit and asks — `framework/commands/groom.md` (breaks the spec's contract) — **fixed** (decided by the rule)
- the retired "without the server the agent runs the CLI" survived in two scenarios and two doc comments; each says what runs there now — `runtime/src/schema/registry.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- a sweep through `research.md` in two specs read `reopens` with `other-file`, against the constitution's "an edit elsewhere"; that arm takes the same test, a real change staying `other-file` — `runtime/src/primitives/classify_spec_edit.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- a run recording a budget stop or a skipped pass left no gating finding, so the re-run the gate demands read over the bound; it counts as leaving one now, as the owner chose — `runtime/src/primitives/measure_reopenings.rs` — **fixed** (decided by the rule)
- 035's scenario said a spec-edit route never reopens, false since `/groom` asked `classify-spec-edit`; corrected and 035 reopened, as the owner chose — `specs/035-groom-session-target/scenarios/confirmation-names-reopen.md` — **fixed** (decided by the rule)
- the record kept `excluded`, `contract-breaks` and `fix-rounds-open` beside `last-run: null`; `open-findings` stays — `runtime/src/primitives/invalidate_review.rs` — **fixed** (decided by the rule)
- the empty-scope Summary omitted a break it holds — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- the `observations` row's sample said "it does not hold done" with a break among them — `framework/reference/review.md` — **fixed** (decided by the rule)
- step 22's `--fix` list omitted an outstanding SHOULD, a listed break, the open mark, an unticked criterion and an unchecked box; it names the drift as a set now — `framework/commands/analyze.md` — **fixed** (decided by the rule)
- "check 12", renamed in 078's data model — `specs/066-audit-records-and-the-completion-gate/scenarios/the-fix-round-loop-is-recorded-and-gated.md` — **fixed** (decided by the rule)
- "check 12" and "check 11", renamed in its own data model — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — **fixed** (decided by the rule)
- doc comments on the wrong item, on `surviving_waivers`, `unreadable_record_block` and `scenario_question_block` — `runtime/src/primitives/check_review_gate.rs` — **fixed** (decided by the rule)
- the accounting sentence omitted what a fold brings, and the file gained `new-waivers`' two-write rule and `decisions:`' `contract-break` — `specs/079-review-and-analysis-work-converges/data-model.md` — **fixed** (decided by the rule)
- enumerations of review-state drift each omitted a condition, a listed break among them, in `CHECK_ARTIFACTS`, `docs/analyze.md`, 067's data model and `check_artifacts.rs`' module doc — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- "What reopens" said the classifier and the staleness check agree on a reference-only edit — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- the size figures, now by `wc -c` — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- M115's name said a tick reopens, where it makes a criterion left unticked reopen — `scripts/mutation/reopen-measure.py` — **fixed** (decided by the rule)
- step 8 names no family — `framework/commands/analyze.md` — **waived**: It points at the description, as AC9 asks. (decided by the rule)
- `1. [ ]`, `> - [ ]`, `* [ ]` and `+ [ ]` are no boxes to `find_checkbox_line` — `runtime/src/primitives/mod.rs` — **waived**: One documented grammar, `mark-task`'s too, every surface alike, and 0 such lines here. (decided by the rule)
- a claim carried only in a reference's label reads `mechanical` — `runtime/src/primitives/derive_references.rs` — **waived**: The scrub removes label and target by its stated design, and `plan.md`'s known limitations say so. (decided by the rule)
- a reference added as an ordered item or a table row reads `reopens` — `runtime/src/primitives/classify_spec_edit.rs` — **waived**: It errs toward reopening. (decided by the rule)
- a reference-only edit reads `mechanical` while staleness reads stale — `runtime/src/primitives/classify_spec_edit.rs` — **waived**: Exempting references from the digest is a change of its own, with 0 occurrences since no spec here carries `references:`. (decided by the rule)
- bug: a first review that stopped early still drops unchanged plan entries from the next scope, so what its skipped pass never read stays unread, against the module doc's 'stopping early leaves nothing permanently unread'. — `runtime/src/primitives/compute_review_scope.rs:201` — **waived**: AC6 keeps an unchanged plan entry only on the first review, and that run was the first; what its skipped pass did not read is named in the record's skipped-passes, and restricting a run is the operator's choice. (decided by the rule)
- perf: check-prescriptions counts an exempt path as unprescribed, so a round that applied exactly the prescribed edits and also touched spec.md, plan.md or tasks.md differs and costs another re-read of paths the check had just matched. — `runtime/src/primitives/check_prescriptions.rs:82` — **waived**: No criterion covers it: AC13 covers the brief, and this costs one re-read on a rare round that also edits the spec's own text. (decided by the rule)
- bug: a spec directory renamed while in-progress counts a record carried from before the rename as a run of this reopening, which can report a false over-bound. — `runtime/src/primitives/measure_reopenings.rs:708` — **waived**: A rename mid-reopening is rare, it over-reports rather than hides, no gate reads the count, and the module doc states that a renamed spec's history starts at the rename. (decided by the rule)
- other: cross_service.rs's second assertion and tests/parity/cross-service/expected.txt are now a copy of the golden whose only effect is requiring a re-bless in two places, as the file's own doc says. — `runtime/tests/cross_service.rs:60` — **waived**: plan.md's Affected Files keeps this assertion on purpose, and 030's plan and tasks name the capture, so removing it is a design choice no criterion here asks for. (decided by the rule)
- bug: /amend's reopen check runs classify-spec-edit only on a delta that excludes a modified scenario and a data model, the two edits §spec-lifecycle says always reopen, and its line then says no scenario edit exists. — `framework/commands/amend.md:58` — **waived**: The delta predates this spec, AC15 governs what classify-spec-edit returns and not which edits reach it, and widening the delta changes what the reconcile pass offers, which is a requirement of its own. (decided by the rule)
- a stored break matched by its key with no path skipped the required-path refusal, so the record read `contract-breaks: 1` with no `open-findings` and the gate passed; it lists the file the key names now, and refuses when nothing names one — `runtime/src/primitives/write_review.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- moving the doc comments left `unchecked_task_block` opening with the scenario-open-questions sentence — `runtime/src/primitives/check_review_gate.rs` — **fixed** (decided by the rule)
- the module doc, `Boundary`, `passed` and a test's doc still said the walk stops past the window — `runtime/src/primitives/measure_reopenings.rs` — **fixed** (decided by the rule)
- the review-state-drift bullet put the criterion and the box beside the grandfather rule that does not apply to them — `runtime/src/primitives/check_artifacts.rs` — **fixed** (decided by the rule)
- `# Errors` said a no-block record is overwritten and omitted the empty-scope refusal and the `new-waivers` exception — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- inserting `/groom`'s step 7 left "step 8" naming the inbox removal in a doc comment, 022's data model and two scenarios — `runtime/src/primitives/remove_inbox_item.rs` — **fixed** (decided by the rule)
- the section said a `done` spec with no `review.md` is never reverted, though an unticked criterion or an unchecked box has no grandfather rule — `framework/reference/analyze.md` — **fixed** (decided by the rule)
- the measurement paragraph still said the walk stops past the window — `specs/079-review-and-analysis-work-converges/plan.md` — **fixed** (decided by the rule)
- a slug rename not yet committed read not stuck, since HEAD was read by the new path; it resolves by number now — `runtime/src/primitives/check_stuck.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- a staging spec renamed, then taken to `done`, then folded brought its commits, since a rename read as a removal; it brings what the same fold with no rename does now — `runtime/src/primitives/compute_review_scope.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- `is_own_record` read the config file once per scope entry; the path is read once now — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- `cross-spec-impact:` names neither 000 nor 035, though the round edited their scenarios and reopened 035 — `specs/079-review-and-analysis-work-converges/spec.md` — **waived**: Each is a correction of a claim, which owes no back-link and names this spec in its commit, as `plan.md`'s Cross-spec impact says; AC12's list is of specs whose surface this changes. (decided by the rule)
- the changelog's size sentence went false with the round's own edit to the analyze reference; it reads 420,580 and 48,696 by `wc -c` now — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- the grandfather paragraph still said a `done` spec with no `review.md` is exempt, unqualified, against the sentence below it — `framework/reference/analyze.md` — **fixed** (decided by the rule)
- `validate_scalar_fields`' comment said every check runs there, after the open-break check moved out — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- a later run matching a stored break by key, with the mark set and no path, was refused while the same run unmarked listed the file; the matched key names it either way now — `runtime/src/primitives/write_review.rs` (breaks the spec's contract) — **fixed** (decided by the rule)
- a stored key passed as an observation's text filled the path and stored a second key with it doubled; only a matched `decision-key` fills one now, and such a run refuses — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- `# Errors` omitted the refusal of an open break naming no file — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- the record's low-confidence heading ended in a full stop, failing MD026; it came from this run's own summary — `specs/079-review-and-analysis-work-converges/review.md` — **fixed** (decided by the rule)
- `an_open_break_names_its_file` refuses a blank-path break whose deferral releases it, though only a break no deferral releases must name its path — `runtime/src/primitives/write_review.rs` — **waived**: It fails closed, the host passes the path it has, and the condition predates this run's fixes. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/audit.md` — 022-deterministic-runtime, 030-cross-service-references, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix
- `.claude/commands/ductus/clarify.md` — 022-deterministic-runtime, 030-cross-service-references, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix
- `.claude/commands/ductus/consolidate.md` — 022-deterministic-runtime, 030-cross-service-references, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix
- `.claude/commands/ductus/fold.md` — 022-deterministic-runtime, 030-cross-service-references, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix
- `.claude/commands/ductus/log.md` — 022-deterministic-runtime, 030-cross-service-references, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix
- `.claude/commands/ductus/plan.md` — 022-deterministic-runtime, 030-cross-service-references, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix
- `.claude/commands/ductus/prune.md` — 022-deterministic-runtime, 030-cross-service-references, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix
- `.claude/commands/ductus/specify.md` — 022-deterministic-runtime, 030-cross-service-references, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix
- `.claude/commands/ductus/target.md` — 022-deterministic-runtime, 030-cross-service-references, 062-concurrent-session-targets, 063-oversized-artifacts-warn-with-a-fix
- `framework/reference/fold.md` — 051-branch-scoped-spec-numbering
- `specs/066-audit-records-and-the-completion-gate/scenarios/an-empty-scope-flag-does-not-decide-the-denominator.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-gate.md` — 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/066-audit-records-and-the-completion-gate/scenarios/the-fix-round-loop-is-recorded-and-gated.md` — 000-slash-commands, 022-deterministic-runtime, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/066-audit-records-and-the-completion-gate/scenarios/write-analysis-and-the-second-gate.md` — 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate, 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 017-derive-dont-ask, 020-code-review, 058-findings-route-at-discovery, 059-project-in-a-repository-subdirectory, 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate, 067-analyze-and-audit-checks, 078-review-fix-rounds-are-re-read-within-a-bound

## Unexamined governance

*None.*
