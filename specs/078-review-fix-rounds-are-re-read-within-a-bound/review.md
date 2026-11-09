---
spec: 078-review-fix-rounds-are-re-read-within-a-bound
last-run: 2026-11-09T21:28:14Z
reviewed-against: f74d606014ab779bfc4391dfa45768f29c25f34a
diff-base: 5a24d0dc1802720d8f8fcb61ce7f80b5320ebb95
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 5
excluded: 1
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 1
diff-checked: 1
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 95cd98fd2db64be4ac315c056ac68c8e70b480723cc124a4db436ca5b570cf78
  scenarios/a-reviews-own-record-is-no-fix-round.md: 8a0190ef5a926425199a4381d558f52db626cdd81803b4e23c9aa1055b130440
  scenarios/fix-round-results-have-a-bounded-size.md: bd6472ece8d3586d1de656fd1d1689848d455ed79183249ff003e9db24817de7
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 4
  undispositioned: 0
decisions:
  - key: "bug: 066's data-model example lists two `modified-since` paths with a count of 1 — `specs/066-audit-records-and-the-completion-gate/data-model.md`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "convention: `compute-review-scope`'s file-form test asserts `deleted` on a fixture that deletes nothing, so three mutations survive — `runtime/src/primitives/compute_review_scope.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "bug: a window holding only deletions under an empty plan reaches the nothing-to-review-yet branch — `framework/commands/review.md`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "19"
    decided-at: 2026-11-04T13:33:22Z
    decided-by: andy@stone.dev
  - key: "bug: an open re-read MUST or SHOULD left at a stop is an observation, and undispositioned it never holds done — `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): low confidence; the loop asks the operator to defer or waive each open finding at a stop, and a design change would cost 078 another cycle."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "perf: a file-form loop result's free text (an echoed path or key in problem, the re-reads-off reason) has no bound — `runtime/src/primitives/check_prescriptions.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): reaching it takes an agent-written path or key, or a configured reason, tens of kilobytes long; the lists, which carry the growth, are bounded."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "other: when the rename exemption cannot run, check-review-freshness answers stale with nothing saying the exemption did not run — `runtime/src/primitives/check_review_freshness.rs`"
    outcome: waived
    reason: "Owner, 2026-11-04: 066's specified, fail-closed behavior, outside 078's code; the marker is an enhancement, not a defect."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "bug: review-freshness.sh's call() sets no timeout and catches no OSError, so an exec failure ends Family 19 in a traceback with no finding or coverage line — `scripts/audit/review-freshness.sh`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "bug: review-freshness.sh's undeterminable advice to re-run review is wrong for a review.md that does not parse — `scripts/audit/review-freshness.sh`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "other: the Family 19 parity test never seeds the never-run or failed-call branches — `runtime/tests/mechanical_sweep_parity.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "bug: check-review-freshness emits last_run and analyzed_against where 066's data-model example shows kebab-case — `specs/066-audit-records-and-the-completion-gate/data-model.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: 026's family-19-review-freshness edge cases contradict its rewritten Behavior — `specs/026-framework-self-audit/scenarios/family-19-review-freshness.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: 066's review-staleness-on-done-specs says review-freshness.sh has always carried the exemption — `specs/066-audit-records-and-the-completion-gate/scenarios/review-staleness-on-done-specs.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: 026's family-19-mechanical-sweep-exemption says Family 19 compares shas — `specs/026-framework-self-audit/scenarios/family-19-mechanical-sweep-exemption.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: review-freshness.sh's header and audit.md step 19 say the gate asserts only last-run and blocking — `scripts/audit/review-freshness.sh`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: mechanical_sweep.rs and its parity test carry doc comments the window made stale — `runtime/src/primitives/mechanical_sweep.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: RecordFreshness::Undeterminable's doc names only a missing digest, where an unparseable review.md also answers it — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: the Family 19 parity test spawns bash without ductus_command, so session identities reach the runtime — `runtime/tests/mechanical_sweep_parity.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "other: 066's plan states counts of audit scripts and done specs with no method, and they re-derive differently — `specs/066-audit-records-and-the-completion-gate/plan.md`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: review-freshness.sh lists done specs with its own dashboard call and keeps a copy of Paths::load — `scripts/audit/review-freshness.sh`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "convention: review-freshness.sh:97's test-and-command ends its block, the set -e trap should -e be added — `scripts/audit/review-freshness.sh`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: rule QUAL-CLAIM-001 at scripts/audit/review-freshness.sh
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "21"
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: rule BE-PAYLOAD-001 at specs/066-audit-records-and-the-completion-gate/data-model.md
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "20"
    decided-at: 2026-11-04T20:51:31Z
    decided-by: andy@stone.dev
  - key: "bug: check-prescriptions does not normalize an edit's path, so ./a, a//b or a/./b fails the call or differs — `runtime/src/primitives/check_prescriptions.rs`"
    outcome: waived
    reason: "Fail-safe: a path spelled other than repo-relative never matches, so the call fails or the round differs and is re-read (AC15); the brief asks for repo-relative paths."
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "perf: an exempt edit beside exactly the prescribed edits makes the diff check differ, costing a re-read of a verified round — `runtime/src/primitives/check_prescriptions.rs`"
    outcome: waived
    reason: It matches AC5 as written; exempting those paths from the diff check is a requirement change, and its cost is one re-read.
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "bug: the re-reads-off reason's 16-character minimum is measured before its control characters and runs of spaces are collapsed — `runtime/src/primitives/fix_rounds.rs`"
    outcome: waived
    reason: Contrived input; the check matches the data model's 'once trimmed'.
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "other: the loop primitives accept a misspelled [review] key silently — `runtime/src/primitives/fix_rounds.rs`"
    outcome: waived
    reason: "/review's step 2, discover-rule-files, reports an unknown [review] key (CFG-ENV-012), verified on a fixture: 'unknown config key: [review] re-read-off'."
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "security: with --allow-sequential, retire-feature screens the feature name by validate_feature_argument's denylist alone, and --feature . deleted every file under specs/ on a fixture — `runtime/src/primitives/retire_feature.rs`"
    outcome: deferred
    target: specs/052-spec-supersession-and-consolidation/tasks.md
    task: "4"
    decided-at: 2026-11-09T18:37:37Z
    decided-by: andy@stone.dev
  - key: "bug: a record's current digest clears review-owed for its contracts even when that run stopped early — `runtime/src/primitives/compute_review_scope.rs`"
    outcome: waived
    reason: It matches compute-review-scope's description and 079's window design, in the narrow case of a contract-only reopening after a restricted run; not 078's loop.
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "other: a fixer's uncommitted edit that another spec's commit sweeps in leaves the round as excluded and is never re-read here — `runtime/src/primitives/fix_rounds.rs`"
    outcome: waived
    reason: "It follows this spec's Edge Case on other sessions: that spec's review reads it as its own change, and git worktree isolates sessions."
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "other: a supplied Summary gets the loop's account but not the generated one's excluded-from-scope sentence — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: 079's contract; the frontmatter's excluded count and the Excluded from scope section carry it.
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "other: fix rounds driven by mechanical NOTEs have no length bound — `framework/reference/review.md`"
    outcome: waived
    reason: The Behavior permits NOTE rounds and AC6's budget counts re-reads with a MUST or SHOULD; a bound on NOTE rounds is a requirement decision.
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "other: 066's task 19 items 2 and 4 prescribe old text that no longer occurs — `specs/066-audit-records-and-the-completion-gate/tasks.md`"
    outcome: waived
    reason: 066's own task list, outside 078's code; the stored deferrals that pointed there expired this run, their issues gone.
    decided-at: 2026-11-09T18:37:37Z
    decided-by: rule
  - key: "convention: AC14's 'a switch without a valid reason halts the run', read alone, covers an empty re-reads-off, which now counts as absent — `specs/078-review-fix-rounds-are-re-read-within-a-bound/spec.md`"
    outcome: waived
    reason: The data model's re-reads-off row, the durable contract the switch rests on, says an empty value counts as absent and so is no switch, as task 17 records (owner, 2026-11-09).
    decided-at: 2026-11-09T20:01:53Z
    decided-by: rule
  - key: "bug: a waived round's reason ending before a closing quote or bracket, or in an ellipsis or a colon, still takes a second stop — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: "Contrived input: which closing characters end a sentence is a style choice, and the record's contract (078's data model, 066's two loop scenarios) asks only that the Summary name each waived round, which it does; the comment that claimed more is fixed in this run."
    decided-at: 2026-11-09T21:28:14Z
    decided-by: rule
  - key: "other: analysis.md counts as fixed in its run a finding an earlier commit had removed — `specs/078-review-fix-rounds-are-re-read-within-a-bound/analysis.md`"
    outcome: waived
    reason: analysis.md is write-analysis's output, which the analysis this change owes rewrites; whether a finding an earlier commit removed counts as fixed is analyze's machinery, outside 078's code.
    decided-at: 2026-11-09T21:28:14Z
    decided-by: rule
---

# Review — 078-review-fix-rounds-are-re-read-within-a-bound

## Summary

Window `5a24d0dc..f74d6060` (base-source `last-review`, `review-owed` true), 5 scope entries from two 078 commits:

- `acc75e71`, the second analysis;
- `f74d6060`, tasks 18 and 19, which the completion gate's criteria verification found: the `[review]` comment in `config_keys.rs` names `fix_rounds::re_reads_off`'s callers, and a waived round's reason ending in `.`, `!` or `?` takes no second stop.

`bfd9c6b0`'s change to 079's `tasks.md` is excluded as that spec's.

One independent read-only agent ran the five passes over all 5 entries, by their window hunks and the code those call, under the 12 rule files `discover-rule-files` selected. It drove `write-review` on a fixture of 13 waived rounds, one per kind of ending, and the supplied and generated Summary both name each waived round. QUAL-TEST-001 holds for the new test, and BE-INPUT-001 is met. There is no violation.

Seven observations:

- a reason ending before a closing quote or bracket, or in an ellipsis or a colon, still takes a second stop: waived by the rule, as a style choice the record's contract does not reach;
- the code comment that claimed more than the code checks, fixed in the run;
- no test covering a reason ending in `!`, fixed in the run with mutant M35;
- the switch's length measured before normalizing: the stored waiver covers it, and its issue still stands;
- `analysis.md` counting as fixed a finding an earlier commit removed, waived: the analysis this change owes rewrites it;
- a supplied Summary without the generated one's excluded-from-scope sentence: the stored waiver covers it, checked against `write_review.rs`;
- the new test's doc comment, which still said what the code comment had said: fix round 1's re-read prescribed the edit, and fix round 2 applied it.

Of the stored decisions, these two matched; 31 on files outside the scope are retained.

2 fix round(s): 1 re-read, 1 diff-checked, 0 exempt, 0 not re-read.

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

## Observations

- bug: a waived round's reason ending before a closing quote or bracket, or in an ellipsis or a colon, still takes a second stop — `runtime/src/primitives/write_review.rs` — **waived**: Contrived input: which closing characters end a sentence is a style choice, and the record's contract (078's data model, 066's two loop scenarios) asks only that the Summary name each waived round, which it does; the comment that claimed more is fixed in this run. (decided by the rule)
- convention: loop_summary's comment said a reason that ends its sentence takes no second stop, while only '.', '!' and '?' are checked — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- convention: no test or mutant covered a waived round's reason ending in '!' — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)
- bug: the re-reads-off reason's 16-character minimum is measured before its control characters and runs of spaces are collapsed — `runtime/src/primitives/fix_rounds.rs` — **waived**: Contrived input; the check matches the data model's 'once trimmed'. (decided by the rule)
- other: analysis.md counts as fixed in its run a finding an earlier commit had removed — `specs/078-review-fix-rounds-are-re-read-within-a-bound/analysis.md` — **waived**: analysis.md is write-analysis's output, which the analysis this change owes rewrites; whether a finding an earlier commit removed counts as fixed is analyze's machinery, outside 078's code. (decided by the rule)
- other: a supplied Summary gets the loop's account but not the generated one's excluded-from-scope sentence — `runtime/src/primitives/write_review.rs` — **waived**: 079's contract; the frontmatter's excluded count and the Excluded from scope section carry it. (decided by the rule)
- convention: the new test's doc comment still said a reason that ends its sentence takes no second stop, the wording fix round 1 narrowed in loop_summary's comment (fix round 1's NOTE) — `runtime/src/primitives/write_review.rs` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/079-review-and-analysis-work-converges/tasks.md` — 079-review-and-analysis-work-converges

## Unexamined governance

*None.*
