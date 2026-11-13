"""Mutation list for 081's task 8: a review states what it read (`unread`,
written by `write-review` whenever the run states it, an entry outside the
run's scope refused), and `compute-review-scope` credits a path of a commit
naming no spec and touching no spec directory once a committed record whose
window holds the commit states it read the path, with no skipped pass, no
budget stop and no open fix-round loop (`runtime/src/primitives/compute_review_scope.rs`,
`runtime/src/primitives/write_review.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/review-read-credit.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 no path is ever credited',
        'src/primitives/compute_review_scope.rs',
        '            match records.iter().find(|record| record.read(commit, &path)) {\n',
        '            match None::<&ReadRecord> {\n',
        'compute_review_scope::tests::a_path_a_complete_record_read_is_credited_and_owes_no_review',
    ),
    (
        'M2 a path the record names unread is credited',
        'src/primitives/compute_review_scope.rs',
        '        self.window.contains(&commit) && !self.unread.names(path)\n',
        '        self.window.contains(&commit)\n',
        'compute_review_scope::tests::a_record_credits_each_path_it_did_not_name_unread',
    ),
    (
        'M3 a record credits a commit its window does not hold',
        'src/primitives/compute_review_scope.rs',
        '        self.window.contains(&commit) && !self.unread.names(path)\n',
        '        !self.unread.names(path)\n',
        'compute_review_scope::tests::a_record_whose_window_does_not_hold_the_commit_credits_nothing',
    ),
    (
        'M4 a record with a skipped pass credits',
        'src/primitives/compute_review_scope.rs',
        '                && record.skipped_passes.is_empty()\n',
        '',
        'compute_review_scope::tests::a_path_no_complete_record_read_stays_and_owes_the_review',
    ),
    (
        'M5 a record with a budget stop credits',
        'src/primitives/compute_review_scope.rs',
        '                && record.budget_stop != Some(true)\n',
        '',
        'compute_review_scope::tests::a_path_no_complete_record_read_stays_and_owes_the_review',
    ),
    (
        'M6 a record with an open fix-round loop credits',
        'src/primitives/compute_review_scope.rs',
        '                && record.fix_rounds_open != Some(true);\n',
        '                ;\n',
        'compute_review_scope::tests::a_path_no_complete_record_read_stays_and_owes_the_review',
    ),
    (
        'M7 a record with no unread credits as a whole read',
        'src/primitives/compute_review_scope.rs',
        '            let Some(unread) = record.unread.filter(|_| complete) else {\n',
        '            let Some(unread) = record.unread.or_else(|| Some(Vec::new())).filter(|_| complete) else {\n',
        'compute_review_scope::tests::a_path_no_complete_record_read_stays_and_owes_the_review',
    ),
    (
        'M8 write-review takes an unread entry outside the scope',
        'src/primitives/write_review.rs',
        '    if outside.is_empty() {\n',
        '    if true {\n',
        'write_review::tests::an_unread_entry_outside_the_scope_is_refused',
    ),
    (
        'M9 an empty unread is written as nothing',
        'src/primitives/write_review.rs',
        '            let _ = writeln!(fm, "unread: []");\n',
        '',
        'write_review::tests::unread_is_written_whenever_the_run_states_it',
    ),
    (
        'M10 unread is never written',
        'src/primitives/write_review.rs',
        '    render_unread(&mut fm, args.unread.as_ref());\n',
        '',
        'write_review::tests::unread_is_written_whenever_the_run_states_it',
    ),
]
