"""Mutation list for spec 086's AC2, AC4 and the review half of AC3:
`process-decisions` (`runtime/src/primitives/process_decisions.rs`) and
`process-waivers` (`runtime/src/primitives/process_waivers.rs`) drop, on a
spec's record and a run neither restricted nor given `since`, an unfired entry
on a file still on disk that the run's scope does not hold and that does not
belong to the spec, asking `LazyOwnership`
(`runtime/src/primitives/spec_ownership.rs`); a deleted file's entry expires
first (`RunScope::why_unread`, `runtime/src/primitives/compute_review_scope.rs`),
and one whose ownership is not examined is retained.

Run with `python3 scripts/mutation/run.py scripts/mutation/review-drops-unowned-086.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a mutation
that no longer applies is removed, and the next one takes the next free number.
When the code a mutation anchors on changes, update the anchor in the same
change; `run.py --check` names every anchor that no longer matches exactly
once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 a decision on a file that does not belong is retained, not dropped',
        'src/primitives/process_decisions.rs',
        '            Some(Ok(false)) => self.drop_unowned(key, path, decision),',
        '            Some(Ok(false)) => {\n                let _ = Self::drop_unowned;\n                self.retain(Retained::NotRead, why, decision);\n            }',
        'primitives::process_decisions::tests::a_silent_decision_on_a_file_that_does_not_belong_is_dropped_and_named primitives::write_review::tests::dropped_entries_leave_the_record_and_the_project_record_gains_none',
    ),
    (
        'M2 a run given since judges decisions by ownership',
        'src/primitives/process_decisions.rs',
        '        .filter(|_| args.since.is_none() && args.record == DecisionRecord::Review)',
        '        .filter(|_| args.record == DecisionRecord::Review)',
        'primitives::process_decisions::tests::a_restricted_run_and_a_since_run_drop_nothing',
    ),
    (
        'M3 a restricted run judges decisions by their files',
        'src/primitives/process_decisions.rs',
        '        } else if args.restricted {',
        '        } else if false {',
        'primitives::process_decisions::tests::a_restricted_run_and_a_since_run_drop_nothing',
    ),
    (
        'M4 an ownership not examined reads as a file that does not belong',
        'src/primitives/spec_ownership.rs',
        '            Ownership::NotExamined { reason } => Err(reason),',
        '            Ownership::NotExamined { .. } => Ok(false),',
        'primitives::process_decisions::tests::a_shallow_history_drops_nothing_and_says_why primitives::process_waivers::tests::a_shallow_history_drops_no_waiver_and_says_why',
    ),
    (
        'M5 a deleted file reads as unread, so its decision reaches the ownership test',
        'src/primitives/compute_review_scope.rs',
        '            Self::Known { .. } => (!self.holds(path) && still_on_disk(repo, path))',
        '            Self::Known { .. } => (!self.holds(path) && { let _ = (repo, still_on_disk); true })',
        'primitives::process_decisions::tests::a_silent_decision_on_a_deleted_file_expires_and_is_not_dropped',
    ),
    (
        'M6 process-decisions reports no ownership',
        'src/primitives/process_decisions.rs',
        '        (result.ownership, result.ownership_reason) = ownership.state();',
        '        let _ = ownership;',
        'primitives::process_decisions::tests::a_silent_decision_on_a_file_that_does_not_belong_is_dropped_and_named primitives::process_decisions::tests::a_shallow_history_drops_nothing_and_says_why',
    ),
    (
        'M7 process-decisions counts no dropped decision',
        'src/primitives/process_decisions.rs',
        '        dropped_count: classified.dropped.len(),',
        '        dropped_count: 0,',
        'primitives::process_decisions::tests::a_silent_decision_on_a_file_that_does_not_belong_is_dropped_and_named',
    ),
    (
        'M8 a waiver on a file that does not belong is retained, not dropped',
        'src/primitives/process_waivers.rs',
        '                    out.dropped.push(anchor);',
        '                    out.retained.push(anchor);',
        'primitives::process_waivers::tests::a_waiver_on_a_file_that_does_not_belong_is_dropped_and_named primitives::write_review::tests::dropped_entries_leave_the_record_and_the_project_record_gains_none',
    ),
    (
        'M9 a run given since judges waivers by ownership',
        'src/primitives/process_waivers.rs',
        '            .filter(|_| args.since.is_none())',
        '            .filter(|_| true)',
        'primitives::process_waivers::tests::a_restricted_run_and_a_since_run_drop_no_waiver',
    ),
    (
        'M10 process-waivers reports no ownership',
        'src/primitives/process_waivers.rs',
        '        (classified.ownership, classified.ownership_reason) = ownership.state();',
        '        let _ = ownership;',
        'primitives::process_waivers::tests::a_waiver_on_a_file_that_does_not_belong_is_dropped_and_named primitives::process_waivers::tests::a_shallow_history_drops_no_waiver_and_says_why',
    ),
    (
        'M11 process-waivers counts no dropped waiver',
        'src/primitives/process_waivers.rs',
        '        dropped_count: classified.dropped.len(),',
        '        dropped_count: 0,',
        'primitives::process_waivers::tests::a_waiver_on_a_file_that_does_not_belong_is_dropped_and_named',
    ),
]
